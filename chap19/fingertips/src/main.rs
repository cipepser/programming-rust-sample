extern crate byteorder;

mod index;
mod tmp;
mod write;
mod read;
mod merge;

use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::thread::{spawn, JoinHandle};
use std::sync::mpsc::{channel, Receiver};
use std::path::{PathBuf, Path};
use argparse::{ArgumentParser, Collect, StoreTrue};

use index::InMemoryIndex;
use tmp::TmpDir;
use write::write_index_to_tmp_file;
use merge::FileMerge;
use std::error::Error;

fn run_single_threaded(documents: Vec<PathBuf>, output_dir: PathBuf) -> io::Result<()> {
    let mut accumulated_index = InMemoryIndex::new();
    let mut merge = FileMerge::new(&output_dir);
    let mut tmp_dir = TmpDir::new(&output_dir);

    for (doc_id, filename) in documents.into_iter().enumerate() {
        let mut f = File::open(filename)?;
        let mut text = String::new();
        f.read_to_string(&mut text)?;

        let index = InMemoryIndex::from_single_document(doc_id, text);
        accumulated_index.merge(index);
        if accumulated_index.is_large() {
            let file = write_index_to_tmp_file(accumulated_index, &mut tmp_dir)?;
            merge.add_file(file)?;
            accumulated_index = InMemoryIndex::new();
        }
    }

    if !accumulated_index.is_empty() {
        let file = write_index_to_tmp_file(accumulated_index, &mut tmp_dir)?;
        merge.add_file(file)?;
    }
    merge.finish()
}

fn start_file_reader_thread(documents: Vec<PathBuf>) -> (Receiver<String>, JoinHandle<io::Result<()>>) {
    let (sender, receiver) = channel();

    let handle = spawn(move || {
        for filename in documents {
            let mut f = File::open(filename)?;
            let mut text = String::new();
            f.read_to_string(&mut text)?;

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
                             -> (Receiver<PathBuf>, JoinHandle<io::Result<()>>)
{
    let (sender, receiver) = channel();

    let mut tmp_dir = TmpDir::new(output_dir);
    let handle = spawn(move || {
        for index in big_indexes {
            let file = write_index_to_tmp_file(index, &mut tmp_dir)?;
            if sender.send(file).is_err() {
                break;
            }
        }
        Ok(())
    });

    (receiver, handle)
}

fn merge_index_files(files: Receiver<PathBuf>, output_dir: &Path)
                     -> io::Result<()>
{
    let mut merge = FileMerge::new(output_dir);
    for file in files {
        merge.add_file(file)?;
    }
    merge.finish()
}

fn run_pipeline(documents: Vec<PathBuf>, output_dir: PathBuf) -> io::Result<()> {
    let (texts, h1) = start_file_reader_thread(documents);
    let (pints, h2) = start_file_indexing_thread(texts);
    let (gallons, h3) = start_in_memory_merge_thread(pints);
    let (files, h4) = start_index_writer_thread(gallons, &output_dir);
    let result = merge_index_files(files, &output_dir);

    let r1 = h1.join().unwrap();
    h2.join().unwrap();
    h3.join().unwrap();
    let r4 = h4.join().unwrap();

    r1?;
    r4?;
    result
}

fn expand_filename_arguments(args: Vec<String>) -> io::Result<Vec<PathBuf>> {
    let mut filenames = vec![];
    for arg in args {
        let path = PathBuf::from(arg);
        if path.metadata()?.is_dir() {
            for entry in path.read_dir()? {
                let entry = entry?;
                if entry.file_type()?.is_file() {
                    filenames.push(entry.path());
                }
            }
        } else {
            filenames.push(path);
        }
    }
    Ok(filenames)
}

fn run(filenames: Vec<String>, single_threaded: bool) -> io::Result<()> {
    let output_dir = PathBuf::from(".");
    let documents = expand_filename_arguments(filenames)?;

    if single_threaded {
        run_single_threaded(documents, output_dir)
    } else {
        run_pipeline(documents, output_dir)
    }
}

fn main() {
    let mut single_threaded = false;
    let mut filenames = vec![];

    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Make an inverted index for searching documents.");
        ap.refer(&mut single_threaded)
            .add_option(&["-1", "--single-threaded"], StoreTrue,
                        "Do all the work on a single thread.");
        ap.refer(&mut filenames)
            .add_argument("filenames", Collect,
                          "Names of files/directories to index.\
            For directories, all .txt files immediately \
            under the directory and indexed.");
        ap.parse_args_or_exit();
    }

    match run(filenames, single_threaded) {
        Ok(()) => {}
        Err(err) => println!("error: {:?}", err.description())
    }
}