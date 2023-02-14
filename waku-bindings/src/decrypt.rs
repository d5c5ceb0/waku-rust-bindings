//! Symmetric and asymmetric waku messages [decrypting](https://rfc.vac.dev/spec/36/#decrypting-messages) methods

// std
use std::ffi::{CStr, CString};
// crates
use aes_gcm::{Aes256Gcm, Key};
use secp256k1::SecretKey;
// internal
use crate::general::{DecodedPayload, JsonResponse, Result, WakuMessage};

/// Decrypt a message using a symmetric key
///
/// As per the [specification](https://rfc.vac.dev/spec/36/#extern-char-waku_decode_symmetricchar-messagejson-char-symmetrickey)
pub fn waku_decode_symmetric(
    message: &WakuMessage,
    symmetric_key: &Key<Aes256Gcm>,
) -> Result<DecodedPayload> {
    let symk = hex::encode(symmetric_key.as_slice());

    let message_ptr = CString::new(
        serde_json::to_string(&message)
            .expect("WakuMessages should always be able to success serializing"),
    )
    .expect("CString should build properly from the serialized waku message")
    .into_raw();
    let symk_ptr = CString::new(symk)
        .expect("CString should build properly from hex encoded symmetric key")
        .into_raw();

    let result_ptr = unsafe {
        let res = waku_sys::waku_decode_symmetric(message_ptr, symk_ptr);
        drop(CString::from_raw(message_ptr));
        drop(CString::from_raw(symk_ptr));
        res
    };

    let result = unsafe { CStr::from_ptr(result_ptr) }
        .to_str()
        .expect("Response should always succeed to load to a &str");

    let response: JsonResponse<DecodedPayload> =
        serde_json::from_str(result).map_err(|e| format!("{e}"))?;

    unsafe { waku_sys::waku_utils_free(result_ptr) };

    response.into()
}

/// Decrypt a message using a symmetric key
///
/// As per the [specification](https://rfc.vac.dev/spec/36/#extern-char-waku_decode_asymmetricchar-messagejson-char-privatekey)
pub fn waku_decode_asymmetric(
    message: &WakuMessage,
    asymmetric_key: &SecretKey,
) -> Result<DecodedPayload> {
    let sk = hex::encode(asymmetric_key.secret_bytes());

    let message_ptr = CString::new(
        serde_json::to_string(&message)
            .expect("WakuMessages should always be able to success serializing"),
    )
    .expect("CString should build properly from the serialized waku message")
    .into_raw();
    let sk_ptr = CString::new(sk)
        .expect("CString should build properly from hex encoded symmetric key")
        .into_raw();

    let result_ptr = unsafe {
        let res = waku_sys::waku_decode_asymmetric(message_ptr, sk_ptr);
        drop(CString::from_raw(message_ptr));
        drop(CString::from_raw(sk_ptr));
        res
    };

    let result = unsafe { CStr::from_ptr(result_ptr) }
        .to_str()
        .expect("Response should always succeed to load to a &str");

    let response: JsonResponse<DecodedPayload> =
        serde_json::from_str(result).map_err(|e| format!("{e}"))?;

    unsafe { waku_sys::waku_utils_free(result_ptr) };

    response.into()
}
