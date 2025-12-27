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
    pub fn run(&self, model: &Model){
        model.run();
        Visualization::plot();
        Reporting::ourput();
        println!("reporting");
    }

}
