use axum::{body::Body, response::IntoResponse};
use reqwest::header::CONTENT_TYPE;
use serde::Serialize;
use thiserror::Error;

use crate::{
    engine::types::{GraphError, VectorError},
    protocol::request::RequestType,
};

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
    code: &'static str,
}

#[derive(Debug, Error)]
pub enum ActiveDBError {
    #[error("{0}")]
    Graph(#[from] GraphError),
    #[error("{0}")]
    Vector(#[from] VectorError),
    #[error("Couldn't find `{name}` of type {ty:?}")]
    NotFound { ty: RequestType, name: String },
    #[error("Invalid API key")]
    InvalidApiKey,
}

impl Serialize for ActiveDBError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.collect_str(&self.to_string())
    }
}

impl ActiveDBError {
    fn code(&self) -> &'static str {
        match self {
            ActiveDBError::Graph(_) => "GRAPH_ERROR",
            ActiveDBError::Vector(_) => "VECTOR_ERROR",
            ActiveDBError::NotFound { .. } => "NOT_FOUND",
            ActiveDBError::InvalidApiKey => "INVALID_API_KEY",
        }
    }
}

impl IntoResponse for ActiveDBError {
    fn into_response(self) -> axum::response::Response {
        let status = match &self {
            ActiveDBError::NotFound { .. }
            | ActiveDBError::Graph(
                GraphError::ConfigFileNotFound
                | GraphError::NodeNotFound
                | GraphError::EdgeNotFound
                | GraphError::LabelNotFound
                | GraphError::ShortestPathNotFound,
            )
            | ActiveDBError::Vector(VectorError::VectorNotFound(_)) => {
                axum::http::StatusCode::NOT_FOUND
            }
            ActiveDBError::Graph(_) | ActiveDBError::Vector(_) => {
                axum::http::StatusCode::INTERNAL_SERVER_ERROR
            }
            ActiveDBError::InvalidApiKey => axum::http::StatusCode::FORBIDDEN,
        };

        let error_response = ErrorResponse {
            error: self.to_string(),
            code: self.code(),
        };

        let body = sonic_rs::to_vec(&error_response).unwrap_or_else(|_| {
            br#"{"error":"Internal serialization error","code":"INTERNAL_ERROR"}"#.to_vec()
        });

        axum::response::Response::builder()
            .status(status)
            .header(CONTENT_TYPE, "application/json")
            .body(Body::from(body))
            .unwrap_or_else(|e| {
                // This should never happen with valid HTTP headers, but handle gracefully
                tracing::error!("Failed to build error response: {e:?}");
                axum::response::Response::builder()
                    .status(500)
                    .body(Body::from(
                        r#"{"error":"Internal server error","code":"INTERNAL_ERROR"}"#,
                    ))
                    .expect("static response should always build")
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ============================================================================
    // ActiveDBError Variant Tests
    // ============================================================================

    #[test]
    fn test_activedb_error_not_found() {
        let error = ActiveDBError::NotFound {
            ty: RequestType::Query,
            name: "test_query".to_string(),
        };

        let error_string = error.to_string();
        assert!(error_string.contains("test_query"));
        assert!(error_string.contains("Couldn't find"));
    }

    #[test]
    fn test_activedb_error_not_found_mcp() {
        let error = ActiveDBError::NotFound {
            ty: RequestType::MCP,
            name: "test_mcp".to_string(),
        };

        let error_string = error.to_string();
        assert!(error_string.contains("test_mcp"));
        assert!(error_string.contains("MCP"));
    }

    #[test]
    fn test_activedb_error_graph() {
        let graph_err = GraphError::DecodeError("test decode error".to_string());
        let activedb_err = ActiveDBError::from(graph_err);

        assert!(matches!(activedb_err, ActiveDBError::Graph(_)));
        let error_string = activedb_err.to_string();
        assert!(error_string.contains("test decode error"));
    }

    #[test]
    fn test_activedb_error_vector() {
        let vector_err = VectorError::InvalidVectorLength;
        let activedb_err = ActiveDBError::from(vector_err);

        assert!(matches!(activedb_err, ActiveDBError::Vector(_)));
    }

    // ============================================================================
    // IntoResponse Tests (HTTP Status Codes and JSON Format)
    // ============================================================================

    #[test]
    fn test_activedb_error_into_response_not_found() {
        let error = ActiveDBError::NotFound {
            ty: RequestType::Query,
            name: "missing".to_string(),
        };

        let response = error.into_response();
        assert_eq!(response.status(), 404);
        assert_eq!(
            response.headers().get(CONTENT_TYPE).unwrap(),
            "application/json"
        );
    }

    #[test]
    fn test_activedb_error_into_response_graph_error() {
        let graph_err = GraphError::DecodeError("decode failed".to_string());
        let activedb_err = ActiveDBError::from(graph_err);

        let response = activedb_err.into_response();
        assert_eq!(response.status(), 500);
        assert_eq!(
            response.headers().get(CONTENT_TYPE).unwrap(),
            "application/json"
        );
    }

    #[test]
    fn test_activedb_error_into_response_vector_error() {
        let vector_err = VectorError::InvalidVectorData;
        let activedb_err = ActiveDBError::from(vector_err);

        let response = activedb_err.into_response();
        assert_eq!(response.status(), 500);
        assert_eq!(
            response.headers().get(CONTENT_TYPE).unwrap(),
            "application/json"
        );
    }

    // ============================================================================
    // Error Code Tests
    // ============================================================================

    #[test]
    fn test_activedb_error_code_graph() {
        let error = ActiveDBError::Graph(GraphError::NodeNotFound);
        assert_eq!(error.code(), "GRAPH_ERROR");
    }

    #[test]
    fn test_activedb_error_code_vector() {
        let error = ActiveDBError::Vector(VectorError::InvalidVectorLength);
        assert_eq!(error.code(), "VECTOR_ERROR");
    }

    #[test]
    fn test_activedb_error_code_not_found() {
        let error = ActiveDBError::NotFound {
            ty: RequestType::Query,
            name: "test".to_string(),
        };
        assert_eq!(error.code(), "NOT_FOUND");
    }

    #[test]
    fn test_activedb_error_code_invalid_api_key() {
        let error = ActiveDBError::InvalidApiKey;
        assert_eq!(error.code(), "INVALID_API_KEY");
    }

    // ============================================================================
    // Error Trait Tests
    // ============================================================================

    #[test]
    fn test_activedb_error_is_error_trait() {
        let error = ActiveDBError::NotFound {
            ty: RequestType::Query,
            name: "test".to_string(),
        };

        // Test that it implements std::error::Error
        fn assert_error<T: std::error::Error>(_: T) {}
        assert_error(error);
    }

    #[test]
    fn test_activedb_error_debug() {
        let error = ActiveDBError::NotFound {
            ty: RequestType::Query,
            name: "debug_test".to_string(),
        };

        let debug_str = format!("{:?}", error);
        assert!(debug_str.contains("NotFound"));
        assert!(debug_str.contains("debug_test"));
    }

    // ============================================================================
    // InvalidApiKey Tests
    // ============================================================================

    #[test]
    fn test_activedb_error_invalid_api_key() {
        let error = ActiveDBError::InvalidApiKey;
        let error_string = error.to_string();
        assert_eq!(error_string, "Invalid API key");
    }

    #[test]
    fn test_activedb_error_invalid_api_key_into_response() {
        let error = ActiveDBError::InvalidApiKey;
        let response = error.into_response();
        assert_eq!(response.status(), 403);
    }

    #[test]
    fn test_activedb_error_invalid_api_key_debug() {
        let error = ActiveDBError::InvalidApiKey;
        let debug_str = format!("{:?}", error);
        assert!(debug_str.contains("InvalidApiKey"));
    }
}
