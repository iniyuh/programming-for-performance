use super::checksum::Checksum;
use super::Event;
use crossbeam::channel::Sender;
use std::fs;
use std::sync::{Arc, Mutex, RwLock};
use std::collections::HashMap;

pub fn access_or_write(hashmap: &Arc<RwLock<HashMap<String, Checksum>>>, name: &String) -> Checksum {
    {
        let map = hashmap.read().unwrap();

        if let Some(val) = map.get(name) {
            return val.clone();
        }
    }

    let mut map = hashmap.write().unwrap();
    let val = Checksum::with_sha256(name);
    map.insert(name.clone(), val.clone());
    return val;
}

pub struct Idea {
    pub name: String,
    pub num_pkg_required: usize,
}

pub struct IdeaGenerator {
    idea_start_idx: usize,
    num_ideas: usize,
    num_students: usize,
    num_pkgs: usize,
    event_sender: Sender<Event>,
}

impl IdeaGenerator {
    pub fn new(
        idea_start_idx: usize,
        num_ideas: usize,
        num_students: usize,
        num_pkgs: usize,
        event_sender: Sender<Event>,
    ) -> Self {
        Self {
            idea_start_idx,
            num_ideas,
            num_students,
            num_pkgs,
            event_sender,
        }
    }

    // Idea names are generated from cross products between product names and customer names
    fn get_next_idea_name(idx: usize, products: &Vec<String>, customers: &Vec<String>) -> String {
        // ASSUMING: MxM matrix, aka length of products and customers is the same.
        let y = idx / products.len();
        let x = idx % customers.len();

        let product = &products[y % products.len()];
        let customer = &customers[x];

        format!("{} for {}", product, customer)
    }

    pub fn run(&self, idea_checksum: Arc<Mutex<Checksum>>, hashmap: Arc<RwLock<HashMap<String, Checksum>>>) {
        let products = fs::read_to_string("data/ideas-products.txt").unwrap().lines().map(ToOwned::to_owned).collect::<Vec<String>>();
        let customers = fs::read_to_string("data/ideas-customers.txt").unwrap().lines().map(ToOwned::to_owned).collect::<Vec<String>>();

        let pkg_per_idea = self.num_pkgs / self.num_ideas;
        let extra_pkgs = self.num_pkgs % self.num_ideas;

        // Generate a set of new ideas and place them into the event-queue
        // Update the idea checksum with all generated idea names
        for i in 0..self.num_ideas {
            let name = Self::get_next_idea_name(self.idea_start_idx + i, &products, &customers);
            let extra = (i < extra_pkgs) as usize;
            let num_pkg_required = pkg_per_idea + extra;
            let idea = Idea {
                name: name.clone(),
                num_pkg_required,
            };

            let val = access_or_write(&hashmap, &name);

            idea_checksum
                .lock()
                .unwrap()
                .update(val);

            self.event_sender.send(Event::NewIdea(idea)).unwrap();
        }

        // Push student termination events into the event queue
        for _ in 0..self.num_students {
            self.event_sender.send(Event::OutOfIdeas).unwrap();
        }
    }
}
