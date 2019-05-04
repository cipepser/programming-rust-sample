mod index;
mod tmp;

use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::thread::{spawn, JoinHandle};
use std::sync::mpsc::{channel, Receiver};
use std::path::{PathBuf, Path};

use index::InMemoryIndex;


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

fn start_file_indexing_thread(texts: Receiver<String>) -> (Receiver<InMemoryIndex>, JoinHandle<()>) {
    let (sender, receiver) = channel();

    let handle = spawn(move || {
        for (doc_id, text) in texts.into_iter().enumerate() {
            let index = InMemoryIndex::from_single_document(doc_id, text);
            if sender.send(index).is_err() {
                break;
            }
        }
    });

    (receiver, handle)
}

// TODO: implement start_in_memory_merge_thread, start_index_writer_thread and merge_index_files

fn start_in_memory_merge_thread(file_indexes: Receiver<InMemoryIndex>)
    -> (Receiver<InMemoryIndex>, JoinHandle<()>)
{
    let (sender, receiver) = channel();

    let handle = spawn(move || {
        let mut accumulated_index = InMemoryIndex::new();
        for fi in file_indexes {
            accumulated_index.merge(fi);
            if accumulated_index.is_large() {
                if sender.send(accumulated_index).is_err() {
                    return;
                }
                accumulated_index = InMemoryIndex::new();
            }
        }
        if !accumulated_index.is_empty() {
            let _ = sender.send(accumulated_index);
        }
    });

    (receiver, handle)
}

fn start_index_writer_thread(big_indexes: Receiver<InMemoryIndex>, output_dir: &Path)
    -> (Receiver<PathBuf>, JoinHandle<io::Result()>)
{
    let (sender, receiver) = channel();

    // TODO: ここから続き
    let mut tmp_dir = TmpDir
}