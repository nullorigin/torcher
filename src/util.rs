use std::alloc::alloc;
use std::cmp::Ordering;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fs::File;
use std::io::Error as IoError;
use std::io::ErrorKind as IoErrorKind;
use std::io::Read;
use std::io::Write;
use std::iter;
use std::iter::Cloned;
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Bound;
use std::ops::Deref;
use std::ops::DerefMut;
use std::ops::Index;
use std::ops::IndexMut;
use std::ops::Range;
use std::ops::RangeBounds;
use std::ops::Sub;
use std::ops::SubAssign;
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq)]
pub struct Set<T: Clone + PartialEq> {
    data: *mut T,
    len: usize,
}


#[derive(Clone, Eq, PartialEq, Default)]
pub struct Paths {
    paths: Vec<PathBuf>,
    max_depth: usize,
    min_depth: usize,
}

impl<T: Clone + PartialEq> Set<T> {
    #[inline]
    pub fn new() -> Self {
        Self { data: std::ptr::null_mut(), len: 0 }
    }
    #[inline]
    fn alloc(len: usize) -> *mut T {
        if len == 0 {
            std::ptr::null_mut()
        } else {
            unsafe {
                let layout = std::alloc::Layout::array::<T>(len).unwrap();
                let ptr = std::alloc::alloc(layout) as *mut T;
                if ptr.is_null() { std::process::abort(); }
                ptr
            }
        }
    }
    #[inline]
    fn free(&mut self) {
        if !self.data.is_null() && self.len > 0 {
            unsafe {
                let layout = std::alloc::Layout::array::<T>(self.len).unwrap();
                std::alloc::dealloc(self.data as *mut u8, layout);
            }
        }
        self.data = std::ptr::null_mut();
        self.len = 0;
    }
    #[inline]
    pub fn default() -> Self { Self::new() }
    pub fn clone(&self) -> Self {
        let mut set = Set::new();
        if self.len > 0 {
            set.data = Self::alloc(self.len);
            unsafe { std::ptr::copy_nonoverlapping(self.data, set.data, self.len); }
            set.len = self.len;
        }
        set
    }
    pub fn copy(&self) -> Self { self.clone() }
    pub fn into_iter(self) -> std::slice::Iter<'static, T> {
        unsafe { std::slice::from_raw_parts(self.data, self.len).into_iter() }
    }
    #[inline]
    pub fn len(&self) -> usize { self.len }
    #[inline]
    pub fn is_empty(&self) -> bool { self.len == 0 }
    #[inline]
    pub fn as_slice(&self) -> &[T] { unsafe { std::slice::from_raw_parts(self.data, self.len) } }
    #[inline]
    pub fn as_mut_slice(&mut self) -> &mut [T] { unsafe { std::slice::from_raw_parts_mut(self.data, self.len) } }
    #[inline]
    pub fn to_vec(&self) -> Vec<T> { self.as_slice().to_vec() }
    #[inline]
    pub fn push(&mut self, v: T) {
        let len = self.len + 1;
        let data = Self::alloc(len);
        unsafe {
            if self.len > 0 {
                std::ptr::copy_nonoverlapping(self.data, data, self.len);
            }
            std::ptr::write(data.add(self.len), v);
        }
        self.free();
        self.data = data;
        self.len = len;
    }
    #[inline]
    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 { None }
        else {
            self.len -= 1;
            Some(unsafe { std::ptr::read(self.data.add(self.len)) })
        }
    }
    #[inline]
    pub fn pop_at(&mut self, i: usize) -> Option<T> {
        if i >= self.len { None }
        else {
            let ret = unsafe { std::ptr::read(self.data.add(i)) };
            if i < self.len - 1 {
                unsafe {
                    std::ptr::copy(self.data.add(i + 1), self.data.add(i), self.len - i - 1);
                }
            }
            self.len -= 1;
            Some(ret)
        }
    }
    #[inline]
    pub fn clear(&mut self) { self.free(); }
    #[inline]
    pub fn index(&self, i: usize) -> Option<&T> {
        if i < self.len { Some(unsafe { &*self.data.add(i) }) } else { None }
    }
    #[inline]
    pub fn index_mut(&mut self, i: usize) -> Option<&mut T> {
        if i < self.len { Some(unsafe { &mut *self.data.add(i) }) } else { None }
    }
    pub fn from_slice(&mut self, val: &[T]) {
        self.free();
        self.data = Self::alloc(val.len());
        if !self.data.is_null() && val.len() > 0 {
            unsafe { std::ptr::copy_nonoverlapping(val.as_ptr(), self.data, val.len()); }
        }
        self.len = val.len();
    }
    pub fn extend(&mut self, set: &Set<T>) {
        let len = self.len + set.len();
        let data = Self::alloc(len);
        unsafe {
            if self.len > 0 {
                std::ptr::copy_nonoverlapping(self.data, data, self.len);
            }
            if set.len() > 0 {
                std::ptr::copy_nonoverlapping(set.data, data.add(self.len), set.len());
            }
        }
        self.free();
        self.data = data;
        self.len = len;
    }
    pub fn extend_as_slice(&mut self, set: &[T]) {
        if set.is_empty() { return; }
        let len = self.len + set.len();
        let data = Self::alloc(len);
        unsafe {
            if self.len > 0 {
                std::ptr::copy_nonoverlapping(self.data, data, self.len);
            }
            std::ptr::copy_nonoverlapping(set.as_ptr(), data.add(self.len), set.len());
        }
        self.free();
        self.data = data;
        self.len = len;
    }
    pub fn truncate(&mut self, len: usize) {
        if len < self.len {
            unsafe {
                std::ptr::drop_in_place(std::slice::from_raw_parts_mut(self.data.add(len), self.len - len));
            }
            self.len = len;
        }
    }
    pub fn iter(&self) -> std::slice::Iter<'_, T> { self.as_slice().iter() }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, T> { self.as_mut_slice().iter_mut() }
    pub fn find<F: FnMut(&T) -> bool>(&self, mut f: F) -> Option<usize> {
        self.iter().position(|x| f(x))
    }
    pub fn retain<F: FnMut(&T) -> bool>(&mut self, mut f: F) {
        let mut k = 0;
        for i in 0..self.len() {
            if f(&self[i]) {
                if i != k { self[k] = self[i].clone(); }
                k += 1;
            }
        }
        self.truncate(k);
    }
    pub fn remove<F: FnMut(&T) -> bool>(&mut self, mut f: F) -> Option<T> {
        if let Some(i) = self.find(|x| f(x)) { self.pop_at(i) } else { None }
    }
    pub fn remove_at(&mut self, i: usize) -> Option<T> { self.pop_at(i) }
    pub fn find_range<F: FnMut(&T) -> bool>(&self, mut f: F) -> Option<Range<usize>> {
        let mut start = 0;
        while start < self.len() && !f(&self[start]) { start += 1; }
        if start == self.len() { return None; }
        let mut end = start + 1;
        while end < self.len() && f(&self[end]) { end += 1; }
        Some(start..end)
    }
    pub fn find_range_mut<F: FnMut(&T) -> bool>(&mut self, mut f: F) -> Option<Range<usize>> {
        let mut start = 0;
        while start < self.len() && !f(&self[start]) { start += 1; }
        if start == self.len() { return None; }
        let mut end = start + 1;
        while end < self.len() && f(&self[end]) { end += 1; }
        Some(start..end)
    }
    pub fn range(&self, range: Range<usize>) -> Set<T> {
        self.iter().skip(range.start).take(range.end - range.start).cloned().collect()
    }
    pub fn range_mut(&mut self, range: Range<usize>) -> Set<T> {
        let mut set = Set::new();
        let sz = range.end - range.start;
        set.data = Self::alloc(sz);
        if !set.data.is_null() && sz > 0 {
            set.len = sz;
            unsafe { std::ptr::copy_nonoverlapping(self.data.add(range.start), set.data, sz); }
        }
        set
    }
    pub fn from_vec(vec: Vec<T>) -> Set<T> {
        let mut set = Set::new();
        set.data = Self::alloc(vec.len());
        if !set.data.is_null() && vec.len() > 0 {
            set.len = vec.len();
            unsafe { std::ptr::copy_nonoverlapping(vec.as_ptr(), set.data, vec.len()); }
        }
        set
    }
}

impl<T: Clone + PartialEq> Drop for Set<T> {
    fn drop(&mut self) { self.free(); }
}
impl<T: Clone + PartialEq> Default for Set<T> {
    fn default() -> Self { Self::new() }
}
impl<T: Clone + PartialEq> Clone for Set<T> {
    fn clone(&self) -> Self { self.clone() }
}
impl<T: Clone + PartialEq> AsMut<[T]> for Set<T> {
    fn as_mut(&mut self) -> &mut [T] { unsafe { std::slice::from_raw_parts_mut(self.data, self.len) } }
}
impl<T: Clone + PartialEq> AsRef<[T]> for Set<T> {
    fn as_ref(&self) -> &[T] { unsafe { std::slice::from_raw_parts(self.data, self.len) } }
}
impl<T: Clone + PartialEq> Deref for Set<T> {
    type Target = [T];
    fn deref(&self) -> &Self::Target { unsafe { std::slice::from_raw_parts(self.data, self.len) } }
}
impl<T: Clone + PartialEq> DerefMut for Set<T> {
    fn deref_mut(&mut self) -> &mut Self::Target { unsafe { std::slice::from_raw_parts_mut(self.data, self.len) } }
}
impl<T: Clone + PartialEq + Ord> AddAssign for Set<T> {
    fn add_assign(&mut self, other: Self) { self.extend(&other); }
}
impl<T: Clone + PartialEq + Ord> Add for Set<T> {
    type Output = Set<T>;
    fn add(self, other: Self) -> Self::Output {
        let mut set = self.clone();
        set.extend(&other);
        set
    }
}
impl<T: Clone + PartialEq + Ord> SubAssign for Set<T> {
    fn sub_assign(&mut self, other: Self) { self.retain(|x| !other.contains(x)); }
}
impl<T: Clone + PartialEq + Ord> Sub for Set<T> {
    type Output = Set<T>;
    fn sub(self, other: Self) -> Self::Output {
        let mut set = self.clone();
        if set.len() > 0 {
            set.retain(|x| !other.contains(x));
        }
        set
    }
}
impl<T: Clone + PartialEq> Index<usize> for Set<T> {
    type Output = T;
    fn index(&self, i: usize) -> &Self::Output {
        if i < self.len { unsafe { &*self.data.add(i) } } else { panic!("index out of bounds") }
    }
}
impl<T: Clone + PartialEq> IndexMut<usize> for Set<T> {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        if i < self.len { unsafe { &mut *self.data.add(i) } } else { panic!("index out of bounds") }
    }
}
impl<T: Clone + PartialEq> RangeBounds<T> for Set<T> {
    fn start_bound(&self) -> Bound<&T> {
        if self.len == 0 { Bound::Unbounded } else { Bound::Included(&self[0]) }
    }
    fn end_bound(&self) -> Bound<&T> {
        if self.len == 0 { Bound::Unbounded } else { Bound::Included(&self[self.len - 1]) }
    }
}
impl<T: Clone + PartialEq> From<*mut T> for Set<T> {
    fn from(ptr: *mut T) -> Self {
        let mut set = Set::new();
        unsafe {
            if !ptr.is_null() {
                set.data = ptr;
                set.len = std::mem::size_of_val(&*ptr) / std::mem::size_of::<T>();
            }
        }
        set
    }
}
impl<T: Clone + PartialEq> From<*const T> for Set<T> {
    fn from(ptr: *const T) -> Self {
        let mut set = Set::new();
        unsafe {
            if !ptr.is_null() {
                set.data = ptr.cast_mut();
                set.len = std::mem::size_of_val(&*ptr) / std::mem::size_of::<T>();
            }
        }
        set
    }
}
impl<T: Clone + PartialEq> From<Vec<T>> for Set<T> {
    fn from(vec: Vec<T>) -> Self { Set::from_vec(vec) }
}
impl<T: Clone + PartialEq> FromIterator<T> for Set<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut set = Set::new();
        for item in iter { set.push(item); }
        set
    }
}
impl<T: Clone + PartialEq> std::iter::Extend<T> for Set<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        let mut items: Vec<T> = iter.into_iter().collect();
        if items.is_empty() { return; }
        let len = self.len + items.len();
        let data = Self::alloc(len);
        unsafe {
            if self.len > 0 {
                std::ptr::copy_nonoverlapping(self.data, data, self.len);
            }
            std::ptr::copy_nonoverlapping(items.as_ptr(), data.add(self.len), items.len());
        }
        self.free();
        self.data = data;
        self.len = len;
    }
}
impl<T: Clone + PartialEq + Eq + Ord> Ord for Set<T> {
    fn cmp(&self, other: &Self) -> Ordering { self.iter().cmp(other.iter()) }
}
impl<T: Clone + PartialEq + Eq + Ord> PartialOrd for Set<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(other)) }
}
impl Paths {
    pub fn new() -> Self {
        Self {
            paths: Vec::new(),
            max_depth: 10,
            min_depth: 1,
        }
    }

    pub fn get_at(&self, index: usize) -> Result<&PathBuf, IoError> {
        self.paths.get(index).ok_or_else(|| IoError::new(IoErrorKind::NotFound, "Path not found"))
    }

    pub fn set_at(&mut self, path: PathBuf, index: usize) -> Result<(), IoError> {
        if !path.exists() {
            return Err(IoError::new(IoErrorKind::NotFound, "Path not found"));
        }
        let count = path.components().count();
        if count < self.min_depth || count > self.max_depth {
            return Err(IoError::new(IoErrorKind::InvalidInput, "Path depth is out of range"));
        }
        if index >= self.paths.len() {
            self.paths.resize_with(index + 1, PathBuf::new);
        }
        self.paths[index] = path;
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
                return Err(IoError::new(IoErrorKind::InvalidInput, "Path depth is out of range"));
            }
            self.paths.push(path);
        }
        Ok(())
    }

    pub fn max_depth(&self) -> usize { self.max_depth }
    pub fn set_max_depth(&mut self, depth: usize) { self.max_depth = depth; }
    pub fn min_depth(&self) -> usize { self.min_depth }
    pub fn set_min_depth(&mut self, depth: usize) { self.min_depth = depth; }

    pub fn paths(&self) -> &[PathBuf] { &self.paths }

    pub fn set_paths(&mut self, paths: Vec<PathBuf>) -> Result<(), IoError> {
        let filtered: Vec<_> = paths
            .into_iter()
            .filter(|p| {
                let c = p.components().count();
                c >= self.min_depth && c <= self.max_depth
            })
            .collect();

        if filtered.is_empty() {
            return Err(IoError::new(IoErrorKind::InvalidInput, "No valid paths in input"));
        }
        self.paths = filtered;
        self.paths.sort_unstable();
        self.paths.dedup();
        Ok(())
    }

    pub fn read(&self, index: usize) -> Result<String, IoError> {
        let path = self.get_at(index)?;
        if !path.exists() || !path.is_file() {
            return Err(IoError::new(IoErrorKind::NotFound, "File not found or not a file"));
        }
        let mut buf = String::new();
        File::open(path)?.read_to_string(&mut buf)?;
        Ok(buf)
    }

    pub fn write(&self, index: usize, contents: &str) -> Result<(), IoError> {
        let path = self.get_at(index)?;
        if !path.exists() || !path.is_file() {
            return Err(IoError::new(IoErrorKind::InvalidInput, "Path not found or not a file"));
        }
        File::create(path)?.write_all(contents.as_bytes())?;
        Ok(())
    }

    pub fn read_all(&self) -> Result<Vec<String>, IoError> {
        self.paths.iter().map(|path| self.read(self.paths.iter().position(|x| x == path).unwrap())).collect()
    }

    pub fn write_all(&self, contents: &[String]) -> Result<(), IoError> {
        if contents.len() != self.paths.len() {
            return Err(IoError::new(IoErrorKind::InvalidInput, "Contents length does not match paths length"));
        }
        for (i, content) in contents.iter().enumerate() {
            self.write(i, content)?;
        }
        Ok(())
    }

    pub fn exists_count(&self) -> usize { self.paths.iter().filter(|p| p.exists()).count() }
    pub fn files_count(&self) -> usize { self.paths.iter().filter(|p| p.is_file()).count() }
    pub fn dirs_count(&self) -> usize { self.paths.iter().filter(|p| p.is_dir()).count() }
    pub fn symlinks_count(&self) -> usize { self.paths.iter().filter(|p| p.is_symlink()).count() }

    pub fn exists(&self) -> bool { self.paths.iter().all(|p| p.exists()) }
    pub fn is_all_files(&self) -> bool { !self.paths.is_empty() && self.paths.iter().all(|p| p.is_file()) }
    pub fn is_all_dirs(&self) -> bool { !self.paths.is_empty() && self.paths.iter().all(|p| p.is_dir()) }
    pub fn is_all_symlinks(&self) -> bool { !self.paths.is_empty() && self.paths.iter().all(|p| p.is_symlink()) }
    pub fn is_all_files_or_dirs(&self) -> bool {
        !self.paths.is_empty() && self.paths.iter().all(|p| p.is_file() || p.is_dir())
    }
    pub fn is_all_files_or_symlinks(&self) -> bool {
        !self.paths.is_empty() && self.paths.iter().all(|p| p.is_file() || p.is_symlink())
    }
    pub fn is_all_dirs_or_symlinks(&self) -> bool {
        !self.paths.is_empty() && self.paths.iter().all(|p| p.is_dir() || p.is_symlink())
    }
}

impl Index<usize> for Paths {
    type Output = PathBuf;
    fn index(&self, index: usize) -> &Self::Output { &self.paths[index] }
}
impl IndexMut<usize> for Paths {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output { &mut self.paths[index] }
}
impl<'a> Index<usize> for &'a Paths {
    type Output = PathBuf;
    fn index(&self, index: usize) -> &Self::Output { &self.paths[index] }
}
impl Deref for Paths {
    type Target = [PathBuf];
    fn deref(&self) -> &Self::Target { self.paths.as_slice() }
}
impl DerefMut for Paths {
    fn deref_mut(&mut self) -> &mut Self::Target { self.paths.as_mut_slice() }
}
impl From<Vec<PathBuf>> for Paths {
    fn from(paths: Vec<PathBuf>) -> Self {
        let mut pv = Self::new();
        pv.paths = paths;
        pv
    }
}
impl FromStr for Paths {
    type Err = IoError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let paths: Vec<PathBuf> = s
            .lines()
            .filter(|l| !l.trim().is_empty())
            .map(PathBuf::from)
            .collect();
        if !paths.is_empty() {
            Ok(Self::from(paths))
        } else {
            Err(IoError::new(IoErrorKind::InvalidInput, "No valid paths"))
        }
    }
}

impl fmt::Debug for Paths {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_struct("PathVec")
            .field("paths", &self.paths)
            .field("max_depth", &self.max_depth)
            .field("min_depth", &self.min_depth)
            .finish()
    }
}

impl Display for Paths {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        for (i, path) in self.paths.iter().enumerate() {
            if i > 0 { write!(f, "\n")?; }
            write!(f, "{}", path.display())?;
        }
        Ok(())
    }
}
