pub mod v4;
pub mod v6;
use std::{
    fmt::{Display, Error as FmtError, Formatter}, io::{Error as IoError, ErrorKind as IoErrorKind}, num::ParseIntError, str::FromStr
};
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Parser {
    family: Family,
    addr: Vec<u8>,
    subnet: u8,
}
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Ip4(u32, u8);
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Ip6(u128, u8);

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Ip(Vec<u8>, u8);
impl Ip4 {
    const FAMILY: Family = Family::V4;
    pub fn new() -> Self {
        Self(0, 0)
    }
    pub fn name() -> String {
        "Ip4".to_string()
    }
    pub fn from_str(s: &str) -> Result<Self, ParseIntError> {
        let mut addr: u32 = 0;
        let mut subnet: u8 = 32;
        if s.contains('/') {
            let parts: Vec<&str> = s.split('/').collect();
            if parts.len() != 2 {
                panic!("Invalid format");
            }
            else if parts[1].len() > 2 {
                panic!("Invalid subnet");
            } else if parts[1].len() == 2 {
                subnet = u8::from_str(parts[1])?;
            }
            let parts: Vec<&str> = parts[0].split('.').collect();
            if parts.len() != 4 {
                panic!("Invalid format");
            }
            for (i, ss) in parts.iter().enumerate() {
                addr |= u32::from_str(ss)? << (24 - i * 8);
            }
        } else {
            let parts: Vec<&str> = s.split('.').collect();
            if parts.len() != 4 {
                panic!("Invalid format");
            }
            for (i, ss) in parts.iter().enumerate() {
                addr |= u32::from_str(ss)? << (24 - i * 8);
            }
        }
        Ok(Self(addr, subnet))
    }
    pub fn to_string(&self) -> String {
        format!("{}.{}.{}.{}/{}", (self.0 >> 24) as u8, (self.0 >> 16) as u8, (self.0 >> 8) as u8, self.0 as u8, self.1)
    }
    pub fn from_vec(bytes: Vec<u8>) -> Self {
        if bytes.len() != 4 {
            panic!("The vector must contain exactly 4 bytes");
        }
        Self(
            (bytes[0] as u32) << 24
                | (bytes[1] as u32) << 16
                | (bytes[2] as u32) << 8
                | (bytes[3] as u32),
            0,
        )
    }
    pub fn to_vec(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }
    pub fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        write!(f, "{}.{}.{}.{}/{}", (self.0 >> 24) as u8, (self.0 >> 16) as u8, (self.0 >> 8) as u8, self.0 as u8, self.1)
    }
    pub const fn size_of() -> usize {
        4
    }
    pub fn abs_diff(&self, other: &Self) -> Self {
        Self(self.0.abs_diff(other.0), self.1.abs_diff(other.1))
    }
    pub fn to_be_bytes(&self) -> [u8; 5] {
        let bytes = self.0.to_be_bytes();
        [bytes[0], bytes[1], bytes[2], bytes[3], self.1]
    }
    pub fn from_be_bytes(bytes: [u8; 5]) -> Self {
        Self(
            (bytes[0] as u32) << 24
                | (bytes[1] as u32) << 16
                | (bytes[2] as u32) << 8
                | (bytes[3] as u32),
            bytes[4],
        )
    }
}

impl Display for Ip4 {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.fmt(f)
    }
}
impl FromStr for Ip4 {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_str(s)
    }
}

impl Ip6 {
    const FAMILY: Family = Family::V6;
    pub fn new() -> Self {
        Self(0, 0)
    }
    pub fn name() -> String {
        "Ip6".to_string()
    }
    pub fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        write!(f, "{:04x}{:04x}:{:04x}{:04x}:{:04x}{:04x}:{:04x}{:04x}/{}", (self.0 >> 112) as u16, (self.0 >> 96) as u16, (self.0 >> 80) as u16, (self.0 >> 64) as u16, (self.0 >> 48) as u16, (self.0 >> 32) as u16, (self.0 >> 16) as u16, self.0 as u16,self.1)
    }
    pub fn to_string(&self) -> String {
        format!("{:04x}{:04x}:{:04x}{:04x}:{:04x}{:04x}:{:04x}{:04x}/{}", (self.0 >> 112) as u16, (self.0 >> 96) as u16, (self.0 >> 80) as u16, (self.0 >> 64) as u16, (self.0 >> 48) as u16, (self.0 >> 32) as u16, (self.0 >> 16) as u16, self.0 as u16,self.1)
    }
    pub fn from_str(s: &str) -> Result<Self, ParseIntError> {
        if s.is_empty() {
            panic!("Empty string");
        }
        let mut addr: u128 = 0;
        let mut subnet: u8 = 128;
        if s.contains('/') {
            let parts: Vec<&str> = s.split('/').collect();
            if parts.len() > 2 {
                panic!("Invalid format");
            }
            for (i, ss) in parts[0].split(':').enumerate() {
                addr |= u128::from_str(ss)? << (112 - i * 16);
            }
            subnet = u8::from_str(parts[1])?;
            Ok(Self(addr, subnet))
        } else {
            let parts: Vec<&str> = s.split(':').collect();
            if parts.len() > 8 {
                panic!("Too many segments in IP address string");
            }
            for (i, ss) in parts.iter().enumerate() {
                addr |= u128::from_str(ss)? << (112 - i * 16);
            }
            Ok(Self(addr, subnet))
        }
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

impl Ip {
    const FAMILY: Family = Family::ALL;
    pub fn new() -> Self {
        Self(vec![], 0)
    }
    pub fn name() -> String {
        "Ip".to_string()
    }
    pub fn to_string(&self) -> String {
        if self.0.len() == 4 {
            let parser = Parser { family: Ip4::FAMILY, addr: self.0.clone(), subnet: self.1 }; 
            return parser.to_string();
        }
        else if self.0.len() == 16 {
            let parser = Parser { family: Ip6::FAMILY, addr: self.0.clone(), subnet: self.1 }; 
            return parser.to_string();
        }
        panic!("Invalid IP format");
    }
    pub fn from_str(s: &str) -> Result<Self, ParseIntError> {
        let mut parser = Parser::new();
        parser.parse(s, Family::ALL)?;
        Ok(Self(parser.addr, parser.subnet))
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
impl Parser {
    pub fn new() -> Self {
        Self {
            family: Family::NONE,
            addr: vec![],
            subnet: 0,
        }
    }
    pub fn set(&mut self, family: Family, addr: Vec<u8>, subnet: u8) {
        self.family = family;
        self.addr = addr;
        self.subnet = subnet;
    }
    pub fn set_v4(&mut self, addr: Vec<u8>, subnet: u8) {
        self.set(Family::V4, addr, subnet)
    }
    pub fn set_v6(&mut self, addr: Vec<u8>, subnet: u8) {
        self.set(Family::V6, addr, subnet)
    }
    pub fn set_all(&mut self, addr: Vec<u8>, subnet: u8) {
        self.set(Family::ALL, addr, subnet)
    }
    pub fn is_v4(&self) -> bool {
        matches!(self.family, Family::V4)
    }
    pub fn is_v6(&self) -> bool {
        matches!(self.family, Family::V6)
    }
    pub fn is_all(&self) -> bool {
        matches!(self.family, Family::ALL)
    }
    pub fn address(&self) -> Vec<u8> {
        self.addr.clone()
    }
    pub fn subnet(&self) -> u8 {
        self.subnet
    }
    pub fn family(&self) -> Family {
        self.family
    }
    pub fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        write!(f, "{}", self.to_string())
    }
    pub fn to_string(&self) -> String {
        if !self.addr.is_empty() {
            match self.family {
                Family::V4 => {
                    if self.addr.len() != 4 {
                        println!(
                            "Invalid address length: {} for family: {}",
                            self.addr.len(),
                            self.family
                        );
                        return String::new();
                    } else {
                        return format!(
                            "{}.{}.{}.{}/{}",
                            self.addr[0], self.addr[1], self.addr[2], self.addr[3], self.subnet
                        );
                    }
                }
                Family::V6 => {
                    if self.addr.len() != 16 {
                        println!(
                            "Invalid address length: {} for family: {}",
                            self.addr.len(),
                            self.family
                        );
                        return String::new();
                    } else {
                        return format!("{:2x}{:2x}:{:2x}{:2x}:{:2x}{:2x}:{:2x}{:2x}:{:2x}{:2x}:{:2x}{:2x}:{:2x}{:2x}:{:2x}{:2x}/{}", self.addr[0], self.addr[1], self.addr[2], self.addr[3], self.addr[4], self.addr[5], self.addr[6], self.addr[7], self.addr[8], self.addr[9], self.addr[10], self.addr[11], self.addr[12], self.addr[13], self.addr[14], self.addr[15], self.subnet);
                    }
                }
                Family::ALL => {
                    if self.addr.len() != 4 && self.addr.len() != 16 {
                        println!(
                            "Invalid address length: {} for family: {}",
                            self.addr.len(),
                            self.family
                        );
                        return String::new();
                    } else if self.addr.len() == 4 {
                        return format!(
                            "{}.{}.{}.{}/{}",
                            self.addr[0], self.addr[1], self.addr[2], self.addr[3], self.subnet
                        );
                    } else if self.addr.len() == 16 {
                        return format!("{:2x}{:2x}:{:2x}{:2x}:{:2x}{:2x}:{:2x}{:2x}:{:2x}{:2x}:{:2x}{:2x}:{:2x}{:2x}:{:2x}{:2x}/{}", self.addr[0], self.addr[1], self.addr[2], self.addr[3], self.addr[4], self.addr[5], self.addr[6], self.addr[7], self.addr[8], self.addr[9], self.addr[10], self.addr[11], self.addr[12], self.addr[13], self.addr[14], self.addr[15], self.subnet);
                    }
                }
                Family::NONE => println!("Family is set to NONE, returning empty string"),
            }
        }
        String::new()
    }
    fn parse_v4(&mut self, s: &str) -> Result<Self, ParseIntError> {
        let mut addr: [u8; 4] = [0; 4];
        let mut subnet: u8 = 32;

        let (ip_part, subnet_part) = if s.contains('/') {
            let parts: Vec<&str> = s.split('/').collect();
            (parts[0], Some(parts[1]))
        } else {
            (s, None)
        };

        for (idx, octet) in ip_part.split('.').enumerate() {
            if octet.is_empty() {
                panic!("Empty octet");
            }
            addr[idx] = octet.parse::<u8>()?;
        }
        if let Some(subnet_str) = subnet_part {
            subnet = subnet_str.parse::<u8>()?;
        }
        self.set_v4(addr.to_vec(), subnet);
        Ok(self.clone())
    }

    fn parse_v6(&mut self, s: &str) -> Result<Self, ParseIntError> {
        let mut addr: [u8; 16] = [0; 16];
        let mut subnet: u8 = 128;

        let (ip_part, subnet_part) = if s.contains('/') {
            let parts: Vec<&str> = s.split('/').collect();
            (parts[0], Some(parts[1]))
        } else {
            (s, None)
        };

        for (idx, octet) in ip_part.split(':').enumerate() {
            let mut octet = octet.trim_start_matches('0');
            if octet.is_empty() {
                octet = "0";
            }
            addr[idx] = u8::from_str(octet)?;
        }
        if let Some(subnet_str) = subnet_part {
            subnet = subnet_str.parse::<u8>()?;
        }
        self.set_v6(addr.to_vec(), subnet);
        Ok(self.clone())
    }
    pub fn parse(&mut self, s: &str, family: Family) -> Result<Self, ParseIntError> {
        if s.is_empty() {
            panic!("Empty string");
        }
        if family == Family::V4 {
            return self.parse_v4(s);
        }
        if family == Family::V6 {
            return self.parse_v6(s);
        }
        panic!("Invalid family");
    }
    pub fn from_str(&mut self, s: &str, family: Family) -> Result<Self, ParseIntError> {
        return self.parse(s, family);
    }
    pub fn to_ip4(&self) -> Ip4 {
        if self.family != Family::V4 {
            panic!("Family is not V4");
        }
        let mut addr: u32 = 0;
        for (i, octet) in self.addr.iter().enumerate() {
            addr |= u32::from(*octet) << (24 - i * 8);
        }
        Ip4(addr, self.subnet)
    }
    pub fn to_ip6(&self) -> Ip6 {
        if self.family != Family::V6 {
            panic!("Family is not V6");
        }
        let mut addr: u128 = 0;
        for (i, octet) in self.addr.iter().enumerate() {
            addr |= u128::from(*octet) << (120 - i * 8);
        }
        Ip6(addr, self.subnet)
    }
}

impl Display for Parser {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        write!(f, "{}", self.to_string())
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Family {
    NONE,
    V4,
    V6,
    ALL,
}
impl Family {
    pub fn is_valid_char(c: char, family: Family) -> bool {
        match family {
            Family::V4 => c.is_digit(10) || c == '.' || c == '/',
            Family::V6 => c.is_digit(10) || c.is_ascii_hexdigit() || c == ':' || c == '/',
            Family::ALL => {
                c.is_digit(10) || c.is_ascii_hexdigit() || c == ':' || c == '.' || c == '/'
            }
            Family::NONE => false,
        }
    }
    pub fn from_str(s: &str) -> Result<Self, &'static str> {
        match s {
            "4" | "v4" | "V4" => Ok(Family::V4),
            "6" | "v6" | "V6" => Ok(Family::V6),
            "64" | "v64" | "V64" | "V46" | "v46" | "ALL" | "all" => Ok(Family::ALL),
            "" => Ok(Family::NONE),
            _ => Err("Invalid data"),
        }
    }
    pub fn to_str(family: Family) -> &'static str {
        match family {
            Family::V4 => "V4",
            Family::V6 => "V6",
            Family::ALL => "ALL",
            Family::NONE => "",
        }
    }
    pub fn ends_with_family(s: &str) -> Family {
        if s.ends_with("64") || s.ends_with("46") || s.ends_with("ALL") || s.ends_with("all") {
            Family::ALL
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
        Family::NONE
    }
}
impl FromStr for Family {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Family::from_str(s)
    }
}
impl Display for Family {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        write!(f, "{}", Family::to_string(self))
    }
}
impl TryFrom<Family> for u8 {
    type Error = &'static str;
    fn try_from(value: Family) -> Result<Self, Self::Error> {
        match value {
            Family::V4 => Ok(4),
            Family::V6 => Ok(6),
            Family::ALL => Ok(64),
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
            64 => Ok(Family::ALL),
            0 => Ok(Family::NONE),
            _ => Err(IoError::new(IoErrorKind::InvalidData, "Invalid data")),
        }
    }
}
