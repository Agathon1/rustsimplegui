//! Plotchart - common enums and methods for all charts.
//!

use std::fmt;

use crate::canvas;
use crate::font;
use crate::image;
use crate::widget;
use crate::wish;

/// Used to give either the number of series in a bar chart, or 
/// indicate they should be drawn in a stacked manner.
#[derive(Clone, Debug, PartialEq)]
pub enum BarSeries {
    Count(u64),
    Stacked,
}

impl fmt::Display for BarSeries {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            BarSeries::Count(n) => n.to_string(),
            BarSeries::Stacked => "stacked".to_string(),
        };
        write!(f, "{}", &value)
    }
}

/// Used to define type of whiskers to draw in a box plot.
#[derive(Clone, Debug, PartialEq)]
pub enum BoxWhiskers {
    Extremes,
    Iqr,
    None,
}

impl fmt::Display for BoxWhiskers {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            BoxWhiskers::Extremes => "extremes",
            BoxWhiskers::Iqr => "iqr",
            BoxWhiskers::None => "none",
        };
        write!(f, "{}", &value)
    }
}

/// Indicates whether a colour should become brighter or darker.
#[derive(Clone, Debug, PartialEq)]
pub enum Brightness {
    Bright,
    Dark,
}

impl fmt::Display for Brightness {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Brightness::Bright => "bright",
            Brightness::Dark => "dark",
        };
        write!(f, "{}", &value)
    }
}

/// Type of line to use in drawing chart ticklines.
#[derive(Clone, Debug, PartialEq)]
pub enum ChartDash {
    Dots1,
    Dots2,
    Dots3,
    Dots4,
    Dots5,
    Lines,
}

impl fmt::Display for ChartDash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            ChartDash::Dots1 => "dots1",
            ChartDash::Dots2 => "dots2",
            ChartDash::Dots3 => "dots3",
            ChartDash::Dots4 => "dots4",
            ChartDash::Dots5 => "dots5",
            ChartDash::Lines => "lines",
        };
        write!(f, "{}", &value)
    }
}

impl ChartDash {
    pub fn to_short_string(&self) -> String {
        let value = match self {
            ChartDash::Dots1 => ".",
            ChartDash::Dots2 => "..",
            ChartDash::Dots3 => "...",
            ChartDash::Dots4 => "....",
            ChartDash::Dots5 => ".....",
            ChartDash::Lines => "{}",
        };
        String::from(value)
    }
}

/// Specifies colour map scheme.
#[derive(Clone, Debug, PartialEq)]
pub enum ColourMap {
    Cool,
    Gray,
    Grey,
    Hot,
    Jet,
}

impl fmt::Display for ColourMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            ColourMap::Cool => "cool",
            ColourMap::Gray => "gray",
            ColourMap::Grey => "grey",
            ColourMap::Hot => "hot",
            ColourMap::Jet => "jet",
        };
        write!(f, "{}", &value)
    }
}

/// Specifies interpretation of coordinates.
#[derive(Clone, Debug, PartialEq)]
pub enum CoordinatesType {
    /// (x, y) coordinate
    Cartesian,
    /// Angles, with 0 to North, 90 to East
    Nautical,
    /// Angles, with 0 on x-axis, 90 on y-axis
    Polar,
}

impl fmt::Display for CoordinatesType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            CoordinatesType::Cartesian => "cartesian",
            CoordinatesType::Nautical => "nautical",
            CoordinatesType::Polar => "polar",
        };
        write!(f, "{}", &value)
    }
}

/// Defines direction of text against given point.
#[derive(Clone, Debug, PartialEq)]
pub enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Direction::North => "north",
            Direction::NorthEast => "north-east",
            Direction::East => "east",
            Direction::SouthEast => "south-east",
            Direction::South => "south",
            Direction::SouthWest => "south-west",
            Direction::West => "west",
            Direction::NorthWest => "north-west",
        };
        write!(f, "{}", &value)
    }
}

impl Direction {
    pub(super) fn to_short_string(&self) -> String {
        let value = match self {
            Direction::North => "n",
            Direction::NorthEast => "ne",
            Direction::East => "e",
            Direction::SouthEast => "se",
            Direction::South => "s",
            Direction::SouthWest => "sw",
            Direction::West => "w",
            Direction::NorthWest => "nw",
        };
        String::from(value)
    }
}

/// Choice of displaying line/symbol or both when plotting a data series.
#[derive(Clone, Debug, PartialEq)]
pub enum DrawingMode {
    Both,
    Line,
    Symbol,
}

impl fmt::Display for DrawingMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            DrawingMode::Both => "both",
            DrawingMode::Line => "line",
            DrawingMode::Symbol => "symbol",
        };
        write!(f, "{}", &value)
    }
}

/// Specifies part of graph to fill.
#[derive(Clone, Debug, PartialEq)]
pub enum FillArea {
    AboveLine,
    BelowLine,
    None,
}

impl fmt::Display for FillArea {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            FillArea::AboveLine => "up",
            FillArea::BelowLine => "down",
            FillArea::None => "no",
        };
        write!(f, "{}", &value)
    }
}

/// Direction of colour gradient.
#[derive(Clone, Debug, PartialEq)]
pub enum GradientDirection {
    TopDown,
    BottomUp,
    LeftRight,
    RightLeft,
}

impl fmt::Display for GradientDirection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            GradientDirection::TopDown => "top-down",
            GradientDirection::BottomUp => "bottom-up",
            GradientDirection::LeftRight => "left-right",
            GradientDirection::RightLeft => "right-left",
        };
        write!(f, "{}", &value)
    }
}

/// Defines display style of a histogram.
pub enum HistogramStyle {
    Filled,
    Plateau,
    Spike,
    Stair,
}

impl fmt::Display for HistogramStyle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            HistogramStyle::Filled => "filled",
            HistogramStyle::Plateau => "plateau",
            HistogramStyle::Spike => "spike",
            HistogramStyle::Stair => "stair",
        };
        write!(f, "{}", &value)
    }
}

/// Defines type of legend.
#[derive(Clone, Debug, PartialEq)]
pub enum LegendType {
    Line,
    Rectangle,
}

impl fmt::Display for LegendType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            LegendType::Line => "line",
            LegendType::Rectangle => "rectangle",
        };
        write!(f, "{}", &value)
    }
}

/// Defines location of text against given point for label-dot.
#[derive(Clone, Debug, PartialEq)]
pub enum Location {
    North,
    East,
    South,
    West,
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Location::North => "n",
            Location::East => "e",
            Location::South => "s",
            Location::West => "w",
        };
        write!(f, "{}", &value)
    }
}

/// Used to give direction of adding charts in [plot_pack](widget::plot_pack).
#[derive(Clone, Debug, PartialEq)]
pub enum PlotDirection {
    Bottom,
    Left,
    Right,
    Top,
}

impl fmt::Display for PlotDirection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            PlotDirection::Bottom => "bottom",
            PlotDirection::Left => "left",
            PlotDirection::Right => "right",
            PlotDirection::Top => "top",
        };
        write!(f, "{}", &value)
    }
}

/// Defines position of legend with respect to plot.
#[derive(Clone, Debug, PartialEq)]
pub enum Position {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Position::TopLeft => "top-left",
            Position::TopRight => "top-right",
            Position::BottomLeft => "bottom-left",
            Position::BottomRight => "bottom-right",
        };
        write!(f, "{}", &value)
    }
}

/// Style of plot on a radial chart
#[derive(Clone, Debug, PartialEq)]
pub enum RadialStyle {
    Cumulative,
    Filled,
    Lines,
}

impl fmt::Display for RadialStyle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            RadialStyle::Cumulative => "cumulative",
            RadialStyle::Filled => "filled",
            RadialStyle::Lines => "lines",
        };
        write!(f, "{}", &value)
    }
}

/// Used to give either the step size on the axes of an isometric plot, or
/// indicate there should be no axes drawn.
#[derive(Clone, Debug, PartialEq)]
pub enum StepSize {
    NoAxes,
    Value(u64),
}

impl fmt::Display for StepSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            StepSize::NoAxes => "noaxes".to_string(),
            StepSize::Value(n) => n.to_string(),
        };
        write!(f, "{}", &value)
    }
}

/// Type of symbol used when plotting data series.
#[derive(Clone, Debug, PartialEq)]
pub enum Symbol {
    Circle,
    Cross,
    /// Filled circle
    Dot,
    /// Triangle pointing down
    Down,
    DownFilled,
    Plus,
    /// Triangle pointing up
    Up,
    UpFilled,
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Symbol::Circle => "circle",
            Symbol::Cross => "cross",
            Symbol::Dot => "dot",
            Symbol::Down => "down",
            Symbol::DownFilled => "downfilled",
            Symbol::Plus => "plus",
            Symbol::Up => "up",
            Symbol::UpFilled => "upfilled",
        };
        write!(f, "{}", &value)
    }
}

/// Methods common to (_almost_) all chart types.
pub trait TkPlotchart {
    /// Returns the widget's id reference - used within tk
    fn id(&self) -> &str;

    /// Sets the background colour for the axes.
    fn background_axes_colour(&self, colour: &str) {
        let msg = format!("global {}; ${} background axes {}",
                          self.id(), self.id(), colour);
        wish::tell_wish(&msg);
    }

    /// Sets a background gradient.
    fn background_gradient_colour(&self, colour: &str, 
                                  direction: GradientDirection, 
                                  brightness: Brightness) {
        let msg = format!("global {}; ${} background gradient {} {} {}",
                          self.id(), self.id(), colour, direction, brightness);
        wish::tell_wish(&msg);
    }

    /// Sets a background image.
    fn background_image(&self, image: &image::TkImage) {
        let msg = format!("global {}; ${} background image {}",
                          self.id(), self.id(), &image.id);
        wish::tell_wish(&msg);
    }

    /// Sets the background colour for the plot area.
    fn background_plot_colour(&self, colour: &str) {
        let msg = format!("global {}; ${} background plot {}",
                          self.id(), self.id(), colour);
        wish::tell_wish(&msg);
    }

    /// Creates balloon text (does not work for 3D plots). 
    fn balloon(&self, (x, y): (f64, f64), text: &str, direction: Direction) {
        let msg = format!("global {}; ${} balloon {} {} {{{}}} {}",
                          self.id(), self.id(), 
                          x, y, text, direction);
        wish::tell_wish(&msg);
    }

    /// Sets arrow-length, in pixels, for balloon text.
    fn balloon_arrow_size(&self, value: u64) {
        let msg = format!("global {}; ${} balloonconfig -arrowsize {}",
                          self.id(), self.id(), value);
        wish::tell_wish(&msg);
    }

    /// Sets background colour of balloon text.
    fn balloon_background_colour(&self, colour: &str) {
        let msg = format!("global {}; ${} balloonconfig -background {{{}}}",
                          self.id(), self.id(), colour);
        wish::tell_wish(&msg);
    }

    /// Sets font for balloon text.
    fn balloon_font(&self, font: &font::TkFont) {
        let msg = format!("global {}; ${} balloonconfig -font {{{}}}",
                          self.id(), self.id(), font);
        wish::tell_wish(&msg);
    }

    /// Sets justification for balloon text.
    fn balloon_justify(&self, value: widget::Justify) {
        let msg = format!("global {}; ${} balloonconfig -justify {}",
                          self.id(), self.id(), value);
        wish::tell_wish(&msg);
    }

    /// Sets outline colour of balloon text.
    fn balloon_outline_colour(&self, colour: &str) {
        let msg = format!("global {}; ${} balloonconfig -outline {{{}}}",
                          self.id(), self.id(), colour);
        wish::tell_wish(&msg);
    }

    /// Sets width of margin, in pixels, around balloon text.
    fn balloon_margin(&self, value: u64) {
        let msg = format!("global {}; ${} balloonconfig -margin {}",
                          self.id(), self.id(), value);
        wish::tell_wish(&msg);
    }

    /// Sets margin size, in pixels, around balloon text.
    fn balloon_rim_width(&self, value: u64) {
        let msg = format!("global {}; ${} balloonconfig -rimwidth {}",
                          self.id(), self.id(), value);
        wish::tell_wish(&msg);
    }

    /// Sets colour of balloon text.
    fn balloon_text_colour(&self, colour: &str) {
        let msg = format!("global {}; ${} balloonconfig -textcolour {{{}}}",
                          self.id(), self.id(), colour);
        wish::tell_wish(&msg);
    }

    /// Draws a horizontal light-grey band.
    fn draw_x_band(&self, y_min: f64, y_max: f64) {
        let msg = format!("global {}; ${} xband {} {}",
                          self.id(), self.id(), y_min, y_max);
        wish::tell_wish(&msg);
    }

    /// Draws a vertical light-grey band.
    fn draw_y_band(&self, y_min: f64, y_max: f64) {
        let msg = format!("global {}; ${} yband {} {}",
                          self.id(), self.id(), y_min, y_max);
        wish::tell_wish(&msg);
    }

    /// Erases this plot and all associated resources.
    fn erase(&self) {
        let msg = format!("::Plotchart::eraseplot ${}", self.id());
        wish::tell_wish(&msg);
    }

    /// Adds a line to legend for given data series.
    fn legend(&self, series: &str, text: &str) {
        let msg = format!("global {}; ${} legend {} {{{}}}",
                          self.id(), self.id(), 
                          series, text);
        wish::tell_wish(&msg);
    }

    /// Sets background colour for legend.
    fn legend_background(&self, colour: &str) {
        let msg = format!("global {}; ${} legendconfig -background {}",
                              self.id(), self.id(), colour);
        wish::tell_wish(&msg);
    }

    /// Sets border colour for legend.
    fn legend_border(&self, colour: &str) {
        let msg = format!("global {}; ${} legendconfig -border {}",
                              self.id(), self.id(), colour);
        wish::tell_wish(&msg);
    }

    /// Sets canvas on which to draw legend.
    fn legend_canvas(&self, canvas: &canvas::TkCanvas) {
        let msg = format!("global {}; ${} legendconfig -canvas {}",
                              self.id(), self.id(), &canvas.id);
        wish::tell_wish(&msg);
    }

    /// Sets font with which to draw legend.
    fn legend_font(&self, font: &font::TkFont) {
        let msg = format!("global {}; ${} legendconfig -font {{{}}}",
                              self.id(), self.id(), font);
        wish::tell_wish(&msg);
    }

    /// Position of legend on display.
    fn legend_position(&self, value: Position) {
        let msg = format!("global {}; ${} legendconfig -position {}",
                              self.id(), self.id(), value);
        wish::tell_wish(&msg);
    }

    /// Removes legend entry for given series.
    fn legend_remove(&self, series: &str) {
        let msg = format!("global {}; ${} removefromlegend {{{}}}",
                          self.id(), self.id(), series);
        wish::tell_wish(&msg);
    }

    /// Sets spacing between rows in legend.
    fn legend_spacing(&self, value: u64) {
        let msg = format!("global {}; ${} legendconfig -spacing {}",
                          self.id(), self.id(), value);
        wish::tell_wish(&msg);
    }

    /// Type of legend to display - series identified by line or colour rectangle.
    fn legend_type(&mut self, value: LegendType) {
        let msg = format!("global {}; ${} legendconfig -legendtype {}",
                              self.id(), self.id(), value);
        wish::tell_wish(&msg);
    }

    /// Starts definition of plain text (does not work for 3D plots). 
    fn plaintext(&self, (x, y): (f64, f64), text: &str, direction: Direction) {
        let msg = format!("global {}; ${} plaintext {} {} {{{}}} {}",
                          self.id(), self.id(), 
                          x, y, text, direction);
        wish::tell_wish(&msg);
    }

    /// Sets colour of plain text.
    fn plaintext_colour(&self, colour: &str) {
        let msg = format!("global {}; ${} plaintextconfig -textcolour {{{}}}",
                          self.id(), self.id(), colour);
        wish::tell_wish(&msg);
    }

    /// Sets font for plain text.
    fn plaintext_font(&self, font: &font::TkFont) {
        let msg = format!("global {}; ${} plaintextconfig -font {{{}}}",
                          self.id(), self.id(), font);
        wish::tell_wish(&msg);
    }

    /// Sets justification for plain text.
    fn plaintext_justify(&self, value: widget::Justify) {
        let msg = format!("global {}; ${} plaintextconfig -justify {}",
                          self.id(), self.id(), value);
        wish::tell_wish(&msg);
    }

    /// Saves chart to a file in postscript format.
    fn save(&self, filename: &str) {
        let msg = format!("global {}; ${} saveplot {{{}}} -plotregion window",
                          self.id(), self.id(), filename);
        wish::tell_wish(&msg);
    }

    /// Sets subtitle of chart.
    fn subtitle(&self, text: &str) {
        let msg = format!(
            "global {}; ${} subtitle {{{}}}",
            self.id(),
            self.id(),
            text,
            );
        wish::tell_wish(&msg);
    }

    /// Sets title of chart.
    fn title(&self, text: &str, placement: widget::Justify) {
        let msg = format!(
            "global {}; ${} title {{{}}} {}",
            self.id(),
            self.id(),
            text,
            placement
            );
        wish::tell_wish(&msg);
    }

    /// Sets subtitle of the (vertical) y-axis, and displays vertically 
    /// along axis.
    fn v_subtitle(&self, text: &str) {
        let msg = format!(
            "global {}; ${} vsubtext {{{}}}",
            self.id(),
            self.id(),
            text,
            );
        wish::tell_wish(&msg);
    }

    /// Sets title of the (vertical) y-axis, and displays vertically 
    /// along axis.
    fn v_title(&self, text: &str) {
        let msg = format!(
            "global {}; ${} vtext {{{}}}",
            self.id(),
            self.id(),
            text,
            );
        wish::tell_wish(&msg);
    }

    /// Sets tcl format string for numbers, see Tk
    /// [manual](https://www.tcl.tk/man/tcl8.5/TclCmd/format.htm)
    fn x_format(&self, format: &str) {
        let msg = format!("global {}; ${} xconfig -format {{{}}}",
                          self.id(), self.id(), format);
        wish::tell_wish(&msg);
    }

    /// Sets space in pixels between label and tickmark.
    fn x_label_offset(&self, value: f64) {
        let msg = format!("global {}; ${} xconfig -labeloffset {}",
                          self.id(), self.id(), value);
        wish::tell_wish(&msg);
    }

    /// Sets number of minor tick marks.
    fn x_minor_ticks(&self, value: u64) {
        let msg = format!("global {}; ${} xconfig -minorticks {}",
                          self.id(), self.id(), value);
        wish::tell_wish(&msg);
    }

    /// Changes x-axis definition to (min, max, step).
    fn x_scale(&self, (min, max, step): (f64, f64, f64)) {
        let msg = format!("global {}; ${} xconfig -scale {{{} {} {} }}",
                          self.id(), self.id(), min, max, step);
        wish::tell_wish(&msg);
    }

    /// Sets subtitle of the (horizontal) x-axis.
    fn x_subtitle(&self, text: &str) {
        let msg = format!(
            "global {}; ${} xsubtext {{{}}}",
            self.id(),
            self.id(),
            text,
            );
        wish::tell_wish(&msg);
    }

    /// Turns on display of vertical ticklines at each tick location.
    fn x_tick_lines(&self, colour: &str, dash: ChartDash) {
        let msg = format!("global {}; ${} xticklines {} {}",
                          self.id(), self.id(), colour, dash);
        wish::tell_wish(&msg);
    }

    /// Sets length in pixels of tick lines.
    fn x_tick_length(&self, value: u64) {
        let msg = format!("global {}; ${} xconfig -ticklength {}",
                          self.id(), self.id(), value);
        wish::tell_wish(&msg);
    }

    /// Sets title of the (horizontal) x-axis.
    fn x_title(&self, text: &str) {
        let msg = format!(
            "global {}; ${} xtext {{{}}}",
            self.id(),
            self.id(),
            text,
            );
        wish::tell_wish(&msg);
    }

    /// Sets tcl format string for numbers, see Tk
    /// [manual](https://www.tcl.tk/man/tcl8.5/TclCmd/format.htm)
    fn y_format(&self, format: &str) {
        let msg = format!("global {}; ${} yconfig -format {{{}}}",
                          self.id(), self.id(), format);
        wish::tell_wish(&msg);
    }

    /// Sets space in pixels between label and tickmark.
    fn y_label_offset(&self, value: f64) {
        let msg = format!("global {}; ${} yconfig -labeloffset {}",
                          self.id(), self.id(), value);
        wish::tell_wish(&msg);
    }

    /// Sets number of minor tick marks.
    fn y_minor_ticks(&self, value: u64) {
        let msg = format!("global {}; ${} yconfig -minorticks {}",
                          self.id(), self.id(), value);
        wish::tell_wish(&msg);
    }

    /// Changes y-axis definition to (min, max, step).
    fn y_scale(&self, (min, max, step): (f64, f64, f64)) {
        let msg = format!("global {}; ${} yconfig -scale {{{} {} {} }}",
                          self.id(), self.id(), min, max, step);
        wish::tell_wish(&msg);
    }

    /// Sets subtitle of the (vertical) y-axis.
    fn y_subtitle(&self, text: &str) {
        let msg = format!(
            "global {}; ${} ysubtext {{{}}}",
            self.id(),
            self.id(),
            text,
            );
        wish::tell_wish(&msg);
    }

    /// Turns on display of vertical ticklines at each tick location.
    fn y_tick_lines(&self, colour: &str, dash: ChartDash) {
        let msg = format!("global {}; ${} yticklines {} {}",
                          self.id(), self.id(), colour, dash);
        wish::tell_wish(&msg);
    }

    /// Sets length in pixels of tick lines.
    fn y_tick_length(&self, value: u64) {
        let msg = format!("global {}; ${} yconfig -ticklength {}",
                          self.id(), self.id(), value);
        wish::tell_wish(&msg);
    }

    /// Sets title of the (vertical) y-axis.
    fn y_title(&self, text: &str) {
        let msg = format!(
            "global {}; ${} ytext {{{}}}",
            self.id(),
            self.id(),
            text,
            );
        wish::tell_wish(&msg);
    }

}

/// Methods for configuring the way that data series are displayed 
/// on some kinds of charts.
pub trait TkChartSeries: TkPlotchart {
    /// Sets colour for displaying data series
    fn series_colour(&self, series: &str, colour: &str) {
        let msg = format!("global {}; ${} dataconfig {} -colour {}",
                          self.id(), self.id(), series, colour);
        wish::tell_wish(&msg);
    }

    /// Sets drawing mode for displaying data series 
    /// (called "type" in tklib's plotchart documentation).
    fn series_drawing_mode(&self, series: &str, mode: DrawingMode) {
        let msg = format!("global {}; ${} dataconfig {} -type {}",
                          self.id(), self.id(), series, mode);
        wish::tell_wish(&msg);
    }

    /// Used to select whether to fill the area above or below the line of 
    /// data series.
    fn series_fill_area(&self, series: &str, area: FillArea) {
        let msg = format!("global {}; ${} dataconfig {} -filled {}",
                          self.id(), self.id(), series, area);
        wish::tell_wish(&msg);
    }

    /// Sets colour to use when drawing filled area.
    fn series_fill_colour(&self, series: &str, colour: &str) {
        let msg = format!("global {}; ${} dataconfig {} -fillcolour {}",
                          self.id(), self.id(), series, colour);
        wish::tell_wish(&msg);
    }

    /// Sets width of line for displaying data series 
    fn series_line_width(&self, series: &str, width: u64) {
        let msg = format!("global {}; ${} dataconfig {} -width {}",
                          self.id(), self.id(), series, width);
        wish::tell_wish(&msg);
    }

    /// Sets symbol type and radius for displaying data series 
    fn series_symbol(&self, series: &str, symbol: Symbol, radius: u64) {
        let msg = format!("global {}; ${} dataconfig {} -symbol {} -radius {}",
                          self.id(), self.id(), series, symbol, radius);
        wish::tell_wish(&msg);
    }

}

/// Configuration options for dots, used in xy_plots and polar plots.
pub trait TkChartDots: TkPlotchart {
    /// Sets colour of dots in given data series.
    fn dot_colour(&self, series: &str, colour: &str) {
        let msg = format!("global {}; ${} dotconfig {} -colour {}",
                          self.id(), self.id(), series, colour);
        wish::tell_wish(&msg);
    }

    /// Defines class limits and colours, e.g. [(0.0, "green"), (2.0, "red"), ...]
    fn dot_classes(&self, series: &str, values: &[(f64, &str)]) {
        let mut class_str = String::new();
        for (value, colour) in values {
            class_str.push_str(&format!("{} {} ", value, colour));
        }

        let msg = format!("global {}; ${} dotconfig {} -classes {{{}}}",
                          self.id(), self.id(), series, class_str);
        wish::tell_wish(&msg);
    }

    /// Sets a 3D effect for dots in given data series.
    fn dot_effect_3d(&self, series: &str, value: bool) {
        let msg = format!("global {}; ${} dotconfig {} -3deffect {}",
                          self.id(), self.id(), series, 
                          if value { "1" } else { "0" });
        wish::tell_wish(&msg);
    }

    /// Sets whether an outline should be displayed for dots in given 
    /// data series.
    fn dot_outline(&self, series: &str, value: bool) {
        let msg = format!("global {}; ${} dotconfig {} -outline {}",
                          self.id(), self.id(), series, 
                          if value { "1" } else { "0" });
        wish::tell_wish(&msg);
    }

    /// Sets radius for dots in given data series.
    fn dot_radius(&self, series: &str, value: f64) {
        let msg = format!("global {}; ${} dotconfig {} -radius {}",
                          self.id(), self.id(), series, value);
        wish::tell_wish(&msg);
    }

    /// Sets scale factor of radius to pixels for dots in given data series.
    fn dot_scale(&self, series: &str, value: f64) {
        let msg = format!("global {}; ${} dotconfig {} -scale {}",
                          self.id(), self.id(), series, value);
        wish::tell_wish(&msg);
    }

    /// Sets whether dots in given data series should all have the 
    /// same size (false) or be scaled by value (true).
    fn dot_scale_by_value(&self, series: &str, value: bool) {
        let msg = format!("global {}; ${} dotconfig {} -scalebyvalue {}",
                          self.id(), self.id(), series, 
                          if value { "1" } else { "0" });
        wish::tell_wish(&msg);
    }

}

