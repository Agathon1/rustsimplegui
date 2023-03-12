//! Progressbar widget - displays feedback on progress through a task.
//!
//! * also see the Tk [manual](https://www.tcl-lang.org/man/tcl8.6/TkCmd/ttk_progressbar.htm)
//!

use super::grid;
use super::pack;
use super::widget;
use super::wish;

/// Refers to a progressbar widget
#[derive(Clone, Debug, PartialEq)]
pub struct TkProgressbar {
    pub id: String,
    mode: widget::ProgressMode,
}

/// Creates an instance of a progressbar in given parent.
pub fn make_progressbar(
    parent: &impl widget::TkWidget,
    orientation: widget::Orientation,
    mode: widget::ProgressMode,
) -> TkProgressbar {
    let id = wish::next_wid(parent.id());
    let msg = format!(
        "ttk::progressbar {} -orient {} -mode {}",
        id, orientation, mode
    );
    wish::tell_wish(&msg);

    TkProgressbar { id, mode }
}

impl widget::TkWidget for TkProgressbar {
    /// Returns the widget's id reference - used within tk
    fn id(&self) -> &str {
        &self.id
    }
}

impl grid::TkGridLayout for TkProgressbar {}
impl pack::TkPackLayout for TkProgressbar {}

impl TkProgressbar {
    /// Displayed length of progress bar in pixels.
    pub fn length(&self, value: u64) {
        widget::configure(&self.id, "length", &value.to_string());
    }

    /// Sets the maximum value for the progress bar - defaults to 100.0.
    pub fn maximum(&self, value: f64) {
        widget::configure(&self.id, "maximum", &value.to_string());
    }

    /// Starts auto-increment for the progress bar, updating
    /// after every 'interval' milliseconds (50 is recommended).
    pub fn start(&self, interval: u64) {
        let msg = format!("{} start {}", &self.id, interval);
        wish::tell_wish(&msg);
    }

    /// Sets the state of the widget (normal or disabled).
    pub fn state(&self, value: widget::State) {
        widget::configure(&self.id, "state", &value.to_string());
    }

    /// Steps the progress bar manually by given amount.
    pub fn step(&self, value: f64) {
        let msg = format!("{} step {}", &self.id, value);
        wish::tell_wish(&msg);
    }

    /// Stops auto-increment for the progress bar.
    pub fn stop(&self) {
        let msg = format!("{} stop", &self.id);
        wish::tell_wish(&msg);
    }

    /// Returns the current value of the progress bar.
    pub fn value_get(&self) -> f64 {
        let result = widget::TkWidget::cget(self, "value");
        if let Ok(value) = result.parse::<f64>() {
            value
        } else {
            0.0
        }
    }

    /// Sets the value of the progress bar.
    pub fn value(&self, value: f64) {
        widget::configure(&self.id, "value", &value.to_string());
    }
}
