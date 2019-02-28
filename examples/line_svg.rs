extern crate plotlib;

use plotlib::style::Line;

fn main() {
    let l1 = plotlib::line::Line::new(&[(0., 1.), (2., 1.5), (3., 1.2), (4., 1.1)])
        .style(plotlib::line::Style::new().colour("burlywood"));
    let v = plotlib::view::ContinuousView::new().add(Box::new(l1));
    plotlib::page::Page::single(Box::new(v))
        .save("line.svg")
        .expect("saving svg");
}
