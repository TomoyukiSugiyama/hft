use crate::evaluation::visualization::Visualization;
use crate::evaluation::reporting::Reporting;
pub use crate::model::Model;

pub struct Evaluation {
}

impl Evaluation {
    pub fn new() -> Self{
        Self {
        }
    }
    pub fn backtest(&self, model: &Model){
        model.run();
        if let Err(err) = Visualization::plot(model) {
            println!("{}",err);
        }

        Reporting::ourput();
        println!("reporting");
    }

}
