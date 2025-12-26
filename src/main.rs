mod model;
mod capital;
mod asset;
mod strategy;
mod evaluation;

use model::Model;
fn main() {
    let model = Model::new();
    model.run();
}
