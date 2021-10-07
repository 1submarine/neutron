use std::{fs::File, io::Write};

use flate2::{write::GzEncoder, Compression};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// (Filename, Serialize String)
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SaveGame {
    objects: Vec<(String, String)>,
    uuidv4: Uuid,
}
impl SaveGame {
    pub fn new(objects: Vec<(String, String)>) -> Self {
        Self {
            objects,
            uuidv4: Uuid::new_v4(),
        }
    }
    pub fn write(&self, cache_dir: &std::path::Path) -> Result<(), String> {
        for (file, data) in self.objects.iter() {
            let mut e = GzEncoder::new(Vec::new(), Compression::default());
            e.write_all(data.as_str().as_bytes()).unwrap();

            let mut file =
                File::create(cache_dir.join(file.to_string() + ".yaml" + ".gz")).unwrap();
            file.write_all(&e.finish().unwrap()).unwrap();
        }
        Ok(())
    }
}
