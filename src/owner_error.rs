//! Owner-boundary refusal types for the filament-mltl wire readers.
//!
//! tl-mltl 0.4 removed `ContractMismatch` and `IdentityMismatch` from its
//! `OwnerReadErrorCode`, but FR-001-AC-13 requires them as distinct codes on
//! the filament-mltl readers. [`OwnerReadErrorCode`] therefore carries every
//! tl-mltl code plus those two, with identical wire labels, and
//! [`OwnerReadError`] converts losslessly from tl-mltl's error.

use std::fmt;

use tl_mltl::wire::{OwnerReadError as TlError, OwnerReadErrorCode as TlCode, OwnerUsage};

/// Stable machine-readable refusal codes of the filament-mltl owner readers.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OwnerReadErrorCode {
    /// Input is not UTF-8.
    InvalidUtf8,
    /// Input is not exactly one valid JSON document.
    InvalidJson,
    /// Input is valid but not the one canonical encoding.
    NonCanonical,
    /// A resource ceiling was exceeded.
    ResourceIncomplete,
    /// An independently supplied owner input does not match.
    ExpectedMismatch,
    /// A profile, topology, state, or lineage combination is invalid.
    InvalidCombination,
    /// Canonical encoding failed.
    Encoding,
    /// The document's contract version is not the supported one.
    ContractMismatch,
    /// The document's content identity does not match its content.
    IdentityMismatch,
}

impl OwnerReadErrorCode {
    /// Stable wire label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InvalidUtf8 => "TL-OWNER-INVALID-UTF8",
            Self::InvalidJson => "TL-OWNER-INVALID-JSON",
            Self::NonCanonical => "TL-OWNER-NONCANONICAL",
            Self::ResourceIncomplete => "TL-OWNER-RESOURCE-INCOMPLETE",
            Self::ExpectedMismatch => "TL-OWNER-EXPECTED-MISMATCH",
            Self::InvalidCombination => "TL-OWNER-INVALID-COMBINATION",
            Self::Encoding => "TL-OWNER-ENCODING",
            Self::ContractMismatch => "TL-OWNER-CONTRACT-MISMATCH",
            Self::IdentityMismatch => "TL-OWNER-IDENTITY-MISMATCH",
        }
    }
}

impl From<TlCode> for OwnerReadErrorCode {
    fn from(code: TlCode) -> Self {
        match code {
            TlCode::InvalidUtf8 => Self::InvalidUtf8,
            TlCode::InvalidJson => Self::InvalidJson,
            TlCode::NonCanonical => Self::NonCanonical,
            TlCode::ResourceIncomplete => Self::ResourceIncomplete,
            TlCode::ExpectedMismatch => Self::ExpectedMismatch,
            TlCode::InvalidCombination => Self::InvalidCombination,
            TlCode::Encoding => Self::Encoding,
        }
    }
}

/// Typed fail-closed owner boundary error.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OwnerReadError {
    code: OwnerReadErrorCode,
    field: &'static str,
    usage: OwnerUsage,
}

impl OwnerReadError {
    /// Constructs a typed owner-boundary refusal.
    pub const fn new(code: OwnerReadErrorCode, field: &'static str, usage: OwnerUsage) -> Self {
        Self { code, field, usage }
    }

    /// Stable refusal code.
    #[must_use]
    pub const fn code(&self) -> OwnerReadErrorCode {
        self.code
    }

    /// Bounded field/category identifying the refusal locus.
    #[must_use]
    pub const fn field(&self) -> &'static str {
        self.field
    }

    /// Work observed before refusal.
    #[must_use]
    pub const fn usage(&self) -> OwnerUsage {
        self.usage
    }
}

impl From<TlError> for OwnerReadError {
    fn from(error: TlError) -> Self {
        Self::new(error.code().into(), error.field(), error.usage())
    }
}

impl fmt::Display for OwnerReadError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}: {}", self.code.as_str(), self.field)
    }
}

impl std::error::Error for OwnerReadError {}
