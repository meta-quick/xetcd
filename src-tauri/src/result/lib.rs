pub(crate) type GenericError = Box<dyn std::error::Error + Send + Sync + 'static>;
pub(crate) type GenericResult<T> = Result<T,GenericError>;
