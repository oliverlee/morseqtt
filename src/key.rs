use crate::timing::Signal;
use itertools::Itertools;
use std::time::{Duration, Instant};
use tokio::prelude::*;
use tokio::timer::Delay;

use std::convert::TryFrom;
use std::sync::Arc;

pub struct Key<A, B>
where
    A: Fn() -> (),
    B: Fn() -> (),
{
    pub on: A,
    pub off: B,
}

pub fn transmit_with_dur<A: Fn() -> (), B: Fn() -> ()>(
    key: Arc<Key<A, B>>,
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
                (k.on)();
            } else {
                (k.off)();
            }

            Delay::new(Instant::now() + count * dur)
        })
        .and_then(move |_| {
            (key.off)();
            future::ok(())
        })
        .map_err(|_| ())
}
