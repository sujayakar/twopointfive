//! The types of `netquietfeed`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: NetFeed, src/game/netquietfeed.ts:12)
#[derive(Clone, Debug)]
pub struct NetFeed {
    pub what: crate::netquietfeed::NetFeedWhat,
    pub by: Option<GuardId>,
    pub who: Option<GuardId>,
    pub men: Vec<GuardId>,
}
impl NetFeed {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        NetFeed {
            what: crate::netquietfeed::NetFeedWhat::Body,
            by: None,
            who: None,
            men: Vec::new(),
        }
    }
}
impl Default for NetFeed { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum NetFeedWhat {
    Body,
    Asked,
}
impl NetFeedWhat {
    pub const fn as_str(self) -> &'static str {
        match self {
            NetFeedWhat::Body => "body",
            NetFeedWhat::Asked => "asked",
        }
    }
}

/// (TypeScript: inline at src/game/netquietfeed.ts:10 NETFEED, src/game/netquietfeed.ts:10)
#[derive(Clone, Copy, Debug)]
pub struct NetquietfeedModNETFEED {
    pub live: bool,
}
impl NetquietfeedModNETFEED {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        NetquietfeedModNETFEED {
            live: false,
        }
    }
}
impl Default for NetquietfeedModNETFEED { fn default() -> Self { Self::zeroed() } }

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct NetquietfeedMod {
    /// (src/game/netquietfeed.ts:10 NETFEED)
    pub netfeed: crate::netquietfeed::NetquietfeedModNETFEED,
    /// (src/game/netquietfeed.ts:13 filed)
    pub filed: Option<Vec<crate::netquietfeed::NetFeed>>,
    /// (src/game/netquietfeed.ts:25 holding)
    pub holding: JsSet<GuardId>,
}
impl NetquietfeedMod {
    pub fn blank() -> Self {
        NetquietfeedMod {
            netfeed: crate::netquietfeed::NetquietfeedModNETFEED { live: false },
            filed: None,
            holding: JsSet::new(),
        }
    }
}
