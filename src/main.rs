#[macro_use]
extern crate serde_derive;
extern crate toml;

use toml::Value;

#[derive(Debug,Deserialize)]
struct Foo {
    foo: String,
    can: Map,
}

#[derive(Debug,Deserialize)]
struct Can {
    can: Map,
}

fn main() {
  let input = "foo = {can ='bar'}";
  let value = input.parse::<Value>().unwrap();

  //assert_eq!(value["foo"].as_str(), Some("bar"));
  println!("Input:\n\n{}", input);
  println!("\nOutput:\n\n{}", value);

  let foo: Foo = toml::from_str(input).unwrap();
  println!("foo = {:?}", foo);
}
