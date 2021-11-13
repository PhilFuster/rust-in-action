use serde::{ Serialize };
use serde_json::to_string as to_json;
#[derive(Serialize)]
struct City {
  name: String,
  population: usize,
  latitude: f64,
  longitude: f64,
}

fn main() {
  let calabar = City {
    name: String::from("Calabar"),
    population: 470_000,
    latitude: 4.95,
    longitude: 8.33,
  };

  let as_json = to_json(&calabar).unwrap();
  println!("{}", &as_json);
}