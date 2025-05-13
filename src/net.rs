pub mod v4;
pub mod v6;
use std::{
    cmp::Ordering,
    fmt::{Display, Error as FmtError, Formatter},
    fs::File,
    io::Read,
    num::ParseIntError,
    ops::{Index, IndexMut},
    path::Path,
    str::FromStr,
    sync::LazyLock,
};

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Family {
    V4 = 4,
    V6 = 6,
    ANY = 0,
    NONE = 255,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Ip4(u8, u8, u8, u8, u8);
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Ip6(
    u8,
    u8,
    u8,
    u8,
    u8,
    u8,
    u8,
    u8,
    u8,
    u8,
    u8,
    u8,
    u8,
    u8,
    u8,
    u8,
    u8,
);

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Ip {
    V4([u8; 5]),
    V6([u8; 17]),
    ANY(Vec<u8>),
    NONE,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct RuleSet {
    ips: Vec<Ip>,
    protocols: Vec<Protocol>,
    ports: Vec<Port>,
    action: Action,
    comment: String,
    family: Family,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Action {
    ACCEPT,
    DROP,
    REJECT,
    SKIP,
    LIMIT,
    RETURN,
    JUMP,
    LOG,
    MARK,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Protocol(u8, String);

#[derive(Debug, Eq, PartialEq)]
pub struct Port(u16, u8, String);

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Mask4(u8, u8, u8, u8);

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Mask6(
    u8,
    u8,
    u8,
    u8,
    u8,
    u8,
    u8,
    u8,
    u8,
    u8,
    u8,
    u8,
    u8,
    u8,
    u8,
    u8,
    u8,
    u8,
    u8,
    u8,
    u8,
    u8,
    u8,
    u8,
    u8,
    u8,
    u8,
    u8,
    u8,
    u8,
    u8,
    u8,
    u8,
);

impl Ip4 {
    pub fn new() -> Self {
        Self(0, 0, 0, 0, 0)
    }
    pub fn default() -> Self {
        Self(0, 0, 0, 0, 32)
    }
    pub fn name() -> &'static str {
        "Ip4"
    }

    fn from_str(s: &str) -> Result<Self, ParseIntError> {
        if s.contains('/') {
            let parts: Vec<&str> = s.split('/').collect();
            if parts.len() != 2 {
                panic!("Invalid format");
            }
            let addr = match Self::from_str(parts[0]) {
                Ok(addr) => addr,
                Err(e) => return Err(e),
            };
            let subnet = match u8::from_str(parts[1]) {
                Ok(subnet) => subnet.min(32),
                Err(e) => return Err(e),
            };
            return Ok(Self(addr.0, addr.1, addr.2, addr.3, subnet));
        }
        let octets: Vec<&str> = s.split('.').collect();
        if octets.len() != 4 {
            panic!("Invalid number of octets in string");
        }
        let mut addr = [0u8; 4];
        for (i, octet) in octets.iter().enumerate() {
            addr[i] = match octet.parse::<u8>() {
                Ok(o) => o,
                Err(e) => return Err(e),
            };
        }
        Ok(Ip4(addr[0], addr[1], addr[2], addr[3], 32))
    }

    pub fn to_string(&self) -> String {
        format!("{}.{}.{}.{}/{}", self.0, self.1, self.2, self.3, self.4)
    }

    pub fn from_vec(bytes: Vec<u8>) -> Result<Self, &'static str> {
        if bytes.len() != 5 {
            return Err("The vector must contain exactly 4 or 5 bytes");
        }
        if bytes.len() == 4 {
            Ok(Self(bytes[0], bytes[1], bytes[2], bytes[3], 32))
        } else {
            Ok(Self(bytes[0], bytes[1], bytes[2], bytes[3], bytes[4]))
        }
    }

    pub fn to_vec(&self) -> Vec<u8> {
        vec![self.0, self.1, self.2, self.3, self.4]
    }

    pub fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        write!(f, "{}.{}.{}.{}/{}", self.0, self.1, self.2, self.3, self.4)
    }

    pub const fn size_of() -> usize {
        5
    }

    pub fn abs_diff(&self, other: &Self) -> Self {
        let mut v1 = 0u32;
        let mut v2 = 0u32;
        for i in 0..3 {
            v1 |= (self[i] as u32) << (24 - i * 8);
            v2 |= (other[i] as u32) << (24 - i * 8);
        }
        let addr = v1.abs_diff(v2).to_be_bytes();
        Self(
            addr[0],
            addr[1],
            addr[2],
            addr[3],
            self.4.abs_diff(other.4).min(32),
        )
    }
    pub fn to_be_bytes(&self) -> [u8; 5] {
        [self.0, self.1, self.2, self.3, self.4]
    }

    pub fn from_be_bytes(bytes: [u8; 5]) -> Self {
        Self(bytes[0], bytes[1], bytes[2], bytes[3], bytes[4])
    }

    pub fn is_valid_str(&self, s: &str) -> bool {
        if s.is_empty() {
            return false;
        }
        let mut dot_count = 0;
        let mut slash_count = 0;
        let mut num_count = 0;
        let mut prev = ' ';
        for (i, c) in s.char_indices() {
            if (c == '.' && (i == 0 || i == s.len() - 1))
                || (c == '/' && (i < 7 || i == s.len() - 1))
            {
                return false;
            } else if c == '.' {
                dot_count += 1;
                if dot_count > 3 || (prev == '.' || prev == '/') {
                    return false;
                }
            } else if c == '/' {
                slash_count += 1;
                if slash_count > 1 || (prev == '.') {
                    return false;
                }
            } else if c >= '0' && c <= '9' {
                num_count += 1;
                if num_count / dot_count > 3 {
                    return false;
                }
            }
            prev = c;
        }
        if dot_count != 3 {
            return false;
        }
        true
    }
}

impl Display for Ip4 {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        write!(f, "{}.{}.{}.{}", self.0, self.1, self.2, self.3)
    }
}

impl FromStr for Ip4 {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ip4::from_str(s)
    }
}

impl From<Ip4> for (u32, u8) {
    fn from(value: Ip4) -> Self {
        (
            ((value.0 as u32) << 24)
                | ((value.1 as u32) << 16)
                | ((value.2 as u32) << 8)
                | (value.3 as u32),
            value.4.min(32),
        )
    }
}

impl TryFrom<(u32, u8)> for Ip4 {
    type Error = &'static str;
    fn try_from(value: (u32, u8)) -> Result<Self, Self::Error> {
        Ok(Self(
            (value.0 >> 24) as u8,
            (value.0 >> 16) as u8,
            (value.0 >> 8) as u8,
            (value.0) as u8,
            value.1.min(32),
        ))
    }
}

impl Index<usize> for Ip4 {
    type Output = u8;
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            3 => &self.3,
            4 => &self.4,
            _ => panic!("Index out of bounds"),
        }
    }
}

impl IndexMut<usize> for Ip4 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.0,
            1 => &mut self.1,
            2 => &mut self.2,
            3 => &mut self.3,
            4 => &mut self.4,
            _ => panic!("Index out of bounds"),
        }
    }
}

impl TryFrom<&[u8]> for Ip4 {
    type Error = &'static str;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() == 4 {
            Ok(Self(value[0], value[1], value[2], value[3], 32))
        } else if value.len() == 5 {
            Ok(Self(value[0], value[1], value[2], value[3], value[4]))
        } else {
            Err("The slice must contain exactly 4 or 5 bytes")
        }
    }
}

impl TryFrom<Vec<u8>> for Ip4 {
    type Error = &'static str;
    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        if value.len() == 4 {
            Ok(Self(value[0], value[1], value[2], value[3], 32))
        } else if value.len() == 5 {
            Ok(Self(value[0], value[1], value[2], value[3], value[4]))
        } else {
            Err("The vector must contain exactly 4 or 5 bytes")
        }
    }
}
impl Ip6 {
    pub fn new() -> Self {
        Self(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
    }

    pub fn name() -> String {
        "Ip6".to_string()
    }

    pub fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        write!(
            f,
            "{:04x}:{:04x}:{:04x}:{:04x}:{:04x}:{:04x}:{:04x}:{:04x}/{}",
            (self.0 as u16) << 8 | self.1 as u16,
            (self.2 as u16) << 8 | self.3 as u16,
            (self.4 as u16) << 8 | self.5 as u16,
            (self.6 as u16) << 8 | self.7 as u16,
            (self.8 as u16) << 8 | self.9 as u16,
            (self.10 as u16) << 8 | self.11 as u16,
            (self.12 as u16) << 8 | self.13 as u16,
            (self.14 as u16) << 8 | self.15 as u16,
            self.16
        )
    }

    pub fn size_of() -> usize {
        17
    }
    pub fn to_be_bytes(&self) -> [u8; 17] {
        [
            self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9,
            self.10, self.11, self.12, self.13, self.14, self.15, self.16,
        ]
    }
    pub fn from_be_bytes(bytes: [u8; 17]) -> Self {
        Self(
            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
            bytes[8], bytes[9], bytes[10], bytes[11], bytes[12], bytes[13], bytes[14], bytes[15],
            bytes[16],
        )
    }
    pub fn to_vec(&self) -> Vec<u8> {
        vec![
            self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9,
            self.10, self.11, self.12, self.13, self.14, self.15, self.16,
        ]
    }
    pub fn from_vec(vec: Vec<u8>) -> Self {
        if vec.len() != 17 {
            panic!("Invalid vector length");
        }
        Self(
            vec[0], vec[1], vec[2], vec[3], vec[4], vec[5], vec[6], vec[7], vec[8], vec[9],
            vec[10], vec[11], vec[12], vec[13], vec[14], vec[15], vec[16],
        )
    }
    pub fn from_str(s: &str) -> Result<Self, ParseIntError> {
        if s.is_empty() {
            panic!("Empty string");
        }
        let mut octets: [u8; 17] = [0; 17];
        let (ip_part, subnet_part) = if s.contains('/') {
            let parts: Vec<&str> = s.split('/').collect();
            if parts.len() != 2 {
                panic!("Invalid format");
            }
            (parts[0], Some(parts[1]))
        } else {
            (s, None)
        };

        let segments: Vec<&str> = ip_part.split(':').collect();
        let mut idx = 0;
        let len = segments.len();
        for segment in segments {
            if !segment.is_empty() {
                match u16::from_str_radix(segment, 16) {
                    Ok(segment_value) => {
                        octets[idx] = (segment_value >> 8) as u8;
                        octets[idx + 1] = segment_value as u8;
                        idx += 2;
                    }
                    Err(e) => return Err(e),
                }
            } else {
                idx += 2 * (8 - len + 1); // Skip zeros
            }
        }

        if let Some(subnet_str) = subnet_part {
            octets[16] = match u8::from_str(subnet_str) {
                Ok(subnet) => subnet.min(128),
                Err(e) => return Err(e),
            };
        } else {
            octets[16] = 128;
        }

        Ok(Self::from_be_bytes(octets))
    }
    pub fn is_valid_str(&self, s: &str) -> bool {
        if s.is_empty() {
            return false;
        }

        let mut colon_count = 0;
        let mut slash_count = 0;
        let mut prev = ' ';
        for c in s.chars() {
            match c {
                ':' => {
                    if prev == ':' || prev == '/' || colon_count >= 7 {
                        return false;
                    }
                    colon_count += 1;
                }
                '/' => {
                    if prev == ':' || slash_count >= 1 {
                        return false;
                    }
                    slash_count += 1;
                }
                '0'..='9' | 'a'..='f' | 'A'..='F' => {}
                _ => return false,
            }
            prev = c;
        }
        colon_count <= 7 && colon_count >= 2 && slash_count <= 1
    }
}
impl Display for Ip6 {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        Self::fmt(self, f)
    }
}
impl FromStr for Ip6 {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_str(s)
    }
}
impl From<[u8; 17]> for Ip6 {
    fn from(value: [u8; 17]) -> Self {
        Self(
            value[0], value[1], value[2], value[3], value[4], value[5], value[6], value[7],
            value[8], value[9], value[10], value[11], value[12], value[13], value[14], value[15],
            value[16],
        )
    }
}
impl From<[u16; 9]> for Ip6 {
    fn from(value: [u16; 9]) -> Self {
        Self(
            value[0] as u8,
            (value[0] >> 8) as u8,
            value[1] as u8,
            (value[1] >> 8) as u8,
            value[2] as u8,
            (value[2] >> 8) as u8,
            value[3] as u8,
            (value[3] >> 8) as u8,
            value[4] as u8,
            (value[4] >> 8) as u8,
            value[5] as u8,
            (value[5] >> 8) as u8,
            value[6] as u8,
            (value[6] >> 8) as u8,
            value[7] as u8,
            (value[7] >> 8) as u8,
            value[8].min(128) as u8,
        )
    }
}
impl TryFrom<&[u8]> for Ip6 {
    type Error = &'static str;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let mut subnet = 128;
        if value.len() == 17 {
            subnet = value[16].min(128);
        }
        if value.len() == 16 || value.len() == 17 {
            Ok(Self(
                value[0], value[1], value[2], value[3], value[4], value[5], value[6], value[7],
                value[8], value[9], value[10], value[11], value[12], value[13], value[14],
                value[15], subnet,
            ))
        } else {
            Err("The slice must contain exactly 16 or 17 bytes")
        }
    }
}
impl TryFrom<Vec<u8>> for Ip6 {
    type Error = &'static str;
    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        let mut subnet = 128;
        if value.len() == 17 {
            subnet = value[16].min(128);
        }
        if value.len() == 16 || value.len() == 17 {
            Ok(Self(
                value[0], value[1], value[2], value[3], value[4], value[5], value[6], value[7],
                value[8], value[9], value[10], value[11], value[12], value[13], value[14],
                value[15], subnet,
            ))
        } else {
            Err("The vector must contain exactly 16 or 17 bytes")
        }
    }
}
impl TryFrom<&[u16]> for Ip6 {
    type Error = &'static str;
    fn try_from(value: &[u16]) -> Result<Self, Self::Error> {
        let mut subnet = 128;
        if value.len() == 9 {
            subnet = value[8].min(128) as u8;
        }
        if value.len() == 8 || value.len() == 9 {
            Ok(Self(
                value[0] as u8,
                (value[0] >> 8) as u8,
                value[1] as u8,
                (value[1] >> 8) as u8,
                value[2] as u8,
                (value[2] >> 8) as u8,
                value[3] as u8,
                (value[3] >> 8) as u8,
                value[4] as u8,
                (value[4] >> 8) as u8,
                value[5] as u8,
                (value[5] >> 8) as u8,
                value[6] as u8,
                (value[6] >> 8) as u8,
                value[7] as u8,
                (value[7] >> 8) as u8,
                subnet,
            ))
        } else {
            Err("The slice must contain exactly 8 or 9 elements (16 or 18 bytes)")
        }
    }
}
impl TryFrom<Vec<u16>> for Ip6 {
    type Error = &'static str;
    fn try_from(value: Vec<u16>) -> Result<Self, Self::Error> {
        let mut subnet = 128;
        if value.len() == 9 {
            subnet = value[8].min(128) as u8;
        }
        if value.len() == 8 || value.len() == 9 {
            Ok(Self(
                value[0] as u8,
                (value[0] >> 8) as u8,
                value[1] as u8,
                (value[1] >> 8) as u8,
                value[2] as u8,
                (value[2] >> 8) as u8,
                value[3] as u8,
                (value[3] >> 8) as u8,
                value[4] as u8,
                (value[4] >> 8) as u8,
                value[5] as u8,
                (value[5] >> 8) as u8,
                value[6] as u8,
                (value[6] >> 8) as u8,
                value[7] as u8,
                (value[7] >> 8) as u8,
                subnet,
            ))
        } else {
            Err("The vector must contain exactly 8 or 9 elements (16 or 18 bytes)")
        }
    }
}

impl Ip {
    pub fn new() -> Self {
        Self::ANY(Vec::new())
    }

    pub fn new_v4(addr: [u8; 5]) -> Self {
        Self::V4(addr)
    }

    pub fn new_v6(addr: [u8; 17]) -> Self {
        Self::V6(addr)
    }

    pub fn name() -> String {
        "Ip".to_string()
    }
    pub fn clone(&self) -> Self {
        match self {
            Ip::V4(addr) => Ip::V4(*addr),
            Ip::V6(addr) => Ip::V6(*addr),
            Ip::ANY(addr) => Ip::ANY(addr.clone()),
            Ip::NONE => Ip::NONE,
        }
    }
    pub fn len(&self) -> usize {
        match self {
            Ip::V4(_) => 5,
            Ip::V6(_) => 17,
            Ip::ANY(a) => a.len(),
            Ip::NONE => 0,
        }
    }
    pub fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
    pub fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Ip::V4(a), Ip::V4(b)) => a.cmp(b),
            (Ip::V4(a), Ip::V6(b)) => a.to_vec().cmp(&b.to_vec()),
            (Ip::V4(a), Ip::ANY(b)) => a.to_vec().cmp(&b.to_vec()),
            (Ip::V6(a), Ip::V4(b)) => a.to_vec().cmp(&b.to_vec()),
            (Ip::V6(a), Ip::V6(b)) => a.cmp(b),
            (Ip::V6(a), Ip::ANY(b)) => a.to_vec().cmp(&b.to_vec()),
            (Ip::ANY(a), Ip::V4(b)) => a.to_vec().cmp(&b.to_vec()),
            (Ip::ANY(a), Ip::V6(b)) => a.to_vec().cmp(&b.to_vec()),
            (Ip::ANY(a), Ip::ANY(b)) => a.cmp(b),
            (Ip::NONE, Ip::NONE) => Ordering::Equal,
            (Ip::NONE, _) => Ordering::Less,
            (_, Ip::NONE) => Ordering::Greater,
        }
    }
    pub fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Ip::V4(a), Ip::V4(b)) => a == b,
            (Ip::V6(a), Ip::V6(b)) => a == b,
            (Ip::ANY(a), Ip::ANY(b)) => a == b,
            (Ip::NONE, Ip::NONE) => true,
            _ => false,
        }
    }
    pub fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
    pub fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        write!(f, "{}", self.to_string())
    }
    pub fn to_string(&self) -> String {
        match self {
            Ip::V4(addr) => format!(
                "{}.{}.{}.{}/{}",
                addr[0],
                addr[1],
                addr[2],
                addr[3],
                addr[4].min(32)
            ),
            Ip::V6(addr) => format!(
                "{:02x}{:02x}:{:02x}{:02x}:{:02x}{:02x}:{:02x}{:02x}:{:02x}{:02x}:{:02x}{:02x}:{:02x}{:02x}:{:02x}{:02x}/{}",
                addr[0],
                addr[1],
                addr[2],
                addr[3],
                addr[4],
                addr[5],
                addr[6],
                addr[7],
                addr[8],
                addr[9],
                addr[10],
                addr[11],
                addr[12],
                addr[13],
                addr[14],
                addr[15],
                addr[16].min(128)
            ),
            Ip::ANY(addr) => format!("{:?}", addr),
            Ip::NONE => "".to_string(),
        }
    }

    pub fn family(&self) -> Family {
        match self {
            Ip::V4(_) => Family::V4,
            Ip::V6(_) => Family::V6,
            Ip::ANY(_) => Family::ANY,
            Ip::NONE => Family::NONE,
        }
    }

    pub fn is_valid_char(&self, c: char) -> bool {
        match self {
            Ip::V4(_) => c.is_digit(10) || c == '.' || c == '/',
            Ip::V6(_) => c.is_digit(10) || c.is_ascii_hexdigit() || c == ':' || c == '/',
            Ip::ANY(_) => {
                c.is_digit(10) || c.is_ascii_hexdigit() || c == ':' || c == '.' || c == '/'
            }
            Ip::NONE => false,
        }
    }
    pub fn is_valid_str(s: &str, family: Family) -> bool {
        match family {
            Family::V4 => Self::is_valid_v4_str(s),
            Family::V6 => Self::is_valid_v6_str(s),
            Family::ANY => Self::is_valid_v4_str(s) || Self::is_valid_v6_str(s),
            Family::NONE => false,
        }
    }
    fn is_valid_v4_str(s: &str) -> bool {
        if s.is_empty() {
            return false;
        }
        if s.contains('.') && !s.contains(':') {
            let mut dot_count = 0;
            let mut slash_count = 0;
            let mut num_count = 0;
            let mut prev = ' ';
            for (i, c) in s.char_indices() {
                if (c == '.' && (i == 0 || i == s.len() - 1))
                    || (c == '/' && (i < 7 || i == s.len() - 1))
                {
                    return false;
                } else if c == '.' {
                    dot_count += 1;
                    if dot_count > 3 || (prev == '.' || prev == '/') {
                        return false;
                    }
                } else if c == '/' {
                    slash_count += 1;
                    if slash_count > 1 || (prev == '.') {
                        return false;
                    }
                } else if c >= '0' && c <= '9' {
                    num_count += 1;
                    if num_count / dot_count > 3 {
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
            let mut colon_count: u32 = 0;
            let mut slash_count: u32 = 0;
            let mut num_count: u32 = 0;
            let mut prev: char = ' ';
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
                } else if (c >= '0' && c <= '9') || (c >= 'a' && c <= 'f') || (c >= 'A' && c <= 'F')
                {
                    num_count += 1;
                    if num_count / colon_count > 4 {
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
    pub fn from_str(s: &str) -> Result<Self, ParseIntError> {
        if s.is_empty() {
            panic!("Empty string");
        }
        let s = s.trim();
        if s.contains('.') && !s.contains(':') {
            let mut addr: [u8; 5] = [0; 5];
            let (ip_part, subnet_part) = if s.contains('/') {
                let parts: Vec<&str> = s.split('/').collect();
                if parts.len() != 2 {
                    panic!("Invalid format");
                }
                (parts[0], Some(parts[1]))
            } else {
                (s, None)
            };
            for (idx, octet) in ip_part.split('.').enumerate() {
                if octet.is_empty() {
                    panic!("Empty octet");
                }
                match octet.parse::<u8>() {
                    Ok(octet) => addr[idx] = octet,
                    Err(e) => return Err(e),
                };
            }
            if let Some(subnet_str) = subnet_part {
                match subnet_str.parse::<u8>() {
                    Ok(subnet) => addr[4] = subnet.min(32),
                    Err(e) => return Err(e),
                };
            } else {
                addr[4] = 32;
            }
            return Ok(Ip::V4(addr));
        } else if s.contains(':') && !s.contains('.') {
            let mut addr: [u8; 17] = [0; 17];
            let (ip_part, subnet_part) = if s.contains('/') {
                let parts: Vec<&str> = s.split('/').collect();
                (parts[0], Some(parts[1]))
            } else {
                (s, None)
            };
            for (idx, octet) in ip_part.split(':').enumerate() {
                match u16::from_str(octet) {
                    Ok(bytes) => {
                        addr[idx * 2] = bytes as u8;
                        addr[idx * 2 + 1] = (bytes >> 8) as u8;
                    }
                    Err(e) => return Err(e),
                }
            }
            if let Some(subnet_str) = subnet_part {
                match subnet_str.parse::<u8>() {
                    Ok(subnet) => addr[16] = subnet.min(128),
                    Err(e) => return Err(e),
                }
            } else {
                addr[16] = 128;
            }
            return Ok(Self::V6(addr));
        } else {
            panic!("Invalid ip data in string");
        }
    }

    pub fn address(&self) -> Vec<u8> {
        match self {
            Ip::V4(addr) => addr[..4].to_vec(),
            Ip::V6(addr) => addr[..16].to_vec(),
            Ip::ANY(addr) => addr[..addr.len() - 1].to_vec(),
            Ip::NONE => vec![],
        }
    }

    pub fn subnet(&self) -> u8 {
        match self {
            Ip::V4(addr) => addr[4],
            Ip::V6(addr) => addr[16],
            Ip::ANY(addr) => addr[addr.len() - 1],
            Ip::NONE => 0,
        }
    }

    pub fn size_of(&self) -> usize {
        match self {
            Ip::V4(_) => 5,
            Ip::V6(_) => 17,
            Ip::ANY(_) => self.to_vec().len(),
            Ip::NONE => 0,
        }
    }

    pub fn to_vec(&self) -> Vec<u8> {
        match self {
            Ip::V4(addr) => addr.to_vec(),
            Ip::V6(addr) => addr.to_vec(),
            Ip::ANY(addr) => addr.clone(),
            Ip::NONE => vec![],
        }
    }
    pub fn from_vec(bytes: Vec<u8>) -> Self {
        match bytes.len() {
            4 => Self::V4([bytes[0], bytes[1], bytes[2], bytes[3], 32]),
            5 => Self::V4([bytes[0], bytes[1], bytes[2], bytes[3], bytes[4].min(32)]),
            16 => Self::V6([
                bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
                bytes[8], bytes[9], bytes[10], bytes[11], bytes[12], bytes[13], bytes[14],
                bytes[15], 128,
            ]),
            17 => Self::V6([
                bytes[0],
                bytes[1],
                bytes[2],
                bytes[3],
                bytes[4],
                bytes[5],
                bytes[6],
                bytes[7],
                bytes[8],
                bytes[9],
                bytes[10],
                bytes[11],
                bytes[12],
                bytes[13],
                bytes[14],
                bytes[15],
                bytes[16].min(128),
            ]),
            _ => Self::NONE,
        }
    }
    fn index(&self, index: usize) -> &u8 {
        match self {
            Ip::V4(addr) => &addr[index],
            Ip::V6(addr) => &addr[index],
            Ip::NONE => panic!("Ip::NONE has no elements"),
            Ip::ANY(addr) => &addr[index],
        }
    }
    fn index_mut(&mut self, index: usize) -> &mut u8 {
        match self {
            Ip::V4(addr) => &mut addr[index],
            Ip::V6(addr) => &mut addr[index],
            Ip::NONE => panic!("Ip::NONE has no elements"),
            Ip::ANY(addr) => &mut addr[index],
        }
    }
}
impl PartialOrd for Ip {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Ip {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cmp(other)
    }
}
impl Display for Ip {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        write!(f, "{}", self.to_string())
    }
}
impl FromStr for Ip {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_str(s)
    }
}
impl From<[u8; 4]> for Ip {
    fn from(value: [u8; 4]) -> Self {
        Self::V4([value[0], value[1], value[2], value[3], 32])
    }
}
impl From<[u8; 5]> for Ip {
    fn from(value: [u8; 5]) -> Self {
        Self::V4([value[0], value[1], value[2], value[3], value[4].min(32)])
    }
}
impl From<[u8; 16]> for Ip {
    fn from(value: [u8; 16]) -> Self {
        Self::V6([
            value[0], value[1], value[2], value[3], value[4], value[5], value[6], value[7],
            value[8], value[9], value[10], value[11], value[12], value[13], value[14], value[15],
            128,
        ])
    }
}
impl From<[u8; 17]> for Ip {
    fn from(value: [u8; 17]) -> Self {
        Self::V6(value)
    }
}
impl TryFrom<&[u8]> for Ip {
    type Error = &'static str;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::from_vec(value.to_vec()))
    }
}
impl Index<usize> for Ip {
    type Output = u8;
    fn index(&self, index: usize) -> &Self::Output {
        match self {
            Ip::V4(addr) => &addr[index],
            Ip::V6(addr) => &addr[index],
            Ip::NONE => panic!("Ip::NONE has no elements"),
            Ip::ANY(addr) => &addr[index],
        }
    }
}
impl IndexMut<usize> for Ip {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match self {
            Ip::V4(addr) => &mut addr[index],
            Ip::V6(addr) => &mut addr[index],
            Ip::NONE => panic!("Ip::NONE has no elements"),
            Ip::ANY(addr) => &mut addr[index],
        }
    }
}
impl Family {
    pub fn new() -> Self {
        Family::NONE
    }
    pub fn set(&mut self, family: Family) -> Self {
        *self = family;
        self.clone()
    }
    pub fn get(&self) -> Self {
        self.clone()
    }
    pub fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        write!(f, "{}", self.to_string())
    }
    pub fn to_string(&self) -> String {
        match self {
            Family::V4 => "v4".to_string(),
            Family::V6 => "v6".to_string(),
            Family::ANY => "any".to_string(),
            Family::NONE => "none".to_string(),
        }
    }
    pub fn from_str(s: &str) -> Result<Self, ParseIntError> {
        if s.is_empty() {
            return Ok(Family::NONE);
        }
        match s.trim().to_lowercase().as_str() {
            "v4" => Ok(Family::V4),
            "v6" => Ok(Family::V6),
            "any" => Ok(Family::ANY),
            "none" => Ok(Family::NONE),
            _ => Ok(Family::NONE),
        }
    }
    pub fn from_ip(ip: &Ip) -> Self {
        match ip {
            Ip::V4(_) => Family::V4,
            Ip::V6(_) => Family::V6,
            Ip::ANY(_) => Family::ANY,
            Ip::NONE => Family::NONE,
        }
    }
}
pub static PROTO_LIST: LazyLock<Vec<Protocol>> = LazyLock::new(|| {
    Protocol::import(&Path::new("/etc/protocols")).expect("Failed to import protocol information")
});
impl Protocol {
    pub fn new() -> Self {
        Self(0, " ".to_string())
    }
    pub fn set(&mut self, number: u8, name: String) -> Self {
        self.0 = number;
        self.1 = name;
        self.clone()
    }
    pub fn clone(&self) -> Self {
        Self(self.0, self.1.clone())
    }
    pub fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
    pub fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
    pub fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
    pub fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
    pub fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        write!(f, "{}", self.to_string())
    }
    pub fn number(&self) -> u8 {
        self.0
    }
    pub fn name(&self) -> String {
        self.1.clone().split(',').next().unwrap().trim().to_string()
    }
    pub fn description(&self) -> String {
        self.1.clone().split(',').last().unwrap().trim().to_string()
    }
    pub fn set_number(&mut self, number: u8) -> Self {
        self.0 = number;
        self.clone()
    }
    pub fn set_name(&mut self, name: &str) -> Self {
        self.1 = name.to_string() + "," + self.description().as_str();
        self.clone()
    }
    pub fn set_description(&mut self, description: &str) -> Self {
        self.1 = self.name() + "," + description;
        self.clone()
    }
    pub fn to_string(&self) -> String {
        return format!(
            "Protocol({}, \"{}, {}\")",
            self.number(),
            self.name(),
            self.description()
        );
    }
    pub fn from_str(s: &str) -> Result<Self, ParseIntError> {
        let mut parts = s.trim().split_whitespace();
        let name = parts.next().unwrap_or("").trim();
        let description = parts.next().unwrap_or("").trim();
        let number = u8::from_str(name)?;
        Ok(Protocol(
            number,
            format!("{}, \"{}\"", name, description).to_string(),
        ))
    }
    pub fn from_number(number: u8) -> Self {
        PROTO_LIST
            .iter()
            .find(|p| p.0 == number)
            .unwrap_or(&Protocol(254, "".to_string()))
            .clone()
    }
    pub fn import(path: &Path) -> Result<Vec<Protocol>, std::io::Error> {
        let mut file = File::open(path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        let mut protos = Vec::<Protocol>::new();
        for line in content.lines() {
            if !line.trim().is_empty() && !line.trim().starts_with('#') {
                let line = line.trim();
                let parts: Vec<&str> = line.split(' ').collect();
                for part in parts {
                    if !part.is_empty() {
                        let mut parts = part.split_whitespace();
                        if let (Some(name), Some(number_str)) = (parts.next(), parts.next()) {
                            let description = line
                                .find('#')
                                .map(|i| line[i + 1..].trim().to_string())
                                .unwrap_or("".to_string());
                            if let Ok(number) = u8::from_str(number_str) {
                                protos.push(Protocol(
                                    number,
                                    format!("{}, {}", name, description).to_string(),
                                ));
                            }
                        }
                    }
                }
            }
        }
        Ok(protos)
    }
}
impl Clone for Protocol {
    fn clone(&self) -> Self {
        Self(self.0, self.1.clone())
    }
}
impl Display for Protocol {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        write!(f, "{}", self.to_string())
    }
}
impl FromStr for Protocol {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.trim().split_whitespace();
        let number = u8::from_str(parts.next().unwrap_or("254").trim())?;
        let name = parts.next().unwrap_or("").trim();
        let description = parts.next().unwrap_or("").trim();
        Ok(Protocol(
            number,
            format!("{},{}", name, description).to_string(),
        ))
    }
}
impl PartialOrd for Protocol {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Protocol {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}
pub static PORT_LIST: LazyLock<Vec<Port>> = LazyLock::new(|| {
    Port::import(&Path::new("/etc/services")).expect("Failed to import port information")
});
impl Port {
    pub fn new() -> Self {
        Self(0, 254, " ".to_string())
    }
    pub fn clone(&self) -> Self {
        Self(self.0.clone(), self.1.clone(), self.2.clone())
    }
    pub fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
    pub fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
    pub fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
    pub fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
    pub fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        write!(f, "{}", self.to_string())
    }
    pub fn to_string(&self) -> String {
        return format!("Port({}, {}, \"{}\")", self.0, self.1, self.2);
    }
    pub fn set(&mut self, number: u16, proto: u8) -> Self {
        self.0 = number;
        self.1 = proto;
        self.clone()
    }
    pub fn set_number(&mut self, number: u16) -> Self {
        self.0 = number;
        self.clone()
    }
    pub fn set_proto(&mut self, proto: u8) -> Self {
        self.1 = proto;
        self.clone()
    }
    pub fn set_service(&mut self, service: String) -> Self {
        self.2 = service;
        self.clone()
    }
    pub fn from_str(s: &str) -> Result<Self, ParseIntError> {
        let mut parts = s.trim().split_whitespace();
        let number = u16::from_str(parts.next().unwrap_or("0").trim())?;
        let proto = u8::from_str(parts.next().unwrap_or("254").trim())?;
        let service = parts.next().unwrap_or("").trim().to_string();
        Ok(Port(number, proto, service))
    }
    pub fn from_number(number: u16) -> Self {
        PORT_LIST
            .iter()
            .find(|p| p.0 == number)
            .unwrap_or(&Port(0, 254, "".to_string()))
            .clone()
    }
    pub fn import(path: &Path) -> Result<Vec<Port>, std::io::Error> {
        let mut file = File::open(path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        let mut ports = Vec::<Port>::new();
        for line in content.lines() {
            if !line.trim().is_empty() && !line.trim().starts_with('#') {
                let line = line.trim();
                let parts: Vec<&str> = line.split(' ').collect();
                for part in parts {
                    if !part.is_empty() {
                        let mut parts = part.split_whitespace();
                        if let (Some(service), Some(port_proto)) = (parts.next(), parts.next()) {
                            let mut port_proto_parts = port_proto.split("/");
                            if let (Some(port_str), Some(proto_str)) =
                                (port_proto_parts.next(), port_proto_parts.next())
                            {
                                let description = line
                                    .find('#')
                                    .map(|i| line[i + 1..].trim().to_string())
                                    .unwrap_or("".to_string());
                                if let Ok(port_number) = port_str.parse::<u16>() {
                                    ports.push(Port(
                                        port_number,
                                        match proto_str.trim() {
                                            "tcp" => 6,
                                            "udp" => 17,
                                            "ddp" => 37,
                                            "sctp" => 132,
                                            _ => 254,
                                        },
                                        format!("{}, {}", service, description).to_string(),
                                    ));
                                }
                            }
                        }
                    }
                }
            }
        }
        Ok(ports)
    }
}
impl Clone for Port {
    fn clone(&self) -> Self {
        Self(self.0.clone(), self.1.clone(), self.2.clone())
    }
}
impl Display for Port {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        write!(f, "{}", self.to_string())
    }
}
impl FromStr for Port {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_str(s)
    }
}
impl PartialOrd for Port {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Port {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}
impl Action {
    pub fn from_str(s: &str) -> Result<Self, ParseIntError> {
        match s.trim().to_lowercase().as_str() {
            "reject" => Ok(Action::REJECT),
            "drop" => Ok(Action::DROP),
            "accept" => Ok(Action::ACCEPT),
            "skip" => Ok(Action::SKIP),
            "limit" => Ok(Action::LIMIT),
            "return" => Ok(Action::RETURN),
            "jump" => Ok(Action::JUMP),
            "log" => Ok(Action::LOG),
            "mark" => Ok(Action::MARK),
            _ => Ok(Action::DROP),
        }
    }
    pub fn to_string(&self) -> String {
        match self {
            Action::REJECT => "reject".to_string(),
            Action::DROP => "drop".to_string(),
            Action::ACCEPT => "accept".to_string(),
            Action::SKIP => "skip".to_string(),
            Action::LIMIT => "limit".to_string(),
            Action::RETURN => "return".to_string(),
            Action::JUMP => "jump".to_string(),
            Action::LOG => "log".to_string(),
            Action::MARK => "mark".to_string(),
        }
    }
}
impl RuleSet {
    pub fn new() -> Self {
        Self {
            family: Family::NONE,
            ips: Vec::new(),
            protocols: Vec::new(),
            ports: Vec::new(),
            action: Action::DROP,
            comment: "".to_string(),
        }
    }
    pub fn clone(&self) -> Self {
        Self {
            family: self.family.clone(),
            ips: self.ips.clone(),
            protocols: self.protocols.clone(),
            ports: self.ports.clone(),
            action: self.action.clone(),
            comment: self.comment.clone(),
        }
    }
    pub fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
    pub fn cmp(&self, other: &Self) -> Ordering {
        self.ips.cmp(&other.ips)
    }
    pub fn eq(&self, other: &Self) -> bool {
        self.ips == other.ips
    }
    pub fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
    pub fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        write!(f, "{}", self.to_string())
    }
    pub fn to_string(&self) -> String {
        let mut s = String::new();
        for (i, ip) in self.ips.iter().enumerate() {
            s.push_str(&format!("{}\n", ip.to_string()));
            s.push_str(&format!("{} ", self.family.to_string()));
            s.push_str(&format!("{} ", self.protocols[i].to_string()));
            s.push_str(&format!("{} ", self.ports[i].to_string()));
            s.push_str(&format!("{} ", self.action.to_string()));
            s.push_str(&format!("{}\n", self.comment.to_string()));
        }
        s
    }
    pub fn from_str(s: &str) -> Result<Self, ParseIntError> {
        let mut parts = s.trim().split_terminator('\n');
        let mut ips = Vec::<Ip>::new();
        let mut family = Family::NONE;
        let mut protocols = Vec::<Protocol>::new();
        let mut ports = Vec::<Port>::new();
        let mut action = Action::DROP;
        let mut comment = String::new();
        while let Some(ip_str) = parts.next() {
            let ip = Ip::from_str(ip_str.trim())?;
            ips.push(ip);
            let family_str = parts.next().unwrap_or("").trim();
            family = Family::from_str(family_str)?;
            let protocol_str = parts.next().unwrap_or("").trim();
            protocols.push(Protocol::from_str(protocol_str)?);
            let port_str = parts.next().unwrap_or("").trim();
            ports.push(Port::from_str(port_str)?);
            let action_str = parts.next().unwrap_or("").trim();
            action = Action::from_str(action_str)?;
            comment = parts.next().unwrap_or("").trim().to_string();
        }
        let rule_set = RuleSet {
            family,
            ips,
            protocols,
            ports,
            action,
            comment,
        };
        Ok(rule_set)
    }
    pub fn optimize(&mut self) {
        self.ips.sort();
        self.ips.dedup();
        self.protocols.sort();
        self.protocols.dedup();
        self.ports.sort();
        self.ports.dedup();
    }
    pub fn append(&mut self, other: &mut Self) {
        self.ips.extend(other.ips.iter().cloned());
        self.protocols.extend(other.protocols.iter().cloned());
        self.ports.extend(other.ports.iter().cloned());
        self.optimize();
    }
    pub fn to_vec(&self) -> Vec<Ip> {
        self.ips.clone()
    }
    pub fn ports(&self) -> Vec<Port> {
        self.ports.clone()
    }
    pub fn protocols(&self) -> Vec<Protocol> {
        self.protocols.clone()
    }
    pub fn action(&self) -> Action {
        self.action.clone()
    }
    pub fn comment(&self) -> String {
        self.comment.clone()
    }
}
impl PartialOrd for RuleSet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for RuleSet {
    fn cmp(&self, other: &Self) -> Ordering {
        self.ips.cmp(&other.ips)
    }
}
impl Display for RuleSet {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        write!(f, "{}", self.to_string())
    }
}
impl FromStr for RuleSet {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_str(s)
    }
}
