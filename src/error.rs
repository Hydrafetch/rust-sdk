use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("hydrafetch: {message} [{code}]{}", .request_id.as_ref().map(|r| format!(" (request {r})")).unwrap_or_default())]
    Api {
        code: String,
        message: String,
        status: u16,
        request_id: Option<String>,
    },

    #[error("hydrafetch: {0}")]
    Timeout(String),

    #[error("hydrafetch: no API key: pass one to Client::new or set HYDRAFETCH_API_KEY (https://app.hydrafetch.com)")]
    MissingApiKey,

    #[error("hydrafetch: transport: {0}")]
    Transport(#[from] reqwest::Error),

    #[error("hydrafetch: decode: {0}")]
    Decode(#[from] serde_json::Error),
}

impl Error {
    pub fn status(&self) -> Option<u16> {
        match self {
            Error::Api { status, .. } => Some(*status),
            Error::Timeout(_) => Some(408),
            Error::MissingApiKey => Some(401),
            _ => None,
        }
    }

    pub fn code(&self) -> &str {
        match self {
            Error::Api { code, .. } => code,
            Error::Timeout(_) => "TIMEOUT",
            Error::MissingApiKey => "MISSING_API_KEY",
            Error::Transport(_) => "TRANSPORT",
            Error::Decode(_) => "DECODE",
        }
    }

    pub fn request_id(&self) -> Option<&str> {
        match self {
            Error::Api { request_id, .. } => request_id.as_deref(),
            _ => None,
        }
    }

    pub fn is_auth(&self) -> bool {
        matches!(self.status(), Some(401) | Some(403))
    }

    pub fn is_out_of_credits(&self) -> bool {
        self.status() == Some(402)
    }

    pub fn is_invalid_request(&self) -> bool {
        matches!(self.status(), Some(400) | Some(422))
    }

    pub fn is_retryable(&self) -> bool {
        matches!(self.status(), Some(429)) || matches!(self.status(), Some(s) if s >= 500)
    }

    pub fn is_timeout(&self) -> bool {
        matches!(self, Error::Timeout(_))
    }
}

pub type Result<T> = std::result::Result<T, Error>;
