use serde::de::Unexpected;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::borrow::Borrow;
use std::cmp::Ordering;
use std::fmt::{Debug, Display, Formatter};
use std::hash::{Hash, Hasher};
use std::mem;
use std::ops::Deref;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum IdentifierError {
    #[error("empty namespace")]
    EmptyNamespace,
    #[error("empty path")]
    EmptyPath,
    #[error("invalid namespace char '{0}'")]
    Namespace(char),
    #[error("invalid path char '{0}'")]
    Path(char),
}

pub type IdentifierResult<T> = Result<T, IdentifierError>;

#[derive(Debug, Clone, Eq)]
pub struct IdentifierBuf {
    value: String,
}

impl IdentifierBuf {
    pub fn new(str: impl Into<String>) -> IdentifierResult<IdentifierBuf> {
        let str = str.into();
        validate_namespace_and_path(&str)?;
        Ok(IdentifierBuf { value: str })
    }
}

impl PartialEq for IdentifierBuf {
    fn eq(&self, other: &Self) -> bool {
        **self == **other
    }
}

impl PartialOrd for IdentifierBuf {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for IdentifierBuf {
    fn cmp(&self, other: &Self) -> Ordering {
        Identifier::cmp(&**self, &**other)
    }
}

impl Hash for IdentifierBuf {
    fn hash<H: Hasher>(&self, state: &mut H) {
        Identifier::hash(&**self, state)
    }
}

impl Display for IdentifierBuf {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.value, f)
    }
}

impl AsRef<Identifier> for IdentifierBuf {
    fn as_ref(&self) -> &Identifier {
        Identifier::from_str(&self.value)
    }
}

impl Borrow<Identifier> for IdentifierBuf {
    fn borrow(&self) -> &Identifier {
        Identifier::from_str(&self.value)
    }
}

impl Deref for IdentifierBuf {
    type Target = Identifier;

    fn deref(&self) -> &Identifier {
        Identifier::from_str(&self.value)
    }
}

impl TryFrom<&str> for IdentifierBuf {
    type Error = IdentifierError;

    fn try_from(value: &str) -> IdentifierResult<Self> {
        IdentifierBuf::new(value)
    }
}

impl TryFrom<String> for IdentifierBuf {
    type Error = IdentifierError;

    fn try_from(value: String) -> IdentifierResult<Self> {
        IdentifierBuf::new(value)
    }
}

impl From<&Identifier> for IdentifierBuf {
    fn from(value: &Identifier) -> Self {
        value.to_owned()
    }
}

pub trait IntoIdentifierBuf {
    fn into_identifier_buf(self) -> IdentifierBuf;
}

impl<T> IntoIdentifierBuf for T
where
    IdentifierBuf: TryFrom<T>,
    <IdentifierBuf as TryFrom<T>>::Error: Debug,
{
    fn into_identifier_buf(self) -> IdentifierBuf {
        self.try_into().unwrap()
    }
}

#[derive(Debug, Eq)]
#[repr(transparent)]
pub struct Identifier {
    value: str,
}

impl Identifier {
    #[inline]
    pub fn new(str: &str) -> IdentifierResult<&Identifier> {
        validate_namespace_and_path(str)?;
        Ok(Self::from_str(str))
    }

    #[must_use]
    pub const fn new_const(str: &str) -> &Identifier {
        validate_namespace_and_path_const(str);
        Self::from_str(str)
    }

    #[must_use]
    const fn from_str(str: &str) -> &Identifier {
        // SAFETY: Identifier has the same layout as str
        unsafe { mem::transmute(str) }
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.value
    }

    #[must_use]
    pub fn namespace(&self) -> &str {
        self.namespace_and_path().0
    }

    #[must_use]
    pub fn path(&self) -> &str {
        self.namespace_and_path().1
    }

    #[must_use]
    #[inline]
    pub fn namespace_and_path(&self) -> (&str, &str) {
        if let Some((namespace, path)) = self.value.split_once(':') {
            (namespace, path)
        } else {
            ("minecraft", &self.value)
        }
    }

    #[must_use]
    pub fn to_datapack_path(&self, folder: &str, extension: &str) -> String {
        let (namespace, path) = self.namespace_and_path();
        format!("data/{namespace}/{folder}/{path}.{extension}")
    }
}

impl PartialEq for Identifier {
    fn eq(&self, other: &Self) -> bool {
        let this = self.value.strip_prefix("minecraft:").unwrap_or(&self.value);
        let other = other
            .value
            .strip_prefix("minecraft:")
            .unwrap_or(&self.value);
        this == other
    }
}

impl PartialOrd for Identifier {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Identifier {
    fn cmp(&self, other: &Self) -> Ordering {
        match (
            self.value.strip_prefix("minecraft:"),
            other.value.strip_prefix("minecraft:"),
        ) {
            (Some(this), Some(other)) => this.cmp(other),
            (Some(_), None) => "minecraft:".cmp(&other.value),
            (None, Some(_)) => self.value.cmp("minecraft:"),
            (None, None) => self.value.cmp(&other.value),
        }
    }
}

impl Hash for Identifier {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let value = self.value.strip_prefix("minecraft:").unwrap_or(&self.value);
        value.hash(state);
    }
}

impl Display for Identifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.value, f)
    }
}

impl ToOwned for Identifier {
    type Owned = IdentifierBuf;

    fn to_owned(&self) -> IdentifierBuf {
        IdentifierBuf {
            value: self.value.to_owned(),
        }
    }
}

impl<'a> TryFrom<&'a str> for &'a Identifier {
    type Error = IdentifierError;

    fn try_from(value: &'a str) -> IdentifierResult<&'a Identifier> {
        Identifier::new(value)
    }
}

impl<'a> From<&'a IdentifierBuf> for &'a Identifier {
    fn from(value: &'a IdentifierBuf) -> &'a Identifier {
        &**value
    }
}

pub trait IntoIdentifier<'a> {
    fn into_id(self) -> &'a Identifier;
}

impl<'a, T> IntoIdentifier<'a> for T
where
    T: TryInto<&'a Identifier>,
    <T as TryInto<&'a Identifier>>::Error: Debug,
{
    fn into_id(self) -> &'a Identifier {
        self.try_into().unwrap()
    }
}

#[inline]
fn validate_namespace_and_path(str: &str) -> IdentifierResult<()> {
    if let Some((namespace, path)) = str.split_once(':') {
        validate_namespace(namespace)?;
        validate_path(path)?;
    } else {
        validate_path(str)?;
    }
    Ok(())
}

#[inline]
fn validate_namespace(namespace: &str) -> IdentifierResult<()> {
    if namespace.is_empty() {
        return Err(IdentifierError::EmptyNamespace);
    }

    if let Some(first_invalid_char) = namespace
        .bytes()
        .position(|char| !is_valid_namespace_char(char))
    {
        // SAFETY: index points to first byte of invalid char
        let invalid_char = unsafe { char_at_unchecked(namespace, first_invalid_char) };
        return Err(IdentifierError::Namespace(invalid_char));
    }

    Ok(())
}

#[inline]
fn validate_path(path: &str) -> IdentifierResult<()> {
    if path.is_empty() {
        return Err(IdentifierError::EmptyPath);
    }

    if let Some(first_invalid_char) = path.bytes().position(|char| !is_valid_path_char(char)) {
        // SAFETY: index points to first byte of invalid char
        let invalid_char = unsafe { char_at_unchecked(path, first_invalid_char) };
        return Err(IdentifierError::Path(invalid_char));
    }

    Ok(())
}

/// # Safety
/// Assumes that `index` is a valid index into `str`, and that the index points to a char boundary
#[inline]
unsafe fn char_at_unchecked(str: &str, index: usize) -> char {
    debug_assert!(index < str.len());
    debug_assert!(str.is_char_boundary(index));

    str.get_unchecked(index..).chars().next().unwrap_unchecked()
}

const fn validate_namespace_and_path_const(str: &str) {
    let bytes = str.as_bytes();
    let mut colon_index = 0;
    while colon_index < bytes.len() {
        if bytes[colon_index] == b':' {
            break;
        }
        colon_index += 1;
    }
    if colon_index == bytes.len() {
        validate_path_const(bytes);
    } else {
        let (namespace, path) = bytes.split_at(colon_index);
        let path = path.split_at(1).1;
        validate_namespace_const(namespace);
        validate_path_const(path);
    }
}

const fn validate_namespace_const(namespace: &[u8]) {
    assert!(!namespace.is_empty(), "empty namespace");

    let mut index = 0;
    while index < namespace.len() {
        assert!(
            is_valid_namespace_char(namespace[index]),
            "invalid namespace char"
        );
        index += 1;
    }
}

const fn validate_path_const(path: &[u8]) {
    assert!(!path.is_empty(), "empty path");

    let mut index = 0;
    while index < path.len() {
        assert!(is_valid_path_char(path[index]), "invalid path char");
        index += 1;
    }
}

#[inline]
const fn is_valid_namespace_char(char: u8) -> bool {
    matches!(char, b'_' | b'-' | b'a'..=b'z' | b'0'..=b'9' | b'.')
}

#[inline]
const fn is_valid_path_char(char: u8) -> bool {
    matches!(char, b'_' | b'-' | b'a'..=b'z' | b'0'..=b'9' | b'/' | b'.')
}

impl<'de> Deserialize<'de> for IdentifierBuf {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let unparsed = String::deserialize(deserializer)?;
        Self::new(&unparsed)
            .map_err(|_| serde::de::Error::invalid_value(Unexpected::Str(&unparsed), &"identifier"))
    }
}

impl Serialize for IdentifierBuf {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.to_string().serialize(serializer)
    }
}
