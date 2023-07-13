use plotters::prelude::*;
use plotters::prelude::BitMapBackend;


pub(crate) fn plot_price_history(price_history: Vec<f64>, name: &str) {
    let file_path = format!("{}.png", name);

    let root = BitMapBackend::new(&file_path, (1000, 820)).into_drawing_area();
    root.fill(&WHITE).unwrap();

    let min_price = price_history.iter().cloned().fold(f64::INFINITY, f64::min);
    let max_price = price_history.iter().cloned().fold(f64::NEG_INFINITY, f64::max);

    let mut chart = ChartBuilder::on(&root)
        .margin(5)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(0..price_history.len() as u32, min_price..max_price)
        .unwrap();

    chart.configure_mesh().draw().unwrap();

    let mut prev_price = price_history[0];
    for (x, &price) in price_history.iter().enumerate() {
        if price > prev_price {
            chart
                .draw_series(LineSeries::new(
                    vec![(x as u32, prev_price), ((x + 1) as u32, price)],
                    &GREEN,
                ))
                .unwrap();
        } else if price < prev_price {
            chart
                .draw_series(LineSeries::new(
                    vec![(x as u32, prev_price), ((x + 1) as u32, price)],
                    &RED,
                ))
                .unwrap();
        }

        prev_price = price;
    }

    // Draw horizontal lines at the first and last values
    let first_value = price_history.first().cloned().unwrap();
    let last_value = price_history.last().cloned().unwrap();

    chart
        .draw_series(LineSeries::new(
            vec![(0, first_value), (price_history.len() as u32 - 1, first_value)],
            &BLUE, // Color for the first horizontal line
        ))
        .unwrap();

    chart
        .draw_series(LineSeries::new(
            vec![(0, last_value), (price_history.len() as u32 - 1, last_value)],
            &MAGENTA, // Color for the second horizontal line
        ))
        .unwrap();

    root.present().unwrap();
}



