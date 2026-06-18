//! Hand-written impls layered on top of generated types.
//!
//! Generated types live in `crate::generated::schemas`. typify already emits
//! `Display`, `FromStr`, and `TryFrom<&str>` for enums, so this module only
//! holds behavior the schema cannot describe - argument parsing on tool-call
//! functions and a `Default` for the chat-completion request.

mod chat_request;
mod tool_call;
