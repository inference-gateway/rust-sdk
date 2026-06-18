// @generated - DO NOT EDIT.
// Regenerate with `task generate-types` (or `cargo run -p gen-types --release`).
// Source: openapi.yaml (components.schemas).

#![allow(clippy::all)]
#![allow(missing_docs)]
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

/// Error types.
pub mod error {
    /// Error from a `TryFrom` or `FromStr` implementation.
    pub struct ConversionError(::std::borrow::Cow<'static, str>);
    impl ::std::error::Error for ConversionError {}
    impl ::std::fmt::Display for ConversionError {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Display::fmt(&self.0, f)
        }
    }
    impl ::std::fmt::Debug for ConversionError {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Debug::fmt(&self.0, f)
        }
    }
    impl From<&'static str> for ConversionError {
        fn from(value: &'static str) -> Self {
            Self(value.into())
        }
    }
    impl From<String> for ConversionError {
        fn from(value: String) -> Self {
            Self(value.into())
        }
    }
}
///`ChatCompletionChoice`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "finish_reason",
///    "index",
///    "message"
///  ],
///  "properties": {
///    "finish_reason": {
///      "$ref": "#/definitions/FinishReason"
///    },
///    "index": {
///      "description": "The index of the choice in the list of choices.",
///      "type": "integer"
///    },
///    "logprobs": {
///      "description": "Log probability information for the choice.",
///      "type": "object",
///      "required": [
///        "content",
///        "refusal"
///      ],
///      "properties": {
///        "content": {
///          "description": "A list of message content tokens with log probability information.",
///          "type": "array",
///          "items": {
///            "$ref": "#/definitions/ChatCompletionTokenLogprob"
///          }
///        },
///        "refusal": {
///          "description": "A list of message refusal tokens with log probability information.",
///          "type": "array",
///          "items": {
///            "$ref": "#/definitions/ChatCompletionTokenLogprob"
///          }
///        }
///      },
///      "nullable": true
///    },
///    "message": {
///      "$ref": "#/definitions/Message"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct ChatCompletionChoice {
    pub finish_reason: FinishReason,
    ///The index of the choice in the list of choices.
    pub index: i64,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub logprobs: ::std::option::Option<ChatCompletionChoiceLogprobs>,
    pub message: Message,
}
///Log probability information for the choice.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Log probability information for the choice.",
///  "type": "object",
///  "required": [
///    "content",
///    "refusal"
///  ],
///  "properties": {
///    "content": {
///      "description": "A list of message content tokens with log probability information.",
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/ChatCompletionTokenLogprob"
///      }
///    },
///    "refusal": {
///      "description": "A list of message refusal tokens with log probability information.",
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/ChatCompletionTokenLogprob"
///      }
///    }
///  },
///  "nullable": true
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct ChatCompletionChoiceLogprobs {
    ///A list of message content tokens with log probability information.
    pub content: ::std::vec::Vec<ChatCompletionTokenLogprob>,
    ///A list of message refusal tokens with log probability information.
    pub refusal: ::std::vec::Vec<ChatCompletionTokenLogprob>,
}
///`ChatCompletionMessageToolCall`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "function",
///    "id",
///    "type"
///  ],
///  "properties": {
///    "extra_content": {
///      "$ref": "#/definitions/ToolCallExtraContent"
///    },
///    "function": {
///      "$ref": "#/definitions/ChatCompletionMessageToolCallFunction"
///    },
///    "id": {
///      "description": "The ID of the tool call.",
///      "type": "string"
///    },
///    "type": {
///      "$ref": "#/definitions/ChatCompletionToolType"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct ChatCompletionMessageToolCall {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub extra_content: ::std::option::Option<ToolCallExtraContent>,
    pub function: ChatCompletionMessageToolCallFunction,
    ///The ID of the tool call.
    pub id: ::std::string::String,
    #[serde(rename = "type")]
    pub type_: ChatCompletionToolType,
}
///`ChatCompletionMessageToolCallChunk`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "index"
///  ],
///  "properties": {
///    "extra_content": {
///      "$ref": "#/definitions/ToolCallExtraContent"
///    },
///    "function": {
///      "$ref": "#/definitions/ChatCompletionMessageToolCallFunction"
///    },
///    "id": {
///      "description": "The ID of the tool call.",
///      "type": "string"
///    },
///    "index": {
///      "type": "integer"
///    },
///    "type": {
///      "description": "The type of the tool. Currently, only `function` is supported.",
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct ChatCompletionMessageToolCallChunk {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub extra_content: ::std::option::Option<ToolCallExtraContent>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub function: ::std::option::Option<ChatCompletionMessageToolCallFunction>,
    ///The ID of the tool call.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    pub index: i64,
    ///The type of the tool. Currently, only `function` is supported.
    #[serde(
        rename = "type",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_: ::std::option::Option<::std::string::String>,
}
///The function that the model called.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "The function that the model called.",
///  "type": "object",
///  "required": [
///    "arguments",
///    "name"
///  ],
///  "properties": {
///    "arguments": {
///      "description": "The arguments to call the function with, as generated by the model in JSON format. Note that the model does not always generate valid JSON, and may hallucinate parameters not defined by your function schema. Validate the arguments in your code before calling your function.",
///      "type": "string"
///    },
///    "name": {
///      "description": "The name of the function to call.",
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct ChatCompletionMessageToolCallFunction {
    ///The arguments to call the function with, as generated by the model in JSON format. Note that the model does not always generate valid JSON, and may hallucinate parameters not defined by your function schema. Validate the arguments in your code before calling your function.
    pub arguments: ::std::string::String,
    ///The name of the function to call.
    pub name: ::std::string::String,
}
/**Specifies a tool the model should use. Use to force the model to call a specific function.
*/
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Specifies a tool the model should use. Use to force the model to call a specific function.\n",
///  "type": "object",
///  "required": [
///    "function",
///    "type"
///  ],
///  "properties": {
///    "function": {
///      "type": "object",
///      "required": [
///        "name"
///      ],
///      "properties": {
///        "name": {
///          "description": "The name of the function to call.",
///          "type": "string"
///        }
///      }
///    },
///    "type": {
///      "$ref": "#/definitions/ChatCompletionToolType"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct ChatCompletionNamedToolChoice {
    pub function: ChatCompletionNamedToolChoiceFunction,
    #[serde(rename = "type")]
    pub type_: ChatCompletionToolType,
}
///`ChatCompletionNamedToolChoiceFunction`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "name"
///  ],
///  "properties": {
///    "name": {
///      "description": "The name of the function to call.",
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct ChatCompletionNamedToolChoiceFunction {
    ///The name of the function to call.
    pub name: ::std::string::String,
}
///`ChatCompletionStreamChoice`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "delta",
///    "index"
///  ],
///  "properties": {
///    "delta": {
///      "$ref": "#/definitions/ChatCompletionStreamResponseDelta"
///    },
///    "finish_reason": {
///      "$ref": "#/definitions/FinishReason"
///    },
///    "index": {
///      "description": "The index of the choice in the list of choices.",
///      "type": "integer"
///    },
///    "logprobs": {
///      "description": "Log probability information for the choice.",
///      "type": "object",
///      "required": [
///        "content",
///        "refusal"
///      ],
///      "properties": {
///        "content": {
///          "description": "A list of message content tokens with log probability information.",
///          "type": "array",
///          "items": {
///            "$ref": "#/definitions/ChatCompletionTokenLogprob"
///          }
///        },
///        "refusal": {
///          "description": "A list of message refusal tokens with log probability information.",
///          "type": "array",
///          "items": {
///            "$ref": "#/definitions/ChatCompletionTokenLogprob"
///          }
///        }
///      }
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct ChatCompletionStreamChoice {
    pub delta: ChatCompletionStreamResponseDelta,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub finish_reason: ::std::option::Option<FinishReason>,
    ///The index of the choice in the list of choices.
    pub index: i64,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub logprobs: ::std::option::Option<ChatCompletionStreamChoiceLogprobs>,
}
///Log probability information for the choice.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Log probability information for the choice.",
///  "type": "object",
///  "required": [
///    "content",
///    "refusal"
///  ],
///  "properties": {
///    "content": {
///      "description": "A list of message content tokens with log probability information.",
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/ChatCompletionTokenLogprob"
///      }
///    },
///    "refusal": {
///      "description": "A list of message refusal tokens with log probability information.",
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/ChatCompletionTokenLogprob"
///      }
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct ChatCompletionStreamChoiceLogprobs {
    ///A list of message content tokens with log probability information.
    pub content: ::std::vec::Vec<ChatCompletionTokenLogprob>,
    ///A list of message refusal tokens with log probability information.
    pub refusal: ::std::vec::Vec<ChatCompletionTokenLogprob>,
}
/**Options for streaming response. Only set this when you set `stream: true`.
*/
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Options for streaming response. Only set this when you set `stream: true`.\n",
///  "type": "object",
///  "required": [
///    "include_usage"
///  ],
///  "properties": {
///    "include_usage": {
///      "description": "If set, an additional chunk will be streamed before the `data: [DONE]` message. The `usage` field on this chunk shows the token usage statistics for the entire request, and the `choices` field will always be an empty array. All other chunks will also include a `usage` field, but with a null value.\n",
///      "type": "boolean"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct ChatCompletionStreamOptions {
    /**If set, an additional chunk will be streamed before the `data: [DONE]` message. The `usage` field on this chunk shows the token usage statistics for the entire request, and the `choices` field will always be an empty array. All other chunks will also include a `usage` field, but with a null value.
     */
    pub include_usage: bool,
}
///A chat completion delta generated by streamed model responses.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "A chat completion delta generated by streamed model responses.",
///  "type": "object",
///  "properties": {
///    "content": {
///      "description": "The contents of the chunk message.",
///      "type": "string"
///    },
///    "reasoning": {
///      "description": "The reasoning of the chunk message. Same as reasoning_content.",
///      "type": "string"
///    },
///    "reasoning_content": {
///      "description": "The reasoning content of the chunk message.",
///      "type": "string"
///    },
///    "refusal": {
///      "description": "The refusal message generated by the model.",
///      "type": "string"
///    },
///    "role": {
///      "$ref": "#/definitions/MessageRole"
///    },
///    "tool_calls": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/ChatCompletionMessageToolCallChunk"
///      }
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct ChatCompletionStreamResponseDelta {
    ///The contents of the chunk message.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub content: ::std::option::Option<::std::string::String>,
    ///The reasoning of the chunk message. Same as reasoning_content.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub reasoning: ::std::option::Option<::std::string::String>,
    ///The reasoning content of the chunk message.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub reasoning_content: ::std::option::Option<::std::string::String>,
    ///The refusal message generated by the model.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub refusal: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub role: ::std::option::Option<MessageRole>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub tool_calls: ::std::vec::Vec<ChatCompletionMessageToolCallChunk>,
}
impl ::std::default::Default for ChatCompletionStreamResponseDelta {
    fn default() -> Self {
        Self {
            content: Default::default(),
            reasoning: Default::default(),
            reasoning_content: Default::default(),
            refusal: Default::default(),
            role: Default::default(),
            tool_calls: Default::default(),
        }
    }
}
///`ChatCompletionTokenLogprob`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "bytes",
///    "logprob",
///    "token",
///    "top_logprobs"
///  ],
///  "properties": {
///    "bytes": {
///      "description": "A list of integers representing the UTF-8 bytes representation of the token. Useful in instances where characters are represented by multiple tokens and their byte representations must be combined to generate the correct text representation. Can be `null` if there is no bytes representation for the token.",
///      "type": "array",
///      "items": {
///        "type": "integer"
///      }
///    },
///    "logprob": {
///      "description": "The log probability of this token, if it is within the top 20 most likely tokens. Otherwise, the value `-9999.0` is used to signify that the token is very unlikely.",
///      "type": "number"
///    },
///    "token": {
///      "description": "The token.",
///      "type": "string"
///    },
///    "top_logprobs": {
///      "description": "List of the most likely tokens and their log probability, at this token position. In rare cases, there may be fewer than the number of requested `top_logprobs` returned.",
///      "type": "array",
///      "items": {
///        "type": "object",
///        "required": [
///          "bytes",
///          "logprob",
///          "token"
///        ],
///        "properties": {
///          "bytes": {
///            "description": "A list of integers representing the UTF-8 bytes representation of the token. Useful in instances where characters are represented by multiple tokens and their byte representations must be combined to generate the correct text representation. Can be `null` if there is no bytes representation for the token.",
///            "type": "array",
///            "items": {
///              "type": "integer"
///            }
///          },
///          "logprob": {
///            "description": "The log probability of this token, if it is within the top 20 most likely tokens. Otherwise, the value `-9999.0` is used to signify that the token is very unlikely.",
///            "type": "number"
///          },
///          "token": {
///            "description": "The token.",
///            "type": "string"
///          }
///        }
///      }
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct ChatCompletionTokenLogprob {
    ///A list of integers representing the UTF-8 bytes representation of the token. Useful in instances where characters are represented by multiple tokens and their byte representations must be combined to generate the correct text representation. Can be `null` if there is no bytes representation for the token.
    pub bytes: ::std::vec::Vec<i64>,
    ///The log probability of this token, if it is within the top 20 most likely tokens. Otherwise, the value `-9999.0` is used to signify that the token is very unlikely.
    pub logprob: f64,
    ///The token.
    pub token: ::std::string::String,
    ///List of the most likely tokens and their log probability, at this token position. In rare cases, there may be fewer than the number of requested `top_logprobs` returned.
    pub top_logprobs: ::std::vec::Vec<ChatCompletionTokenLogprobTopLogprobsItem>,
}
///`ChatCompletionTokenLogprobTopLogprobsItem`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "bytes",
///    "logprob",
///    "token"
///  ],
///  "properties": {
///    "bytes": {
///      "description": "A list of integers representing the UTF-8 bytes representation of the token. Useful in instances where characters are represented by multiple tokens and their byte representations must be combined to generate the correct text representation. Can be `null` if there is no bytes representation for the token.",
///      "type": "array",
///      "items": {
///        "type": "integer"
///      }
///    },
///    "logprob": {
///      "description": "The log probability of this token, if it is within the top 20 most likely tokens. Otherwise, the value `-9999.0` is used to signify that the token is very unlikely.",
///      "type": "number"
///    },
///    "token": {
///      "description": "The token.",
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct ChatCompletionTokenLogprobTopLogprobsItem {
    ///A list of integers representing the UTF-8 bytes representation of the token. Useful in instances where characters are represented by multiple tokens and their byte representations must be combined to generate the correct text representation. Can be `null` if there is no bytes representation for the token.
    pub bytes: ::std::vec::Vec<i64>,
    ///The log probability of this token, if it is within the top 20 most likely tokens. Otherwise, the value `-9999.0` is used to signify that the token is very unlikely.
    pub logprob: f64,
    ///The token.
    pub token: ::std::string::String,
}
///`ChatCompletionTool`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "function",
///    "type"
///  ],
///  "properties": {
///    "function": {
///      "$ref": "#/definitions/FunctionObject"
///    },
///    "type": {
///      "$ref": "#/definitions/ChatCompletionToolType"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct ChatCompletionTool {
    pub function: FunctionObject,
    #[serde(rename = "type")]
    pub type_: ChatCompletionToolType,
}
/**Controls which (if any) tool is called by the model. `none` means the model will not call any tool and instead generates a message. `auto` means the model can pick between generating a message or calling one or more tools. `required` means the model must call one or more tools. Specifying a particular tool via `{"type": "function", "function": {"name": "my_function"}}` forces the model to call that tool.
`none` is the default when no tools are present. `auto` is the default if tools are present.
*/
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Controls which (if any) tool is called by the model. `none` means the model will not call any tool and instead generates a message. `auto` means the model can pick between generating a message or calling one or more tools. `required` means the model must call one or more tools. Specifying a particular tool via `{\"type\": \"function\", \"function\": {\"name\": \"my_function\"}}` forces the model to call that tool.\n`none` is the default when no tools are present. `auto` is the default if tools are present.\n",
///  "oneOf": [
///    {
///      "description": "`none` means the model will not call any tool and instead generates a message. `auto` means the model can pick between generating a message or calling one or more tools. `required` means the model must call one or more tools.\n",
///      "type": "string",
///      "enum": [
///        "none",
///        "auto",
///        "required"
///      ]
///    },
///    {
///      "$ref": "#/definitions/ChatCompletionNamedToolChoice"
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum ChatCompletionToolChoiceOption {
    String(ChatCompletionToolChoiceOptionString),
    ChatCompletionNamedToolChoice(ChatCompletionNamedToolChoice),
}
impl ::std::convert::From<ChatCompletionToolChoiceOptionString> for ChatCompletionToolChoiceOption {
    fn from(value: ChatCompletionToolChoiceOptionString) -> Self {
        Self::String(value)
    }
}
impl ::std::convert::From<ChatCompletionNamedToolChoice> for ChatCompletionToolChoiceOption {
    fn from(value: ChatCompletionNamedToolChoice) -> Self {
        Self::ChatCompletionNamedToolChoice(value)
    }
}
/**`none` means the model will not call any tool and instead generates a message. `auto` means the model can pick between generating a message or calling one or more tools. `required` means the model must call one or more tools.
*/
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "`none` means the model will not call any tool and instead generates a message. `auto` means the model can pick between generating a message or calling one or more tools. `required` means the model must call one or more tools.\n",
///  "type": "string",
///  "enum": [
///    "none",
///    "auto",
///    "required"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ChatCompletionToolChoiceOptionString {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "required")]
    Required,
}
impl ::std::fmt::Display for ChatCompletionToolChoiceOptionString {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::None => f.write_str("none"),
            Self::Auto => f.write_str("auto"),
            Self::Required => f.write_str("required"),
        }
    }
}
impl ::std::str::FromStr for ChatCompletionToolChoiceOptionString {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "none" => Ok(Self::None),
            "auto" => Ok(Self::Auto),
            "required" => Ok(Self::Required),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ChatCompletionToolChoiceOptionString {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ChatCompletionToolChoiceOptionString {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ChatCompletionToolChoiceOptionString {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///The type of the tool. Currently, only `function` is supported.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "The type of the tool. Currently, only `function` is supported.",
///  "type": "string",
///  "enum": [
///    "function"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ChatCompletionToolType {
    #[serde(rename = "function")]
    Function,
}
impl ::std::fmt::Display for ChatCompletionToolType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Function => f.write_str("function"),
        }
    }
}
impl ::std::str::FromStr for ChatCompletionToolType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "function" => Ok(Self::Function),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ChatCompletionToolType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ChatCompletionToolType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ChatCompletionToolType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///Usage statistics for the completion request.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Usage statistics for the completion request.",
///  "type": "object",
///  "required": [
///    "completion_tokens",
///    "prompt_tokens",
///    "total_tokens"
///  ],
///  "properties": {
///    "completion_tokens": {
///      "description": "Number of tokens in the generated completion.",
///      "default": 0,
///      "type": "integer",
///      "format": "int64"
///    },
///    "prompt_tokens": {
///      "description": "Number of tokens in the prompt.",
///      "default": 0,
///      "type": "integer",
///      "format": "int64"
///    },
///    "total_tokens": {
///      "description": "Total number of tokens used in the request (prompt + completion).",
///      "default": 0,
///      "type": "integer",
///      "format": "int64"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct CompletionUsage {
    ///Number of tokens in the generated completion.
    pub completion_tokens: i64,
    ///Number of tokens in the prompt.
    pub prompt_tokens: i64,
    ///Total number of tokens used in the request (prompt + completion).
    pub total_tokens: i64,
}
///`Config`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct Config(pub ::serde_json::Value);
impl ::std::ops::Deref for Config {
    type Target = ::serde_json::Value;
    fn deref(&self) -> &::serde_json::Value {
        &self.0
    }
}
impl ::std::convert::From<Config> for ::serde_json::Value {
    fn from(value: Config) -> Self {
        value.0
    }
}
impl ::std::convert::From<::serde_json::Value> for Config {
    fn from(value: ::serde_json::Value) -> Self {
        Self(value)
    }
}
///A content part within a multimodal message
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "A content part within a multimodal message",
///  "type": "object",
///  "oneOf": [
///    {
///      "$ref": "#/definitions/TextContentPart"
///    },
///    {
///      "$ref": "#/definitions/ImageContentPart"
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum ContentPart {
    TextContentPart(TextContentPart),
    ImageContentPart(ImageContentPart),
}
impl ::std::convert::From<TextContentPart> for ContentPart {
    fn from(value: TextContentPart) -> Self {
        Self::TextContentPart(value)
    }
}
impl ::std::convert::From<ImageContentPart> for ContentPart {
    fn from(value: ImageContentPart) -> Self {
        Self::ImageContentPart(value)
    }
}
///`CreateChatCompletionRequest`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "messages",
///    "model"
///  ],
///  "properties": {
///    "frequency_penalty": {
///      "description": "Number between -2.0 and 2.0. Positive values penalize new tokens based on their existing frequency in the text so far, decreasing the model's likelihood to repeat the same line verbatim.\n",
///      "default": 0,
///      "type": "number",
///      "maximum": 2.0,
///      "minimum": -2.0
///    },
///    "logit_bias": {
///      "description": "Modify the likelihood of specified tokens appearing in the completion. Accepts a JSON object that maps tokens (specified by their token ID in the tokenizer) to an associated bias value from -100 to 100. The bias is added to the logits generated by the model prior to sampling.\n",
///      "type": "object",
///      "additionalProperties": {
///        "type": "integer"
///      }
///    },
///    "logprobs": {
///      "description": "Whether to return log probabilities of the output tokens or not. If true, returns the log probabilities of each output token returned in the `content` of `message`.\n",
///      "default": false,
///      "type": "boolean"
///    },
///    "max_completion_tokens": {
///      "description": "An upper bound for the number of tokens that can be generated for a completion, including visible output tokens and reasoning tokens.\n",
///      "type": "integer"
///    },
///    "max_tokens": {
///      "description": "The maximum number of tokens that can be generated in the chat completion. This value can be used to control costs for text generated via API. This value is now deprecated in favor of `max_completion_tokens`, and is not compatible with o-series models.\n",
///      "deprecated": true,
///      "type": "integer"
///    },
///    "messages": {
///      "description": "A list of messages comprising the conversation so far.\n",
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/Message"
///      },
///      "minItems": 1
///    },
///    "model": {
///      "description": "Model ID to use",
///      "type": "string"
///    },
///    "n": {
///      "description": "How many chat completion choices to generate for each input message.\n",
///      "default": 1,
///      "type": "integer",
///      "maximum": 128.0,
///      "minimum": 1.0
///    },
///    "parallel_tool_calls": {
///      "description": "Whether to enable parallel function calling during tool use.\n",
///      "default": true,
///      "type": "boolean"
///    },
///    "presence_penalty": {
///      "description": "Number between -2.0 and 2.0. Positive values penalize new tokens based on whether they appear in the text so far, increasing the model's likelihood to talk about new topics.\n",
///      "default": 0,
///      "type": "number",
///      "maximum": 2.0,
///      "minimum": -2.0
///    },
///    "reasoning_effort": {
///      "description": "Constrains effort on reasoning for reasoning models. Currently supported values are `minimal`, `low`, `medium`, and `high`. Reducing reasoning effort can result in faster responses and fewer tokens used on reasoning in a response.\n",
///      "type": "string",
///      "enum": [
///        "minimal",
///        "low",
///        "medium",
///        "high"
///      ]
///    },
///    "reasoning_format": {
///      "description": "The format of the reasoning content. Can be `raw` or `parsed`.\nWhen specified as raw some reasoning models will output <think /> tags. When specified as parsed the model will output the reasoning under `reasoning` or `reasoning_content` attribute.\n",
///      "type": "string"
///    },
///    "response_format": {
///      "description": "An object specifying the format that the model must output. Setting to `{ \"type\": \"json_schema\", \"json_schema\": {...} }` enables Structured Outputs which guarantees the model will match your supplied JSON schema. Setting to `{ \"type\": \"json_object\" }` enables the older JSON mode, which ensures the message the model generates is valid JSON.\n",
///      "oneOf": [
///        {
///          "$ref": "#/definitions/ResponseFormatText"
///        },
///        {
///          "$ref": "#/definitions/ResponseFormatJsonSchema"
///        },
///        {
///          "$ref": "#/definitions/ResponseFormatJsonObject"
///        }
///      ]
///    },
///    "seed": {
///      "description": "If specified, our system will make a best effort to sample deterministically, such that repeated requests with the same `seed` and parameters should return the same result. Determinism is not guaranteed, and you should refer to the `system_fingerprint` response parameter to monitor changes in the backend.\n",
///      "type": "integer"
///    },
///    "stop": {
///      "description": "Up to 4 sequences where the API will stop generating further tokens.\n",
///      "oneOf": [
///        {
///          "type": "string"
///        },
///        {
///          "type": "array",
///          "items": {
///            "type": "string"
///          },
///          "maxItems": 4,
///          "minItems": 1
///        }
///      ]
///    },
///    "stream": {
///      "description": "If set to true, the model response data will be streamed to the client as it is generated using [server-sent events](https://developer.mozilla.org/en-US/docs/Web/API/Server-sent_events/Using_server-sent_events#Event_stream_format).\n",
///      "default": false,
///      "type": "boolean"
///    },
///    "stream_options": {
///      "$ref": "#/definitions/ChatCompletionStreamOptions"
///    },
///    "temperature": {
///      "description": "What sampling temperature to use, between 0 and 2. Higher values like 0.8 will make the output more random, while lower values like 0.2 will make it more focused and deterministic.\n",
///      "default": 1,
///      "type": "number",
///      "maximum": 2.0,
///      "minimum": 0.0
///    },
///    "tool_choice": {
///      "$ref": "#/definitions/ChatCompletionToolChoiceOption"
///    },
///    "tools": {
///      "description": "A list of tools the model may call. Currently, only functions are supported as a tool. Use this to provide a list of functions the model may generate JSON inputs for. A max of 128 functions are supported.\n",
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/ChatCompletionTool"
///      }
///    },
///    "top_logprobs": {
///      "description": "An integer between 0 and 20 specifying the number of most likely tokens to return at each token position, each with an associated log probability. `logprobs` must be set to `true` if this parameter is used.\n",
///      "type": "integer",
///      "maximum": 20.0,
///      "minimum": 0.0
///    },
///    "top_p": {
///      "description": "An alternative to sampling with temperature, called nucleus sampling, where the model considers the results of the tokens with top_p probability mass.\n",
///      "default": 1,
///      "type": "number",
///      "maximum": 1.0,
///      "minimum": 0.0
///    },
///    "user": {
///      "description": "A unique identifier representing your end-user, which can help to monitor and detect abuse.\n",
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct CreateChatCompletionRequest {
    /**Number between -2.0 and 2.0. Positive values penalize new tokens based on their existing frequency in the text so far, decreasing the model's likelihood to repeat the same line verbatim.
     */
    #[serde(default = "defaults::create_chat_completion_request_frequency_penalty")]
    pub frequency_penalty: f64,
    /**Modify the likelihood of specified tokens appearing in the completion. Accepts a JSON object that maps tokens (specified by their token ID in the tokenizer) to an associated bias value from -100 to 100. The bias is added to the logits generated by the model prior to sampling.
     */
    #[serde(
        default,
        skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
    )]
    pub logit_bias: ::std::collections::HashMap<::std::string::String, i64>,
    /**Whether to return log probabilities of the output tokens or not. If true, returns the log probabilities of each output token returned in the `content` of `message`.
     */
    #[serde(default)]
    pub logprobs: bool,
    /**An upper bound for the number of tokens that can be generated for a completion, including visible output tokens and reasoning tokens.
     */
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub max_completion_tokens: ::std::option::Option<i64>,
    /**The maximum number of tokens that can be generated in the chat completion. This value can be used to control costs for text generated via API. This value is now deprecated in favor of `max_completion_tokens`, and is not compatible with o-series models.
     */
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub max_tokens: ::std::option::Option<i64>,
    /**A list of messages comprising the conversation so far.
     */
    pub messages: ::std::vec::Vec<Message>,
    ///Model ID to use
    pub model: ::std::string::String,
    /**How many chat completion choices to generate for each input message.
     */
    #[serde(default = "defaults::default_nzu64::<::std::num::NonZeroU64, 1>")]
    pub n: ::std::num::NonZeroU64,
    /**Whether to enable parallel function calling during tool use.
     */
    #[serde(default = "defaults::default_bool::<true>")]
    pub parallel_tool_calls: bool,
    /**Number between -2.0 and 2.0. Positive values penalize new tokens based on whether they appear in the text so far, increasing the model's likelihood to talk about new topics.
     */
    #[serde(default = "defaults::create_chat_completion_request_presence_penalty")]
    pub presence_penalty: f64,
    /**Constrains effort on reasoning for reasoning models. Currently supported values are `minimal`, `low`, `medium`, and `high`. Reducing reasoning effort can result in faster responses and fewer tokens used on reasoning in a response.
     */
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub reasoning_effort: ::std::option::Option<CreateChatCompletionRequestReasoningEffort>,
    /**The format of the reasoning content. Can be `raw` or `parsed`.
    When specified as raw some reasoning models will output <think /> tags. When specified as parsed the model will output the reasoning under `reasoning` or `reasoning_content` attribute.
    */
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub reasoning_format: ::std::option::Option<::std::string::String>,
    /**An object specifying the format that the model must output. Setting to `{ "type": "json_schema", "json_schema": {...} }` enables Structured Outputs which guarantees the model will match your supplied JSON schema. Setting to `{ "type": "json_object" }` enables the older JSON mode, which ensures the message the model generates is valid JSON.
     */
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub response_format: ::std::option::Option<CreateChatCompletionRequestResponseFormat>,
    /**If specified, our system will make a best effort to sample deterministically, such that repeated requests with the same `seed` and parameters should return the same result. Determinism is not guaranteed, and you should refer to the `system_fingerprint` response parameter to monitor changes in the backend.
     */
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub seed: ::std::option::Option<i64>,
    /**Up to 4 sequences where the API will stop generating further tokens.
     */
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub stop: ::std::option::Option<CreateChatCompletionRequestStop>,
    /**If set to true, the model response data will be streamed to the client as it is generated using [server-sent events](https://developer.mozilla.org/en-US/docs/Web/API/Server-sent_events/Using_server-sent_events#Event_stream_format).
     */
    #[serde(default)]
    pub stream: bool,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub stream_options: ::std::option::Option<ChatCompletionStreamOptions>,
    /**What sampling temperature to use, between 0 and 2. Higher values like 0.8 will make the output more random, while lower values like 0.2 will make it more focused and deterministic.
     */
    #[serde(default = "defaults::create_chat_completion_request_temperature")]
    pub temperature: f64,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub tool_choice: ::std::option::Option<ChatCompletionToolChoiceOption>,
    /**A list of tools the model may call. Currently, only functions are supported as a tool. Use this to provide a list of functions the model may generate JSON inputs for. A max of 128 functions are supported.
     */
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub tools: ::std::vec::Vec<ChatCompletionTool>,
    /**An integer between 0 and 20 specifying the number of most likely tokens to return at each token position, each with an associated log probability. `logprobs` must be set to `true` if this parameter is used.
     */
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub top_logprobs: ::std::option::Option<i64>,
    /**An alternative to sampling with temperature, called nucleus sampling, where the model considers the results of the tokens with top_p probability mass.
     */
    #[serde(default = "defaults::create_chat_completion_request_top_p")]
    pub top_p: f64,
    /**A unique identifier representing your end-user, which can help to monitor and detect abuse.
     */
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub user: ::std::option::Option<::std::string::String>,
}
/**Constrains effort on reasoning for reasoning models. Currently supported values are `minimal`, `low`, `medium`, and `high`. Reducing reasoning effort can result in faster responses and fewer tokens used on reasoning in a response.
*/
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Constrains effort on reasoning for reasoning models. Currently supported values are `minimal`, `low`, `medium`, and `high`. Reducing reasoning effort can result in faster responses and fewer tokens used on reasoning in a response.\n",
///  "type": "string",
///  "enum": [
///    "minimal",
///    "low",
///    "medium",
///    "high"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum CreateChatCompletionRequestReasoningEffort {
    #[serde(rename = "minimal")]
    Minimal,
    #[serde(rename = "low")]
    Low,
    #[serde(rename = "medium")]
    Medium,
    #[serde(rename = "high")]
    High,
}
impl ::std::fmt::Display for CreateChatCompletionRequestReasoningEffort {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Minimal => f.write_str("minimal"),
            Self::Low => f.write_str("low"),
            Self::Medium => f.write_str("medium"),
            Self::High => f.write_str("high"),
        }
    }
}
impl ::std::str::FromStr for CreateChatCompletionRequestReasoningEffort {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "minimal" => Ok(Self::Minimal),
            "low" => Ok(Self::Low),
            "medium" => Ok(Self::Medium),
            "high" => Ok(Self::High),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CreateChatCompletionRequestReasoningEffort {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
    for CreateChatCompletionRequestReasoningEffort
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CreateChatCompletionRequestReasoningEffort {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
/**An object specifying the format that the model must output. Setting to `{ "type": "json_schema", "json_schema": {...} }` enables Structured Outputs which guarantees the model will match your supplied JSON schema. Setting to `{ "type": "json_object" }` enables the older JSON mode, which ensures the message the model generates is valid JSON.
*/
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "An object specifying the format that the model must output. Setting to `{ \"type\": \"json_schema\", \"json_schema\": {...} }` enables Structured Outputs which guarantees the model will match your supplied JSON schema. Setting to `{ \"type\": \"json_object\" }` enables the older JSON mode, which ensures the message the model generates is valid JSON.\n",
///  "oneOf": [
///    {
///      "$ref": "#/definitions/ResponseFormatText"
///    },
///    {
///      "$ref": "#/definitions/ResponseFormatJsonSchema"
///    },
///    {
///      "$ref": "#/definitions/ResponseFormatJsonObject"
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum CreateChatCompletionRequestResponseFormat {
    Text(ResponseFormatText),
    JsonSchema(ResponseFormatJsonSchema),
    JsonObject(ResponseFormatJsonObject),
}
impl ::std::convert::From<ResponseFormatText> for CreateChatCompletionRequestResponseFormat {
    fn from(value: ResponseFormatText) -> Self {
        Self::Text(value)
    }
}
impl ::std::convert::From<ResponseFormatJsonSchema> for CreateChatCompletionRequestResponseFormat {
    fn from(value: ResponseFormatJsonSchema) -> Self {
        Self::JsonSchema(value)
    }
}
impl ::std::convert::From<ResponseFormatJsonObject> for CreateChatCompletionRequestResponseFormat {
    fn from(value: ResponseFormatJsonObject) -> Self {
        Self::JsonObject(value)
    }
}
/**Up to 4 sequences where the API will stop generating further tokens.
*/
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Up to 4 sequences where the API will stop generating further tokens.\n",
///  "oneOf": [
///    {
///      "type": "string"
///    },
///    {
///      "type": "array",
///      "items": {
///        "type": "string"
///      },
///      "maxItems": 4,
///      "minItems": 1
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum CreateChatCompletionRequestStop {
    String(::std::string::String),
    Array(::std::vec::Vec<::std::string::String>),
}
impl ::std::convert::From<::std::vec::Vec<::std::string::String>>
    for CreateChatCompletionRequestStop
{
    fn from(value: ::std::vec::Vec<::std::string::String>) -> Self {
        Self::Array(value)
    }
}
///Represents a chat completion response returned by model, based on the provided input.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Represents a chat completion response returned by model, based on the provided input.",
///  "type": "object",
///  "required": [
///    "choices",
///    "created",
///    "id",
///    "model",
///    "object"
///  ],
///  "properties": {
///    "choices": {
///      "description": "A list of chat completion choices. Can be more than one if `n` is greater than 1.",
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/ChatCompletionChoice"
///      }
///    },
///    "created": {
///      "description": "The Unix timestamp (in seconds) of when the chat completion was created.",
///      "type": "integer"
///    },
///    "id": {
///      "description": "A unique identifier for the chat completion.",
///      "type": "string"
///    },
///    "model": {
///      "description": "The model used for the chat completion.",
///      "type": "string"
///    },
///    "object": {
///      "description": "The object type, which is always `chat.completion`.",
///      "type": "string"
///    },
///    "usage": {
///      "$ref": "#/definitions/CompletionUsage"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct CreateChatCompletionResponse {
    ///A list of chat completion choices. Can be more than one if `n` is greater than 1.
    pub choices: ::std::vec::Vec<ChatCompletionChoice>,
    ///The Unix timestamp (in seconds) of when the chat completion was created.
    pub created: i64,
    ///A unique identifier for the chat completion.
    pub id: ::std::string::String,
    ///The model used for the chat completion.
    pub model: ::std::string::String,
    ///The object type, which is always `chat.completion`.
    pub object: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub usage: ::std::option::Option<CompletionUsage>,
}
/**Represents a streamed chunk of a chat completion response returned
by the model, based on the provided input.
*/
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Represents a streamed chunk of a chat completion response returned\nby the model, based on the provided input.\n",
///  "type": "object",
///  "required": [
///    "choices",
///    "created",
///    "id",
///    "model",
///    "object"
///  ],
///  "properties": {
///    "choices": {
///      "description": "A list of chat completion choices. Can contain more than one elements if `n` is greater than 1. Can also be empty for the\nlast chunk if you set `stream_options: {\"include_usage\": true}`.\n",
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/ChatCompletionStreamChoice"
///      }
///    },
///    "created": {
///      "description": "The Unix timestamp (in seconds) of when the chat completion was created. Each chunk has the same timestamp.",
///      "type": "integer"
///    },
///    "id": {
///      "description": "A unique identifier for the chat completion. Each chunk has the same ID.",
///      "type": "string"
///    },
///    "model": {
///      "description": "The model to generate the completion.",
///      "type": "string"
///    },
///    "object": {
///      "description": "The object type, which is always `chat.completion.chunk`.",
///      "type": "string"
///    },
///    "reasoning_format": {
///      "description": "The format of the reasoning content. Can be `raw` or `parsed`.\nWhen specified as raw some reasoning models will output <think /> tags. When specified as parsed the model will output the reasoning under reasoning_content.\n",
///      "type": "string"
///    },
///    "system_fingerprint": {
///      "description": "This fingerprint represents the backend configuration that the model runs with.\nCan be used in conjunction with the `seed` request parameter to understand when backend changes have been made that might impact determinism.\n",
///      "type": "string"
///    },
///    "usage": {
///      "$ref": "#/definitions/CompletionUsage"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct CreateChatCompletionStreamResponse {
    /**A list of chat completion choices. Can contain more than one elements if `n` is greater than 1. Can also be empty for the
    last chunk if you set `stream_options: {"include_usage": true}`.
    */
    pub choices: ::std::vec::Vec<ChatCompletionStreamChoice>,
    ///The Unix timestamp (in seconds) of when the chat completion was created. Each chunk has the same timestamp.
    pub created: i64,
    ///A unique identifier for the chat completion. Each chunk has the same ID.
    pub id: ::std::string::String,
    ///The model to generate the completion.
    pub model: ::std::string::String,
    ///The object type, which is always `chat.completion.chunk`.
    pub object: ::std::string::String,
    /**The format of the reasoning content. Can be `raw` or `parsed`.
    When specified as raw some reasoning models will output <think /> tags. When specified as parsed the model will output the reasoning under reasoning_content.
    */
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub reasoning_format: ::std::option::Option<::std::string::String>,
    /**This fingerprint represents the backend configuration that the model runs with.
    Can be used in conjunction with the `seed` request parameter to understand when backend changes have been made that might impact determinism.
    */
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub system_fingerprint: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub usage: ::std::option::Option<CompletionUsage>,
}
///`Endpoints`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "chat",
///    "models"
///  ],
///  "properties": {
///    "chat": {
///      "type": "string"
///    },
///    "models": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct Endpoints {
    pub chat: ::std::string::String,
    pub models: ::std::string::String,
}
///`Error`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "error": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct Error {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub error: ::std::option::Option<::std::string::String>,
}
impl ::std::default::Default for Error {
    fn default() -> Self {
        Self {
            error: Default::default(),
        }
    }
}
/**The reason the model stopped generating tokens. This will be `stop` if the model hit a natural stop point or a provided stop sequence,
`length` if the maximum number of tokens specified in the request was reached,
`content_filter` if content was omitted due to a flag from our content filters,
`tool_calls` if the model called a tool.
*/
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "The reason the model stopped generating tokens. This will be `stop` if the model hit a natural stop point or a provided stop sequence,\n`length` if the maximum number of tokens specified in the request was reached,\n`content_filter` if content was omitted due to a flag from our content filters,\n`tool_calls` if the model called a tool.\n",
///  "type": "string",
///  "enum": [
///    "stop",
///    "length",
///    "tool_calls",
///    "content_filter",
///    "function_call"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum FinishReason {
    #[serde(rename = "stop")]
    Stop,
    #[serde(rename = "length")]
    Length,
    #[serde(rename = "tool_calls")]
    ToolCalls,
    #[serde(rename = "content_filter")]
    ContentFilter,
    #[serde(rename = "function_call")]
    FunctionCall,
}
impl ::std::fmt::Display for FinishReason {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Stop => f.write_str("stop"),
            Self::Length => f.write_str("length"),
            Self::ToolCalls => f.write_str("tool_calls"),
            Self::ContentFilter => f.write_str("content_filter"),
            Self::FunctionCall => f.write_str("function_call"),
        }
    }
}
impl ::std::str::FromStr for FinishReason {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "stop" => Ok(Self::Stop),
            "length" => Ok(Self::Length),
            "tool_calls" => Ok(Self::ToolCalls),
            "content_filter" => Ok(Self::ContentFilter),
            "function_call" => Ok(Self::FunctionCall),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for FinishReason {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for FinishReason {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FinishReason {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`FunctionObject`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "name"
///  ],
///  "properties": {
///    "description": {
///      "description": "A description of what the function does, used by the model to choose when and how to call the function.",
///      "type": "string"
///    },
///    "name": {
///      "description": "The name of the function to be called. Must be a-z, A-Z, 0-9, or contain underscores and dashes, with a maximum length of 64.",
///      "type": "string"
///    },
///    "parameters": {
///      "$ref": "#/definitions/FunctionParameters"
///    },
///    "strict": {
///      "description": "Whether to enable strict schema adherence when generating the function call. If set to true, the model will follow the exact schema defined in the `parameters` field. Only a subset of JSON Schema is supported when `strict` is `true`. Learn more about Structured Outputs in the [function calling guide](docs/guides/function-calling).",
///      "default": false,
///      "type": "boolean"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct FunctionObject {
    ///A description of what the function does, used by the model to choose when and how to call the function.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    ///The name of the function to be called. Must be a-z, A-Z, 0-9, or contain underscores and dashes, with a maximum length of 64.
    pub name: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub parameters: ::std::option::Option<FunctionParameters>,
    ///Whether to enable strict schema adherence when generating the function call. If set to true, the model will follow the exact schema defined in the `parameters` field. Only a subset of JSON Schema is supported when `strict` is `true`. Learn more about Structured Outputs in the [function calling guide](docs/guides/function-calling).
    #[serde(default)]
    pub strict: bool,
}
/**The parameters the functions accepts, described as a JSON Schema object. See the [guide](/docs/guides/function-calling) for examples, and the [JSON Schema reference](https://json-schema.org/understanding-json-schema/) for documentation about the format.
Omitting `parameters` defines a function with an empty parameter list.*/
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "The parameters the functions accepts, described as a JSON Schema object. See the [guide](/docs/guides/function-calling) for examples, and the [JSON Schema reference](https://json-schema.org/understanding-json-schema/) for documentation about the format. \nOmitting `parameters` defines a function with an empty parameter list.",
///  "type": "object",
///  "additionalProperties": true
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct FunctionParameters(pub ::serde_json::Map<::std::string::String, ::serde_json::Value>);
impl ::std::ops::Deref for FunctionParameters {
    type Target = ::serde_json::Map<::std::string::String, ::serde_json::Value>;
    fn deref(&self) -> &::serde_json::Map<::std::string::String, ::serde_json::Value> {
        &self.0
    }
}
impl ::std::convert::From<FunctionParameters>
    for ::serde_json::Map<::std::string::String, ::serde_json::Value>
{
    fn from(value: FunctionParameters) -> Self {
        value.0
    }
}
impl ::std::convert::From<::serde_json::Map<::std::string::String, ::serde_json::Value>>
    for FunctionParameters
{
    fn from(value: ::serde_json::Map<::std::string::String, ::serde_json::Value>) -> Self {
        Self(value)
    }
}
///Image content part
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Image content part",
///  "type": "object",
///  "required": [
///    "image_url",
///    "type"
///  ],
///  "properties": {
///    "image_url": {
///      "$ref": "#/definitions/ImageURL"
///    },
///    "type": {
///      "description": "Content type identifier",
///      "type": "string",
///      "enum": [
///        "image_url"
///      ]
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct ImageContentPart {
    pub image_url: ImageUrl,
    ///Content type identifier
    #[serde(rename = "type")]
    pub type_: ImageContentPartType,
}
///Content type identifier
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Content type identifier",
///  "type": "string",
///  "enum": [
///    "image_url"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ImageContentPartType {
    #[serde(rename = "image_url")]
    ImageUrl,
}
impl ::std::fmt::Display for ImageContentPartType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ImageUrl => f.write_str("image_url"),
        }
    }
}
impl ::std::str::FromStr for ImageContentPartType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "image_url" => Ok(Self::ImageUrl),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ImageContentPartType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ImageContentPartType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ImageContentPartType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///Image URL configuration
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Image URL configuration",
///  "type": "object",
///  "required": [
///    "url"
///  ],
///  "properties": {
///    "detail": {
///      "description": "Image detail level for vision processing",
///      "default": "auto",
///      "type": "string",
///      "enum": [
///        "auto",
///        "low",
///        "high"
///      ]
///    },
///    "url": {
///      "description": "URL of the image (data URLs supported)",
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct ImageUrl {
    ///Image detail level for vision processing
    #[serde(default = "defaults::image_url_detail")]
    pub detail: ImageUrlDetail,
    ///URL of the image (data URLs supported)
    pub url: ::std::string::String,
}
///Image detail level for vision processing
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Image detail level for vision processing",
///  "default": "auto",
///  "type": "string",
///  "enum": [
///    "auto",
///    "low",
///    "high"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ImageUrlDetail {
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "low")]
    Low,
    #[serde(rename = "high")]
    High,
}
impl ::std::fmt::Display for ImageUrlDetail {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Auto => f.write_str("auto"),
            Self::Low => f.write_str("low"),
            Self::High => f.write_str("high"),
        }
    }
}
impl ::std::str::FromStr for ImageUrlDetail {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "auto" => Ok(Self::Auto),
            "low" => Ok(Self::Low),
            "high" => Ok(Self::High),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ImageUrlDetail {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ImageUrlDetail {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ImageUrlDetail {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::default::Default for ImageUrlDetail {
    fn default() -> Self {
        ImageUrlDetail::Auto
    }
}
///Response structure for listing models
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Response structure for listing models",
///  "type": "object",
///  "required": [
///    "data",
///    "object"
///  ],
///  "properties": {
///    "data": {
///      "default": [],
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/Model"
///      }
///    },
///    "object": {
///      "type": "string"
///    },
///    "provider": {
///      "$ref": "#/definitions/Provider"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct ListModelsResponse {
    pub data: ::std::vec::Vec<Model>,
    pub object: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub provider: ::std::option::Option<Provider>,
}
///Response structure for listing MCP tools
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Response structure for listing MCP tools",
///  "type": "object",
///  "required": [
///    "data",
///    "object"
///  ],
///  "properties": {
///    "data": {
///      "description": "Array of available MCP tools",
///      "default": [],
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/MCPTool"
///      }
///    },
///    "object": {
///      "description": "Always \"list\"",
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct ListToolsResponse {
    ///Array of available MCP tools
    pub data: ::std::vec::Vec<McpTool>,
    ///Always "list"
    pub object: ::std::string::String,
}
///An MCP tool definition
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "An MCP tool definition",
///  "type": "object",
///  "required": [
///    "description",
///    "name",
///    "server"
///  ],
///  "properties": {
///    "description": {
///      "description": "A description of what the tool does",
///      "type": "string"
///    },
///    "input_schema": {
///      "description": "JSON schema for the tool's input parameters",
///      "type": "object",
///      "additionalProperties": true
///    },
///    "name": {
///      "description": "The name of the tool",
///      "type": "string"
///    },
///    "server": {
///      "description": "The MCP server that provides this tool",
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct McpTool {
    ///A description of what the tool does
    pub description: ::std::string::String,
    ///JSON schema for the tool's input parameters
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub input_schema: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    ///The name of the tool
    pub name: ::std::string::String,
    ///The MCP server that provides this tool
    pub server: ::std::string::String,
}
///Message structure for provider requests
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Message structure for provider requests",
///  "type": "object",
///  "required": [
///    "content",
///    "role"
///  ],
///  "properties": {
///    "content": {
///      "$ref": "#/definitions/MessageContent"
///    },
///    "reasoning": {
///      "description": "The reasoning of the chunk message. Same as reasoning_content.",
///      "type": "string"
///    },
///    "reasoning_content": {
///      "description": "The reasoning content of the chunk message.",
///      "type": "string"
///    },
///    "role": {
///      "$ref": "#/definitions/MessageRole"
///    },
///    "tool_call_id": {
///      "type": "string"
///    },
///    "tool_calls": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/ChatCompletionMessageToolCall"
///      }
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct Message {
    pub content: MessageContent,
    ///The reasoning of the chunk message. Same as reasoning_content.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub reasoning: ::std::option::Option<::std::string::String>,
    ///The reasoning content of the chunk message.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub reasoning_content: ::std::option::Option<::std::string::String>,
    pub role: MessageRole,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub tool_call_id: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub tool_calls: ::std::vec::Vec<ChatCompletionMessageToolCall>,
}
///Message content - either text or multimodal content parts
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Message content - either text or multimodal content parts",
///  "oneOf": [
///    {
///      "description": "Text content (backward compatibility)",
///      "type": "string"
///    },
///    {
///      "description": "Array of content parts for multimodal messages",
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/ContentPart"
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum MessageContent {
    String(::std::string::String),
    Array(::std::vec::Vec<ContentPart>),
}
impl ::std::convert::From<::std::vec::Vec<ContentPart>> for MessageContent {
    fn from(value: ::std::vec::Vec<ContentPart>) -> Self {
        Self::Array(value)
    }
}
///Role of the message sender
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Role of the message sender",
///  "type": "string",
///  "enum": [
///    "system",
///    "user",
///    "assistant",
///    "tool"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum MessageRole {
    #[serde(rename = "system")]
    System,
    #[serde(rename = "user")]
    User,
    #[serde(rename = "assistant")]
    Assistant,
    #[serde(rename = "tool")]
    Tool,
}
impl ::std::fmt::Display for MessageRole {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::System => f.write_str("system"),
            Self::User => f.write_str("user"),
            Self::Assistant => f.write_str("assistant"),
            Self::Tool => f.write_str("tool"),
        }
    }
}
impl ::std::str::FromStr for MessageRole {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "system" => Ok(Self::System),
            "user" => Ok(Self::User),
            "assistant" => Ok(Self::Assistant),
            "tool" => Ok(Self::Tool),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for MessageRole {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for MessageRole {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for MessageRole {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///Common model information
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Common model information",
///  "type": "object",
///  "required": [
///    "created",
///    "id",
///    "object",
///    "owned_by",
///    "served_by"
///  ],
///  "properties": {
///    "created": {
///      "type": "integer",
///      "format": "int64"
///    },
///    "id": {
///      "type": "string"
///    },
///    "object": {
///      "type": "string"
///    },
///    "owned_by": {
///      "type": "string"
///    },
///    "served_by": {
///      "$ref": "#/definitions/Provider"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct Model {
    pub created: i64,
    pub id: ::std::string::String,
    pub object: ::std::string::String,
    pub owned_by: ::std::string::String,
    pub served_by: Provider,
}
///`Provider`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "ollama",
///    "ollama_cloud",
///    "groq",
///    "openai",
///    "cloudflare",
///    "cohere",
///    "anthropic",
///    "deepseek",
///    "google",
///    "mistral",
///    "minimax",
///    "moonshot"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum Provider {
    #[serde(rename = "ollama")]
    Ollama,
    #[serde(rename = "ollama_cloud")]
    OllamaCloud,
    #[serde(rename = "groq")]
    Groq,
    #[serde(rename = "openai")]
    Openai,
    #[serde(rename = "cloudflare")]
    Cloudflare,
    #[serde(rename = "cohere")]
    Cohere,
    #[serde(rename = "anthropic")]
    Anthropic,
    #[serde(rename = "deepseek")]
    Deepseek,
    #[serde(rename = "google")]
    Google,
    #[serde(rename = "mistral")]
    Mistral,
    #[serde(rename = "minimax")]
    Minimax,
    #[serde(rename = "moonshot")]
    Moonshot,
}
impl ::std::fmt::Display for Provider {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Ollama => f.write_str("ollama"),
            Self::OllamaCloud => f.write_str("ollama_cloud"),
            Self::Groq => f.write_str("groq"),
            Self::Openai => f.write_str("openai"),
            Self::Cloudflare => f.write_str("cloudflare"),
            Self::Cohere => f.write_str("cohere"),
            Self::Anthropic => f.write_str("anthropic"),
            Self::Deepseek => f.write_str("deepseek"),
            Self::Google => f.write_str("google"),
            Self::Mistral => f.write_str("mistral"),
            Self::Minimax => f.write_str("minimax"),
            Self::Moonshot => f.write_str("moonshot"),
        }
    }
}
impl ::std::str::FromStr for Provider {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "ollama" => Ok(Self::Ollama),
            "ollama_cloud" => Ok(Self::OllamaCloud),
            "groq" => Ok(Self::Groq),
            "openai" => Ok(Self::Openai),
            "cloudflare" => Ok(Self::Cloudflare),
            "cohere" => Ok(Self::Cohere),
            "anthropic" => Ok(Self::Anthropic),
            "deepseek" => Ok(Self::Deepseek),
            "google" => Ok(Self::Google),
            "mistral" => Ok(Self::Mistral),
            "minimax" => Ok(Self::Minimax),
            "moonshot" => Ok(Self::Moonshot),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for Provider {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for Provider {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for Provider {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///Authentication type for providers
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Authentication type for providers",
///  "type": "string",
///  "enum": [
///    "bearer",
///    "xheader",
///    "query",
///    "none"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ProviderAuthType {
    #[serde(rename = "bearer")]
    Bearer,
    #[serde(rename = "xheader")]
    Xheader,
    #[serde(rename = "query")]
    Query,
    #[serde(rename = "none")]
    None,
}
impl ::std::fmt::Display for ProviderAuthType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Bearer => f.write_str("bearer"),
            Self::Xheader => f.write_str("xheader"),
            Self::Query => f.write_str("query"),
            Self::None => f.write_str("none"),
        }
    }
}
impl ::std::str::FromStr for ProviderAuthType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "bearer" => Ok(Self::Bearer),
            "xheader" => Ok(Self::Xheader),
            "query" => Ok(Self::Query),
            "none" => Ok(Self::None),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ProviderAuthType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ProviderAuthType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ProviderAuthType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
/**Provider-specific response format. Examples:

OpenAI GET /v1/models?provider=openai response:
```json
{
  "provider": "openai",
  "object": "list",
  "data": [
    {
      "id": "gpt-4",
      "object": "model",
      "created": 1687882410,
      "owned_by": "openai",
      "served_by": "openai"
    }
  ]
}
```

Anthropic GET /v1/models?provider=anthropic response:
```json
{
  "provider": "anthropic",
  "object": "list",
  "data": [
    {
      "id": "gpt-4",
      "object": "model",
      "created": 1687882410,
      "owned_by": "openai",
      "served_by": "openai"
    }
  ]
}
```
*/
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Provider-specific response format. Examples:\n\nOpenAI GET /v1/models?provider=openai response:\n```json\n{\n  \"provider\": \"openai\",\n  \"object\": \"list\",\n  \"data\": [\n    {\n      \"id\": \"gpt-4\",\n      \"object\": \"model\",\n      \"created\": 1687882410,\n      \"owned_by\": \"openai\",\n      \"served_by\": \"openai\"\n    }\n  ]\n}\n```\n\nAnthropic GET /v1/models?provider=anthropic response:\n```json\n{\n  \"provider\": \"anthropic\",\n  \"object\": \"list\",\n  \"data\": [\n    {\n      \"id\": \"gpt-4\",\n      \"object\": \"model\",\n      \"created\": 1687882410,\n      \"owned_by\": \"openai\",\n      \"served_by\": \"openai\"\n    }\n  ]\n}\n```\n",
///  "type": "object"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct ProviderSpecificResponse(
    pub ::serde_json::Map<::std::string::String, ::serde_json::Value>,
);
impl ::std::ops::Deref for ProviderSpecificResponse {
    type Target = ::serde_json::Map<::std::string::String, ::serde_json::Value>;
    fn deref(&self) -> &::serde_json::Map<::std::string::String, ::serde_json::Value> {
        &self.0
    }
}
impl ::std::convert::From<ProviderSpecificResponse>
    for ::serde_json::Map<::std::string::String, ::serde_json::Value>
{
    fn from(value: ProviderSpecificResponse) -> Self {
        value.0
    }
}
impl ::std::convert::From<::serde_json::Map<::std::string::String, ::serde_json::Value>>
    for ProviderSpecificResponse
{
    fn from(value: ::serde_json::Map<::std::string::String, ::serde_json::Value>) -> Self {
        Self(value)
    }
}
/**JSON object response format. An older method of generating JSON responses. Using `json_schema` is recommended for models that support it. Note that the model will not generate JSON without a system or user message instructing it to do so.
*/
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "JSON object response format. An older method of generating JSON responses. Using `json_schema` is recommended for models that support it. Note that the model will not generate JSON without a system or user message instructing it to do so.\n",
///  "type": "object",
///  "required": [
///    "type"
///  ],
///  "properties": {
///    "type": {
///      "description": "The type of response format being defined. Always `json_object`.",
///      "type": "string",
///      "enum": [
///        "json_object"
///      ]
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct ResponseFormatJsonObject {
    ///The type of response format being defined. Always `json_object`.
    #[serde(rename = "type")]
    pub type_: ResponseFormatJsonObjectType,
}
///The type of response format being defined. Always `json_object`.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "The type of response format being defined. Always `json_object`.",
///  "type": "string",
///  "enum": [
///    "json_object"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ResponseFormatJsonObjectType {
    #[serde(rename = "json_object")]
    JsonObject,
}
impl ::std::fmt::Display for ResponseFormatJsonObjectType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::JsonObject => f.write_str("json_object"),
        }
    }
}
impl ::std::str::FromStr for ResponseFormatJsonObjectType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "json_object" => Ok(Self::JsonObject),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ResponseFormatJsonObjectType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ResponseFormatJsonObjectType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ResponseFormatJsonObjectType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
/**JSON Schema response format. Used to generate structured JSON responses.
*/
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "JSON Schema response format. Used to generate structured JSON responses.\n",
///  "type": "object",
///  "required": [
///    "json_schema",
///    "type"
///  ],
///  "properties": {
///    "json_schema": {
///      "description": "Structured Outputs configuration options, including a JSON Schema.",
///      "type": "object",
///      "required": [
///        "name"
///      ],
///      "properties": {
///        "description": {
///          "description": "A description of what the response format is for, used by the model to determine how to respond in the format.\n",
///          "type": "string"
///        },
///        "name": {
///          "description": "The name of the response format. Must be a-z, A-Z, 0-9, or contain underscores and dashes, with a maximum length of 64.\n",
///          "type": "string"
///        },
///        "schema": {
///          "$ref": "#/definitions/ResponseFormatJsonSchemaSchema"
///        },
///        "strict": {
///          "description": "Whether to enable strict schema adherence when generating the output. If set to true, the model will always follow the exact schema defined in the `schema` field. Only a subset of JSON Schema is supported when `strict` is `true`.\n",
///          "default": false,
///          "type": "boolean"
///        }
///      }
///    },
///    "type": {
///      "description": "The type of response format being defined. Always `json_schema`.",
///      "type": "string",
///      "enum": [
///        "json_schema"
///      ]
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct ResponseFormatJsonSchema {
    pub json_schema: ResponseFormatJsonSchemaJsonSchema,
    ///The type of response format being defined. Always `json_schema`.
    #[serde(rename = "type")]
    pub type_: ResponseFormatJsonSchemaType,
}
///Structured Outputs configuration options, including a JSON Schema.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Structured Outputs configuration options, including a JSON Schema.",
///  "type": "object",
///  "required": [
///    "name"
///  ],
///  "properties": {
///    "description": {
///      "description": "A description of what the response format is for, used by the model to determine how to respond in the format.\n",
///      "type": "string"
///    },
///    "name": {
///      "description": "The name of the response format. Must be a-z, A-Z, 0-9, or contain underscores and dashes, with a maximum length of 64.\n",
///      "type": "string"
///    },
///    "schema": {
///      "$ref": "#/definitions/ResponseFormatJsonSchemaSchema"
///    },
///    "strict": {
///      "description": "Whether to enable strict schema adherence when generating the output. If set to true, the model will always follow the exact schema defined in the `schema` field. Only a subset of JSON Schema is supported when `strict` is `true`.\n",
///      "default": false,
///      "type": "boolean"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct ResponseFormatJsonSchemaJsonSchema {
    /**A description of what the response format is for, used by the model to determine how to respond in the format.
     */
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    /**The name of the response format. Must be a-z, A-Z, 0-9, or contain underscores and dashes, with a maximum length of 64.
     */
    pub name: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub schema: ::std::option::Option<ResponseFormatJsonSchemaSchema>,
    /**Whether to enable strict schema adherence when generating the output. If set to true, the model will always follow the exact schema defined in the `schema` field. Only a subset of JSON Schema is supported when `strict` is `true`.
     */
    #[serde(default)]
    pub strict: bool,
}
/**The schema for the response format, described as a JSON Schema object.
*/
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "The schema for the response format, described as a JSON Schema object.\n",
///  "type": "object",
///  "additionalProperties": true
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct ResponseFormatJsonSchemaSchema(
    pub ::serde_json::Map<::std::string::String, ::serde_json::Value>,
);
impl ::std::ops::Deref for ResponseFormatJsonSchemaSchema {
    type Target = ::serde_json::Map<::std::string::String, ::serde_json::Value>;
    fn deref(&self) -> &::serde_json::Map<::std::string::String, ::serde_json::Value> {
        &self.0
    }
}
impl ::std::convert::From<ResponseFormatJsonSchemaSchema>
    for ::serde_json::Map<::std::string::String, ::serde_json::Value>
{
    fn from(value: ResponseFormatJsonSchemaSchema) -> Self {
        value.0
    }
}
impl ::std::convert::From<::serde_json::Map<::std::string::String, ::serde_json::Value>>
    for ResponseFormatJsonSchemaSchema
{
    fn from(value: ::serde_json::Map<::std::string::String, ::serde_json::Value>) -> Self {
        Self(value)
    }
}
///The type of response format being defined. Always `json_schema`.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "The type of response format being defined. Always `json_schema`.",
///  "type": "string",
///  "enum": [
///    "json_schema"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ResponseFormatJsonSchemaType {
    #[serde(rename = "json_schema")]
    JsonSchema,
}
impl ::std::fmt::Display for ResponseFormatJsonSchemaType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::JsonSchema => f.write_str("json_schema"),
        }
    }
}
impl ::std::str::FromStr for ResponseFormatJsonSchemaType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "json_schema" => Ok(Self::JsonSchema),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ResponseFormatJsonSchemaType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ResponseFormatJsonSchemaType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ResponseFormatJsonSchemaType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///Default response format. Used to generate text responses.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Default response format. Used to generate text responses.",
///  "type": "object",
///  "required": [
///    "type"
///  ],
///  "properties": {
///    "type": {
///      "description": "The type of response format being defined. Always `text`.",
///      "type": "string",
///      "enum": [
///        "text"
///      ]
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct ResponseFormatText {
    ///The type of response format being defined. Always `text`.
    #[serde(rename = "type")]
    pub type_: ResponseFormatTextType,
}
///The type of response format being defined. Always `text`.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "The type of response format being defined. Always `text`.",
///  "type": "string",
///  "enum": [
///    "text"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ResponseFormatTextType {
    #[serde(rename = "text")]
    Text,
}
impl ::std::fmt::Display for ResponseFormatTextType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Text => f.write_str("text"),
        }
    }
}
impl ::std::str::FromStr for ResponseFormatTextType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "text" => Ok(Self::Text),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ResponseFormatTextType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ResponseFormatTextType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ResponseFormatTextType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`SsEvent`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "data": {
///      "type": "string",
///      "format": "byte"
///    },
///    "event": {
///      "type": "string",
///      "enum": [
///        "message-start",
///        "stream-start",
///        "content-start",
///        "content-delta",
///        "content-end",
///        "message-end",
///        "stream-end"
///      ]
///    },
///    "retry": {
///      "type": "integer"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct SsEvent {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub data: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub event: ::std::option::Option<SsEventEvent>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub retry: ::std::option::Option<i64>,
}
impl ::std::default::Default for SsEvent {
    fn default() -> Self {
        Self {
            data: Default::default(),
            event: Default::default(),
            retry: Default::default(),
        }
    }
}
///`SsEventEvent`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "message-start",
///    "stream-start",
///    "content-start",
///    "content-delta",
///    "content-end",
///    "message-end",
///    "stream-end"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum SsEventEvent {
    #[serde(rename = "message-start")]
    MessageStart,
    #[serde(rename = "stream-start")]
    StreamStart,
    #[serde(rename = "content-start")]
    ContentStart,
    #[serde(rename = "content-delta")]
    ContentDelta,
    #[serde(rename = "content-end")]
    ContentEnd,
    #[serde(rename = "message-end")]
    MessageEnd,
    #[serde(rename = "stream-end")]
    StreamEnd,
}
impl ::std::fmt::Display for SsEventEvent {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::MessageStart => f.write_str("message-start"),
            Self::StreamStart => f.write_str("stream-start"),
            Self::ContentStart => f.write_str("content-start"),
            Self::ContentDelta => f.write_str("content-delta"),
            Self::ContentEnd => f.write_str("content-end"),
            Self::MessageEnd => f.write_str("message-end"),
            Self::StreamEnd => f.write_str("stream-end"),
        }
    }
}
impl ::std::str::FromStr for SsEventEvent {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "message-start" => Ok(Self::MessageStart),
            "stream-start" => Ok(Self::StreamStart),
            "content-start" => Ok(Self::ContentStart),
            "content-delta" => Ok(Self::ContentDelta),
            "content-end" => Ok(Self::ContentEnd),
            "message-end" => Ok(Self::MessageEnd),
            "stream-end" => Ok(Self::StreamEnd),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for SsEventEvent {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for SsEventEvent {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for SsEventEvent {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///Text content part
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Text content part",
///  "type": "object",
///  "required": [
///    "text",
///    "type"
///  ],
///  "properties": {
///    "text": {
///      "description": "The text content",
///      "type": "string"
///    },
///    "type": {
///      "description": "Content type identifier",
///      "type": "string",
///      "enum": [
///        "text"
///      ]
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct TextContentPart {
    ///The text content
    pub text: ::std::string::String,
    ///Content type identifier
    #[serde(rename = "type")]
    pub type_: TextContentPartType,
}
///Content type identifier
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Content type identifier",
///  "type": "string",
///  "enum": [
///    "text"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum TextContentPartType {
    #[serde(rename = "text")]
    Text,
}
impl ::std::fmt::Display for TextContentPartType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Text => f.write_str("text"),
        }
    }
}
impl ::std::str::FromStr for TextContentPartType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "text" => Ok(Self::Text),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for TextContentPartType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for TextContentPartType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for TextContentPartType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
/**Provider-specific opaque data attached to a tool call. The contents are
not interpreted by the gateway, but must be echoed back verbatim on the
next request that references this tool call. Currently used by Google
Gemini extended-thinking models to carry the per-call `thought_signature`.
Other providers may ignore the field.
*/
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Provider-specific opaque data attached to a tool call. The contents are\nnot interpreted by the gateway, but must be echoed back verbatim on the\nnext request that references this tool call. Currently used by Google\nGemini extended-thinking models to carry the per-call `thought_signature`.\nOther providers may ignore the field.\n",
///  "type": "object",
///  "properties": {
///    "google": {
///      "description": "Google Gemini-specific extra content.",
///      "type": "object",
///      "properties": {
///        "thought_signature": {
///          "description": "Opaque signature returned with reasoning-enabled tool calls.\nMust be echoed back verbatim in the next request that includes\nthis tool call, or Google will reject the request.\n",
///          "type": "string"
///        }
///      },
///      "additionalProperties": true
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct ToolCallExtraContent {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub google: ::std::option::Option<ToolCallExtraContentGoogle>,
}
impl ::std::default::Default for ToolCallExtraContent {
    fn default() -> Self {
        Self {
            google: Default::default(),
        }
    }
}
///Google Gemini-specific extra content.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Google Gemini-specific extra content.",
///  "type": "object",
///  "properties": {
///    "thought_signature": {
///      "description": "Opaque signature returned with reasoning-enabled tool calls.\nMust be echoed back verbatim in the next request that includes\nthis tool call, or Google will reject the request.\n",
///      "type": "string"
///    }
///  },
///  "additionalProperties": true
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct ToolCallExtraContentGoogle {
    /**Opaque signature returned with reasoning-enabled tool calls.
    Must be echoed back verbatim in the next request that includes
    this tool call, or Google will reject the request.
    */
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub thought_signature: ::std::option::Option<::std::string::String>,
}
impl ::std::default::Default for ToolCallExtraContentGoogle {
    fn default() -> Self {
        Self {
            thought_signature: Default::default(),
        }
    }
}
/// Generation of default values for serde.
pub mod defaults {
    pub(super) fn default_bool<const V: bool>() -> bool {
        V
    }
    pub(super) fn default_i64<T, const V: i64>() -> T
    where
        T: ::std::convert::TryFrom<i64>,
        <T as ::std::convert::TryFrom<i64>>::Error: ::std::fmt::Debug,
    {
        T::try_from(V).unwrap()
    }
    pub(super) fn default_nzu64<T, const V: u64>() -> T
    where
        T: ::std::convert::TryFrom<::std::num::NonZeroU64>,
        <T as ::std::convert::TryFrom<::std::num::NonZeroU64>>::Error: ::std::fmt::Debug,
    {
        T::try_from(::std::num::NonZeroU64::try_from(V).unwrap()).unwrap()
    }
    pub(super) fn create_chat_completion_request_frequency_penalty() -> f64 {
        0_f64
    }
    pub(super) fn create_chat_completion_request_presence_penalty() -> f64 {
        0_f64
    }
    pub(super) fn create_chat_completion_request_temperature() -> f64 {
        1_f64
    }
    pub(super) fn create_chat_completion_request_top_p() -> f64 {
        1_f64
    }
    pub(super) fn image_url_detail() -> super::ImageUrlDetail {
        super::ImageUrlDetail::Auto
    }
}
