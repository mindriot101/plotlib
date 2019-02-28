extern crate plotlib;

use plotlib::style::BarChart;

fn main() {
    let b1 = plotlib::barchart::BarChart::new(5.3).label("1");
    let b2 = plotlib::barchart::BarChart::new(2.6)
        .label("2")
        .style(plotlib::barchart::Style::new().fill("darkolivegreen"));
    let v = plotlib::view::CategoricalView::new()
        .add(Box::new(b1))
        .add(Box::new(b2))
        .x_label("Experiment");
    plotlib::page::Page::single(Box::new(v))
        .save("barchart.svg")
        .expect("saving svg");
}
