/// Errors that could be outputed by a [`Language`](crate::languages::Language).
#[derive(Debug, Clone, PartialEq, Hash)]
pub enum LanguageError {
    FormatError(String),
}

impl std::error::Error for LanguageError {}

impl std::fmt::Display for LanguageError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            LanguageError::FormatError(error) => {
                write!(f, "Unknown Language: {}", error)
            }
        }
    }
}

/// Errors that could be outputed by [`translate`](crate::translate).
#[derive(Debug, Clone, PartialEq, Hash)]
pub enum TranslateError {
    HttpError(String),
    ParseError(String),
    DetectError,
    LengthError,
}

impl std::error::Error for TranslateError {}

impl std::fmt::Display for TranslateError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            TranslateError::HttpError(error) => {
                write!(f, "HTTP request error: {}", error.to_string())
            }
            TranslateError::ParseError(error) => {
                write!(f, "JSON parsing error: {}", error.to_string())
            }
            TranslateError::DetectError => {
                write!(f, "Language detection error")
            }
            TranslateError::LengthError => {
                write!(f, "Requested text is too long")
            }
        }
    }
}