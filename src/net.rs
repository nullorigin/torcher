pub mod v4;
pub mod v6;
use std::{
    cmp::Ordering,
    fmt::{Display, Error as FmtError, Formatter},
    fs::{File, read_to_string},
    io::{Error as IoError, ErrorKind as IoErrorKind, Read},
    num::ParseIntError,
    ops::{Index, IndexMut},
    path::{Path, PathBuf},
    ptr::read,
    str::FromStr,
    sync::LazyLock,
};

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
pub struct Ip(Vec<u8>);

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Family {
    V4,
    V6,
    ANY,
    NONE,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Proto(u8, String);

#[derive(Debug, Clone, Eq, PartialEq)]
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
        Self(vec![])
    }

    pub fn new_v4(addr: [u8; 5]) -> Self {
        Self(vec![addr[0], addr[1], addr[2], addr[3], addr[4].min(32)])
    }

    pub fn new_v6(addr: [u8; 17]) -> Self {
        Self(vec![
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
            addr[16].min(128),
        ])
    }

    pub fn name() -> String {
        "Ip".to_string()
    }

    pub fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        write!(f, "{}", self.to_string())
    }

    pub fn to_string(&self) -> String {
        match self.0.len() {
            5 => format!(
                "{}.{}.{}.{}/{}",
                self.0[0],
                self.0[1],
                self.0[2],
                self.0[3],
                self.0[4].min(32)
            ),
            17 => format!(
                "{:02x}{:02x}:{:02x}{:02x}:{:02x}{:02x}:{:02x}{:02x}:{:02x}{:02x}:{:02x}{:02x}:{:02x}{:02x}:{:02x}{:02x}/{}",
                self.0[0],
                self.0[1],
                self.0[2],
                self.0[3],
                self.0[4],
                self.0[5],
                self.0[6],
                self.0[7],
                self.0[8],
                self.0[9],
                self.0[10],
                self.0[11],
                self.0[12],
                self.0[13],
                self.0[14],
                self.0[15],
                self.0[16].min(128)
            ),
            _ => panic!("Invalid IP length: {}", self.0.len()),
        }
    }

    pub fn family(&self) -> Family {
        match self.0.len() {
            5 => Family::V4,
            17 => Family::V6,
            _ => Family::NONE,
        }
    }

    pub fn from_str(s: &str) -> Result<Self, ParseIntError> {
        if s.is_empty() {
            panic!("Empty string");
        }

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
                }
            }
            if let Some(subnet_str) = subnet_part {
                match subnet_str.parse::<u8>() {
                    Ok(subnet) => addr[4] = subnet.min(32),
                    Err(e) => return Err(e),
                }
            }
            return Ok(Self(addr.to_vec()));
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
            }
            return Ok(Self(addr.to_vec()));
        } else {
            panic!("Invalid ip data in string");
        }
    }

    pub fn address(&self) -> Vec<u8> {
        match self.0.len() {
            5 => self.0[..4].to_vec(),
            17 => self.0[..16].to_vec(),
            _ => panic!("Invalid IP length: {}", self.0.len()),
        }
    }

    pub fn subnet(&self) -> u8 {
        match self.0.len() {
            5 => self.0[4],
            17 => self.0[16],
            _ => panic!("Invalid IP length: {}", self.0.len()),
        }
    }

    pub fn size_of(&self) -> usize {
        match self.0.len() {
            5 => 4,
            17 => 16,
            _ => panic!("Invalid IP length: {}", self.0.len()),
        }
    }

    pub fn to_vec(&self) -> Vec<u8> {
        self.0.clone()
    }

    pub fn from_vec(bytes: Vec<u8>) -> Self {
        match bytes.len() {
            4 => Self(vec![bytes[0], bytes[1], bytes[2], bytes[3], 32]),
            5 => Self(vec![
                bytes[0],
                bytes[1],
                bytes[2],
                bytes[3],
                bytes[4].min(32),
            ]),
            16 => Self(vec![
                bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
                bytes[8], bytes[9], bytes[10], bytes[11], bytes[12], bytes[13], bytes[14],
                bytes[15], 128,
            ]),
            17 => Self(vec![
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
            _ => panic!("Invalid vector length"),
        }
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
        Self(vec![value[0], value[1], value[2], value[3], 32])
    }
}
impl From<[u8; 5]> for Ip {
    fn from(value: [u8; 5]) -> Self {
        Self(vec![
            value[0],
            value[1],
            value[2],
            value[3],
            value[4].min(32),
        ])
    }
}
impl From<[u8; 16]> for Ip {
    fn from(value: [u8; 16]) -> Self {
        Self(vec![
            value[0], value[1], value[2], value[3], value[4], value[5], value[6], value[7],
            value[8], value[9], value[10], value[11], value[12], value[13], value[14], value[15],
            128,
        ])
    }
}
impl From<[u8; 17]> for Ip {
    fn from(value: [u8; 17]) -> Self {
        Self(vec![
            value[0],
            value[1],
            value[2],
            value[3],
            value[4],
            value[5],
            value[6],
            value[7],
            value[8],
            value[9],
            value[10],
            value[11],
            value[12],
            value[13],
            value[14],
            value[15],
            value[16].min(128),
        ])
    }
}
impl From<Ip4> for Ip {
    fn from(value: Ip4) -> Self {
        let v = value.to_vec();
        Self(v)
    }
}
impl From<Ip6> for Ip {
    fn from(value: Ip6) -> Self {
        let v = value.to_vec();
        Self(v)
    }
}
impl TryFrom<&[u8]> for Ip {
    type Error = &'static str;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        match value.len() {
            4 => Ok(Self(vec![value[0], value[1], value[2], value[3], 32])),
            5 => Ok(Self(vec![
                value[0],
                value[1],
                value[2],
                value[3],
                value[4].min(32),
            ])),
            16 => Ok(Self(vec![
                value[0], value[1], value[2], value[3], value[4], value[5], value[6], value[7],
                value[8], value[9], value[10], value[11], value[12], value[13], value[14],
                value[15], 128,
            ])),
            17 => Ok(Self(vec![
                value[0],
                value[1],
                value[2],
                value[3],
                value[4],
                value[5],
                value[6],
                value[7],
                value[8],
                value[9],
                value[10],
                value[11],
                value[12],
                value[13],
                value[14],
                value[15],
                value[16].min(128),
            ])),
            _ => Err("The slice must contain exactly 4, 5, 16 or 17 elements"),
        }
    }
}
impl Family {
    pub fn is_valid_char(&self, c: char) -> bool {
        match self {
            Family::V4 => c.is_digit(10) || c == '.' || c == '/',
            Family::V6 => c.is_digit(10) || c.is_ascii_hexdigit() || c == ':' || c == '/',
            Family::ANY => {
                c.is_digit(10) || c.is_ascii_hexdigit() || c == ':' || c == '.' || c == '/'
            }
            Family::NONE => false,
        }
    }
    fn is_valid_str(&self, s: &str, family: Family) -> bool {
        match family {
            Family::V4 => {
                if !s.is_empty() {
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
            Family::V6 => {
                if !s.is_empty() {
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
                        } else if (c >= '0' && c <= '9')
                            || (c >= 'a' && c <= 'f')
                            || (c >= 'A' && c <= 'F')
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
            Family::ANY => self.is_valid_str(s, Family::V4) || self.is_valid_str(s, Family::V6),
            Family::NONE => false,
        }
    }
    pub fn from_str(s: &'static str) -> Result<Self, &'static str> {
        match s.to_lowercase().as_str() {
            "4" | "v4" => Ok(Family::V4),
            "6" | "v6" => Ok(Family::V6),
            "64" | "v64" | "v46" | "all" => Ok(Family::ANY),
            "" => Ok(Family::NONE),
            _ => Err("Invalid data"),
        }
    }
    pub fn to_str(&self) -> &'static str {
        match self {
            Family::V4 => "V4",
            Family::V6 => "V6",
            Family::ANY => "ALL",
            Family::NONE => "",
        }
    }
    pub fn from_suffix(s: &str) -> Family {
        if s.is_empty() {
            Family::NONE
        } else if s.ends_with("64") || s.ends_with("46") || s.ends_with("all") {
            Family::ANY
        } else if s.ends_with("6") {
            Family::V6
        } else if s.ends_with("4") {
            Family::V4
        } else {
            Family::NONE
        }
    }
}
impl Default for Family {
    fn default() -> Self {
        Family::V4
    }
}
impl FromStr for Family {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "4" | "v4" => Ok(Family::V4),
            "6" | "v6" => Ok(Family::V6),
            "64" | "v64" | "v46" | "all" => Ok(Family::ANY),
            "" => Ok(Family::NONE),
            _ => Err("Invalid data"),
        }
    }
}
impl Display for Family {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        write!(f, "{}", self.to_string())
    }
}
impl TryFrom<Family> for u8 {
    type Error = &'static str;
    fn try_from(value: Family) -> Result<Self, Self::Error> {
        match value {
            Family::V4 => Ok(4),
            Family::V6 => Ok(6),
            Family::ANY => Ok(64),
            Family::NONE => Ok(0),
        }
    }
}
impl TryFrom<u8> for Family {
    type Error = IoError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            4 => Ok(Family::V4),
            6 => Ok(Family::V6),
            64 => Ok(Family::ANY),
            0 => Ok(Family::NONE),
            _ => Err(IoError::new(IoErrorKind::InvalidData, "Invalid data")),
        }
    }
}
pub static PROTO_LIST: LazyLock<Vec<Proto>> = LazyLock::new(|| {
    Proto::import(&Path::new("/etc/protocols")).expect("Failed to import protocol information")
});
impl Proto {
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
            "Proto({}, \"{}, {}\")",
            self.number(),
            self.name(),
            self.description()
        );
    }
    pub fn from_str(s: &str) -> Result<Self, ParseIntError> {
        let mut parts = s.trim().split_whitespace();
        let number_str = parts.next().unwrap().trim();
        let proto_str = parts.next().unwrap_or("254").trim();
        let description = parts.next().unwrap_or("").trim();
        let number = u8::from_str(number_str)?;
        let proto = u8::from_str(proto_str)?;
        Ok(Proto(
            proto,
            format!("{}, \"{}, {}\"", number, proto_str, description).to_string(),
        ))
    }
    pub fn from_number(number: u8) -> Self {
        PROTO_LIST.iter().find(|p| p.0 == number).unwrap_or(&Proto(254, "".to_string())).clone()
    }
    pub fn import(path: &Path) -> Result<Vec<Proto>, std::io::Error> {
        let mut file = File::open(path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        let mut protos = Vec::<Proto>::new();
        for line in content.lines() {
            if !line.trim().is_empty() && !line.trim().starts_with('#') {
                let line = line.trim();
                let parts: Vec<&str> = line.split(' ').collect();
                for part in parts {
                    if !part.is_empty() {
                        let mut parts = part.split_whitespace();
                        if let (Some(proto_str), Some(number_str)) = (parts.next(), parts.next()) {
                            let description = line
                                .find('#')
                                .map(|i| line[i + 1..].trim().to_string())
                                .unwrap_or("".to_string());
                            if let Ok(proto_number) = u8::from_str(number_str) {
                                protos.push(Proto(
                                    proto_number,
                                    format!("{}, {}", proto_str, description).to_string(),
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

impl Display for Proto {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        write!(f, "{}", self.to_string())
    }
}
impl FromStr for Proto {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.trim().split_whitespace();
        let number_str = parts.next().unwrap().trim();
        let proto_str = parts.next().unwrap_or("254").trim();
        let description = parts.next().unwrap_or("").trim();
        let number = u16::from_str(number_str)?;
        let proto = u8::from_str(proto_str)?;
        Ok(Proto(
            proto,
            format!("{}, \"{},{}\"", number, proto_str, description).to_string(),
        ))
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
        Self(self.0, self.1, self.2.clone())
    }
    pub fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
    pub fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
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
        let number_str = parts.next().unwrap().trim();
        let proto_str = parts.next().unwrap_or("254").trim();
        let service = parts.next().unwrap_or("").trim().to_string();
        let number = u16::from_str(number_str)?;
        let proto = u8::from_str(proto_str)?;
        Ok(Port(number, proto, service))
    }
    pub fn from_number(number: u16) -> Self {
        PORT_LIST.iter().find(|p| p.0 == number).unwrap_or(&Port(0, 254, "".to_string())).clone()
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
