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
        self.paths
            .get(index)
            .cloned()
            .ok_or_else(|| IoError::new(IoErrorKind::NotFound, "Path not found"))
    }

    pub fn get_paths(&self) -> Result<Vec<PathBuf>, IoError> {
        Ok(self.paths.clone())
    }

    pub fn set_path(&mut self, path: &Path, index: usize) -> Result<(), IoError> {
        if !path.exists() {
            return Err(IoError::new(IoErrorKind::NotFound, "Path not found"));
        }

        let count = path.components().count();
        if count < self.min_depth || count > self.max_depth {
            return Err(IoError::new(
                IoErrorKind::InvalidInput,
                "Path depth is out of range",
            ));
        }

        if index >= self.paths.len() {
            self.paths.resize(index + 1, PathBuf::new());
        }
        self.paths[index] = path.to_path_buf();
        Ok(())
    }

    pub fn set_paths(&mut self, paths: Vec<PathBuf>) -> Result<(), IoError> {
        self.paths.clear();
        for path in paths {
            if !path.exists() {
                return Err(IoError::new(IoErrorKind::NotFound, "Path not found"));
            }

            let count = path.components().count();
            if count < self.min_depth || count > self.max_depth {
                return Err(IoError::new(
                    IoErrorKind::InvalidInput,
                    "Path depth is out of range",
                ));
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
        Ok(self
            .paths
            .iter()
            .map(|p| p.file_name().unwrap().to_str().unwrap().to_string())
            .collect())
    }
    pub fn set_names(&mut self, names: Vec<String>) -> Result<(), IoError> {
        self.paths = names
            .clone()
            .into_iter()
            .map(PathBuf::from)
            .filter(|pathbuf| {
                let count = pathbuf.components().count();
                count >= self.min_depth && count <= self.max_depth
            })
            .collect();

        if self.paths.len() != names.len() {
            return Err(IoError::new(
                IoErrorKind::InvalidInput,
                "Some paths are out of range",
            ));
        }

        self.paths.sort_unstable();
        self.paths.dedup();
        Ok(())
    }
    pub fn read_from_path(&self, index: usize) -> Result<String, IoError> {
        let path = self.get_path(index)?;
        if !path.exists() || !path.is_file() {
            return Err(IoError::new(
                IoErrorKind::NotFound,
                "File not found or not a file",
            ));
        }
        let mut buf = String::new();
        File::open(path)?.read_to_string(&mut buf)?;
        Ok(buf)
    }

    pub fn write_to_path(&self, index: usize, contents: &str) -> Result<(), IoError> {
        let path = self.get_path(index)?;
        if !path.exists() || !path.is_file() {
            return Err(IoError::new(
                IoErrorKind::InvalidInput,
                "Path not found or not a file",
            ));
        }
        let mut file = File::create(path)?;
        file.write_all(contents.as_bytes())?;
        Ok(())
    }

    pub fn read_from_paths(&self) -> Result<Vec<String>, IoError> {
        (0..self.paths.len())
            .map(|i| self.read_from_path(i))
            .collect()
    }

    pub fn write_to_paths(&self, contents: Vec<String>) -> Result<(), IoError> {
        if contents.len() != self.paths.len() {
            return Err(IoError::new(
                IoErrorKind::InvalidInput,
                "Contents length does not match paths length",
            ));
        }
        for (i, content) in contents.into_iter().enumerate() {
            self.write_to_path(i, &content)?;
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
    pub fn read_lines_from_path(&self, index: usize) -> Result<Vec<String>, IoError> {
        let path = self.get_path(index)?;
        if !path.exists() || !path.is_file() {
            return Err(IoError::new(
                IoErrorKind::NotFound,
                "File not found or not a file",
            ));
        }
        let mut buf = String::new();
        File::open(path)?.read_to_string(&mut buf)?;
        Ok(buf.lines().map(String::from).collect())
    }

    pub fn read_lines_from_paths(&self, reverse: bool) -> Result<Vec<String>, IoError> {
        let mut vec = Vec::new();
        let indices = if reverse {
            (0..self.paths.len()).rev().collect::<Vec<_>>()
        } else {
            (0..self.paths.len()).collect()
        };

        for i in indices {
            vec.extend(self.read_lines_from_path(i)?);
        }
        Ok(vec)
    }

    pub fn write_lines_to_path(&self, index: usize, contents: Vec<String>) -> Result<(), IoError> {
        let path = self.get_path(index)?;
        if !path.exists() || !path.is_file() {
            return Err(IoError::new(
                IoErrorKind::NotFound,
                "File not found or not a file",
            ));
        }
        let mut file = File::create(path)?;
        for line in contents {
            writeln!(file, "{}", line)?;
        }
        Ok(())
    }

    pub fn write_lines_to_paths(&self, contents: Vec<String>) -> Result<(), IoError> {
        for i in 0..self.paths.len() {
            self.write_lines_to_path(i, contents.clone())?;
        }
        Ok(())
    }
}
