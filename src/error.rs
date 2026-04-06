//! Error types for Azure HTTP client operations.

use std::time::Duration;
use thiserror::Error;

/// Result type alias using AzureError.
pub type Result<T> = std::result::Result<T, AzureError>;

/// Errors that can occur during Azure API operations.
#[derive(Debug, Error, Clone)]
pub enum AzureError {
    /// Authentication failed (invalid credentials, expired token).
    #[error("Authentication failed: {message}")]
    Auth { message: String },

    /// Access denied (insufficient RBAC permissions).
    #[error("Permission denied: {message}")]
    PermissionDenied { message: String },

    /// Resource not found.
    #[error("Resource not found: {resource}")]
    NotFound { resource: String },

    /// Request throttled.
    #[error("Throttled (retry after {retry_after:?})")]
    Throttled {
        retry_after: Option<Duration>,
        message: String,
    },

    /// Resource conflict (e.g., already exists).
    #[error("Resource conflict: {message}")]
    ResourceConflict { message: String },

    /// Azure service error.
    #[error("Service error ({code}): {message}")]
    ServiceError {
        code: String,
        message: String,
        status: u16,
    },

    /// Network error.
    #[error("Network error: {0}")]
    Network(String),

    /// Invalid response from service.
    #[error("Invalid response: {message}")]
    InvalidResponse {
        message: String,
        body: Option<String>,
    },
}

impl From<reqwest::Error> for AzureError {
    fn from(err: reqwest::Error) -> Self {
        Self::Network(err.to_string())
    }
}

impl AzureError {
    /// Returns true if this error is retryable.
    pub fn is_retryable(&self) -> bool {
        match self {
            Self::Throttled { .. } | Self::Network(_) => true,
            Self::ServiceError { status, .. } => matches!(status, 500 | 502 | 503 | 504),
            _ => false,
        }
    }

    /// Extract retry-after duration if present.
    pub fn retry_after(&self) -> Option<Duration> {
        match self {
            Self::Throttled {
                retry_after: Some(duration),
                ..
            } => Some(*duration),
            _ => None,
        }
    }
}

/// Map an Azure error code + HTTP status to a typed `AzureError`.
pub(crate) fn classify_error(status: u16, code: &str, message: &str) -> AzureError {
    match status {
        401 => AzureError::Auth {
            message: format!("{code}: {message}"),
        },
        403 => AzureError::PermissionDenied {
            message: format!("{code}: {message}"),
        },
        404 => AzureError::NotFound {
            resource: message.to_string(),
        },
        409 => AzureError::ResourceConflict {
            message: message.to_string(),
        },
        429 => AzureError::Throttled {
            retry_after: None,
            message: message.to_string(),
        },
        _ if code == "TooManyRequests" || code == "429" => AzureError::Throttled {
            retry_after: None,
            message: message.to_string(),
        },
        _ => AzureError::ServiceError {
            code: code.to_string(),
            message: message.to_string(),
            status,
        },
    }
}

/// Parse an Azure JSON error response.
///
/// ARM error format:
/// ```json
/// {"error": {"code": "ResourceNotFound", "message": "..."}}
/// ```
pub(crate) fn parse_json_error(status: u16, body: &str) -> AzureError {
    let parsed: std::result::Result<serde_json::Value, _> = serde_json::from_str(body);
    let (code, message) = match parsed {
        Ok(val) => {
            let error_obj = val.get("error").unwrap_or(&val);
            let code = error_obj
                .get("code")
                .and_then(|v| v.as_str())
                .unwrap_or_default()
                .to_string();
            let message = error_obj
                .get("message")
                .and_then(|v| v.as_str())
                .unwrap_or_default()
                .to_string();
            (code, message)
        }
        Err(_) => (String::new(), truncate_body(body)),
    };

    if code.is_empty() {
        return AzureError::ServiceError {
            code: format!("HttpError{status}"),
            message,
            status,
        };
    }

    classify_error(status, &code, &message)
}

/// Truncate a body string for error messages.
fn truncate_body(body: &str) -> String {
    if body.len() > 200 {
        let end = body.floor_char_boundary(200);
        format!("{}...", &body[..end])
    } else {
        body.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn throttled_is_retryable() {
        let err = AzureError::Throttled {
            retry_after: None,
            message: "slow down".into(),
        };
        assert!(err.is_retryable());
    }

    #[test]
    fn network_is_retryable() {
        let err = AzureError::Network("timeout".into());
        assert!(err.is_retryable());
    }

    #[test]
    fn auth_is_not_retryable() {
        let err = AzureError::Auth {
            message: "bad creds".into(),
        };
        assert!(!err.is_retryable());
    }

    #[test]
    fn service_error_500_is_retryable() {
        let err = AzureError::ServiceError {
            code: "InternalError".into(),
            message: "internal".into(),
            status: 500,
        };
        assert!(err.is_retryable());
    }

    #[test]
    fn service_error_400_is_not_retryable() {
        let err = AzureError::ServiceError {
            code: "ValidationError".into(),
            message: "bad param".into(),
            status: 400,
        };
        assert!(!err.is_retryable());
    }

    #[test]
    fn parse_json_error_arm_format() {
        let body = r#"{"error": {"code": "ResourceNotFound", "message": "Resource not found"}}"#;
        let err = parse_json_error(404, body);
        assert!(matches!(err, AzureError::NotFound { .. }));
    }

    #[test]
    fn parse_json_error_flat_format() {
        let body = r#"{"code": "Unauthorized", "message": "Token expired"}"#;
        let err = parse_json_error(401, body);
        assert!(matches!(err, AzureError::Auth { .. }));
    }

    #[test]
    fn parse_json_error_fallback_on_invalid() {
        let err = parse_json_error(500, "not json");
        match err {
            AzureError::ServiceError { code, status, .. } => {
                assert_eq!(code, "HttpError500");
                assert_eq!(status, 500);
            }
            other => panic!("expected ServiceError, got: {other}"),
        }
    }

    #[test]
    fn classify_409_as_conflict() {
        let err = classify_error(409, "Conflict", "already exists");
        assert!(matches!(err, AzureError::ResourceConflict { .. }));
    }

    #[test]
    fn retry_after_returns_duration_for_throttled() {
        let err = AzureError::Throttled {
            retry_after: Some(Duration::from_secs(5)),
            message: "slow down".into(),
        };
        assert_eq!(err.retry_after(), Some(Duration::from_secs(5)));
    }

    #[test]
    fn retry_after_returns_none_for_non_throttled() {
        let err = AzureError::Auth {
            message: "bad creds".into(),
        };
        assert_eq!(err.retry_after(), None);
    }
}
