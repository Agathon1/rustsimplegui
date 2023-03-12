//! TX Plot - a chart for displaying 2D data, where the x-axis represents _time_.
//!

use crate::canvas;
use crate::chart::plotchart;
use crate::wish;

/// Refers to a tx_plot
#[derive(Clone, Debug, PartialEq)]
pub struct TkTXPlot {
    pub id: String,
}

/// Intermediate definition for a tx_plot
#[derive(Clone, Debug, PartialEq)]
pub struct TkTXDefinition {
    canvas_id: String,
    time_axis: (String, String, u64),
    y_axis: (f64, f64, f64),
    time_format: Option<String>,
    gmt: Option<bool>,
    axes_at_zero: Option<bool>,
    isometric: Option<bool>,
}

/// Methods to set options for TX-plot - call 'plot' method at
/// end to finally create the chart.
impl TkTXDefinition {
    /// Adds string containing tk time format definition.
    pub fn time_format(&mut self, format: &str) -> &mut Self {
        self.time_format = Some(String::from(format));
        self
    }

    /// Sets value for GMT setting (whether to show day-light saving time).
    pub fn gmt(&mut self, value: bool) -> &mut Self {
        self.gmt = Some(value);
        self
    }

    /// Set to true to show axes at (0.0, 0.0) rather than lower-left corner.
    pub fn axes_at_zero(&mut self, value: bool) -> &mut Self {
        self.axes_at_zero = Some(value);
        self
    }

    /// Set to true to draw isometrically, i.e. x/y axes have equal physical size.
    pub fn isometric(&mut self, value: bool) -> &mut Self {
        self.isometric = Some(value);
        self
    }

    /// Completes the definition of an XY-plot and creates the chart.
    pub fn plot(&self) -> TkTXPlot {
        let id = wish::next_var();
        let mut msg = format!(
            "global {}; set {} [::Plotchart::createTXPlot {} {{{} {} {}}} {{{} {} {}}}",
            id, id, &self.canvas_id,
            self.time_axis.0, self.time_axis.1, self.time_axis.2,
            self.y_axis.0, self.y_axis.1, self.y_axis.2
        );

        if let Some(value) = &self.time_format {
            msg.push_str(&format!("-timeformat {{{}}} ", value));
        }
        if let Some(value) = self.gmt {
            msg.push_str(&format!("-gmt {} ", if value { "1" } else { "0" }));
        }
        if let Some(value) = self.axes_at_zero {
            msg.push_str(&format!("-axesatzero {} ", if value { "1" } else { "0" }));
        }
        if let Some(value) = self.isometric {
            msg.push_str(&format!("-isometric {} ", if value { "1" } else { "0" }));
        }

        msg.push_str("]");
        wish::tell_wish(&msg);

        TkTXPlot { id }
    }
}

/// Creates a tx_plot.
///
/// Constructor creates an instance of a TX plot definition in given canvas.
///
/// Options must be added and then 'plot' called to finally
/// create the chart.
pub fn make_tx(
    canvas: &canvas::TkCanvas,
    (min, max, step): (&str, &str, u64),
    y_axis: (f64, f64, f64),
) -> TkTXDefinition {
    TkTXDefinition {
        canvas_id: String::from(&canvas.id),
        time_axis: (String::from(min), String::from(max), step),
        y_axis,
        time_format: None,
        gmt: None,
        axes_at_zero: None,
        isometric: None,
    }
}

impl plotchart::TkPlotchart for TkTXPlot {
    /// Returns the widget's id reference - used within tk
    fn id(&self) -> &str {
        &self.id
    }
}

impl plotchart::TkChartSeries for TkTXPlot {}

impl TkTXPlot {
    /// Adds a vertical error interval to chart.
    ///
    /// * `series` - name of data series
    /// * `time_coord` - time-coordinate
    /// * `y_min` - minimum y-value
    /// * `y_max` - maximum y-value
    pub fn draw_interval(&self, series: &str, time_coord: &str, y_min: f64, y_max: f64) {
        let msg = format!(
            "global {}; ${} interval {} {} {} {}",
            &self.id, &self.id, series, time_coord, y_min, y_max
        );
        wish::tell_wish(&msg);
    }

    /// Adds a vertical error interval to chart with central symbol.
    ///
    /// * `series` - name of data series
    /// * `time_coord` - time-coordinate
    /// * `y_min` - minimum y-value
    /// * `y_max` - maximum y-value
    /// * `y_centre` - position of symbol
    pub fn draw_interval_with_symbol(
        &self,
        series: &str,
        time_coord: &str,
        y_min: f64,
        y_max: f64,
        y_centre: f64,
    ) {
        let msg = format!(
            "global {}; ${} interval {} {} {} {} {}",
            &self.id, &self.id, series, time_coord, y_min, y_max, y_centre
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

    /// Plots a single point.
    pub fn plot(&self, series: &str, (time, y): (&str, f64)) {
        let msg = format!(
            "global {}; ${} plot {} {} {}",
            &self.id, &self.id, series, time, y
        );
        wish::tell_wish(&msg);
    }
}

