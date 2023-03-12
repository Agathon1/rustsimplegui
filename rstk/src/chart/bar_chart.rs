//! Bar chart - choice of display with vertical or horizontal bars.
//!

use crate::chart::plotchart;
use crate::canvas;
use crate::font;
use crate::wish;

/// Refers to a bar chart
#[derive(Clone, Debug, PartialEq)]
pub struct TkBarChart {
    pub id: String,
}

/// Refers to a bar chart plotted with horizontal bars
#[derive(Clone, Debug, PartialEq)]
pub struct TkHorizontalBarChart {
    pub id: String,
}

/// Creates an instance of a bar chart in given canvas.
pub fn make_bar_chart(
    canvas: &canvas::TkCanvas,
    x_labels: &[&str],
    y_axis: (f64, f64, f64),
    num_series: plotchart::BarSeries,
    x_label_angle: f64
) -> TkBarChart {
    let mut labels_str = String::new();
    for label in x_labels {
        labels_str.push('{');
        labels_str.push_str(label);
        labels_str.push('}');
        labels_str.push(' ');
    }
    let id = wish::next_var();
    let msg = format!(
        "global {}; set {} [::Plotchart::createBarchart {} {{{}}} {{ {} {} {} }} {}  -xlabelangle {}]",
        id, id, &canvas.id, labels_str, y_axis.0, y_axis.1, y_axis.2, num_series, x_label_angle
    );
    wish::tell_wish(&msg);

    TkBarChart { id }
}

/// Creates an instance of a horizontal bar chart in given canvas.
pub fn make_horizontal_bar_chart(
    canvas: &canvas::TkCanvas,
    x_axis: (f64, f64, f64),
    y_labels: &[&str],
    num_series: plotchart::BarSeries,
) -> TkHorizontalBarChart {
    let mut labels_str = String::new();
    for label in y_labels {
        labels_str.push('{');
        labels_str.push_str(label);
        labels_str.push('}');
        labels_str.push(' ');
    }

    let id = wish::next_var();
    let msg = format!(
        "global {}; set {} [::Plotchart::createHorizontalBarchart {} {{ {} {} {} }} {{{}}} {}]",
        id, id, &canvas.id, x_axis.0, x_axis.1, x_axis.2, labels_str, num_series
        );
    wish::tell_wish(&msg);

    TkHorizontalBarChart { id }
}

impl plotchart::TkPlotchart for TkBarChart {
    /// Returns the widget's id reference - used within tk
    fn id(&self) -> &str {
        &self.id
    }
}

impl plotchart::TkPlotchart for TkHorizontalBarChart {
    /// Returns the widget's id reference - used within tk
    fn id(&self) -> &str {
        &self.id
    }
}

pub trait BarChartMethods : plotchart::TkPlotchart {
    /// Plot given data.
    fn plot(&self, series: &str, data: &[f64], colour: &str) {
        let mut data_str = String::new();
        for datum in data {
            data_str.push_str(&format!("{} ", datum));
        }

        let msg = format!("global {}; ${} plot {} {{{}}} {}",
                          self.id(), self.id(), series,
                          data_str, colour);
        wish::tell_wish(&msg);
    }

    /// Plot given data with a colour gradient.
    fn plot_gradient(&self, series: &str, data: &[f64], colour: &str, 
                     direction: plotchart::GradientDirection, 
                     brightness: plotchart::Brightness) {
        let mut data_str = String::new();
        for datum in data {
            data_str.push_str(&format!("{} ", datum));
        }

        let msg = format!("global {}; ${} plot {} {{{}}} {} {} {}",
                          self.id(), self.id(), series,
                          data_str, colour, direction, brightness);
        wish::tell_wish(&msg);
    }

    /// Set to true to show values on top of bar.
    fn show_values(&self, value: bool) {
        let msg = format!("global {}; ${} config -showvalues {}",
                          self.id(), self.id(),
                          if value { "1" } else { "0" });
        wish::tell_wish(&msg);
    }
    
    /// Colour to use when showing values.
    fn value_colour(&self, colour: &str) {
        let msg = format!("global {}; ${} config -valuecolour {}",
                          self.id(), self.id(), colour);
        wish::tell_wish(&msg);
    } 

    /// Font to use when showing values.
    fn value_font(&self, font: &font::TkFont) {
        let msg = format!("global {}; ${} config -valuefont {{{}}}",
                          self.id(), self.id(), font);
        wish::tell_wish(&msg);
    }

    /// Format to use when showing values, in Tk format:
    ///
    /// * see Tk [manual](https://www.tcl.tk/man/tcl8.5/TclCmd/format.htm)
    fn value_format(&self, format: &str) {
        let msg = format!("global {}; ${} config -valueformat {{{}}}",
                          self.id(), self.id(), format);
        wish::tell_wish(&msg);
    } 
}

impl BarChartMethods for TkBarChart {}
 
impl BarChartMethods for TkHorizontalBarChart {}

