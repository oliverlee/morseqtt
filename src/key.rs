use crate::timing::Signal;
use indicatif::{ProgressBar, ProgressStyle};
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
    progress: Option<ProgressBar>,
}

impl MqttKey {
    pub fn new(client: MqttClient, topic: String, on_payload: String, off_payload: String) -> Self {
        Self {
            client,
            topic,
            on_payload,
            off_payload,
            progress: None,
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
    progress_bar: Option<ProgressBar>,
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

    if groups.is_empty() {
        future::Either::A(future::ok(()))
    } else {
        if let Some(pb) = progress_bar {
            key.lock().unwrap().deref_mut().progress.replace(pb);
        };

        future::Either::B(
            stream::iter_ok(groups.into_iter())
                .for_each(move |(k, signal, count)| {
                    {
                        let mut guard = k.lock().unwrap();
                        let key = guard.deref_mut();

                        if signal == Signal::On {
                            key.send_on();
                        } else {
                            key.send_off();
                        }
                    }

                    Delay::new(Instant::now() + count * dur).and_then(move |_| {
                        let mut guard = k.lock().unwrap();

                        let progress = guard.deref_mut().progress.as_ref();
                        if let Some(pb) = progress {
                            pb.inc(count.into());
                        }

                        future::ok(())
                    })
                })
                .and_then(move |_| {
                    let mut guard = key.lock().unwrap();

                    let key = guard.deref_mut();
                    key.send_off();

                    if let Some(pb) = key.progress.take() {
                        pb.set_style(ProgressStyle::default_bar().template("{msg}"));
                        pb.finish();
                    }

                    future::ok(())
                })
                .map_err(|_| ()),
        )
    }
}
