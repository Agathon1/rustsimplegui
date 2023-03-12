//! Font - support for defining and customising fonts.
//!
//! * also see the Tk [manual](https://www.tcl-lang.org/man/tcl8.6/TkCmd/font.htm)
//!
//! Tk provides a lot of control over font objects.
//!
//! Fonts are created using the struct, specifying those properties we want to 
//! set, and relying on its defaults for the remaining fields.
//! For example, creating a bold Helvetica font with size 12:
//!
//! ```
//! let font = rstk::TkFont { family: "Helvetica",
//!                             size: 12,
//!                             weight: rstk::Weight::Bold,
//!                             ..Default::default() // completes the remaining values
//! };
//! ```
//!
//! Fonts can be provided to many widgets which display text, e.g. a label:
//!
//! ```
//! let label = rstk::make_label(&root);
//! label.font(&font);
//! label.text("Label text");
//! ```
//!

use std::fmt;
use std::str;

use super::wish;

/// Defines possible weights for font: normal and bold.
/// See Tk [manual](https://www.tcl-lang.org/man/tcl8.6/TkCmd/font.htm#M27)
#[derive(Clone, Debug, PartialEq)]
pub enum Weight {
    Normal,
    Bold,
}

impl fmt::Display for Weight {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Weight::Normal => "normal",
            Weight::Bold => "bold",
        };
        write!(f, "{}", &value)
    }
}

impl Default for Weight {
    fn default() -> Self {
        Weight::Normal
    }
}

/// Defines possible slants for font: roman and italic.
/// See Tk [manual](https://www.tcl-lang.org/man/tcl8.6/TkCmd/font.htm#M28)
#[derive(Clone, Debug, PartialEq)]
pub enum Slant {
    Italic,
    Roman,
}

impl fmt::Display for Slant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Slant::Italic => "italic",
            Slant::Roman => "roman",
        };
        write!(f, "{}", &value)
    }
}

impl Default for Slant {
    fn default() -> Self {
        Slant::Roman
    }
}

/// Information on a font's metrics.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct TkFontMetrics {
    pub ascent: u64,
    pub descent: u64,
    pub line_space: u64,
    pub fixed: bool,
}

impl fmt::Display for TkFontMetrics {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut font = String::new();

        font.push_str(&format!("-ascent {} ", self.ascent));
        font.push_str(&format!("-descent {} ", self.descent));
        font.push_str(&format!("-linespace {} ", self.line_space));
        font.push_str(&format!("-fixed {} ", if self.fixed { "1" } else { "0" }));

        write!(f, "{}", &font)
    }
}

/// Error returned if expected font-metrics are incorrectly formatted.
#[derive(Debug)]
pub struct ParseFontMetricsErr;

impl str::FromStr for TkFontMetrics {
    type Err = ParseFontMetricsErr;

    /// Parse from the {-option value} representation for font metrics,
    /// as described in 5th point of Tk [manual](https://www.tcl-lang.org/man/tcl8.6/TkCmd/font.htm#M19)
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut font = TkFontMetrics {
            ..Default::default()
        };

        for part in s.split('-') {
            if part.starts_with("ascent") {
                if let Ok(size) = &part[7..].trim().parse::<u64>() {
                    font.ascent = *size;
                }
            } else if part.starts_with("descent") {
                if let Ok(size) = &part[8..].trim().parse::<u64>() {
                    font.descent = *size;
                }
            } else if part.starts_with("linespace") {
                if let Ok(size) = &part[10..].trim().parse::<u64>() {
                    font.line_space = *size;
                }
            } else if part.starts_with("fixed 1") {
                font.fixed = true;
            }
        }

        Ok(font)
    }
}

/// A font definition.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct TkFont {
    pub family: String,
    pub size: u64,
    pub weight: Weight,
    pub slant: Slant,
    pub underline: bool,
    pub overstrike: bool,
}

impl fmt::Display for TkFont {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut font = String::new();

        font.push_str(&format!("-family {{{}}} ", self.family));
        font.push_str(&format!("-size {} ", self.size));
        font.push_str(&format!("-weight {} ", self.weight));
        font.push_str(&format!("-slant {} ", self.slant));
        font.push_str(&format!(
            "-underline {} ",
            if self.underline { "1" } else { "0" }
        ));
        font.push_str(&format!(
            "-overstrike {} ",
            if self.overstrike { "1" } else { "0" }
        ));

        write!(f, "{}", &font)
    }
}

/// Error returned if expected font definition is incorrectly formatted.
#[derive(Debug)]
pub struct ParseFontErr;

impl str::FromStr for TkFont {
    type Err = ParseFontErr;

    /// Parse from the {-option value} representation for fonts,
    /// as described in 5th point of Tk [manual](https://www.tcl-lang.org/man/tcl8.6/TkCmd/font.htm#M13)
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut font = TkFont {
            ..Default::default()
        };

        for part in s.split('-') {
            if part.starts_with("family") {
                let mut family = String::from(&part[7..]);
                if let Some(index) = family.find('{') {
                    family.remove(index);
                }
                if let Some(index) = family.find('}') {
                    family.remove(index);
                }
                family = String::from(family.trim());
                font.family = family;
            } else if part.starts_with("size") {
                if let Ok(size) = &part[5..].trim().parse::<u64>() {
                    font.size = *size;
                }
            } else if part.starts_with("weight bold") {
                font.weight = Weight::Bold;
            } else if part.starts_with("slant italic") {
                font.slant = Slant::Italic;
            } else if part.starts_with("underline 1") {
                font.underline = true;
            } else if part.starts_with("overstrike 1") {
                font.overstrike = true;
            }
        }

        Ok(font)
    }
}

impl TkFont {
    /// Returns width in pixels of text if displayed with this font.
    pub fn measure(&self, text: &str) -> u64 {
        let msg = format!(
            "puts [font measure {{{}}} {{{}}}] ; flush stdout",
            self, text
        );
        let result = wish::ask_wish(&msg);
        if let Ok(value) = result.parse::<u64>() {
            value
        } else {
            // TODO can this fail?
            0
        }
    }

    /// Returns font's metrics
    pub fn metrics(&self) -> TkFontMetrics {
        let msg = format!("puts [font metrics {{{}}}] ; flush stdout", self);
        let result = wish::ask_wish(&msg);
        if let Ok(value) = result.parse::<TkFontMetrics>() {
            value
        } else {
            // TODO can this fail?
            TkFontMetrics {
                ..Default::default()
            }
        }
    }
}

// Returns a font definition obtained by reading font
// description for named font from wish.
fn font_from_name(name: &str) -> TkFont {
    let msg = format!("puts [font actual {}] ; flush stdout", name);
    let result = wish::ask_wish(&msg);

    // assume this cannot error
    result.parse::<TkFont>().unwrap_or(TkFont {
        ..Default::default()
    })
}

/// Retrieves copy of standard default font: see tk
/// [manual](https://www.tcl-lang.org/man/tcl8.6/TkCmd/font.htm#M32).
pub fn tk_default_font() -> TkFont {
    font_from_name("TkDefaultFont")
}

/// Retrieves copy of standard text font: see tk
/// [manual](https://www.tcl-lang.org/man/tcl8.6/TkCmd/font.htm#M33).
pub fn tk_text_font() -> TkFont {
    font_from_name("TkTextFont")
}

/// Retrieves copy of standard fixed font: see tk
/// [manual](https://www.tcl-lang.org/man/tcl8.6/TkCmd/font.htm#M34).
pub fn tk_fixed_font() -> TkFont {
    font_from_name("TkFixedFont")
}

/// Retrieves copy of standard menu font: see tk
/// [manual](https://www.tcl-lang.org/man/tcl8.6/TkCmd/font.htm#M35).
pub fn tk_menu_font() -> TkFont {
    font_from_name("TkMenuFont")
}

/// Retrieves copy of standard heading font: see tk
/// [manual](https://www.tcl-lang.org/man/tcl8.6/TkCmd/font.htm#M36).
pub fn tk_heading_font() -> TkFont {
    font_from_name("TkHeadingFont")
}

/// Retrieves copy of standard caption font: see tk
/// [manual](https://www.tcl-lang.org/man/tcl8.6/TkCmd/font.htm#M37).
pub fn tk_caption_font() -> TkFont {
    font_from_name("TkCaptionFont")
}

/// Retrieves copy of standard small-caption font: see tk
/// [manual](https://www.tcl-lang.org/man/tcl8.6/TkCmd/font.htm#M38).
pub fn tk_small_caption_font() -> TkFont {
    font_from_name("TkSmallCaptionFont")
}

/// Retrieves copy of standard icon font: see tk
/// [manual](https://www.tcl-lang.org/man/tcl8.6/TkCmd/font.htm#M39).
pub fn tk_icon_font() -> TkFont {
    font_from_name("TkIconFont")
}

/// Retrieves copy of standard tooltip font: see tk
/// [manual](https://www.tcl-lang.org/man/tcl8.6/TkCmd/font.htm#M40).
pub fn tk_tooltip_font() -> TkFont {
    font_from_name("TkTooltipFont")
}

/// Return list of font families available on current platform.
pub fn font_families() -> Vec<String> {
    let result = wish::ask_wish("puts [font families] ; flush stdout");
    wish::split_items(&result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn font_to_str() {
        let font = TkFont {
            family: String::from("Helvetica"),
            size: 14,
            ..Default::default()
        };
        let font_str = font.to_string();
        assert_eq!(
            "-family {Helvetica} -size 14 -weight normal -slant roman -underline 0 -overstrike 0 ",
            font_str
        );
    }

    #[test]
    fn font_to_str_2() {
        let font = TkFont {
            family: String::from("Helvetica Special"),
            size: 14,
            ..Default::default()
        };
        let font_str = font.to_string();
        assert_eq!("-family {Helvetica Special} -size 14 -weight normal -slant roman -underline 0 -overstrike 0 ",
                   font_str);
    }

    #[test]
    fn str_to_font() {
        let font_str =
            "-family {Helvetica} -size 14 -weight normal -slant italic -underline 0 -overstrike 1";
        let font = font_str.parse::<TkFont>().unwrap();
        assert_eq!(
            TkFont {
                family: String::from("Helvetica"),
                size: 14,
                slant: Slant::Italic,
                overstrike: true,
                ..Default::default()
            },
            font
        );
    }

    #[test]
    fn str_to_font_2() {
        let font_str = "-family {Helvetica Special} -size 14 -weight normal -slant italic -underline 0 -overstrike 1";
        let font = font_str.parse::<TkFont>().unwrap();
        assert_eq!(
            TkFont {
                family: String::from("Helvetica Special"),
                size: 14,
                slant: Slant::Italic,
                overstrike: true,
                ..Default::default()
            },
            font
        );
    }
}
