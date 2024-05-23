use csv::Reader;
use plotters::prelude::*;
use serde::Deserialize;
use std::error::Error;

#[derive(Debug, Deserialize)]
struct Location {
    x: i32,
    y: i32,
}

fn main() -> Result<(), Box<dyn Error>> {
    // Paths to the CSV files
    let zandvoort_csv_path = "zandvoort_coords.csv";
    let led_csv_path = "led_coords.csv";

    // Create readers for the CSV files
    let mut zandvoort_rdr = Reader::from_path(zandvoort_csv_path)?;
    let mut led_rdr = Reader::from_path(led_csv_path)?;

    // Load data from both datasets
    let zandvoort_locations: Vec<Location> = zandvoort_rdr.deserialize().collect::<Result<_, _>>()?;
    let led_locations: Vec<Location> = led_rdr.deserialize().collect::<Result<_, _>>()?;

    // Set up plot with adjusted Cartesian limits
    let root = BitMapBackend::new("combined_plot.png", (1024, 1024)).into_drawing_area();
    root.fill(&WHITE)?;

    // Adjust the limits based on the data range
    let x_min = zandvoort_locations.iter().map(|loc| loc.x).min().unwrap_or(-5000);
    let x_max = zandvoort_locations.iter().map(|loc| loc.x).max().unwrap_or(5000);
    let y_min = zandvoort_locations.iter().map(|loc| loc.y).min().unwrap_or(-5000);
    let y_max = zandvoort_locations.iter().map(|loc| loc.y).max().unwrap_or(5000);

    let mut chart = ChartBuilder::on(&root)
        .caption("Zandvoort Track with LED Positions - X and Y", ("sans-serif", 40))
        .margin(20)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(x_min..x_max, y_min..y_max)?;

    chart.configure_mesh().draw()?;

    // Plot zandvoort data points
    chart.draw_series(PointSeries::of_element(
        zandvoort_locations.iter().map(|loc| (loc.x, loc.y)),
        3, // Point size
        &RED,
        &|coord, size, style| {
            EmptyElement::at(coord) + Circle::new((0, 0), size, style.filled())
        },
    ))?;

    // Plot LED data points as larger green squares
    chart.draw_series(PointSeries::of_element(
        led_locations.iter().map(|loc| (loc.x, loc.y)),
        5, // Larger point size
        &GREEN,
        &|coord, size, style| {
            EmptyElement::at(coord) + Rectangle::new(
                [(coord.0 - size as i32 / 2, coord.1 - size as i32 / 2), (coord.0 + size as i32 / 2, coord.1 + size as i32 / 2)], 
                style.filled()
            )
        },
    ))?;

    // Save the plot
    root.present()?;
    println!("Plot is generated and saved as combined_plot.png.");

    Ok(())
}
