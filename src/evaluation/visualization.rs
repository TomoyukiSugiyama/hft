pub use crate::model::Model;
use chrono::Duration;
use chrono::prelude::*;
use plotters::prelude::*;

pub struct Visualization {}
impl Visualization {
    pub fn plot(model: &Model) -> Result<(), Box<dyn std::error::Error>> {
        const OUT_FILE_NAME: &str = "plot_data/stock.png";
        let start = Local
            .with_ymd_and_hms(2024, 1, 1, 0, 0, 0)
            .single()
            .ok_or("Invalid start date")?;

        let end = Local
            .with_ymd_and_hms(2024, 1, 1, 0, 59, 59)
            .single()
            .ok_or("Invalid end date")?;
        let hps = model
            .asset_manager
            .stock()
            .historical_prices()
            .filter_by(start, end);

        let (max, min) = hps.range();
        let hps_plot = hps.to_plot();

        let (from_date, to_date) = (
            hps_plot.first().unwrap().timestamp - Duration::minutes(1),
            hps_plot.last().unwrap().timestamp + Duration::minutes(1),
        );

        let root = BitMapBackend::new(OUT_FILE_NAME, (1024, 768)).into_drawing_area();
        root.fill(&WHITE)?;
        let mut chart = ChartBuilder::on(&root)
            .x_label_area_size(40)
            .y_label_area_size(40)
            .caption("SMPL Stock Price", ("sans-serif", 50.0).into_font())
            .build_cartesian_2d(from_date..to_date, min as f32..max as f32)?;

        chart.configure_mesh().light_line_style(WHITE).draw()?;

        chart.draw_series(hps_plot.iter().map(|hp| {
            CandleStick::new(
                hp.timestamp,
                hp.open,
                hp.high,
                hp.low,
                hp.close,
                GREEN.filled(),
                RED,
                3,
            )
        }))?;

        let iss = model
        .strategy_engine
        .trend_engine()
        .indicator()
        .calculate(&hps);

        chart.draw_series(LineSeries::new(
            iss.iter().map(|is| (is.timestamp, is.value as f32)),
            &BLUE,
        ))?;

        // To avoid the IO failure being ignored silently, we manually call the present function
        root.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
        println!("Result has been saved to {}", OUT_FILE_NAME);

        Ok(())
    }
}
