use serde::{Deserialize, Serialize};
use sha1::{Digest, Sha1};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct File {
    pub path: String,
    pub hash: String,
    pub modified: u128,
}

impl File {
    pub async fn new(path: String) -> Self {
        let content = async_std::fs::read(&path).await.unwrap();
        let file = std::str::from_utf8(&content).unwrap();
        let h = Sha1::digest_str(&file);
        let modified = Self::get_modified_time(&path).await;

        Self {
            hash: format!("{:x}", h),
            path,
            modified,
        }
    }

    async fn get_modified_time(path: &str) -> u128 {
        async_std::fs::metadata(path)
            .await
            .unwrap()
            .modified()
            .unwrap()
            .duration_since(std::time::SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_millis()
    }

    pub async fn invalidate(&self) -> (bool, Self) {
        let modified = Self::get_modified_time(&self.path).await;
        if modified != self.modified {
            let new_file = Self::new(self.path.to_owned()).await;
            return (new_file.hash != self.hash, new_file);
        }
        (false, self.clone())
    }
}
