//! Box plot - choice of display with vertical or horizontal bars.
//!

use crate::chart::plotchart;
use crate::canvas;
use crate::wish;

/// Refers to a box plot
#[derive(Clone, Debug, PartialEq)]
pub struct TkBoxPlot {
    pub id: String,
}

/// Refers to a box plot plotted with horizontal bars
#[derive(Clone, Debug, PartialEq)]
pub struct TkHorizontalBoxPlot {
    pub id: String,
}

/// Creates an instance of a box plot in given canvas.
pub fn make_box_plot(
    canvas: &canvas::TkCanvas,
    x_labels: &[&str],
    y_axis: (f64, f64, f64)
) -> TkBoxPlot {
    let mut labels_str = String::new();
    for label in x_labels {
        labels_str.push('{');
        labels_str.push_str(label);
        labels_str.push('}');
        labels_str.push(' ');
    }
    let id = wish::next_var();
    let msg = format!(
        "global {}; set {} [::Plotchart::createBoxplot {} {{{}}} {{ {} {} {} }} vertical]",
        id, id, &canvas.id, labels_str, y_axis.0, y_axis.1, y_axis.2
    );
    wish::tell_wish(&msg);

    TkBoxPlot { id }
}

/// Creates an instance of a horizontal box plot in given canvas.
pub fn make_horizontal_box_plot(
    canvas: &canvas::TkCanvas,
    x_axis: (f64, f64, f64),
    y_labels: &[&str],
) -> TkHorizontalBoxPlot {
    let mut labels_str = String::new();
    for label in y_labels {
        labels_str.push('{');
        labels_str.push_str(label);
        labels_str.push('}');
        labels_str.push(' ');
    }

    let id = wish::next_var();
    let msg = format!(
        "global {}; set {} [::Plotchart::createBoxplot {} {{ {} {} {} }} {{{}}} horizontal]",
        id, id, &canvas.id, x_axis.0, x_axis.1, x_axis.2, labels_str
        );
    wish::tell_wish(&msg);

    TkHorizontalBoxPlot { id }
}

impl plotchart::TkPlotchart for TkBoxPlot {
    /// Returns the widget's id reference - used within tk
    fn id(&self) -> &str {
        &self.id
    }
}

impl plotchart::TkPlotchart for TkHorizontalBoxPlot {
    /// Returns the widget's id reference - used within tk
    fn id(&self) -> &str {
        &self.id
    }
}

pub trait BoxPlotMethods : plotchart::TkPlotchart {
    /// Sets width (or height) in pixels of box.
    fn box_width(&self, series: &str, value: u64) {
        let msg = format!("global {}; ${} dataconfig {} -boxwidth {}",
                          self.id(), self.id(), series, value);
        wish::tell_wish(&msg);
    }

    /// Sets colour of median line.
    fn median_colour(&self, series: &str, colour: &str) {
        let msg = format!("global {}; ${} dataconfig {} -mediancolour {}",
                          self.id(), self.id(), series, colour);
        wish::tell_wish(&msg);
    }

    /// Sets thickness in pixels of median line.
    fn median_width(&self, series: &str, value: u64) {
        let msg = format!("global {}; ${} dataconfig {} -medianwidth {}",
                          self.id(), self.id(), series, value);
        wish::tell_wish(&msg);
    }

    /// Plot given data.
    fn plot(&self, series: &str, label: &str, data: &[f64]) {
        let mut data_str = String::new();
        for datum in data {
            data_str.push_str(&format!("{} ", datum));
        }

        let msg = format!("global {}; ${} plot {} {{{}}} {{{}}}",
                          self.id(), self.id(), series, label, data_str);
        wish::tell_wish(&msg);
    }

    /// Sets how to draw whiskers.
    fn whiskers(&self, series: &str, value: plotchart::BoxWhiskers) {
        let msg = format!("global {}; ${} dataconfig {} -whiskers {}",
                          self.id(), self.id(), series, value);
        wish::tell_wish(&msg);
    }

    /// Sets width (or height) of box whiskers.
    fn whisker_width(&self, series: &str, value: u64) {
        let msg = format!("global {}; ${} dataconfig {} -whiskerwidth {}",
                          self.id(), self.id(), series, value);
        wish::tell_wish(&msg);
    }
}

impl BoxPlotMethods for TkBoxPlot {}
impl BoxPlotMethods for TkHorizontalBoxPlot {}
