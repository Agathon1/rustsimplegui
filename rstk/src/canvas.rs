//! Canvas widget - displays (interactive) graphics.
//!
//! * also see the Tk [manual](https://www.tcl-lang.org/man/tcl8.6/TkCmd/canvas.htm)

use super::grid;
use super::image;
use super::pack;
use super::widget;
use super::wish;

/// Refers to a canvas widget
#[derive(Clone, Debug, PartialEq)]
pub struct TkCanvas {
    pub id: String,
}

/// Refers to a canvas arc
#[derive(Clone, Debug, PartialEq)]
pub struct TkCanvasArc {
    pub canvas: String,
    pub id: String,
}

/// Refers to a canvas image
#[derive(Clone, Debug, PartialEq)]
pub struct TkCanvasImage {
    pub canvas: String,
    pub id: String,
}

/// Refers to a canvas line
#[derive(Clone, Debug, PartialEq)]
pub struct TkCanvasLine {
    pub canvas: String,
    pub id: String,
}

/// Refers to a canvas oval
#[derive(Clone, Debug, PartialEq)]
pub struct TkCanvasOval {
    pub canvas: String,
    pub id: String,
}

/// Refers to a canvas polygon
#[derive(Clone, Debug, PartialEq)]
pub struct TkCanvasPolygon {
    pub canvas: String,
    pub id: String,
}

/// Refers to a canvas rectangle
#[derive(Clone, Debug, PartialEq)]
pub struct TkCanvasRectangle {
    pub canvas: String,
    pub id: String,
}

/// Refers to a canvas text
#[derive(Clone, Debug, PartialEq)]
pub struct TkCanvasText {
    pub canvas: String,
    pub id: String,
}

/// Refers to a canvas widget
#[derive(Clone, Debug, PartialEq)]
pub struct TkCanvasWidget {
    pub canvas: String,
    pub id: String,
}

/// Creates an instance of a canvas widget in given parent.
pub fn make_canvas(parent: &impl widget::TkWidget) -> TkCanvas {
    let id = wish::next_wid(parent.id());
    let msg = format!("canvas {}", id);
    wish::tell_wish(&msg);

    TkCanvas { id }
}

impl widget::TkWidget for TkCanvas {
    /// Returns the widget's id reference - used within tk
    fn id(&self) -> &str {
        &self.id
    }
}
impl grid::TkGridLayout for TkCanvas {}
impl pack::TkPackLayout for TkCanvas {}

impl TkCanvas {
    /// Colour of canvas background.
    ///
    /// Colours are specified as a string, by either:
    ///
    /// * `name` - using one of the values in the tk [colours](https://tcl.tk/man/tcl8.6/TkCmd/colors.htm) list
    /// * `rgb` - as a 6-digit hexadecimal value in form "#RRGGBB"
    pub fn background(&self, colour: &str) {
        widget::configure(&self.id, "background", colour);
    }

    /// Size of border around widget.
    pub fn border_width(&self, width: u64) {
        widget::configure(&self.id, "borderwidth", &width.to_string());
    }

    /// Configures the item(s) with given tag.
    pub fn configure_tag(&self, tag: &str, option: &str, value: &str) {
        let msg = format!(
            "{} itemconfigure {} -{} {{{}}}",
            &self.id, tag, option, value
        );
        wish::tell_wish(&msg);
    }

    /// Creates an arc where (x1, y1) (x2, y2) define a rectangle
    /// enclosing the oval which defines the arc.
    pub fn create_arc(&self, (x1, y1): (u64, u64), (x2, y2): (u64, u64)) -> TkCanvasArc {
        let msg = format!(
            "puts [{} create arc {} {} {} {}] ; flush stdout",
            &self.id, x1, y1, x2, y2
        );
        let id = wish::ask_wish(&msg);

        TkCanvasArc {
            canvas: self.id.clone(),
            id,
        }
    }

    /// Creates an image at (x, y) according to given image reference.
    pub fn create_image(&self, (x, y): (u64, u64), image: &image::TkImage) -> TkCanvasImage {
        let msg = format!(
            "puts [{} create image {} {} {}] ; flush stdout",
            &self.id, x, y, &image.id
        );
        let id = wish::ask_wish(&msg);

        TkCanvasImage {
            canvas: self.id.clone(),
            id,
        }
    }

    /// Creates a line using slice of (x, y) coordinates.
    pub fn create_line(&self, points: &[(u64, u64)]) -> TkCanvasLine {
        let mut line_defn = String::new();
        for (x, y) in points {
            line_defn.push_str(&format!("{} {} ", x, y));
        }

        let msg = format!(
            "puts [{} create line {}] ; flush stdout",
            &self.id, &line_defn
        );
        let id = wish::ask_wish(&msg);

        TkCanvasLine {
            canvas: self.id.clone(),
            id,
        }
    }

    /// Creates an oval where (x1, y1) (x2, y2) define a rectangle
    /// enclosing the oval.
    pub fn create_oval(&self, (x1, y1): (u64, u64), (x2, y2): (u64, u64)) -> TkCanvasOval {
        let msg = format!(
            "puts [{} create oval {} {} {} {}] ; flush stdout",
            &self.id, x1, y1, x2, y2
        );
        let id = wish::ask_wish(&msg);

        TkCanvasOval {
            canvas: self.id.clone(),
            id,
        }
    }

    /// Creates a polygon using slice of (x, y) coordinates.
    pub fn create_polygon(&self, points: &[(u64, u64)]) -> TkCanvasPolygon {
        let mut line_defn = String::new();
        for (x, y) in points {
            line_defn.push_str(&format!("{} {} ", x, y));
        }

        let msg = format!(
            "puts [{} create polygon {}] ; flush stdout",
            &self.id, &line_defn
        );
        let id = wish::ask_wish(&msg);

        TkCanvasPolygon {
            canvas: self.id.clone(),
            id,
        }
    }

    /// Creates a rectangle with opposite corners (x1, y1) (x2, y2).
    pub fn create_rectangle(&self, (x1, y1): (u64, u64), (x2, y2): (u64, u64)) -> TkCanvasRectangle {
        let msg = format!(
            "puts [{} create rectangle {} {} {} {}] ; flush stdout",
            &self.id, x1, y1, x2, y2
        );
        let id = wish::ask_wish(&msg);

        TkCanvasRectangle {
            canvas: self.id.clone(),
            id,
        }
    }

    /// Creates a text item at (x, y) with given contents.
    pub fn create_text(&self, (x, y): (u64, u64), text: &str) -> TkCanvasText {
        let msg = format!(
            "puts [{} create text {} {} {{{}}}] ; flush stdout",
            &self.id, x, y, text
        );
        let id = wish::ask_wish(&msg);

        TkCanvasText {
            canvas: self.id.clone(),
            id,
        }
    }

    /// Creates a widget at (x, y) according to given widget reference.
    pub fn create_widget(&self, (x, y): (u64, u64), widget: &impl widget::TkWidget) -> TkCanvasWidget {
        let msg = format!(
            "puts [{} create window {} {} {}] ; flush stdout",
            &self.id,
            x,
            y,
            widget.id()
        );
        let id = wish::ask_wish(&msg);

        TkCanvasWidget {
            canvas: self.id.clone(),
            id,
        }
    }

    /// Deletes given item from canvas.
    pub fn delete(&self, item: &impl TkCanvasItem) {
        let msg = format!("{} delete {}", &self.id, item.id());
        wish::tell_wish(&msg);
    }

    /// Height of canvas, in pixels.
    pub fn height(&self, height: u64) {
        widget::configure(&self.id, "height", &height.to_string());
    }

    /// Style of interior relative to exterior.
    pub fn relief(&self, value: widget::Relief) {
        widget::configure(&self.id, "relief", &value.to_string());
    }

    /// Sets the state of the widget.
    pub fn state(&self, value: widget::State) {
        widget::configure(&self.id, "state", &value.to_string());
    }

    /// Width of canvas, in pixels.
    pub fn width(&self, width: u64) {
        widget::configure(&self.id, "width", &width.to_string());
    }
}

// -- functionality for each of the canvas items

/// Common functionality for all canvas items
pub trait TkCanvasItem {
    fn canvas(&self) -> &str;
    fn id(&self) -> &str;

    /// Binds event to item.
    fn bind(&self, pattern: &str, command: impl Fn(widget::TkEvent) + Send + 'static) {
        // tag+pattern used as identifier, as multiple commands can be bound to each entity
        let tag_pattern = format!("{}{}{}", self.canvas(), self.id(), pattern);
        wish::add_callback1_event(&tag_pattern, wish::mk_callback1_event(command));
        let msg = format!(
            "{} bind {} {} {{ puts cb1e:{}:%x:%y:%X:%Y:%h:%w:%k:%K:%b ; flush stdout }}",
            self.canvas(),
            self.id(),
            pattern,
            tag_pattern
        );
        wish::tell_wish(&msg);
    }

    /// Configures the individual item.
    fn configure(&self, option: &str, value: &str) {
        let msg = format!(
            "{} itemconfigure {} -{} {{{}}}",
            self.canvas(),
            self.id(),
            option,
            value
        );
        wish::tell_wish(&msg);
    }
}

/// Each item can have one or more named tags attached to it.
/// These tags can be used to configure groups of items.
pub trait TkCanvasTags: TkCanvasItem {
    /// Adds given tag to this canvas item.
    fn add_tag(&self, tag: &str) {
        let msg = format!("{} addtag {} withtag {}", &self.canvas(), tag, &self.id());
        wish::tell_wish(&msg);
    }

    /// Deletes tag from this canvas item.
    fn delete_tag(&self, tag: &str) {
        let msg = format!("{} dtag {} {}", &self.canvas(), &self.id(), tag);
        wish::tell_wish(&msg);
    }

    /// Returns all tags associated with this canvas item.
    fn get_tags(&self) -> Vec<String> {
        let msg = format!("{} gettags {}", &self.canvas(), &self.id());
        let tags = wish::ask_wish(&msg);

        let mut result: Vec<String> = vec![];
        for tag in tags.split_whitespace() {
            result.push(String::from(tag));
        }

        result
    }
}

/// Specifies which or both ends of lines to draw arrows.
pub enum TkArrowWhere {
    Both,
    First,
    Last,
    None,
}

/// Specifies how to draw an arc.
pub enum TkArcStyle {
    Arc,
    Chord,
    PieSlice,
}

/// Specifies how ends of lines are drawn.
pub enum TkCapStyle {
    Butt,
    Projecting,
    Round,
}

/// Specifies pattern with which to draw lines.
pub enum TkDash {
    Dot,
    Dash,
    DashDot,
    DashDotDot,
}

/// Specifies how lines are connected.
pub enum TkJoinStyle {
    Bevel,
    Miter,
    Round,
}

impl TkCanvasItem for TkCanvasArc {
    fn canvas(&self) -> &str {
        &self.canvas
    }

    fn id(&self) -> &str {
        &self.id
    }
}

impl TkCanvasTags for TkCanvasArc {}

impl TkCanvasArc {
    /// Pattern for drawing line.
    pub fn dash(&self, dash: TkDash) {
        let dash = match dash {
            TkDash::Dot => ".",
            TkDash::Dash => "-",
            TkDash::DashDot => "-.",
            TkDash::DashDotDot => "-..",
        };
        self.configure("dash", dash);
    }

    /// Fill colour for area.
    pub fn fill(&self, colour: &str) {
        self.configure("fill", colour);
    }

    /// Colour for outline.
    pub fn outline(&self, colour: &str) {
        self.configure("outline", colour);
    }

    /// Width of outline, in pixels.
    pub fn width(&self, value: u64) {
        self.configure("width", &value.to_string());
    }

    /// Extent is amount in degrees that arc extends counter-clockwise
    /// from its start angle.
    pub fn extent(&self, degrees: u64) {
        self.configure("extent", &degrees.to_string());
    }

    /// Start is starting angle in degrees, measured counter-clockwise
    /// from the 3 o'clock/x-axis position.
    pub fn start(&self, degrees: u64) {
        self.configure("start", &degrees.to_string());
    }

    /// Style in which to draw the arc: a pieslice, chord or arc-only.
    pub fn style(&self, value: TkArcStyle) {
        let value = match value {
            TkArcStyle::Arc => "arc",
            TkArcStyle::Chord => "chord",
            TkArcStyle::PieSlice => "pieslice",
        };
        self.configure("style", value);
    }
}

impl TkCanvasItem for TkCanvasImage {
    fn canvas(&self) -> &str {
        &self.canvas
    }

    fn id(&self) -> &str {
        &self.id
    }
}

impl TkCanvasTags for TkCanvasImage {}

impl TkCanvasImage {
    /// Positioning of image with respect to internal margins.
    pub fn anchor(&self, value: widget::Anchor) {
        self.configure("anchor", &value.to_string());
    }
}

impl TkCanvasItem for TkCanvasLine {
    fn canvas(&self) -> &str {
        &self.canvas
    }

    fn id(&self) -> &str {
        &self.id
    }
}

impl TkCanvasTags for TkCanvasLine {}

impl TkCanvasLine {
    /// Colour for line (same as Tk's "fill" option).
    pub fn colour(&self, colour: &str) {
        self.configure("fill", colour);
    }

    /// Pattern for drawing line.
    pub fn dash(&self, dash: TkDash) {
        let dash = match dash {
            TkDash::Dot => ".",
            TkDash::Dash => "-",
            TkDash::DashDot => "-.",
            TkDash::DashDotDot => "-..",
        };
        self.configure("dash", dash);
    }

    /// Width of line, in pixels.
    pub fn width(&self, value: u64) {
        self.configure("width", &value.to_string());
    }

    /// Location of arrow(s) on line.
    pub fn arrow(&self, value: TkArrowWhere) {
        let value = match value {
            TkArrowWhere::Both => "both",
            TkArrowWhere::First => "first",
            TkArrowWhere::Last => "last",
            TkArrowWhere::None => "none",
        };
        self.configure("arrow", value);
    }

    /// Shape of arrow(s) to draw: see Tk
    /// [manual](https://www.tcl-lang.org/man/tcl8.6/TkCmd/canvas.htm#M145).
    pub fn arrow_shape(&self, v1: u64, v2: u64, v3: u64) {
        let msg = format!("{{{} {} {} }}", v1, v2, v3);
        self.configure("arrowshape", &msg);
    }

    /// Style of ends of lines.
    pub fn cap_style(&self, value: TkCapStyle) {
        let value = match value {
            TkCapStyle::Butt => "butt",
            TkCapStyle::Projecting => "projecting",
            TkCapStyle::Round => "round",
        };
        self.configure("capstyle", value);
    }

    /// Style in which lines are connected.
    pub fn join_style(&self, value: TkJoinStyle) {
        let value = match value {
            TkJoinStyle::Bevel => "bevel",
            TkJoinStyle::Miter => "miter",
            TkJoinStyle::Round => "round",
        };
        self.configure("joinstyle", value);
    }
}

impl TkCanvasItem for TkCanvasOval {
    fn canvas(&self) -> &str {
        &self.canvas
    }

    fn id(&self) -> &str {
        &self.id
    }
}

impl TkCanvasTags for TkCanvasOval {}

impl TkCanvasOval {
    /// Pattern for drawing line.
    pub fn dash(&self, dash: TkDash) {
        let dash = match dash {
            TkDash::Dot => ".",
            TkDash::Dash => "-",
            TkDash::DashDot => "-.",
            TkDash::DashDotDot => "-..",
        };
        self.configure("dash", dash);
    }

    /// Fill colour for area.
    pub fn fill(&self, colour: &str) {
        self.configure("fill", colour);
    }

    /// Colour for outline.
    pub fn outline(&self, colour: &str) {
        self.configure("outline", colour);
    }

    /// Width of outline, in pixels.
    pub fn width(&self, value: u64) {
        self.configure("width", &value.to_string());
    }
}

impl TkCanvasItem for TkCanvasPolygon {
    fn canvas(&self) -> &str {
        &self.canvas
    }

    fn id(&self) -> &str {
        &self.id
    }
}

impl TkCanvasTags for TkCanvasPolygon {}

impl TkCanvasPolygon {
    /// Pattern for drawing line.
    pub fn dash(&self, dash: TkDash) {
        let dash = match dash {
            TkDash::Dot => ".",
            TkDash::Dash => "-",
            TkDash::DashDot => "-.",
            TkDash::DashDotDot => "-..",
        };
        self.configure("dash", dash);
    }

    /// Fill colour for area.
    pub fn fill(&self, colour: &str) {
        self.configure("fill", colour);
    }

    /// Colour for outline.
    pub fn outline(&self, colour: &str) {
        self.configure("outline", colour);
    }

    /// Width of outline, in pixels.
    pub fn width(&self, value: u64) {
        self.configure("width", &value.to_string());
    }

    /// Style in which lines are connected.
    pub fn join_style(&self, value: TkJoinStyle) {
        let value = match value {
            TkJoinStyle::Bevel => "bevel",
            TkJoinStyle::Miter => "miter",
            TkJoinStyle::Round => "round",
        };
        self.configure("joinstyle", value);
    }
}

impl TkCanvasItem for TkCanvasRectangle {
    fn canvas(&self) -> &str {
        &self.canvas
    }

    fn id(&self) -> &str {
        &self.id
    }
}

impl TkCanvasTags for TkCanvasRectangle {}

impl TkCanvasRectangle {
    /// Pattern for drawing line.
    pub fn dash(&self, dash: TkDash) {
        let dash = match dash {
            TkDash::Dot => ".",
            TkDash::Dash => "-",
            TkDash::DashDot => "-.",
            TkDash::DashDotDot => "-..",
        };
        self.configure("dash", dash);
    }

    /// Fill colour for area.
    pub fn fill(&self, colour: &str) {
        self.configure("fill", colour);
    }

    /// Colour for outline.
    pub fn outline(&self, colour: &str) {
        self.configure("outline", colour);
    }

    /// Width of outline, in pixels.
    pub fn width(&self, value: u64) {
        self.configure("width", &value.to_string());
    }
}

impl TkCanvasItem for TkCanvasText {
    fn canvas(&self) -> &str {
        &self.canvas
    }

    fn id(&self) -> &str {
        &self.id
    }
}

impl TkCanvasTags for TkCanvasText {}

impl TkCanvasText {
    /// Positioning of image with respect to internal margins.
    pub fn anchor(&self, value: widget::Anchor) {
        self.configure("anchor", &value.to_string());
    }

    /// Colour for line (same as Tk's "fill" option).
    pub fn colour(&self, colour: &str) {
        self.configure("fill", colour);
    }

    /// Angle of text - float in range 0 to 360 degrees.
    pub fn angle(&self, degrees: f64) {
        // - silently ensure value is in valid range
        let degrees = degrees.max(0.0);
        let degrees = degrees.min(360.0);
        self.configure("angle", &degrees.to_string());
    }

    /// Specifies the font to use for text.
    pub fn font(&self, definition: &str) {
        self.configure("font", definition);
    }

    /// Alignment of text within its bounding region.
    pub fn justify(&self, value: widget::Justify) {
        widget::configure(&self.id, "justify", &value.to_string());
    }

    /// Sets the text to display.
    pub fn text(&self, value: &str) {
        self.configure("text", value);
    }

    /// Underlines the character at the given index position.
    pub fn underline(&self, index: u64) {
        self.configure("underline", &index.to_string());
    }

    /// Sets the width of the text item, in pixels
    pub fn width(&self, value: i64) {
        self.configure("width", &value.to_string());
    }
}

impl TkCanvasItem for TkCanvasWidget {
    fn canvas(&self) -> &str {
        &self.canvas
    }

    fn id(&self) -> &str {
        &self.id
    }
}

impl TkCanvasTags for TkCanvasWidget {}

impl TkCanvasWidget {
    /// Positioning of widget with respect to internal margins.
    pub fn anchor(&self, value: widget::Anchor) {
        self.configure("anchor", &value.to_string());
    }
}
