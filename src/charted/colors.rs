
use plotters::prelude::*;

const SLICE_COLORS: [plotters::style::RGBColor; 8] = [
    RGBColor(253, 224, 71),
    RGBColor(163, 230, 53),
    RGBColor(248, 113, 113),
    RGBColor(59, 130, 246),
    RGBColor(232, 121, 249),
    RGBColor(255, 153, 0),
    RGBColor(0, 204, 204),
    RGBColor(255, 102, 102),
];

pub fn random_color() -> RGBColor {
    let r = rand::random::<u8>();
    let g = rand::random::<u8>();
    let b = rand::random::<u8>();
    RGBColor(r, g, b)
}

pub fn get_slice_color(index: usize) -> RGBColor {
    if index < SLICE_COLORS.len() {
        SLICE_COLORS[index % SLICE_COLORS.len()]
    } else {
        random_color()
    }
}

// function generate random color array with length of n
pub fn graph_colors(n: usize) -> Vec<RGBColor> {
    let mut colors = Vec::new();
    for i in 0..n {
        colors.push(get_slice_color(i));
    }
    colors
}
