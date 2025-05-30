use std::alloc;
use std::fs::File;
use std::io::Error as IoError;
use std::io::ErrorKind as IoErrorKind;
use std::io::Read;
use std::io::Write;
use std::ops::Index;
use std::ops::IndexMut;
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Pool<'a, T: Clone + PartialEq> {
    data: &'a [T],
    len: usize,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PathVec {
    paths: Vec<PathBuf>,
    max_depth: usize,
    min_depth: usize,
}

impl<'a, T: Clone + PartialEq> Pool<'a, T> {
    pub fn new() -> Self {
        Self { data: &[], len: 0 }
    }

    pub fn alloc(len: usize) -> &'static mut [T] {
        if len == 0 {
            &mut []
        } else {
            unsafe {
                let bytes = std::mem::size_of::<T>() * len;
                let align = std::mem::align_of::<T>();
                let ptr = alloc::alloc(std::alloc::Layout::from_size_align_unchecked(bytes, align))
                    as *mut T;
                if ptr.is_null() {
                    std::process::abort();
                }
                std::slice::from_raw_parts_mut(ptr, len)
            }
        }
    }

    pub fn free(&mut self) {
        if !self.data.is_empty() {
            unsafe {
                let bytes = std::mem::size_of::<T>() * self.data.len();
                let align = std::mem::align_of::<T>();
                alloc::dealloc(
                    self.data.as_ptr() as *mut u8,
                    std::alloc::Layout::from_size_align_unchecked(bytes, align),
                );
            }
            self.data = &[];
            self.len = 0;
        }
    }

    #[inline]
    pub fn swap(&mut self, i: usize, j: usize) {
        if i < self.len && j < self.len && i != j {
            let slice = self.as_mut();
            let tmp = slice[i].clone();
            slice[i] = slice[j].clone();
            slice[j] = tmp;
        }
    }

    #[inline]
    pub fn size() -> usize {
        std::mem::size_of::<T>()
    }

    #[inline]
    pub fn copy(&mut self, i: usize, src: &T) {
        if i < self.len && self.as_ref()[i] != *src {
            self.as_mut()[i] = src.clone();
        }
    }

    #[inline]
    pub fn split(&self, i: usize, src: &[T]) -> (T, T) {
        let n = src.len();
        if n < 2 || i >= n {
            panic!();
        }
        let mid = n / 2;
        (src[i].clone(), src[mid].clone())
    }

    #[inline]
    pub fn find(&self, i: usize, v: T) -> Option<usize> {
        if i >= self.len {
            None
        } else {
            let mut idx = i;
            while idx < self.len {
                if self.data[idx] == v {
                    return Some(idx);
                }
                idx += 1;
            }
            None
        }
    }

    #[inline]
    pub fn remove(&mut self, i: usize) {
        if i < self.len {
            self.swap(i, self.len - 1);
            self.len -= 1;
        }
    }

    #[inline]
    pub fn bounds(&self) -> (std::ops::Bound<usize>, std::ops::Bound<usize>) {
        (
            std::ops::Bound::Included(0),
            std::ops::Bound::Excluded(self.len),
        )
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    #[inline]
    pub fn clear(&mut self) {
        self.len = 0;
    }

    #[inline]
    pub fn push(&mut self, v: T) {
        let len = self.len;
        if len < self.data.len() {
            self.as_mut()[len] = v;
            self.len += 1;
        } else {
            panic!();
        }
    }

    #[inline]
    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            Some(self.data[self.len].clone())
        }
    }

    #[inline]
    pub fn get(&self, i: usize) -> Option<&T> {
        if i < self.len {
            Some(&self.data[i])
        } else {
            None
        }
    }

    #[inline]
    pub fn get_mut(&mut self, i: usize) -> Option<&mut T> {
        if i < self.len {
            Some(&mut self.as_mut()[i])
        } else {
            None
        }
    }

    pub fn set(&mut self, value: &'a [T]) {
        self.data = value;
        self.len = value.len();
    }

    pub fn append(&mut self, value: &[T]) {
        let len = self.len;
        if len + value.len() > self.data.len() {
            panic!();
        }
        let dst = &mut self.as_mut()[len..len + value.len()];
        let mut i = 0;
        while i < value.len() {
            dst[i] = value[i].clone();
            i += 1;
        }
        self.len += value.len();
    }

    pub fn truncate(&mut self, len: usize) {
        if len < self.len {
            self.len = len;
        }
    }

    pub fn clone(&self) -> Self {
        Self {
            data: self.data,
            len: self.len,
        }
    }

    pub fn into_vec(self) -> Vec<T> {
        let mut v = Vec::with_capacity(self.len);
        let mut i = 0;
        while i < self.len {
            v.push(self.data[i].clone());
            i += 1;
        }
        v
    }

    pub fn as_ref(&self) -> &[T] {
        unsafe { std::slice::from_raw_parts(self.data.as_ptr(), self.len) }
    }

    pub fn as_mut(&mut self) -> &mut [T] {
        unsafe { std::slice::from_raw_parts_mut(self.data.as_ptr() as *mut T, self.len) }
    }

    pub fn as_ptr(&self) -> *const [T] {
        &self.data[..self.len] as *const [T]
    }

    pub fn as_mut_ptr(&mut self) -> *mut T {
        self.data.as_ptr() as *mut T
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, T> {
        self.as_ref().iter()
    }

    #[inline]
    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, T> {
        self.as_mut().iter_mut()
    }

    #[inline]
    pub fn enumerate(&self) -> std::iter::Enumerate<std::slice::Iter<'_, T>> {
        self.iter().enumerate()
    }

    #[inline]
    pub fn enumerate_mut(&mut self) -> std::iter::Enumerate<std::slice::IterMut<'_, T>> {
        self.iter_mut().enumerate()
    }
    #[inline]
    pub fn map<F, U: Clone + PartialEq>(&self, mut f: F) -> Pool<'a, U>
    where
        F: FnMut(&T) -> U,
    {
        let mut p = Pool::<'a, U>::new();
        for item in self.iter() {
            p.push(f(item));
        }
        p
    }
}
impl<'a, T: Clone + PartialEq> Iterator for Pool<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.len == 0 {
            None
        } else {
            let item = self.data[0].clone();
            self.data = &self.data[1..self.len];
            self.len -= 1;
            Some(item)
        }
    }
}
impl<'a, T: Clone + PartialEq> Drop for Pool<'a, T> {
    fn drop(&mut self) {
        self.free();
    }
}

impl<'a, T: Clone + PartialEq> Index<usize> for Pool<'a, T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        if index < self.len {
            &self.data[index]
        } else {
            panic!("index out of bounds");
        }
    }
}

impl<'a, T: Copy + PartialEq> IndexMut<usize> for Pool<'a, T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index < self.len {
            &mut self.as_mut()[index]
        } else {
            panic!("index out of bounds");
        }
    }
}

impl PathVec {
    pub fn new() -> Self {
        Self {
            paths: Vec::<PathBuf>::new(),
            max_depth: 10,
            min_depth: 1,
        }
    }
    pub fn index(&self, index: usize) -> Result<PathBuf, IoError> {
        self.paths
            .get(index)
            .cloned()
            .ok_or_else(|| IoError::new(IoErrorKind::NotFound, "Path not found"))
    }
    pub fn get_at(&self, index: usize) -> Result<PathBuf, IoError> {
        self.index(index)
    }
    pub fn set_at(&mut self, path: PathBuf, index: usize) -> Result<(), IoError> {
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
        self.paths[index] = path.clone();
        Ok(())
    }

    pub fn set(&mut self, paths: Vec<PathBuf>) -> Result<(), IoError> {
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
    pub fn max_depth(&self) -> usize {
        self.max_depth
    }
    pub fn set_max_depth(&mut self, depth: usize) {
        self.max_depth = depth;
    }
    pub fn min_depth(&self) -> usize {
        self.min_depth
    }
    pub fn set_min_depth(&mut self, depth: usize) {
        self.min_depth = depth;
    }
    pub fn paths(&self) -> Result<Vec<PathBuf>, IoError> {
        Ok(self.paths.clone())
    }
    pub fn set_paths(&mut self, paths: Vec<PathBuf>) -> Result<(), IoError> {
        self.paths = paths
            .clone()
            .into_iter()
            .filter(|pathbuf| {
                let count = pathbuf.components().count();
                count >= self.min_depth && count <= self.max_depth
            })
            .collect();

        if self.paths.len() != paths.len() {
            return Err(IoError::new(
                IoErrorKind::InvalidInput,
                "Some paths are out of range",
            ));
        }

        self.paths.sort_unstable();
        self.paths.dedup();
        Ok(())
    }
    pub fn read(&self, index: usize) -> Result<String, IoError> {
        let path = self.index(index)?;
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

    pub fn write(&self, index: usize, contents: &str) -> Result<(), IoError> {
        let path = self.index(index)?;
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

    pub fn read_all(&self) -> Result<Vec<String>, IoError> {
        (0..self.paths.len()).map(|i| self.read(i)).collect()
    }

    pub fn write_all(&self, contents: Vec<String>) -> Result<(), IoError> {
        if contents.len() != self.paths.len() {
            return Err(IoError::new(
                IoErrorKind::InvalidInput,
                "Contents length does not match paths length",
            ));
        }
        for (i, content) in contents.into_iter().enumerate() {
            self.write(i, &content)?;
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
    pub fn is_all_files(&self) -> bool {
        self.paths.iter().all(|p| p.is_file())
    }
    pub fn is_all_dirs(&self) -> bool {
        self.paths.iter().all(|p| p.is_dir())
    }
    pub fn is_all_symlinks(&self) -> bool {
        self.paths.iter().all(|p| p.is_symlink())
    }
    pub fn is_all_files_or_dirs(&self) -> bool {
        self.paths.iter().all(|p| p.is_file() || p.is_dir())
    }
    pub fn is_all_files_or_symlinks(&self) -> bool {
        self.paths.iter().all(|p| p.is_file() || p.is_symlink())
    }
    pub fn is_all_dirs_or_symlinks(&self) -> bool {
        self.paths.iter().all(|p| p.is_dir() || p.is_symlink())
    }
}
impl Index<usize> for PathVec {
    type Output = PathBuf;

    fn index(&self, index: usize) -> &Self::Output {
        &self.paths[index]
    }
}
impl IndexMut<usize> for PathVec {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.paths[index]
    }
}
impl<'a> Index<usize> for &'a PathVec {
    type Output = PathBuf;

    fn index(&self, index: usize) -> &Self::Output {
        &self.paths[index]
    }
}
impl From<Vec<PathBuf>> for PathVec {
    fn from(paths: Vec<PathBuf>) -> Self {
        Self {
            paths,
            max_depth: 10,
            min_depth: 1,
        }
    }
}
impl FromStr for PathVec {
    type Err = IoError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let paths = s
            .split("\n")
            .map(|p| PathBuf::from(p))
            .collect::<Vec<PathBuf>>();
        match !paths.is_empty() {
            true => Ok(Self {
                paths,
                max_depth: 10,
                min_depth: 1,
            }),
            false => {
                let paths = s
                    .split(" ")
                    .map(|p| PathBuf::from(p))
                    .collect::<Vec<PathBuf>>();
                match !paths.is_empty() {
                    true => Ok(Self {
                        paths,
                        max_depth: 10,
                        min_depth: 1,
                    }),
                    false => Err(IoError::new(IoErrorKind::InvalidInput, "Invalid input")),
                }
            }
        }
    }
}
