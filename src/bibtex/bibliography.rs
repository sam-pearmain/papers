use std::collections::HashMap;

use crate::bibtex::entry::Entry;
use crate::bibtex::error::BibliographyError;


pub struct Bibliography {
    entries: HashMap<String, Entry>, 
}

impl Bibliography {
    pub fn new() -> Self {
        Bibliography { entries: HashMap::new() }
    }

    pub fn add(&mut self, entry: Entry) {
        let citekey = entry.citekey.clone();
        self.entries.insert(citekey, entry);
    }

    pub fn remove(&mut self, citekey: &str) -> Option<Entry> {
        self.entries.remove(citekey)
    }

    pub fn discard(&mut self, citekey: &str) -> Result<(), BibliographyError> {
        if let Some(_) = self.remove(citekey) {
            Ok(())
        } else {
            Err(BibliographyError::EntryNotFound { citekey: citekey.to_string() })
        }
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}