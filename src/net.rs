pub mod v4;
pub mod v6;
use std::{
    cmp::Ordering, fmt::{Display, Formatter, Result, Debug}, fs::File, hash::Hash, io::Read, num::ParseIntError, ops::{Bound, Index, IndexMut, RangeBounds}, path::Path, str::FromStr, sync::LazyLock
};

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Family {
    V4 = 4,
    V6 = 6,
    NONE = 255,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Ip {
    V4([u8; 5]),
    V6([u8; 17]),
    NONE,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Action {
    ACCEPT,
    DROP,
    REJECT,
    SKIP,
    LIMIT(u32),
    RETURN,
    JUMP(u32),
    LOG(String),
    MARK(u32),
    META(String),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Chain(Vec<Rule>, u32, String);

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Protocol(u8, String);

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Port(u16, u8, String);

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Mask {
    V4([u8; 4]),
    V6([u8; 16]),
    NONE,
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Rule(
    Set<Ip, 65535>,
    Set<Ip, 65535>,   
    Set<Protocol, 255>,
    Set<Port, 65535>,
    Set<Port, 65535>,
    Action,
    String,
);
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Set<T: Clone + Eq + Hash + Ord + Default + FromStr + Display + Debug, const N: usize>
(
    [T; N],
    usize,
    String,
    u32,
);

impl Mask {
    pub fn new(family: Family) -> Self {
        match family {
            Family::V4 => Self::V4([0; 4]),
            Family::V6 => Self::V6([0; 16]),
            Family::NONE => Self::NONE,
        }
    }

    pub fn clone(&self) -> Self {
        match self {
            Mask::V4(mask) => Mask::V4(*mask),
            Mask::V6(mask) => Mask::V6(*mask),
            Mask::NONE => Mask::NONE,
        }
    }

    pub fn len(&self) -> usize {
        match self {
            Mask::V4(_) => 4,
            Mask::V6(_) => 16,
            Mask::NONE => 0,
        }
    }

    pub fn family(&self) -> Family {
        match self {
            Mask::V4(_) => Family::V4,
            Mask::V6(_) => Family::V6,
            Mask::NONE => Family::NONE,
        }
    }

    pub fn to_vec(&self) -> Vec<u8> {
        match self {
            Mask::V4(a) => a.to_vec(),
            Mask::V6(a) => a.to_vec(),
            Mask::NONE => Vec::new(),
        }
    }

    pub fn from_vec(family: Family, v: Vec<u8>) -> Self {
        match family {
            Family::V4 => {
                if v.len() == 4 {
                    let mut arr = [0u8; 4];
                    arr.copy_from_slice(&v);
                    Mask::V4(arr)
                } else {
                    Mask::NONE
                }
            }
            Family::V6 => {
                if v.len() == 16 {
                    let mut arr = [0u8; 16];
                    arr.copy_from_slice(&v);
                    Mask::V6(arr)
                } else {
                    Mask::NONE
                }
            }
            Family::NONE => Mask::NONE,
        }
    }

    /// Convert mask to its short prefix length form, e.g. [255,255,255,0] -> 24.
    pub fn to_pfxlen(&self) -> Option<u8> {
        let bits = match self {
            Mask::V4(a) => a.iter().flat_map(|b| (0..8).rev().map(move |i| (b >> i) & 1)).collect::<Vec<_>>(),
            Mask::V6(a) => a.iter().flat_map(|b| (0..8).rev().map(move |i| (b >> i) & 1)).collect::<Vec<_>>(),
            Mask::NONE => return None,
        };
        let mut prefixlen = 0u8;
        for (i, bit) in bits.iter().enumerate() {
            if *bit == 1 {
                prefixlen += 1;
            } else {
                if bits.iter().skip(i).any(|&b| b == 1) {
                    return None; // not a valid contiguous mask
                }
                break;
            }
        }
        Some(prefixlen)
    }

    /// Construct a mask from a prefixlen. E.g. (Family::V4, 24) => [255,255,255,0]
    pub fn from_pfxlen(family: Family, prefixlen: u8) -> Self {
        match family {
            Family::V4 => {
                if prefixlen > 32 {
                    return Mask::NONE;
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
                    return Mask::NONE;
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
            Family::NONE => Mask::NONE,
        }
    }

    pub fn from_str(s: &str) -> Self {
        let s = s.trim();
        if s.is_empty() {
            return Mask::NONE;
        }
        if s.contains('.') && !s.contains(':') {
            let mut octets = [0u8; 4];
            let splits: Vec<&str> = s.split('.').collect();
            if splits.len() != 4 {
                return Mask::NONE;
            }
            for (i, ss) in splits.into_iter().enumerate() {
                if ss.is_empty() {
                    return Mask::NONE;
                }
                match u8::from_str(ss) {
                    Ok(val) => octets[i] = val,
                    Err(_) => return Mask::NONE,
                }
            }
            Mask::V4(octets)
        } else if s.contains(':') && !s.contains('.') {
            let splits: Vec<&str> = s.split(':').collect();
            if splits.len() != 16 && splits.len() != 8 {
                return Mask::NONE;
            }
            let mut octets = [0u8; 16];
            for (i, ss) in splits.into_iter().enumerate() {
                if ss.is_empty() {
                    octets[i] = 0;
                } else {
                    match u8::from_str(ss) {
                        Ok(val) => octets[i] = val,
                        Err(_) => return Mask::NONE,
                    }
                }
            }
            Mask::V6(octets)
        } else {
            Mask::NONE
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Mask::V4(arr) => arr.iter().map(|b| b.to_string()).collect::<Vec<_>>().join("."),
            Mask::V6(arr) => {
                arr.chunks(2)
                    .map(|chunk| chunk.iter().map(|b| format!("{:02X}", b)).collect::<String>())
                    .collect::<Vec<_>>()
                    .join(":")
            },
            Mask::NONE => "".to_string(),
        }
    }

    pub fn is_valid_str(s: &str, family: Family) -> bool {
        match family {
            Family::V4 => Self::is_valid_ip4_str(s),
            Family::V6 => Self::is_valid_ip6_str(s),
            Family::NONE => false,
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

    pub fn is_ip4(&self) -> bool {
        matches!(self, Mask::V4(_))
    }

    pub fn is_ip6(&self) -> bool {
        matches!(self, Mask::V6(_))
    }

    pub fn is_none(&self) -> bool {
        matches!(self, Mask::NONE)
    }
}
impl Default for Mask {
    fn default() -> Self {
        Mask::NONE
    }
}
impl Display for Mask {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.to_string())
    }
}

impl PartialOrd for Mask {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Mask {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Mask::V4(a), Mask::V4(b)) => a.cmp(b),
            (Mask::V4(a), Mask::V6(b)) => a.to_vec().cmp(&b.to_vec()),
            (Mask::V6(a), Mask::V4(b)) => a.to_vec().cmp(&b.to_vec()),
            (Mask::V6(a), Mask::V6(b)) => a.cmp(b),
            (Mask::NONE, Mask::NONE) => Ordering::Equal,
            (Mask::NONE, _) => Ordering::Less,
            (_, Mask::NONE) => Ordering::Greater,
        }
    }
}

impl Hash for Mask {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            Mask::V4(addr) => addr.hash(state),
            Mask::V6(addr) => addr.hash(state),
            Mask::NONE => 0.hash(state),
        }
    }
}

impl Index<usize> for Mask {
    type Output = u8;
    fn index(&self, index: usize) -> &Self::Output {
        match self {
            Mask::V4(mask) => &mask[index],
            Mask::V6(mask) => &mask[index],
            Mask::NONE => panic!("Mask::NONE has no elements"),
        }
    }
}

impl IndexMut<usize> for Mask {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match self {
            Mask::V4(mask) => &mut mask[index],
            Mask::V6(mask) => &mut mask[index],
            Mask::NONE => panic!("Mask::NONE has no elements"),
        }
    }
}
impl Ip {
    pub fn new(family: Family) -> Self {
        match family {
            Family::V4 => Ip::V4([0; 5]),
            Family::V6 => Ip::V6([0; 17]),
            Family::NONE => Ip::NONE,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Ip::V4(_) => "IP4",
            Ip::V6(_) => "IP6",
            Ip::NONE => "",
        }
    }

    pub fn family(&self) -> Family {
        match self {
            Ip::V4(_) => Family::V4,
            Ip::V6(_) => Family::V6,
            Ip::NONE => Family::NONE,
        }
    }

    pub fn len(&self) -> usize {
        match self {
            Ip::V4(_) => 5,
            Ip::V6(_) => 17,
            Ip::NONE => 0,
        }
    }

    pub fn address(&self) -> Vec<u8> {
        match self {
            Ip::V4(ip) => ip[..4].to_vec(),
            Ip::V6(ip) => ip[..16].to_vec(),
            Ip::NONE => vec![],
        }
    }

    pub fn subnet(&self) -> u8 {
        match self {
            Ip::V4(ip) => ip[4],
            Ip::V6(ip) => ip[16],
            Ip::NONE => 0,
        }
    }

    pub fn to_vec(&self) -> Vec<u8> {
        match self {
            Ip::V4(ip) => ip.to_vec(),
            Ip::V6(ip) => ip.to_vec(),
            Ip::NONE => vec![],
        }
    }

    pub fn from_vec(bytes: Vec<u8>) -> Self {
        match bytes.len() {
            5 => {
                let mut ip = [0u8; 5];
                ip.copy_from_slice(&bytes);
                Ip::V4(ip)
            }
            17 => {
                let mut ip = [0u8; 17];
                ip.copy_from_slice(&bytes);
                Ip::V6(ip)
            }
            _ => Ip::NONE,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Ip::V4(ip) => format!("{}.{}.{}.{}{}", ip[0], ip[1], ip[2], ip[3], if ip[4] < 32 { format!("/{}", ip[4]) } else { "".to_string() }),
            Ip::V6(ip) => {
                let addr = (0..8)
                    .map(|i| format!("{:x}", ((ip[i * 2] as u16) << 8) | ip[i * 2 + 1] as u16))
                    .collect::<Vec<_>>()
                    .join(":");
                if ip[16] < 128 {
                    format!("{}/{}", addr, ip[16])
                } else {
                    addr
                }
            }
            Ip::NONE => "".to_string(),
        }
    }

    fn from_str(s: &str) -> std::result::Result<Ip, ParseIntError> {
        if s.is_empty() {
            return Ok(Ip::NONE);
        }
        let s = s.trim();
        if s.contains('.') && !s.contains(':') {
            let mut ip = [0u8; 5];
            let (addr_part, subnet_part) = s.split_once('/').unwrap_or((s, ""));
            for (idx, octet) in addr_part.split('.').enumerate().take(4) {
                ip[idx] = octet.parse::<u8>()?;
            }
            ip[4] = if let Ok(subnet) = subnet_part.parse::<u8>() { subnet.min(32) } else { 32 };
            return Ok(Ip::V4(ip));
        }
        if s.contains(':') && !s.contains('.') {
            let mut ip = [0u8; 17];
            let (addr_part, subnet_part) = s.split_once('/').unwrap_or((s, ""));
            let mut idx = 0;
            for part in addr_part.split(':').take(8) {
                let val = u16::from_str_radix(part, 16)?;
                ip[idx * 2] = (val >> 8) as u8;
                ip[idx * 2 + 1] = (val & 0xFF) as u8;
                idx += 1;
            }
            ip[16] = if let Ok(subnet) = subnet_part.parse::<u8>() { subnet.min(128) } else { 128 };
            return Ok(Ip::V6(ip));
        }
        Ok(Ip::NONE)
    }

    pub fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            Ip::V4(addr) => addr.hash(state),
            Ip::V6(addr) => addr.hash(state),
            Ip::NONE => 0.hash(state),
        }
    }

    pub fn index(&self, index: usize) -> &u8 {
        match self {
            Ip::V4(addr) => &addr[index],
            Ip::V6(addr) => &addr[index],
            Ip::NONE => panic!("Ip::NONE has no elements"),
        }
    }

    pub fn index_mut(&mut self, index: usize) -> &mut u8 {
        match self {
            Ip::V4(addr) => &mut addr[index],
            Ip::V6(addr) => &mut addr[index],
            Ip::NONE => panic!("Ip::NONE has no elements"),
        }
    }

    pub fn is_ip4(&self) -> bool {
        matches!(self, Ip::V4(_))
    }

    pub fn is_ip6(&self) -> bool {
        matches!(self, Ip::V6(_))
    }

    pub fn is_none(&self) -> bool {
        matches!(self, Ip::NONE)
    }

    pub fn is_valid_char(&self, c: char) -> bool {
        match self {
            Ip::V4(_) => c.is_ascii_digit() || c == '.' || c == '/',
            Ip::V6(_) => c.is_ascii_hexdigit() || c == ':' || c == '/',
            Ip::NONE => false,
        }
    }

    pub fn is_valid_str(s: &str, family: Family) -> bool {
        match family {
            Family::V4 => Self::is_valid_v4_str(s),
            Family::V6 => Self::is_valid_v6_str(s),
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
            Ip::V4(ip) => {
                let subnet = ip[4].min(32);
                if subnet == 0 {
                    return Ip::V4([255, 255, 255, 255, 0]);
                }
                let addr = u32::from_be_bytes([ip[0], ip[1], ip[2], ip[3]]);
                let mask = u32::MAX << (32 - subnet);
                let broadcast_addr = addr | !mask;
                let octets = broadcast_addr.to_be_bytes();
                Ip::V4([octets[0], octets[1], octets[2], octets[3], subnet])
            }
            Ip::V6(ip) => {
                let subnet = ip[16].min(128);
                let mut addr = [0u8; 16];
                addr.copy_from_slice(&ip[0..16]);
                let mut bits_left = 128 - subnet as usize;
                let mut i = 15;
                while bits_left > 0 && i < 16 {
                    let set_bits = bits_left.min(8);
                    addr[i] |= (1u8 << set_bits) - 1;
                    bits_left -= set_bits;
                    if i == 0 {
                        break;
                    }
                    i -= 1;
                }
                let mut broadcast = [0u8; 17];
                broadcast[0..16].copy_from_slice(&addr);
                broadcast[16] = subnet;
                Ip::V6(broadcast)
            }
            Ip::NONE => Ip::NONE,
        }
    }

    pub fn network(&self) -> Self {
        match self {
            Ip::V4(ip) => {
                let subnet = ip[4].min(32);
                let addr = u32::from_be_bytes([ip[0], ip[1], ip[2], ip[3]]);
                let mask = u32::MAX << (32 - subnet);
                let net_addr = addr & mask;
                let octets = net_addr.to_be_bytes();
                Ip::V4([octets[0], octets[1], octets[2], octets[3], subnet])
            }
            Ip::V6(ip) => {
                let subnet = ip[16].min(128);
                let mut addr: u128 = 0;
                for i in 0..16 {
                    addr |= (ip[i] as u128) << (8 * (15 - i));
                }
                let mask = u128::MAX << (128 - subnet);
                let net_addr = addr & mask;
                let mut net = [0u8; 17];
                for i in 0..16 {
                    net[i] = ((net_addr >> (8 * (15 - i))) & 0xFF) as u8;
                }
                net[16] = subnet;
                Ip::V6(net)
            }
            Ip::NONE => Ip::NONE,
        }
    }

    pub fn wildcard(&self) -> Self {
        match self {
            Ip::V4(ip) => {
                let subnet = ip[4].min(32);
                let mask = u32::MAX << (32 - subnet);
                let wildcard_val = !mask & 0xFFFF_FFFF;
                let octets = wildcard_val.to_be_bytes();
                Ip::V4([octets[0], octets[1], octets[2], octets[3], subnet])
            }
            Ip::V6(ip) => {
                let subnet = ip[16].min(128);
                let mask = u128::MAX << (128 - subnet);
                let wildcard_val = !mask;
                let mut wildcard = [0u8; 17];
                for i in 0..16 {
                    wildcard[i] = ((wildcard_val >> (8 * (15 - i))) & 0xFF) as u8;
                }
                wildcard[16] = subnet;
                Ip::V6(wildcard)
            }
            Ip::NONE => Ip::NONE,
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
        match (self, other) {
            (Ip::V4(a), Ip::V4(b)) => a.cmp(b),
            (Ip::V4(a), Ip::V6(b)) => a.to_vec().cmp(&b.to_vec()),
            (Ip::V6(a), Ip::V4(b)) => a.to_vec().cmp(&b.to_vec()),
            (Ip::V6(a), Ip::V6(b)) => a.cmp(b),
            (Ip::NONE, Ip::NONE) => Ordering::Equal,
            (Ip::NONE, _) => Ordering::Less,
            (_, Ip::NONE) => Ordering::Greater,
        }
    }
}
impl Default for Ip {
    fn default() -> Self {
        Ip::NONE
    }
}
impl Display for Ip {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result {
        write!(f, "{}", self.to_string())
    }
}

impl FromStr for Ip {
    type Err = ParseIntError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ip::from_str(s)
    }
}

impl std::hash::Hash for Ip {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        Ip::hash(self, state);
    }
}

impl Index<usize> for Ip {
    type Output = u8;
    fn index(&self, index: usize) -> &Self::Output {
        match self {
            Ip::V4(addr) => &addr[index],
            Ip::V6(addr) => &addr[index],
            Ip::NONE => panic!("Ip has an invalid number of elements"),
        }
    }
}

impl IndexMut<usize> for Ip {
    fn index_mut(&mut self, index: usize) -> &mut u8 {
        match self {
            Ip::V4(addr) => &mut addr[index],
            Ip::V6(addr) => &mut addr[index],
            Ip::NONE => panic!("Ip has an invalid number of elements"),
        }
    }
}

impl Family {
    pub fn new(family: Family) -> Self {
        family
    }
    pub fn default() -> Self {
        Family::NONE
    }
    pub fn set(&mut self, family: Family) -> &mut Self {
        *self = family;
        self
    }

    pub fn get(&self) -> Self {
        self.clone()
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Family::V4 => "v4",
            Family::V6 => "v6",
            Family::NONE => "none",
        }
    }

    pub fn to_string(&self) -> String {
        self.as_str().to_string()
    }

    pub fn from_str(s: &str) -> Self {
        match s.trim().to_ascii_lowercase().as_str() {
            "v4" | "4" | "ipv4" | "ip4" => Family::V4,
            "v6" | "6" | "ipv6" | "ip6" => Family::V6,
            "none" | "" => Family::NONE,
            _ => Family::NONE,
        }
    }

    pub fn from_ip(ip: &Ip) -> Self {
        match ip {
            Ip::V4(_) => Family::V4,
            Ip::V6(_) => Family::V6,
            Ip::NONE => Family::NONE,
        }
    }

    pub fn is_v4(&self) -> bool {
        *self == Family::V4
    }

    pub fn is_v6(&self) -> bool {
        *self == Family::V6
    }

    pub fn is_none(&self) -> bool {
        *self == Family::NONE
    }
}

impl Display for Family {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.as_str())
    }
}

impl FromStr for Family {
    type Err = ParseIntError;

    fn from_str(s: &str) -> std::result::Result<Family, ParseIntError> {
        Ok(Family::from_str(s))
    }
}

impl From<&Ip> for Family {
    fn from(ip: &Ip) -> Self {
        Family::from_ip(ip)
    }
}

impl From<Ip> for Family {
    fn from(ip: Ip) -> Self {
        Family::from_ip(&ip)
    }
}

impl PartialOrd for Family {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Family {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Family::V4, Family::V4) => Ordering::Equal,
            (Family::V6, Family::V4) => Ordering::Greater,
            (Family::V6, Family::V6) => Ordering::Equal,
            (Family::NONE, Family::NONE) => Ordering::Equal,
            _ => Ordering::Less,
        }
    }
}

pub static PROTO_LIST: LazyLock<Vec<Protocol>> = LazyLock::new(|| {
    Protocol::import(&Path::new("/etc/protocols")).expect("Failed to import protocol information")
});
impl Protocol {
    pub fn new() -> Self {
        Self(0, "".to_string())
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

    pub fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.to_string())
    }

    pub fn number(&self) -> u8 {
        self.0
    }

    pub fn name(&self) -> String {
        let s = self.1.split(',').next().unwrap_or("").trim();
        if s.is_empty() {
            "unknown".to_string()
        } else {
            s.to_string()
        }
    }

    pub fn description(&self) -> String {
        let mut parts = self.1.splitn(2, ',');
        parts.next();
        parts.next().unwrap_or("").trim().to_string()
    }

    pub fn set_number(&mut self, number: u8) -> Self {
        self.0 = number;
        self.clone()
    }

    pub fn set_name(&mut self, name: &str) -> Self {
        self.1 = format!("{},{}", name, self.description());
        self.clone()
    }

    pub fn set_description(&mut self, description: &str) -> Self {
        self.1 = format!("{},{}", self.name(), description);
        self.clone()
    }

    pub fn to_string(&self) -> String {
        let name = self.name();
        let desc = self.description();
        if desc.is_empty() {
            format!("{} ({})", name, self.number())
        } else {
            format!("{} ({}) # {}", name, self.number(), desc)
        }
    }

    pub fn from_str(s: &str) -> std::result::Result<Self, ParseIntError> {
        // Accepts nftables style: "tcp 6 # Transmission Control"
        let s = s.trim();
        let mut main_part = s;
        let mut description = "";
        if let Some(hash_idx) = s.find('#') {
            main_part = &s[..hash_idx];
            description = s[hash_idx + 1..].trim();
        }
        let mut parts = main_part.trim().split_whitespace();
        let proto_name = parts.next().unwrap_or("").trim();
        let number_str = parts.next().unwrap_or("").trim();
        let number = if !number_str.is_empty() {
            u8::from_str(number_str)?
        } else if let Ok(num) = u8::from_str(proto_name) {
            num
        } else {
            254
        };
        let name = if number_str.is_empty() || proto_name == number_str {
            proto_name
        } else {
            proto_name
        };
        Ok(Protocol(
            number,
            if description.is_empty() {
                format!("{},", name)
            } else {
                format!("{},{}", name, description)
            },
        ))
    }

    pub fn from_num(number: u8) -> Self {
        PROTO_LIST
            .iter()
            .find(|p| p.0 == number)
            .unwrap_or(&Protocol(254, "".to_string()))
            .clone()
    }

    /// Import protocols from `/etc/protocols` or nftables-formatted file
    pub fn import(path: &Path) -> std::result::Result<Vec<Protocol>, std::io::Error> {
        let mut file = File::open(path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        let mut protos = Vec::<Protocol>::new();
        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            // Try to parse nftables-style: "tcp 6 # Transmission Control"
            let mut parts = line.splitn(2, '#');
            let main_part = parts.next().unwrap().trim();
            let description = parts.next().unwrap_or("").trim();
            let mut fields = main_part.split_whitespace();
            let proto_name = fields.next().unwrap_or("").trim();
            let proto_number = fields.next().unwrap_or("").trim();
            if !proto_name.is_empty() && !proto_number.is_empty() {
                if let Ok(number) = u8::from_str(proto_number) {
                    protos.push(Protocol(
                        number,
                        if description.is_empty() {
                            format!("{},", proto_name)
                        } else {
                            format!("{},{}", proto_name, description)
                        },
                    ));
                    continue;
                }
            }
            // Fallback: try /etc/protocols-style: "icmp 1 ICMP # Internet Control Message"
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
                    protos.push(Protocol(
                        number,
                        if desc.is_empty() {
                            format!("{},", name)
                        } else {
                            format!("{},{}", name, desc)
                        },
                    ));
                }
            }
        }
        Ok(protos)
    }
}
impl Default for Protocol {
    fn default() -> Self {
        Protocol(255, "".to_string())
    }
}
impl Display for Protocol {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.to_string())
    }
}

impl FromStr for Protocol {
    type Err = ParseIntError;
    fn from_str(s: &str) -> std::result::Result<Protocol, ParseIntError> {
        Protocol::from_str(s)
    }
}

impl Hash for Protocol {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl Ord for Protocol {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}
impl PartialOrd for Protocol {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub static PORT_LIST: LazyLock<Vec<Port>> = LazyLock::new(|| {
    Port::import(&Path::new("/etc/services")).expect("Failed to import port information")
});
impl Port {
    pub fn new() -> Self {
        Self(0, 254, String::from(""))
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
            1 => "icmp",
            2 => "igmp",
            6 => "tcp",
            17 => "udp",
            58 => "icmpv6",
            132 => "sctp",
            37 => "ddp",
            _ => "unknown",
        };
        if self.2.is_empty() {
            format!("{} dport {}", proto, self.0)
        } else {
            // If service name is available, add as comment
            format!("{} dport {} # {}", proto, self.0, self.2)
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
        let mut proto = 254u8;
        let mut number = 0u16;
        let mut service = String::new();

        // Split off comments
        let (main, comment) = if let Some(idx) = s.find('#') {
            (s[..idx].trim(), s[idx + 1..].trim())
        } else {
            (s, "")
        };

        let mut parts = main.split_whitespace();
        if let Some(proto_str) = parts.next() {
            proto = match proto_str.to_ascii_lowercase().as_str() {
                "tcp" => 6,
                "udp" => 17,
                "sctp" => 132,
                "ddp" => 37,
                "igmp" => 2,
                "icmp" => 1,
                "icmpv6" => 58,
                _ => 254,
            };
        }
        if let Some(port_str) = parts.next() {
            if port_str == "dport" {
                if let Some(port_str2) = parts.next() {
                    number = u16::from_str(port_str2)?;
                }
            } else {
                number = u16::from_str(port_str)?;
            }
        }
        if !comment.is_empty() {
            service = comment.to_string();
        }

        Ok(Port(number, proto, service))
    }

    pub fn from_num(number: u16) -> Self {
        PORT_LIST
            .iter()
            .find(|p| p.0 == number)
            .cloned()
            .unwrap_or(Port(number, 254, String::from("")))
    }

    /// Import ports from `/etc/services` or nftables-formatted file
    pub fn import(path: &Path) -> std::result::Result<Vec<Port>, std::io::Error> {
        let mut file = File::open(path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        let mut ports = Vec::<Port>::new();

        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            // Try nftables syntax first: e.g. "tcp dport 22 # ssh"
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
                let description = line
                    .find('#')
                    .map(|i| line[i + 1..].trim().to_string())
                    .unwrap_or(String::new());
                if let Ok(port_number) = port_str.parse::<u16>() {
                    let proto_num = match proto_str.trim() {
                        "tcp" => 6,
                        "udp" => 17,
                        "ddp" => 37,
                        "sctp" => 132,
                        "igmp" => 2,
                        "icmp" => 1,
                        "icmpv6" => 58,
                        _ => 254,
                    };
                    ports.push(Port(
                        port_number,
                        proto_num,
                        if !description.is_empty() {
                            format!("{}, {}", service, description)
                        } else {
                            service.to_string()
                        },
                    ));
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
impl FromStr for Port {
    type Err = ParseIntError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
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
impl Hash for Port {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}
impl Action {
    pub fn new() -> Self {
        Action::DROP
    }
    pub fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.to_string())
    }
    /// Converts the Action variant into its nftables syntax string representation.
    pub fn to_string(&self) -> String {
        match self {
            Action::ACCEPT => "accept".to_string(),
            Action::DROP => "drop".to_string(),
            Action::REJECT => "reject".to_string(),
            Action::SKIP => "skip".to_string(),
            Action::LIMIT(val) => format!("limit rate over {}/second drop", val),
            Action::RETURN => "return".to_string(),
            Action::JUMP(chain) => format!("jump chain_{}", chain),
            Action::LOG(msg) => {
                if msg.is_empty() {
                    "log".to_string()
                } else {
                    format!("log prefix \"{}\"", msg)
                }
            }
            Action::MARK(val) => format!("meta mark set {}", val),
            Action::META(msg) => format!("meta {}", msg),
        }
    }

    pub fn from_str(s: &str) -> std::result::Result<Self, ParseIntError> {
        let mut parts = s.trim().split_whitespace().peekable();
        let action = parts.next().unwrap_or("drop").to_lowercase();

        match action.as_str() {
            "accept" => Ok(Action::ACCEPT),
            "drop" => Ok(Action::DROP),
            "reject" => Ok(Action::REJECT),
            "skip" => Ok(Action::SKIP),
            "limit" => {
                let mut val = None;
                while let Some(token) = parts.next() {
                    if token == "over" {
                        if let Some(rate_token) = parts.next() {
                            if let Some(num) = rate_token.split('/').next() {
                                val = Some(num.parse::<u32>()?);
                                break;
                            }
                        }
                    }
                }
                Ok(Action::LIMIT(val.unwrap_or(0)))
            }
            "return" => Ok(Action::RETURN),
            "jump" => {
                if let Some(chain_token) = parts.next() {
                    let chain_str = chain_token.trim_start_matches("chain_");
                    let val = chain_str.parse::<u32>()?;
                    Ok(Action::JUMP(val))
                } else {
                    Ok(Action::JUMP(0))
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
                Ok(Action::LOG(msg))
            }
            "meta" => {
                // Support both meta mark set <val> and meta <str>
                let next1 = parts.next();
                let next2 = parts.next();
                if let (Some(next1), Some(next2)) = (next1, next2) {
                    if next1 == "mark" && next2 == "set" {
                        if let Some(val_str) = parts.next() {
                            let val = val_str.parse::<u32>()?;
                            return Ok(Action::MARK(val));
                        } else {
                            return Ok(Action::MARK(0));
                        }
                    } else {
                        // meta <str>
                        let rest: Vec<_> = std::iter::once(next2)
                            .chain(parts)
                            .map(|s| s.to_string())
                            .collect();
                        return Ok(Action::META(rest.join(" ")));
                    }
                }
                Ok(Action::META("".to_string()))
            }
            _ => Ok(Action::DROP),
        }
    }

    /// Standard string representation (for debug, not nft syntax)

    pub fn cmp(&self, other: &Self) -> Ordering {
        use Action::*;
        match (self, other) {
            (ACCEPT, ACCEPT) => Ordering::Equal,
            (DROP, DROP) => Ordering::Equal,
            (REJECT, REJECT) => Ordering::Equal,
            (SKIP, SKIP) => Ordering::Equal,
            (LIMIT(value1), LIMIT(value2)) => value1.cmp(value2),
            (RETURN, RETURN) => Ordering::Equal,
            (JUMP(value1), JUMP(value2)) => value1.cmp(value2),
            (LOG(value1), LOG(value2)) => value1.cmp(value2),
            (MARK(value1), MARK(value2)) => value1.cmp(value2),
            (META(value1), META(value2)) => value1.cmp(value2),
            (a, b) => (a.variant_index()).cmp(&b.variant_index()),
        }
    }

    /// Returns true if the action is terminal (ACCEPT, DROP, REJECT, RETURN).
    pub fn is_terminal(&self) -> bool {
        matches!(
            self,
            Action::ACCEPT | Action::DROP | Action::REJECT | Action::RETURN
        )
    }

    /// Returns true if the action is a jump.
    pub fn is_jump(&self) -> bool {
        matches!(self, Action::JUMP(_))
    }

    /// Returns Some(limit value) if the action is LIMIT.
    pub fn limit_value(&self) -> Option<u32> {
        if let Action::LIMIT(v) = self {
            Some(*v)
        } else {
            None
        }
    }

    /// Returns the index of the action variant for consistent ordering in cmp.
    fn variant_index(&self) -> u8 {
        match self {
            Action::ACCEPT => 0,
            Action::DROP => 1,
            Action::REJECT => 2,
            Action::SKIP => 3,
            Action::LIMIT(_) => 4,
            Action::RETURN => 5,
            Action::JUMP(_) => 6,
            Action::LOG(_) => 7,
            Action::MARK(_) => 8,
            Action::META(_) => 9,
        }
    }
}

impl PartialOrd for Action {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Action {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cmp(other)
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

impl Chain {
    pub fn new(name: &str, number: u32) -> Self {
        Self {
            0: Vec::new(),
            1: number,
            2: name.to_string(),
        }
    }

    pub fn clone(&self) -> Self {
        Self {
            0: self.0.clone(),
            1: self.1,
            2: self.2.clone(),
        }
    }

    pub fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.to_string())
    }
    pub fn to_string(&self) -> String {
        let mut s = String::new();
        s.push_str(&format!("chain {} ({})\n", self.2, self.1));
        for rule in &self.0 {
            s.push_str("  ");
            s.push_str(&rule.to_string());
            s.push('\n');
        }
        s
    }

    pub fn from_str(s: &str) -> std::result::Result<Self, ParseIntError> {
        let mut rules = Vec::new();
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
            } else {
                rules.push(Rule::from_str(line)?);
            }
        }
        Ok(Self {
            0: rules,
            1: number,
            2: name,
        })
    }
    pub fn len(&self) -> usize {
        self.0.len()
    }
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
    pub fn push(&mut self, rule: Rule) {
        self.0.push(rule);
    }
    pub fn insert(&mut self, index: usize, rule: Rule) {
        self.0.insert(index, rule);
    }
    pub fn remove(&mut self, index: usize) -> Rule {
        self.0.remove(index)
    }
    pub fn swap(&mut self, index: usize, other: usize) {
        self.0.swap(index, other);
    }
    pub fn swap_remove(&mut self, index: usize) -> Rule {
        self.0.swap_remove(index)
    }
    pub fn clear(&mut self) {
        self.0.clear();
    }
    pub fn rules(&self) -> &Vec<Rule> {
        &self.0
    }
    pub fn set_rules(&mut self, rules: Vec<Rule>) {
        self.0 = rules;
    }
    pub fn name(&self) -> &str {
        &self.2
    }
    pub fn set_name(&mut self, name: &str) {
        self.2 = name.to_string();
    }
    pub fn number(&self) -> u32 {
        self.1
    }
    pub fn set_number(&mut self, number: u32) {
        self.1 = number;
    }
}
impl Rule {
    pub fn new(
        src_ips: Set<Ip, 65535>,
        dst_ips: Set<Ip, 65535>,
        protocols: Set<Protocol, 255>,
        src_ports: Set<Port, 65535>,
        dst_ports: Set<Port, 65535>,
        action: Action,
        comment: String,
    ) -> Rule {
        Self {
            0: src_ips,
            1: dst_ips,
            2: protocols,
            3: src_ports,
            4: dst_ports,
            5: action,
            6: comment,
        }
    }
    pub fn clone(&self) -> Self {
        Self {
            0: self.0.clone(),
            1: self.1.clone(),
            2: self.2.clone(),
            3: self.3.clone(),
            4: self.4.clone(),
            5: self.5.clone(),
            6: self.6.clone(),
        }
    }
    pub fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }

    pub fn cmp(&self, other: &Self) -> Ordering {
        (
            &self.0,
            &self.1,
            &self.2,
            &self.3,
            &self.4,
            &self.5,
            &self.6,
        )
            .cmp(&(
                &other.0,
                &other.1,
                &other.2,
                &other.3,
                &other.4,
                &other.5,
                &other.6,
            ))
    }

    pub fn eq(&self, other: &Self) -> bool {
        (
            &self.0,
            &self.1,
            &self.2,
            &self.3,
            &self.4,
            &self.5,
            &self.6,
        ) == (
            &other.0,
            &other.1,
            &other.2,
            &other.3,
            &other.4,
            &other.5,
            &other.6,
        )
    }

    pub fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
    pub fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.to_string())
    }
    pub fn src_ips(&self) -> &Set<Ip, 65535> {
        &self.0
    }
    pub fn dst_ips(&self) -> &Set<Ip, 65535> {
        &self.1
    }
    pub fn protocols(&self) -> &Set<Protocol, 255> {
        &self.2
    }
    pub fn src_ports(&self) -> &Set<Port, 65535> {
        &self.3
    }
    pub fn dst_ports(&self) -> &Set<Port, 65535> {
        &self.4
    }
    pub fn action(&self) -> &Action {
        &self.5
    }
    pub fn comment(&self) -> &str {
        &self.6
    }
    pub fn to_string(&self) -> String {
        let mut s = String::new();
        let family = self.family();
        // Family, table, and chain (default "filter input" for now)
        s.push_str(&format!("{} filter input ", family.as_str()));

        // Source IPs
        if self.src_ips().len() != 0 {
            if self.src_ips().len() > 1 {
                let mut ips: Vec<String> = self.src_ips().iter().map(|ip| if ip.family() == family { ip.to_string() } else { String::new() }).collect();
                ips.retain(|ip| !ip.is_empty());
                if !ips.is_empty() {
                    s.push_str(&format!("ip saddr {{ {} }} ", ips.join(", ")));
                }
            } else if self.src_ips().len() == 1 && self.src_ips().iter().next().unwrap().family() == family {
                s.push_str(&format!("ip saddr {} ", self.src_ips().iter().next().unwrap().to_string()));
            }
        }

        // Destination IPs
        if !self.dst_ips().is_empty() {
            if self.dst_ips().len() > 1 {
                let mut ips: Vec<String> = self.dst_ips().iter().map(|ip| if ip.family() == family { ip.to_string() } else { String::new() }).collect();
                ips.retain(|ip| !ip.is_empty());
                if !ips.is_empty() {
                    s.push_str(&format!("ip daddr {{ {} }} ", ips.join(", ")));
                }
            } else if self.dst_ips().len() == 1 && self.dst_ips().iter().next().unwrap().family() == family {
                s.push_str(&format!("ip daddr {} ", self.dst_ips().iter().next().unwrap().to_string()));
            }
        }

        // Protocols
        if !self.protocols().is_empty() {
            if self.protocols().len() > 1 {
                let mut protos: Vec<String> = self.protocols().iter().map(|p| p.to_string()).collect();
                protos.retain(|p| !p.is_empty());
                if !protos.is_empty() {
                    s.push_str(&format!("{{ {} }} ", protos.join(", ")));
                }
            } else {
                s.push_str(&format!("{} ", self.protocols().iter().next().unwrap().to_string()));
            }
        }

        // Source Ports
        if !self.src_ports().is_empty() {
            if self.src_ports().len() > 1 {
                let ports: Vec<String> = self.src_ports().iter().map(|p| p.to_string()).collect();
                s.push_str(&format!("{{ {} }} ", ports.join(", ")));
            } else {
                s.push_str(&format!("{} ", self.src_ports().iter().next().unwrap().to_string()));
            }
        }

        // Destination Ports
        if !self.dst_ports().is_empty() {
            if self.dst_ports().len() > 1 {
                let ports: Vec<String> = self.dst_ports().iter().map(|p| p.to_string()).collect();
                s.push_str(&format!("{{ {} }} ", ports.join(", ")));
            } else {
                s.push_str(&format!("{} ", self.dst_ports().iter().next().unwrap().to_string()));
            }
        }

        // Action
        s.push_str(&self.action().to_string());

        // Comment if present
        if !self.comment().is_empty() {
            s.push_str(&format!(" comment \"{}\"", self.comment()));
        }

        s.trim_end().to_string()
    }
    /// Parse a rule string in nftables style into a Rule struct.
    /// Example: `ip filter input ip saddr { 192.168.1.1 } ip daddr { 10.0.0.1 } ip protocol { tcp, udp } tcp sport { 80, 443 } tcp dport { 8080 } accept comment "My rule"`
    pub fn from_str(s: &str) -> std::result::Result<Self, ParseIntError> {
        let mut src_ips = Set::new();
        let mut dst_ips = Set::new();
        let mut protocols = Set::new();
        let mut src_ports = Set::new();
        let mut dst_ports = Set::new();
        let mut action = Action::DROP;
        let mut comment = String::new();

        let s = s.trim();
        let mut tokens = s.split_whitespace().peekable();

        while let Some(token) = tokens.next() {
            let token = token.trim().to_lowercase();
            match token.as_str() {
                "ip" | "ip4" | "ip6" | "filter" | "input" | "output" | "forward" => {
                    // skip family/table/chain keywords
                }
                "saddr" | "daddr" => {
                    let is_src = token == "saddr";
                    let out_set = if is_src { &mut src_ips } else { &mut dst_ips };
                    if let Some(next) = tokens.peek() {
                        if next.starts_with('{') {
                            // Multi-IP block: { ... }
                            let mut ips = String::new();
                            let mut done = false;
                            while let Some(ip_token) = tokens.next() {
                                let ip_token = ip_token.trim();
                                ips.push_str(ip_token);
                                if ip_token.ends_with('}') {
                                    done = true;
                                    break;
                                }
                                ips.push(' ');
                            }
                            if done {
                                let ips_trim = ips.trim().trim_start_matches('{').trim_end_matches('}');
                                for ip_str in ips_trim.split(',') {
                                    let ip_str = ip_str.trim();
                                    if !ip_str.is_empty() {
                                        out_set.push(Ip::from_str(ip_str)?);
                                    }
                                }
                            }
                        } else if let Some(ip_token) = tokens.next() {
                            let ip = ip_token.trim_end_matches('}').trim();
                            if !ip.is_empty() {
                                out_set.push(Ip::from_str(ip)?);
                            }
                        }
                    }
                }
                "protocol" | "proto" => {
                    if let Some(next) = tokens.peek() {
                        if next.starts_with('{') {
                            // Multi-protocol block: { ... }
                            let mut protos = String::new();
                            let mut done = false;
                            while let Some(proto_token) = tokens.next() {
                                let proto_token = proto_token.trim();
                                protos.push_str(proto_token);
                                if proto_token.ends_with('}') {
                                    done = true;
                                    break;
                                }
                                protos.push(' ');
                            }
                            if done {
                                let protos_trim = protos.trim().trim_start_matches('{').trim_end_matches('}');
                                for proto in protos_trim.split(',') {
                                    let proto = proto.trim();
                                    if !proto.is_empty() {
                                        protocols.push(Protocol::from_str(proto)?);
                                    }
                                }
                            }
                        } else if let Some(proto_token) = tokens.next() {
                            let proto = proto_token.trim_end_matches('}').trim();
                            if !proto.is_empty() {
                                protocols.push(Protocol::from_str(proto)?);
                            }
                        }
                    }
                }
                "tcp" | "udp" | "sctp" | "icmp" | "icmpv6" | "igmp" => {
                    // Add protocol if not present
                    if !protocols.iter().any(|p| p.to_string().to_lowercase() == token.to_lowercase()) {
                        protocols.push(Protocol::from_str(token.as_str())?);
                    }
                    // Look for sport/dport after proto
                    while let Some(next) = tokens.peek() {
                        match *next {
                            "sport" | "dport" => {
                                let is_src = *next == "sport";
                                tokens.next(); // consume "sport" or "dport"
                                if let Some(val_token) = tokens.peek() {
                                    if val_token.starts_with('{') {
                                        // Multi-port block: { ... }
                                        let mut ports = String::new();
                                        let mut done = false;
                                        while let Some(port_token) = tokens.next() {
                                            let port_token = port_token.trim();
                                            ports.push_str(port_token);
                                            if port_token.ends_with('}') {
                                                done = true;
                                                break;
                                            }
                                            ports.push(' ');
                                        }
                                        if done {
                                            let ports_trim = ports.trim().trim_start_matches('{').trim_end_matches('}');
                                            for p in ports_trim.split(',') {
                                                let p = p.trim();
                                                if !p.is_empty() {
                                                    let port = Port::from_str(&format!("{} {}", token, p))?;
                                                    if is_src {
                                                        src_ports.push(port);
                                                    } else {
                                                        dst_ports.push(port);
                                                    }
                                                }
                                            }
                                        }
                                    } else if let Some(p) = tokens.next() {
                                        let port = p.trim();
                                        if !port.is_empty() {
                                            let port = Port::from_str(&format!("{} {}", token, port))?;
                                            if is_src {
                                                src_ports.push(port);
                                            } else {
                                                dst_ports.push(port);
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
                    let is_src = token == "sport";
                    if let Some(next) = tokens.peek() {
                        if next.starts_with('{') {
                            let mut ports = String::new();
                            let mut done = false;
                            while let Some(port_token) = tokens.next() {
                                let port_token = port_token.trim();
                                ports.push_str(port_token);
                                if port_token.ends_with('}') {
                                    done = true;
                                    break;
                                }
                                ports.push(' ');
                            }
                            if done {
                                let ports_trim = ports.trim().trim_start_matches('{').trim_end_matches('}');
                                for p in ports_trim.split(',') {
                                    let p = p.trim();
                                    if !p.is_empty() {
                                        let port = Port::from_str(p)?;
                                        if is_src {
                                            src_ports.push(port);
                                        } else {
                                            dst_ports.push(port);
                                        }
                                    }
                                }
                            }
                        } else if let Some(p) = tokens.next() {
                            let port = Port::from_str(p.trim())?;
                            if is_src {
                                src_ports.push(port);
                            } else {
                                dst_ports.push(port);
                            }
                        }
                    }
                }
                "accept" | "drop" | "reject" | "skip" | "return" => {
                    action = Action::from_str(token.as_str()).unwrap_or(Action::DROP);
                }
                "limit" | "jump" | "log" | "mark" | "meta" => {
                    if let Some(arg) = tokens.next() {
                        action = Action::from_str(&format!("{} {}", token, arg)).unwrap_or(Action::DROP);
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
                            comment = comment_str.trim_end_matches('"').trim().to_string();
                        } else {
                            comment = c.to_string();
                        }
                    }
                }
                _ => {}
            }
        }

        Ok(Rule {
            0: src_ips,
            1: dst_ips,
            2: protocols,
            3: src_ports,
            4: dst_ports,
            5: action,
            6: comment,
        })
    }

    pub fn optimize(&mut self) {
        self.0.optimize();
        self.1.optimize();
        self.2.optimize();
        self.3.optimize();
        self.4.optimize();
    }

    pub fn append(&mut self, other: &mut Self) {
        self.0.append(&mut other.0);
        self.1.append(&mut other.1);
        self.2.append(&mut other.2);
        self.3.append(&mut other.3);
        self.4.append(&mut other.4);
        self.optimize();
    }

    pub fn family(&self) -> Family {
        // Family is inferred from contained IP addresses if possible
        if self
            .0
            .iter()
            .chain(self.1.iter())
            .any(|ip| matches!(ip, Ip::V6(_)))
        {
            Family::V6
        } else if self
            .0
            .iter()
            .chain(self.1.iter())
            .any(|ip| matches!(ip, Ip::V4(_)))
        {
            Family::V4
        } else {
            Family::NONE
        }
    }

    /// Executes the action specified by the rule.

    const ACCEPT: &'static str = "ACCEPT";
    const DROP: &'static str = "DROP";
    const REJECT: &'static str = "REJECT";
    const SKIP: &'static str = "SKIP";
    const RETURN: &'static str = "RETURN";
    const JUMP: &'static str = "JUMP";
    const LOG: &'static str = "LOG";
    const MARK: &'static str = "MARK";
    const LIMIT: &'static str = "LIMIT";
    const META: &'static str = "META";

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

    pub fn limit(_value: u32) -> String {
        format!("{} {}", Self::LIMIT, _value)
    }

    pub fn jump(_value: u32) -> String {
        format!("{} {}", Self::JUMP, _value)
    }

    pub fn log(_message: &str) -> String {
        format!("{} {}", Self::LOG, _message)
    }

    pub fn mark(_value: u32) -> String {
        format!("{} {}", Self::MARK, _value)
    }

    pub fn meta(_message: &str) -> String {
        format!("{} {}", Self::META, _message)
    }
}
impl Default for Rule {
    fn default() -> Self {
        Self {
            0: Set::new(),
            1: Set::new(),
            2: Set::new(),
            3: Set::new(),
            4: Set::new(),
            5: Action::DROP,
            6: String::new(),
        }
    }
}
impl Display for Rule {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.to_string())
    }
}
impl PartialOrd for Rule {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Rule {
    fn cmp(&self, other: &Self) -> Ordering {
        Self::cmp(self, other)
    }
}

impl FromStr for Rule {
    type Err = ParseIntError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::from_str(s)
    }
}

impl<T: Clone + Eq + Hash + Ord + Default + FromStr + Display + Debug, const N: usize> Set<T, N>
{
    pub fn new() -> Self {
        Self {
            0: std::array::from_fn(|_| T::default()),
            1: 0,
            2: String::new(),
            3: 0,
        }
    }
    pub fn name(&self) -> &str {
        &self.2
    }
    pub fn set_name(&mut self, name: String) {
        self.2 = name;
    }
    pub fn id(&self) -> u32 {
        self.3
    }
    pub fn set_id(&mut self, id: u32) {
        self.3 = id;
    }
    pub fn items(&self) -> &[T] {
        &self.0[..self.1]
    }

    pub fn is_empty(&self) -> bool {
        self.1 == 0
    }

    pub fn len(&self) -> usize {
        self.1
    }

    pub fn push(&mut self, item: T) {
        if self.1 >= N {
            return;
        }
        if !self.contains(&item) {
            self.0[self.1] = item;
            self.1 += 1;
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.1 > 0 {
            self.1 -= 1;
            Some(std::mem::replace(&mut self.0[self.1], T::default()))
        } else {
            None
        }
    }

    pub fn remove(&mut self, item: &T) -> bool {
        for i in 0..self.1 {
            if &self.0[i] == item {
                self.0[i] = std::mem::replace(&mut self.0[self.1 - 1], T::default());
                self.1 -= 1;
                return true;
            }
        }
        false
    }

    pub fn insert(&mut self, item: T) {
        if self.1 >= N {
            return;
        }
        if !self.contains(&item) {
            self.0[self.1] = item;
            self.1 += 1;
        }
    }

    pub fn append(&mut self, other: &Self) {
        for i in 0..other.1 {
            self.push(other.0[i].clone());
        }
    }

    pub fn retain<F>(&mut self, mut f: F)
    where
        F: FnMut(&T) -> bool,
    {
        if self.1 >= N {
            return;
        }
        let mut k = 0;
        for i in 0..self.1 {
            if f(&self.0[i]) {
                if i != k {
                    self.0[k] = self.0[i].clone();
                }
                k += 1;
            }
        }
        // Clear tail
        for i in k..self.1 {
            self.0[i] = T::default();
        }
        self.1 = k;
    }

    pub fn clear(&mut self) {
        for i in 0..self.1 {
            self.0[i] = T::default();
        }
        self.1 = 0;
    }

    pub fn contains(&self, item: &T) -> bool {
        self.0[..self.1].contains(item)
    }

    pub fn to_string(&self) -> String {
        self.items()
            .iter()
            .map(|item| item.to_string())
            .collect::<Vec<_>>()
            .join("\n")
    }

    pub fn from_str(s: &str) -> std::result::Result<Self, <T as FromStr>::Err> {
        let mut set = Self::new();
        for item_str in s
            .split('\n')
            .map(str::trim)
            .filter(|s| !s.is_empty())
        {
            if set.1 >= N {
                break;
            }
            set.push(T::from_str(item_str)?);
        }
        Ok(set)
    }

    pub fn from_vec(vec: Vec<T>) -> Self {
        let mut set = Self::new();
        for item in vec {
            set.push(item);
        }
        set
    }
    pub fn to_vec(&self) -> Vec<T> {
        self.0[..self.1].to_vec()
    }
    pub fn iter(&self) -> std::slice::Iter<'_, T> {
        self.items().iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, T> {
        self.0[..self.1].iter_mut()
    }

    pub fn sort(&mut self) {
        self.0[..self.1].sort();
    }

    pub const fn max_size(&self) -> usize {
        N
    }

    /// Remove duplicates and sort the set.
    pub fn optimize(&mut self) {
        self.sort();
        let mut k = 0;
        for i in 0..self.1 {
            if k == 0 || self.0[i] != self.0[k - 1] {
                if i != k {
                    self.0[k] = self.0[i].clone();
                }
                k += 1;
            }
        }
        // Clear tail
        for i in k..self.1 {
            self.0[i] = T::default();
        }
        self.1 = k;
    }
}


impl<T, const N: usize> std::ops::Deref for Set<T, N>
where
    T: Clone + Eq + Hash + Ord + Default + FromStr + Display + Debug,
{
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        &self.0[..self.1]
    }
}

impl<T, const N: usize> std::ops::DerefMut for Set<T, N>
where
    T: Clone + Eq + Hash + Ord + Default + FromStr + Display + Debug,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0[..self.1]
    }
}

impl<T, const N: usize> FromStr for Set<T, N>
where
    T: FromStr + Display + Clone + Ord + Hash + Default + Debug,
{
    type Err = T::Err;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut set = Self::new();
        for item_str in s
            .split('\n')
            .map(str::trim)
            .filter(|s| !s.is_empty())
        {
            if set.1 >= N {
                break;
            }
            set.push(T::from_str(item_str)?);
        }
        Ok(set)
    }
}

impl<T, const N: usize> From<Vec<T>> for Set<T, N>
where
    T: FromStr + Display + Clone + Ord + Hash + Default + Debug,
{
    fn from(vec: Vec<T>) -> Self {
        let mut set = Set::new();
        let len = N.min(vec.len());
        let iter = vec.into_iter().take(len);
        for item in iter {
            set.push(item);
        }
        set
    }
}

impl<T, const N: usize> Display for Set<T, N>
where
    T: Clone + Eq + Hash + Ord + Default + FromStr + Display + Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl<T, const N: usize> Extend<T> for Set<T, N>
where
    T: Clone + Ord + Hash + Default + Debug + FromStr + Display,
{
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for item in iter {
            self.push(item);
        }
    }
}

impl<T: Ord + Clone + Eq + Hash + Default + FromStr + Display + Debug, const N: usize> PartialOrd
    for Set<T, N>
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: Ord + Clone + Eq + Hash + Default + FromStr + Display + Debug, const N: usize> Ord
    for Set<T, N>
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.0[..self.1].cmp(&other.0[..other.1])
    }
}

impl<T: Clone + Eq + Hash + Ord + Default + FromStr + Display + Debug, const N: usize> std::ops::Index<usize> for Set<T, N>
{
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        if index >= self.1 {
            panic!("index out of bounds");
        }
        &self.0[index]
    }
}

impl<T: Clone + Eq + Hash + Ord + Default + FromStr + Display + Debug, const N: usize> std::ops::IndexMut<usize> for Set<T, N>
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index >= self.1 {
            panic!("index out of bounds");
        }
        &mut self.0[index]
    }
}

impl<T: Clone + Eq + Hash + Ord + Default + FromStr + Display + Debug, const N: usize> RangeBounds<usize> for Set<T, N>
{
    fn start_bound(&self) -> Bound<&usize> {
        Bound::Included(&0)
    }
    fn end_bound(&self) -> Bound<&usize> {
        Bound::Included(&self.1)
    }
}

impl<T: Clone + Eq + Hash + Ord + Default + FromStr + Display + Debug, const N: usize> IntoIterator for Set<T, N>
{
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;
    fn into_iter(self) -> Self::IntoIter {
        self.0[..self.1].to_vec().into_iter()
    }
}

impl<'a, T: Clone + Eq + Hash + Ord + Default + FromStr + Display + Debug, const N: usize> IntoIterator for &'a Set<T, N>
{
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.0[..self.1].iter()
    }
}

impl<'a, T: Clone + Eq + Hash + Ord + Default + FromStr + Display + Debug, const N: usize> IntoIterator for &'a mut Set<T, N>
{
    type Item = &'a mut T;
    type IntoIter = std::slice::IterMut<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.0[..self.1].iter_mut()
    }
}
