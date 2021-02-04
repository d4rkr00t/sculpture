use serde::{Deserialize, Serialize};
use sha1::{Digest, Sha1};
use std::fs;
use std::io::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct File {
    pub path: String,
    pub hash: String,
}

impl File {
    pub fn new(path: String) -> File {
        let mut file = fs::File::open(&path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let h = Sha1::digest_str(&contents);
        return File {
            hash: format!("{:x}", h),
            path,
        };
    }
}
