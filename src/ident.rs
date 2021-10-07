use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Ident {
    name: String,
    refer: String,
    pub uuidv4: Uuid,
}
impl Ident {
    pub fn new<S: Into<String>>(nname: S) -> Self {
        let name = nname.into();
        let refer = Self::create_refer(name.clone());
        Self {
            name,
            refer,
            uuidv4: Uuid::new_v4(),
        }
    }
    fn create_refer(name: String) -> String {
        name.to_lowercase().split_whitespace().collect()
    }
    pub fn update<S: Into<String>>(&mut self, nname: S) {
        self.name = nname.into();
        self.refer = Self::create_refer(self.name.clone())
    }
}
