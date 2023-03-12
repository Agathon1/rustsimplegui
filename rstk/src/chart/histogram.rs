//! Histogram - a bar chart where area, not height, represents frequency.
//!

use crate::canvas;
use crate::chart::plotchart;
use crate::widget;
use crate::wish;

/// Refers to a histogram
#[derive(Clone, Debug, PartialEq)]
pub struct TkHistogram {
    pub id: String,
}

/// Intermediate definition for a histogram
#[derive(Clone, Debug, PartialEq)]
pub struct TkHistogramDefinition {
    canvas_id: String,
    x_axis: (f64, f64, f64),
    y_axis: (f64, f64, f64),
    x_labels: Option<String>,
    y_labels: Option<String>,
}

/// Methods to set options for histogram - call 'plot' method at
/// end to finally create the chart.
impl TkHistogramDefinition {
    /// Adds custom labels for the x-axis.
    ///
    /// This will replace the step value for the x-axis.
    /// Labels which are numbers will be placed according to the given scale, 
    /// and otherwise are evenly distributed.
    pub fn x_labels(&mut self, labels: &[&str]) -> &mut Self {
        self.x_labels = Some(widget::strings_list(labels));
        self
    }

    /// Adds custom labels for the y-axis.
    ///
    /// This will replace the step value for the y-axis.
    /// Labels which are numbers will be placed according to the given scale, 
    /// and otherwise are evenly distributed.
    pub fn y_labels(&mut self, labels: &[&str]) -> &mut Self {
        self.y_labels = Some(widget::strings_list(labels));
        self
    }

    /// Completes the definition of a histogram and creates the chart.
    pub fn plot(&self) -> TkHistogram {
        // if a labels set is defined, ignore the relevant step value.
        let x_str;
        if self.x_labels.is_none() {
            x_str = format!("{{{} {} {}}}", self.x_axis.0, self.x_axis.1, self.x_axis.2);
        } else { 
            x_str = format!("{{{} {}}}", self.x_axis.0, self.x_axis.1);
        };
        let y_str;
        if self.y_labels.is_none() {
            y_str = format!("{{{} {} {}}}", self.y_axis.0, self.y_axis.1, self.y_axis.2);
        } else { 
            y_str = format!("{{{} {}}}", self.y_axis.0, self.y_axis.1);
        };
        let id = wish::next_var();
        let mut msg = format!(
            "global {}; set {} [::Plotchart::createHistogram {} {} {} ",
            id, id, &self.canvas_id, &x_str, &y_str);

        if let Some(labels) = &self.x_labels {
            msg.push_str(&format!("-xlabels {{{}}} ", labels));
        }
        if let Some(labels) = &self.y_labels {
            msg.push_str(&format!("-ylabels {{{}}} ", labels));
        }

        msg.push_str("]");
        wish::tell_wish(&msg);

        TkHistogram { id }
    }
}

/// Creates a histogram.
///
/// Constructor creates an instance of a histogram definition in given canvas.
///
/// Options must be added and then 'plot' called to finally
/// create the chart.
pub fn make_histogram(
    canvas: &canvas::TkCanvas,
    x_axis: (f64, f64, f64),
    y_axis: (f64, f64, f64),
) -> TkHistogramDefinition {
    TkHistogramDefinition {
        canvas_id: String::from(&canvas.id),
        x_axis,
        y_axis,
        x_labels: None,
        y_labels: None,
    }
}

impl plotchart::TkPlotchart for TkHistogram {
    /// Returns the widget's id reference - used within tk
    fn id(&self) -> &str {
        &self.id
    }
}

impl plotchart::TkChartSeries for TkHistogram {}

impl TkHistogram {
    /// Plots a single bar - the x-value defines the right-hand side of the bar.
    pub fn plot(&self, series: &str, (x, y): (f64, f64)) {
        let msg = format!(
            "global {}; ${} plot {} {} {}",
            &self.id, &self.id, series, x, y
        );
        wish::tell_wish(&msg);
    }

    /// Plots a single bar, accumulating the previous values.
    /// The x-value defines the right-hand side of the bar.
    pub fn plot_cumulative(&self, series: &str, (x, y): (f64, f64)) {
        let msg = format!(
            "global {}; ${} plotcumulative {} {} {}",
            &self.id, &self.id, series, x, y
        );
        wish::tell_wish(&msg);
    }

    /// Sets display-style of histogram.
    pub fn series_style(&self, series: &str, value: plotchart::HistogramStyle) {
        let msg = format!("global {}; ${} dataconfig {} -style {}",
                          &self.id, &self.id, series, value);
        wish::tell_wish(&msg);
    }
}
