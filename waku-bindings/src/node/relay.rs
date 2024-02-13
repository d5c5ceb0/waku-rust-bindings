//! Waku [relay](https://rfc.vac.dev/spec/36/#waku-relay) protocol related methods

// std
use std::ffi::CString;
use std::time::Duration;
// crates
use libc::*;
// internal
use crate::general::{Encoding, MessageId, Result, WakuContentTopic, WakuMessage, WakuPubSubTopic};
use crate::utils::{get_trampoline, handle_json_response, handle_no_response, handle_response};

/// Create a content topic according to [RFC 23](https://rfc.vac.dev/spec/23/)
/// As per the [specification](https://rfc.vac.dev/spec/36/#extern-char-waku_content_topicchar-applicationname-unsigned-int-applicationversion-char-contenttopicname-char-encoding)
pub fn waku_create_content_topic(
    ctx: *mut c_void,
    application_name: &str,
    application_version: u32,
    content_topic_name: &str,
    encoding: Encoding,
) -> WakuContentTopic {
    let application_name_ptr = CString::new(application_name)
        .expect("Application name should always transform to CString")
        .into_raw();
    let content_topic_name_ptr = CString::new(content_topic_name)
        .expect("Content topic should always transform to CString")
        .into_raw();
    let encoding_ptr = CString::new(encoding.to_string())
        .expect("Encoding should always transform to CString")
        .into_raw();

    let mut result: String = Default::default();
    let result_cb = |v: &str| result = v.to_string();
    let code = unsafe {
        let mut closure = result_cb;
        let cb = get_trampoline(&closure);
        let out = waku_sys::waku_content_topic(
            ctx,
            application_name_ptr,
            application_version,
            content_topic_name_ptr,
            encoding_ptr,
            cb,
            &mut closure as *mut _ as *mut c_void,
        );

        drop(CString::from_raw(application_name_ptr));
        drop(CString::from_raw(content_topic_name_ptr));
        drop(CString::from_raw(encoding_ptr));

        out
    };

    handle_response::<WakuContentTopic>(code, &result)
        .expect("&str from result should always be extracted")
}

/// Default pubsub topic used for exchanging waku messages defined in [RFC 10](https://rfc.vac.dev/spec/10/)
pub fn waku_default_pubsub_topic(ctx: *mut c_void) -> WakuPubSubTopic {
    let mut result: String = Default::default();
    let result_cb = |v: &str| result = v.to_string();
    let code = unsafe {
        let mut closure = result_cb;
        let cb = get_trampoline(&closure);
        waku_sys::waku_default_pubsub_topic(ctx, cb, &mut closure as *mut _ as *mut c_void)
    };

    handle_response(code, &result).expect("&str from result should always be extracted")
}

/// Publish a message using Waku Relay
/// As per the [specification](https://rfc.vac.dev/spec/36/#extern-char-waku_relay_publishchar-messagejson-char-pubsubtopic-int-timeoutms)
pub fn waku_relay_publish_message(
    ctx: *mut c_void,
    message: &WakuMessage,
    pubsub_topic: &WakuPubSubTopic,
    timeout: Option<Duration>,
) -> Result<MessageId> {
    let pubsub_topic = pubsub_topic.to_string();

    let message_ptr = CString::new(
        serde_json::to_string(&message)
            .expect("WakuMessages should always be able to success serializing"),
    )
    .expect("CString should build properly from the serialized waku message")
    .into_raw();
    let pubsub_topic_ptr = CString::new(pubsub_topic)
        .expect("CString should build properly from pubsub topic")
        .into_raw();

    let mut result: String = Default::default();
    let result_cb = |v: &str| result = v.to_string();
    let code = unsafe {
        let mut closure = result_cb;
        let cb = get_trampoline(&closure);
        let out = waku_sys::waku_relay_publish(
            ctx,
            message_ptr,
            pubsub_topic_ptr,
            timeout
                .map(|duration| {
                    duration
                        .as_millis()
                        .try_into()
                        .expect("Duration as milliseconds should fit in a u32")
                })
                .unwrap_or(0),
            cb,
            &mut closure as *mut _ as *mut c_void,
        );

        drop(CString::from_raw(message_ptr));
        drop(CString::from_raw(pubsub_topic_ptr));

        out
    };

    handle_response(code, &result)
}

pub fn waku_relay_subscribe(ctx: *mut c_void, pubsub_topic: &WakuPubSubTopic) -> Result<()> {
    let pubsub_topic = pubsub_topic.to_string();
    let pubsub_topic_ptr = CString::new(pubsub_topic)
        .expect("CString should build properly from pubsub topic")
        .into_raw();

    let mut error: String = Default::default();
    let error_cb = |v: &str| error = v.to_string();
    let code = unsafe {
        let mut closure = error_cb;
        let cb = get_trampoline(&closure);
        let out = waku_sys::waku_relay_subscribe(
            ctx,
            pubsub_topic_ptr,
            cb,
            &mut closure as *mut _ as *mut c_void,
        );

        drop(CString::from_raw(pubsub_topic_ptr));

        out
    };

    handle_no_response(code, &error)
}

pub fn waku_relay_unsubscribe(ctx: *mut c_void, pubsub_topic: &WakuPubSubTopic) -> Result<()> {
    let pubsub_topic = pubsub_topic.to_string();
    let pubsub_topic_ptr = CString::new(pubsub_topic)
        .expect("CString should build properly from pubsub topic")
        .into_raw();

    let mut error: String = Default::default();
    let error_cb = |v: &str| error = v.to_string();
    let code = unsafe {
        let mut closure = error_cb;
        let cb = get_trampoline(&closure);
        let out = waku_sys::waku_relay_subscribe(
            ctx,
            pubsub_topic_ptr,
            cb,
            &mut closure as *mut _ as *mut c_void,
        );

        drop(CString::from_raw(pubsub_topic_ptr));

        out
    };

    handle_no_response(code, &error)
}
