use std::{
    cmp::Ordering,
    fmt::{Display, Error as FmtError, Formatter},
    iter::IntoIterator,
    num::ParseIntError,
    ops::{
        Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div,
        DivAssign, Index, IndexMut, Mul, MulAssign, Not, Rem, RemAssign, Shl, ShlAssign, Shr,
        ShrAssign, Sub, SubAssign,
    },
    slice::{Iter, IterMut},
    str::{self, FromStr},
    vec::IntoIter,
};

use crate::{
    impl_abs_diff, impl_add, impl_bitand, impl_bitor, impl_bitxor, impl_default, impl_div,
    impl_from, impl_into_iter, impl_mul, impl_not, impl_octet_quad, impl_op, impl_op_assign,
    impl_ord, impl_range_bounds, impl_rem, impl_shl, impl_shr, impl_size_of, impl_sub, impl_vec,
};

const LOGGING_ENABLED: bool = true;
pub type Protocol = u8;
pub type Action = u8;
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Port(u16);
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Addr(u32);
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Subnet(u8);
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Net(u32, u8);
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Range<T>(T, T);
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct NetSet(Vec<Net>);
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Rule {
    nets: NetSet,
    ports: Vec<Port>,
    proto: Protocol,
    action: Action,
}

impl Port {
    pub fn new() -> Self {
        Self(0)
    }
    pub fn name() -> String {
        "Port".to_string()
    }
    pub fn to_string(&self) -> String {
        format!(":{}", self.0)
    }
    pub fn from_str(s: &str) -> Result<Self, ParseIntError> {
        if s.starts_with(":") {
            Ok(Self(u16::from_str(&s[1..])?))
        } else {
            Ok(Self(u16::from_str(s)?))
        }
    }
}
impl Display for Port {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(f, ":{}", self.0)
    }
}
impl FromStr for Port {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_str(s)
    }
}
impl_abs_diff!(Port);
impl_add!(Port, u16);
impl_sub!(Port, u16);
impl_mul!(Port, u16);
impl_div!(Port, u16);
impl_rem!(Port, u16);
impl_shl!(Port, u16);
impl_shr!(Port, u16);
impl_bitand!(Port, u16);
impl_bitor!(Port, u16);
impl_bitxor!(Port, u16);
impl_not!(Port);
impl_ord!(Port);
impl_from!(Port, u16);
impl_size_of!(Port);
impl_octet_quad!(Address);
impl Addr {
    pub fn new() -> Self {
        Self(0)
    }
    pub fn name() -> String {
        "Address".to_string()
    }
    pub fn to_string(&self) -> String {
        let bytes = self.to_be_bytes();
        format!("{}.{}.{}.{}", bytes[0], bytes[1], bytes[2], bytes[3])
    }
    pub fn from_str(s: &str) -> Result<Self, ParseIntError> {
        let mut addr: u32 = 0;
        for (i, ss) in s.split('.').enumerate() {
            addr |= u32::from_str(ss)? << (24 - i * 8);
        }
        Ok(Self(addr))
    }
    pub fn to_be_bytes(&self) -> [u8; 4] {
        [
            (self.0 >> 24) as u8,
            (self.0 >> 16) as u8,
            (self.0 >> 8) as u8,
            self.0 as u8,
        ]
    }
    pub fn from_be_bytes(bytes: [u8; 4]) -> Self {
        Self(
            (bytes[0] as u32) << 24
                | (bytes[1] as u32) << 16
                | (bytes[2] as u32) << 8
                | (bytes[3] as u32),
        )
    }
    pub fn to_vec(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }
    pub fn from_vec(bytes: Vec<u8>) -> Self {
        Self::from_be_bytes(bytes.try_into().unwrap())
    }
}
impl_range_bounds!(Addr, u32);
impl_default!(Addr);
impl_abs_diff!(Addr);
impl_add!(Addr, u32);
impl_sub!(Addr, u32);
impl_mul!(Addr, u32);
impl_div!(Addr, u32);
impl_rem!(Addr, u32);
impl_shl!(Addr, u32);
impl_shr!(Addr, u32);
impl_bitand!(Addr, u32);
impl_bitor!(Addr, u32);
impl_bitxor!(Addr, u32);
impl_not!(Addr);
impl_ord!(Addr);
impl_from!(Addr, u32);
impl_size_of!(Addr);
impl Display for Addr {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}.{}.{}.{}",
            (self.0 >> 24) as u8,
            (self.0 >> 16) as u8,
            (self.0 >> 8) as u8,
            self.0 as u8
        )
    }
}
impl From<[u8; 4]> for Addr {
    fn from(bytes: [u8; 4]) -> Self {
        Self::from_be_bytes(bytes)
    }
}
impl From<Net> for Addr {
    fn from(net: Net) -> Self {
        Self(net.0)
    }
}
impl Into<[u8; 4]> for Addr {
    fn into(self) -> [u8; 4] {
        self.to_be_bytes()
    }
}

impl PartialEq<u32> for Addr {
    fn eq(&self, addr: &u32) -> bool {
        self.0 == *addr
    }
}
impl PartialEq<Net> for Addr {
    fn eq(&self, net: &Net) -> bool {
        self.0 == net.0
    }
}
impl PartialEq<NetSet> for Addr {
    fn eq(&self, set: &NetSet) -> bool {
        set.contains(Net(self.0, 32))
    }
}
impl BitAndAssign<Net> for Addr {
    fn bitand_assign(&mut self, net: Net) {
        self.0 &= net.0;
    }
}

impl BitOrAssign<Net> for Addr {
    fn bitor_assign(&mut self, net: Net) {
        self.0 |= net.0;
    }
}

impl AddAssign<Net> for Addr {
    fn add_assign(&mut self, net: Net) {
        self.0 += net.0;
    }
}
impl SubAssign<Net> for Addr {
    fn sub_assign(&mut self, net: Net) {
        self.0 -= net.0;
    }
}

impl Subnet {
    pub fn new() -> Self {
        Self(32)
    }
    pub fn name() -> String {
        "Subnet".to_string()
    }
    pub fn to_string(&self) -> String {
        format!("/{}", self.0)
    }
    pub fn from_str(s: &str) -> Result<Self, ParseIntError> {
        if s.starts_with('/') {
            Ok(Self(u8::from_str(s.split('/').last().unwrap())?))
        } else {
            Ok(Self(u8::from_str(s)?))
        }
    }
}
impl Display for Subnet {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "/{}", self.0)
    }
}
impl FromStr for Subnet {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_str(s)
    }
}
impl_default!(Subnet);
impl_abs_diff!(Subnet);
impl_add!(Subnet, u8);
impl_sub!(Subnet, u8);
impl_mul!(Subnet, u8);
impl_div!(Subnet, u8);
impl_rem!(Subnet, u8);
impl_shl!(Subnet, u8);
impl_shr!(Subnet, u8);
impl_bitand!(Subnet, u8);
impl_bitor!(Subnet, u8);
impl_bitxor!(Subnet, u8);
impl_not!(Subnet);
impl_ord!(Subnet);
impl_from!(Subnet, u8);
impl_size_of!(Subnet);

type Mask = Addr;

impl From<Subnet> for Mask {
    fn from(value: Subnet) -> Self {
        Addr(1 << (32 - value.0))
    }
}
impl From<Mask> for Subnet {
    fn from(value: Mask) -> Self {
        Subnet(32 - value.0.leading_zeros() as u8)
    }
}

impl Net {
    pub fn new() -> Self {
        Self(0, 32)
    }
    pub fn name() -> String {
        "Network".to_string()
    }
    pub fn address(&self) -> u32 {
        self.0
    }
    pub fn subnet(&self) -> u8 {
        self.1
    }
    pub fn set_address(&mut self, addr: u32) {
        self.0 = addr;
    }
    pub fn set_subnet(&mut self, subnet: u8) {
        self.1 = subnet;
    }
    pub fn netmask(&self) -> u32 {
        !(0xFFFFFFFF << (32 - self.1))
    }
    pub fn wildcard(&self) -> u32 {
        0xFFFFFFFF << (32 - self.1)
    }
    pub fn network(&self) -> u32 {
        self.0 & self.wildcard()
    }
    pub fn broadcast(&self) -> u32 {
        self.0 | self.netmask()
    }
    pub fn contains(&self, other: &Net) -> bool {
        self.network() <= other.network() && self.broadcast() >= other.broadcast()
    }
    pub fn contained(&self, other: &Net) -> bool {
        self.network() >= other.network() && self.broadcast() <= other.broadcast()
    }
    pub fn to_vec(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }
    pub fn from_vec(bytes: Vec<u8>) -> Self {
        if bytes.len() != 5 {
            panic!("The vector must contain exactly 5 bytes");
        }
        Self(
            (bytes[0] as u32) << 24
                | (bytes[1] as u32) << 16
                | (bytes[2] as u32) << 8
                | (bytes[3] as u32),
            bytes[4].min(32),
        )
    }
    pub const fn size_of() -> usize {
        5
    }
    pub fn abs_diff(&self, other: &Self) -> Self {
        Self(self.0.abs_diff(other.0), self.1.abs_diff(other.1))
    }
    pub fn to_string(&self) -> String {
        let bytes = self.to_be_bytes();
        format!(
            "{}.{}.{}.{}/{}",
            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4],
        )
    }
    pub fn to_be_bytes(&self) -> [u8; 5] {
        [
            (self.0 >> 24) as u8,
            (self.0 >> 16) as u8,
            (self.0 >> 8) as u8,
            self.0 as u8,
            self.1,
        ]
    }
    pub fn from_be_bytes(bytes: [u8; 5]) -> Self {
        Self(
            (bytes[0] as u32) << 24
                | (bytes[1] as u32) << 16
                | (bytes[2] as u32) << 8
                | (bytes[3] as u32),
            bytes[4].min(32),
        )
    }
    pub fn from_str(s: &str) -> Result<Self, ParseIntError> {
        let mut addr: u32 = 0;
        let mut subnet: u8 = 32;

        let (ip_part, subnet_part) = if s.contains('/') {
            let parts: Vec<&str> = s.split('/').collect();
            (parts[0], Some(parts[1]))
        } else {
            (s, None)
        };

        for (idx, octet) in ip_part.split('.').enumerate() {
            addr |= u32::from_str(octet)? << (24 - idx * 8);
        }

        if let Some(subnet_str) = subnet_part {
            subnet = subnet_str.parse::<u8>()?.min(32);
        }

        Ok(Self(addr, subnet))
    }
    pub fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0).then(self.1.cmp(&other.1))
    }
}
impl Display for Net {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        let bytes = self.to_be_bytes();
        write!(
            f,
            "{}.{}.{}.{}/{}",
            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4]
        )
    }
}
impl Ord for Net {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0).then(self.1.cmp(&other.1))
    }
}
impl PartialOrd for Net {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl TryFrom<&[u8]> for Net {
    type Error = &'static str;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() == 4 {
            Ok(Self(u32::from_be_bytes(value.try_into().unwrap()), 32))
        } else if value.len() == 5 {
            Ok(Self(
                u32::from_be_bytes(value[0..4].try_into().unwrap()),
                value[4].min(32),
            ))
        } else {
            Err("The vector must contain exactly 4 or 5 bytes")
        }
    }
}
impl_op!(Net, u32, u8, Add, add, +, 3);
impl_op!(Net, u32, u8, Sub, sub, -, 3);
impl_op!(Net, u32, u8, BitAnd, bitand, &, 3);
impl_op!(Net, u32, u8, BitOr, bitor, |, 3);
impl_op!(Net, u32, u8, BitXor, bitxor, ^, 3);
impl_op!(Net, u32, u8, Shr, shr, >>, 3);
impl_op_assign!(Net, u32, u8, AddAssign, add_assign, +=, 3);
impl_op_assign!(Net, u32, u8, SubAssign, sub_assign, -=, 3);
impl_op_assign!(Net, u32, u8, BitAndAssign, bitand_assign, &=, 3);
impl_op_assign!(Net, u32, u8, BitOrAssign, bitor_assign, |=, 3);
impl_op_assign!(Net, u32, u8, BitXorAssign, bitxor_assign, ^=, 3);
impl_op_assign!(Net, u32, u8, ShrAssign, shr_assign, >>, 3);

impl From<Addr> for Net {
    fn from(addr: Addr) -> Self {
        Self(addr.0, 32)
    }
}
impl From<u32> for Net {
    fn from(addr: u32) -> Self {
        Self(addr, 32)
    }
}
impl FromStr for Net {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_str(s)
    }
}
impl PartialEq<NetSet> for Net {
    fn eq(&self, set: &NetSet) -> bool {
        set.contains(*self)
    }
}
impl BitAnd<Addr> for Net {
    type Output = Self;
    fn bitand(self, addr: Addr) -> Self::Output {
        Self(self.0 & addr.0, self.1)
    }
}
impl BitAndAssign<Addr> for Net {
    fn bitand_assign(&mut self, addr: Addr) {
        self.0 &= addr;
    }
}
impl BitOr<Addr> for Net {
    type Output = Self;
    fn bitor(self, addr: Addr) -> Self::Output {
        Self(self.0 | addr.0, self.1)
    }
}

impl BitOrAssign<Addr> for Net {
    fn bitor_assign(&mut self, addr: Addr) {
        self.0 |= addr.0;
    }
}
impl BitXor<Addr> for Net {
    type Output = Self;
    fn bitxor(self, addr: Addr) -> Self::Output {
        Self(self.0 ^ addr.0, self.1)
    }
}
impl Add<Addr> for Net {
    type Output = Self;
    fn add(self, addr: Addr) -> Self::Output {
        Self(self.0 + addr.0, self.1)
    }
}
impl AddAssign<Addr> for Net {
    fn add_assign(&mut self, addr: Addr) {
        self.0 += addr.0;
    }
}
impl Sub<Addr> for Net {
    type Output = Self;
    fn sub(self, addr: Addr) -> Self::Output {
        Self(self.0 - addr.0, self.1)
    }
}
impl SubAssign<Addr> for Net {
    fn sub_assign(&mut self, addr: Addr) {
        self.0 -= addr.0;
    }
}
impl NetSet {
    pub fn new() -> Self {
        Self(Vec::new())
    }
    pub fn optimize(&mut self) -> Self {
        let old_len = self.len();
        if old_len == 0 {
            return self.clone();
        }
        if LOGGING_ENABLED {
            println!("Current Total: {} entries", old_len);
        }
        let mut i = 0;
        let mut changed = false;
        while i < self.len() - 1 {
            let net1 = Net(self[i].network(), self[i].subnet());
            let net2 = Net(self[i + 1].network(), self[i + 1].subnet());
            if net1 == net2 {
                self.swap_remove(i + 1);
                changed = true;
            } else if net1 > net2 {
                self.swap(i, i + 1);
                changed = true;
            } else if net1.contains(&net2) {
                self.swap_remove(i + 1);
                changed = true;
            } else if net1.broadcast().abs_diff(net2.network()) == 1
                && net1.subnet() == net2.subnet()
            {
                self[i] = Net(net1.address(), net1.subnet() - 1);
                self.swap_remove(i + 1);
                changed = true;
            } else {
                i += 1;
            }
            if changed && i > 0 {
                i -= 1; // Check previous entry again as it might now merge with the next
            }
            changed = false;
        }
        self.shrink_to_fit();

        if LOGGING_ENABLED {
            println!(
                "New Total: {} entries\nDifference: {} entries",
                self.len(),
                old_len - self.len(),
            );
        }

        self.clone()
    }
    pub fn to_string(&self) -> String {
        let mut str = String::new();
        let mut tmp = self.iter().into_iter();
        while let Some(net) = tmp.next() {
            str.push_str(&net.to_string());
            str.push_str("\n");
        }
        str
    }
    pub fn from_str(str: &str) -> Result<Self, ParseIntError> {
        let mut netset = Self::new();
        let mut prev_segment = "";
        for line in str.lines() {
            let mut start = 0;
            for (i, c) in line.char_indices() {
                if !Self::is_valid_char(c) {
                    let segment = &line[start..i];
                    if prev_segment != segment {
                        if Self::is_valid_str(&segment) {
                            match Net::from_str(&segment) {
                                Ok(net) => netset.push(net),
                                Err(e) => return Err(e),
                            }
                        }
                        prev_segment = segment;
                    }
                    start = i + 1;
                }
            }

            // Check the last segment
            if Self::is_valid_str(&line[start..]) && prev_segment != &line[start..] {
                match Net::from_str(&line[start..]) {
                    Ok(net) => netset.push(net),
                    Err(e) => return Err(e),
                }
            }
        }

        Ok(netset)
    }
    fn is_valid_char(c: char) -> bool {
        match c {
            '0'..='9' | '.' | '/' => true,
            _ => false,
        }
    }
    fn is_valid_str(s: &str) -> bool {
        let mut dot_count = 0;
        let mut slash_count = 0;
        let mut num_count = 0;
        if s.is_empty() || s.len() < 7 || s.len() > 18 {
            return false;
        }
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
            } else if c < '0' || c > '9' {
                return false;
            } else {
                num_count += 1;
                if num_count > 14 {
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
impl_vec!(NetSet, Net);
impl_into_iter!(NetSet, Net);
impl_size_of!(NetSet);
impl FromStr for NetSet {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_str(s)
    }
}

impl Display for NetSet {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        write!(f, "{}", self.to_string())
    }
}
impl std::io::Read for NetSet {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.to_string().as_bytes().read(buf)
    }
}
impl std::io::Write for NetSet {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.push(Net::from_str(std::str::from_utf8(buf).unwrap()).unwrap());
        Ok(buf.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
impl Index<usize> for NetSet {
    type Output = Net;
    fn index(&self, idx: usize) -> &Self::Output {
        self.0.index(idx)
    }
}
impl IndexMut<usize> for NetSet {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        self.0.index_mut(idx)
    }
}
impl From<Vec<Net>> for NetSet {
    fn from(vec: Vec<Net>) -> Self {
        Self(vec)
    }
}
impl From<&[Net]> for NetSet {
    fn from(slice: &[Net]) -> Self {
        Self(slice.to_vec())
    }
}
impl Rule {
    pub fn new() -> Self {
        Self {
            nets: NetSet::new(),
            ports: Vec::new(),
            proto: 0,
            action: 0,
        }
    }
}
impl From<Addr> for String {
    fn from(addr: Addr) -> Self {
        addr.to_string()
    }
}

impl From<Net> for String {
    fn from(net: Net) -> Self {
        net.to_string()
    }
}
impl From<NetSet> for String {
    fn from(set: NetSet) -> Self {
        set.to_string()
    }
}
impl From<Subnet> for u32 {
    fn from(subnet: Subnet) -> Self {
        subnet.0 as u32
    }
}
impl Shl<Net> for u32 {
    type Output = u32;
    fn shl(self, net: Net) -> Self::Output {
        self << net.address()
    }
}
impl ShlAssign<Net> for u32 {
    fn shl_assign(&mut self, net: Net) {
        *self <<= net.address();
    }
}
