mod model;

#[tokio::main]
async fn main() {
    let m = model::load("model.bin").expect("model");
    println!("scoring-engine up, model v{}", m.version);
}
