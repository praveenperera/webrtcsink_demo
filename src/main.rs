use gst::prelude::*;
use webrtcsink::webrtcsink::WebRTCSink;
mod signaller;

use eyre::{Result, WrapErr};

use std::time::Duration;
use tokio::net::TcpListener;

async fn start_heartbeats() {
    let mut heartbeat_number = 0;

    tokio::task::spawn(async move {
        loop {
            tokio::time::sleep(Duration::from_millis(3000)).await;
            heartbeat_number += 1;
            println!("heartbeat called: {}", heartbeat_number)
        }
    });
}

async fn start_pipeline() {
    tokio::task::spawn(async {
        let custom_signaller = signaller::Signaller::new();
        dbg!("signaller created");

        let webrtcsink = WebRTCSink::with_signaller(Box::new(custom_signaller));
        dbg!("webrtcsink created");

        let pipeline = gst::Pipeline::new(None);
        dbg!("pipline started");

        let video_src_element = gst::ElementFactory::make("videotestsrc", None).unwrap();
        dbg!("video src element init");

        pipeline
            .add_many(&[&video_src_element, webrtcsink.upcast_ref()])
            .unwrap();
        dbg!("video element added to pipeline");

        video_src_element
            .link(webrtcsink.upcast_ref::<gst::Element>())
            .unwrap();
        dbg!("video src element linked");

        let bus = pipeline.bus().unwrap();
        dbg!("pipeline bus");

        pipeline.set_state(gst::State::Playing).unwrap();
        dbg!("pipeline set state");

        let _msg = bus.timed_pop_filtered(gst::ClockTime::NONE, &[gst::MessageType::Eos]);
        dbg!("message");
    });
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    env_logger::init();
    gst::init().wrap_err("gst init failed")?;

    start_heartbeats().await;
    start_pipeline().await;

    let listener = TcpListener::bind("127.0.0.1:8081").await.unwrap();

    let _ = listener.accept().await;
    signaller::imp::element()?.add_consumer("1")?;

    loop {
        tokio::time::sleep(Duration::from_secs(60)).await;
    }
}
