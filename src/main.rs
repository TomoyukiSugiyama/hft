mod model;
mod capital;
mod asset;
mod strategy;
mod evaluation;
mod trend;
mod indicator;

use model::Model;
use evaluation::Evaluation;

fn main() {
    let model = Model::new();
    let eval = Evaluation::new();
    eval.backtest(&model);
}
