//! # Waku
//!
//! Implementation on top of [`waku-bindings`](https://rfc.vac.dev/spec/36/)
mod general;
pub mod node;
pub mod utils;
use std::time::SystemTime;

// Re-export the LibwakuResponse type to make it accessible outside this module
pub use utils::LibwakuResponse;

// Required so functions inside libwaku can call RLN functions even if we
// use it within the bindings functions
#[allow(clippy::single_component_path_imports)]
#[allow(unused)]
use rln;

pub use node::{
    waku_create_content_topic, waku_destroy, waku_new, Event, Initialized, Key, Multiaddr,
    PublicKey, RLNConfig, Running, SecretKey, WakuMessageEvent, WakuNodeConfig, WakuNodeContext,
    WakuNodeHandle,
};

pub use general::{
    Encoding, MessageHash, Result, WakuContentTopic, WakuMessage, WakuMessageVersion,
};

#[no_mangle]
pub fn say_hello(waku: &WakuNodeHandle) {
    println!("hello world");
}

#[derive(Debug)]
struct Response {
    payload: Vec<u8>,
}

#[no_mangle]
pub fn waku_new_wrapper(
    cluster_id: usize,
    shards: Vec<usize>,
    discovery_url: &'static str,
    node_key: Option<SecretKey>,
    topic: &str,
) -> Result<WakuNodeHandle> {
    //let topic = "/waku/2/rs/16/32";
    let waku = waku_new(Some(WakuNodeConfig {
        port: Some(60010),
        cluster_id: Some(cluster_id),
        shards,
        //cluster_id: Some(16),
        //shards: vec![1, 32, 64, 128, 256],
        // node_key: Some(SecretKey::from_str("2fc0515879e52b7b73297cfd6ab3abf7c344ef84b7a90ff6f4cc19e05a198027").unwrap()),
        node_key: node_key,
        max_message_size: Some("1024KiB".to_string()),
        relay_topics: vec![topic.to_string()],
        log_level: Some("ERROR"), // Supported: TRACE, DEBUG, INFO, NOTICE, WARN, ERROR or FATAL

        keep_alive: Some(true),

        // Discovery
        dns_discovery: Some(true),
        //dns_discovery_url: Some("enrtree://AMOJVZX4V6EXP7NTJPMAYJYST2QP6AJXYW76IU6VGJS7UVSNDYZG4@boot.prod.status.nodes.status.im"),
        dns_discovery_url: Some(discovery_url),
        discv5_discovery: Some(true),
        discv5_udp_port: Some(9000),
        discv5_enr_auto_update: Some(false),

        ..Default::default()
    }))
    .expect("should instantiate");

    if let Ok(_) = waku.start() {
        println!("It works!");
    } else {
        println!("It didn't work. Let's go home xD");
    }

    Ok(waku)
}

#[no_mangle]
pub fn waku_listen(
    waku: &WakuNodeHandle,
    pubsub_topic: &str,
    content_topic: &str,
    tx: tokio::sync::mpsc::Sender<Response>,
) -> Result<()> {
    //let pubsub_topic: WakuPubSubTopic = pubsub_topic.parse().unwrap();
    let content_topic: WakuContentTopic = content_topic.parse().unwrap();

    let my_closure = move |response| {
        if let LibwakuResponse::Success(v) = response {
            let event: Event =
                serde_json::from_str(v.unwrap().as_str()).expect("Parsing event to succeed");

            match event {
                Event::WakuMessage(evt) => {
                    //println!("WakuMessage event received: {:?}", evt.waku_message.content_topic);
                    if evt.waku_message.content_topic == content_topic {
                        println!("WakuMessage event received: {:?}", evt.waku_message);
                        futures::executor::block_on(tx.send(Response {
                            payload: evt.waku_message.payload,
                        }))
                        .expect("send response to the receiver");
                    }
                }
                Event::Unrecognized(err) => panic!("Unrecognized waku event: {:?}", err),
                _ => panic!("event case not expected"),
            };
        }
    };

    // Establish a closure that handles the incoming messages
    waku.ctx.waku_set_event_callback(my_closure);
    waku.relay_subscribe(&pubsub_topic.to_string())
        .expect("waku should subscribe");

    // Wait for Ctrl+C (SIGINT) signal
    //signal::ctrl_c()
    //    .await
    //    .expect("Failed to listen for Ctrl+C signal");

    //println!("Ctrl+C pressed, shutting down.");

    //waku.stop().expect("Failed to stop Waku");

    //signal::ctrl_c().await?;
    //println!("ctrl-c received!");
    Ok(())
}

#[no_mangle]
pub fn waku_send(waku: &WakuNodeHandle, pubsub_topic: &str, content_topic: &str, payload: String) {
    let content_topic: WakuContentTopic = content_topic.parse().unwrap();

    let message = WakuMessage::new(
        payload,
        content_topic,
        1,
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_millis()
            .try_into()
            .unwrap(),
        Vec::new(),
        false,
    );
    match waku.relay_publish_message(&message, &pubsub_topic.to_string(), None) {
        Ok(r) => println!("ok: {:?}", r),
        Err(e) => println!("err: {:?}", e),
    };
}
