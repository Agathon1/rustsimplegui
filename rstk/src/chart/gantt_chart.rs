//! Gantt chart - displays information against a time line.
//!
//! Like the [time chart](crate::chart::time_chart), rows are defined for two 
//! kinds of display:
//! _milestones_ and _periods_.
//!
//! * A _milestone_ is fixed at a specific point of time.
//! * A _period_ identifies a range of time.
//!
//! The Gantt chart also has _tasks_. Each task displays a completed proportion,
//! and can be connected to other tasks.
//!

use crate::canvas;
use crate::chart::plotchart;
use crate::font;
use crate::scrollbar;
use crate::wish;

/// Refers to a gantt chart
#[derive(Clone, Debug, PartialEq)]
pub struct TkGanttChart {
    pub id: String,
}

/// Intermediate definition for a gantt chart
#[derive(Clone, Debug, PartialEq)]
pub struct TkGanttChartDefinition {
    canvas_id: String,
    time_begin: String,
    time_end: String,
    num_items: Option<u64>,
    bar_height: Option<u64>,
    ylabel_width: Option<u64>,
    max_width: Option<u64>
}

/// Refers to a task on a gantt chart
#[derive(Clone, Debug, PartialEq)]
pub struct TkGanttTask {
    pub id: String,
}

/// Methods to set options for gantt-chart - call 'plot' method at
/// end to finally create the chart.
impl TkGanttChartDefinition {
    /// Adds information on the required height of each bar in pixels
    /// (to specify spacing).
    pub fn bar_height(&mut self, value: u64) -> &mut Self {
        self.bar_height = Some(value);
        self
    }

    /// Adds information on the expected width of descriptive text
    /// (to specify spacing). Value defaults to 20 characters.
    pub fn max_width(&mut self, value: u64) -> &mut Self {
        self.max_width = Some(value);
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
    pub fn plot(&self) -> TkGanttChart {
        let id = wish::next_var();
        let mut msg = format!(
            "global {}; set {} [::Plotchart::createGanttchart {} {{{}}} {{{}}} ",
            id, id, &self.canvas_id, &self.time_begin, &self.time_end);

        if let Some(value) = &self.num_items {
            msg.push_str(&format!("{} ", value));
        }
        if let Some(value) = &self.max_width {
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

        TkGanttChart { id }
    }
}

/// Creates a Gantt chart.
///
/// Constructor creates an instance of a gantt chart definition in given 
/// canvas.
///
/// Options must be added and then 'plot' called to finally
/// create the chart.
pub fn make_gantt_chart(
    canvas: &canvas::TkCanvas,
    time_begin: &str,
    time_end: &str,
) -> TkGanttChartDefinition {
    TkGanttChartDefinition {
        canvas_id: String::from(&canvas.id),
        time_begin: String::from(time_begin),
        time_end: String::from(time_end),
        num_items: None,
        bar_height: None,
        ylabel_width: None,
        max_width: None
    }
}

impl plotchart::TkPlotchart for TkGanttChart {
    /// Returns the widget's id reference - used within tk
    fn id(&self) -> &str {
        &self.id
    }
}

impl TkGanttChart {
    /// Sets colour of description.
    pub fn description_colour(&self, colour: &str) {
        let msg = format!("global {}; ${} colour description {}",
                          &self.id, &self.id, colour);
        wish::tell_wish(&msg);
    }

    /// Sets colour of description.
    pub fn description_font(&self, font: &font::TkFont) {
        let msg = format!("global {}; ${} font description {{{}}}",
                          &self.id, &self.id, font);
        wish::tell_wish(&msg);
    }

    /// Sets colour of completed part of task.
    pub fn completed_colour(&self, colour: &str) {
        let msg = format!("global {}; ${} colour completed {}",
                          &self.id, &self.id, colour);
        wish::tell_wish(&msg);
    }

    /// Sets colour of uncompleted part of task.
    pub fn uncompleted_colour(&self, colour: &str) {
        let msg = format!("global {}; ${} colour left {}",
                          &self.id, &self.id, colour);
        wish::tell_wish(&msg);
    }

    /// Sets background colour of odd entries.
    pub fn odd_colour(&self, colour: &str) {
        let msg = format!("global {}; ${} colour odd {}",
                          &self.id, &self.id, colour);
        wish::tell_wish(&msg);
    }

    /// Sets background colour of even entries.
    pub fn even_colour(&self, colour: &str) {
        let msg = format!("global {}; ${} colour even {}",
                          &self.id, &self.id, colour);
        wish::tell_wish(&msg);
    }

    /// Sets colour of summary text.
    pub fn summary_colour(&self, colour: &str) {
        let msg = format!("global {}; ${} colour summary {}",
                          &self.id, &self.id, colour);
        wish::tell_wish(&msg);
    }

    /// Sets font of summary text.
    pub fn summary_font(&self, font: &font::TkFont) {
        let msg = format!("global {}; ${} font summary {{{}}}",
                          &self.id, &self.id, font);
        wish::tell_wish(&msg);
    }

    /// Sets colour of summary bar.
    pub fn summary_bar_colour(&self, colour: &str) {
        let msg = format!("global {}; ${} colour summarybar {}",
                          &self.id, &self.id, colour);
        wish::tell_wish(&msg);
    }

    /// Sets font of time scale.
    pub fn scale_font(&self, font: &font::TkFont) {
        let msg = format!("global {}; ${} font scale {{{}}}",
                          &self.id, &self.id, font);
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

    /// Connect two tasks.
    pub fn connect(&self, from_task: &TkGanttTask, to_task: &TkGanttTask) {
        let msg = format!("global {}; ${} connect ${} ${}",
                          &self.id, &self.id, &from_task.id, &to_task.id);
        wish::tell_wish(&msg);
    }

    /// Summary for given tasks.
    pub fn summary(&self, text: &str, tasks: &[&TkGanttTask]) {
        let mut task_str = String::new();
        for task in tasks {
            task_str.push_str(&format!("${} ", task.id));
        }

        let msg = format!("global {}; ${} summary {{{}}} {}",
                          &self.id, &self.id, text, task_str);
        wish::tell_wish(&msg);
    }

    /// Adds a task to the chart.
    pub fn task(&self, text: &str, (time_begin, time_end): (&str, &str), completed: u64) -> TkGanttTask {
        let id = wish::next_var();
        let msg = format!("global {}; set {} [${} task {{{}}} {{{}}} {{{}}} {}]",
                          &self.id, id, &self.id, text, time_begin, time_end, completed);
        wish::tell_wish(&msg);

        TkGanttTask { id }
    }
}

