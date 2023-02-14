//! Waku [store](https://rfc.vac.dev/spec/36/#waku-store) handling methods

// std
use std::ffi::CString;
use std::time::Duration;
// crates
// internal
use crate::general::{PeerId, Result, StoreQuery, StoreResponse};
use crate::utils::decode_and_free_response;

/// Retrieves historical messages on specific content topics. This method may be called with [`PagingOptions`](`crate::general::PagingOptions`),
/// to retrieve historical messages on a per-page basis. If the request included [`PagingOptions`](`crate::general::PagingOptions`),
/// the node must return messages on a per-page basis and include [`PagingOptions`](`crate::general::PagingOptions`) in the response.
/// These [`PagingOptions`](`crate::general::PagingOptions`) must contain a cursor pointing to the Index from which a new page can be requested
pub fn waku_store_query(
    query: &StoreQuery,
    peer_id: &PeerId,
    timeout: Option<Duration>,
) -> Result<StoreResponse> {
    let query_ptr = CString::new(
        serde_json::to_string(query).expect("StoreQuery should always be able to be serialized"),
    )
    .expect("CString should build properly from the serialized filter subscription")
    .into_raw();
    let peer_id_ptr = CString::new(peer_id.clone())
        .expect("CString should build properly from peer id")
        .into_raw();

    let result_ptr = unsafe {
        let res = waku_sys::waku_store_query(
            query_ptr,
            peer_id_ptr,
            timeout
                .map(|timeout| {
                    timeout
                        .as_millis()
                        .try_into()
                        .expect("Duration as milliseconds should fit in a i32")
                })
                .unwrap_or(0),
        );
        drop(CString::from_raw(query_ptr));
        drop(CString::from_raw(peer_id_ptr));
        res
    };

    decode_and_free_response(result_ptr)
}

/// Retrieves locally stored historical messages on specific content topics from the local archive system. This method may be called with [`PagingOptions`](`crate::general::PagingOptions`),
/// to retrieve historical messages on a per-page basis. If the request included [`PagingOptions`](`crate::general::PagingOptions`),
/// the node must return messages on a per-page basis and include [`PagingOptions`](`crate::general::PagingOptions`) in the response.
/// These [`PagingOptions`](`crate::general::PagingOptions`) must contain a cursor pointing to the Index from which a new page can be requested
pub fn waku_local_store_query(query: &StoreQuery) -> Result<StoreResponse> {
    let query_ptr = CString::new(
        serde_json::to_string(query).expect("StoreQuery should always be able to be serialized"),
    )
    .expect("CString should build properly from the serialized filter subscription")
    .into_raw();
    let result_ptr = unsafe {
        let res = waku_sys::waku_store_local_query(query_ptr);
        drop(CString::from_raw(query_ptr));
        res
    };
    decode_and_free_response(result_ptr)
}
