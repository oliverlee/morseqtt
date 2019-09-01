use crate::timing::Signal;
use itertools::Itertools;
use std::time::{Duration, Instant};
use tokio::prelude::*;
use tokio::timer::Delay;

use std::convert::TryFrom;

pub struct Key<A, B>
where
    A: Fn() -> (),
    B: Fn() -> (),
{
    pub on: A,
    pub off: B,
}

impl<A, B> Key<A, B>
where
    A: Fn() -> (),
    B: Fn() -> (),
{
    pub fn transmit_with_dur(
        &self,
        timing: impl Iterator<Item = Signal>,
        dur: Duration,
    ) -> impl Future<Item = (), Error = ()> {
        // We need to force evaluation since group_by() is lazy
        let groups: Vec<_> = timing
            .group_by(|x| *x)
            .into_iter()
            .map(|(key, group)| (key, u32::try_from(group.count()).unwrap()))
            .collect();

        stream::iter_ok(groups.into_iter())
            .for_each(move |(key, count)| {
                Delay::new(Instant::now() + count * dur).and_then(move |_| {
                    print!("{}", key.to_string().repeat(count as usize));
                    std::io::stdout().flush().unwrap();

                    future::ok(())
                })
            })
            .and_then(|_| {
                println!();
                future::ok(())
            })
            .map_err(|_| ())
    }

    pub fn transmit(
        &self,
        timing: impl Iterator<Item = Signal>,
    ) -> impl Future<Item = (), Error = ()> {
        self.transmit_with_dur(timing, Duration::from_millis(50))
    }
}
