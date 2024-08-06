pub struct Model { pub version: u32 }
pub fn load(_path: &str) -> std::io::Result<Model> { Ok(Model { version: 7 }) }
