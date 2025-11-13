use std::collections::HashMap;

use crate::bibtex::entry::Entry;


pub struct Bibliography<'a> {
    entries: HashMap<&'a String, Entry>, 
}

impl<'a> Bibliography<'a> {
    fn new() -> Self {
        Bibliography { entries: HashMap::new() }
    }

    fn add(&mut self, entry: &'a Entry) {
        let citekey = &entry.citekey;
        self.entries.insert(citekey, entry.clone());
    }

    fn remove(&mut self, citekey: String) {
        
    }
}