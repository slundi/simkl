use std::fmt;

pub type Result<T> = std::result::Result<T, SimklError>;

#[derive(Debug)]
pub enum SimklError {
    /// Erreur de sérialisation/désérialisation JSON
    Json(serde_json::Error),
    /// URL invalide
    InvalidUrl(url::ParseError),
    /// Paramètres manquants ou invalides
    InvalidParameters(String),
    /// Erreur de parsing de la réponse
    ParseError(String),
}

impl fmt::Display for SimklError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SimklError::Json(err) => write!(f, "Erreur JSON: {}", err),
            SimklError::InvalidUrl(err) => write!(f, "URL invalide: {}", err),
            SimklError::InvalidParameters(msg) => write!(f, "Paramètres invalides: {}", msg),
            SimklError::ParseError(msg) => write!(f, "Erreur de parsing: {}", msg),
        }
    }
}

impl std::error::Error for SimklError {}

impl From<serde_json::Error> for SimklError {
    fn from(err: serde_json::Error) -> Self {
        SimklError::Json(err)
    }
}

impl From<url::ParseError> for SimklError {
    fn from(err: url::ParseError) -> Self {
        SimklError::InvalidUrl(err)
    }
}
