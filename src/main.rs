#[macro_use]
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate void;

extern crate toml;

use std::fmt;
use std::marker::PhantomData;
use std::str::FromStr;
use core::num::ParseIntError;

use void::Void;
use serde::de::{self, Deserialize, Deserializer, Visitor, MapAccess};

fn main() {
    let build_string = r#"
    script = [[
        "echo SIMPLE",
        "echo SIMPLE"
    ]]
    "#;
    let task: Task = toml::from_str(build_string).unwrap();

    println!("{:?}", task);

    // let build_string = r#"
    // script = "echo SIMPLE echo SIMPLE"
    // "#;
    // let task: Task = toml::from_str(build_string).unwrap();

    // println!("{:?}", task);

    let build_struct = "
        script = { url = 'wdeewde'}
        script_runner = 'ewfwf'
    ";
    let task: Task = toml::from_str(build_struct).unwrap();

    println!("{:?}", task);

    let build_struct = "
        script_runner = 'ewfwf'
    ";
    let task: Task = toml::from_str(build_struct).unwrap();

    println!("{:?}", task);
}

#[derive(Debug, Deserialize, Clone)]
struct Task {
    #[serde(deserialize_with = "string_or_struct")]
    #[serde(default)]
    script: Option<Script>,
    script_runner: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
enum Script {
    Inline(Vec<String>),
    Url(String),
}

impl FromStr for Script {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Script::Inline(vec![String::from(s)]))
    }
}


fn string_or_struct<'de, T, D>(deserializer: D) -> Result<Option<Script>, D::Error>
where
    T: Deserialize<'de> + FromStr<Err = Void>,
    D: Deserializer<'de>,
{
    // This is a Visitor that forwards string types to T's `FromStr` impl and
    // forwards map types to T's `Deserialize` impl. The `PhantomData` is to
    // keep the compiler from complaining about T being an unused generic type
    // parameter. We need T in order to know the Value type for the Visitor
    // impl.
    struct StringOrStruct<T>(PhantomData<fn() -> T>);

    impl<'de, T> Visitor<'de> for StringOrStruct<T>
    where
        T: Deserialize<'de> + FromStr<Err = Void>,
    {
        type Value = T;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("string or map")
        }

        fn visit_str<E>(self, value: &str) -> Result<T, E>
        where
            E: de::Error,
        {
            Ok(FromStr::from_str(value).unwrap())
        }

        fn visit_map<M>(self, visitor: M) -> Result<T, M::Error>
        where
            M: MapAccess<'de>,
        {
            // `MapAccessDeserializer` is a wrapper that turns a `MapAccess`
            // into a `Deserializer`, allowing it to be used as the input to T's
            // `Deserialize` implementation. T then deserializes itself using
            // the entries from the map visitor.
            Deserialize::deserialize(de::value::MapAccessDeserializer::new(visitor))
        }
    }

    deserializer.deserialize_any(StringOrStruct(PhantomData))
}