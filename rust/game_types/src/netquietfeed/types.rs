//! The types of `netquietfeed`.
#![allow(unused_imports)]
use crate::{Blank, bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Debug, ToBytes, Default)]
pub struct NetFeed {
    pub what: crate::netquietfeed::NetFeedWhat,
    pub by: Option<GuardId>,
    pub who: Option<GuardId>,
    pub men: Vec<GuardId>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum NetFeedWhat {
    #[default] Body,
    Asked,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct NetquietfeedModNETFEED {
    pub live: bool,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes, Blank)]
pub struct NetquietfeedMod {
    #[is(crate::netquietfeed::NetquietfeedModNETFEED { live: false })] pub netfeed: crate::netquietfeed::NetquietfeedModNETFEED,
    pub filed: Option<Vec<crate::netquietfeed::NetFeed>>,
}
