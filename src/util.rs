use std::fs::File;
use std::io::Error as IoError;
use std::io::ErrorKind as IoErrorKind;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use std::usize;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PathIO {
    paths: Vec<PathBuf>,
    max_depth: usize,
    min_depth: usize,
}
impl PathIO {
    pub fn new() -> Self {
        Self {
            paths: Vec::<PathBuf>::new(),
            max_depth: 10,
            min_depth: 1,
        }
    }
    pub fn get_path(&self, index: usize) -> Result<PathBuf, IoError> {
        match self.paths.get(index) {
            Some(path) => Ok(path.clone()),
            None => Err(IoError::new(IoErrorKind::NotFound, "Path not found")),
        }
    }
    pub fn get_paths(&self) -> Result<Vec<PathBuf>, IoError> {
        Ok(self.paths.clone())
    }
    pub fn set_path(&mut self, path: &Path, index: usize) -> Result<(), IoError> {
        if index >= self.paths.len() {
            self.paths.resize(index + 1, PathBuf::new());
        }
        if path.exists() {
            let count = path.components().count();
            if count < self.min_depth || count > self.max_depth {
                return Err(IoError::new(IoErrorKind::InvalidInput, "Path depth is out of range"));
            }
            self.paths[index] = path.to_path_buf();
            Ok(())
        } else {
            Err(IoError::new(IoErrorKind::NotFound, "Path not found"))
        }
    }
    pub fn set_paths(&mut self, paths: Vec<PathBuf>) -> Result<(), IoError> {
        self.paths.clear();
        for path in paths {
            let count = path.components().count();
            if count < self.min_depth || count > self.max_depth {
                return Err(IoError::new(IoErrorKind::InvalidInput, "Path depth is out of range"));
            }
            self.paths.push(path);
        }
        Ok(())
    }
    pub fn get_max_depth(&self) -> usize {
        self.max_depth
    }
    pub fn set_max_depth(&mut self, depth: usize) {
        self.max_depth = depth;
    }
    pub fn get_min_depth(&self) -> usize {
        self.min_depth
    }
    pub fn set_min_depth(&mut self, depth: usize) {
        self.min_depth = depth;
    }
    pub fn get_names(&self) -> Result<Vec<String>, IoError> {
        Ok(self.paths.iter().map(|p| p.file_name().unwrap().to_str().unwrap().to_string()).collect())
    }
    pub fn set_names(&mut self, names: Vec<String>) -> Result<(), IoError> {
        self.paths.clear();
        for name in names {
            let pathbuf = PathBuf::from(name.clone());
            let count = pathbuf.components().count();
            if count < self.min_depth || count > self.max_depth {
                return Err(IoError::new(IoErrorKind::InvalidInput, "Path depth is out of range"));
            }
            self.paths.push(pathbuf);
        }
        self.paths.sort();
        self.paths.dedup();
        Ok(())
    }
    pub fn read_from_path(&self, index: usize) -> Result<String, IoError> {
        if index >= self.paths.len() {
            return Err(IoError::new(IoErrorKind::InvalidInput, "Index out of bounds"));
        }
        let path = self.get_path(index).unwrap();
        if !path.exists() {
            return Err(IoError::new(IoErrorKind::NotFound, "File not found"));
        }
        if !path.is_file() {
            return Err(IoError::new(IoErrorKind::InvalidInput, "Not a file"));
        }
        let mut file = File::open(path)?;
        let mut buf = String::new();
        file.read_to_string(&mut buf)?;
        Ok(buf)
    }
    pub fn write_to_path(&self, index: usize, contents: &str) -> Result<(), IoError> {
        if index >= self.paths.len() {
            return Err(IoError::new(IoErrorKind::InvalidInput, "Index out of bounds"));
        }
        let path = self.get_path(index).unwrap();
        if !path.exists() {
            let mut file = File::create(path)?;
            file.write_all(contents.as_bytes())?;
            return Ok(());
        } else if path.is_file() {
            let mut file = File::create(path)?;
            file.write_all(contents.as_bytes())?;
            return Ok(());
        } else {
            return Err(IoError::new(IoErrorKind::InvalidInput, "Not a file"));
        }
    }
    pub fn read_from_paths(&self) -> Result<Vec<String>, IoError> {
        let mut vec = Vec::<String>::new();
        for i in 0..self.paths.len() {
            match self.read_from_path(i) {
                Ok(contents) => vec.push(contents),
                Err(e) => return Err(e),
            }
        }
        Ok(vec)
    }
    pub fn write_to_paths(&self, contents: Vec<String>) -> Result<(), IoError> {
        for i in 0..self.paths.len() {
            match self.write_to_path(i, &contents[i]) {
                Ok(_) => {},
                Err(e) => return Err(e),
            }
        }
        Ok(())
    }
    pub fn exists_count(&self) -> usize {
        self.paths.iter().filter(|p| p.exists()).count()
    }
    pub fn files_count(&self) -> usize {
        self.paths.iter().filter(|p| p.is_file()).count()
    }
    pub fn dirs_count(&self) -> usize {
        self.paths.iter().filter(|p| p.is_dir()).count()
    }
    pub fn symlinks_count(&self) -> usize {
        self.paths.iter().filter(|p| p.is_symlink()).count()
    }
    pub fn exists(&self) -> bool {
        self.paths.iter().all(|p| p.exists())
    }
    pub fn count_files(&self) -> usize {
        self.paths.iter().filter(|p| p.is_file()).count()
    }
    pub fn all_files(&self) -> bool {
        self.paths.iter().all(|p| p.is_file())
    }
    pub fn count_dirs(&self) -> usize {
        self.paths.iter().filter(|p| p.is_dir()).count()
    }
    pub fn all_dirs(&self) -> bool {
        self.paths.iter().all(|p| p.is_dir())
    }
    pub fn all_symlinks(&self) -> bool {
        self.paths.iter().all(|p| p.is_symlink())
    }
    pub fn all_files_or_dirs(&self) -> bool {
        self.paths.iter().all(|p| p.is_file() || p.is_dir())
    }
    pub fn all_files_or_symlinks(&self) -> bool {
        self.paths.iter().all(|p| p.is_file() || p.is_symlink())
    }
    pub fn all_dirs_or_symlinks(&self) -> bool {
        self.paths.iter().all(|p| p.is_dir() || p.is_symlink())
    }
}