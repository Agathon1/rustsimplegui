//! Status timeline - displays information against a time line like a 
//!                   horizontal bar chart.
//!

use crate::canvas;
use crate::chart::plotchart;
use crate::widget;
use crate::wish;

/// Refers to a status timeline
#[derive(Clone, Debug, PartialEq)]
pub struct TkStatusTimeline {
    pub id: String,
}

/// Creates a status timeline.
///
pub fn make_status_timeline(
    canvas: &canvas::TkCanvas,
    x_axis: (f64, f64, f64),
    y_labels: &[&str],
    show_x_axis: bool) -> TkStatusTimeline {
    let id = wish::next_var();
    let msg = format!(
        "global {}; set {} [::Plotchart::createStatusTimeline {} {{ {} {} {} }} {{{}}} -xaxis {}]",
        id, id, &canvas.id, x_axis.0, x_axis.1, x_axis.2, 
        widget::strings_list(&y_labels),
        if show_x_axis { "1" } else { "0" });
    wish::tell_wish(&msg);

    TkStatusTimeline { id }
}

impl plotchart::TkPlotchart for TkStatusTimeline {
    /// Returns the widget's id reference - used within tk
    fn id(&self) -> &str {
        &self.id
    }
}

impl TkStatusTimeline {
    /// Draws a vertical line at given time point.
    pub fn draw_line(&self, text: &str, time_point: f64, colour: &str, dash: plotchart::ChartDash, width: f64) {
        let msg = format!(
            "global {}; ${} vertline {{{}}} {} -fill {} -dash {} -width {}",
            &self.id, &self.id, text, time_point, colour, &dash.to_short_string(), width
        );
        wish::tell_wish(&msg);
    }

    /// Draws a bar between the start and stop times.
    pub fn plot(&self, series: &str, start: f64, stop: f64, colour: &str) {
        let msg = format!(
            "global {}; ${} plot {} {} {} {}",
            &self.id, &self.id, series, start, stop, colour
        );
        wish::tell_wish(&msg);
    }
}

