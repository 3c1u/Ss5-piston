use serde::de::{self, Deserialize, Deserializer, Visitor};

#[derive(Copy, Clone, Debug)]
pub enum BlendType {
    Mix,
    Multiple,
    Additive,
    Subtractive,
}

impl<'de> Deserialize<'de> for BlendType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use std::fmt;

        struct BlendTypeVisitor;

        impl<'de> Visitor<'de> for BlendTypeVisitor {
            type Value = BlendType;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("should be vaild blend type.")
            }

            fn visit_i8<E>(self, value: i8) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                self.visit_i32(i32::from(value))
            }

            fn visit_i32<E>(self, value: i32) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                match value {
                    0 => Ok(BlendType::Mix),
                    1 => Ok(BlendType::Multiple),
                    2 => Ok(BlendType::Additive),
                    3 => Ok(BlendType::Subtractive),
                    _ => Err(E::custom("Invalid blend type")),
                }
            }

            fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                use std::i32;
                if value >= i64::from(i32::MIN) && value <= i64::from(i32::MAX) {
                    self.visit_i32(value as i32)
                } else {
                    Err(E::custom(format!("i32 out of range: {}", value)))
                }
            }

            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                self.visit_i32(value as i32)
            }
        }

        deserializer.deserialize_i32(BlendTypeVisitor)
    }
}
