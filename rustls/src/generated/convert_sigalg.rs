
match scheme {
    ECDSA_NISTP256_SHA256 => Ok(&webpki::ECDSA_P256_SHA256),
    ECDSA_NISTP384_SHA384 => Ok(&webpki::ECDSA_P384_SHA384),
    ED25519 => Ok(&webpki::ED25519),
    RSA_PSS_SHA256 => Ok(&webpki::RSA_PSS_2048_8192_SHA256_LEGACY_KEY),
    RSA_PSS_SHA384 => Ok(&webpki::RSA_PSS_2048_8192_SHA384_LEGACY_KEY),
    RSA_PSS_SHA512 => Ok(&webpki::RSA_PSS_2048_8192_SHA512_LEGACY_KEY),    DILITHIUM2 => Ok(&webpki::DILITHIUM2),
    DILITHIUM3 => Ok(&webpki::DILITHIUM3),
    FALCON512 => Ok(&webpki::FALCON512),
    SPHINCSSHA256128FROBUST => Ok(&webpki::SPHINCSSHA256128FROBUST),
    SPHINCSSHA256128FSIMPLE => Ok(&webpki::SPHINCSSHA256128FSIMPLE),

    _ => {
        let error_msg = format!("received unsupported sig scheme {:?}", scheme);
        Err(TLSError::PeerMisbehavedError(error_msg))
    }
}