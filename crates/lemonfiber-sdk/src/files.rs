use std::fmt;

/// The types a picture arrives as: raster images, which carry nothing a browser runs.
pub const PICTURE_TYPES: &[&str] = &[
    "image/jpeg",
    "image/png",
    "image/webp",
    "image/gif",
    "image/avif",
];

/// The most bytes a picture is.
pub const PICTURE_MOST: usize = 2 * 1024 * 1024;

/// A file lemonfiber handed over, kept as the bytes that arrived.
#[derive(Clone, PartialEq, Eq)]
pub struct File {
    /// Its bytes.
    pub content: Vec<u8>,
    /// The type it was labelled with, where it was labelled.
    pub content_type: Option<String>,
}

impl fmt::Debug for File {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "File({} bytes, {:?})",
            self.content.len(),
            self.content_type
        )
    }
}

/// One of a title's pictures: a raster image of at most [`PICTURE_MOST`] bytes, and its type.
#[derive(Clone, PartialEq, Eq)]
pub struct Picture {
    /// Its bytes.
    pub content: Vec<u8>,
    /// Its type, one of [`PICTURE_TYPES`].
    pub media_type: String,
}

impl fmt::Debug for Picture {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Picture({} bytes, {})",
            self.content.len(),
            self.media_type
        )
    }
}
