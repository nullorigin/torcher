pub mod v4;
pub mod v6;
use std::{
    cmp::Ordering,
    fmt::{Debug, Display, Formatter, Result},
    fs::File,
    hash::{Hash, Hasher},
    io::Read,
    num::ParseIntError,
    ops::{Index, IndexMut},
    path::Path,
    str::FromStr,
    sync::LazyLock,
    usize,
};

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    None,
    In,
    Out,
    Fwd,
}

impl Direction {
    pub fn new(i: bool, o: bool) -> Self {
        match (i, o) {
            (true, true) => Direction::Fwd,
            (true, false) => Direction::In,
            (false, true) => Direction::Out,
            (false, false) => Direction::None,
        }
    }
    pub fn default() -> Self {
        Direction::None
    }
    pub fn as_ref(&self) -> &Self {
        self
    }
    pub fn as_mut(&mut self) -> &mut Self {
        self
    }
    pub fn as_str(&self) -> &'static str {
        match self {
            Direction::None => "none",
            Direction::In => "in",
            Direction::Out => "out",
            Direction::Fwd => "fwd",
        }
    }

    pub fn from_str(s: &str) -> Direction {
        let s = s.trim().to_ascii_lowercase();
        match s.as_str() {
            "in" | "i" | "input" => Direction::In,
            "out" | "o" | "output" => Direction::Out,
            "fwd" | "f" | "forward" => Direction::Fwd,
            _ => Direction::None,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Direction::None => "none".to_string(),
            Direction::In => "in".to_string(),
            Direction::Out => "out".to_string(),
            Direction::Fwd => "fwd".to_string(),
        }
    }
    pub fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            Direction::None => 0.hash(state),
            Direction::In => 1.hash(state),
            Direction::Out => 2.hash(state),
            Direction::Fwd => 3.hash(state),
        }
    }
    pub fn cmp(&self, other: &Direction) -> Ordering {
        match (self, other) {
            (Direction::None, Direction::None) => Ordering::Equal,
            (Direction::In, Direction::In) => Ordering::Equal,
            (Direction::Out, Direction::Out) => Ordering::Equal,
            (Direction::Fwd, Direction::Fwd) => Ordering::Equal,
            (Direction::None, Direction::In) => Ordering::Less,
            (Direction::None, Direction::Out) => Ordering::Less,
            (Direction::None, Direction::Fwd) => Ordering::Less,
            (Direction::In, Direction::Out) => Ordering::Less,
            (Direction::In, Direction::Fwd) => Ordering::Less,
            (Direction::Out, Direction::Fwd) => Ordering::Less,
            _ => Ordering::Greater,
        }
    }
    pub fn eq(&self, other: &Direction) -> bool {
        self == other
    }
    pub fn ne(&self, other: &Direction) -> bool {
        self != other
    }
    pub fn le(&self, other: &Direction) -> bool {
        self <= other
    }
    pub fn ge(&self, other: &Direction) -> bool {
        self >= other
    }
    pub fn lt(&self, other: &Direction) -> bool {
        self < other
    }
    pub fn gt(&self, other: &Direction) -> bool {
        self > other
    }
    pub fn to_bool(&self) -> (bool, bool) {
        match self {
            Direction::None => (false, false),
            Direction::In => (true, false),
            Direction::Out => (false, true),
            Direction::Fwd => (true, true),
        }
    }
    pub fn is_none(&self) -> bool {
        matches!(self, Direction::None)
    }
    pub fn is_in(&self) -> bool {
        matches!(self, Direction::In)
    }
    pub fn is_out(&self) -> bool {
        matches!(self, Direction::Out)
    }
    pub fn is_fwd(&self) -> bool {
        matches!(self, Direction::Fwd)
    }
    pub fn is_any(&self) -> bool {
        matches!(self, Direction::Fwd | Direction::Out | Direction::In)
    }
}
impl AsMut<Direction> for Direction {
    fn as_mut(&mut self) -> &mut Direction {
        self
    }
}
impl AsRef<Direction> for Direction {
    fn as_ref(&self) -> &Direction {
        self
    }
}
impl Default for Direction {
    fn default() -> Self {
        Direction::None
    }
}
impl Display for Direction {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.to_string())
    }
}
impl Eq for Direction {}
impl From<Direction> for (bool, bool) {
    fn from(d: Direction) -> (bool, bool) {
        match d {
            Direction::None => (false, false),
            Direction::In => (true, false),
            Direction::Out => (false, true),
            Direction::Fwd => (true, true),
        }
    }
}
impl From<Direction> for String {
    fn from(d: Direction) -> Self {
        d.to_string()
    }
}
impl From<Direction> for u8 {
    fn from(d: Direction) -> Self {
        match d {
            Direction::None => 0u8,
            Direction::In => 1u8,
            Direction::Out => 2u8,
            Direction::Fwd => 3u8,
        }
    }
}
impl From<Direction> for usize {
    fn from(d: Direction) -> Self {
        match d {
            Direction::None => 0usize,
            Direction::In => 1usize,
            Direction::Out => 2usize,
            Direction::Fwd => 3usize,
        }
    }
}
impl From<u8> for Direction {
    fn from(d: u8) -> Self {
        match d {
            0u8 => Direction::None,
            1u8 => Direction::In,
            2u8 => Direction::Out,
            3u8 => Direction::Fwd,
            _ => Direction::None,
        }
    }
}
impl From<usize> for Direction {
    fn from(index: usize) -> Self {
        match index {
            0usize => Direction::None,
            1usize => Direction::In,
            2usize => Direction::Out,
            3usize => Direction::Fwd,
            _ => Direction::None,
        }
    }
}
impl FromStr for Direction {
    type Err = ParseIntError;
    fn from_str(s: &str) -> std::result::Result<Direction, ParseIntError> {
        match s.trim().to_ascii_lowercase().as_str() {
            "in" | "i" | "input" => Ok(Direction::In),
            "out" | "o" | "output" => Ok(Direction::Out),
            "fwd" | "f" | "forward" => Ok(Direction::Fwd),
            _ => Ok(Direction::None),
        }
    }
}
impl Hash for Direction {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            Direction::None => 0.hash(state),
            Direction::In => 1.hash(state),
            Direction::Out => 2.hash(state),
            Direction::Fwd => 3.hash(state),
        }
    }
}
impl Index<usize> for Direction {
    type Output = Self;
    fn index(&self, index: usize) -> &Self {
        match index {
            0 => &Direction::None,
            1 => &Direction::In,
            2 => &Direction::Out,
            3 => &Direction::Fwd,
            _ => &Direction::None,
        }
    }
}
impl IndexMut<usize> for Direction {
    fn index_mut(&mut self, index: usize) -> &mut Self {
        *self = Direction::from(index);
        self
    }
}
impl Ord for Direction {
    fn cmp(&self, other: &Direction) -> Ordering {
        self.cmp(other)
    }
}
impl PartialEq for Direction {
    fn eq(&self, other: &Direction) -> bool {
        self == other
    }
}
impl PartialOrd for Direction {
    fn partial_cmp(&self, other: &Direction) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Family {
    None = 0,
    V4 = 1,
    V6 = 2,
    Any = 3,
}
impl Family {
    pub fn new(v4: bool, v6: bool) -> Self {
        match (v4, v6) {
            (true, true) => Family::Any,
            (true, false) => Family::V4,
            (false, true) => Family::V6,
            (false, false) => Family::None,
        }
    }
    pub fn default() -> Self {
        Family::None
    }
    pub fn as_str(&self) -> &'static str {
        match self {
            Family::None => "none",
            Family::V4 => "v4",
            Family::V6 => "v6",
            Family::Any => "any",
        }
    }
    pub fn from_str(s: &str) -> Self {
        match s.trim_ascii().to_lowercase().as_str() {
            "none" | "" => Family::None,
            "v4" | "4" | "ipv4" | "ip4" => Family::V4,
            "v6" | "6" | "ipv6" | "ip6" => Family::V6,
            "any" | "a" | "all" => Family::Any,
            _ => Family::None,
        }
    }
    pub fn to_string(&self) -> String {
        match self {
            Family::None => "none".to_string(),
            Family::V4 => "v4".to_string(),
            Family::V6 => "v6".to_string(),
            Family::Any => "any".to_string(),
        }
    }
    pub fn from_bool(v4: bool, v6: bool) -> Family {
        match (v4, v6) {
            (false, false) => Family::None,
            (true, false) => Family::V4,
            (false, true) => Family::V6,
            (true, true) => Family::Any,
        }
    }
    pub fn to_bool(&self) -> (bool, bool) {
        match self {
            Family::None => (false, false),
            Family::V4 => (true, false),
            Family::V6 => (false, true),
            Family::Any => (true, true),
        }
    }
    pub fn index(&self) -> usize {
        match self {
            Family::None => 0usize,
            Family::V4 => 1usize,
            Family::V6 => 2usize,
            Family::Any => 3usize,
        }
    }
    pub fn cmp(&self, other: &Family) -> Ordering {
        (self.index()).cmp(&other.index())
    }
    pub fn eq(&self, other: &Family) -> bool {
        self.index() == other.index()
    }
    pub fn ne(&self, other: &Family) -> bool {
        self.index() != other.index()
    }
    pub fn le(&self, other: &Family) -> bool {
        self.index() <= other.index()
    }
    pub fn ge(&self, other: &Family) -> bool {
        self.index() >= other.index()
    }
    pub fn lt(&self, other: &Family) -> bool {
        self.index() < other.index()
    }
    pub fn gt(&self, other: &Family) -> bool {
        self.index() > other.index()
    }
    pub fn is_v4(&self) -> bool {
        *self == Family::V4 || *self == Family::Any
    }
    pub fn is_v6(&self) -> bool {
        *self == Family::V6 || *self == Family::Any
    }
    pub fn is_any(&self) -> bool {
        *self == Family::Any
    }
    pub fn is_none(&self) -> bool {
        *self == Family::None
    }
}
impl AsRef<str> for Family {
    fn as_ref(&self) -> &str {
        match self {
            Family::None => "none",
            Family::V4 => "v4",
            Family::V6 => "v6",
            Family::Any => "any",
        }
    }
}
impl Default for Family {
    fn default() -> Self {
        Family::None
    }
}
impl Display for Family {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.to_string())
    }
}
impl Eq for Family {}
impl From<(bool, bool)> for Family {
    fn from((v4, v6): (bool, bool)) -> Self {
        Family::from_bool(v4, v6)
    }
}
impl From<Family> for (bool, bool) {
    fn from(f: Family) -> (bool, bool) {
        f.to_bool()
    }
}
impl From<Family> for String {
    fn from(f: Family) -> Self {
        f.to_string()
    }
}
impl From<Family> for u8 {
    fn from(f: Family) -> Self {
        match f {
            Family::None => 0,
            Family::V4 => 1,
            Family::V6 => 2,
            Family::Any => 3,
        }
    }
}
impl From<Family> for usize {
    fn from(f: Family) -> Self {
        match f {
            Family::None => 0,
            Family::V4 => 1,
            Family::V6 => 2,
            Family::Any => 3,
        }
    }
}
impl From<Ip> for Family {
    fn from(ip: Ip) -> Self {
        match ip {
            Ip::Any(_) => Family::Any,
            Ip::V4(_) => Family::V4,
            Ip::V6(_) => Family::V6,
            Ip::None => Family::None,
        }
    }
}
impl From<u8> for Family {
    fn from(f: u8) -> Self {
        match f {
            0 => Family::None,
            1 => Family::V4,
            2 => Family::V6,
            3 => Family::Any,
            _ => Family::None,
        }
    }
}
impl From<usize> for Family {
    fn from(index: usize) -> Self {
        match index {
            0 => Family::None,
            1 => Family::V4,
            2 => Family::V6,
            3 => Family::Any,
            _ => Family::None,
        }
    }
}
impl Hash for Family {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.index().hash(state);
    }
}
impl Ord for Family {
    fn cmp(&self, other: &Family) -> Ordering {
        self.index().cmp(&other.index())
    }
}
impl PartialEq for Family {
    fn eq(&self, other: &Family) -> bool {
        self.index() == other.index()
    }
}
impl PartialOrd for Family {
    fn partial_cmp(&self, other: &Family) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl FromStr for Family {
    type Err = ParseIntError;
    fn from_str(s: &str) -> std::result::Result<Family, ParseIntError> {
        match s {
            "none" | "" => Ok(Family::None),
            "v4" | "4" | "ipv4" | "ip4" => Ok(Family::V4),
            "v6" | "6" | "ipv6" | "ip6" => Ok(Family::V6),
            "any" | "a" | "all" => Ok(Family::Any),
            _ => Ok(Family::None),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Ip {
    Any(Vec<u8>),
    V4([u8; 5]),
    V6([u8; 17]),
    None,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Mask {
    Any(Vec<u8>),
    V4([u8; 4]),
    V6([u8; 16]),
    None,
}
impl Mask {
    pub fn new(family: Family) -> Self {
        match family {
            Family::Any => Self::Any(Vec::new()),
            Family::V4 => Self::V4([0; 4]),
            Family::V6 => Self::V6([0; 16]),
            Family::None => Self::None,
        }
    }

    pub fn clone(&self) -> Self {
        match self {
            Mask::Any(mask) => Mask::Any(mask.clone()),
            Mask::V4(mask) => Mask::V4(*mask),
            Mask::V6(mask) => Mask::V6(*mask),
            Mask::None => Mask::None,
        }
    }

    pub fn len(&self) -> usize {
        match self {
            Mask::Any(v) => v.len(),
            Mask::V4(_) => 4,
            Mask::V6(_) => 16,
            Mask::None => 0,
        }
    }

    pub fn family(&self) -> Family {
        match self {
            Mask::Any(_) => Family::Any,
            Mask::V4(_) => Family::V4,
            Mask::V6(_) => Family::V6,
            Mask::None => Family::None,
        }
    }

    pub fn to_vec(&self) -> Vec<u8> {
        match self {
            Mask::Any(v) => v.clone(),
            Mask::V4(a) => a.to_vec(),
            Mask::V6(a) => a.to_vec(),
            Mask::None => Vec::new(),
        }
    }

    pub fn from_vec(family: Family, v: Vec<u8>) -> Self {
        match family {
            Family::Any => {
                if v.is_empty() {
                    Mask::None
                } else {
                    Mask::Any(v)
                }
            }
            Family::V4 => {
                if v.len() == 4 {
                    let mut arr = [0u8; 4];
                    arr.copy_from_slice(&v);
                    Mask::V4(arr)
                } else {
                    Mask::None
                }
            }
            Family::V6 => {
                if v.len() == 16 {
                    let mut arr = [0u8; 16];
                    arr.copy_from_slice(&v);
                    Mask::V6(arr)
                } else {
                    Mask::None
                }
            }
            Family::None => Mask::None,
        }
    }

    /// Convert mask to its short prefix length form, e.g. [255,255,255,0] -> 24.
    pub fn to_pfxlen(&self) -> Option<u8> {
        match self {
            Mask::None => return None,
            Mask::V4(a) => {
                let bits = a
                    .iter()
                    .flat_map(|b| (0..8).rev().map(move |i| (b >> i) & 1))
                    .collect::<Vec<_>>();
                let mut prefixlen = 0u8;
                for (i, bit) in bits.iter().enumerate() {
                    if *bit == 1 {
                        prefixlen += 1;
                    } else {
                        if bits.iter().skip(i).any(|b| *b == 1) {
                            return None; // not a valid contiguous mask
                        }
                        break;
                    }
                }
                Some(prefixlen)
            }
            Mask::V6(a) => {
                let bits = a
                    .iter()
                    .flat_map(|b| (0..8).rev().map(move |i| (b >> i) & 1))
                    .collect::<Vec<_>>();
                let mut prefixlen = 0u8;
                for (i, bit) in bits.iter().enumerate() {
                    if *bit == 1 {
                        prefixlen += 1;
                    } else {
                        if bits.iter().skip(i).any(|b| *b == 1) {
                            return None; // not a valid contiguous mask
                        }
                        break;
                    }
                }
                Some(prefixlen)
            }
            Mask::Any(a) => {
                let bits = a
                    .iter()
                    .flat_map(|b| (0..8).rev().map(move |i| (b >> i) & 1))
                    .collect::<Vec<_>>();
                let mut prefixlen = 0u8;
                for (i, bit) in bits.iter().enumerate() {
                    if *bit == 1 {
                        prefixlen += 1;
                    } else {
                        if bits.iter().skip(i).any(|b| *b == 1) {
                            return None; // not a valid contiguous mask
                        }
                        break;
                    }
                }
                Some(prefixlen)
            }
        }
    }

    /// Construct a mask from a prefixlen. E.g. (Family::V4, 24) => [255,255,255,0]
    pub fn from_pfxlen(family: Family, prefixlen: u8) -> Self {
        match family {
            Family::Any => match prefixlen {
                0 => Mask::None,
                32 => Mask::Any(vec![0xFF; 4]),
                128 => Mask::Any(vec![0xFF; 16]),
                _ => {
                    if prefixlen > 128 {
                        return Mask::None;
                    }
                    let mut arr = vec![0u8; 16];
                    for i in 0..16 {
                        let remain = prefixlen.saturating_sub(i * 8);
                        arr[i as usize] = if remain >= 8 {
                            0xFF
                        } else if remain > 0 {
                            !(0xFF >> remain)
                        } else {
                            0
                        };
                    }
                    Mask::Any(arr)
                }
            },
            Family::V4 => {
                if prefixlen > 32 {
                    return Mask::None;
                }
                let mut arr = [0u8; 4];
                for i in 0..4 {
                    let remain = prefixlen.saturating_sub(i * 8);
                    arr[i as usize] = if remain >= 8 {
                        0xFF
                    } else if remain > 0 {
                        !(0xFF >> remain)
                    } else {
                        0
                    };
                }
                Mask::V4(arr)
            }
            Family::V6 => {
                if prefixlen > 128 {
                    return Mask::None;
                }
                let mut arr = [0u8; 16];
                for i in 0..16 {
                    let remain = prefixlen.saturating_sub(i * 8);
                    arr[i as usize] = if remain >= 8 {
                        0xFF
                    } else if remain > 0 {
                        !(0xFF >> remain)
                    } else {
                        0
                    };
                }
                Mask::V6(arr)
            }
            Family::None => Mask::None,
        }
    }

    pub fn from_str(s: &str) -> Self {
        let s = s.trim();
        if s.is_empty() {
            return Mask::None;
        }
        if s.contains('.') && !s.contains(':') {
            let mut octets = [0u8; 4];
            let splits: Vec<&str> = s.split('.').collect();
            if splits.len() != 4 {
                return Mask::None;
            }
            for (i, ss) in splits.into_iter().enumerate() {
                if ss.is_empty() {
                    return Mask::None;
                }
                match u8::from_str(ss) {
                    Ok(val) => octets[i] = val,
                    Err(_) => return Mask::None,
                }
            }
            Mask::V4(octets)
        } else if s.contains(':') && !s.contains('.') {
            let splits: Vec<&str> = s.split(':').collect();
            if splits.len() != 16 && splits.len() != 8 {
                return Mask::None;
            }
            let mut octets = [0u8; 16];
            for (i, ss) in splits.into_iter().enumerate() {
                if ss.is_empty() {
                    octets[i] = 0;
                } else {
                    match u8::from_str(ss) {
                        Ok(val) => octets[i] = val,
                        Err(_) => return Mask::None,
                    }
                }
            }
            Mask::V6(octets)
        } else {
            Mask::None
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Mask::Any(v) => {
                if v.is_empty() {
                    "".to_string()
                } else if v.len() == 4 {
                    v.iter()
                        .map(|b| b.to_string())
                        .collect::<Vec<_>>()
                        .join(".")
                } else if v.len() == 16 {
                    v.iter()
                        .map(|b| b.to_string())
                        .collect::<Vec<_>>()
                        .join(":")
                } else {
                    "".to_string()
                }
            }
            Mask::V4(arr) => arr
                .iter()
                .map(|b| b.to_string())
                .collect::<Vec<_>>()
                .join("."),
            Mask::V6(arr) => arr
                .chunks(2)
                .map(|chunk| {
                    chunk
                        .iter()
                        .map(|b| format!("{:02X}", b))
                        .collect::<String>()
                })
                .collect::<Vec<_>>()
                .join(":"),
            Mask::None => "".to_string(),
        }
    }

    pub fn is_valid_str(s: &str, family: Family) -> bool {
        match family {
            Family::Any => Self::is_valid_ip4_str(s) || Self::is_valid_ip6_str(s),
            Family::V4 => Self::is_valid_ip4_str(s),
            Family::V6 => Self::is_valid_ip6_str(s),
            Family::None => false,
        }
    }

    fn is_valid_ip4_str(s: &str) -> bool {
        let parts: Vec<&str> = s.trim().split('.').collect();
        if parts.len() != 4 {
            return false;
        }
        for part in parts {
            if part.is_empty() || u8::from_str(part).is_err() {
                return false;
            }
        }
        true
    }

    fn is_valid_ip6_str(s: &str) -> bool {
        let parts: Vec<&str> = s.trim().split(':').collect();
        if parts.len() != 8 && parts.len() != 16 {
            return false;
        }
        for part in parts {
            if !part.is_empty() && u8::from_str(part).is_err() {
                return false;
            }
        }
        true
    }

    pub fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.to_string())
    }

    pub fn is_any(&self) -> bool {
        matches!(self, Mask::Any(_))
    }
    pub fn is_ip4(&self) -> bool {
        matches!(self, Mask::V4(_))
    }

    pub fn is_ip6(&self) -> bool {
        matches!(self, Mask::V6(_))
    }

    pub fn is_none(&self) -> bool {
        matches!(self, Mask::None)
    }
}
impl Default for Mask {
    fn default() -> Self {
        Mask::None
    }
}

impl Display for Mask {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.to_string())
    }
}

impl From<Mask> for Vec<u8> {
    fn from(value: Mask) -> Self {
        match value {
            Mask::Any(v) => v,
            Mask::V4(arr) => arr.to_vec(),
            Mask::V6(arr) => arr.to_vec(),
            Mask::None => vec![],
        }
    }
}

impl From<Vec<u8>> for Mask {
    fn from(value: Vec<u8>) -> Self {
        match value.len() {
            4 => Mask::V4(value.try_into().unwrap()),
            16 => Mask::V6(value.try_into().unwrap()),
            _ => Mask::None,
        }
    }
}

impl Hash for Mask {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            Mask::Any(v) => v.hash(state),
            Mask::V4(addr) => addr.hash(state),
            Mask::V6(addr) => addr.hash(state),
            Mask::None => 0.hash(state),
        }
    }
}

impl Index<usize> for Mask {
    type Output = u8;
    fn index(&self, index: usize) -> &Self::Output {
        match self {
            Mask::Any(mask) => &mask[index],
            Mask::V4(mask) => &mask[index],
            Mask::V6(mask) => &mask[index],
            Mask::None => panic!("Mask::None has no elements"),
        }
    }
}

impl IndexMut<usize> for Mask {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match self {
            Mask::Any(mask) => &mut mask[index],
            Mask::V4(mask) => &mut mask[index],
            Mask::V6(mask) => &mut mask[index],
            Mask::None => panic!("Mask::None has no elements"),
        }
    }
}

impl Ord for Mask {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Mask::Any(a), Mask::Any(b)) => a.cmp(b),
            (Mask::Any(a), Mask::V4(b)) => a.cmp(&b.to_vec()),
            (Mask::Any(a), Mask::V6(b)) => a.cmp(&b.to_vec()),
            (Mask::V4(a), Mask::Any(b)) => a.to_vec().cmp(&b),
            (Mask::V4(a), Mask::V4(b)) => a.cmp(b),
            (Mask::V4(a), Mask::V6(b)) => a.to_vec().cmp(&b.to_vec()),
            (Mask::V6(a), Mask::Any(b)) => a.to_vec().cmp(&b),
            (Mask::V6(a), Mask::V4(b)) => a.to_vec().cmp(&b.to_vec()),
            (Mask::V6(a), Mask::V6(b)) => a.cmp(b),
            (Mask::None, Mask::None) => Ordering::Equal,
            (Mask::None, _) => Ordering::Less,
            (_, Mask::None) => Ordering::Greater,
        }
    }
}

impl PartialOrd for Mask {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ip {
    pub fn new(family: Family) -> Self {
        match family {
            Family::Any => Ip::Any(Vec::new()),
            Family::V4 => Ip::V4([0; 5]),
            Family::V6 => Ip::V6([0; 17]),
            Family::None => Ip::None,
        }
    }

    pub fn family(&self) -> Family {
        match self {
            Ip::Any(_) => Family::Any,
            Ip::V4(_) => Family::V4,
            Ip::V6(_) => Family::V6,
            Ip::None => Family::None,
        }
    }

    pub fn len(&self) -> usize {
        match self {
            Ip::Any(a) => a.len(),
            Ip::V4(_) => 5,
            Ip::V6(_) => 17,
            Ip::None => 0,
        }
    }

    pub fn address(&self) -> Vec<u8> {
        match self {
            Ip::Any(a) => a.clone(),
            Ip::V4(a) => a[..3].to_vec(),
            Ip::V6(a) => a[..15].to_vec(),
            Ip::None => vec![],
        }
    }

    pub fn subnet(&self) -> u8 {
        match self {
            Ip::Any(a) => {
                if a.len() == 5 {
                    a[4]
                } else if a.len() == 17 {
                    a[16]
                } else {
                    0
                }
            }
            Ip::V4(a) => a[4],
            Ip::V6(a) => a[16],
            Ip::None => 0,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            Ip::Any(a) => a.clone(),
            Ip::V4(a) => a.to_vec(),
            Ip::V6(a) => a.to_vec(),
            Ip::None => vec![],
        }
    }

    pub fn from_bytes(bytes: Vec<u8>) -> Self {
        match bytes.len() {
            5 => {
                return Ip::V4([bytes[0], bytes[1], bytes[2], bytes[3], bytes[4]]);
            }
            17 => {
                return Ip::V6([
                    bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
                    bytes[8], bytes[9], bytes[10], bytes[11], bytes[12], bytes[13], bytes[14],
                    bytes[15], bytes[16],
                ]);
            }
            _ => Ip::None,
        }
    }
    pub fn to_string(&self) -> String {
        match self {
            Ip::Any(a) => {
                if a.len() == 5 {
                    let subnet = if a[4] < 32 {
                        format!("/{}", a[4])
                    } else {
                        "".to_string()
                    };
                    format!("{}.{}.{}.{}{}", a[0], a[1], a[2], a[3], subnet)
                } else if a.len() == 17 {
                    let addr = (0..8)
                        .map(|i| format!("{:x}", ((a[i * 2] as u16) << 8) | (a[i * 2 + 1] as u16)))
                        .collect::<Vec<_>>()
                        .join(":");
                    let subnet = if a[16] < 128 {
                        format!("/{}", a[16])
                    } else {
                        "".to_string()
                    };
                    format!("{}{}", addr, subnet)
                } else {
                    "".to_string()
                }
            }
            Ip::V4(a) => {
                let subnet = if a[4] < 32 {
                    format!("/{}", a[4])
                } else {
                    "".to_string()
                };
                format!("{}.{}.{}.{}{}", a[0], a[1], a[2], a[3], subnet)
            }
            Ip::V6(a) => {
                let addr = (0..8)
                    .map(|i| format!("{:x}", ((a[i * 2] as u16) << 8) | (a[i * 2 + 1] as u16)))
                    .collect::<Vec<_>>()
                    .join(":");
                if a[16] < 128 {
                    format!("{}/{}", addr, a[16])
                } else {
                    addr
                }
            }
            Ip::None => "".to_string(),
        }
    }

    fn from_str(s: &str) -> std::result::Result<Ip, ParseIntError> {
        let s = s.trim();
        if s.is_empty() {
            return Ok(Ip::None);
        }

        // Try IPv4
        if s.contains('.') && !s.contains(':') {
            let (addr_part, subnet_part) = s.split_once('/').unwrap_or((s, ""));
            // Accept short forms: "1", "1.2", "1.2.3", "1.2.3.4"
            let mut octets = [0u8; 4];
            let parts: Vec<&str> = addr_part.split('.').collect();
            for i in 0..parts.len().min(4) {
                octets[i] = u8::from_str(parts[i])?;
            }
            // Fill trailing octets with 0 if missing
            for i in parts.len()..4 {
                octets[i] = 0;
            }
            let subnet = if !subnet_part.is_empty() {
                u8::from_str(subnet_part)?.min(32)
            } else {
                32
            };
            let mut ip = [0u8; 5];
            ip[..4].copy_from_slice(&octets);
            ip[4] = subnet;
            return Ok(Ip::V4(ip));
        }

        // Try IPv6
        if s.contains(':') {
            let (addr_part, subnet_part) = s.split_once('/').unwrap_or((s, ""));
            // Expand zero compression and parse
            let parts = addr_part.split("::").collect::<Vec<_>>();
            let mut addr_bytes = [0u8; 16];
            let mut segs: Vec<u16> = Vec::with_capacity(8);

            if parts.len() == 1 {
                // No zero compression
                for part in addr_part.split(':') {
                    if !part.is_empty() {
                        segs.push(u16::from_str_radix(part, 16)?);
                    }
                }
            } else if parts.len() == 2 {
                // There is zero compression
                let left: Vec<&str> = parts[0].split(':').filter(|p| !p.is_empty()).collect();
                let right: Vec<&str> = parts[1].split(':').filter(|p| !p.is_empty()).collect();
                for part in &left {
                    segs.push(u16::from_str_radix(part, 16)?);
                }
                // Fill zeros
                let zeros = 8 - (left.len() + right.len());
                for _ in 0..zeros {
                    segs.push(0);
                }
                for part in &right {
                    segs.push(u16::from_str_radix(part, 16)?);
                }
            } else {
                // Multiple "::" is not valid
                return Ok(Ip::None);
            }
            if segs.len() != 8 {
                // Try to parse IPv4-mapped IPv6, e.g. ::ffff:192.168.1.1
                if let Some(_last) = segs.last().copied() {
                    if segs.len() == 6 {
                        // The last segment should be an IPv4 address in dotted form
                        if let Some(ipv4_str) = addr_part.rsplitn(1, ':').next() {
                            if ipv4_str.contains('.') {
                                let ipv4 = Ip::from_str(ipv4_str)?;
                                let octets = ipv4.to_bytes();
                                segs.push(((octets[0] as u16) << 8) | (octets[1] as u16));
                                segs.push(((octets[2] as u16) << 8) | (octets[3] as u16));
                            }
                        }
                    }
                }
            }
            if segs.len() != 8 {
                return Ok(Ip::None);
            }
            for (i, seg) in segs.iter().enumerate() {
                addr_bytes[i * 2] = (seg >> 8) as u8;
                addr_bytes[i * 2 + 1] = (seg & 0xFF) as u8;
            }
            let subnet = if !subnet_part.is_empty() {
                u8::from_str(subnet_part)?.min(128)
            } else {
                128
            };
            let mut ip = [0u8; 17];
            ip[..16].copy_from_slice(&addr_bytes);
            ip[16] = subnet;
            return Ok(Ip::V6(ip));
        }

        // Ip::Any
        if s.eq_ignore_ascii_case("any") {
            return Ok(Ip::Any(vec![0u8; 5]));
        }

        Ok(Ip::None)
    }

    pub fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            Ip::Any(a) => a.hash(state),
            Ip::V4(addr) => addr.hash(state),
            Ip::V6(addr) => addr.hash(state),
            Ip::None => 0.hash(state),
        }
    }

    pub fn index(&self, index: usize) -> &u8 {
        match self {
            Ip::Any(a) => &a[index],
            Ip::V4(addr) => &addr[index],
            Ip::V6(addr) => &addr[index],
            Ip::None => panic!("Ip::None has no elements"),
        }
    }

    pub fn index_mut(&mut self, index: usize) -> &mut u8 {
        match self {
            Ip::Any(a) => &mut a[index],
            Ip::V4(addr) => &mut addr[index],
            Ip::V6(addr) => &mut addr[index],
            Ip::None => panic!("Ip::None has no elements"),
        }
    }
    pub fn is_any(&self) -> bool {
        matches!(self, Ip::Any(_))
    }
    pub fn is_ip4(&self) -> bool {
        matches!(self, Ip::V4(_))
    }

    pub fn is_ip6(&self) -> bool {
        matches!(self, Ip::V6(_))
    }

    pub fn is_none(&self) -> bool {
        matches!(self, Ip::None)
    }

    pub fn is_valid_char(&self, c: char) -> bool {
        match self {
            Ip::Any(a) => {
                if a.len() == 5 {
                    c.is_ascii_digit() || c == '.' || c == '/'
                } else if a.len() == 17 {
                    c.is_ascii_hexdigit() || c == ':' || c == '/'
                } else {
                    false
                }
            }
            Ip::V4(_) => c.is_ascii_digit() || c == '.' || c == '/',
            Ip::V6(_) => c.is_ascii_hexdigit() || c == ':' || c == '/',
            Ip::None => false,
        }
    }

    pub fn is_valid_str(s: &str, family: Family) -> bool {
        match family {
            Family::Any => Self::is_valid_v4_str(s) || Self::is_valid_v6_str(s),
            Family::V4 => Self::is_valid_v4_str(s),
            Family::V6 => Self::is_valid_v6_str(s),
            Family::None => false,
        }
    }

    fn is_valid_v4_str(s: &str) -> bool {
        if s.is_empty() {
            return false;
        }
        if s.contains('.') && !s.contains(':') {
            let mut dot_count = 0;
            let mut slash_count = 0;
            let mut prev = ' ';
            for (i, c) in s.char_indices() {
                if (c == '.' && (i == 0 || i == s.len() - 1))
                    || (c == '/' && (i < 7 || i == s.len() - 1))
                {
                    return false;
                } else if c == '.' {
                    dot_count += 1;
                    if dot_count > 3 || prev == '.' || prev == '/' {
                        return false;
                    }
                } else if c == '/' {
                    slash_count += 1;
                    if slash_count > 1 || prev == '.' {
                        return false;
                    }
                }
                prev = c;
            }
            if dot_count != 3 {
                return false;
            }
        }
        true
    }

    fn is_valid_v6_str(s: &str) -> bool {
        if s.is_empty() {
            return false;
        }
        let s = s.trim();
        if s.contains(':') && !s.contains('.') {
            let mut colon_count = 0;
            let mut slash_count = 0;
            let mut prev = ' ';
            for (i, c) in s.char_indices() {
                if (c == ':' && (i == 0 || i == s.len() - 1))
                    || (c == '/' && (i < 39 || i == s.len() - 1))
                {
                    return false;
                } else if c == ':' {
                    colon_count += 1;
                    if colon_count > 7 || prev == '/' {
                        return false;
                    }
                } else if c == '/' {
                    slash_count += 1;
                    if slash_count > 1 || prev == '/' {
                        return false;
                    }
                }
                prev = c;
            }
            if colon_count < 2 {
                return false;
            }
        }
        true
    }

    pub fn broadcast(&self) -> Self {
        match self {
            Ip::Any(a) => {
                if a.len() == 5 {
                    let subnet = a[4].min(32);
                    let addr = u32::from_be_bytes([a[0], a[1], a[2], a[3]]);
                    let mask = u32::MAX << (32 - subnet);
                    let broadcast_addr: [u8; 4] = (addr | !mask).to_be_bytes();
                    return Ip::Any(vec![
                        broadcast_addr[0],
                        broadcast_addr[1],
                        broadcast_addr[2],
                        broadcast_addr[3],
                        subnet,
                    ]);
                } else if a.len() == 17 {
                    let subnet = a[16].min(128);
                    let addr = u128::from_be_bytes([
                        a[0], a[1], a[2], a[3], a[4], a[5], a[6], a[7], a[8], a[9], a[10], a[11],
                        a[12], a[13], a[14], a[15],
                    ]);
                    let mask = u128::MAX << (128 - subnet);
                    let broadcast_addr: [u8; 16] = (addr | !mask).to_be_bytes();
                    return Ip::Any(vec![
                        broadcast_addr[0],
                        broadcast_addr[1],
                        broadcast_addr[2],
                        broadcast_addr[3],
                        broadcast_addr[4],
                        broadcast_addr[5],
                        broadcast_addr[6],
                        broadcast_addr[7],
                        broadcast_addr[8],
                        broadcast_addr[9],
                        broadcast_addr[10],
                        broadcast_addr[11],
                        broadcast_addr[12],
                        broadcast_addr[13],
                        broadcast_addr[14],
                        broadcast_addr[15],
                        subnet,
                    ]);
                }
                return Ip::None;
            }
            Ip::V4(ip) => {
                let subnet = ip[4].min(32);
                let addr = u32::from_be_bytes([ip[0], ip[1], ip[2], ip[3]]);
                let mask = u32::MAX << (32 - subnet);
                let broadcast_addr: [u8; 4] = (addr | !mask).to_be_bytes();
                return Ip::V4([
                    broadcast_addr[0],
                    broadcast_addr[1],
                    broadcast_addr[2],
                    broadcast_addr[3],
                    subnet,
                ]);
            }
            Ip::V6(ip) => {
                let subnet = ip[16].min(128);
                let addr = u128::from_be_bytes([
                    ip[0], ip[1], ip[2], ip[3], ip[4], ip[5], ip[6], ip[7], ip[8], ip[9], ip[10],
                    ip[11], ip[12], ip[13], ip[14], ip[15],
                ]);
                let mask = u128::MAX << (128 - subnet);
                let broadcast_addr: [u8; 16] = (addr | !mask).to_be_bytes();
                return Ip::V6([
                    broadcast_addr[0],
                    broadcast_addr[1],
                    broadcast_addr[2],
                    broadcast_addr[3],
                    broadcast_addr[4],
                    broadcast_addr[5],
                    broadcast_addr[6],
                    broadcast_addr[7],
                    broadcast_addr[8],
                    broadcast_addr[9],
                    broadcast_addr[10],
                    broadcast_addr[11],
                    broadcast_addr[12],
                    broadcast_addr[13],
                    broadcast_addr[14],
                    broadcast_addr[15],
                    subnet,
                ]);
            }
            Ip::None => Ip::None,
        }
    }

    pub fn network(&self) -> Self {
        match self {
            Ip::Any(a) => {
                if a.len() == 5 {
                    let subnet = a[4].min(32);
                    let addr = u32::from_be_bytes([a[0], a[1], a[2], a[3]]);
                    let mask = u32::MAX << (32 - subnet);
                    let net_addr: [u8; 4] = (addr & mask).to_be_bytes();
                    return Ip::Any(vec![
                        net_addr[0],
                        net_addr[1],
                        net_addr[2],
                        net_addr[3],
                        subnet,
                    ]);
                } else if a.len() == 17 {
                    let subnet = a[16].min(128);
                    let addr = u128::from_be_bytes([
                        a[0], a[1], a[2], a[3], a[4], a[5], a[6], a[7], a[8], a[9], a[10], a[11],
                        a[12], a[13], a[14], a[15],
                    ]);
                    let mask = u128::MAX << (128 - subnet);
                    let net_addr: [u8; 16] = (addr & mask).to_be_bytes();
                    return Ip::Any(vec![
                        net_addr[0],
                        net_addr[1],
                        net_addr[2],
                        net_addr[3],
                        net_addr[4],
                        net_addr[5],
                        net_addr[6],
                        net_addr[7],
                        net_addr[8],
                        net_addr[9],
                        net_addr[10],
                        net_addr[11],
                        net_addr[12],
                        net_addr[13],
                        net_addr[14],
                        net_addr[15],
                        a[16],
                    ]);
                }
                return Ip::None;
            }
            Ip::V4(a) => {
                let subnet = a[4].min(32);
                let addr = u32::from_be_bytes([a[0], a[1], a[2], a[3]]);
                let mask = u32::MAX << (32 - subnet);
                let net_addr: [u8; 4] = (addr & mask).to_be_bytes();
                return Ip::V4([net_addr[0], net_addr[1], net_addr[2], net_addr[3], subnet]);
            }
            Ip::V6(a) => {
                let subnet = a[16].min(128);
                let addr = u128::from_be_bytes([
                    a[0], a[1], a[2], a[3], a[4], a[5], a[6], a[7], a[8], a[9], a[10], a[11],
                    a[12], a[13], a[14], a[15],
                ]);
                let mask = u128::MAX << (128 - subnet);
                let net_addr: [u8; 16] = (addr & mask).to_be_bytes();
                return Ip::V6([
                    net_addr[0],
                    net_addr[1],
                    net_addr[2],
                    net_addr[3],
                    net_addr[4],
                    net_addr[5],
                    net_addr[6],
                    net_addr[7],
                    net_addr[8],
                    net_addr[9],
                    net_addr[10],
                    net_addr[11],
                    net_addr[12],
                    net_addr[13],
                    net_addr[14],
                    net_addr[15],
                    subnet,
                ]);
            }
            Ip::None => Ip::None,
        }
    }

    pub fn wildcard(&self) -> Self {
        match self {
            Ip::Any(a) => {
                if a.len() == 5 {
                    let subnet = a[4].min(32);
                    let addr = u32::from_be_bytes([a[0], a[1], a[2], a[3]]);
                    let mask = u32::MAX << (32 - subnet);
                    let wildcard_addr: [u8; 4] = (addr & !mask).to_be_bytes();
                    return Ip::Any(vec![
                        wildcard_addr[0],
                        wildcard_addr[1],
                        wildcard_addr[2],
                        wildcard_addr[3],
                        subnet,
                    ]);
                } else if a.len() == 17 {
                    let subnet = a[16].min(128);
                    let addr = u128::from_be_bytes([
                        a[0], a[1], a[2], a[3], a[4], a[5], a[6], a[7], a[8], a[9], a[10], a[11],
                        a[12], a[13], a[14], a[15],
                    ]);
                    let mask = u128::MAX << (128 - subnet);
                    let wildcard_addr: [u8; 16] = (addr & !mask).to_be_bytes();
                    return Ip::Any(vec![
                        wildcard_addr[0],
                        wildcard_addr[1],
                        wildcard_addr[2],
                        wildcard_addr[3],
                        wildcard_addr[4],
                        wildcard_addr[5],
                        wildcard_addr[6],
                        wildcard_addr[7],
                        wildcard_addr[8],
                        wildcard_addr[9],
                        wildcard_addr[10],
                        wildcard_addr[11],
                        wildcard_addr[12],
                        wildcard_addr[13],
                        wildcard_addr[14],
                        wildcard_addr[15],
                        subnet,
                    ]);
                }
                return Ip::None;
            }
            Ip::V4(a) => {
                let subnet = a[4].min(32);
                let addr = u32::from_be_bytes([a[0], a[1], a[2], a[3]]);
                let mask = u32::MAX << (32 - subnet);
                let wildcard_addr: [u8; 4] = (addr & !mask).to_be_bytes();
                return Ip::V4([
                    wildcard_addr[0],
                    wildcard_addr[1],
                    wildcard_addr[2],
                    wildcard_addr[3],
                    subnet,
                ]);
            }
            Ip::V6(a) => {
                let subnet = a[16].min(128);
                let addr = u128::from_be_bytes([
                    a[0], a[1], a[2], a[3], a[4], a[5], a[6], a[7], a[8], a[9], a[10], a[11],
                    a[12], a[13], a[14], a[15],
                ]);
                let mask = u128::MAX << (128 - subnet);
                let wildcard_addr: [u8; 16] = (addr & !mask).to_be_bytes();
                return Ip::V6([
                    wildcard_addr[0],
                    wildcard_addr[1],
                    wildcard_addr[2],
                    wildcard_addr[3],
                    wildcard_addr[4],
                    wildcard_addr[5],
                    wildcard_addr[6],
                    wildcard_addr[7],
                    wildcard_addr[8],
                    wildcard_addr[9],
                    wildcard_addr[10],
                    wildcard_addr[11],
                    wildcard_addr[12],
                    wildcard_addr[13],
                    wildcard_addr[14],
                    wildcard_addr[15],
                    subnet,
                ]);
            }
            Ip::None => Ip::None,
        }
    }
    pub fn contains(&self, other: &Ip) -> bool {
        self.network() <= other.network() && self.broadcast() >= other.broadcast()
    }
}
impl Default for Ip {
    fn default() -> Self {
        Ip::None
    }
}

impl Display for Ip {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.to_string())
    }
}

impl From<(u128, u8)> for Ip {
    fn from(value: (u128, u8)) -> Self {
        Self::V6([
            (value.0 >> 120) as u8,
            (value.0 >> 112) as u8,
            (value.0 >> 104) as u8,
            (value.0 >> 96) as u8,
            (value.0 >> 88) as u8,
            (value.0 >> 80) as u8,
            (value.0 >> 72) as u8,
            (value.0 >> 64) as u8,
            (value.0 >> 56) as u8,
            (value.0 >> 48) as u8,
            (value.0 >> 40) as u8,
            (value.0 >> 32) as u8,
            (value.0 >> 24) as u8,
            (value.0 >> 16) as u8,
            (value.0 >> 8) as u8,
            (value.0) as u8,
            value.1,
        ])
    }
}

impl From<(u32, u8)> for Ip {
    fn from(value: (u32, u8)) -> Self {
        Self::V4([
            (value.0 >> 24) as u8,
            (value.0 >> 16) as u8,
            (value.0 >> 8) as u8,
            (value.0) as u8,
            value.1,
        ])
    }
}

impl From<Ip> for (u128, u8) {
    fn from(value: Ip) -> Self {
        match value {
            Ip::V4(_) => panic!("Cannot convert IPV4 to (u128, u8)"),
            Ip::V6(addr) => (
                u128::from_be_bytes(addr[..16].try_into().unwrap()),
                addr[16],
            ),
            Ip::Any(addr) => (
                u128::from_be_bytes(addr[..16].try_into().unwrap()),
                addr[16],
            ),
            Ip::None => (0, 0),
        }
    }
}

impl From<Ip> for (u32, u8) {
    fn from(value: Ip) -> Self {
        match value {
            Ip::V4(addr) => (u32::from_be_bytes(addr[..4].try_into().unwrap()), addr[4]),
            Ip::V6(_) => panic!("Cannot convert IPV6 to (u32, u8)"),
            Ip::Any(addr) => (u32::from_be_bytes(addr[..4].try_into().unwrap()), addr[4]),
            Ip::None => (0, 0),
        }
    }
}

impl FromStr for Ip {
    type Err = ParseIntError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ip::from_str(s)
    }
}

impl Index<usize> for Ip {
    type Output = u8;
    fn index(&self, index: usize) -> &Self::Output {
        match self {
            Ip::Any(addr) => &addr[index],
            Ip::V4(addr) => &addr[index],
            Ip::V6(addr) => &addr[index],
            Ip::None => panic!("Ip has an invalid number of elements"),
        }
    }
}

impl IndexMut<usize> for Ip {
    fn index_mut(&mut self, index: usize) -> &mut u8 {
        match self {
            Ip::Any(addr) => &mut addr[index],
            Ip::V4(addr) => &mut addr[index],
            Ip::V6(addr) => &mut addr[index],
            Ip::None => panic!("Ip has an invalid number of elements"),
        }
    }
}

impl Ord for Ip {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Ip::V4(a), Ip::V4(b)) => a.cmp(b),
            (Ip::V4(a), Ip::V6(b)) => a.to_vec().cmp(&b.to_vec()),
            (Ip::V4(a), Ip::Any(b)) => a.to_vec().cmp(&b),
            (Ip::V6(a), Ip::V4(b)) => a.to_vec().cmp(&b.to_vec()),
            (Ip::V6(a), Ip::V6(b)) => a.cmp(b),
            (Ip::V6(a), Ip::Any(b)) => a.to_vec().cmp(&b),
            (Ip::Any(a), Ip::V4(b)) => a.cmp(&b.to_vec()),
            (Ip::Any(a), Ip::V6(b)) => a.cmp(&b.to_vec()),
            (Ip::Any(a), Ip::Any(b)) => a.cmp(&b),
            (Ip::None, Ip::None) => Ordering::Equal,
            (Ip::None, _) => Ordering::Less,
            (_, Ip::None) => Ordering::Greater,
        }
    }
}

impl PartialOrd for Ip {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl std::hash::Hash for Ip {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        Ip::hash(self, state);
    }
}

pub static PROTO_LIST: LazyLock<Vec<Proto>> = LazyLock::new(|| {
    Proto::import(&Path::new("/etc/protocols")).expect("Failed to import protocol information")
});

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Proto(u8, String);

impl Proto {
    pub fn clone(&self) -> Self {
        Self(self.0, self.1.clone())
    }

    pub fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }

    pub fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }

    pub fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.to_string())
    }

    pub fn from_num(number: u8) -> Self {
        PROTO_LIST
            .iter()
            .find(|p| p.0 == number)
            .unwrap_or(&Proto(254, String::new()))
            .clone()
    }

    pub fn from_str(s: &str) -> std::result::Result<Self, ParseIntError> {
        let s = s.trim();
        let (main_part, description) = match s.split_once('#') {
            Some((main, desc)) => (main.trim(), desc.trim()),
            None => (s, ""),
        };
        let mut parts = main_part.split_whitespace();
        let proto_name = parts.next().unwrap_or("").trim();
        let number_str = parts.next().unwrap_or("").trim();

        let (number, name) = if !number_str.is_empty() {
            let num = u8::from_str(number_str)?;
            (num, proto_name)
        } else if let Ok(num) = u8::from_str(proto_name) {
            (num, proto_name)
        } else {
            (254, proto_name)
        };

        Ok(Proto(
            number,
            if description.is_empty() {
                format!("{},", name)
            } else {
                format!("{},{}", name, description)
            },
        ))
    }

    pub fn get_description(&self) -> String {
        let mut parts = self.1.splitn(2, ',');
        parts.next();
        parts.next().unwrap_or("").trim().to_string()
    }

    pub fn get_name(&self) -> String {
        let s = self.1.split(',').next().unwrap_or("").trim();
        if s.is_empty() {
            "unknown".to_string()
        } else {
            s.trim().to_string()
        }
    }

    pub fn get_number(&self) -> u8 {
        self.0
    }

    /// Import protocols from `/etc/protocols` or nftables-formatted file
    pub fn import(path: &Path) -> std::result::Result<Vec<Proto>, std::io::Error> {
        let mut file = File::open(path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        let mut protos = Vec::<Proto>::new();

        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            // Try nftables-style: "tcp 6 # Transmission Control"
            let (main_part, description) = match line.split_once('#') {
                Some((main, desc)) => (main.trim(), desc.trim()),
                None => (line, ""),
            };
            let mut fields = main_part.split_whitespace();
            let proto_name = fields.next().unwrap_or("").trim();
            let proto_number = fields.next().unwrap_or("").trim();

            if !proto_name.is_empty() && !proto_number.is_empty() {
                if let Ok(number) = u8::from_str(proto_number) {
                    let name = if description.is_empty() {
                        format!("{},", proto_name)
                    } else {
                        format!("{},{}", proto_name, description)
                    };
                    protos.push(Proto(number, name));
                    continue;
                }
            }

            // Fallback: /etc/protocols-style: "icmp 1 ICMP # Internet Control Message"
            let fields: Vec<&str> = main_part.split_whitespace().collect();
            if fields.len() >= 2 {
                let name = fields[0];
                let number_str = fields[1];
                if let Ok(number) = u8::from_str(number_str) {
                    let desc = if !description.is_empty() {
                        description
                    } else if fields.len() > 2 {
                        &fields[2..].join(" ")
                    } else {
                        ""
                    };
                    let proto_name = if desc.is_empty() {
                        format!("{},", name)
                    } else {
                        format!("{},{}", name, desc)
                    };
                    protos.push(Proto(number, proto_name));
                }
            }
        }
        Ok(protos)
    }

    pub fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }

    pub fn new() -> Self {
        Self(0, String::new())
    }

    pub fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }

    pub fn set(&mut self, number: u8, name: String) -> Self {
        self.0 = number;
        self.1 = name;
        self.clone()
    }

    pub fn set_description(&mut self, description: &str) -> Self {
        self.1 = format!("{},{}", self.get_name(), description);
        self.clone()
    }

    pub fn set_name(&mut self, name: &str) -> Self {
        let desc = self.get_description();
        self.1 = if desc.is_empty() {
            format!("{},", name)
        } else {
            format!("{},{}", name, desc)
        };
        self.clone()
    }

    pub fn set_number(&mut self, number: u8) -> Self {
        self.0 = number;
        self.clone()
    }

    pub fn to_string(&self) -> String {
        let name = self.get_name();
        let desc = self.get_description();
        if desc.is_empty() {
            format!("{} ({})", name, self.get_number())
        } else {
            format!("{} ({}) # {}", name, self.get_number(), desc)
        }
    }
}

impl Default for Proto {
    fn default() -> Self {
        Proto(255, String::new())
    }
}

impl Display for Proto {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.to_string())
    }
}
impl From<(u8, &str)> for Proto {
    fn from(value: (u8, &str)) -> Self {
        Self(value.0, value.1.to_string())
    }
}
impl From<u8> for Proto {
    fn from(value: u8) -> Self {
        Self(value, String::new())
    }
}
impl From<Proto> for u8 {
    fn from(value: Proto) -> Self {
        value.0
    }
}
impl From<(u8, String)> for Proto {
    fn from(value: (u8, String)) -> Self {
        Self(value.0, value.1)
    }
}
impl From<Proto> for (u8, String) {
    fn from(value: Proto) -> Self {
        (value.0, value.1)
    }
}

impl FromStr for Proto {
    type Err = ParseIntError;
    fn from_str(s: &str) -> std::result::Result<Proto, ParseIntError> {
        Proto::from_str(s)
    }
}

impl std::hash::Hash for Proto {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl Ord for Proto {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl PartialOrd for Proto {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub const PORT_LIST: LazyLock<Vec<Port>> = LazyLock::new(|| {
    Port::import(&Path::new("/etc/services")).expect("Failed to import port information")
});

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Port(u16, u8, String);

impl Port {
    pub fn new() -> Self {
        Self(0, 254, String::new())
    }
    pub fn default() -> Self {
        Self(0, 254, String::new())
    }
    pub fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
        self.1.hash(state);
    }

    pub fn clone(&self) -> Self {
        Self(self.0, self.1, self.2.clone())
    }

    pub fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0).then_with(|| self.1.cmp(&other.1))
    }

    pub fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }

    pub fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }

    pub fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }

    pub fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.to_string())
    }

    /// Format for nftables, e.g. `tcp dport 80` or `udp dport 53`
    pub fn to_string(&self) -> String {
        let proto = match self.1 {
            6 => "tcp",
            17 => "udp",
            37 => "ddp",
            132 => "sctp",
            2 => "igmp",
            1 => "icmp",
            58 => "icmpv6",
            _ => "unknown",
        };
        match self.2.trim() {
            "" => format!("{} {}", proto, self.0),
            desc => format!("{} ({}) # {}", proto, self.0, desc),
        }
    }

    /// Set port number and protocol, clear service name
    pub fn set(&mut self, number: u16, proto: u8) -> Self {
        self.0 = number;
        self.1 = proto;
        self.2.clear();
        self.clone()
    }

    /// Parse Port from string in nftables style: "tcp 80" or "udp 53 # dns"
    pub fn from_str(s: &str) -> std::result::Result<Self, ParseIntError> {
        let s = s.trim();
        let mut port = Port::new();
        let (main, comment) = if let Some(idx) = s.find('#') {
            (s[..idx].trim(), s[idx + 1..].trim())
        } else {
            (s, "")
        };

        // Try nftables style: "tcp dport 80", "udp sport 53", or just "tcp 80"
        let mut parts = main.split_whitespace();
        let proto_str = parts.next().unwrap_or("");
        let field = parts.next().unwrap_or("");
        let value = parts.next().unwrap_or("");
        let (port_num, proto_num) = match (
            proto_str.to_ascii_lowercase().as_str(),
            field.to_ascii_lowercase().as_str(),
        ) {
            // e.g. "tcp dport 80"
            ("tcp", "dport")
            | ("udp", "dport")
            | ("icmp", "dport")
            | ("ddp", "dport")
            | ("sctp", "dport")
            | ("igmp", "dport")
            | ("icmpv6", "dport") => {
                let proto_num = match proto_str.to_ascii_lowercase().as_str() {
                    "tcp" => 6,
                    "udp" => 17,
                    "ddp" => 37,
                    "sctp" => 132,
                    "igmp" => 2,
                    "icmp" => 1,
                    "icmpv6" => 58,
                    _ => 254,
                };
                (u16::from_str(value)?, proto_num)
            }
            // e.g. "tcp sport 80"
            ("tcp", "sport")
            | ("udp", "sport")
            | ("icmp", "sport")
            | ("ddp", "sport")
            | ("sctp", "sport")
            | ("igmp", "sport")
            | ("icmpv6", "sport") => {
                let proto_num = match proto_str.to_ascii_lowercase().as_str() {
                    "tcp" => 6,
                    "udp" => 17,
                    "ddp" => 37,
                    "sctp" => 132,
                    "igmp" => 2,
                    "icmp" => 1,
                    "icmpv6" => 58,
                    _ => 254,
                };
                (u16::from_str(value)?, proto_num)
            }
            // e.g. "tcp 80"
            ("tcp", port_candidate)
            | ("udp", port_candidate)
            | ("icmp", port_candidate)
            | ("ddp", port_candidate)
            | ("sctp", port_candidate)
            | ("igmp", port_candidate)
            | ("icmpv6", port_candidate)
                if !port_candidate.is_empty() && value.is_empty() =>
            {
                let proto_num = match proto_str.to_ascii_lowercase().as_str() {
                    "tcp" => 6,
                    "udp" => 17,
                    "ddp" => 37,
                    "sctp" => 132,
                    "igmp" => 2,
                    "icmp" => 1,
                    "icmpv6" => 58,
                    _ => 254,
                };
                (u16::from_str(port_candidate)?, proto_num)
            }
            // fallback: just port (numeric)
            (port_candidate, "") if !port_candidate.is_empty() && value.is_empty() => {
                (u16::from_str(port_candidate)?, 254)
            }
            _ => (0, 254),
        };

        port.0 = port_num;
        port.1 = proto_num;
        if !comment.is_empty() {
            port.2 = comment.trim_matches('#').to_string();
        }
        Ok(port)
    }

    pub fn from_num(number: u16) -> Self {
        PORT_LIST
            .iter()
            .find(|p| p.0 == number)
            .cloned()
            .unwrap_or(Port(number, 254, String::new()))
    }

    /// Import ports from `/etc/services` or nftables-formatted file
    pub fn import(path: &Path) -> std::result::Result<Vec<Port>, std::io::Error> {
        let mut file = File::open(path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        let mut ports = Vec::new();

        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            if let Ok(port) = Port::from_str(line) {
                if port.0 != 0 {
                    ports.push(port);
                    continue;
                }
            }
            // Fallback: /etc/services style: "ssh 22/tcp # SSH Remote Login Protocol"
            let mut fields = line.split_whitespace();
            let service = fields.next().unwrap_or("").trim();
            let port_proto = fields.next().unwrap_or("").trim();
            let mut port_proto_parts = port_proto.split('/');

            if let (Some(port_str), Some(proto_str)) =
                (port_proto_parts.next(), port_proto_parts.next())
            {
                let description = if let Some(idx) = line.find('#') {
                    line[idx + 1..].trim()
                } else {
                    ""
                };
                if let Ok(port_number) = port_str.parse::<u16>() {
                    let proto_num = match proto_str.trim().to_ascii_lowercase().as_str() {
                        "tcp" => 6,
                        "udp" => 17,
                        "ddp" => 37,
                        "sctp" => 132,
                        "igmp" => 2,
                        "icmp" => 1,
                        "icmpv6" => 58,
                        _ => 254,
                    };
                    let name = if !description.is_empty() {
                        format!("{}, {}", service, description)
                    } else {
                        service.to_string()
                    };
                    ports.push(Port(port_number, proto_num, name));
                }
            }
        }
        Ok(ports)
    }
}

impl Default for Port {
    fn default() -> Self {
        Self(0, 254, String::new())
    }
}

impl Display for Port {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.to_string())
    }
}

impl From<(u16, u8, &str)> for Port {
    fn from(value: (u16, u8, &str)) -> Self {
        Self(value.0, value.1, value.2.to_string())
    }
}

impl From<Port> for (u16, u8, String) {
    fn from(value: Port) -> Self {
        (value.0, value.1, value.2)
    }
}

impl FromStr for Port {
    type Err = ParseIntError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Port::from_str(s)
    }
}

impl Hash for Port {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
        self.1.hash(state);
    }
}

impl Ord for Port {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0).then_with(|| self.1.cmp(&other.1))
    }
}

impl PartialOrd for Port {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Action {
    None,
    Accept,
    Drop,
    Reject,
    Skip,
    Limit(u32),
    Return,
    Jump(u32),
    Log(String),
    Mark(u32),
    Meta(String),
    Set(String, String),            // set <key> <value>
    Masquerade,                     // masquerade
    Snat(String),                   // snat to <addr>
    Dnat(String),                   // dnat to <addr>
    Redirect(Option<u16>),          // redirect [to-port]
    Ct,                             // ct
    Queue,                          // queue
    Quota(u64),                     // quota <bytes>
    TProxy(u16, u32),               // tproxy to port with mark
    Counter,                        // counter
    Hashlimit(String, Option<u32>), // hashlimit rate, burst
    FlowOffload,                    // flow offload
    Reclassify,                     // reclassify
}
impl Action {
    pub fn new() -> Self {
        Action::Drop
    }
    pub fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.to_string())
    }
    pub fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Action::Accept => "accept".hash(state),
            Action::Drop => "drop".hash(state),
            Action::Reject => "reject".hash(state),
            Action::Skip => "skip".hash(state),
            Action::Limit(value) => value.hash(state),
            Action::Return => "return".hash(state),
            Action::Jump(value) => value.hash(state),
            Action::Log(value) => value.hash(state),
            Action::Mark(value) => value.hash(state),
            Action::Meta(value) => value.hash(state),
            Action::Set(key, value) => (value, key).hash(state),
            Action::Masquerade => "masquerade".hash(state),
            Action::Snat(value) => value.hash(state),
            Action::Dnat(value) => value.hash(state),
            Action::Redirect(value) => value.hash(state),
            Action::Ct => "ct".hash(state),
            Action::Queue => "queue".hash(state),
            Action::Quota(value) => value.hash(state),
            Action::TProxy(value, mark) => (value, mark).hash(state),
            Action::Counter => "counter".hash(state),
            Action::Hashlimit(value, mark) => (value, mark).hash(state),
            Action::FlowOffload => "flowoffload".hash(state),
            Action::Reclassify => "reclassify".hash(state),
            Action::None => "none".hash(state),
        }
    }
    /// Converts the Action variant into its nftables syntax string representation.
    pub fn to_string(&self) -> String {
        match self {
            Action::None => String::new(),
            Action::Accept => "accept".to_string(),
            Action::Drop => "drop".to_string(),
            Action::Reject => "reject".to_string(),
            Action::Skip => "skip".to_string(),
            Action::Limit(val) => format!("limit rate over {}/second drop", val),
            Action::Return => "return".to_string(),
            Action::Jump(val) => format!("jump chain_{}", val),
            Action::Log(msg) => {
                if msg.is_empty() {
                    "log".to_string()
                } else {
                    format!("log prefix \"{}\"", msg)
                }
            }
            Action::Mark(val) => format!("meta mark set {}", val),
            Action::Meta(msg) => format!("meta {}", msg),
            Action::Set(key, value) => format!("set {} {}", key, value),
            Action::Masquerade => "masquerade".to_string(),
            Action::Snat(addr) => format!("snat to {}", addr),
            Action::Dnat(addr) => format!("dnat to {}", addr),
            Action::Redirect(port) => format!("redirect {}", port.unwrap_or(0)),
            Action::Ct => "ct".to_string(),
            Action::Queue => "queue".to_string(),
            Action::Quota(bytes) => format!("quota {}", bytes),
            Action::TProxy(port, mark) => format!("tproxy to {} with mark {}", port, mark),
            Action::Counter => "counter".to_string(),
            Action::Hashlimit(rate, burst) => format!("hashlimit {} {}", rate, burst.unwrap_or(0)),
            Action::FlowOffload => "flow offload".to_string(),
            Action::Reclassify => "reclassify".to_string(),
        }
    }

    pub fn from_str(s: &str) -> std::result::Result<Self, ParseIntError> {
        let s = s.trim();
        if s.is_empty() {
            return Ok(Action::None);
        }
        let mut parts = s.split_whitespace().peekable();
        match parts.next().unwrap_or("").to_ascii_lowercase().as_str() {
            "accept" => Ok(Action::Accept),
            "drop" => Ok(Action::Drop),
            "reject" => Ok(Action::Reject),
            "skip" => Ok(Action::Skip),
            "limit" => {
                while let Some(token) = parts.next() {
                    if token == "over" {
                        if let Some(rate_token) = parts.next() {
                            if let Some(num) = rate_token.split('/').next() {
                                return Ok(Action::Limit(num.parse::<u32>()?));
                            }
                        }
                    }
                }
                Ok(Action::Limit(0))
            }
            "return" => Ok(Action::Return),
            "jump" => {
                if let Some(chain_token) = parts.next() {
                    let chain_str = chain_token.trim_start_matches("chain_");
                    let val = chain_str.parse::<u32>()?;
                    Ok(Action::Jump(val))
                } else {
                    Ok(Action::Jump(0))
                }
            }
            "log" => {
                let mut msg = String::new();
                let mut prefix_next = false;
                while let Some(token) = parts.next() {
                    if token == "prefix" {
                        prefix_next = true;
                        continue;
                    }
                    if prefix_next {
                        msg = token.trim_matches('"').to_string();
                        for t in parts {
                            msg.push(' ');
                            msg.push_str(t.trim_matches('"'));
                        }
                        break;
                    }
                }
                Ok(Action::Log(msg))
            }
            "mark" => {
                if let Some(val_token) = parts.next() {
                    Ok(Action::Mark(val_token.trim().parse::<u32>()?))
                } else {
                    Ok(Action::Mark(0))
                }
            }
            "meta" => {
                if let Some(msg_token) = parts.next() {
                    Ok(Action::Meta(msg_token.trim_matches('"').to_string()))
                } else {
                    Ok(Action::Meta(String::new()))
                }
            }
            "set" => {
                let key = parts.next().unwrap_or("").trim_matches('"').to_string();
                let value = parts.next().unwrap_or("").trim_matches('"').to_string();
                Ok(Action::Set(key, value))
            }
            "masquerade" => Ok(Action::Masquerade),
            "snat" => {
                if let Some(_) = parts.next_if(|t| *t == "to") {}
                let addr = parts.next().unwrap_or("").trim_matches('"').to_string();
                Ok(Action::Snat(addr))
            }
            "dnat" => {
                if let Some(_) = parts.next_if(|t| *t == "to") {}
                let addr = parts.next().unwrap_or("").trim_matches('"').to_string();
                Ok(Action::Dnat(addr))
            }
            "redirect" => {
                let port = parts.next().and_then(|s| s.parse::<u16>().ok());
                Ok(Action::Redirect(port))
            }
            "ct" => Ok(Action::Ct),
            "queue" => Ok(Action::Queue),
            "quota" => {
                if let Some(bytes_token) = parts.next() {
                    Ok(Action::Quota(bytes_token.parse::<u64>()?))
                } else {
                    Ok(Action::Quota(0))
                }
            }
            "tproxy" => {
                let mut port: u16 = 0;
                let mut mark: u32 = 0;
                while let Some(token) = parts.next() {
                    if token == "to" {
                        if let Some(port_token) = parts.next() {
                            port = port_token.parse::<u16>().unwrap_or(0);
                        }
                    }
                    if token == "with" {
                        if let Some(_) = parts.next_if(|t| *t == "mark") {
                            if let Some(mark_token) = parts.next() {
                                mark = mark_token.parse::<u32>().unwrap_or(0);
                            }
                        }
                    }
                }
                Ok(Action::TProxy(port, mark))
            }
            "counter" => Ok(Action::Counter),
            "hashlimit" => {
                let rate = parts.next().unwrap_or("").to_string();
                let burst = parts.next().and_then(|s| s.parse::<u32>().ok());
                Ok(Action::Hashlimit(rate, burst))
            }
            "flow" => {
                if let Some("offload") = parts.next() {
                    Ok(Action::FlowOffload)
                } else {
                    Ok(Action::None)
                }
            }
            "reclassify" => Ok(Action::Reclassify),
            _ => Ok(Action::None),
        }
    }

    /// Standard string representation (for debug, not nft syntax)
    pub fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Action::Accept, Action::Accept) => Ordering::Equal,
            (Action::Drop, Action::Drop) => Ordering::Equal,
            (Action::Reject, Action::Reject) => Ordering::Equal,
            (Action::Skip, Action::Skip) => Ordering::Equal,
            (Action::Limit(value1), Action::Limit(value2)) => value1.cmp(value2),
            (Action::Return, Action::Return) => Ordering::Equal,
            (Action::Jump(value1), Action::Jump(value2)) => value1.cmp(value2),
            (Action::Log(value1), Action::Log(value2)) => value1.cmp(value2),
            (Action::Mark(value1), Action::Mark(value2)) => value1.cmp(value2),
            (Action::Meta(value1), Action::Meta(value2)) => value1.cmp(value2),
            (Action::Set(k1, v1), Action::Set(k2, v2)) => k1.cmp(k2).then_with(|| v1.cmp(v2)),
            (Action::Masquerade, Action::Masquerade) => Ordering::Equal,
            (Action::Snat(a1), Action::Snat(a2)) => a1.cmp(a2),
            (Action::Dnat(a1), Action::Dnat(a2)) => a1.cmp(a2),
            (Action::Redirect(p1), Action::Redirect(p2)) => p1.cmp(p2),
            (Action::Ct, Action::Ct) => Ordering::Equal,
            (Action::Queue, Action::Queue) => Ordering::Equal,
            (Action::Quota(q1), Action::Quota(q2)) => q1.cmp(q2),
            (Action::TProxy(p1, m1), Action::TProxy(p2, m2)) => p1.cmp(p2).then_with(|| m1.cmp(m2)),
            (Action::Counter, Action::Counter) => Ordering::Equal,
            (Action::Hashlimit(r1, b1), Action::Hashlimit(r2, b2)) => {
                r1.cmp(r2).then_with(|| b1.cmp(b2))
            }
            (Action::FlowOffload, Action::FlowOffload) => Ordering::Equal,
            (Action::Reclassify, Action::Reclassify) => Ordering::Equal,
            (a, b) => (a.variant_at()).cmp(&b.variant_at()),
        }
    }

    /// Returns true if the action is terminal (ACCEPT, DROP, REJECT, RETURN).
    pub fn is_terminal(&self) -> bool {
        matches!(
            self,
            Action::Accept | Action::Drop | Action::Reject | Action::Return
        )
    }
    pub fn is_accept(&self) -> bool {
        matches!(self, Action::Accept)
    }
    pub fn is_drop(&self) -> bool {
        matches!(self, Action::Drop)
    }
    pub fn is_reject(&self) -> bool {
        matches!(self, Action::Reject)
    }
    pub fn is_return(&self) -> bool {
        matches!(self, Action::Return)
    }
    pub fn is_skip(&self) -> bool {
        matches!(self, Action::Skip)
    }
    pub fn is_limit(&self) -> bool {
        matches!(self, Action::Limit(_))
    }
    pub fn is_jump(&self) -> bool {
        matches!(self, Action::Jump(_))
    }
    pub fn is_log(&self) -> bool {
        matches!(self, Action::Log(_))
    }
    pub fn is_mark(&self) -> bool {
        matches!(self, Action::Mark(_))
    }
    pub fn is_meta(&self) -> bool {
        matches!(self, Action::Meta(_))
    }
    pub fn is_set(&self) -> bool {
        matches!(self, Action::Set(_, _))
    }
    pub fn is_masquerade(&self) -> bool {
        matches!(self, Action::Masquerade)
    }
    pub fn is_snat(&self) -> bool {
        matches!(self, Action::Snat(_))
    }
    pub fn is_dnat(&self) -> bool {
        matches!(self, Action::Dnat(_))
    }
    pub fn is_redirect(&self) -> bool {
        matches!(self, Action::Redirect(_))
    }
    pub fn is_ct(&self) -> bool {
        matches!(self, Action::Ct)
    }
    pub fn is_queue(&self) -> bool {
        matches!(self, Action::Queue)
    }
    pub fn is_quota(&self) -> bool {
        matches!(self, Action::Quota(_))
    }
    pub fn is_tproxy(&self) -> bool {
        matches!(self, Action::TProxy(_, _))
    }
    pub fn is_counter(&self) -> bool {
        matches!(self, Action::Counter)
    }
    pub fn is_hashlimit(&self) -> bool {
        matches!(self, Action::Hashlimit(_, _))
    }
    pub fn is_flow_offload(&self) -> bool {
        matches!(self, Action::FlowOffload)
    }
    pub fn is_reclassify(&self) -> bool {
        matches!(self, Action::Reclassify)
    }
    pub fn is_none(&self) -> bool {
        matches!(self, Action::None)
    }
    /// Returns Some(limit value) if the action is LIMIT.
    pub fn limit_value(&self) -> Option<u32> {
        if let Action::Limit(v) = self {
            Some(*v)
        } else {
            None
        }
    }

    /// Returns the index of the action variant for consistent ordering in cmp.
    pub fn variant_at(&self) -> usize {
        match self {
            Action::Accept => 0,
            Action::Drop => 1,
            Action::Reject => 2,
            Action::Skip => 3,
            Action::Limit(_) => 4,
            Action::Return => 5,
            Action::Jump(_) => 6,
            Action::Log(_) => 7,
            Action::Mark(_) => 8,
            Action::Meta(_) => 9,
            Action::Set(_, _) => 10,
            Action::Masquerade => 11,
            Action::Snat(_) => 12,
            Action::Dnat(_) => 13,
            Action::Redirect(_) => 14,
            Action::Ct => 15,
            Action::Queue => 16,
            Action::Quota(_) => 17,
            Action::TProxy(_, _) => 18,
            Action::Counter => 19,
            Action::Hashlimit(_, _) => 20,
            Action::FlowOffload => 21,
            Action::Reclassify => 22,
            Action::None => 23,
        }
    }
}
impl Default for Action {
    fn default() -> Self {
        Action::Drop
    }
}

impl Display for Action {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.to_string())
    }
}

impl FromStr for Action {
    type Err = ParseIntError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::from_str(s)
    }
}

impl Hash for Action {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Action::Accept => "accept".hash(state),
            Action::Drop => "drop".hash(state),
            Action::Reject => "reject".hash(state),
            Action::Skip => "skip".hash(state),
            Action::Limit(value) => value.hash(state),
            Action::Return => "return".hash(state),
            Action::Jump(value) => value.hash(state),
            Action::Log(value) => value.hash(state),
            Action::Mark(value) => value.hash(state),
            Action::Meta(value) => value.hash(state),
            Action::Set(key, value) => (value, key).hash(state),
            Action::Masquerade => "masquerade".hash(state),
            Action::Snat(value) => value.hash(state),
            Action::Dnat(value) => value.hash(state),
            Action::Redirect(value) => value.hash(state),
            Action::Ct => "ct".hash(state),
            Action::Queue => "queue".hash(state),
            Action::Quota(value) => value.hash(state),
            Action::TProxy(value, mark) => (value, mark).hash(state),
            Action::Counter => "counter".hash(state),
            Action::Hashlimit(value, mark) => (value, mark).hash(state),
            Action::FlowOffload => "flowoffload".hash(state),
            Action::Reclassify => "reclassify".hash(state),
            Action::None => "none".hash(state),
        }
    }
}
impl Ord for Action {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cmp(other)
    }
}

impl PartialOrd for Action {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone)]
pub struct Chain {
    rules: Vec<Rule>,
    number: u32,
    name: String,
}

impl Chain {
    pub fn clear(&mut self) {
        self.rules.clear();
    }

    pub fn clone(&self) -> Self {
        Self {
            rules: self.rules.clone(),
            number: self.number,
            name: self.name.clone(),
        }
    }

    pub fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.to_string())
    }

    pub fn from_str(s: &str) -> std::result::Result<Self, ParseIntError> {
        let mut rules = Vec::<Rule>::new();
        let mut name = String::new();
        let mut number = 0u32;
        for line in s.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            if let Some(rest) = line.strip_prefix("chain ") {
                let mut parts = rest.split_whitespace();
                if let Some(n) = parts.next() {
                    name = n.to_string();
                }
                if let Some(part) = parts.next() {
                    let part = part.trim_matches(|c| c == '(' || c == ')');
                    if let Ok(n) = part.parse::<u32>() {
                        number = n;
                    }
                }
            } else if let Some(r) = line.strip_prefix("  ") {
                rules.push(Rule::from_str(r)?);
            } else {
                rules.push(Rule::from_str(line)?);
            }
        }
        Ok(Self {
            rules,
            number,
            name,
        })
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_number(&self) -> u32 {
        self.number
    }

    pub fn get_rules(&self) -> &Vec<Rule> {
        &self.rules
    }

    pub fn insert(&mut self, index: usize, rule: Rule) {
        self.rules.insert(index, rule);
    }

    pub fn is_empty(&self) -> bool {
        self.rules.is_empty()
    }

    pub fn len(&self) -> usize {
        self.rules.len()
    }

    pub fn new() -> Self {
        Self {
            rules: Vec::new(),
            number: 0,
            name: String::new(),
        }
    }

    pub fn push(&mut self, rule: Rule) {
        self.rules.push(rule);
    }

    pub fn remove(&mut self, index: usize) -> Rule {
        self.rules.remove(index)
    }

    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    pub fn set_number(&mut self, number: u32) {
        self.number = number;
    }

    pub fn set_rules(&mut self, rules: Vec<Rule>) {
        self.rules = rules;
    }

    pub fn swap(&mut self, index: usize, other: usize) {
        self.rules.swap(index, other);
    }

    pub fn swap_remove(&mut self, index: usize) -> Rule {
        self.rules.swap_remove(index)
    }

    pub fn to_string(&self) -> String {
        let mut s = String::new();
        s.push_str(&format!("chain {} ({})\n", self.name, self.number));
        for rule in &self.rules {
            s.push_str("  ");
            s.push_str(&rule.to_string());
            s.push('\n');
        }
        s
    }
}
impl Default for Chain {
    fn default() -> Self {
        Self {
            name: "default".to_string(),
            number: 0,
            rules: Vec::new(),
        }
    }
}

impl Display for Chain {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.to_string())
    }
}

impl Eq for Chain {}

impl FromStr for Chain {
    type Err = ParseIntError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::from_str(s)
    }
}

impl Ord for Chain {
    fn cmp(&self, other: &Self) -> Ordering {
        self.number
            .cmp(&other.number)
            .then_with(|| self.name.cmp(&other.name))
            .then_with(|| self.rules.cmp(&other.rules))
    }
}

impl PartialEq for Chain {
    fn eq(&self, other: &Self) -> bool {
        self.number == other.number && self.name == other.name && self.rules == other.rules
    }
}

impl PartialOrd for Chain {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
#[derive(Debug, Clone)]
pub struct Rule {
    ips: (Vec<Ip>, Vec<Ip>),
    ports: (Vec<Port>, Vec<Port>),
    protocols: Vec<Proto>,
    family: Family,
    direction: Direction,
    action: Action,
    name: String,
    comment: String,
}

impl Rule {
    pub fn new(
        ips: (Vec<Ip>, Vec<Ip>),
        ports: (Vec<Port>, Vec<Port>),
        protocols: Vec<Proto>,
        family: Family,
        direction: Direction,
        action: Action,
        name: String,
        comment: String,
    ) -> Self {
        Self {
            ips,
            ports,
            protocols,
            family,
            direction,
            action,
            name,
            comment,
        }
    }

    pub fn clone(&self) -> Self {
        Self {
            ips: (self.ips.0.clone(), self.ips.1.clone()),
            ports: (self.ports.0.clone(), self.ports.1.clone()),
            protocols: self.protocols.clone(),
            family: self.family,
            direction: self.direction,
            action: self.action.clone(),
            name: self.name.clone(),
            comment: self.comment.clone(),
        }
    }

    pub fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }

    pub fn cmp(&self, other: &Self) -> Ordering {
        self.ips
            .0
            .cmp(&other.ips.0)
            .then_with(|| self.ips.1.cmp(&other.ips.1))
            .then_with(|| self.ports.0.cmp(&other.ports.0))
            .then_with(|| self.ports.1.cmp(&other.ports.1))
            .then_with(|| self.protocols.cmp(&other.protocols))
            .then_with(|| self.family.cmp(&other.family))
            .then_with(|| self.direction.cmp(&other.direction))
            .then_with(|| self.action.cmp(&other.action))
            .then_with(|| self.name.cmp(&other.name))
            .then_with(|| self.comment.cmp(&other.comment))
    }

    pub fn eq(&self, other: &Self) -> bool {
        self.ips.0 == other.ips.0
            && self.ips.1 == other.ips.1
            && self.ports.0 == other.ports.0
            && self.ports.1 == other.ports.1
            && self.protocols == other.protocols
            && self.family == other.family
            && self.direction == other.direction
            && self.action == other.action
            && self.name == other.name
            && self.comment == other.comment
    }

    pub fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }

    pub fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{self}")
    }

    pub fn src_ips(&self) -> &[Ip] {
        &self.ips.0
    }
    pub fn dst_ips(&self) -> &[Ip] {
        &self.ips.1
    }
    pub fn src_ports(&self) -> &[Port] {
        &self.ports.0
    }
    pub fn dst_ports(&self) -> &[Port] {
        &self.ports.1
    }
    pub fn protocols(&self) -> &[Proto] {
        &self.protocols
    }
    pub fn family(&self) -> &Family {
        &self.family
    }
    pub fn direction(&self) -> &Direction {
        &self.direction
    }
    pub fn action(&self) -> &Action {
        &self.action
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn comment(&self) -> &str {
        &self.comment
    }

    pub fn to_string(&self) -> String {
        let mut parts = Vec::new();
        let family = self.family().as_str();

        // Family, chain, and direction
        let dir_str = match self.direction() {
            Direction::In => "input",
            Direction::Out => "output",
            Direction::Fwd => "forward",
            Direction::None => "",
        };
        if !dir_str.is_empty() {
            parts.push(format!("{} filter {}", family, dir_str));
        } else {
            parts.push(format!("{} filter", family));
        }

        // Source IPs
        if !self.ips.0.is_empty() {
            let ips: Vec<_> = self
                .ips
                .0
                .iter()
                .filter(|ip| family == "any" || ip.family().as_str() == family)
                .map(|ip| ip.to_string())
                .collect();
            if !ips.is_empty() {
                if ips.len() > 1 {
                    parts.push(format!("saddr {{ {} }}", ips.join(", ")));
                } else {
                    parts.push(format!("saddr {}", ips[0]));
                }
            }
        }

        // Destination IPs
        if !self.ips.1.is_empty() {
            let ips: Vec<_> = self
                .ips
                .1
                .iter()
                .filter(|ip| family == "any" || ip.family().as_str() == family)
                .map(|ip| ip.to_string())
                .collect();
            if !ips.is_empty() {
                if ips.len() > 1 {
                    parts.push(format!("daddr {{ {} }}", ips.join(", ")));
                } else {
                    parts.push(format!("daddr {}", ips[0]));
                }
            }
        }

        // Protocols
        if !self.protocols.is_empty() {
            let protos: Vec<_> = self.protocols.iter().map(|p| p.to_string()).collect();
            if protos.len() > 1 {
                parts.push(format!("protocol {{ {} }}", protos.join(", ")));
            } else {
                parts.push(format!("protocol {}", protos[0]));
            }
        }

        // Source Ports
        if !self.ports.0.is_empty() {
            let ports: Vec<_> = self.ports.0.iter().map(|p| p.to_string()).collect();
            if ports.len() > 1 {
                parts.push(format!("sport {{ {} }}", ports.join(", ")));
            } else {
                parts.push(format!("sport {}", ports[0]));
            }
        }

        // Destination Ports
        if !self.ports.1.is_empty() {
            let ports: Vec<_> = self.ports.1.iter().map(|p| p.to_string()).collect();
            if ports.len() > 1 {
                parts.push(format!("dport {{ {} }}", ports.join(", ")));
            } else {
                parts.push(format!("dport {}", ports[0]));
            }
        }

        // Action
        let action_str = self.action().to_string();
        if !action_str.is_empty() {
            parts.push(action_str);
        }

        // Name (if present)
        if !self.name.is_empty() {
            parts.push(format!("name \"{}\"", self.name));
        }

        // Comment
        if !self.comment.is_empty() {
            parts.push(format!("comment \"{}\"", self.comment));
        }

        parts.join(" ")
    }
    /// Parse a rule string in nftables style into a Rule struct.
    pub fn from_str(s: &str) -> std::result::Result<Self, ParseIntError> {
        let mut rule = Self::default();
        let mut tokens = s.trim().split_whitespace().peekable();

        while let Some(token) = tokens.next() {
            let token_lc = token.to_lowercase();
            match token_lc.as_str() {
                "ip" => rule.family = Family::Any,
                "ip4" => rule.family = Family::V4,
                "ip6" => rule.family = Family::V6,
                "input" => rule.direction = Direction::In,
                "output" => rule.direction = Direction::Out,
                "forward" => rule.direction = Direction::Fwd,
                "saddr" | "daddr" => {
                    let is_src = token_lc == "saddr";
                    let ip_set = if is_src {
                        &mut rule.ips.0
                    } else {
                        &mut rule.ips.1
                    };
                    if let Some(next) = tokens.peek() {
                        if next.starts_with('{') {
                            let mut block = String::new();
                            while let Some(ip_token) = tokens.next() {
                                block.push_str(ip_token);
                                if ip_token.ends_with('}') {
                                    break;
                                }
                                block.push(' ');
                            }
                            let ips_trim =
                                block.trim().trim_start_matches('{').trim_end_matches('}');
                            for ip_str in
                                ips_trim.split(',').map(str::trim).filter(|s| !s.is_empty())
                            {
                                ip_set.push(Ip::from_str(ip_str)?);
                            }
                        } else if let Some(ip_token) = tokens.next() {
                            let ip = ip_token.trim_end_matches('}').trim();
                            if !ip.is_empty() {
                                ip_set.push(Ip::from_str(ip)?);
                            }
                        }
                    }
                }
                "protocol" | "proto" => {
                    if let Some(next) = tokens.peek() {
                        if next.starts_with('{') {
                            let mut block = String::new();
                            while let Some(proto_token) = tokens.next() {
                                block.push_str(proto_token);
                                if proto_token.ends_with('}') {
                                    break;
                                }
                                block.push(' ');
                            }
                            let protos_trim =
                                block.trim().trim_start_matches('{').trim_end_matches('}');
                            for proto in protos_trim
                                .split(',')
                                .map(str::trim)
                                .filter(|s| !s.is_empty())
                            {
                                rule.protocols.push(Proto::from_str(proto)?);
                            }
                        } else if let Some(proto_token) = tokens.next() {
                            let proto = proto_token.trim_end_matches('}').trim();
                            if !proto.is_empty() {
                                rule.protocols.push(Proto::from_str(proto)?);
                            }
                        }
                    }
                }
                "tcp" | "udp" | "sctp" | "icmp" | "icmpv6" | "igmp" => {
                    if rule.protocols.iter().all(|p| p.get_name() != token_lc) {
                        rule.protocols.push(Proto::from_str(&token_lc)?);
                    }
                    while let Some(next) = tokens.peek() {
                        match next.as_ref() {
                            "sport" | "dport" => {
                                let is_src = *next == "sport";
                                tokens.next();
                                if let Some(val_token) = tokens.peek() {
                                    if val_token.starts_with('{') {
                                        let mut block = String::new();
                                        while let Some(port_token) = tokens.next() {
                                            block.push_str(port_token);
                                            if port_token.ends_with('}') {
                                                break;
                                            }
                                            block.push(' ');
                                        }
                                        let ports_trim = block
                                            .trim()
                                            .trim_start_matches('{')
                                            .trim_end_matches('}');
                                        for p in ports_trim
                                            .split(',')
                                            .map(str::trim)
                                            .filter(|s| !s.is_empty())
                                        {
                                            let port =
                                                Port::from_str(&format!("{} {}", token_lc, p))?;
                                            if is_src {
                                                rule.ports.0.push(port);
                                            } else {
                                                rule.ports.1.push(port);
                                            }
                                        }
                                    } else if let Some(p) = tokens.next() {
                                        let port = p.trim();
                                        if !port.is_empty() {
                                            let port =
                                                Port::from_str(&format!("{} {}", token_lc, port))?;
                                            if is_src {
                                                rule.ports.0.push(port);
                                            } else {
                                                rule.ports.1.push(port);
                                            }
                                        }
                                    }
                                }
                            }
                            _ => break,
                        }
                    }
                }
                "sport" | "dport" => {
                    let is_src = token_lc == "sport";
                    if let Some(next) = tokens.peek() {
                        if next.starts_with('{') {
                            let mut block = String::new();
                            while let Some(port_token) = tokens.next() {
                                block.push_str(port_token);
                                if port_token.ends_with('}') {
                                    break;
                                }
                                block.push(' ');
                            }
                            let ports_trim =
                                block.trim().trim_start_matches('{').trim_end_matches('}');
                            for p in ports_trim
                                .split(',')
                                .map(str::trim)
                                .filter(|s| !s.is_empty())
                            {
                                let port = Port::from_str(p)?;
                                if is_src {
                                    rule.ports.0.push(port);
                                } else {
                                    rule.ports.1.push(port);
                                }
                            }
                        } else if let Some(p) = tokens.next() {
                            let port = Port::from_str(p.trim())?;
                            if is_src {
                                rule.ports.0.push(port);
                            } else {
                                rule.ports.1.push(port);
                            }
                        }
                    }
                }
                "accept" | "drop" | "reject" | "skip" | "return" => {
                    rule.action = Action::from_str(&token_lc).unwrap_or(Action::Drop);
                }
                "limit" | "jump" | "log" | "mark" | "meta" => {
                    if let Some(arg) = tokens.next() {
                        rule.action = Action::from_str(&format!("{} {}", token_lc, arg))
                            .unwrap_or(Action::Drop);
                    }
                }
                "comment" => {
                    if let Some(c) = tokens.next() {
                        if c.starts_with('"') {
                            let mut comment_str = c.trim_start_matches('"').to_string();
                            if !c.ends_with('"') {
                                while let Some(next_c) = tokens.next() {
                                    comment_str.push(' ');
                                    comment_str.push_str(next_c);
                                    if next_c.ends_with('"') {
                                        break;
                                    }
                                }
                            }
                            rule.comment = comment_str.trim_end_matches('"').trim().to_string();
                        } else {
                            rule.comment = c.to_string();
                        }
                    }
                }
                "name" => {
                    if let Some(n) = tokens.next() {
                        if n.starts_with('"') {
                            let mut name_str = n.trim_start_matches('"').to_string();
                            if !n.ends_with('"') {
                                while let Some(next_n) = tokens.next() {
                                    name_str.push(' ');
                                    name_str.push_str(next_n);
                                    if next_n.ends_with('"') {
                                        break;
                                    }
                                }
                            }
                            rule.name = name_str.trim_end_matches('"').trim().to_string();
                        } else {
                            rule.name = n.to_string();
                        }
                    }
                }
                _ => {}
            }
        }

        Ok(rule)
    }

    pub fn append(&mut self, other: &mut Self) {
        self.ips.0.append(&mut other.ips.0);
        self.ips.1.append(&mut other.ips.1);
        self.ports.0.append(&mut other.ports.0);
        self.ports.1.append(&mut other.ports.1);
        self.protocols.append(&mut other.protocols);
    }

    pub const ACCEPT: &'static str = "ACCEPT";
    pub const DROP: &'static str = "DROP";
    pub const REJECT: &'static str = "REJECT";
    pub const SKIP: &'static str = "SKIP";
    pub const RETURN: &'static str = "RETURN";
    pub const JUMP: &'static str = "JUMP";
    pub const LOG: &'static str = "LOG";
    pub const MARK: &'static str = "MARK";
    pub const LIMIT: &'static str = "LIMIT";
    pub const META: &'static str = "META";
    pub const SET: &'static str = "SET";
    pub const MASQUERADE: &'static str = "MASQUERADE";
    pub const SNAT: &'static str = "SNAT";
    pub const DNAT: &'static str = "DNAT";
    pub const REDIRECT: &'static str = "REDIRECT";
    pub const CT: &'static str = "CT";
    pub const QUEUE: &'static str = "QUEUE";
    pub const QUOTA: &'static str = "QUOTA";
    pub const TPROXY: &'static str = "TPROXY";
    pub const COUNTER: &'static str = "COUNTER";
    pub const HASHLIMIT: &'static str = "HASHLIMIT";
    pub const FLOW_OFFLOAD: &'static str = "FLOW_OFFLOAD";
    pub const RECLASSIFY: &'static str = "RECLASSIFY";
    pub const NONE: &'static str = "NONE";

    pub fn accept() -> String {
        Self::ACCEPT.to_string()
    }
    pub fn drop() -> String {
        Self::DROP.to_string()
    }
    pub fn reject() -> String {
        Self::REJECT.to_string()
    }
    pub fn skip() -> String {
        Self::SKIP.to_string()
    }
    pub fn return_() -> String {
        Self::RETURN.to_string()
    }
    pub fn limit(value: u32) -> String {
        format!("{} {}", Self::LIMIT, value)
    }
    pub fn jump(value: u32) -> String {
        format!("{} {}", Self::JUMP, value)
    }
    pub fn log(msg: &str) -> String {
        format!("{} {}", Self::LOG, msg)
    }
    pub fn mark(value: u32) -> String {
        format!("{} {}", Self::MARK, value)
    }
    pub fn meta(msg: &str) -> String {
        format!("{} {}", Self::META, msg)
    }
    pub fn set(key: &str, value: &str) -> String {
        format!("{} {}", Self::SET, format!("{} {}", key, value))
    }
    pub fn masquerade() -> String {
        Self::MASQUERADE.to_string()
    }
    pub fn snat(addr: &str) -> String {
        format!("{} {}", Self::SNAT, addr)
    }
    pub fn dnat(addr: &str) -> String {
        format!("{} {}", Self::DNAT, addr)
    }
    pub fn redirect(port: u16) -> String {
        format!("{} {}", Self::REDIRECT, port)
    }
    pub fn ct() -> String {
        Self::CT.to_string()
    }
    pub fn queue() -> String {
        Self::QUEUE.to_string()
    }
    pub fn quota(bytes: u64) -> String {
        format!("{} {}", Self::QUOTA, bytes)
    }
    pub fn tproxy(port: u16, mark: u32) -> String {
        format!("{} {} {}", Self::TPROXY, port, mark)
    }
    pub fn counter() -> String {
        Self::COUNTER.to_string()
    }
    pub fn hashlimit(rate: &str, burst: Option<u32>) -> String {
        format!("{} {} {}", Self::HASHLIMIT, rate, burst.unwrap_or(0))
    }
    pub fn flow_offload() -> String {
        Self::FLOW_OFFLOAD.to_string()
    }
    pub fn reclassify() -> String {
        Self::RECLASSIFY.to_string()
    }
    pub fn none() -> String {
        Self::NONE.to_string()
    }
}

impl Default for Rule {
    fn default() -> Self {
        Self {
            ips: (Vec::new(), Vec::new()),
            ports: (Vec::new(), Vec::new()),
            protocols: Vec::new(),
            action: Action::None,
            family: Family::None,
            direction: Direction::None,
            name: String::new(),
            comment: String::new(),
        }
    }
}

impl Display for Rule {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.to_string())
    }
}

impl Eq for Rule {}
impl From<Rule>
    for (
        Vec<Ip>,
        Vec<Ip>,
        Vec<Port>,
        Vec<Port>,
        Vec<Proto>,
        Family,
        Direction,
        Action,
        String,
        String,
    )
{
    fn from(rule: Rule) -> Self {
        (
            rule.ips.0,
            rule.ips.1,
            rule.ports.0,
            rule.ports.1,
            rule.protocols,
            rule.family,
            rule.direction,
            rule.action,
            rule.name,
            rule.comment,
        )
    }
}
impl
    From<(
        Vec<Ip>,
        Vec<Ip>,
        Vec<Port>,
        Vec<Port>,
        Vec<Proto>,
        Family,
        Direction,
        Action,
        String,
        String,
    )> for Rule
{
    fn from(
        value: (
            Vec<Ip>,
            Vec<Ip>,
            Vec<Port>,
            Vec<Port>,
            Vec<Proto>,
            Family,
            Direction,
            Action,
            String,
            String,
        ),
    ) -> Self {
        Self {
            ips: (value.0, value.1),
            ports: (value.2, value.3),
            protocols: value.4,
            family: value.5,
            direction: value.6,
            action: value.7,
            name: value.8,
            comment: value.9,
        }
    }
}
impl FromStr for Rule {
    type Err = ParseIntError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::from_str(s)
    }
}
impl Hash for Rule {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.ips.0.hash(state);
        self.ips.1.hash(state);
        self.ports.0.hash(state);
        self.ports.1.hash(state);
        self.protocols.hash(state);
        self.family.hash(state);
        self.direction.hash(state);
        self.action.hash(state);
        self.name.hash(state);
        self.comment.hash(state);
    }
}
impl Ord for Rule {
    fn cmp(&self, other: &Self) -> Ordering {
        self.ips
            .0
            .cmp(&other.ips.0)
            .then_with(|| self.ips.1.cmp(&other.ips.1))
            .then_with(|| self.ports.0.cmp(&other.ports.0))
            .then_with(|| self.ports.1.cmp(&other.ports.1))
            .then_with(|| self.protocols.cmp(&other.protocols))
            .then_with(|| self.family.cmp(&other.family))
            .then_with(|| self.direction.cmp(&other.direction))
            .then_with(|| self.action.cmp(&other.action))
            .then_with(|| self.name.cmp(&other.name))
            .then_with(|| self.comment.cmp(&other.comment))
    }
}

impl PartialEq for Rule {
    fn eq(&self, other: &Self) -> bool {
        self.ips == other.ips
            && self.ports == other.ports
            && self.protocols == other.protocols
            && self.family == other.family
            && self.direction == other.direction
            && self.action == other.action
            && self.name == other.name
            && self.comment == other.comment
    }
}

impl PartialOrd for Rule {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
