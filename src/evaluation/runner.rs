use crate::evaluation::reporting::Reporting;
use crate::evaluation::visualization::Visualization;
pub use crate::model::Model;

pub struct Evaluation {}

impl Evaluation {
    pub fn new() -> Self {
        Self {}
    }
    pub fn backtest(&self, model: &Model) {
        if let Err(err) = Visualization::plot(model) {
            println!("{}", err);
        }
        Reporting::output(model);
    }
}
