// Copyright 2015 Brian Smith.
//
// Permission to use, copy, modify, and/or distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHORS DISCLAIM ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHORS BE LIABLE FOR
// ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
// ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
// OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.

use core::fmt;
use core::ops::ControlFlow;

/// An error that occurs during certificate validation or name validation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum Error {
    /// The encoding of some ASN.1 DER-encoded item is invalid.
    BadDer,

    /// The encoding of an ASN.1 DER-encoded time is invalid.
    BadDerTime,

    /// A CA certificate is being used as an end-entity certificate.
    CaUsedAsEndEntity,

    /// The certificate is expired; i.e. the time it is being validated for is
    /// later than the certificate's notAfter time.
    CertExpired,

    /// The certificate is not valid for the name it is being validated for.
    CertNotValidForName,

    /// The certificate is not valid yet; i.e. the time it is being validated
    /// for is earlier than the certificate's notBefore time.
    CertNotValidYet,

    /// An end-entity certificate is being used as a CA certificate.
    EndEntityUsedAsCa,

    /// An X.509 extension is invalid.
    ExtensionValueInvalid,

    /// The certificate validity period (notBefore, notAfter) is invalid; e.g.
    /// the notAfter time is earlier than the notBefore time.
    InvalidCertValidity,

    /// A iPAddress name constraint was invalid:
    /// - it had a sparse network mask (ie, cannot be written in CIDR form).
    /// - it was too long or short
    InvalidNetworkMaskConstraint,

    /// The signature is invalid for the given public key.
    InvalidSignatureForPublicKey,

    /// The certificate extensions are malformed.
    ///
    /// In particular, webpki requires the DNS name(s) be in the subjectAltName
    /// extension as required by the CA/Browser Forum Baseline Requirements
    /// and as recommended by RFC6125.
    MalformedExtensions,

    /// The maximum number of name constraint comparisons has been reached.
    MaximumNameConstraintComparisonsExceeded,

    /// The maximum number of internal path building calls has been reached. Path complexity is too great.
    MaximumPathBuildCallsExceeded,

    /// The path search was terminated because it became too deep.
    MaximumPathDepthExceeded,

    /// The maximum number of signature checks has been reached. Path complexity is too great.
    MaximumSignatureChecksExceeded,

    /// The certificate violates one or more name constraints.
    NameConstraintViolation,

    /// The certificate violates one or more path length constraints.
    PathLenConstraintViolated,

    /// The certificate is not valid for the Extended Key Usage for which it is
    /// being validated.
    RequiredEkuNotFound,

    /// The algorithm in the TBSCertificate "signature" field of a certificate
    /// does not match the algorithm in the signature of the certificate.
    SignatureAlgorithmMismatch,

    /// A valid issuer for the certificate could not be found.
    UnknownIssuer,

    /// The certificate is not a v3 X.509 certificate.
    ///
    /// This error may be also reported if the certificate version field
    /// is malformed.
    UnsupportedCertVersion,

    /// The certificate contains an unsupported critical extension.
    UnsupportedCriticalExtension,

    /// The signature algorithm for a signature is not in the set of supported
    /// signature algorithms given.
    UnsupportedSignatureAlgorithm,

    /// The signature's algorithm does not match the algorithm of the public
    /// key it is being validated for. This may be because the public key
    /// algorithm's OID isn't recognized (e.g. DSA), or the public key
    /// algorithm's parameters don't match the supported parameters for that
    /// algorithm (e.g. ECC keys for unsupported curves), or the public key
    /// algorithm and the signature algorithm simply don't match (e.g.
    /// verifying an RSA signature with an ECC public key).
    UnsupportedSignatureAlgorithmForPublicKey,
}

impl Error {
    /// Returns true for errors that should be considered fatal during path building. Errors of
    /// this class should halt any further path building and be returned immediately.
    #[inline]
    pub(crate) fn is_fatal(&self) -> bool {
        matches!(
            self,
            Error::MaximumSignatureChecksExceeded
                | Error::MaximumPathBuildCallsExceeded
                | Error::MaximumNameConstraintComparisonsExceeded
        )
    }
}

impl From<Error> for ControlFlow<Error, Error> {
    fn from(value: Error) -> Self {
        match value {
            // If an error is fatal, we've exhausted the potential for continued search.
            err if err.is_fatal() => Self::Break(err),
            // Otherwise we've rejected one candidate chain, but may continue to search for others.
            err => Self::Continue(err),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Requires the `std` feature.
#[cfg(feature = "std")]
impl ::std::error::Error for Error {}
