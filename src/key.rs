use crate::timing::Signal;
use itertools::Itertools;
use rumqtt::{MqttClient, QoS};
use std::convert::TryFrom;
use std::ops::DerefMut;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tokio::prelude::*;
use tokio::timer::Delay;

#[allow(clippy::module_name_repetitions)]
pub struct MqttKey {
    client: MqttClient,
    topic: String,
    on_payload: String,
    off_payload: String,
}

impl MqttKey {
    pub fn new(client: MqttClient, topic: String, on_payload: String, off_payload: String) -> Self {
        Self {
            client,
            topic,
            on_payload,
            off_payload,
        }
    }

    fn send_on(&mut self) {
        self.client
            .publish(
                self.topic.as_str(),
                QoS::AtLeastOnce,
                false,
                self.on_payload.as_str(),
            )
            .unwrap();
    }

    fn send_off(&mut self) {
        self.client
            .publish(
                self.topic.as_str(),
                QoS::AtLeastOnce,
                false,
                self.off_payload.as_str(),
            )
            .unwrap();
    }
}

pub fn transmit_with_dur(
    key: Arc<Mutex<MqttKey>>,
    timing: impl Iterator<Item = Signal>,
    dur: Duration,
) -> impl Future<Item = (), Error = ()> {
    // We need to force evaluation since group_by() is lazy
    let groups: Vec<_> = timing
        .group_by(|x| *x)
        .into_iter()
        .map(|(signal, group)| {
            (
                Arc::clone(&key),
                signal,
                u32::try_from(group.count()).unwrap(),
            )
        })
        .collect();

    stream::iter_ok(groups.into_iter())
        .for_each(move |(k, signal, count)| {
            if signal == Signal::On {
                k.lock().unwrap().deref_mut().send_on();
            } else {
                k.lock().unwrap().deref_mut().send_off();
            }

            Delay::new(Instant::now() + count * dur)
        })
        .and_then(move |_| {
            key.lock().unwrap().deref_mut().send_off();
            future::ok(())
        })
        .map_err(|_| ())
}
