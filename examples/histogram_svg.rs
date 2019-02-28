extern crate plotlib;

use plotlib::style::Bar;

fn main() {
    let data = [0.3, 0.5, 6.4, 5.3, 3.6, 3.6, 3.5, 7.5, 4.0];
    let h = plotlib::histogram::Histogram::from_slice(&data, plotlib::histogram::Bins::Count(10))
        .style(plotlib::histogram::Style::new().fill("burlywood"));
    let v = plotlib::view::ContinuousView::new().add(Box::new(h));
    plotlib::page::Page::single(Box::new(v))
        .save("histogram.svg")
        .expect("saving svg");
}
