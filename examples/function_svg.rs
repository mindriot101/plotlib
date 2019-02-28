extern crate plotlib;

use plotlib::style::Line;

fn main() {
    let f1 = plotlib::function::Function::new(|x| x * 5., 0., 10.)
        .style(plotlib::function::Style::new().colour("burlywood"));
    let f2 = plotlib::function::Function::new(|x| x.powi(2), 0., 10.).style(
        plotlib::function::Style::new()
            .colour("darkolivegreen")
            .width(2.),
    );
    let f3 = plotlib::function::Function::new(|x| x.sqrt() * 20., 0., 10.)
        .style(plotlib::function::Style::new().colour("brown").width(1.));
    let v = plotlib::view::ContinuousView::new()
        .add(Box::new(f1))
        .add(Box::new(f2))
        .add(Box::new(f3));
    plotlib::page::Page::single(Box::new(v))
        .save("function.svg")
        .expect("saving svg");
}
