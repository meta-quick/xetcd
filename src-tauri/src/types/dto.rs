use serde::{Serialize, Deserialize};

#[derive(Debug,Serialize, Deserialize,Clone)]
pub struct DtoSimpleKeyValue {
    pub key: String,
    pub value: String,
}

#[derive(Debug,Serialize, Deserialize,Clone)]
pub struct DtoKeyValue {
    pub root: String,
    pub name: String,
    pub ttl: i64,
    pub isdir: bool,
    pub value: String,
}


#[derive(Debug,Serialize, Deserialize,Clone)]
pub struct JSonKeyValue {
    pub key: String,
    pub create_revision: i64,
    pub mod_revision: i64,
    pub version: i64,
    pub value: String,
    pub lease: i64,
}

#[derive(Deserialize,Serialize,Debug,Clone)]
pub struct EtcdConfig {
    pub name: String,
    pub address: String,
    pub authway: String,
    pub username: Option<String>,
    pub password: Option<String>,
    pub cert: Option<String>,
}

#[derive(Debug,Serialize, Deserialize,Clone)]
pub struct SimpleKeyValue {
    pub key: String,
    pub value: String,
}


impl SimpleKeyValue {
    #[inline]
    pub fn new(key: impl Into<String>, value: impl Into<String>) -> Self {
        SimpleKeyValue {
            key: key.into(),
            value: value.into(),
        }
    }

    #[inline]
    pub fn key(&self) -> &str {
        &self.key
    }

    #[inline]
    pub fn value(&self) -> &str {
        &self.value
    }
}


impl JSonKeyValue {
    #[inline]
    pub fn new(key: impl Into<String>, value: impl Into<String>) -> Self {
        JSonKeyValue {
            key: key.into(),
            value: value.into(),
            create_revision: 0,
            mod_revision: 0,
            version: 0,
            lease: 0
        }
    }

    #[inline]
    pub fn key(&self) -> &str {
        &self.key
    }

    #[inline]
    pub fn value(&self) -> &str {
        &self.value
    }
}

#[derive(Deserialize,Serialize,Debug,Clone)]
pub struct PermissionValue {
    pub ty: i32,
    pub key: String,
    pub range_end: String,
    pub with_prefix: bool,
    pub with_from_key: bool,
}