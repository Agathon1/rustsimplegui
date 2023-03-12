//! Right axis - adds a right-hand axis to an xy-style chart.
//!

use crate::chart::plotchart;
use crate::canvas;
use crate::wish;

/// Refers to a right axis
#[derive(Clone, Debug, PartialEq)]
pub struct TkRightAxis {
    pub id: String,
}

/// Creates an instance of a right axis in given canvas.
///
/// You _must_ have created an xy-style chart first.
pub fn make_right_axis(
    canvas: &canvas::TkCanvas,
    y_axis: (f64, f64, f64)
) -> TkRightAxis {
    let id = wish::next_var();
    let msg = format!(
        "global {}; set {} [::Plotchart::createRightAxis {} {{ {} {} {} }}]",
        id, id, &canvas.id, y_axis.0, y_axis.1, y_axis.2 
    );
    wish::tell_wish(&msg);

    TkRightAxis { id }
}

impl plotchart::TkPlotchart for TkRightAxis {
    /// Returns the widget's id reference - used within tk
    fn id(&self) -> &str {
        &self.id
    }
}

impl plotchart::TkChartSeries for TkRightAxis {}

impl TkRightAxis {
    /// Plots a single point with respect to this axis.
    pub fn plot(&self, series: &str, (x, y): (f64, f64)) {
        let msg = format!("global {}; ${} plot {} {} {}", 
                          &self.id, &self.id, series, x, y);
        wish::tell_wish(&msg);
    }
}
