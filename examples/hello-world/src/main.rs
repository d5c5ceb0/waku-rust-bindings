use tokio::signal;
use std::str::FromStr;

use waku::{
    waku_new, Encoding, Event, Initialized, LibwakuResponse, Multiaddr, Running, WakuContentTopic,
    WakuMessage, WakuNodeConfig, WakuNodeContext, WakuNodeHandle,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let topic = "/waku/2/rs/16/32";
    // Create a Waku instance
    let waku = waku_new(Some(WakuNodeConfig {
        port: Some(60010),
        cluster_id: Some(16),
        shards: vec![1, 32, 64, 128, 256],
        // node_key: Some(SecretKey::from_str("2fc0515879e52b7b73297cfd6ab3abf7c344ef84b7a90ff6f4cc19e05a198027").unwrap()),
        max_message_size: Some("1024KiB".to_string()),
        relay_topics: vec![topic.to_string()],
        log_level: Some("DEBUG"), // Supported: TRACE, DEBUG, INFO, NOTICE, WARN, ERROR or FATAL

        keep_alive: Some(true),

        // Discovery
        dns_discovery: Some(true),
        dns_discovery_url: Some("enrtree://AMOJVZX4V6EXP7NTJPMAYJYST2QP6AJXYW76IU6VGJS7UVSNDYZG4@boot.prod.status.nodes.status.im"),
        discv5_discovery: Some(true),
        discv5_udp_port: Some(9000),
        discv5_enr_auto_update: Some(false),

        ..Default::default()
    }))
    .expect("should instantiate");
    
    if let Ok(_) = waku.start() {
        println!("It works!");
    }
    else {
        println!("It didn't work. Let's go home xD");
    }

    let my_closure = move |response| {
        if let LibwakuResponse::Success(v) = response {
            let event: Event =
                serde_json::from_str(v.unwrap().as_str()).expect("Parsing event to succeed");

            match event {
                Event::WakuMessage(evt) => {
                    println!("WakuMessage event received: {:?}", evt.waku_message);
                }
                Event::Unrecognized(err) => panic!("Unrecognized waku event: {:?}", err),
                _ => panic!("event case not expected"),
            };
        }
    };

    // Establish a closure that handles the incoming messages
    waku.ctx.waku_set_event_callback(my_closure);
    waku.relay_subscribe(&topic.to_string()).expect("waku should subscribe");
    
    // Wait for Ctrl+C (SIGINT) signal
    signal::ctrl_c().await.expect("Failed to listen for Ctrl+C signal");

    println!("Ctrl+C pressed, shutting down.");

    waku.stop().expect("Failed to stop Waku");

    signal::ctrl_c().await?;
    println!("ctrl-c received!");
    Ok(())
}

