use std::collections::{HashMap, HashSet};
use std::collections::hash_set::Iter;

pub struct Department {
    people: HashSet<String>,
}

impl Department {
    pub fn new() -> Self {
        Self {
            people: HashSet::new()
        }
    }
    
    pub fn contains(&self, person: &String) -> bool {
        self.people.contains(person)
    }

    pub fn add(&mut self, person: String) {
        self.people.insert(person);
    }

    pub fn remove(&mut self, person: &String) {
        self.people.remove(person);
    }

    pub fn get_all(&self) -> Iter<String> {
        self.people.iter()
    }
}

pub struct Company {
    it: HashMap<String, Department>,
}

impl Company {
    pub fn new() -> Self {
        Company {
            it: HashMap::new()
        }
    }
    
    pub fn contains(&self, dep: &String) -> bool {
        self.it.contains_key(dep)
    }

    pub fn get(&mut self, dep: &String) -> &mut Department {
        self.it.get_mut(dep).unwrap()
    }

    pub fn add(&mut self, dep: String) {
        self.it.insert(dep, Department::new());
    }

    pub fn remove(&mut self, dep: &String) {
        self.it.remove(dep);
    }
}