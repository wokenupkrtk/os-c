use std::collections::HashMap;

#[derive(Debug)]
pub struct File {
    pub name: String,
    pub content: Vec<u8>,
}

#[derive(Debug)]
pub struct Directory {
    pub name: String,
    pub files: HashMap<String, File>,
}

pub struct FileSystem {
    pub root: Directory,
}

impl FileSystem {
    pub fn new() -> Self {
        FileSystem {
            root: Directory {
                name: String::from("/"),
                files: HashMap::new(),
            },
        }
    }

    pub fn create_file(&mut self, name: &str, content: Vec<u8>) {
        let file = File {
            name: name.to_string(),
            content,
        };
        self.root.files.insert(name.to_string(), file);
    }

    pub fn read_file(&self, name: &str) -> Option<&File> {
        self.root.files.get(name)
    }

    pub fn delete_file(&mut self, name: &str) {
        self.root.files.remove(name);
    }
}