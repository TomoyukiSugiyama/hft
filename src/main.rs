mod asset;
mod capital;
mod evaluation;
mod indicator;
mod strategy;
mod trend;

fn main() {
    if let Err(err) = evaluation::run_backtest() {
        println!("{}", err);
    }
}
