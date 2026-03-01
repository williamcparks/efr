use thiserror::Error;

#[derive(Debug, Error)]
pub enum EfrCodesHeaderError {
    /// a pem parsing / serialization error
    #[error(transparent)]
    Pem(#[from] pem::PemError),

    /// a der parsing / serialization error
    #[error(transparent)]
    Der(#[from] der::Error),
}
