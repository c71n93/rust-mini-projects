use std::{fs, io};
use std::collections::HashSet;
use std::path::Path;

pub struct FileContent {
    content: String
}

impl FileContent {
    pub fn new() -> FileContent {
        FileContent {
            content: String::new(),
        }
    }

    pub fn lines(&self) -> std::str::Lines<'_> {
        self.content.lines()
    }

    // TODO: to make the method more optimal after studying iterators
    pub fn common_lines<'a>(&self, content: &'a FileContent) -> Vec<&'a str> {
        let lines_set: HashSet<&str> = self.lines().collect();
        let mut intersections = HashSet::new();
        let mut res = Vec::new();
        for line in content.lines() {
            if lines_set.contains(line) && !intersections.contains(line) {
                intersections.insert(line);
                res.push(line)
            }
        }
        res
    }
}

impl From<&str> for FileContent {
    fn from(str: &str) -> Self {
        FileContent {
            content: String::from(str)
        }
    }
}

impl From<&String> for FileContent {
    fn from(str: &String) -> Self {
        FileContent {
            content: String::from(str)
        }
    }
}

impl TryFrom<&Path> for FileContent {
    type Error = io::Error;

    fn try_from(path: &Path) -> Result<Self, Self::Error> {
        Ok(
            FileContent {
                content: fs::read_to_string(path)?
            }
        )
    }
}
