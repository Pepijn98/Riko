use std::{sync::Arc};

use serenity::{
    client::bridge::gateway::ShardManager,
    prelude::{Mutex, TypeMapKey},
};

use time::{self, Tm};
use num_integer::Integer;
use typemap::Key;

// == Uptimer == //
pub struct UptimerKey;

impl Key for UptimerKey {
    type Value = Uptimer;
}

pub struct Uptimer {
    started_at: Tm,
}

impl Uptimer {
    pub fn new() -> Uptimer {
        Uptimer {
            started_at: time::now_utc(),
        }
    }
    pub fn uptime_string(&self) -> String {
        let seconds = (time::now_utc() - self.started_at).num_seconds();
        let (minutes, seconds) = seconds.div_rem(&60);
        let (hours, minutes) = minutes.div_rem(&60);
        let (days, hours) = hours.div_rem(&24);
        format!("{}d {}h {}m {}s", days, hours, minutes, seconds)
    }
}

// == ShardManager == //
pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}