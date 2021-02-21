use serde::{Deserialize, Serialize};
use sha1::{Digest, Sha1};
use std::fs;
use std::io::prelude::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct File {
    pub path: String,
    pub hash: String,
}

impl File {
    pub fn _new(path: String) -> Self {
        let mut file = fs::File::open(&path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let h = Sha1::digest_str(&contents);

        Self {
            hash: format!("{:x}", h),
            path,
        }
    }

    pub async fn new_async(path: String) -> Self {
        let content = async_std::fs::read(&path).await.unwrap();
        let file = std::str::from_utf8(&content).unwrap();
        let h = Sha1::digest_str(&file);

        Self {
            hash: format!("{:x}", h),
            path,
        }
    }
}
