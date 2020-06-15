
#[derive(RustcDecodable, RustcEncodable, Debug)]
pub struct Player {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}
