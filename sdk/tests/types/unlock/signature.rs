// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_sdk::types::block::{rand::signature::rand_signature, unlock::SignatureUnlock, Error};
use packable::{error::UnpackError, PackableExt};

#[test]
fn unlock_kind() {
    assert_eq!(SignatureUnlock::KIND, 0);
}

#[test]
fn signature_kind() {
    assert_eq!(SignatureUnlock::from(rand_signature()).kind(), 0);
}

#[test]
fn packed_len() {
    let signature = SignatureUnlock::from(rand_signature());

    assert_eq!(signature.packed_len(), 1 + 32 + 64);
    assert_eq!(signature.pack_to_vec().len(), 1 + 32 + 64);
}

#[test]
fn pack_unpack_valid_ed25519() {
    let signature_1 = SignatureUnlock::from(rand_signature());
    let signature_bytes = signature_1.pack_to_vec();
    let signature_2 = SignatureUnlock::unpack_verified(signature_bytes.as_slice(), &()).unwrap();

    assert_eq!(signature_bytes[0], 0);
    assert_eq!(signature_1, signature_2);
}

#[test]
fn pack_unpack_invalid_kind() {
    assert!(matches!(
        SignatureUnlock::unpack_verified(
            [
                1, 111, 225, 221, 28, 247, 253, 234, 110, 187, 52, 129, 153, 130, 84, 26, 7, 226, 27, 212, 145, 96,
                151, 196, 124, 135, 176, 31, 48, 0, 213, 200, 82, 227, 169, 21, 179, 253, 115, 184, 209, 107, 138, 0,
                62, 252, 20, 255, 24, 193, 203, 255, 137, 142, 158, 25, 171, 86, 195, 20, 70, 56, 136, 204, 2, 219,
                254, 218, 2, 234, 91, 56, 50, 122, 112, 200, 110, 181, 15, 166, 100, 53, 115, 124, 220, 90, 50, 188,
                45, 124, 67, 30, 124, 38, 34, 89, 92
            ],
            &()
        ),
        Err(UnpackError::Packable(Error::InvalidSignatureKind(1)))
    ));
}
