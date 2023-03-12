//! Ternary diagram - displays ratios of three quantities within an
//! equilateral triangle.
//!

use crate::canvas;
use crate::chart::plotchart;
use crate::wish;

/// Refers to a ternary diagram
#[derive(Clone, Debug, PartialEq)]
pub struct TkTernaryDiagram {
    pub id: String,
}

/// Creates an instance of a ternary diagram in given canvas.
///
/// * `fractions` - set to true to show values in range [0, 1] instead
///                 of as a percentage.
/// * `steps` - number of labels to show on each side
pub fn make_ternary_diagram(
    canvas: &canvas::TkCanvas,
    fractions: bool,
    steps: u64,
) -> TkTernaryDiagram {
    let id = wish::next_var();
    let msg = format!(
        "global {}; set {} [::Plotchart::createTernaryDiagram {} -fractions {} -steps {}]",
        id,
        id,
        &canvas.id,
        if fractions { "1" } else { "0" },
        steps
    );
    wish::tell_wish(&msg);

    TkTernaryDiagram { id }
}

impl plotchart::TkPlotchart for TkTernaryDiagram {
    /// Returns the widget's id reference - used within tk
    fn id(&self) -> &str {
        &self.id
    }
}

impl plotchart::TkChartSeries for TkTernaryDiagram {}

impl TkTernaryDiagram {
    /// Text labels for the three corners.
    pub fn corner_titles(&self, bottom_left: &str, bottom_right: &str, top_centre: &str) {
        let msg = format!("global {}; ${} text {{{}}} {{{}}} {{{}}}",
                          &self.id, &self.id, bottom_left, bottom_right, top_centre);
        wish::tell_wish(&msg);
    }

    /// Draws a closed polygon, using given points.
    pub fn draw_filled_polygon(&self, series: &str, points: &[(f64, f64, f64)]) {
        let mut points_str = String::new();
        for (x, y, z) in points {
            points_str.push_str(&format!("{} {} {} ", x, y, z));
        }

        let msg = format!("global {}; ${} fill {} {{{}}}",
                          &self.id, &self.id, series, &points_str);
        wish::tell_wish(&msg);
    }

    /// Draws a line, using given points.
    pub fn draw_line(&self, series: &str, points: &[(f64, f64, f64)]) {
        let mut points_str = String::new();
        for (x, y, z) in points {
            points_str.push_str(&format!("{} {} {} ", x, y, z));
        }

        let msg = format!("global {}; ${} line {} {{{}}}",
                          &self.id, &self.id, series, &points_str);
        wish::tell_wish(&msg);
    }

    /// Turns on ticklines, with given colour.
    pub fn draw_ticklines(&self, colour: &str) {
        let msg = format!("global {}; ${} ticklines {}",
                          &self.id, &self.id, colour);
        wish::tell_wish(&msg);
    }

    /// Plots a labelled point on the diagram.
    pub fn plot(&self, series: &str, (x, y, z): (f64, f64, f64), text: &str, 
                direction: plotchart::Direction) {
        let msg = format!("global {}; ${} plot {} {} {} {} {{{}}} {}",
                          &self.id, &self.id, series, x, y, z,
                          text, &direction.to_short_string());
        wish::tell_wish(&msg);
    }

    /// Set to true to use smooth corners.
    pub fn series_smooth(&self, series: &str, smooth: bool) {
        let msg = format!("global {}; ${} dataconfig {} -smooth {}",
                          &self.id, &self.id, series, 
                          if smooth { "1" } else { "0" });
        wish::tell_wish(&msg);
    }
}
