//! Owner-boundary refusal types for the filament-mltl wire readers.
//!
//! tl-mltl 0.4 removed `ContractMismatch` and `IdentityMismatch` from its
//! `OwnerReadErrorCode`, but FR-001-AC-13 requires them as distinct codes on
//! the filament-mltl readers. [`OwnerReadErrorCode`] therefore carries every
//! tl-mltl code plus those two, with identical wire labels, and
//! [`OwnerReadError`] converts losslessly from tl-mltl's error.

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
            Self::InvalidUtf8 => TlCode::InvalidUtf8.as_str(),
            Self::InvalidJson => TlCode::InvalidJson.as_str(),
            Self::NonCanonical => TlCode::NonCanonical.as_str(),
            Self::ResourceIncomplete => TlCode::ResourceIncomplete.as_str(),
            Self::ExpectedMismatch => TlCode::ExpectedMismatch.as_str(),
            Self::InvalidCombination => TlCode::InvalidCombination.as_str(),
            Self::Encoding => TlCode::Encoding.as_str(),
            Self::ContractMismatch => "TL-OWNER-CONTRACT-MISMATCH",
            Self::IdentityMismatch => "TL-OWNER-IDENTITY-MISMATCH",
        }
    }

    /// Every supported condition in stable catalog order.
    #[must_use]
    pub const fn all() -> &'static [Self] {
        &[
            Self::InvalidUtf8,
            Self::InvalidJson,
            Self::NonCanonical,
            Self::ResourceIncomplete,
            Self::ExpectedMismatch,
            Self::InvalidCombination,
            Self::Encoding,
            Self::ContractMismatch,
            Self::IdentityMismatch,
        ]
    }

    /// Resolves an exact stable code, refusing unrecognized spellings.
    #[must_use]
    pub fn from_code(code: &str) -> Option<Self> {
        Self::all()
            .iter()
            .copied()
            .find(|item| item.as_str() == code)
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
#[derive(Clone, Debug, Eq, PartialEq, thiserror::Error)]
#[error("{}: {field}", .code.as_str())]
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

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::{OwnerReadError, OwnerReadErrorCode, OwnerUsage, TlCode, TlError};

    /// Trace: FR-001-AC-13
    #[test]
    fn shared_boundary_errors_retain_owner_labels_and_payloads() {
        let usage = OwnerUsage {
            wire_bytes: 7,
            depth: 2,
            ..OwnerUsage::default()
        };
        for code in [
            TlCode::InvalidUtf8,
            TlCode::InvalidJson,
            TlCode::NonCanonical,
            TlCode::ResourceIncomplete,
            TlCode::ExpectedMismatch,
            TlCode::InvalidCombination,
            TlCode::Encoding,
        ] {
            let source = TlError::new(code, "document", usage);
            let converted = OwnerReadError::from(source.clone());
            assert_eq!(converted.code().as_str(), code.as_str());
            assert_eq!(converted.field(), source.field());
            assert_eq!(converted.usage(), source.usage());
            assert_eq!(converted.to_string(), source.to_string());
        }
    }

    /// Trace: FR-001-AC-13
    #[test]
    fn owner_error_catalog_is_unique_and_round_trips_exactly() {
        let catalog = OwnerReadErrorCode::all();
        assert_eq!(catalog.len(), 9);
        let labels: BTreeSet<_> = catalog.iter().map(|code| code.as_str()).collect();
        assert_eq!(labels.len(), catalog.len());
        for &code in catalog {
            assert_eq!(OwnerReadErrorCode::from_code(code.as_str()), Some(code));
            assert_eq!(
                OwnerReadErrorCode::from_code(&format!(" {}", code.as_str())),
                None
            );
            assert_eq!(
                OwnerReadErrorCode::from_code(&code.as_str().to_ascii_lowercase()),
                None
            );
        }
        assert_eq!(OwnerReadErrorCode::from_code(""), None);
        assert_eq!(OwnerReadErrorCode::from_code("unknown"), None);
        assert_eq!(
            OwnerReadErrorCode::ContractMismatch.as_str(),
            "TL-OWNER-CONTRACT-MISMATCH"
        );
        assert_eq!(
            OwnerReadErrorCode::IdentityMismatch.as_str(),
            "TL-OWNER-IDENTITY-MISMATCH"
        );
    }
}
