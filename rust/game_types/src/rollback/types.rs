//! The types of `rollback`.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, Debug)]
pub struct Typed {
    pub bytes_per_element: f64,
    pub buffer: crate::rollback::ArrayBufferLike,
    pub byte_length: f64,
    pub byte_offset: f64,
    pub length: f64,
    pub ___to_string_tag_599: crate::rollback::TypedToStringTag599,
}
impl Default for Typed {
    fn default() -> Self {
        Typed {
            bytes_per_element: 0.0,
            buffer: crate::rollback::ArrayBufferLike::ArrayBuffer(crate::Opaque),
            byte_length: 0.0,
            byte_offset: 0.0,
            length: 0.0,
            ___to_string_tag_599: crate::rollback::TypedToStringTag599::Float32Array,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum ArrayBufferLike {
    ArrayBuffer(crate::Opaque),
    SharedArrayBuffer(crate::Opaque),
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum TypedToStringTag599 {
    Float32Array,
}
impl TypedToStringTag599 {
    pub const fn as_str(self) -> &'static str {
        match self {
            TypedToStringTag599::Float32Array => "Float32Array",
        }
    }
}

#[derive(Clone, Debug)]
pub enum Table {
    SideMap(JsMap<crate::ObjKey, crate::Opaque>),
    SideSet(JsSet<crate::ObjKey>),
}

#[derive(Clone, Debug, Default)]
pub struct Chain {
    pub index: JsMap<crate::ObjKey, f64>,
    pub kinds: Vec<f64>,
    pub views: Vec<Option<Vec<u32>>>,
    pub base: crate::rollback::ChainBase,
    pub stamp: Vec<i32>,
    pub met: Vec<u8>,
    pub fresh: Vec<u8>,
    pub epoch: f64,
    pub last: Vec<crate::Opaque>,
    pub last_shape: Vec<Option<crate::rollback::Shape>>,
    pub stack: Vec<f64>,
    pub busy: bool,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct ChainBase {
    pub n: f64,
}

#[derive(Clone, Debug, Default)]
pub struct Shape {
    pub keys: Vec<String>,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct RollbackMod {
    /// (src/game/rollback.ts:37 PROTO_KIND)
    pub proto_kind: JsMap<crate::ObjKey, f64>,
    /// (src/game/rollback.ts:255 MAKES)
    pub makes: JsMap<String, crate::rollback::Shape>,
    /// (src/game/rollback.ts:256 written)
    pub written: bool,
    /// (src/game/rollback.ts:354 BROUGHT)
    pub brought: JsSet<crate::ObjKey>,
}
impl RollbackMod {
    pub fn blank() -> Self {
        RollbackMod {
            proto_kind: JsMap::new(),
            makes: JsMap::new(),
            written: true,
            brought: JsSet::new(),
        }
    }
}

/// (src/game/rollback.ts:36 K_ATOM)
pub const K_ATOM: f64 = 0.0;

/// (src/game/rollback.ts:36 K_ARR)
pub const K_ARR: f64 = 1.0;

/// (src/game/rollback.ts:36 K_TYPED)
pub const K_TYPED: f64 = 2.0;

/// (src/game/rollback.ts:36 K_OBJ)
pub const K_OBJ: f64 = 3.0;

/// (src/game/rollback.ts:36 K_MAP)
pub const K_MAP: f64 = 4.0;

/// (src/game/rollback.ts:36 K_SET)
pub const K_SET: f64 = 5.0;

/// (src/game/rollback.ts:36 K_BUF)
pub const K_BUF: f64 = 6.0;

/// (src/game/rollback.ts:36 K_SIDEMAP)
pub const K_SIDEMAP: f64 = 7.0;

/// (src/game/rollback.ts:36 K_SIDESET)
pub const K_SIDESET: f64 = 8.0;

/// (src/game/rollback.ts:36 K_RAWWEAK)
pub const K_RAWWEAK: f64 = 9.0;

/// (src/game/rollback.ts:36 K_WORDS)
pub const K_WORDS: f64 = 10.0;

/// (src/game/rollback.ts:36 K_SIM)
pub const K_SIM: f64 = 11.0;

/// (src/game/rollback.ts:55 HOST_FIELDS: a table nothing writes)
pub fn host_fields() -> [String; 4] { [String::from("engine"), String::from("audio"), String::from("smoke"), String::from("journal")] }

/// (src/game/rollback.ts:56 STATIC_FIELDS: a table nothing writes)
pub fn static_fields() -> [String; 1] { [String::from("ch")] }

/// (src/game/rollback.ts:95 WORDS_FROM)
pub const WORDS_FROM: f64 = 256.0;

/// (src/game/rollback.ts:98 WALK_AGAIN)
pub const WALK_AGAIN: f64 = 1.25;

/// (src/game/rollback.ts:98 WALK_SLACK)
pub const WALK_SLACK: f64 = 2048.0;

/// (src/game/rollback.ts:98 CHAIN_GROWS)
pub const CHAIN_GROWS: f64 = 8.0;

/// (src/game/rollback.ts:98 CHAIN_SLACK)
pub const CHAIN_SLACK: f64 = 65536.0;

/// (src/game/rollback.ts:100 YOUNG)
pub const YOUNG: f64 = 3.0;
