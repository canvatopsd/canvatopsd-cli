//! URL-safe, 22-character encoding of a complete UUIDv7 task identity.
use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use serde::{Deserialize, Serialize};
use std::{fmt, str::FromStr};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct TaskId(pub uuid::Uuid);

impl FromStr for TaskId {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let bytes: [u8; 16] = URL_SAFE_NO_PAD
            .decode(value)
            .map_err(|_| "task_id_invalid")?
            .try_into()
            .map_err(|_| "task_id_invalid")?;
        let id = uuid::Uuid::from_bytes(bytes);
        if id.get_version_num() != 7 || id.get_variant() != uuid::Variant::RFC4122 {
            return Err("task_id_invalid");
        }
        Ok(Self(id))
    }
}
impl TryFrom<String> for TaskId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}
impl From<TaskId> for String {
    fn from(value: TaskId) -> Self {
        URL_SAFE_NO_PAD.encode(value.0.as_bytes())
    }
}
impl fmt::Display for TaskId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&String::from(*self))
    }
}
