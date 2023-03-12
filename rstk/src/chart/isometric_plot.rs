//! Isometric plot - draws circles/rectangles to true dimensions.
//!

use crate::chart::plotchart;
use crate::canvas;
use crate::wish;

/// Refers to an isometric plot
#[derive(Clone, Debug, PartialEq)]
pub struct TkIsometricPlot {
    pub id: String,
}

/// Creates an instance of an isometric plot in given canvas.
pub fn make_isometric_plot(
    canvas: &canvas::TkCanvas,
    x_axis: (f64, f64),
    y_axis: (f64, f64),
    step_size: plotchart::StepSize
) -> TkIsometricPlot {
    let id = wish::next_var();
    let msg = format!(
        "global {}; set {} [::Plotchart::createIsometricPlot {} {{ {} {} }} {{ {} {} }} {}]",
        id, id, &canvas.id, x_axis.0, x_axis.1, y_axis.0, y_axis.1, step_size
    );
    wish::tell_wish(&msg);

    TkIsometricPlot { id }
}

impl plotchart::TkPlotchart for TkIsometricPlot {
    /// Returns the widget's id reference - used within tk
    fn id(&self) -> &str {
        &self.id
    }
}

impl TkIsometricPlot {
    /// Draws a circle
    pub fn circle(&self, (x, y): (f64, f64), radius: f64, colour: &str) {
        let msg = format!("global {}; ${} plot circle {} {} {} {}",
                          &self.id, &self.id, x, y, radius, colour);
        wish::tell_wish(&msg);
    }

    /// Draws a filled circle
    pub fn filled_circle(&self, (x, y): (f64, f64), radius: f64, colour: &str) {
        let msg = format!("global {}; ${} plot filled-circle {} {} {} {}",
                          &self.id, &self.id, x, y, radius, colour);
        wish::tell_wish(&msg);
    }

    /// Draws a filled rectangle
    pub fn filled_rectangle(&self, 
                            (x1, y1): (f64, f64), 
                            (x2, y2): (f64, f64), 
                            colour: &str) {
        let msg = format!("global {}; ${} plot filled-rectangle {} {} {} {} {}",
                          &self.id, &self.id, x1, y1, x2, y2, colour);
        wish::tell_wish(&msg);
    }

    /// Draws a rectangle
    pub fn rectangle(&self, 
                     (x1, y1): (f64, f64), 
                     (x2, y2): (f64, f64), 
                     colour: &str) {
        let msg = format!("global {}; ${} plot rectangle {} {} {} {} {}",
                          &self.id, &self.id, x1, y1, x2, y2, colour);
        wish::tell_wish(&msg);
    }
}
