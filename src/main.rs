mod asset;
mod capital;
mod evaluation;
mod indicator;
mod model;
mod strategy;
mod trend;

use evaluation::Evaluation;
use model::Model;

fn main() {
    let model = Model::new();
    let eval = Evaluation::new();
    eval.backtest(&model);
}
