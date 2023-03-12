//! Time chart - displays information against a time line.
//!
//! Rows are defined for two kinds of display: _milestones_ and _periods_.
//!
//! * A _milestone_ is fixed at a specific point of time.
//! * A _period_ identifies a range of time.
//!
//! Use the methods [milestone](TkTimeChart::milestone) and 
//! [period](TkTimeChart::period) to create a new labelled row 
//! in the chart of the appropriate kind, with one milestone or period 
//! indicated. Extra milestones or periods can be added to the current 
//! row using the [add_milestone](TkTimeChart::add_milestone) and 
//! [add_period](TkTimeChart::add_period) methods, as appropriate.

use crate::canvas;
use crate::chart::plotchart;
use crate::scrollbar;
use crate::wish;

/// Refers to a time chart
#[derive(Clone, Debug, PartialEq)]
pub struct TkTimeChart {
    pub id: String,
}

/// Intermediate definition for a time chart
#[derive(Clone, Debug, PartialEq)]
pub struct TkTimeChartDefinition {
    canvas_id: String,
    time_begin: String,
    time_end: String,
    num_items: Option<u64>,
    bar_height: Option<u64>,
    ylabel_width: Option<u64>
}

/// Methods to set options for time-chart - call 'plot' method at
/// end to finally create the chart.
impl TkTimeChartDefinition {
    /// Adds information on the required height of each bar in pixels
    /// (to specify spacing).
    pub fn bar_height(&mut self, value: u64) -> &mut Self {
        self.bar_height = Some(value);
        self
    }

    /// Adds information on the expected number of items (to specify spacing).
    pub fn num_items(&mut self, value: u64) -> &mut Self {
        self.num_items = Some(value);
        self
    }

    /// Adds information on the width of each label, in pixels.
    pub fn ylabel_width(&mut self, value: u64) -> &mut Self {
        self.ylabel_width = Some(value);
        self
    }
 
    /// Completes the definition of a time chart and creates the chart.
    pub fn plot(&self) -> TkTimeChart {
        let id = wish::next_var();
        let mut msg = format!(
            "global {}; set {} [::Plotchart::createTimechart {} {{{}}} {{{}}} ",
            id, id, &self.canvas_id, &self.time_begin, &self.time_end);

        if let Some(value) = &self.num_items {
            msg.push_str(&format!("{} ", value));
        }
        if let Some(value) = &self.bar_height {
            msg.push_str(&format!("-barheight {} ", value));
        }
        if let Some(value) = &self.ylabel_width {
            msg.push_str(&format!("-ylabelwidth {} ", value));
        }

        msg.push_str("]");
        wish::tell_wish(&msg);

        TkTimeChart { id }
    }
}

/// Creates a time chart.
///
/// Constructor creates an instance of a time chart definition in given canvas.
///
/// Options must be added and then 'plot' called to finally
/// create the chart.
pub fn make_time_chart(
    canvas: &canvas::TkCanvas,
    time_begin: &str,
    time_end: &str,
) -> TkTimeChartDefinition {
    TkTimeChartDefinition {
        canvas_id: String::from(&canvas.id),
        time_begin: String::from(time_begin),
        time_end: String::from(time_end),
        num_items: None,
        bar_height: None,
        ylabel_width: None
    }
}

impl plotchart::TkPlotchart for TkTimeChart {
    /// Returns the widget's id reference - used within tk
    fn id(&self) -> &str {
        &self.id
    }
}

impl TkTimeChart {
    /// Adds a new milestone to the current row of the chart.
    pub fn add_milestone(&self, time_point: &str, colour: &str) {
        let msg = format!("global {}; ${} addmilestone {{{}}} {}",
                          &self.id, &self.id, time_point, colour);
        wish::tell_wish(&msg);
    }

    /// Adds a new period to the current row of the chart.
    pub fn add_period(&self, (time_begin, time_end): (&str, &str), colour: &str) {
        let msg = format!("global {}; ${} addperiod {{{}}} {{{}}} {}",
                          &self.id, &self.id, time_begin, time_end, colour);
        wish::tell_wish(&msg);
    }

    /// Draws a vertical line on the chart.
    ///
    /// * `text` - the text to display at top of line
    /// * `time_point` - the time coordinate at which to draw the line
    /// * `colour` - colour for line
    pub fn draw_line(&self, text: &str, time_point: &str, colour: &str) {
        let msg = format!("global {}; ${} vertline {{{}}} {{{}}} {}",
                          &self.id, &self.id, text, time_point, colour);
        wish::tell_wish(&msg);
    }

    /// Adds a new row to the chart with the given milestone.
    pub fn milestone(&self, text: &str, time_point: &str, colour: &str) {
        let msg = format!("global {}; ${} milestone {{{}}} {{{}}} {}",
                          &self.id, &self.id, text, time_point, colour);
        wish::tell_wish(&msg);
    }

    /// Adds a new row to the chart with the given period.
    pub fn period(&self, text: &str, (time_begin, time_end): (&str, &str), colour: &str) {
        let msg = format!("global {}; ${} period {{{}}} {{{}}} {{{}}} {}",
                          &self.id, &self.id, text, time_begin, time_end, colour);
        wish::tell_wish(&msg);
    }

    /// Adds a horizontal scrollbar to the chart.
    pub fn scrollbar_horizontal(&self, scroll_bar: &scrollbar::TkScrollbar) {
        let msg = format!("global {}; ${} hscroll {}",
                          &self.id, &self.id, &scroll_bar.id);
        wish::tell_wish(&msg);
    }

    /// Adds a vertical scrollbar to the chart.
    pub fn scrollbar_vertical(&self, scroll_bar: &scrollbar::TkScrollbar) {
        let msg = format!("global {}; ${} vscroll {}",
                          &self.id, &self.id, &scroll_bar.id);
        wish::tell_wish(&msg);
    }
}

