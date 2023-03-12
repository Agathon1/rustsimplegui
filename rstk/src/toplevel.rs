//! Toplevel widget - defines top-level and popup windows.
//!
//! * also see the Tk [manual](https://www.tcl-lang.org/man/tcl8.6/TkCmd/toplevel.htm)

use super::menu;
use super::widget;
use super::wish;

/// Refers to a top-level widget (window)
#[derive(Clone, Debug, PartialEq)]
pub struct TkTopLevel {
    pub id: String,
}

/// Creates an instance of a toplevel widget with given parent.
pub fn make_toplevel(parent: &impl widget::TkWidget) -> TkTopLevel {
    let id = wish::next_wid(parent.id());
    let msg = format!("toplevel {}", id);
    wish::tell_wish(&msg);

    TkTopLevel { id }
}

impl widget::TkWidget for TkTopLevel {
    /// Returns the widget's id reference - used within tk
    fn id(&self) -> &str {
        &self.id
    }
}

impl TkTopLevel {
    /// Specifies the background colour.
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

    /// De-iconify the window.
    pub fn deiconify(&self) {
        let msg = format!("wm deiconify {}", self.id);
        wish::tell_wish(&msg);
    }

    /// Expand window to occupy the full screen.
    pub fn full_screen(&self) {
        let msg = format!("wm attributes {} -fullscreen 1", self.id);
        wish::tell_wish(&msg);
    }

    /// Retrieves the geometry of the window as a tuple: (width, height, x, y).
    pub fn geometry_get(&self) -> (u64, u64, u64, u64) {
        let msg = format!("puts [wm geometry {}] ; flush stdout", self.id);
        let result = wish::ask_wish(&msg);

        string_geometry(&result)
    }

    /// Sets the size and position of a top-level window.
    ///
    /// * `height` - the vertical height of the window, in pixels
    /// * `width` - the horizontal width of the window, in pixels
    /// * `x` - a positive value gives position relative to _left_ edge of screen,
    ///         and a negative value gives position relative to _right_ edge.
    /// * `y` - a positive value gives position relative to _top_ edge of screen,
    ///         and a negative value gives position relative to _bottom_ edge.
    pub fn geometry(&self, width: u64, height: u64, x: i64, y: i64) {
        let msg = format!(
            "wm geometry {} {}x{}{}{}{}{}",
            self.id,
            width,
            height,
            if x < 0 { "-" } else { "+" },
            x.abs(),
            if y < 0 { "-" } else { "+" },
            y.abs()
        );
        wish::tell_wish(&msg);
    }

    /// Height of window, in rows.
    pub fn height(&self, height: u64) {
        widget::configure(&self.id, "height", &height.to_string());
    }

    /// Iconify the window.
    pub fn iconify(&self) {
        let msg = format!("wm iconify {}", self.id);
        wish::tell_wish(&msg);
    }

    /// Sets the maximum width/height in pixels for the window.
    pub fn maximum_size(&self, width: u64, height: u64) {
        let msg = format!("wm maxsize {} {} {}", self.id, width, height);
        wish::tell_wish(&msg);
    }

    /// Sets the menu of the top-level window.
    pub fn menu(&self, menu: &menu::TkMenu) {
        widget::configure(&self.id, "menu", &menu.id);
    }

    /// Sets the minimum width/height in pixels for the window.
    pub fn minimum_size(&self, width: u64, height: u64) {
        let msg = format!("wm minsize {} {} {}", self.id, width, height);
        wish::tell_wish(&msg);
    }

    /// Call given command on closing the window.
    pub fn on_close(&self, command: impl Fn() + Send + 'static) {
        wish::add_callback0(&self.id, wish::mk_callback0(command));
        let msg = format!(
            "wm protocol {} WM_DELETE_WINDOW {{ puts clicked-{} ; flush stdout }}",
            self.id, self.id
        );
        wish::tell_wish(&msg);
    }

    /// Amount of horizontal padding for widget.
    pub fn padx(&self, value: u64) {
        widget::configure(&self.id, "padx", &value.to_string());
    }

    /// Amount of vertical padding for widget.
    pub fn pady(&self, value: u64) {
        widget::configure(&self.id, "pady", &value.to_string());
    }

    /// Style of border around label.
    pub fn relief(&self, value: widget::Relief) {
        widget::configure(&self.id, "relief", &value.to_string());
    }

    /// Sets if window can be resized vertically or horizontally.
    pub fn resizable(&self, width: bool, height: bool) {
        let msg = format!(
            "wm resizable {} {} {}",
            self.id,
            if width { "1" } else { "0" },
            if height { "1" } else { "0" }
        );
        wish::tell_wish(&msg);
    }

    /// Sets the title text on a top-level window.
    pub fn title(&self, title: &str) {
        let msg = format!("wm title {} {{{}}}\n", self.id, title);
        wish::tell_wish(&msg);
    }

    /// Updates the display.
    pub fn update_idle_tasks(&self) {
        wish::tell_wish("update idletasks");
    }

    /// Width of window, in columns.
    pub fn width(&self, width: u64) {
        widget::configure(&self.id, "width", &width.to_string());
    }

    /// Withdraw the window.
    pub fn withdraw(&self) {
        let msg = format!("wm withdraw {}", self.id);
        wish::tell_wish(&msg);
    }
}

// Parse the widthxheight+x+y tcl string into a tuple: (width, height, x, y).
// -- we will ignore errors because this is only called on the return value 
// from a tcl call.
// -- return (0,0,0,0) if there is a problem
fn string_geometry(text: &str) -> (u64, u64, u64, u64) {
    let parts: Vec<&str> = text.split(&['x', '+'][..]).collect();
    let mut w = 0;
    let mut h = 0;
    let mut x = 0;
    let mut y = 0;
    if parts.len() == 4 {
        w = parts[0].parse::<u64>().unwrap_or(0);
        h = parts[1].parse::<u64>().unwrap_or(0);
        x = parts[2].parse::<u64>().unwrap_or(0);
        y = parts[3].parse::<u64>().unwrap_or(0);
    }
    (w, h, x, y)
}

#[cfg(test)]
mod tests {
    use super::string_geometry;

    #[test]
    fn geometry() {
        assert_eq!((0, 0, 0, 0), string_geometry("0x0+0+0"));
        assert_eq!((10, 20, 100, 200), string_geometry("10x20+100+200"));
        // - check some unlikely errors
        assert_eq!((0, 0, 0, 0), string_geometry("0x00+0"));
        assert_eq!((0, 0, 0, 0), string_geometry(""));
        assert_eq!((0, 0, 0, 0), string_geometry("axbxcxd"));
    }
}
