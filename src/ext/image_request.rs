//! Ergonomics for the image-generation request.
//!
//! The spec now types `CreateImageRequest::quality` and `::size`
//! (`CreateImageRequestQuality` and `ImageSize`), so the former hand-written
//! enums are gone - use the generated types directly. Only the `Default` impl
//! remains: typify cannot derive one for the enums in the schema set, so it is
//! maintained here and mirrors `chat_request.rs` so callers set just the fields
//! they care about.

use std::num::NonZeroU64;

use crate::generated::schemas::{CreateImageRequest, CreateImageRequestResponseFormat};

impl Default for CreateImageRequest {
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
    fn default_matches_schema_defaults() {
        let req = CreateImageRequest::default();
        assert_eq!(req.n, NonZeroU64::MIN);
        assert_eq!(req.response_format, CreateImageRequestResponseFormat::Url);
        assert!(req.quality.is_none());
    }
}
