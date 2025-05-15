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

pub struct Package {
    pub name: String,
}

pub struct PackageDownloader {
    pkg_start_idx: usize,
    num_pkgs: usize,
    event_sender: Sender<Event>,
}

impl PackageDownloader {
    pub fn new(pkg_start_idx: usize, num_pkgs: usize, event_sender: Sender<Event>) -> Self {
        Self {
            pkg_start_idx,
            num_pkgs,
            event_sender,
        }
    }

    pub fn run(&self, pkg_checksum: Arc<Mutex<Checksum>>, hashmap: Arc<RwLock<HashMap<String, Checksum>>>) {
        let file = fs::read_to_string("data/packages.txt").unwrap().lines().map(ToOwned::to_owned).collect::<Vec<String>>();

        // Generate a set of packages and place them into the event queue
        // Update the package checksum with each package name
        for i in 0..self.num_pkgs {
            let name = file[(self.pkg_start_idx + i) % file.len()].clone();

            let val = access_or_write(&hashmap, &name);

            pkg_checksum
                .lock()
                .unwrap()
                .update(val);
            self.event_sender
                .send(Event::DownloadComplete(Package { name }))
                .unwrap();
        }
    }
}
