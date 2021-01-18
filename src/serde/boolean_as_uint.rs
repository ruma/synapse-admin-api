//! De-/serialization functions for casting boolean values to int and vise versa.
//! Delegates to `u64` to ensure integer size is within bounds.

use serde::{
    de::{self, Deserialize, Deserializer, Unexpected},
    ser::{Serialize, Serializer},
};

/// Serialize a boolean to u64.
pub fn serialize<S>(value: &bool, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match value {
        false => u8::serialize(&0u8, serializer),
        true => u8::serialize(&1u8, serializer),
    }
}

/// Deserializes a boolean from u64.
///
/// Will fail if integer is greater than 1.
pub fn deserialize<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    match u8::deserialize(deserializer)? {
        0 => Ok(false),
        1 => Ok(true),
        other => Err(de::Error::invalid_value(Unexpected::Unsigned(other as u64), &"zero or one")),
    }
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};
    use serde_json::json;

    #[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
    struct U64BooleanTest {
        #[serde(with = "super")]
        value: bool,
    }

    #[test]
    fn deserialize_false() {
        let json = json!({ "value": 0 });

        assert_eq!(
            serde_json::from_value::<U64BooleanTest>(json).unwrap(),
            U64BooleanTest { value: false },
        );
    }

    #[test]
    fn deserialize_true() {
        let json = json!({ "value": 1 });

        assert_eq!(
            serde_json::from_value::<U64BooleanTest>(json).unwrap(),
            U64BooleanTest { value: true },
        );
    }

    #[test]
    fn serialize_false() {
        let request = U64BooleanTest { value: false };
        assert_eq!(serde_json::to_value(&request).unwrap(), json!({ "value": 0 }));
    }

    #[test]
    fn serialize_true() {
        let request = U64BooleanTest { value: true };
        assert_eq!(serde_json::to_value(&request).unwrap(), json!({ "value": 1 }));
    }
}
