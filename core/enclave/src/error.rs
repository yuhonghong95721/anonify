use std::{
    io,
};
use thiserror::Error;
use anyhow::anyhow;

pub type Result<T> = std::result::Result<T, EnclaveError>;

#[derive(Error, Debug)]
pub enum EnclaveError {
    #[error("I/O error: {0}")]
    IoError(#[from] io::Error),

    #[error("Ed25519 error: {0}")]
    Ed25519Error(ed25519_dalek::SignatureError),

    #[error("Ring Error: {err:?}")]
    RingError { #[from] err: ring::error::Unspecified },

    #[error("Sgx Error: {err:?}")]
    SgxError { err: sgx_types::sgx_status_t },

    #[error("Hex error: {0}")]
    HexError(hex::FromHexError),

    #[error("Webpki error: {0}")]
    WebpkiError(#[from] webpki::Error),

    #[error("Base64 error: {0}")]
    Base64Error(#[from] base64::DecodeError),

    #[error("Secp256k1 error: {0:?}")]
    Secp256k1Error(secp256k1::Error),

    #[error("Codec error: {0:?}")]
    CodecError(codec::Error),

    #[error("Anyhow error: {0}")]
    AnyhowError(#[from] anyhow::Error),
}

impl From<ed25519_dalek::SignatureError> for EnclaveError {
    fn from(err: ed25519_dalek::SignatureError) -> Self {
        anyhow!("Ed25519 error: {:?}", err).into()
    }
}

impl From<sgx_types::sgx_status_t> for EnclaveError {
    fn from(err: sgx_types::sgx_status_t) -> Self {
        anyhow!("Sgx error: {:?}", err).into()
    }
}

impl From<hex::FromHexError> for EnclaveError {
    fn from(err: hex::FromHexError) -> Self {
        anyhow!("Hex error: {:?}", err).into()
    }
}

impl From<secp256k1::Error> for EnclaveError {
    fn from(err: secp256k1::Error) -> Self {
        anyhow!("Secp256k1 error: {:?}", err).into()
    }
}

impl From<codec::Error> for EnclaveError {
    fn from(err: codec::Error) -> Self {
        anyhow!("Codec error: {:?}", err).into()
    }
}
