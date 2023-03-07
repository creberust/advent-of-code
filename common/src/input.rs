use std::{
    fs::File,
    io::{BufRead, BufReader, Cursor},
    path::{Path, PathBuf},
};

pub enum Input {
    File(PathBuf),
    Text(String),
}

impl From<&Path> for Input {
    fn from(value: &Path) -> Self {
        Input::File(value.to_path_buf())
    }
}

impl From<String> for Input {
    fn from(value: String) -> Self {
        Input::Text(value)
    }
}

impl Input {
    pub fn read(&self) -> Box<dyn BufRead + '_> {
        match self {
            Input::File(path) => {
                let file = File::open(path).unwrap();
                Box::new(BufReader::new(file))
            }
            Input::Text(text) => Box::new(Cursor::new(text)),
        }
    }
}
