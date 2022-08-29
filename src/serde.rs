//! This implements an opinionated version of the serde's (de)serializer for all `TypedId` whose
//! underlying type is (de)serializable. In short, `TypedId`s are (de)serialized as thier
//! underlying type. Otherwise, thier use as indices in maps is impractical.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

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

#[cfg(test)]
mod tests {
    use crate::TypedId;
    use serde::{Deserialize, Serialize};

    type CustomerId = TypedId<u32, Customer>;

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct Customer {
        name: String,
        id: CustomerId,
    }

    #[test]
    fn can_map() {
        use std::collections::HashMap;
        
        let map: HashMap<CustomerId, Customer> = (0..10)
            .map(|i| {
                (
                    i.into(),
                    Customer {
                        name: i.to_string(),
                        id: i.into(),
                    },
                )
            })
            .collect();
        let json = serde_json::to_string(&map).expect("Customer");
        let new_map : HashMap<CustomerId, Customer> = serde_json::from_str(&json).expect("Typed Customer");
        assert_eq!(new_map, map);
    }
}
