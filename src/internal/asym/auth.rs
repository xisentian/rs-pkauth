
use crypto_abstract::{ToPublicKey};
use crypto_abstract::asym::auth::{PublicKey, PrivateKey};

use {ToAlgorithm, ToIdentifier};
use internal::{PKAIdentifier, generate_identifier, EncodePSF, PSF};

impl ToIdentifier for PublicKey {
    fn to_identifier( key : &PublicKey) -> PKAIdentifier {
        let serialized = EncodePSF::encode_psf( key);

        generate_identifier( serialized)
    }
}

impl ToIdentifier for PrivateKey {
    fn to_identifier( key : &PrivateKey) -> PKAIdentifier {
        ToIdentifier::to_identifier( &ToPublicKey::to_public_key( key))
    }
}

impl EncodePSF for PublicKey {
    fn encode_psf( _ : &PublicKey) -> PSF<PublicKey> {
        unimplemented!()
    }
}
