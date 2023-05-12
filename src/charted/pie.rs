use crate::charted::colors::*;
use plotters::{prelude::*, style::full_palette::ORANGE};
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;

#[derive(Deserialize)]
struct PieData {
    title: String,
    width: u32,
    height: u32,
    values: HashMap<String, f64>,
}

pub fn generate(input_file: String, output_path: String) -> Result<(), Box<dyn std::error::Error>> {
    let json_data = fs::read_to_string(input_file).expect("Unable to read file");
    let pie_data: PieData = serde_json::from_str(&json_data).expect("Unable to parse JSON");

    let pie_values: Vec<(&str, f64)> = pie_data
        .values
        .iter()
        .map(|(k, v)| (k.as_str(), *v))
        .collect();

    println!("Pie values: {:?}", pie_values);

    let root_area =
        SVGBackend::new(&output_path, (pie_data.width, pie_data.height)).into_drawing_area();

    // root_area.fill(&WHITE).unwrap();
    if pie_data.title != "" {
        let title_style = TextStyle::from(("sans-serif", 30).into_font()).color(&(BLACK));
        root_area.titled(&pie_data.title, title_style).unwrap();
    }

    let dims = root_area.dim_in_pixel();
    let center = (dims.0 as i32 / 2, dims.1 as i32 / 2);
    let radius = dims.0.min(dims.1) as f64 * 0.4;
    let sizes = pie_values.iter().map(|(_, v)| *v).collect::<Vec<f64>>();

    let colors = graph_colors(pie_values.len());
    let labels = pie_values
        .iter()
        .map(|(k, _)| k.to_string())
        .collect::<Vec<String>>();

    let mut pie = Pie::new(&center, &radius, &sizes, &colors, &labels);
    pie.start_angle(66.0);
    pie.label_style((("sans-serif", 32).into_font()).color(&(ORANGE)));
    pie.percentages((("sans-serif", radius * 0.08).into_font()).color(&BLACK));
    root_area.draw(&pie)?;

    println!("Pie chart generated to {}", output_path);

    Ok(())
}
