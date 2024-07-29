use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Boolean {
    value: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Date {
    value: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DateTime {
    value: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Decimal {
    value: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Float {
    value: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HexColorCode {
    value: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ID {
    value: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Int {
    value: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JSON {
    value: serde_json::Value,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct URL {
    value: String,
}
