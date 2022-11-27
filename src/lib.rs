use chrono::{Utc, DateTime};
use serde::{Serialize, Deserialize, de::DeserializeOwned};

use rmp_serde::{Serializer};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct WsMessage<B: Serialize> {
    id: u64,
    path: String,
    body: Option<B>,
    sent: DateTime<Utc>
}

impl<B: Serialize + DeserializeOwned> TryFrom<Vec<u8>> for WsMessage<B> {
    type Error = rmp_serde::decode::Error;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        rmp_serde::from_slice(&value)
    }
}

impl<B: Serialize> TryFrom<WsMessage<B>> for Vec<u8> {
    type Error = String;

    fn try_from(value: WsMessage<B>) -> Result<Self, Self::Error> {
        let mut buf = Vec::new();
        match value.serialize(&mut Serializer::new(&mut buf)) {
            Ok(_) => Ok(buf),
            Err(e) => Err(e.to_string()),
        }
    }
}