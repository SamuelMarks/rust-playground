#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate toml;

use std::fmt;
use std::marker::PhantomData;

use serde::de::{self, Deserializer, Visitor, MapAccess, SeqAccess};

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
    #[serde(deserialize_with = "script_deserializer")]
    #[serde(default)]
    script: Option<Script>,
    script_runner: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
enum Script {
    Inline(Vec<String>),
    Url(String),
}

// #[derive(Debug, Deserialize, Clone)]
// struct InlineScript(Vec<String>);

fn script_deserializer<'de, D>(deserializer: D) -> Result<Option<Script>, D::Error>
where
    D: Deserializer<'de>,
{
    struct ScriptDeserializer(PhantomData<fn() -> Option<Script>>);

    impl<'de> Visitor<'de> for ScriptDeserializer
    {
        type Value = Option<Script>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("string or map")
        }

        fn visit_str<E>(self, value: &str) -> Result<Option<Script>, E>
        where
            E: de::Error,
        {
            Ok(Some(Script::Inline(vec![value.to_string()])))
        }

        fn visit_seq<S>(self, mut visitor: S) -> Result<Option<Script>, S::Error>
        where
            S: SeqAccess<'de>,
        {
            

            // let mut lines = Vec::new();

            // // Update the max while there are additional values.
            // while let Some(value) = visitor.next_element()? {
            //     println!("{:?}", value);
            //     lines.push(value);
            // }

            // Ok(Some(Script::Inline(lines)))

            let secs = visitor.next_element()?
                .ok_or_else(|| de::Error::invalid_length(0, &self))?;
            //let _: Option<String> = visitor.next_key().unwrap(); 
            Ok(Some(Script::Inline(secs)))
        }

        fn visit_map<M>(self, mut visitor: M) -> Result<Option<Script>, M::Error>
        where
            M: MapAccess<'de>,
        {
            let _: Option<String> = visitor.next_key().unwrap(); 
            let url_string: Result<Option<String>, _> = visitor.next_value();
            match url_string {
                Ok(Some(string)) => Ok(Some(Script::Url(string))),
                _ => Ok(None)
            }    
        }
    }

    deserializer.deserialize_any(ScriptDeserializer(PhantomData))
}

