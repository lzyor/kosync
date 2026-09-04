// ╦  ┌─┐┬ ┬┌─┐┬─┐ Lzyor Studio
// ║  ┌─┘└┬┘│ │├┬┘ kosync-project
// ╩═╝└─┘ ┴ └─┘┴└─ https://lzyor.work/koreader/
// 2023 (c) Lzyor

use sled::{IVec, Result, Tree};
use std::path::Path;

use crate::defs::{self, ProgressState};

macro_rules! key_user {
    ($s:expr) => {
        format!("U:{}:K", $s)
    };
}

macro_rules! key_doc {
    ($u:expr, $d:expr) => {
        format!("U:{}:D:{}", $u, $d)
    };
}

#[derive(Debug, Clone)]
pub struct DB(Tree);

impl DB {
    pub fn new<P: AsRef<Path>>(root: &P) -> Result<Self> {
        let tree = sled::Config::new()
            .path(root)
            .mode(sled::Mode::LowSpace)
            .cache_capacity(256 * 1024)
            .open()?
            .open_tree(defs::DEFAULT_TREE_NAME)?;
        Ok(Self(tree))
    }

    #[inline]
    pub fn put_user(&self, name: &str, key: &str) -> Result<Option<IVec>> {
        self.0.insert(key_user!(name), key)
    }

    #[inline]
    pub fn get_user(&self, name: &str) -> Result<Option<IVec>> {
        self.0.get(key_user!(name))
    }

    #[inline]
    pub fn put_doc(&self, user: &str, doc: &str, value: &ProgressState) -> Result<Option<IVec>> {
        match serde_json::to_vec(value) {
            Ok(v) => self.0.insert(key_doc!(user, doc), v),
            Err(_) => Ok(None),
        }
    }

    #[inline]
    pub fn get_doc(&self, user: &str, doc: &str) -> Result<Option<ProgressState>> {
        match self.0.get(key_doc!(user, doc)) {
            Ok(Some(v)) => Ok(serde_json::from_slice(&v).ok()),
            Ok(None) => Ok(None),
            Err(e) => Err(e),
        }
    }
}
