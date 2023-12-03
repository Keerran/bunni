use std::{sync::Mutex, path::PathBuf, fs::{File, create_dir_all}, collections::{HashMap, HashSet}};

use serde::{Deserialize, Serialize};

use crate::connectors::Format;

pub struct UserPrefs {
    path: PathBuf,
    pub inner: Mutex<PrefData>
}

impl UserPrefs {
    pub fn new(data_dir: PathBuf) -> Self {
        let val = Self {
            path: data_dir.join("userdata.json"),
            inner: Mutex::new(PrefData::default()),
        };
        val.load().unwrap();
        val
    }

    pub fn save(&self) -> Result<(), serde_json::Error> {
        let inner = self.inner.lock().unwrap();
        create_dir_all(self.path.parent().expect("blank save path")).unwrap();
        serde_json::to_writer(File::create(self.path.clone()).unwrap(), &*inner)
    }

    pub fn load(&self) -> Result<(), serde_json::Error> {
        let mut inner = self.inner.lock().unwrap();
        match File::open(self.path.clone()) {
            Ok(file) => *inner = serde_json::from_reader(file)?,
            Err(_) => (),
        };
        Ok(())
    }
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Hash, Eq)]
pub struct StoredManga {
    pub connector_idx: u32,
    pub manga_id: String
}

#[derive(Serialize, Deserialize)]
pub struct PrefData {
    pub liked: Vec<StoredManga>,
    pub views: HashMap<u32, HashMap<String, Format>>,
    pub read: HashMap<u32, HashSet<String>>
}

impl Default for PrefData {
    fn default() -> Self {
        PrefData {
            liked: Vec::new(),
            views: HashMap::new(),
            read: HashMap::new(),
        }
    }
}
