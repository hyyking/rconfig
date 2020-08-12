use std::{
    collections::HashMap,
    fs::{self, OpenOptions},
    io::{self, Write},
    path::{Path, PathBuf},
};

#[derive(Debug)]
pub struct TrackFile {
    file: PathBuf,
    map: Map,
}
#[derive(Default, Debug, ::serde_derive::Serialize, ::serde_derive::Deserialize)]
pub struct Map {
    entries: HashMap<String, Entry>,
}

#[derive(Debug, ::serde_derive::Serialize, ::serde_derive::Deserialize)]
pub struct Entry {
    hash: u64,
    file: String,
}
impl Entry {
    pub fn new(file: String, abbr: &str) -> Self {
        use std::{
            collections::hash_map::DefaultHasher,
            hash::{Hash, Hasher},
        };
        let mut hasher = DefaultHasher::default();
        file.hash(&mut hasher);
        abbr.hash(&mut hasher);
        Self {
            hash: hasher.finish(),
            file,
        }
    }
    pub fn file(&self) -> &str {
        &self.file
    }
    pub fn hash(&self) -> u64 {
        self.hash
    }
}

impl TrackFile {
    pub fn open(path: impl AsRef<Path>) -> io::Result<Self> {
        let file = PathBuf::from(path.as_ref());
        let buffer = fs::read_to_string(&file).unwrap_or_default();
        let map: Map = serde_json::from_str(&buffer).unwrap_or_default();
        Ok(Self { file, map })
    }
    pub fn path(&self) -> &Path {
        &self.file
    }
    pub fn map(&self) -> &HashMap<String, Entry> {
        &self.map
    }
    pub fn map_mut(&mut self) -> &mut HashMap<String, Entry> {
        &mut self.map
    }
}

impl Drop for TrackFile {
    fn drop(&mut self) {
        let c = serde_json::to_string(&self.map).expect("unable to deserialize track file content");
        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&self.file)
            .unwrap();
        write!(file, "{}", c).unwrap();
        file.flush().unwrap()
    }
}

impl std::ops::Deref for Map {
    type Target = HashMap<String, Entry>;
    fn deref(&self) -> &Self::Target {
        &self.entries
    }
}
impl std::ops::DerefMut for Map {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entries
    }
}
