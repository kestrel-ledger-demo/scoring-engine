pub struct Model { pub version: u32, pub features: Vec<String> }
pub fn load(_path: &str) -> std::io::Result<Model> { Ok(Model { version: 8, features: vec!["sum_1m".into(), "sum_10m".into(), "sum_60m".into(), "new_merchants_1h".into()] }) }
