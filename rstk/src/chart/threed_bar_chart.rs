//! 3D bar chart - bar chart with 3D-like display and backdrop.
//!

use crate::chart::plotchart;
use crate::canvas;
use crate::font;
use crate::wish;

/// Refers to a 3D bar chart
#[derive(Clone, Debug, PartialEq)]
pub struct Tk3DBarChart {
    pub id: String,
}

/// Creates an instance of a 3D bar chart in given canvas.
pub fn make_3d_bar_chart(
    canvas: &canvas::TkCanvas,
    y_axis: (f64, f64, f64),
    num_series: u64
) -> Tk3DBarChart {
    let id = wish::next_var();
    let msg = format!(
        "global {}; set {} [::Plotchart::create3DBarchart {} {{ {} {} {} }} {}]",
        id, id, &canvas.id, y_axis.0, y_axis.1, y_axis.2, num_series
    );
    wish::tell_wish(&msg);

    Tk3DBarChart { id }
}

impl plotchart::TkPlotchart for Tk3DBarChart {
    /// Returns the widget's id reference - used within tk
    fn id(&self) -> &str {
        &self.id
    }
}

impl Tk3DBarChart  {
    /// Colour to use when showing labels.
    pub fn label_colour(&self, colour: &str) {
        let msg = format!("global {}; ${} config -labelcolour {}",
                          &self.id, &self.id, colour);
        wish::tell_wish(&msg);
    } 

    /// Font to use when showing labels.
    pub fn label_font(&self, font: &font::TkFont) {
        let msg = format!("global {}; ${} config -labelfont {{{}}}",
                          &self.id, &self.id, font);
        wish::tell_wish(&msg);
    }

    /// Plot given data.
    pub fn plot(&self, label: &str, y_value: f64, colour: &str) {
        let msg = format!("global {}; ${} plot {{{}}} {} {}",
                          &self.id, &self.id, 
                          label, y_value, colour);
        wish::tell_wish(&msg);
    }

    /// Set to true to show values on top of bar.
    pub fn show_values(&self, value: bool) {
        let msg = format!("global {}; ${} config -showvalues {}",
                          &self.id, &self.id,
                          if value { "1" } else { "0" });
        wish::tell_wish(&msg);
    }

    /// Set to true to show left/back walls.
    pub fn show_background(&self, value: bool) {
        let msg = format!("global {}; ${} config -usebackground {}",
                          &self.id, &self.id,
                          if value { "1" } else { "0" });
        wish::tell_wish(&msg);
    }
        /// Set to true to show ticklines.
    pub fn show_ticklines(&self, value: bool) {
        let msg = format!("global {}; ${} config -useticklines {}",
                          &self.id, &self.id,
                          if value { "1" } else { "0" });
        wish::tell_wish(&msg);
    }
    
    /// Colour to use when showing values.
    pub fn value_colour(&self, colour: &str) {
        let msg = format!("global {}; ${} config -valuecolour {}",
                          &self.id, &self.id, colour);
        wish::tell_wish(&msg);
    } 

    /// Font to use when showing values.
    pub fn value_font(&self, font: &font::TkFont) {
        let msg = format!("global {}; ${} config -valuefont {{{}}}",
                          &self.id, &self.id, font);
        wish::tell_wish(&msg);
    }
}

