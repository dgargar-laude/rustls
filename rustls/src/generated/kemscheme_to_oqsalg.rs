match scheme {
    SignatureScheme::KEMTLS_KYBER512 => oqs::kem::Algorithm::Kyber512,
    SignatureScheme::KEMTLS_LIGHTSABER => oqs::kem::Algorithm::Lightsaber,
    SignatureScheme::KEMTLS_SIDHP434 => oqs::kem::Algorithm::SidhP434,
    _ => unreachable!(),}