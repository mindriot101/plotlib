use plotlib::style::Point;

fn main() {
    let data = [
        (-3.0, 2.3),
        (-1.6, 5.3),
        (0.3, 0.7),
        (4.3, -1.4),
        (6.4, 4.3),
        (8.5, 3.7),
    ];
    let s1 = plotlib::scatter::Scatter::from_slice(&data).style(
        plotlib::scatter::Style::new()
            .marker(plotlib::style::Marker::Square)
            .colour("burlywood")
            .size(2.),
    );
    let s2 = plotlib::scatter::Scatter::from_slice(&[(-1.4, 2.5), (7.2, -0.3)])
        .style(plotlib::scatter::Style::new().colour("darkseagreen"));
    let v = plotlib::view::ContinuousView::new()
        .add(s1)
        .add(s2)
        .x_range(-5., 10.)
        .y_range(-2., 6.)
        .x_label("Some varying variable")
        .y_label("The response of something");
    plotlib::page::Page::single(v)
        .save("scatter.svg")
        .expect("saving svg");
}
