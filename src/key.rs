use crate::timing::Signal;
use itertools::Itertools;
use std::convert::TryFrom;
use std::ops::DerefMut;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tokio::prelude::*;
use tokio::timer::Delay;

pub struct Key<A, B>
where
    A: FnMut() -> (),
    B: FnMut() -> (),
{
    pub on: A,
    pub off: B,
}

pub fn transmit_with_dur<A: FnMut() -> (), B: FnMut() -> ()>(
    key: Arc<Mutex<Key<A, B>>>,
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
                (k.lock().unwrap().deref_mut().on)();
            } else {
                (k.lock().unwrap().deref_mut().off)();
            }

            Delay::new(Instant::now() + count * dur)
        })
        .and_then(move |_| {
            (key.lock().unwrap().deref_mut().off)();
            future::ok(())
        })
        .map_err(|_| ())
}
