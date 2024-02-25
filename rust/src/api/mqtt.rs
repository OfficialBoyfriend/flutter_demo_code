use std::{thread::sleep, time::Duration};
use flutter_rust_bridge::frb;
pub use std::sync::atomic::AtomicBool;

use crate::frb_generated::StreamSink;

pub struct Mqtt {
    pub client_id: String,

    pub test: AtomicBool,
}

impl Mqtt {
    #[frb(sync)]
    pub fn new(client_id: String) -> Mqtt {
        Mqtt {
            client_id,
            test: AtomicBool::new(false),
        }
    }

    pub fn send(&self, _payload: String) -> Result<(), String> {
        Ok(())
    }

    pub fn connect(&self, host: String, sink: StreamSink<String>) {
        for _ in 0..10 {
            sleep(Duration::from_secs(7));
            let _ = sink.add(format!("Connected to {}", host));
        }
    }
}
