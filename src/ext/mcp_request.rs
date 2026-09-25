//! Constructors for the MCP JSON-RPC request envelope.
//!
//! The spec models `params` as a free-form object, so every caller would
//! otherwise have to hand-assemble the `_meta` block (`RequestMetaObject`) that
//! `POST /mcp` requires on every request. These constructors fill it in.

use std::sync::atomic::{AtomicU64, Ordering};

use serde_json::{Map, Value, json};

use crate::generated::schemas::{McpjsonrpcRequest, McpjsonrpcRequestId, McpjsonrpcRequestMethod};

/// The only MCP protocol version `POST /mcp` speaks.
pub const MCP_PROTOCOL_VERSION: &str = "2026-07-28";

static NEXT_ID: AtomicU64 = AtomicU64::new(1);

fn request(method: McpjsonrpcRequestMethod, mut params: Map<String, Value>) -> McpjsonrpcRequest {
    params.insert(
        "_meta".to_string(),
        json!({
            "io.modelcontextprotocol/protocolVersion": MCP_PROTOCOL_VERSION,
            "io.modelcontextprotocol/clientInfo": {
                "name": env!("CARGO_PKG_NAME"),
                "version": env!("CARGO_PKG_VERSION"),
            },
            "io.modelcontextprotocol/clientCapabilities": {},
        }),
    );
    McpjsonrpcRequest {
        id: Some(McpjsonrpcRequestId::Integer(
            NEXT_ID.fetch_add(1, Ordering::Relaxed) as i64,
        )),
        jsonrpc: "2.0".to_string(),
        method,
        params,
    }
}

impl McpjsonrpcRequest {
    /// A `server/discover` request - the supported protocol versions and
    /// capabilities of the gateway's MCP server.
    pub fn server_discover() -> Self {
        request(McpjsonrpcRequestMethod::ServerDiscover, Map::new())
    }

    /// A `tools/list` request, optionally resuming from a pagination `cursor`.
    pub fn tools_list(cursor: Option<&str>) -> Self {
        let mut params = Map::new();
        if let Some(cursor) = cursor {
            params.insert("cursor".to_string(), Value::String(cursor.to_string()));
        }
        request(McpjsonrpcRequestMethod::ToolsList, params)
    }

    /// A `tools/call` request for the namespaced tool `name`
    /// (`mcp_<server alias>_<tool name>`) with the given `arguments` object.
    pub fn tools_call(name: &str, arguments: Value) -> Self {
        let mut params = Map::new();
        params.insert("name".to_string(), Value::String(name.to_string()));
        params.insert("arguments".to_string(), arguments);
        request(McpjsonrpcRequestMethod::ToolsCall, params)
    }

    /// The protocol version the request declares in
    /// `params._meta["io.modelcontextprotocol/protocolVersion"]`, which the
    /// `MCP-Protocol-Version` header must mirror.
    pub fn protocol_version(&self) -> &str {
        self.params
            .get("_meta")
            .and_then(|meta| meta.get("io.modelcontextprotocol/protocolVersion"))
            .and_then(Value::as_str)
            .unwrap_or(MCP_PROTOCOL_VERSION)
    }
}
