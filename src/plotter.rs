use crate::core::CollatzSequence;
use plotters::prelude::*;

pub struct Plotter {
    pub width: u32,
    pub height: u32,
}

impl Plotter {
    pub fn new() -> Self {
        return Plotter {
            width: 1280,
            height: 720,
        };
    }

    pub fn plot(&self, cs: &CollatzSequence) -> Result<(), Box<dyn std::error::Error>> {
        let chart_name = "Collatz Sequence";
        let file_name: &str = &format!("sequence_from_{}.png", cs.starting_number);
        let root = BitMapBackend::new(file_name, (self.width, self.height)).into_drawing_area();
        root.fill(&WHITE)?;
        let mut chart = ChartBuilder::on(&root)
            .caption(chart_name, ("sans-serif", 50).into_font())
            .margin(20)
            .x_label_area_size(30)
            .y_label_area_size(30)
            .build_cartesian_2d(
                0u128..cs.hailstone.len() as u128,
                1u128..*cs.hailstone.iter().max().unwrap(),
            )?;

        chart.configure_mesh().draw()?;

        chart
            .draw_series(LineSeries::new(
                cs.hailstone
                    .clone()
                    .iter()
                    .enumerate()
                    .map(|(index, &element)| (index as u128, element)),
                &RED,
            ))?
            .label(&format!("CollatzSequence({})", cs.starting_number))
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

        chart
            .configure_series_labels()
            .background_style(&WHITE.mix(0.8))
            .border_style(&BLACK)
            .draw()?;

        root.present()?;

        Ok(())
    }
}
