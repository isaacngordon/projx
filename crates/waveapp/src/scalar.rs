pub struct Boolean {
    value: bool,
}

pub struct Date {
    value: String,
}

pub struct DateTime {
    value: String,
}

pub struct Decimal {
    value: f64,
}

pub struct Float {
    value: f64,
}

pub struct HexColorCode {
    value: String,
}

pub struct ID {
    value: String,
}

pub struct Int {
    value: i32,
}

pub struct JSON {
    value: serde_json::Value,
}

pub struct URL {
    value: String,
}