use std::cmp::Ordering;
use std::{
    fmt::{Display, Formatter},
    ops::{Add, AddAssign, Index, IndexMut},
    str::FromStr,
};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Info {
    pub data: Vec<String>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Error {
    pub message: String,
    pub code: u32,
}

impl Error {
    pub fn new(message: String, code: u32) -> Self {
        Self { message, code }
    }
    pub fn clone(&self) -> Self {
        Self {
            message: self.message.clone(),
            code: self.code,
        }
    }
    pub fn cmp(&self, other: &Self) -> Ordering {
        self.code.cmp(&other.code)
    }
    pub fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
    pub fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{}: {}", self.code, self.message)
    }
    pub fn to_string(&self) -> String {
        format!("{}: {}", self.code, self.message)
    }
    pub fn from_str(s: &str) -> Result<Self, Self> {
        let mut message = String::new();
        let mut code = 0;
        for line in s.lines() {
            if message.is_empty() {
                message = line.to_string();
            } else {
                code = line.parse().unwrap();
            }
        }
        Ok(Self { message, code })
    }
    pub fn display(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{}: {}", self.code, self.message)
    }
    pub fn message(&self) -> &String {
        &self.message
    }
    pub fn set_message(&mut self, message: String) -> &mut Self {
        self.message = message;
        self
    }
    pub fn set_code(&mut self, code: u32) -> &mut Self {
        self.code = code;
        self
    }
}
impl Clone for Error {
    fn clone(&self) -> Self {
        Self {
            message: self.message.clone(),
            code: self.code,
        }
    }
}
impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }

    fn description(&self) -> &str {
        &self.message
    }

    fn cause(&self) -> Option<&dyn std::error::Error> {
        None
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{}: {}", self.code, self.message)
    }
}

#[derive(Debug, Clone)]
pub struct Parser<T> {
    pub new: fn() -> Parser<T>,
    pub output: T,
    pub parse: fn(&str) -> Result<T, Error>,
    pub fmt: fn(&T, &mut Formatter) -> Result<(), std::fmt::Error>,
    pub to_string: fn(&T) -> String,
    pub from_str: fn(&str) -> Result<T, Error>,
}

impl<T: Display + ToString + Default + Clone + FromStr + PartialEq + Ord + PartialOrd> Parser<T> {
    pub fn new() -> Self {
        Self {
            new: || Self::new(),
            output: T::default(),
            parse: |s: &str| match T::from_str(s) {
                Ok(t) => Ok(t),
                Err(_e) => Err(Error {
                    message: "Format error".to_string(),
                    code: 2,
                }),
            },
            fmt: |t: &T, f: &mut Formatter| write!(f, "{}", t),
            to_string: |t: &T| t.to_string(),
            from_str: |s: &str| match T::from_str(s) {
                Ok(t) => Ok(t),
                Err(_e) => Err(Error {
                    message: "Format error".to_string(),
                    code: 2,
                }),
            },
        }
    }
    pub fn parse(&self, s: &str) -> Result<T, Error> {
        (self.parse)(s)
    }

    pub fn fmt(&self, value: &T, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        (self.fmt)(value, f)
    }

    pub fn to_string(&self) -> String {
        (self.to_string)(&self.output)
    }

    pub fn from_str(&self, s: &str) -> Result<T, Error> {
        (self.from_str)(s)
    }
    pub fn display(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.to_string())
    }
}
impl<T: Eq> Eq for Parser<T> {}

impl<T: PartialEq> PartialEq for Parser<T> {
    fn eq(&self, other: &Self) -> bool {
        self.output == other.output
    }
}

impl<T: Display> Display for Parser<T> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.output)
    }
}
impl<T: PartialOrd> PartialOrd for Parser<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.output.partial_cmp(&other.output) {
            ord => return ord,
        }
    }
}
impl<T: Ord> Ord for Parser<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.output.cmp(&other.output)
    }
}
impl Info {
    pub fn new() -> Self {
        Self {
            data: Vec::<String>::new(),
        }
    }
    pub fn set(&mut self, name: String, description: String, details: Vec<String>) -> Self {
        self.data.clear();
        self.data.push(name);
        self.data.push(description);
        for detail in details {
            self.data.push(detail);
        }
        self.clone()
    }
    pub fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
        }
    }
    pub fn name(&self) -> &String {
        &self.data[0]
    }
    pub fn description(&self) -> &String {
        &self.data[1]
    }
    pub fn details(&self) -> Vec<String> {
        self.data[2..].to_vec()
    }
    pub fn set_name(&mut self, name: String) -> &mut Self {
        self.data[0] = name;
        self
    }
    pub fn set_description(&mut self, description: String) -> &mut Self {
        self.data[1] = description;
        self
    }
    pub fn set_details(&mut self, details: Vec<String>) -> &mut Self {
        let name = self.name().clone();
        let description = self.description().clone();
        self.data.clear();
        self.data.push(name);
        self.data.push(description);
        for detail in details {
            self.data.push(detail);
        }
        self
    }
    pub fn append(&mut self, info: &mut Info) {
        self.data.append(&mut info.data[2..].to_vec());
    }
    pub fn push(&mut self, detail: String) {
        let mut details = self.data[2..].to_vec();
        details.push(detail);
        self.set_details(details);
    }
    pub fn pop(&mut self) -> Option<String> {
        let mut details = self.data[2..].to_vec();
        if details.is_empty() {
            return None;
        }
        let detail = details.pop().unwrap();
        self.set_details(details);
        Some(detail)
    }
    pub fn len(&self) -> usize {
        self.data[2..].len()
    }
    pub fn is_empty(&self) -> bool {
        self.data[2..].is_empty()
    }
    pub fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        if self.data.len() < 3 {
            write!(f, "Invalid format")?;
            return Ok(());
        }
        write!(
            f,
            "{}\n{}\n{}",
            self.data[0],
            self.data[1],
            self.data[2..].join("\n")
        )
    }
    pub fn to_string(&self) -> String {
        format!(
            "{}\n{}\n{}",
            self.data[0],
            self.data[1],
            self.data[2..].join("\n")
        )
    }
    pub fn from_str(s: &str) -> Result<Self, Error> {
        let mut name = String::new();
        let mut description = String::new();
        let mut details = Vec::<String>::new();
        for line in s.lines() {
            if name.is_empty() {
                name = line.to_string();
            } else if description.is_empty() {
                description = line.to_string();
            } else {
                details.push(line.to_string());
            }
        }
        if name.is_empty() || description.is_empty() {
            return Err(Error {
                message: "Invalid format".to_string(),
                code: 2,
            });
        }
        Ok(Self {
            data: vec![name, description, details.join("\n")],
        })
    }
}

impl Display for Info {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}\n{}\n{}",
            self.data[0],
            self.data[1],
            self.data[2..].join("\n")
        )
    }
}

impl FromStr for Info {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut name = String::new();
        let mut description = String::new();
        let mut details = Vec::<String>::new();
        for line in s.lines() {
            if name.is_empty() {
                name = line.to_string();
            } else if description.is_empty() {
                description = line.to_string();
            } else {
                details.push(line.to_string());
            }
        }
        if name.is_empty() || description.is_empty() {
            return Err(Error {
                message: "Invalid format".to_string(),
                code: 2,
            });
        }
        Ok(Self {
            data: vec![name, description, details.join("\n")],
        })
    }
}
impl Add for Info {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            data: [self.data, other.data].concat(),
        }
    }
}
impl AddAssign for Info {
    fn add_assign(&mut self, other: Self) {
        *self = self.clone() + other;
    }
}
impl Index<usize> for Info {
    type Output = String;
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}
impl IndexMut<usize> for Info {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}
