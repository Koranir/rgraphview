mod node;

mod edge;

mod graph;

pub fn strtocol(color: sapp_jsutils::JsObject) -> macroquad::color::Color {
    let mut string = String::new();
    color.to_string(&mut string);
    string.make_ascii_lowercase();
    match string.as_str().trim() {
        "red" => macroquad::color::RED,
        s if s.starts_with('#') => {
            macroquad::color::Color::from_hex(u32::from_str_radix(&s[1..], 16).unwrap())
        }
        _ => {
            let hex = color.field_u32("color");
            macroquad::color::Color::from_hex(hex)
        }
    }
}
