//! Ergonomics for the speech-generation request.
//!
//! Only the `Default` impl lives here - typify cannot derive one for the
//! struct (required fields with no schema defaults), so it mirrors
//! `chat_request.rs` and `image_request.rs` so callers set just the fields
//! they care about.

use crate::generated::schemas::{CreateSpeechRequest, CreateSpeechRequestResponseFormat};

impl Default for CreateSpeechRequest {
    /// Mirrors the schema defaults: `response_format` = `mp3`, `speed` = 1.
    fn default() -> Self {
        Self {
            model: String::new(),
            input: String::new()
                .parse()
                .expect("empty text always fits the 4096-character limit"),
            voice: String::new(),
            instructions: None,
            reference_audio: None,
            response_format: CreateSpeechRequestResponseFormat::Mp3,
            speed: 1.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_matches_schema_defaults() {
        let req = CreateSpeechRequest::default();
        assert_eq!(req.response_format, CreateSpeechRequestResponseFormat::Mp3);
        assert_eq!(req.speed, 1.0);
        assert!(req.instructions.is_none());
        assert!(req.reference_audio.is_none());
    }
}
