
use plotters::prelude::*;
use chrono::Duration;
pub use crate::model::Model;

pub struct Visualization {

} 
impl Visualization {
    pub fn plot(model: &Model) -> Result<(), Box<dyn std::error::Error>>{
        const OUT_FILE_NAME: &str = "plot_data/stock.png";
        let data = model.asset_manager.plot_data();
        let root = BitMapBackend::new(OUT_FILE_NAME, (1024, 768)).into_drawing_area();
        root.fill(&WHITE)?;


        let (from_date, to_date) = (
            data.first().unwrap().0 - Duration::minutes(1),
            data.last().unwrap().0 + Duration::minutes(1),
        );
    
        let mut chart = ChartBuilder::on(&root)
            .x_label_area_size(40)
            .y_label_area_size(40)
            .caption("MSFT Stock Price", ("sans-serif", 50.0).into_font())
            .build_cartesian_2d(from_date..to_date, 4f32..6f32)?;
    
        chart.configure_mesh().light_line_style(WHITE).draw()?;

        chart.draw_series(
            data.iter().map(|x| {
                CandleStick::new(x.0, x.1, x.2, x.3, x.4, GREEN.filled(), RED, 15)
            }),
        )?;
    
        // To avoid the IO failure being ignored silently, we manually call the present function
        root.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
        println!("Result has been saved to {}", OUT_FILE_NAME);

        Ok(())
    }
}