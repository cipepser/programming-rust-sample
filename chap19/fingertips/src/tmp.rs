use std::io::{self, BufWriter};
use std::fs::{self, File};
use std::path::{Path, PathBuf};
use std::fs::File;

#[derive(Clone)]
pub struct TmpDir {
    dir: PathBuf,
    n: usize,
}

impl TmpDir {
    pub fn new<P: AsRef<Path>>(dir: P) -> Self {
        Self {
            dir: dir.as_ref().to_owned(),
            n: 1,
        }
    }

    pub fn create(&mut self) -> io::Result<(PathBuf, BufWriter<File>)> {
        let mut trial = 1;
        loop {
            let filename = self.dir.join(PathBuf::from(
                format!("tmp{:08x}.data", self.n)
            ));
            self.n += 1;
            match fs::OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(&filename)
                {
                    Ok(f) => return Ok((filename, BufWriter::new(f))),
                    Err(exc) =>
                        if trial < 999 && exc.kind() == io::ErrorKind::AlreadyExists {
                            // keep going
                        } else {
                            return Err(exc);
                        }
                }
            trial += 1;
        }
    }
}