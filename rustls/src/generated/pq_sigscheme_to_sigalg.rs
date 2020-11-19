match self.scheme {
    SignatureScheme::DILITHIUM2 => SignatureAlgorithm::DILITHIUM2,
    SignatureScheme::DILITHIUM3 => SignatureAlgorithm::DILITHIUM3,
    SignatureScheme::FALCON512 => SignatureAlgorithm::FALCON512,
    SignatureScheme::SPHINCSSHA256128FROBUST => SignatureAlgorithm::SPHINCSSHA256128FROBUST,
    SignatureScheme::SPHINCSSHA256128FSIMPLE => SignatureAlgorithm::SPHINCSSHA256128FSIMPLE,
    _ => unreachable!(),}