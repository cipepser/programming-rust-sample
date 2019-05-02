mod index;

use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::thread::{spawn, JoinHandle};
use std::sync::mpsc::{channel, Receiver};
use std::path::PathBuf;

use index::InMemoryIndex

fn start_file_reader_thread(documents: Vec<PathBuf>) -> (Receiver<String>, JoinHandle<io::Result<()>>) {
    let (sender, receiver) = channel();

    let handle = spawn(move || {
        for filename in documents {
            let mut f = File::open(filename)?;
            let mut text = String::new();

            f.read_to_string(&mut text);

            if sender.send(text).is_err() {
                break;
            }
        }
        Ok(())
    });

    (receiver, handle)
}

fn start_file_indexing_thread(texts: Receiver<String>) -> (Receiver<InMemoryIndex>, JoinHandle<()>){
    let (sender, receiver) = channel();

    let handle = spawn(move || {
       for (doc_id, text) in texts.into_iter().enumerate() {
           let index = InMemoryIndex::from_single_document(doc_id, text);
           // TODO: ここから続き
       }
    });
}