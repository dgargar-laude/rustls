
enum_builder! {
    /// The `SignatureAlgorithm` TLS protocol enum.  Values in this enum are taken
    /// from the various RFCs covering TLS, and are listed by IANA.
    /// The `Unknown` item is used when processing unrecognised ordinals.
    @U8
    EnumName: SignatureAlgorithm;
    EnumVal{
        Anonymous => 0x00,
        RSA => 0x01,
        DSA => 0x02,
        ECDSA => 0x03,
        ED25519 => 0x07,
        ED448 => 0x08,
        KEMTLS => 0xdf,
        DILITHIUM2 => 0xe0,
        DILITHIUM3 => 0xe1,
        FALCON512 => 0xe2,
        SPHINCSSHA256128FROBUST => 0xe3,
        SPHINCSSHA256128FSIMPLE => 0xe4,
    }
}