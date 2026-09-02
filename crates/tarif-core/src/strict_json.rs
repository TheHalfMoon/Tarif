use crate::ActionError;
use serde::de::{self, DeserializeSeed, MapAccess, SeqAccess, Visitor};
use serde_json::{Map, Number, Value};
use std::cell::RefCell;
use std::fmt;

#[derive(Clone, Copy)]
struct StrictSeed<'a> {
    duplicate: &'a RefCell<Option<String>>,
}

struct StrictVisitor<'a> {
    duplicate: &'a RefCell<Option<String>>,
}

impl<'de> DeserializeSeed<'de> for StrictSeed<'_> {
    type Value = Value;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_any(StrictVisitor {
            duplicate: self.duplicate,
        })
    }
}

impl<'de> Visitor<'de> for StrictVisitor<'_> {
    type Value = Value;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("strict RFC 8259 JSON within Tarif's JCS input profile")
    }

    fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E> {
        Ok(Value::Bool(value))
    }

    fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        number_from_f64(value as f64)
    }

    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        number_from_f64(value as f64)
    }

    fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        number_from_f64(value)
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Value::String(value.to_owned()))
    }

    fn visit_borrowed_str<E>(self, value: &'de str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Value::String(value.to_owned()))
    }

    fn visit_string<E>(self, value: String) -> Result<Self::Value, E> {
        Ok(Value::String(value))
    }

    fn visit_none<E>(self) -> Result<Self::Value, E> {
        Ok(Value::Null)
    }

    fn visit_unit<E>(self) -> Result<Self::Value, E> {
        Ok(Value::Null)
    }

    fn visit_seq<A>(self, mut sequence: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut values = Vec::new();
        while let Some(value) = sequence.next_element_seed(StrictSeed {
            duplicate: self.duplicate,
        })? {
            values.push(value);
        }
        Ok(Value::Array(values))
    }

    fn visit_map<A>(self, mut access: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut object = Map::new();
        while let Some(key) = access.next_key::<String>()? {
            if object.contains_key(&key) {
                *self.duplicate.borrow_mut() = Some(key.clone());
                return Err(de::Error::custom("duplicate JSON object key"));
            }
            let value = access.next_value_seed(StrictSeed {
                duplicate: self.duplicate,
            })?;
            object.insert(key, value);
        }
        Ok(Value::Object(object))
    }
}

fn number_from_f64<E>(value: f64) -> Result<Value, E>
where
    E: de::Error,
{
    Number::from_f64(value)
        .map(Value::Number)
        .ok_or_else(|| E::custom("JSON number is not finite IEEE-754"))
}

pub(crate) fn parse_strict_json(input: &str) -> Result<Value, ActionError> {
    let duplicate = RefCell::new(None);
    let mut deserializer = serde_json::Deserializer::from_str(input);
    let result = StrictSeed {
        duplicate: &duplicate,
    }
    .deserialize(&mut deserializer);

    let value = match result {
        Ok(value) => value,
        Err(error) => {
            if let Some(key) = duplicate.into_inner() {
                return Err(ActionError::DuplicateJsonKey(key));
            }
            return Err(ActionError::InvalidJson(error.to_string()));
        }
    };

    deserializer
        .end()
        .map_err(|error| ActionError::InvalidJson(error.to_string()))?;
    Ok(value)
}
