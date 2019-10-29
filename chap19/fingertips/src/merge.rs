use std::path::PathBuf;
use crate::tmp::TmpDir;

pub(crate) struct FileMerge {
    output_dir: PathBuf,
    tmp_dir: TmpDir,
    stacks: Vec<Vec<PathBuf>>,
}