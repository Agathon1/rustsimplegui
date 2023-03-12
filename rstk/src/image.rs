//! Images - method to read in an image from file.
//!

use super::wish;

/// Refers to an image
#[derive(Clone, Debug, PartialEq)]
pub struct TkImage {
    pub id: String,
}

/// Reads an image from a given filename and returns the image reference.
pub fn read_image(filename: &str) -> TkImage {
    let id = wish::next_wid(".");
    let msg = format!("image create photo {} -file {}", id, filename);
    wish::tell_wish(&msg);

    TkImage { id }
}
