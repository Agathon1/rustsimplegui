//! XY Plots - a set of charts for displaying 2D numeric data.
//!

use crate::canvas;
use crate::chart::plotchart;
use crate::widget;
use crate::wish;

#[derive(Clone,Debug,PartialEq)]
enum XYAxis {
    Log(f64, f64),
    Scale(f64, f64, f64),
}

/// Refers to an xy_plot
#[derive(Clone, Debug, PartialEq)]
pub struct TkXYPlot {
    pub id: String,
}

/// Intermediate definition for an xy_plot
#[derive(Clone, Debug, PartialEq)]
pub struct TkXYDefinition {
    canvas_id: String,
    tkname: String, // used in creating the plot
    x_axis: XYAxis,
    y_axis: XYAxis,
    x_labels: Option<String>,
    y_labels: Option<String>,
    time_format: Option<String>,
    gmt: Option<bool>,
    axes_at_zero: Option<bool>,
    isometric: Option<bool>,
}

/// Methods to set options for XY-plot - call 'plot' method at
/// end to finally create the chart.
impl TkXYDefinition {
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
    pub fn plot(&self) -> TkXYPlot {
        // if a labels set is defined, ignore the relevant step value.
        let x_str;
        match self.x_axis {
            XYAxis::Log(min, max) => x_str = format!("{{{} {}}}", min, max),
            XYAxis::Scale(min, max, step) => if self.x_labels.is_none() {
                x_str = format!("{{{} {} {}}}", min, max, step)
            } else {
                x_str = format!("{{{} {}}}", min, max)
            },
        };
        let y_str;
        match self.y_axis {
            XYAxis::Log(min, max) => y_str = format!("{{{} {}}}", min, max),
            XYAxis::Scale(min, max, step) => if self.y_labels.is_none() {
                y_str = format!("{{{} {} {}}}", min, max, step)
            } else {
                y_str = format!("{{{} {}}}", min, max)
            },
        };
        let id = wish::next_var();
        let mut msg = format!(
            "global {}; set {} [::Plotchart::create{} {} {} {} ",
            id, id, &self.tkname, &self.canvas_id, &x_str, &y_str
        );

        if let Some(labels) = &self.x_labels {
            msg.push_str(&format!("-xlabels {{{}}} ", labels));
        }
        if let Some(labels) = &self.y_labels {
            msg.push_str(&format!("-ylabels {{{}}} ", labels));
        }
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

        TkXYPlot { id }
    }
}

/// Creates an xy_plot.
///
/// Constructor creates an instance of an XY plot definition in given canvas.
///
/// Options must be added and then 'plot' called to finally
/// create the chart.
pub fn make_x_y(
    canvas: &canvas::TkCanvas,
    x_axis: (f64, f64, f64),
    y_axis: (f64, f64, f64),
) -> TkXYDefinition {
    TkXYDefinition {
        canvas_id: String::from(&canvas.id),
        tkname: String::from("XYPlot"),
        x_axis: XYAxis::Scale(x_axis.0, x_axis.1, x_axis.2),
        y_axis: XYAxis::Scale(y_axis.0, y_axis.1, y_axis.2),
        x_labels: None,
        y_labels: None,
        time_format: None,
        gmt: None,
        axes_at_zero: None,
        isometric: None,
    }
}

/// Creates an xy_plot where the x axis uses log values.
///
/// Constructor ceates an instance of an XY plot definition in given canvas.
///
/// Options must be added and then 'plot' called to finally
/// create the chart.
pub fn make_logx_y(
    canvas: &canvas::TkCanvas,
    x_axis: (f64, f64),
    y_axis: (f64, f64, f64),
) -> TkXYDefinition {
    TkXYDefinition {
        canvas_id: String::from(&canvas.id),
        tkname: String::from("LogXYPlot"),
        x_axis: XYAxis::Log(x_axis.0, x_axis.1),
        y_axis: XYAxis::Scale(y_axis.0, y_axis.1, y_axis.2),
        x_labels: None,
        y_labels: None,
        time_format: None,
        gmt: None,
        axes_at_zero: None,
        isometric: None,
    }
}

/// Creates an xy_plot where the y axis uses log values.
///
/// Cronstructor ceates an instance of an XY plot definition in given canvas.
///
/// Options must be added and then 'plot' called to finally
/// create the chart.
pub fn make_x_logy(
    canvas: &canvas::TkCanvas,
    x_axis: (f64, f64, f64),
    y_axis: (f64, f64),
) -> TkXYDefinition {
    TkXYDefinition {
        canvas_id: String::from(&canvas.id),
        tkname: String::from("XLogYPlot"),
        x_axis: XYAxis::Scale(x_axis.0, x_axis.1, x_axis.2),
        y_axis: XYAxis::Log(y_axis.0, y_axis.1),
        x_labels: None,
        y_labels: None,
        time_format: None,
        gmt: None,
        axes_at_zero: None,
        isometric: None,
    }
}

/// Creates an xy_plot where the x and y axes use log values.
///
/// Constructor creates an instance of an XY plot definition in given canvas.
///
/// Options must be added and then 'plot' called to finally
/// create the chart.
pub fn make_logx_logy(
    canvas: &canvas::TkCanvas,
    x_axis: (f64, f64),
    y_axis: (f64, f64),
) -> TkXYDefinition {
    TkXYDefinition {
        canvas_id: String::from(&canvas.id),
        tkname: String::from("LogXLogYPlot"),
        x_axis: XYAxis::Log(x_axis.0, x_axis.1),
        y_axis: XYAxis::Log(y_axis.0, y_axis.1),
        x_labels: None,
        y_labels: None,
        time_format: None,
        gmt: None,
        axes_at_zero: None,
        isometric: None,
    }
}

/// A strip_chart is an xy_plot whose x-axis expands to fit new data.
///
/// Constructor creates an instance of an XY plot definition in given canvas.
///
/// Options must be added and then 'plot' called to finally
/// create the chart.
pub fn make_strip_chart(
    canvas: &canvas::TkCanvas,
    x_axis: (f64, f64, f64),
    y_axis: (f64, f64, f64),
) -> TkXYDefinition {
    TkXYDefinition {
        canvas_id: String::from(&canvas.id),
        tkname: String::from("Stripchart"),
        x_axis: XYAxis::Scale(x_axis.0, x_axis.1, x_axis.2),
        y_axis: XYAxis::Scale(y_axis.0, y_axis.1, y_axis.2),
        x_labels: None,
        y_labels: None,
        time_format: None,
        gmt: None,
        axes_at_zero: None,
        isometric: None,
    }
}

impl plotchart::TkPlotchart for TkXYPlot {
    /// Returns the widget's id reference - used within tk
    fn id(&self) -> &str {
        &self.id
    }
}

impl plotchart::TkChartSeries for TkXYPlot {}
impl plotchart::TkChartDots for TkXYPlot {}

impl TkXYPlot {
    /// Draws a box-and-whiskers image on plot, in x direction.
    pub fn draw_box_and_whiskers_x(&self, series: &str, values: &[f64], y: f64) {
        let msg = format!(
            "global {}; ${} box-and-whiskers {} {{{}}} {}",
            &self.id,
            &self.id,
            series,
            &widget::str_list(values),
            y
        );
        wish::tell_wish(&msg);
    }

    /// Draws a box-and-whiskers image on plot, in y direction.
    pub fn draw_box_and_whiskers_y(&self, series: &str, x: f64, values: &[f64]) {
        let msg = format!(
            "global {}; ${} box-and-whiskers {} {} {{{}}}",
            &self.id,
            &self.id,
            series,
            x,
            &widget::str_list(values)
        );
        wish::tell_wish(&msg);
    }

    /// Draws filled contours for the given values on the grid.
    ///
    /// * `x_coords` is a slice of lists of x-coordinates
    /// * `y_coords` is a slice of lists of y-coordinates
    /// * `values` is a slice of lists of values at each (x, y) position
    /// * `classes` is a slice of class levels (leave empty for default).
    pub fn draw_contour_fill<M: AsRef<[R]>, R: AsRef<[f64]>>(
        &self,
        x_coords: M,
        y_coords: M,
        values: M,
        classes: &[f64],
    ) {
        let msg = format!(
            "global {}; ${} contourfill {{{}}} {{{}}} {{{}}} {{{}}}",
            &self.id,
            &self.id,
            &widget::str_list_lists(&x_coords),
            &widget::str_list_lists(&y_coords),
            &widget::str_list_lists(&values),
            &widget::str_list(classes)
        );
        wish::tell_wish(&msg);
    }

    /// Draws contour lines for the given values on the grid.
    ///
    /// * `x_coords` is a slice of lists of x-coordinates
    /// * `y_coords` is a slice of lists of y-coordinates
    /// * `values` is a slice of lists of values at each (x, y) position
    /// * `classes` is a slice of class levels (leave empty for default).
    pub fn draw_contour_lines<M: AsRef<[R]>, R: AsRef<[f64]>>(
        &self,
        x_coords: M,
        y_coords: M,
        values: M,
        classes: &[f64],
    ) {
        let msg = format!(
            "global {}; ${} contourlines {{{}}} {{{}}} {{{}}} {{{}}}",
            &self.id,
            &self.id,
            &widget::str_list_lists(&x_coords),
            &widget::str_list_lists(&y_coords),
            &widget::str_list_lists(&values),
            &widget::str_list(classes)
        );
        wish::tell_wish(&msg);
    }

    /// Draws contour lines for the given values on the grid.
    ///
    /// * `x_coords` is a slice of lists of x-coordinates
    /// * `y_coords` is a slice of lists of y-coordinates
    /// * `values` is a slice of lists of values at each (x, y) position
    /// * `classes` is a slice of class levels (leave empty for default).
    pub fn draw_contour_lines_function_values<M: AsRef<[R]>, R: AsRef<[f64]>>(
        &self,
        x_coords: M,
        y_coords: M,
        values: M,
        classes: &[f64],
    ) {
        let msg = format!(
            "global {}; ${} contourlinesfunctionvalues {{{}}} {{{}}} {{{}}} {{{}}}",
            &self.id,
            &self.id,
            &widget::str_list_lists(x_coords),
            &widget::str_list_lists(y_coords),
            &widget::str_list_lists(values),
            &widget::str_list(classes)
        );
        wish::tell_wish(&msg);
    }

    /// Draws a dot - value determines symbol/colour.
    pub fn draw_dot(&self, series: &str, (x, y): (f64, f64), value: f64) {
        let msg = format!(
            "global {}; ${} dot {} {} {} {}",
            &self.id, &self.id, series, x, y, value
        );
        wish::tell_wish(&msg);
    }

    /// Draws grid cells as lines connecting the given coordinates.
    ///
    /// * `x_coords` is a slice of lists of x-coordinates
    /// * `y_coords` is a slice of lists of y-coordinates
    pub fn draw_grid<M: AsRef<[R]>, R: AsRef<[f64]>>(&self, x_coords: M, y_coords: M) {
        let msg = format!(
            "global {}; ${} grid {{{}}} {{{}}}",
            &self.id,
            &self.id,
            &widget::str_list_lists(&x_coords),
            &widget::str_list_lists(&y_coords)
        );
        wish::tell_wish(&msg);
    }

    /// Adds a vertical error interval to chart.
    ///
    /// * `series` - name of data series
    /// * `x_coord` - x-coordinate
    /// * `y_min` - minimum y-value
    /// * `y_max` - maximum y-value
    pub fn draw_interval(&self, series: &str, x_coord: f64, y_min: f64, y_max: f64) {
        let msg = format!(
            "global {}; ${} interval {} {} {} {}",
            &self.id, &self.id, series, x_coord, y_min, y_max
        );
        wish::tell_wish(&msg);
    }

    /// Adds a vertical error interval to chart with central symbol.
    ///
    /// * `series` - name of data series
    /// * `x_coord` - x-coordinate
    /// * `y_min` - minimum y-value
    /// * `y_max` - maximum y-value
    /// * `y_centre` - position of symbol
    pub fn draw_interval_with_symbol(
        &self,
        series: &str,
        x_coord: f64,
        y_min: f64,
        y_max: f64,
        y_centre: f64,
    ) {
        let msg = format!(
            "global {}; ${} interval {} {} {} {} {}",
            &self.id, &self.id, series, x_coord, y_min, y_max, y_centre
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
            "global {}; ${} labeldot {} {} {{{}}} {}",
            &self.id, &self.id, x, y, label, location
        );
        wish::tell_wish(&msg);
    }

    /// Draws a horizontal band on chart.
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

    /// Draws a vector on chart.
    ///
    /// * `series` - name of data series
    /// * `(x, y)` - coordinate for arrow
    /// * `(u, v)` - depending on configuration, can be source point or angle, etc.
    ///
    /// Use vector_CONFIG methods to alter display of vector.
    pub fn draw_vector(&self, series: &str, (x, y): (f64, f64), (u, v): (f64, f64)) {
        let msg = format!(
            "global {}; ${} vector {} {} {} {} {}",
            &self.id, &self.id, series, x, y, u, v
        );
        wish::tell_wish(&msg);
    }

    /// Adds contour classes to the legend as coloured lines.
    ///
    /// * `values` is a slice of lists of values at each (x, y) position
    /// * `classes` is a slice of class levels (leave empty for default).
    pub fn legend_isometric_lines<M: AsRef<[R]>, R: AsRef<[f64]>>(&self, values: M, classes: &[f64]) {
        let msg = format!(
            "global {}; ${} legendisolines {{{}}} {{{}}}",
            &self.id,
            &self.id,
            &widget::str_list_lists(values.as_ref()),
            &widget::str_list(classes)
        );
        wish::tell_wish(&msg);
    }

    /// Adds contour classes to the legend as coloured rectangles.
    ///
    /// * `values` is a slice of lists of values at each (x, y) position
    /// * `classes` is a slice of class levels (leave empty for default).
    pub fn legend_shades<M: AsRef<[R]>, R: AsRef<[f64]>>(&self, values: M, classes: &[f64]) {
        let msg = format!(
            "global {}; ${} legendshades {{{}}} {{{}}}",
            &self.id,
            &self.id,
            &widget::str_list_lists(values.as_ref()),
            &widget::str_list(classes)
            );
        wish::tell_wish(&msg);
    }

    /// Plots a single point.
    pub fn plot(&self, series: &str, (x, y): (f64, f64)) {
        let msg = format!(
            "global {}; ${} plot {} {} {}",
            &self.id, &self.id, series, x, y
        );
        wish::tell_wish(&msg);
    }

    /// Plots a list of points.
    ///
    /// * `every` - set to true if a symbol should be drawn for every point
    pub fn plot_list(&self, series: &str, points: &[(f64, f64)], every: bool) {
        let xs: Vec<f64> = points.iter().map(|(x, _)| x).cloned().collect();
        let ys: Vec<f64> = points.iter().map(|(_, y)| y).cloned().collect();

        let msg = format!(
            "global {}; ${} plotlist {} {{{}}} {{{}}} {}",
            &self.id,
            &self.id,
            series,
            &widget::str_list(&xs),
            &widget::str_list(&ys),
            if every { "1" } else { "0" }
        );
        wish::tell_wish(&msg);
    }

    /// Plots a single point, like [plot](TkXYPlot::plot), but includes a
    /// +/- s.d. line on chart.
    pub fn rchart(&self, series: &str, (x, y): (f64, f64)) {
        let msg = format!(
            "global {}; ${} rchart {} {} {}",
            &self.id, &self.id, series, x, y
        );
        wish::tell_wish(&msg);
    }

    /// Plots points for a trendline.
    pub fn trend_line(&self, series: &str, (x, y): (f64, f64)) {
        let msg = format!(
            "global {}; ${} trend {} {} {}",
            &self.id, &self.id, series, x, y
        );
        wish::tell_wish(&msg);
    }

    /// Sets whether coordinates specify start or centre of vector arrow.
    pub fn vector_centred(&self, series: &str, value: bool) {
        let msg = format!( "global {}; ${} vectorconfig {} -centred {}",
                           &self.id, &self.id, series, 
                           if value { "1" } else { "0" });
        wish::tell_wish(&msg);
    }
    
    /// Sets colour of vector arrow.
    pub fn vector_colour(&self, series: &str, colour: &str) {
        let msg = format!( "global {}; ${} vectorconfig {} -colour {}",
                           &self.id, &self.id, series, colour);
        wish::tell_wish(&msg);
    }
    
    /// Sets scale factor for converting vector length into pixels.
    pub fn vector_scale(&self, series: &str, scale: f64) {
        let msg = format!( "global {}; ${} vectorconfig {} -scale {}",
                           &self.id, &self.id, series, scale);
        wish::tell_wish(&msg);
    } 

    /// Sets scale factor for converting vector length into pixels.
    pub fn vector_type(&self, series: &str, value: plotchart::CoordinatesType) {
        let msg = format!( "global {}; ${} vectorconfig {} -type {}",
                           &self.id, &self.id, series, value);
        wish::tell_wish(&msg);
    }
}

