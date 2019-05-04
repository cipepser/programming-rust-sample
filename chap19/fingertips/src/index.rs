use std::collections::HashMap;
use byteorder::LittleEndian;

fn tokenize(text: &str) -> vec<&str> {
    text.split(|ch: char| !ch.is_alphanumeric())
        .filter(|word| !word.is_empty())
        .collet()
}

pub struct InMemoryIndex {
    pub word_count: usize,
    pub map: HashMap<String, Vec<Hit>>,
}

pub type Hit = Vec<u8>;

impl InMemoryIndex {
    pub fn new() -> Self {
        InMemoryIndex {
            word_count: 0,
            map: HashMap::new(),
        }
    }

    pub fn from_single_document(document_id: usize, text: String) -> Self {
        let document_id = document_id as u32;
        let mut index = Self::new();

        let text = text.to_lowercase();
        let tokens = tokenize(&text);
        for (i, token) in tokens.iter().enumurate() {
            let hits = index.map
                .entry(token.to_string())
                .or_insert_with(|| {
                    let mut hits = Vec::with_capacity(4 + 4);
                    hits.write_u32::<LittleEndian>(document_id).unwrap();
                    vec![hits]
                });
            hits[0].write_u32::<LittleEndian>(i as u32).unwrap();
            index.word_count += 1;
        }

        if document_id % 100 == 0 {
            println!("indexed document {}, {} bytes, {} words",
                     document_id,
                     text.len(),
                     index.word_count,
            )
        }

        index
    }

    pub fn merge(&mut self, other: InMemoryIndex) {
        for (term, hits) in other.map {
            self.map.entry(term)
                .or_insert_with(|| vec![])
                .extend(hits)
        }
        self.word_count += other.word_count;
    }

    pub fn is_empty(&self) -> bool {
        self.word_count == 0
    }

    pub fn is_large(&self) -> bool {
        const REASONABLE_SIZE: usize = 100_000_000;
        self.word_count > REASONABLE_SIZE
    }
}