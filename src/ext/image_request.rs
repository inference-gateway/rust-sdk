//! Ergonomics for the image-generation request.
//!
//! Upstream declares `CreateImageRequest::quality` and `::size` as bare
//! strings (no `enum` in the spec), so typify generates `Option<String>` and
//! the struct gets no `Default`. These enums remove the magic strings at the
//! call site - convert with `.into()` when populating the request - and the
//! `Default` impl mirrors `chat_request.rs` so callers only set the fields
//! they care about. The long-term home for these enums is the schema in
//! `inference-gateway/schemas`; drop them once the spec types them.

use std::fmt;
use std::num::NonZeroU64;

use crate::generated::schemas::{CreateImageRequest, CreateImageRequestResponseFormat};

/// Quality of a generated image ([`CreateImageRequest::quality`]).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ImageQuality {
    Standard,
    Hd,
}

impl fmt::Display for ImageQuality {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Standard => "standard",
            Self::Hd => "hd",
        })
    }
}

impl From<ImageQuality> for String {
    fn from(value: ImageQuality) -> Self {
        value.to_string()
    }
}

/// Size of a generated image ([`CreateImageRequest::size`]).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ImageSize {
    Square256,
    Square512,
    Square1024,
    Portrait1024x1792,
    Landscape1792x1024,
}

impl fmt::Display for ImageSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Square256 => "256x256",
            Self::Square512 => "512x512",
            Self::Square1024 => "1024x1024",
            Self::Portrait1024x1792 => "1024x1792",
            Self::Landscape1792x1024 => "1792x1024",
        })
    }
}

impl From<ImageSize> for String {
    fn from(value: ImageSize) -> Self {
        value.to_string()
    }
}

impl Default for CreateImageRequest {
    /// The generated struct has no `Default` derive (typify cannot derive one
    /// for the enums in the schema set), so it is maintained here by hand.
    /// Mirrors the schema defaults: `n` = 1, `response_format` = `url`.
    fn default() -> Self {
        Self {
            model: None,
            n: NonZeroU64::MIN, // schema default: 1
            prompt: String::new(),
            quality: None,
            response_format: CreateImageRequestResponseFormat::Url,
            size: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enums_render_the_wire_strings() {
        assert_eq!(String::from(ImageQuality::Standard), "standard");
        assert_eq!(String::from(ImageQuality::Hd), "hd");
        assert_eq!(String::from(ImageSize::Square1024), "1024x1024");
        assert_eq!(String::from(ImageSize::Portrait1024x1792), "1024x1792");
    }

    #[test]
    fn default_matches_schema_defaults() {
        let req = CreateImageRequest::default();
        assert_eq!(req.n, NonZeroU64::MIN);
        assert_eq!(req.response_format, CreateImageRequestResponseFormat::Url);
        assert!(req.quality.is_none());
    }
}
