use std::collections::{HashMap, VecDeque};

const CACHE_SIZE: usize = 10; // max buscas armazenadas

#[derive(Debug)]
pub struct SearchCache {
    data: HashMap<String, Vec<usize>>,
    order: VecDeque<String>,
}

impl SearchCache {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
            order: VecDeque::new(),
        }
    }

    pub fn get(&self, term: &str) -> Option<&Vec<usize>> {
        self.data.get(term)
    }

    pub fn insert(&mut self, term: String, result: Vec<usize>) {
        if self.data.contains_key(&term) {
            return; // já no cache, ignora
        }

        if self.data.len() >= CACHE_SIZE {
            if let Some(oldest) = self.order.pop_front() {
                self.data.remove(&oldest);
            }
        }

        self.order.push_back(term.clone());
        self.data.insert(term, result);
    }
}
