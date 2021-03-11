//use std::collections::HashMap;
use hashbrown::HashMap;
use std::sync::{ Arc, Mutex };
use rayon::prelude::*;

pub struct InvertedIndex {
    pub index: HashMap<String, HashMap<u32, u32>>,
}

impl InvertedIndex {
    fn new() -> InvertedIndex {
        InvertedIndex {
            index: HashMap::new(),
        }
    }

    fn add(&mut self, word: &String, docid: &u32) {
        if let Some(word_info) = self.index.get_mut(word) {
            if let Some(freq) = word_info.get_mut(&docid) {
                *freq += 1;
            } else {
                word_info.insert(*docid, 1);
            }
        } else {
            let mut new_word_info = HashMap::new();
            new_word_info.insert(*docid, 1);
            self.index.insert(word.to_string(), new_word_info);
        }
    }

    pub fn get_doc_freq(&self, word: &String, docid: &u32) -> Option<u32> {
        match self.index.get(word) {
            Some(info)  =>  match info.get(docid) {
                Some(freq)  =>  { Some(*freq) },
                None        =>  {
                    println!("There are no instances of {} in doc {}", word, docid);  
                    None
                },
            },
            None        =>  {
                println!("There are no instances of {}", word);
                None
            },
        }
    }

    pub fn get_idx_freq(&self, word: &String) -> Option<u32> {
        match self.index.get(word) {
            Some(info)  =>  Some(info.len() as u32),
            None        =>  None,
        }
    }

    pub fn print(&self) {
        for (word, info) in self.index.iter() {
            for (docid, freq) in info.iter() {
                println!("word: {}, docid: {}, freq: {}", word, docid, freq);
            }
        }
    }
}


pub struct DocumentLengthTable {
    pub table: HashMap<u32, u32>
}

impl DocumentLengthTable {
    fn new() -> DocumentLengthTable {
        DocumentLengthTable {
            table: HashMap::new(),
        }
    }

    fn add(&mut self, docid: &u32, length: &u32) {
        self.table.insert(*docid, *length);
    }

    pub fn get_length(&self, docid: &u32) -> Option<u32> {
        match self.table.get(&docid) {
            Some(len)   =>  Some(*len),
            None        =>  {
                println!("No document ID {} found in table.", docid);
                None
            },
        }
    }

    pub fn get_mean_len(&self) -> f64 {
        self.table.values().sum::<u32>() as f64 / self.table.len() as f64 
    }

    pub fn print(&self) {
        for (docid, length) in self.table.iter() {
            println!("doc {} has length {}", docid, length);
        }
    }
}


pub fn build_data_structures(corpus: Vec<Vec<&str>>) -> (InvertedIndex, DocumentLengthTable) {
    let mut idx = InvertedIndex::new();
    let mut dlt = DocumentLengthTable::new();

    for (docid, doc) in corpus.iter().enumerate() {
        for word in doc {
            idx.add(&word.to_string(), &(docid as u32));
        }
        dlt.add(&(docid as u32), &(doc.len() as u32));
    }
    (idx, dlt)
}
