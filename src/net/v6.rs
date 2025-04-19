use std::{
    cmp::Ordering,
    fmt::{Display, Error as FmtError, Formatter},
    fs::File,
    hash::{Hash, Hasher},
    io::{Error as IoError, ErrorKind as IoErrorKind, Read as IoRead, Write as IoWrite},
    num::ParseIntError,
    str::FromStr,
};
const LOGGING_ENABLED: bool = true;
pub type Addr = u128;
pub type Subnet = u8;
pub type Mask = u128;
#[derive(Debug, Clone)]
pub struct Net {
    address: Addr,
    subnet: Subnet,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Set {
    data: Vec<Net>,
}
impl Net {
    pub fn new(address: Addr, subnet: Subnet) -> Self {
        Net { address, subnet }
    }
    pub fn address(&self) -> Addr {
        self.address
    }
    pub fn subnet(&self) -> Subnet {
        self.subnet
    }
    pub fn set_address(&mut self, address: Addr) {
        self.address = address;
    }
    pub fn set_subnet(&mut self, subnet: Subnet) {
        self.subnet = subnet.min(128);
    }
    pub fn to_vec(&self) -> Vec<u8> {
        vec![
            (self.address >> 120) as u8,
            (self.address >> 112) as u8,
            (self.address >> 104) as u8,
            (self.address >> 96) as u8,
            (self.address >> 88) as u8,
            (self.address >> 80) as u8,
            (self.address >> 72) as u8,
            (self.address >> 64) as u8,
            (self.address >> 56) as u8,
            (self.address >> 48) as u8,
            (self.address >> 40) as u8,
            (self.address >> 32) as u8,
            (self.address >> 24) as u8,
            (self.address >> 16) as u8,
            (self.address >> 8) as u8,
            (self.address as u8),
            self.subnet,
        ]
    }
    pub fn from_vec(bytes: Vec<u8>) -> Self {
        if bytes.len() != 17 {
            panic!("The vector must contain exactly 17 bytes");
        }
        Net {
            address: (bytes[0] as Addr) << 120
                | (bytes[1] as Addr) << 112
                | (bytes[2] as Addr) << 104
                | (bytes[3] as Addr) << 96
                | (bytes[4] as Addr) << 88
                | (bytes[5] as Addr) << 80
                | (bytes[6] as Addr) << 72
                | (bytes[7] as Addr) << 64
                | (bytes[8] as Addr) << 56
                | (bytes[9] as Addr) << 48
                | (bytes[10] as Addr) << 40
                | (bytes[11] as Addr) << 32
                | (bytes[12] as Addr) << 24
                | (bytes[13] as Addr) << 16
                | (bytes[14] as Addr) << 8
                | (bytes[15] as Addr),
            subnet: bytes[16],
        }
    }
    pub fn to_bytes(&self) -> [u8; 17] {
        [
            (self.address >> 120) as u8,
            (self.address >> 112) as u8,
            (self.address >> 104) as u8,
            (self.address >> 96) as u8,
            (self.address >> 88) as u8,
            (self.address >> 80) as u8,
            (self.address >> 72) as u8,
            (self.address >> 64) as u8,
            (self.address >> 56) as u8,
            (self.address >> 48) as u8,
            (self.address >> 40) as u8,
            (self.address >> 32) as u8,
            (self.address >> 24) as u8,
            (self.address >> 16) as u8,
            (self.address >> 8) as u8,
            (self.address as u8),
            self.subnet,
        ]
    }
    pub fn from_bytes(bytes: [u8; 17]) -> Self {
        Net {
            address: (bytes[0] as Addr) << 120
                | (bytes[1] as Addr) << 112
                | (bytes[2] as Addr) << 104
                | (bytes[3] as Addr) << 96
                | (bytes[4] as Addr) << 88
                | (bytes[5] as Addr) << 80
                | (bytes[6] as Addr) << 72
                | (bytes[7] as Addr) << 64
                | (bytes[8] as Addr) << 56
                | (bytes[9] as Addr) << 48
                | (bytes[10] as Addr) << 40
                | (bytes[11] as Addr) << 32
                | (bytes[12] as Addr) << 24
                | (bytes[13] as Addr) << 16
                | (bytes[14] as Addr) << 8
                | (bytes[15] as Addr),
            subnet: bytes[16],
        }
    }
    pub fn to_string(&self) -> String {
        format!("{:016x}/{}", self.address, self.subnet)
    }
    pub fn from_str(s: &str) -> Result<Self, ParseIntError> {
        let mut address = 0;
        for (i, byte) in s.split(':').enumerate() {
            let byte = u16::from_str_radix(byte, 16)?;
            address |= (byte as Addr) << (16 * i);
        }
        if s.contains('/') {
            let subnet = s.split('/').last().unwrap().parse::<Subnet>().unwrap();
            Ok(Net { address, subnet })
        } else {
            Ok(Net {
                address,
                subnet: 128,
            })
        }
    }
}
impl Default for Net {
    fn default() -> Net {
        Net {
            address: 0,
            subnet: 128,
        }
    }
}
impl Ord for Net {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.address.cmp(&other.address) {
            Ordering::Equal => self.subnet.cmp(&other.subnet),
            other => other,
        }
    }
}
impl PartialEq for Net {
    fn eq(&self, other: &Self) -> bool {
        self.address == other.address && self.subnet == other.subnet
    }
}
impl PartialOrd for Net {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Eq for Net {}
impl Hash for Net {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.address.hash(state);
        self.subnet.hash(state);
    }
}
impl Display for Net {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        write!(f, "{}", self.to_string())
    }
}
impl From<Addr> for Net {
    fn from(v: Addr) -> Self {
        Net {
            address: v,
            subnet: 128,
        }
    }
}
impl From<(Addr, Subnet)> for Net {
    fn from(v: (Addr, Subnet)) -> Self {
        Net {
            address: v.0,
            subnet: v.1,
        }
    }
}
impl FromStr for Net {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Net::from_str(s)
    }
}
impl From<Net> for String {
    fn from(v: Net) -> Self {
        v.to_string()
    }
}
struct Range {
    start: Addr,
    end: Addr,
}
impl Range {
    pub fn new() -> Self {
        Range { start: 0, end: 0 }
    }
}
impl Display for Range {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        write!(f, "{}-{}", self.start, self.end)
    }
}
impl From<Range> for String {
    fn from(v: Range) -> Self {
        v.to_string()
    }
}
impl FromStr for Range {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split('-');
        let start = parts.next().unwrap();
        let end = parts.next().unwrap();
        Ok(Range {
            start: Addr::from_str(start)?,
            end: Addr::from_str(end)?,
        })
    }
}

impl Set {
    const VALID_CHARS: &'static [char] = &[
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F', 'a', 'b',
        'c', 'd', 'e', 'f', ':', '/',
    ];
    pub fn new() -> Set {
        Set { data: Vec::new() }
    }
    pub fn push(&mut self, ip: &Net) {
        self.data.push(ip.clone());
    }
    pub fn pop(&mut self) -> Option<Net> {
        self.data.pop()
    }
    pub fn len(&self) -> usize {
        self.data.len()
    }
    pub fn clear(&mut self) {
        self.data.clear();
    }
    pub fn contains(&self, ip: &Net) -> bool {
        self.data.contains(ip)
    }
    pub fn copy(&self) -> Self {
        self.clone()
    }
    pub fn append(&mut self, other: &Set) {
        self.data.append(&mut other.data.clone());
    }
    pub fn prepend(&mut self, other: &Set) {
        let mut other_clone = other.clone();
        other_clone.append(&self);
        self.data = other_clone.data;
    }
    pub fn remove(&mut self, ip: &Net) {
        self.data.retain(|x| x != ip);
    }
    pub fn insert(&mut self, ip: Net, index: usize) {
        self.data.insert(index, ip);
    }
    pub fn dedup(&mut self) {
        self.data.dedup();
    }
    pub fn sort(&mut self) {
        self.data.sort_by(|lhs, rhs| {
            lhs.address
                .cmp(&rhs.address)
                .then_with(|| lhs.subnet.cmp(&rhs.subnet))
        });
    }
    pub fn to_string(&self) -> String {
        self.data
            .iter()
            .map(|ip| ip.to_string())
            .collect::<Vec<String>>()
            .join("\n")
    }
    pub fn from_str(s: &str) -> Result<Set, ParseIntError> {
        let mut ipset = Set::new();
        s.split('\n').filter(|s| !s.is_empty()).for_each(|ip_str| {
            if let Ok(ip) = Net::from_str(ip_str.trim()) {
                ipset.push(&ip);
            }
        });
        Ok(ipset)
    }
    pub fn from_file(file: &File) -> Result<Set, IoError> {
        let ipset_str = Set::read_file(file)?;
        let result = Set::from_str(&ipset_str);
        match result.is_err() {
            true => Err(IoError::new(
                IoErrorKind::InvalidInput,
                "Could Not convert string to IpSet6",
            )),
            false => Ok(result.unwrap()),
        }
    }
    pub fn read_file(mut file: &File) -> Result<String, IoError> {
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        contents = Set::filter_str(&mut contents);
        Ok(contents)
    }
    pub fn write_file(&self, mut file: &File) -> Result<(), IoError> {
        file.write_all(self.to_string().as_bytes())?;
        Ok(())
    }
    pub fn filter_str(s: &mut String) -> String {
        let mut newline = true;
        let mut buffer = String::with_capacity(s.len());
        for c in s.chars() {
            if Set::is_valid_char(c) {
                if newline {
                    buffer.push('\n');
                    newline = false;
                }
                buffer.push(c);
            } else if c == '\n' || c == '\r' || c == '\t' || c == ' ' || c == ';' || c == '=' {
                newline = true;
            }
        }
        buffer.shrink_to_fit();
        buffer
    }
    fn is_valid_char(c: char) -> bool {
        Set::VALID_CHARS.iter().any(|&valid_char| c == valid_char)
    }
}
impl Default for Set {
    fn default() -> Set {
        Set::new()
    }
}
impl FromStr for Set {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Set, Self::Err> {
        Set::from_str(s)
    }
}
impl Display for Set {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        let s = self.to_string();
        write!(f, "{}", s)
    }
}
