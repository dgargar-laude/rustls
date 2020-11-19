match scheme {
    SignatureScheme::DILITHIUM2 => oqs::sig::Algorithm::Dilithium2,
    SignatureScheme::DILITHIUM3 => oqs::sig::Algorithm::Dilithium3,
    SignatureScheme::FALCON512 => oqs::sig::Algorithm::Falcon512,
    SignatureScheme::SPHINCSSHA256128FROBUST => oqs::sig::Algorithm::SphincsSha256128fRobust,
    SignatureScheme::SPHINCSSHA256128FSIMPLE => oqs::sig::Algorithm::SphincsSha256128fSimple,
    _ => unreachable!(),}