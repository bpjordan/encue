pub mod string_or_struct {
    use std::{convert::Infallible, marker::PhantomData, str::FromStr};

    use serde::{
        de::{self, Visitor},
        Deserialize, Deserializer,
    };

    pub fn deserialize<'de, T, D>(deserializer: D) -> Result<T, D::Error>
    where
        T: Deserialize<'de> + FromStr<Err = Infallible>,
        D: Deserializer<'de>,
    {
        struct StringOrStructVisitor<T>(PhantomData<fn() -> T>);

        impl<'de, T> Visitor<'de> for StringOrStructVisitor<T>
        where
            T: Deserialize<'de> + FromStr<Err = Infallible>,
        {
            type Value = T;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("string or map")
            }

            fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(FromStr::from_str(v).unwrap())
            }

            fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                Deserialize::deserialize(de::value::MapAccessDeserializer::new(map))
            }
        }

        deserializer.deserialize_any(StringOrStructVisitor(PhantomData))
    }
}
