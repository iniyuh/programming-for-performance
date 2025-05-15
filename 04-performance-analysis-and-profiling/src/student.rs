use super::{checksum::Checksum, idea::Idea, package::Package, Event};
use crossbeam::channel::{Receiver, Sender};
use std::io::{stdout, Write};
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

pub struct Student {
    id: usize,
    idea: Option<Idea>,
    pkgs: Vec<Package>,
    skipped_idea: bool,
    event_sender: Sender<Event>,
    event_recv: Receiver<Event>,
}

impl Student {
    pub fn new(id: usize, event_sender: Sender<Event>, event_recv: Receiver<Event>) -> Self {
        Self {
            id,
            event_sender,
            event_recv,
            idea: None,
            pkgs: vec![],
            skipped_idea: false,
        }
    }

    fn build_idea(
        &mut self,
        idea_checksum: &Arc<Mutex<Checksum>>,
        pkg_checksum: &Arc<Mutex<Checksum>>,
        hashmap1: &Arc<RwLock<HashMap<String, Checksum>>>, 
        hashmap2: &Arc<RwLock<HashMap<String, Checksum>>>
    ) {
        if let Some(ref idea) = self.idea {
            // Can only build ideas if we have acquired sufficient packages
            let pkgs_required = idea.num_pkg_required;
            if pkgs_required <= self.pkgs.len() {
                let (mut idea_checksum, mut pkg_checksum) =
                    (idea_checksum.lock().unwrap(), pkg_checksum.lock().unwrap());

                // Update idea and package checksums
                // All of the packages used in the update are deleted, along with the idea
                let val1 = access_or_write(hashmap1, &idea.name);
                idea_checksum.update(val1);

                let pkgs_used = self.pkgs.drain(0..pkgs_required).collect::<Vec<_>>();
                for pkg in pkgs_used.iter() {
                    let val2 = access_or_write(hashmap2, &pkg.name);
                    pkg_checksum.update(val2);
                }

                self.idea = None;
            }
        }
    }

    pub fn run(&mut self, idea_checksum: Arc<Mutex<Checksum>>, pkg_checksum: Arc<Mutex<Checksum>>, hashmap1: Arc<RwLock<HashMap<String, Checksum>>>, hashmap2: Arc<RwLock<HashMap<String, Checksum>>>) {
        loop {
            let event = self.event_recv.recv().unwrap();
            match event {
                Event::NewIdea(idea) => {
                    // If the student is not working on an idea, then they will take the new idea
                    // and attempt to build it. Otherwise, the idea is skipped.
                    if self.idea.is_none() {
                        self.idea = Some(idea);
                        self.build_idea(&idea_checksum, &pkg_checksum, &hashmap1, &hashmap2);
                    } else {
                        self.event_sender.send(Event::NewIdea(idea)).unwrap();
                        self.skipped_idea = true;
                    }
                }

                Event::DownloadComplete(pkg) => {
                    // Getting a new package means the current idea may now be buildable, so the
                    // student attempts to build it
                    self.pkgs.push(pkg);
                    self.build_idea(&idea_checksum, &pkg_checksum, &hashmap1, &hashmap2);
                }

                Event::OutOfIdeas => {
                    // If an idea was skipped, it may still be in the event queue.
                    // If the student has an unfinished idea, they have to finish it, since they
                    // might be the last student remaining.
                    // In both these cases, we can't terminate, so the termination event is
                    // deferred ti the back of the queue.
                    if self.skipped_idea || self.idea.is_some() {
                        self.event_sender.send(Event::OutOfIdeas).unwrap();
                        self.skipped_idea = false;
                    } else {
                        // Any unused packages are returned to the queue upon termination
                        for pkg in self.pkgs.drain(..) {
                            self.event_sender
                                .send(Event::DownloadComplete(pkg))
                                .unwrap();
                        }
                        return;
                    }
                }
            }
        }
    }
}
