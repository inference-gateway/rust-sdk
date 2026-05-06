use crate::generated::schemas::ChatCompletionMessageToolCallFunction;

impl ChatCompletionMessageToolCallFunction {
    /// Deserialize the `arguments` JSON string into a typed value.
    ///
    /// The OpenAI-style schema represents tool-call arguments as a JSON-encoded
    /// string rather than a structured object, so callers must parse them.
    pub fn parse_arguments<T: serde::de::DeserializeOwned>(&self) -> Result<T, serde_json::Error> {
        serde_json::from_str(&self.arguments)
    }
}
