//! Polar plot - plots points along a circular axis.
//!

use crate::chart::plotchart;
use crate::canvas;
use crate::widget;
use crate::wish;

/// Refers to a polar plot
#[derive(Clone, Debug, PartialEq)]
pub struct TkPolarPlot {
    pub id: String,
}

/// Creates an instance of a polar plot in given canvas.
pub fn make_polar(canvas: &canvas::TkCanvas, 
                  radius_data: (f64, f64)) -> TkPolarPlot {
    let id = wish::next_var();
    let msg = format!(
        "global {}; set {} [::Plotchart::createPolarplot {} {{{} {}}}]",
        id, id, &canvas.id,
        radius_data.0, radius_data.1
        );
    wish::tell_wish(&msg);

    TkPolarPlot { id }
}

impl plotchart::TkPlotchart for TkPolarPlot {
    /// Returns the widget's id reference - used within tk
    fn id(&self) -> &str {
        &self.id
    }
}

impl plotchart::TkChartSeries for TkPolarPlot {}
impl plotchart::TkChartDots for TkPolarPlot {}

impl TkPolarPlot {
    /// Draws a dot - value determines symbol/colour.
    pub fn draw_dot(&self, series: &str, (x, y): (f64, f64), value: f64) {
        let msg = format!(
            "global {}; ${} dot {} {} {} {}",
            &self.id, &self.id, series, x, y, value
        );
        wish::tell_wish(&msg);
    }

    /// Draws a dot with a text label.
    ///
    /// See [TkChartDots](plotchart::TkChartDots) for configuration options.
    pub fn draw_labelled_dot(
        &self,
        (x, y): (f64, f64),
        label: &str,
        location: plotchart::Location,
    ) {
        let msg = format!(
            "global {}; ${} labeldot {} {} {} {}",
            &self.id, &self.id, x, y, label, location
        );
        wish::tell_wish(&msg);
    }

    /// Draws a band on chart, parallel to x-axis so it appears as a ring.
    ///
    /// * `series` - name of data series
    /// * `x_coord` - x-coordinate
    /// * `y_min` - minimum y-value
    /// * `y_max` - maximum y-value
    pub fn draw_minmax(&self, series: &str, x_coord: f64, y_min: f64, y_max: f64) {
        let msg = format!(
            "global {}; ${} minmax {} {} {} {}",
            &self.id, &self.id, series, x_coord, y_min, y_max
        );
        wish::tell_wish(&msg);
    }

    /// Draws a closed polygon, using given points.
    pub fn draw_region(&self, series: &str, points: &[(f64, f64)]) {
        let xs: Vec<f64> = points.iter().map(|(x, _)| x).cloned().collect();
        let ys: Vec<f64> = points.iter().map(|(_, y)| y).cloned().collect();

        let msg = format!(
            "global {}; ${} region {} {{{}}} {{{}}}",
            &self.id,
            &self.id,
            series,
            &widget::str_list(&xs),
            &widget::str_list(&ys),
        );
        wish::tell_wish(&msg);
    }

    /// Plots a single point with respect to this axis.
    pub fn plot(&self, series: &str, (x, y): (f64, f64)) {
        let msg = format!("global {}; ${} plot {} {} {}", 
                          &self.id, &self.id, series, x, y);
        wish::tell_wish(&msg);
    }
}

