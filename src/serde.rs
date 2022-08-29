//! This implements an opinionated version of the serde's (de)serializer for all `TypedId` whose
//! underlying type is (de)serializable. In short, `TypedId`s are (de)serialized as thier
//! underlying type. Otherwise, thier use as indices in maps is impractical.

use serde::{Deserialize, Serialize, Deserializer, Serializer};

use crate::TypedId;

impl<'de, I: Deserialize<'de>, T> Deserialize<'de> for TypedId<I, T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        I::deserialize(deserializer).map(|id| id.into())
    }
}

impl<I: Serialize, T> Serialize for TypedId<I, T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.0.serialize(serializer)
    }
}
