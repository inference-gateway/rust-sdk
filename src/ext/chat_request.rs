use std::num::NonZeroU64;

use crate::generated::schemas::CreateChatCompletionRequest;

impl Default for CreateChatCompletionRequest {
    /// An empty request carrying the schema's documented default sampling
    /// parameters.
    ///
    /// The generated struct has no `Default` derive - typify cannot derive one
    /// for the enums in the schema set - so it is maintained here by hand. The
    /// non-`Option` fields mirror the `default:` values declared on
    /// `CreateChatCompletionRequest` in `openapi.yaml`; keep them in sync when
    /// the spec is regenerated.
    fn default() -> Self {
        Self {
            model: String::new(),
            messages: Vec::new(),
            frequency_penalty: 0.0,
            logit_bias: std::collections::HashMap::new(),
            logprobs: false,
            max_completion_tokens: None,
            max_tokens: None,
            n: NonZeroU64::MIN, // schema default: 1
            parallel_tool_calls: true,
            presence_penalty: 0.0,
            reasoning_effort: None,
            reasoning_format: None,
            response_format: None,
            seed: None,
            stop: None,
            stream: false,
            stream_options: None,
            temperature: 1.0,
            tool_choice: None,
            tools: Vec::new(),
            top_logprobs: None,
            top_p: 1.0,
            user: None,
        }
    }
}
