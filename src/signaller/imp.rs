use eyre::Result;
use gst_webrtc::gst_sdp;
use log::{debug, error};
use tokio::task;

use gst::glib::prelude::*;
use gst::glib::{self, WeakRef};
use gst::subclass::prelude::*;
use gst::{gst_debug, gst_error, gst_info};

use once_cell::sync::{Lazy, OnceCell};

use webrtcsink::webrtcsink::WebRTCSink;

static CAT: Lazy<gst::DebugCategory> = Lazy::new(|| {
    gst::DebugCategory::new(
        "webrtcsink-signaller",
        gst::DebugColorFlags::empty(),
        Some("WebRTC sink signaller"),
    )
});

static ELEMENT: OnceCell<WeakRef<WebRTCSink>> = OnceCell::new();

pub fn element() -> Result<WebRTCSink> {
    let element = ELEMENT
        .get()
        .expect("element not init")
        .clone()
        .upgrade()
        .ok_or_else(|| eyre::eyre!("webrtcsink element instance not found"))?;

    Ok(element)
}

#[derive(Default)]
pub struct Signaller {}

impl Signaller {
    /// When Signaller is started it inits a SignallerActor and lets the ProjectManager root actor
    /// aware of it
    pub fn start(&self, element: &WebRTCSink) {
        debug!("signaller imp starting now");
        gst_info!(CAT, obj: element, "Starting now");

        let element = element.downgrade();

        ELEMENT.set(element).expect("element instance already set");

        task::spawn(async move {
            println!("start in tokio task");
        });
    }

    /// Sends the server local SDP  to the client over WAMP
    pub fn handle_sdp(
        &self,
        element: &WebRTCSink,
        _peer_id: &str,
        sdp: &gst_webrtc::WebRTCSessionDescription,
    ) {
        let sdp = sdp.sdp();
        let element = element.downgrade();

        task::spawn(async move { println!("in tokio handle sdp") });
    }

    /// Sends the servers ICE candidates to the client over WAMP
    pub fn handle_ice(
        &self,
        element: &WebRTCSink,
        peer_id: &str,
        candidate: &str,
        sdp_mline_index: Option<u32>,
        _sdp_mid: Option<String>,
    ) {
        let peer_id = peer_id.to_string();
        let element = element.downgrade();
        let candidate = candidate.to_string();

        task::spawn(async move { println!("in tokio handle_ice") });
    }

    pub fn stop(&self, element: &WebRTCSink) {
        gst_info!(CAT, obj: element, "Stopping now");
        //TODO: what should we do here?
    }

    pub fn consumer_removed(&self, element: &WebRTCSink, peer_id: &str) {
        gst_debug!(CAT, obj: element, "Signalling consumer {} removed", peer_id);
        //TODO: what should we do here?
    }
}

#[glib::object_subclass]
impl ObjectSubclass for Signaller {
    const NAME: &'static str = "WampWebRTCSinkSignaller";
    type Type = super::Signaller;
    type ParentType = glib::Object;
}

impl ObjectImpl for Signaller {
    fn properties() -> &'static [glib::ParamSpec] {
        //TODO: what properties do we add here? The WAMP server URL?
        static PROPERTIES: Lazy<Vec<glib::ParamSpec>> = Lazy::new(|| {
            vec![glib::ParamSpecString::new(
                "address",
                "Address",
                "Address of the signalling server",
                Some("ws://127.0.0.1:8443"),
                glib::ParamFlags::READWRITE,
            )]
        });

        PROPERTIES.as_ref()
    }

    fn set_property(
        &self,
        _obj: &Self::Type,
        _id: usize,
        _value: &glib::Value,
        pspec: &glib::ParamSpec,
    ) {
        // TODO: what is the purpose of this function? Do we need to do anything here?
        match pspec.name() {
            "address" => {
                // let address: Option<_> = value.get().expect("type checked upstream");

                // if let Some(address) = address {
                //     gst_info!(CAT, "Signaller address set to {}", address);

                //     let mut settings = self.settings.lock().unwrap();
                //     settings.address = Some(address);
                // } else {
                //     gst_error!(CAT, "address can't be None");
                // }
                unimplemented!()
            }
            _ => unimplemented!(),
        }
    }

    fn property(&self, _obj: &Self::Type, _id: usize, pspec: &glib::ParamSpec) -> glib::Value {
        // TODO: what is the purpose of this function? Do we need to do anything here?

        match pspec.name() {
            "address" => {
                // let settings = self.settings.lock().unwrap();
                // settings.address.to_value()
                unimplemented!()
            }
            _ => unimplemented!(),
        }
    }
}
