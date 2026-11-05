//! The types of `dialogue`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: DialogueSnap, src/game/dialogue.ts:2064)
#[derive(Clone, Debug)]
pub struct DialogueSnap {
    pub recent: Vec<(String, f64)>,
    pub sam: crate::dialogue::DialogueSnapSam,
    pub radio: Option<crate::dialogue::DialogueSnapRadio>,
    pub ambient: crate::dialogue::DialogueSnapAmbient,
}
impl DialogueSnap {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DialogueSnap {
            recent: Vec::new(),
            sam: crate::dialogue::DialogueSnapSam::zeroed(),
            radio: None,
            ambient: crate::dialogue::DialogueSnapAmbient::zeroed(),
        }
    }
}
impl Default for DialogueSnap { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/dialogue.ts:2065 DialogueSnap.sam, src/game/dialogue.ts:2065)
#[derive(Clone, Debug)]
pub struct DialogueSnapSam {
    pub said: Vec<(String, f64)>,
    pub last: Option<String>,
}
impl DialogueSnapSam {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DialogueSnapSam {
            said: Vec::new(),
            last: None,
        }
    }
}
impl Default for DialogueSnapSam { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/dialogue.ts:2066 DialogueSnap.radio, src/game/dialogue.ts:2066)
#[derive(Clone, Debug)]
pub struct DialogueSnapRadio {
    pub done: Vec<String>,
    pub queue: Vec<crate::dialogue::DialogueSnapRadioQueueItem>,
    pub reply: Option<crate::dialogue::DialogueSnapRadioReply>,
    pub last_at: f64,
    pub esc: f64,
    pub blackout: bool,
    pub found: f64,
}
impl DialogueSnapRadio {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DialogueSnapRadio {
            done: Vec::new(),
            queue: Vec::new(),
            reply: None,
            last_at: 0.0,
            esc: 0.0,
            blackout: false,
            found: 0.0,
        }
    }
}
impl Default for DialogueSnapRadio { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/dialogue.ts:2066 DialogueSnapRadio.queue, src/game/dialogue.ts:2066)
#[derive(Clone, Debug)]
pub struct DialogueSnapRadioQueueItem {
    pub at: f64,
    pub beat: String,
}
impl DialogueSnapRadioQueueItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DialogueSnapRadioQueueItem {
            at: 0.0,
            beat: String::new(),
        }
    }
}
impl Default for DialogueSnapRadioQueueItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/dialogue.ts:2066 DialogueSnapRadio.reply, src/game/dialogue.ts:2066)
#[derive(Clone, Debug)]
pub struct DialogueSnapRadioReply {
    pub at: f64,
    pub text: String,
}
impl DialogueSnapRadioReply {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DialogueSnapRadioReply {
            at: 0.0,
            text: String::new(),
        }
    }
}
impl Default for DialogueSnapRadioReply { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/dialogue.ts:2067 DialogueSnap.ambient, src/game/dialogue.ts:2067)
#[derive(Clone, Debug)]
pub struct DialogueSnapAmbient {
    pub next: f64,
    pub played: Vec<(String, f64)>,
    pub count: f64,
}
impl DialogueSnapAmbient {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DialogueSnapAmbient {
            next: 0.0,
            played: Vec::new(),
            count: 0.0,
        }
    }
}
impl Default for DialogueSnapAmbient { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Knowledge {
    None,
    Heard,
    Glimpsed,
    Saw,
    SawHurt,
}
impl Knowledge {
    pub const fn as_str(self) -> &'static str {
        match self {
            Knowledge::None => "none",
            Knowledge::Heard => "heard",
            Knowledge::Glimpsed => "glimpsed",
            Knowledge::Saw => "saw",
            Knowledge::SawHurt => "sawHurt",
        }
    }
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Band {
    Low,
    Mid,
    High,
}
impl Band {
    pub const fn as_str(self) -> &'static str {
        match self {
            Band::Low => "low",
            Band::Mid => "mid",
            Band::High => "high",
        }
    }
}

/// (TypeScript: a union)
#[derive(Clone, Copy, Debug)]
pub enum StageKey {
    Num(f64),
    X,
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Need {
    Shift,
    RollcallLive,
    Blackout,
    BodyFound,
    FloorUp,
    Lockdown,
    Episode,
    HeardSmall,
    HeardShots,
    HeardBang,
    BodyCalled,
    BodyKnown,
    MissingKnown,
    HeldLong,
    Dazzled,
    SawShot,
    SawDrop,
    AlertNow,
    DrewOnYou,
    PartnerKnown,
    Office,
}
impl Need {
    pub const fn as_str(self) -> &'static str {
        match self {
            Need::Shift => "shift",
            Need::RollcallLive => "rollcallLive",
            Need::Blackout => "blackout",
            Need::BodyFound => "bodyFound",
            Need::FloorUp => "floorUp",
            Need::Lockdown => "lockdown",
            Need::Episode => "episode",
            Need::HeardSmall => "heardSmall",
            Need::HeardShots => "heardShots",
            Need::HeardBang => "heardBang",
            Need::BodyCalled => "bodyCalled",
            Need::BodyKnown => "bodyKnown",
            Need::MissingKnown => "missingKnown",
            Need::HeldLong => "heldLong",
            Need::Dazzled => "dazzled",
            Need::SawShot => "sawShot",
            Need::SawDrop => "sawDrop",
            Need::AlertNow => "alertNow",
            Need::DrewOnYou => "drewOnYou",
            Need::PartnerKnown => "partnerKnown",
            Need::Office => "office",
        }
    }
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Token {
    Door,
    Code,
    Missing,
    Body,
    N,
    Up,
    Post,
    Room,
    Route,
    Partner,
    Victim,
    SawWhere,
    HeardWhere,
    PartnerRoute,
    Caller,
    Checker,
    Checked,
    ClearRoom,
    BreakerRoom,
    CheckEta,
    Minutes,
    Badge,
    PadDoor,
}
impl Token {
    pub const fn as_str(self) -> &'static str {
        match self {
            Token::Door => "door",
            Token::Code => "code",
            Token::Missing => "missing",
            Token::Body => "body",
            Token::N => "n",
            Token::Up => "up",
            Token::Post => "post",
            Token::Room => "room",
            Token::Route => "route",
            Token::Partner => "partner",
            Token::Victim => "victim",
            Token::SawWhere => "sawWhere",
            Token::HeardWhere => "heardWhere",
            Token::PartnerRoute => "partnerRoute",
            Token::Caller => "caller",
            Token::Checker => "checker",
            Token::Checked => "checked",
            Token::ClearRoom => "clearRoom",
            Token::BreakerRoom => "breakerRoom",
            Token::CheckEta => "checkEta",
            Token::Minutes => "minutes",
            Token::Badge => "badge",
            Token::PadDoor => "padDoor",
        }
    }
}

/// (TypeScript: Line, src/game/dialogue.ts:195)
#[derive(Clone, Debug)]
pub struct Line {
    pub id: String,
    pub k: crate::dialogue::LineK,
    pub a: Option<crate::dialogue::LineA>,
    pub t: Option<crate::game::Temperament>,
    pub s: crate::dialogue::StageKey,
    pub text: String,
    pub needs: Option<Vec<crate::dialogue::Need>>,
    pub w: Option<f64>,
    pub only: Option<crate::dialogue::LineOnly>,
    pub now: Option<String>,
    pub away: Option<String>,
    pub guide: Option<String>,
    pub bare: Option<String>,
}
impl Line {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Line {
            id: String::new(),
            k: crate::dialogue::LineK::None,
            a: None,
            t: None,
            s: crate::dialogue::StageKey::Num(0.0),
            text: String::new(),
            needs: None,
            w: None,
            only: None,
            now: None,
            away: None,
            guide: None,
            bare: None,
        }
    }
}
impl Default for Line { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum LineK {
    None,
    Heard,
    Glimpsed,
    Saw,
    SawHurt,
    Empty,
}
impl LineK {
    pub const fn as_str(self) -> &'static str {
        match self {
            LineK::None => "none",
            LineK::Heard => "heard",
            LineK::Glimpsed => "glimpsed",
            LineK::Saw => "saw",
            LineK::SawHurt => "sawHurt",
            LineK::Empty => "*",
        }
    }
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum LineA {
    Low,
    Mid,
    High,
    Mid2,
}
impl LineA {
    pub const fn as_str(self) -> &'static str {
        match self {
            LineA::Low => "low",
            LineA::Mid => "mid",
            LineA::High => "high",
            LineA::Mid2 => "mid+",
        }
    }
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum LineOnly {
    Shift,
    Office,
}
impl LineOnly {
    pub const fn as_str(self) -> &'static str {
        match self {
            LineOnly::Shift => "shift",
            LineOnly::Office => "office",
        }
    }
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum IntelKind {
    CheckOn,
    Clearing,
    RollcallLive,
    Blackout,
    Door,
    Code,
    Rollcall,
    LockdownPosts,
    Routes,
    Missing,
    Drive,
}
impl IntelKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            IntelKind::CheckOn => "checkOn",
            IntelKind::Clearing => "clearing",
            IntelKind::RollcallLive => "rollcallLive",
            IntelKind::Blackout => "blackout",
            IntelKind::Door => "door",
            IntelKind::Code => "code",
            IntelKind::Rollcall => "rollcall",
            IntelKind::LockdownPosts => "lockdownPosts",
            IntelKind::Routes => "routes",
            IntelKind::Missing => "missing",
            IntelKind::Drive => "drive",
        }
    }
}

/// (TypeScript: IntelItem, src/game/dialogue.ts:311)
#[derive(Clone, Debug)]
pub struct IntelItem {
    pub kind: crate::dialogue::IntelKind,
    pub text: String,
    pub known: bool,
    pub live: bool,
}
impl IntelItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        IntelItem {
            kind: crate::dialogue::IntelKind::CheckOn,
            text: String::new(),
            known: false,
            live: false,
        }
    }
}
impl Default for IntelItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum ShapeKey {
    Pedant,
    Selfown,
    Folder,
    Ramble,
    Fan,
    Stoic,
    Con,
    Game,
    Menace,
}
impl ShapeKey {
    pub const fn as_str(self) -> &'static str {
        match self {
            ShapeKey::Pedant => "pedant",
            ShapeKey::Selfown => "selfown",
            ShapeKey::Folder => "folder",
            ShapeKey::Ramble => "ramble",
            ShapeKey::Fan => "fan",
            ShapeKey::Stoic => "stoic",
            ShapeKey::Con => "con",
            ShapeKey::Game => "game",
            ShapeKey::Menace => "menace",
        }
    }
}

/// (TypeScript: a union)
#[derive(Clone, Debug)]
pub enum IntelKey {
    Steady,
    Nervy,
    Hard,
    Empty,
    Pedant,
    Selfown,
    Folder,
    Ramble,
    Fan,
    Stoic,
    Con,
    Game,
    Menace,
    SteadyNone,
    SteadyHeard,
    SteadyGlimpsed,
    SteadySaw,
    SteadySawHurt,
    NervyNone,
    NervyHeard,
    NervyGlimpsed,
    NervySaw,
    NervySawHurt,
    HardNone,
    HardHeard,
    HardGlimpsed,
    HardSaw,
    HardSawHurt,
    None,
    Heard,
    Glimpsed,
    Saw,
    SawHurt,
    PedantSteady,
    PedantNervy,
    PedantHard,
    SelfownSteady,
    SelfownNervy,
    SelfownHard,
    FolderSteady,
    FolderNervy,
    FolderHard,
    RambleSteady,
    RambleNervy,
    RambleHard,
    FanSteady,
    FanNervy,
    FanHard,
    StoicSteady,
    StoicNervy,
    StoicHard,
    ConSteady,
    ConNervy,
    ConHard,
    GameSteady,
    GameNervy,
    GameHard,
    MenaceSteady,
    MenaceNervy,
    MenaceHard,
    Text(String),
}

/// (TypeScript: Rect, src/game/dialogue.ts:373)
#[derive(Clone, Debug)]
pub struct Rect {
    pub name: String,
    pub x0: f64,
    pub x1: f64,
    pub z0: f64,
    pub z1: f64,
}
impl Rect {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Rect {
            name: String::new(),
            x0: 0.0,
            x1: 0.0,
            z0: 0.0,
            z1: 0.0,
        }
    }
}
impl Default for Rect { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: PlaceRect, src/game/dialogue.ts:383)
#[derive(Clone, Debug)]
pub struct PlaceRect {
    pub id: Option<String>,
    pub name: String,
    pub x0: f64,
    pub x1: f64,
    pub z0: f64,
    pub z1: f64,
}
impl PlaceRect {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        PlaceRect {
            id: None,
            name: String::new(),
            x0: 0.0,
            x1: 0.0,
            z0: 0.0,
            z1: 0.0,
        }
    }
}
impl Default for PlaceRect { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: DialogueCtx, src/game/dialogue.ts:520)
#[derive(Clone, Debug)]
pub struct DialogueCtx {
    pub who: String,
    pub k: crate::dialogue::Knowledge,
    pub a: f64,
    pub band: crate::dialogue::Band,
    pub t: crate::game::Temperament,
    pub why: Vec<String>,
    pub facts: crate::dialogue::DialogueCtxFacts,
    pub tokens: crate::dialogue::DialogueCtxTokens,
    pub own_rooms: Option<bool>,
    pub nameless: Option<bool>,
    pub intel: Vec<crate::dialogue::IntelItem>,
    pub time: f64,
    pub recent: Option<JsMap<String, f64>>,
}
impl DialogueCtx {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DialogueCtx {
            who: String::new(),
            k: crate::dialogue::Knowledge::None,
            a: 0.0,
            band: crate::dialogue::Band::Low,
            t: crate::game::Temperament::Steady,
            why: Vec::new(),
            facts: crate::dialogue::DialogueCtxFacts::zeroed(),
            tokens: crate::dialogue::DialogueCtxTokens::zeroed(),
            own_rooms: None,
            nameless: None,
            intel: Vec::new(),
            time: 0.0,
            recent: None,
        }
    }
}
impl Default for DialogueCtx { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/dialogue.ts:523 DialogueCtx.facts, ../.npm-global/lib/node_modules/typescript/lib/lib.es5.d.ts:1583)
#[derive(Clone, Copy, Debug)]
pub struct DialogueCtxFacts {
    pub shift: Option<bool>,
    pub rollcall_live: Option<bool>,
    pub blackout: Option<bool>,
    pub body_found: Option<bool>,
    pub floor_up: Option<bool>,
    pub lockdown: Option<bool>,
    pub episode: Option<bool>,
    pub heard_small: Option<bool>,
    pub heard_shots: Option<bool>,
    pub heard_bang: Option<bool>,
    pub body_called: Option<bool>,
    pub body_known: Option<bool>,
    pub missing_known: Option<bool>,
    pub held_long: Option<bool>,
    pub dazzled: Option<bool>,
    pub saw_shot: Option<bool>,
    pub saw_drop: Option<bool>,
    pub alert_now: Option<bool>,
    pub drew_on_you: Option<bool>,
    pub partner_known: Option<bool>,
    pub office: Option<bool>,
}
impl DialogueCtxFacts {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DialogueCtxFacts {
            shift: None,
            rollcall_live: None,
            blackout: None,
            body_found: None,
            floor_up: None,
            lockdown: None,
            episode: None,
            heard_small: None,
            heard_shots: None,
            heard_bang: None,
            body_called: None,
            body_known: None,
            missing_known: None,
            held_long: None,
            dazzled: None,
            saw_shot: None,
            saw_drop: None,
            alert_now: None,
            drew_on_you: None,
            partner_known: None,
            office: None,
        }
    }
}
impl Default for DialogueCtxFacts { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/dialogue.ts:524 DialogueCtx.tokens, ../.npm-global/lib/node_modules/typescript/lib/lib.es5.d.ts:1583)
#[derive(Clone, Debug)]
pub struct DialogueCtxTokens {
    pub door: Option<String>,
    pub code: Option<String>,
    pub missing: Option<String>,
    pub body: Option<String>,
    pub n: Option<String>,
    pub up: Option<String>,
    pub post: Option<String>,
    pub room: Option<String>,
    pub route: Option<String>,
    pub partner: Option<String>,
    pub victim: Option<String>,
    pub saw_where: Option<String>,
    pub heard_where: Option<String>,
    pub partner_route: Option<String>,
    pub caller: Option<String>,
    pub checker: Option<String>,
    pub checked: Option<String>,
    pub clear_room: Option<String>,
    pub breaker_room: Option<String>,
    pub check_eta: Option<String>,
    pub minutes: Option<String>,
    pub badge: Option<String>,
    pub pad_door: Option<String>,
}
impl DialogueCtxTokens {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DialogueCtxTokens {
            door: None,
            code: None,
            missing: None,
            body: None,
            n: None,
            up: None,
            post: None,
            room: None,
            route: None,
            partner: None,
            victim: None,
            saw_where: None,
            heard_where: None,
            partner_route: None,
            caller: None,
            checker: None,
            checked: None,
            clear_room: None,
            breaker_room: None,
            check_eta: None,
            minutes: None,
            badge: None,
            pad_door: None,
        }
    }
}
impl Default for DialogueCtxTokens { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: DialoguePick, src/game/dialogue.ts:638)
#[derive(Clone, Debug)]
pub struct DialoguePick {
    pub text: String,
    pub id: String,
    pub kind: Option<crate::dialogue::IntelKind>,
    pub stage: crate::dialogue::StageKey,
    pub stage_n: f64,
    pub cell: String,
    pub k: crate::dialogue::Knowledge,
    pub band: crate::dialogue::Band,
    pub t: crate::game::Temperament,
    pub line_k: crate::dialogue::DialoguePickLineK,
    pub widened: crate::dialogue::DialoguePickWidened,
    pub after: Option<String>,
    pub script: Option<String>,
    pub shape: Option<crate::dialogue::ShapeKey>,
}
impl DialoguePick {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DialoguePick {
            text: String::new(),
            id: String::new(),
            kind: None,
            stage: crate::dialogue::StageKey::Num(0.0),
            stage_n: 0.0,
            cell: String::new(),
            k: crate::dialogue::Knowledge::None,
            band: crate::dialogue::Band::Low,
            t: crate::game::Temperament::Steady,
            line_k: crate::dialogue::DialoguePickLineK::None,
            widened: crate::dialogue::DialoguePickWidened::Empty,
            after: None,
            script: None,
            shape: None,
        }
    }
}
impl Default for DialoguePick { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum DialoguePickLineK {
    None,
    Heard,
    Glimpsed,
    Saw,
    SawHurt,
    Empty,
}
impl DialoguePickLineK {
    pub const fn as_str(self) -> &'static str {
        match self {
            DialoguePickLineK::None => "none",
            DialoguePickLineK::Heard => "heard",
            DialoguePickLineK::Glimpsed => "glimpsed",
            DialoguePickLineK::Saw => "saw",
            DialoguePickLineK::SawHurt => "sawHurt",
            DialoguePickLineK::Empty => "*",
        }
    }
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum DialoguePickWidened {
    Empty,
    T,
    A,
    Repeat,
}
impl DialoguePickWidened {
    pub const fn as_str(self) -> &'static str {
        match self {
            DialoguePickWidened::Empty => "",
            DialoguePickWidened::T => "t",
            DialoguePickWidened::A => "a",
            DialoguePickWidened::Repeat => "repeat",
        }
    }
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum SamStage {
    Press,
    Open,
    Ask,
    Dry,
}
impl SamStage {
    pub const fn as_str(self) -> &'static str {
        match self {
            SamStage::Press => "press",
            SamStage::Open => "open",
            SamStage::Ask => "ask",
            SamStage::Dry => "dry",
        }
    }
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum SamNeed {
    Knife,
    Blackout,
    Calm,
    Alarm,
    Gun,
    Arm,
    Lockdown,
    HeldLong,
}
impl SamNeed {
    pub const fn as_str(self) -> &'static str {
        match self {
            SamNeed::Knife => "knife",
            SamNeed::Blackout => "blackout",
            SamNeed::Calm => "calm",
            SamNeed::Alarm => "alarm",
            SamNeed::Gun => "gun",
            SamNeed::Arm => "arm",
            SamNeed::Lockdown => "lockdown",
            SamNeed::HeldLong => "heldLong",
        }
    }
}

/// (TypeScript: SamLine, src/game/dialogue.ts:805)
#[derive(Clone, Debug)]
pub struct SamLine {
    pub id: String,
    pub s: crate::dialogue::SamLineS,
    pub q: Option<crate::dialogue::IntelKind>,
    pub t: Option<crate::game::Temperament>,
    pub k: Option<crate::dialogue::Knowledge>,
    pub needs: Option<Vec<crate::dialogue::SamNeed>>,
    pub text: String,
    pub w: Option<f64>,
    pub now: Option<String>,
    pub away: Option<String>,
    pub guide: Option<String>,
}
impl SamLine {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SamLine {
            id: String::new(),
            s: crate::dialogue::SamLineS::Press,
            q: None,
            t: None,
            k: None,
            needs: None,
            text: String::new(),
            w: None,
            now: None,
            away: None,
            guide: None,
        }
    }
}
impl Default for SamLine { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum SamLineS {
    Press,
    Key,
    Open,
    Q,
    Ask,
    Dry,
    Script,
}
impl SamLineS {
    pub const fn as_str(self) -> &'static str {
        match self {
            SamLineS::Press => "press",
            SamLineS::Key => "key",
            SamLineS::Open => "open",
            SamLineS::Q => "q",
            SamLineS::Ask => "ask",
            SamLineS::Dry => "dry",
            SamLineS::Script => "script",
        }
    }
}

/// (TypeScript: SamMemory, src/game/dialogue.ts:928)
#[derive(Clone, Debug)]
pub struct SamMemory {
    pub said: JsMap<String, f64>,
    pub last: Option<String>,
}
impl SamMemory {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SamMemory {
            said: JsMap::new(),
            last: None,
        }
    }
}
impl Default for SamMemory { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: SamPick, src/game/dialogue.ts:932)
#[derive(Clone, Debug)]
pub struct SamPick {
    pub text: String,
    pub id: String,
    pub s: crate::dialogue::SamStage,
    pub q: Option<crate::dialogue::IntelKind>,
    pub stage: crate::dialogue::StageKey,
    pub fresh: bool,
    pub script: Option<String>,
}
impl SamPick {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SamPick {
            text: String::new(),
            id: String::new(),
            s: crate::dialogue::SamStage::Press,
            q: None,
            stage: crate::dialogue::StageKey::Num(0.0),
            fresh: false,
            script: None,
        }
    }
}
impl Default for SamPick { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: a union)
#[derive(Clone, Debug)]
pub enum Him {
    Text(String),
    Arm1(JsMap<String, String>),
}

/// (TypeScript: Beat, src/game/dialogue.ts:1267)
#[derive(Clone, Debug)]
pub struct Beat {
    pub sam: String,
    pub him: crate::dialogue::Him,
    pub after: Option<String>,
    pub ask: Option<String>,
    pub take: Option<String>,
    pub room: Option<bool>,
    pub mild: Option<String>,
    pub sam_now: Option<String>,
}
impl Beat {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Beat {
            sam: String::new(),
            him: crate::dialogue::Him::Text(String::new()),
            after: None,
            ask: None,
            take: None,
            room: None,
            mild: None,
            sam_now: None,
        }
    }
}
impl Default for Beat { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Script, src/game/dialogue.ts:1268)
#[derive(Clone, Debug)]
pub struct Script {
    pub id: String,
    pub shape: crate::dialogue::ShapeKey,
    pub t: crate::game::Temperament,
    pub k: Vec<crate::dialogue::Knowledge>,
    pub band: Option<Vec<crate::dialogue::Band>>,
    pub needs: Option<Vec<crate::dialogue::Need>>,
    pub not: Option<Vec<crate::dialogue::Need>>,
    pub pin: Option<crate::dialogue::IntelKind>,
    pub beats: crate::dialogue::ScriptBeats,
}
impl Script {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Script {
            id: String::new(),
            shape: crate::dialogue::ShapeKey::Pedant,
            t: crate::game::Temperament::Steady,
            k: Vec::new(),
            band: None,
            needs: None,
            not: None,
            pin: None,
            beats: crate::dialogue::ScriptBeats::zeroed(),
        }
    }
}
impl Default for Script { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/dialogue.ts:1274 Script.beats, ../.npm-global/lib/node_modules/typescript/lib/lib.es5.d.ts:1611)
#[derive(Clone, Debug)]
pub struct ScriptBeats {
    pub _3: crate::dialogue::Beat,
    pub _1: crate::dialogue::Beat,
    pub _2: crate::dialogue::Beat,
    pub x: crate::dialogue::Beat,
}
impl ScriptBeats {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ScriptBeats {
            _3: crate::dialogue::Beat::zeroed(),
            _1: crate::dialogue::Beat::zeroed(),
            _2: crate::dialogue::Beat::zeroed(),
            x: crate::dialogue::Beat::zeroed(),
        }
    }
}
impl Default for ScriptBeats { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: RoomLine, src/game/dialogue.ts:1337)
#[derive(Clone, Debug)]
pub struct RoomLine {
    pub sam: String,
    pub him: crate::dialogue::Him,
    pub after: Option<String>,
}
impl RoomLine {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RoomLine {
            sam: String::new(),
            him: crate::dialogue::Him::Text(String::new()),
            after: None,
        }
    }
}
impl Default for RoomLine { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum HandlerBeat {
    Blackout,
    Drive,
    Alarm,
    Rack,
    Lockdown,
    Server,
    BodyKo,
    BodyDead,
}
impl HandlerBeat {
    pub const fn as_str(self) -> &'static str {
        match self {
            HandlerBeat::Blackout => "blackout",
            HandlerBeat::Drive => "drive",
            HandlerBeat::Alarm => "alarm",
            HandlerBeat::Rack => "rack",
            HandlerBeat::Lockdown => "lockdown",
            HandlerBeat::Server => "server",
            HandlerBeat::BodyKo => "bodyKo",
            HandlerBeat::BodyDead => "bodyDead",
        }
    }
}

/// (TypeScript: RadioState, src/game/dialogue.ts:1713)
#[derive(Clone, Debug)]
pub struct RadioState {
    pub run: Option<u32>,
    pub done: JsSet<crate::dialogue::HandlerBeat>,
    pub queue: Vec<crate::dialogue::RadioStateQueueItem>,
    pub reply: Option<crate::dialogue::RadioStateReply>,
    pub last_at: f64,
    pub esc: f64,
    pub blackout: bool,
    pub found: f64,
}
impl RadioState {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RadioState {
            run: None,
            done: JsSet::new(),
            queue: Vec::new(),
            reply: None,
            last_at: 0.0,
            esc: 0.0,
            blackout: false,
            found: 0.0,
        }
    }
}
impl Default for RadioState { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at quiet RadioState.run, quiet RadioState.run)
#[derive(Clone, Copy, Debug)]
pub struct RadioStateRun {
}
impl RadioStateRun {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RadioStateRun {
        }
    }
}
impl Default for RadioStateRun { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/dialogue.ts:1713 RadioState.queue, src/game/dialogue.ts:1713)
#[derive(Clone, Copy, Debug)]
pub struct RadioStateQueueItem {
    pub at: f64,
    pub beat: crate::dialogue::HandlerBeat,
}
impl RadioStateQueueItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RadioStateQueueItem {
            at: 0.0,
            beat: crate::dialogue::HandlerBeat::Blackout,
        }
    }
}
impl Default for RadioStateQueueItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/dialogue.ts:1713 RadioState.reply, src/game/dialogue.ts:1713)
#[derive(Clone, Debug)]
pub struct RadioStateReply {
    pub at: f64,
    pub text: String,
}
impl RadioStateReply {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RadioStateReply {
            at: 0.0,
            text: String::new(),
        }
    }
}
impl Default for RadioStateReply { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: PairTalk, src/game/dialogue.ts:1767)
#[derive(Clone, Debug)]
pub struct PairTalk {
    pub id: String,
    pub lines: Vec<crate::dialogue::PairTalkLinesItem>,
    pub note: Option<String>,
    pub office: Option<bool>,
}
impl PairTalk {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        PairTalk {
            id: String::new(),
            lines: Vec::new(),
            note: None,
            office: None,
        }
    }
}
impl Default for PairTalk { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/dialogue.ts:1767 PairTalk.lines, src/game/dialogue.ts:1767)
#[derive(Clone, Debug)]
pub struct PairTalkLinesItem {
    pub who: f64,
    pub text: String,
    pub own: Option<String>,
}
impl PairTalkLinesItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        PairTalkLinesItem {
            who: 0.0,
            text: String::new(),
            own: None,
        }
    }
}
impl Default for PairTalkLinesItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: AmbientRun, src/game/dialogue.ts:1780)
#[derive(Clone, Debug)]
pub struct AmbientRun {
    pub a: GuardId,
    pub b: GuardId,
    pub talk: crate::dialogue::PairTalk,
    pub i: f64,
    pub at: f64,
    pub other: Option<String>,
    pub pad: Option<crate::dialogue::AmbientRunPad>,
    pub held: [bool; 2],
}
impl AmbientRun {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        AmbientRun {
            a: GuardId(0),
            b: GuardId(0),
            talk: crate::dialogue::PairTalk::zeroed(),
            i: 0.0,
            at: 0.0,
            other: None,
            pad: None,
            held: [false; 2],
        }
    }
}
impl Default for AmbientRun { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/dialogue.ts:1780 AmbientRun.pad, src/game/dialogue.ts:1780)
#[derive(Clone, Debug)]
pub struct AmbientRunPad {
    pub code: String,
    pub door: String,
    pub lock_id: String,
}
impl AmbientRunPad {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        AmbientRunPad {
            code: String::new(),
            door: String::new(),
            lock_id: String::new(),
        }
    }
}
impl Default for AmbientRunPad { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum HostageMoment {
    Shot,
    Wait,
    Threat,
    Aimed,
    Entry,
    Alone,
    Secured,
}
impl HostageMoment {
    pub const fn as_str(self) -> &'static str {
        match self {
            HostageMoment::Shot => "shot",
            HostageMoment::Wait => "wait",
            HostageMoment::Threat => "threat",
            HostageMoment::Aimed => "aimed",
            HostageMoment::Entry => "entry",
            HostageMoment::Alone => "alone",
            HostageMoment::Secured => "secured",
        }
    }
}

/// (TypeScript: AmbientState, src/game/dialogue.ts:2000)
#[derive(Clone, Debug)]
pub struct AmbientState {
    pub next: f64,
    pub run: Option<crate::dialogue::AmbientRun>,
    pub played: JsMap<String, f64>,
    pub count: f64,
}
impl AmbientState {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        AmbientState {
            next: 0.0,
            run: None,
            played: JsMap::new(),
            count: 0.0,
        }
    }
}
impl Default for AmbientState { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/dialogue.ts:187 TOKEN_WORST_NOW, ../.npm-global/lib/node_modules/typescript/lib/lib.es5.d.ts:1583)
#[derive(Clone, Debug)]
pub struct TOKENWORSTNOW {
    pub door: Option<String>,
    pub code: Option<String>,
    pub missing: Option<String>,
    pub body: Option<String>,
    pub n: Option<String>,
    pub up: Option<String>,
    pub post: Option<String>,
    pub room: Option<String>,
    pub route: Option<String>,
    pub partner: Option<String>,
    pub victim: Option<String>,
    pub saw_where: Option<String>,
    pub heard_where: Option<String>,
    pub partner_route: Option<String>,
    pub caller: Option<String>,
    pub checker: Option<String>,
    pub checked: Option<String>,
    pub clear_room: Option<String>,
    pub breaker_room: Option<String>,
    pub check_eta: Option<String>,
    pub minutes: Option<String>,
    pub badge: Option<String>,
    pub pad_door: Option<String>,
}
impl TOKENWORSTNOW {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        TOKENWORSTNOW {
            door: None,
            code: None,
            missing: None,
            body: None,
            n: None,
            up: None,
            post: None,
            room: None,
            route: None,
            partner: None,
            victim: None,
            saw_where: None,
            heard_where: None,
            partner_route: None,
            caller: None,
            checker: None,
            checked: None,
            clear_room: None,
            breaker_room: None,
            check_eta: None,
            minutes: None,
            badge: None,
            pad_door: None,
        }
    }
}
impl Default for TOKENWORSTNOW { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/dialogue.ts:319 INTEL_TEXT, ../.npm-global/lib/node_modules/typescript/lib/lib.es5.d.ts:1583)
#[derive(Clone, Debug)]
pub struct INTELTEXTValue {
    pub steady: Option<String>,
    pub nervy: Option<String>,
    pub hard: Option<String>,
    pub under: Option<String>,
    pub pedant: Option<String>,
    pub selfown: Option<String>,
    pub folder: Option<String>,
    pub ramble: Option<String>,
    pub fan: Option<String>,
    pub stoic: Option<String>,
    pub con: Option<String>,
    pub game: Option<String>,
    pub menace: Option<String>,
    pub steady_none: Option<String>,
    pub steady_heard: Option<String>,
    pub steady_glimpsed: Option<String>,
    pub steady_saw: Option<String>,
    pub steady_saw_hurt: Option<String>,
    pub nervy_none: Option<String>,
    pub nervy_heard: Option<String>,
    pub nervy_glimpsed: Option<String>,
    pub nervy_saw: Option<String>,
    pub nervy_saw_hurt: Option<String>,
    pub hard_none: Option<String>,
    pub hard_heard: Option<String>,
    pub hard_glimpsed: Option<String>,
    pub hard_saw: Option<String>,
    pub hard_saw_hurt: Option<String>,
    pub __none: Option<String>,
    pub __heard: Option<String>,
    pub __glimpsed: Option<String>,
    pub __saw: Option<String>,
    pub __saw_hurt: Option<String>,
    pub pedant_steady: Option<String>,
    pub pedant_nervy: Option<String>,
    pub pedant_hard: Option<String>,
    pub selfown_steady: Option<String>,
    pub selfown_nervy: Option<String>,
    pub selfown_hard: Option<String>,
    pub folder_steady: Option<String>,
    pub folder_nervy: Option<String>,
    pub folder_hard: Option<String>,
    pub ramble_steady: Option<String>,
    pub ramble_nervy: Option<String>,
    pub ramble_hard: Option<String>,
    pub fan_steady: Option<String>,
    pub fan_nervy: Option<String>,
    pub fan_hard: Option<String>,
    pub stoic_steady: Option<String>,
    pub stoic_nervy: Option<String>,
    pub stoic_hard: Option<String>,
    pub con_steady: Option<String>,
    pub con_nervy: Option<String>,
    pub con_hard: Option<String>,
    pub game_steady: Option<String>,
    pub game_nervy: Option<String>,
    pub game_hard: Option<String>,
    pub menace_steady: Option<String>,
    pub menace_nervy: Option<String>,
    pub menace_hard: Option<String>,
}
impl INTELTEXTValue {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        INTELTEXTValue {
            steady: None,
            nervy: None,
            hard: None,
            under: None,
            pedant: None,
            selfown: None,
            folder: None,
            ramble: None,
            fan: None,
            stoic: None,
            con: None,
            game: None,
            menace: None,
            steady_none: None,
            steady_heard: None,
            steady_glimpsed: None,
            steady_saw: None,
            steady_saw_hurt: None,
            nervy_none: None,
            nervy_heard: None,
            nervy_glimpsed: None,
            nervy_saw: None,
            nervy_saw_hurt: None,
            hard_none: None,
            hard_heard: None,
            hard_glimpsed: None,
            hard_saw: None,
            hard_saw_hurt: None,
            __none: None,
            __heard: None,
            __glimpsed: None,
            __saw: None,
            __saw_hurt: None,
            pedant_steady: None,
            pedant_nervy: None,
            pedant_hard: None,
            selfown_steady: None,
            selfown_nervy: None,
            selfown_hard: None,
            folder_steady: None,
            folder_nervy: None,
            folder_hard: None,
            ramble_steady: None,
            ramble_nervy: None,
            ramble_hard: None,
            fan_steady: None,
            fan_nervy: None,
            fan_hard: None,
            stoic_steady: None,
            stoic_nervy: None,
            stoic_hard: None,
            con_steady: None,
            con_nervy: None,
            con_hard: None,
            game_steady: None,
            game_nervy: None,
            game_hard: None,
            menace_steady: None,
            menace_nervy: None,
            menace_hard: None,
        }
    }
}
impl Default for INTELTEXTValue { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/dialogue.ts:340 INTEL_NOW, ../.npm-global/lib/node_modules/typescript/lib/lib.es5.d.ts:1583)
#[derive(Clone, Debug)]
pub struct INTELNOW {
    pub check_on: Option<crate::dialogue::INTELNOWCheckOn>,
    pub clearing: Option<crate::dialogue::INTELNOWClearing>,
    pub rollcall_live: Option<crate::dialogue::INTELNOWRollcallLive>,
    pub blackout: Option<crate::dialogue::INTELNOWBlackout>,
    pub door: Option<crate::dialogue::INTELNOWDoor>,
    pub code: Option<crate::dialogue::INTELNOWCode>,
    pub rollcall: Option<crate::dialogue::INTELNOWRollcall>,
    pub lockdown_posts: Option<crate::dialogue::INTELNOWLockdownPosts>,
    pub routes: Option<crate::dialogue::INTELNOWRoutes>,
    pub missing: Option<crate::dialogue::INTELNOWMissing>,
    pub drive: Option<crate::dialogue::INTELNOWDrive>,
}
impl INTELNOW {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        INTELNOW {
            check_on: None,
            clearing: None,
            rollcall_live: None,
            blackout: None,
            door: None,
            code: None,
            rollcall: None,
            lockdown_posts: None,
            routes: None,
            missing: None,
            drive: None,
        }
    }
}
impl Default for INTELNOW { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at INTELNOW.checkOn, ../.npm-global/lib/node_modules/typescript/lib/lib.es5.d.ts:1583)
#[derive(Clone, Debug)]
pub struct INTELNOWCheckOn {
    pub steady: Option<String>,
    pub nervy: Option<String>,
    pub hard: Option<String>,
    pub under: Option<String>,
    pub pedant: Option<String>,
    pub selfown: Option<String>,
    pub folder: Option<String>,
    pub ramble: Option<String>,
    pub fan: Option<String>,
    pub stoic: Option<String>,
    pub con: Option<String>,
    pub game: Option<String>,
    pub menace: Option<String>,
    pub steady_none: Option<String>,
    pub steady_heard: Option<String>,
    pub steady_glimpsed: Option<String>,
    pub steady_saw: Option<String>,
    pub steady_saw_hurt: Option<String>,
    pub nervy_none: Option<String>,
    pub nervy_heard: Option<String>,
    pub nervy_glimpsed: Option<String>,
    pub nervy_saw: Option<String>,
    pub nervy_saw_hurt: Option<String>,
    pub hard_none: Option<String>,
    pub hard_heard: Option<String>,
    pub hard_glimpsed: Option<String>,
    pub hard_saw: Option<String>,
    pub hard_saw_hurt: Option<String>,
    pub __none: Option<String>,
    pub __heard: Option<String>,
    pub __glimpsed: Option<String>,
    pub __saw: Option<String>,
    pub __saw_hurt: Option<String>,
    pub pedant_steady: Option<String>,
    pub pedant_nervy: Option<String>,
    pub pedant_hard: Option<String>,
    pub selfown_steady: Option<String>,
    pub selfown_nervy: Option<String>,
    pub selfown_hard: Option<String>,
    pub folder_steady: Option<String>,
    pub folder_nervy: Option<String>,
    pub folder_hard: Option<String>,
    pub ramble_steady: Option<String>,
    pub ramble_nervy: Option<String>,
    pub ramble_hard: Option<String>,
    pub fan_steady: Option<String>,
    pub fan_nervy: Option<String>,
    pub fan_hard: Option<String>,
    pub stoic_steady: Option<String>,
    pub stoic_nervy: Option<String>,
    pub stoic_hard: Option<String>,
    pub con_steady: Option<String>,
    pub con_nervy: Option<String>,
    pub con_hard: Option<String>,
    pub game_steady: Option<String>,
    pub game_nervy: Option<String>,
    pub game_hard: Option<String>,
    pub menace_steady: Option<String>,
    pub menace_nervy: Option<String>,
    pub menace_hard: Option<String>,
}
impl INTELNOWCheckOn {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        INTELNOWCheckOn {
            steady: None,
            nervy: None,
            hard: None,
            under: None,
            pedant: None,
            selfown: None,
            folder: None,
            ramble: None,
            fan: None,
            stoic: None,
            con: None,
            game: None,
            menace: None,
            steady_none: None,
            steady_heard: None,
            steady_glimpsed: None,
            steady_saw: None,
            steady_saw_hurt: None,
            nervy_none: None,
            nervy_heard: None,
            nervy_glimpsed: None,
            nervy_saw: None,
            nervy_saw_hurt: None,
            hard_none: None,
            hard_heard: None,
            hard_glimpsed: None,
            hard_saw: None,
            hard_saw_hurt: None,
            __none: None,
            __heard: None,
            __glimpsed: None,
            __saw: None,
            __saw_hurt: None,
            pedant_steady: None,
            pedant_nervy: None,
            pedant_hard: None,
            selfown_steady: None,
            selfown_nervy: None,
            selfown_hard: None,
            folder_steady: None,
            folder_nervy: None,
            folder_hard: None,
            ramble_steady: None,
            ramble_nervy: None,
            ramble_hard: None,
            fan_steady: None,
            fan_nervy: None,
            fan_hard: None,
            stoic_steady: None,
            stoic_nervy: None,
            stoic_hard: None,
            con_steady: None,
            con_nervy: None,
            con_hard: None,
            game_steady: None,
            game_nervy: None,
            game_hard: None,
            menace_steady: None,
            menace_nervy: None,
            menace_hard: None,
        }
    }
}
impl Default for INTELNOWCheckOn { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at INTELNOW.clearing, ../.npm-global/lib/node_modules/typescript/lib/lib.es5.d.ts:1583)
#[derive(Clone, Debug)]
pub struct INTELNOWClearing {
    pub steady: Option<String>,
    pub nervy: Option<String>,
    pub hard: Option<String>,
    pub under: Option<String>,
    pub pedant: Option<String>,
    pub selfown: Option<String>,
    pub folder: Option<String>,
    pub ramble: Option<String>,
    pub fan: Option<String>,
    pub stoic: Option<String>,
    pub con: Option<String>,
    pub game: Option<String>,
    pub menace: Option<String>,
    pub steady_none: Option<String>,
    pub steady_heard: Option<String>,
    pub steady_glimpsed: Option<String>,
    pub steady_saw: Option<String>,
    pub steady_saw_hurt: Option<String>,
    pub nervy_none: Option<String>,
    pub nervy_heard: Option<String>,
    pub nervy_glimpsed: Option<String>,
    pub nervy_saw: Option<String>,
    pub nervy_saw_hurt: Option<String>,
    pub hard_none: Option<String>,
    pub hard_heard: Option<String>,
    pub hard_glimpsed: Option<String>,
    pub hard_saw: Option<String>,
    pub hard_saw_hurt: Option<String>,
    pub __none: Option<String>,
    pub __heard: Option<String>,
    pub __glimpsed: Option<String>,
    pub __saw: Option<String>,
    pub __saw_hurt: Option<String>,
    pub pedant_steady: Option<String>,
    pub pedant_nervy: Option<String>,
    pub pedant_hard: Option<String>,
    pub selfown_steady: Option<String>,
    pub selfown_nervy: Option<String>,
    pub selfown_hard: Option<String>,
    pub folder_steady: Option<String>,
    pub folder_nervy: Option<String>,
    pub folder_hard: Option<String>,
    pub ramble_steady: Option<String>,
    pub ramble_nervy: Option<String>,
    pub ramble_hard: Option<String>,
    pub fan_steady: Option<String>,
    pub fan_nervy: Option<String>,
    pub fan_hard: Option<String>,
    pub stoic_steady: Option<String>,
    pub stoic_nervy: Option<String>,
    pub stoic_hard: Option<String>,
    pub con_steady: Option<String>,
    pub con_nervy: Option<String>,
    pub con_hard: Option<String>,
    pub game_steady: Option<String>,
    pub game_nervy: Option<String>,
    pub game_hard: Option<String>,
    pub menace_steady: Option<String>,
    pub menace_nervy: Option<String>,
    pub menace_hard: Option<String>,
}
impl INTELNOWClearing {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        INTELNOWClearing {
            steady: None,
            nervy: None,
            hard: None,
            under: None,
            pedant: None,
            selfown: None,
            folder: None,
            ramble: None,
            fan: None,
            stoic: None,
            con: None,
            game: None,
            menace: None,
            steady_none: None,
            steady_heard: None,
            steady_glimpsed: None,
            steady_saw: None,
            steady_saw_hurt: None,
            nervy_none: None,
            nervy_heard: None,
            nervy_glimpsed: None,
            nervy_saw: None,
            nervy_saw_hurt: None,
            hard_none: None,
            hard_heard: None,
            hard_glimpsed: None,
            hard_saw: None,
            hard_saw_hurt: None,
            __none: None,
            __heard: None,
            __glimpsed: None,
            __saw: None,
            __saw_hurt: None,
            pedant_steady: None,
            pedant_nervy: None,
            pedant_hard: None,
            selfown_steady: None,
            selfown_nervy: None,
            selfown_hard: None,
            folder_steady: None,
            folder_nervy: None,
            folder_hard: None,
            ramble_steady: None,
            ramble_nervy: None,
            ramble_hard: None,
            fan_steady: None,
            fan_nervy: None,
            fan_hard: None,
            stoic_steady: None,
            stoic_nervy: None,
            stoic_hard: None,
            con_steady: None,
            con_nervy: None,
            con_hard: None,
            game_steady: None,
            game_nervy: None,
            game_hard: None,
            menace_steady: None,
            menace_nervy: None,
            menace_hard: None,
        }
    }
}
impl Default for INTELNOWClearing { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at INTELNOW.rollcallLive, ../.npm-global/lib/node_modules/typescript/lib/lib.es5.d.ts:1583)
#[derive(Clone, Debug)]
pub struct INTELNOWRollcallLive {
    pub steady: Option<String>,
    pub nervy: Option<String>,
    pub hard: Option<String>,
    pub under: Option<String>,
    pub pedant: Option<String>,
    pub selfown: Option<String>,
    pub folder: Option<String>,
    pub ramble: Option<String>,
    pub fan: Option<String>,
    pub stoic: Option<String>,
    pub con: Option<String>,
    pub game: Option<String>,
    pub menace: Option<String>,
    pub steady_none: Option<String>,
    pub steady_heard: Option<String>,
    pub steady_glimpsed: Option<String>,
    pub steady_saw: Option<String>,
    pub steady_saw_hurt: Option<String>,
    pub nervy_none: Option<String>,
    pub nervy_heard: Option<String>,
    pub nervy_glimpsed: Option<String>,
    pub nervy_saw: Option<String>,
    pub nervy_saw_hurt: Option<String>,
    pub hard_none: Option<String>,
    pub hard_heard: Option<String>,
    pub hard_glimpsed: Option<String>,
    pub hard_saw: Option<String>,
    pub hard_saw_hurt: Option<String>,
    pub __none: Option<String>,
    pub __heard: Option<String>,
    pub __glimpsed: Option<String>,
    pub __saw: Option<String>,
    pub __saw_hurt: Option<String>,
    pub pedant_steady: Option<String>,
    pub pedant_nervy: Option<String>,
    pub pedant_hard: Option<String>,
    pub selfown_steady: Option<String>,
    pub selfown_nervy: Option<String>,
    pub selfown_hard: Option<String>,
    pub folder_steady: Option<String>,
    pub folder_nervy: Option<String>,
    pub folder_hard: Option<String>,
    pub ramble_steady: Option<String>,
    pub ramble_nervy: Option<String>,
    pub ramble_hard: Option<String>,
    pub fan_steady: Option<String>,
    pub fan_nervy: Option<String>,
    pub fan_hard: Option<String>,
    pub stoic_steady: Option<String>,
    pub stoic_nervy: Option<String>,
    pub stoic_hard: Option<String>,
    pub con_steady: Option<String>,
    pub con_nervy: Option<String>,
    pub con_hard: Option<String>,
    pub game_steady: Option<String>,
    pub game_nervy: Option<String>,
    pub game_hard: Option<String>,
    pub menace_steady: Option<String>,
    pub menace_nervy: Option<String>,
    pub menace_hard: Option<String>,
}
impl INTELNOWRollcallLive {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        INTELNOWRollcallLive {
            steady: None,
            nervy: None,
            hard: None,
            under: None,
            pedant: None,
            selfown: None,
            folder: None,
            ramble: None,
            fan: None,
            stoic: None,
            con: None,
            game: None,
            menace: None,
            steady_none: None,
            steady_heard: None,
            steady_glimpsed: None,
            steady_saw: None,
            steady_saw_hurt: None,
            nervy_none: None,
            nervy_heard: None,
            nervy_glimpsed: None,
            nervy_saw: None,
            nervy_saw_hurt: None,
            hard_none: None,
            hard_heard: None,
            hard_glimpsed: None,
            hard_saw: None,
            hard_saw_hurt: None,
            __none: None,
            __heard: None,
            __glimpsed: None,
            __saw: None,
            __saw_hurt: None,
            pedant_steady: None,
            pedant_nervy: None,
            pedant_hard: None,
            selfown_steady: None,
            selfown_nervy: None,
            selfown_hard: None,
            folder_steady: None,
            folder_nervy: None,
            folder_hard: None,
            ramble_steady: None,
            ramble_nervy: None,
            ramble_hard: None,
            fan_steady: None,
            fan_nervy: None,
            fan_hard: None,
            stoic_steady: None,
            stoic_nervy: None,
            stoic_hard: None,
            con_steady: None,
            con_nervy: None,
            con_hard: None,
            game_steady: None,
            game_nervy: None,
            game_hard: None,
            menace_steady: None,
            menace_nervy: None,
            menace_hard: None,
        }
    }
}
impl Default for INTELNOWRollcallLive { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at INTELNOW.blackout, ../.npm-global/lib/node_modules/typescript/lib/lib.es5.d.ts:1583)
#[derive(Clone, Debug)]
pub struct INTELNOWBlackout {
    pub steady: Option<String>,
    pub nervy: Option<String>,
    pub hard: Option<String>,
    pub under: Option<String>,
    pub pedant: Option<String>,
    pub selfown: Option<String>,
    pub folder: Option<String>,
    pub ramble: Option<String>,
    pub fan: Option<String>,
    pub stoic: Option<String>,
    pub con: Option<String>,
    pub game: Option<String>,
    pub menace: Option<String>,
    pub steady_none: Option<String>,
    pub steady_heard: Option<String>,
    pub steady_glimpsed: Option<String>,
    pub steady_saw: Option<String>,
    pub steady_saw_hurt: Option<String>,
    pub nervy_none: Option<String>,
    pub nervy_heard: Option<String>,
    pub nervy_glimpsed: Option<String>,
    pub nervy_saw: Option<String>,
    pub nervy_saw_hurt: Option<String>,
    pub hard_none: Option<String>,
    pub hard_heard: Option<String>,
    pub hard_glimpsed: Option<String>,
    pub hard_saw: Option<String>,
    pub hard_saw_hurt: Option<String>,
    pub __none: Option<String>,
    pub __heard: Option<String>,
    pub __glimpsed: Option<String>,
    pub __saw: Option<String>,
    pub __saw_hurt: Option<String>,
    pub pedant_steady: Option<String>,
    pub pedant_nervy: Option<String>,
    pub pedant_hard: Option<String>,
    pub selfown_steady: Option<String>,
    pub selfown_nervy: Option<String>,
    pub selfown_hard: Option<String>,
    pub folder_steady: Option<String>,
    pub folder_nervy: Option<String>,
    pub folder_hard: Option<String>,
    pub ramble_steady: Option<String>,
    pub ramble_nervy: Option<String>,
    pub ramble_hard: Option<String>,
    pub fan_steady: Option<String>,
    pub fan_nervy: Option<String>,
    pub fan_hard: Option<String>,
    pub stoic_steady: Option<String>,
    pub stoic_nervy: Option<String>,
    pub stoic_hard: Option<String>,
    pub con_steady: Option<String>,
    pub con_nervy: Option<String>,
    pub con_hard: Option<String>,
    pub game_steady: Option<String>,
    pub game_nervy: Option<String>,
    pub game_hard: Option<String>,
    pub menace_steady: Option<String>,
    pub menace_nervy: Option<String>,
    pub menace_hard: Option<String>,
}
impl INTELNOWBlackout {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        INTELNOWBlackout {
            steady: None,
            nervy: None,
            hard: None,
            under: None,
            pedant: None,
            selfown: None,
            folder: None,
            ramble: None,
            fan: None,
            stoic: None,
            con: None,
            game: None,
            menace: None,
            steady_none: None,
            steady_heard: None,
            steady_glimpsed: None,
            steady_saw: None,
            steady_saw_hurt: None,
            nervy_none: None,
            nervy_heard: None,
            nervy_glimpsed: None,
            nervy_saw: None,
            nervy_saw_hurt: None,
            hard_none: None,
            hard_heard: None,
            hard_glimpsed: None,
            hard_saw: None,
            hard_saw_hurt: None,
            __none: None,
            __heard: None,
            __glimpsed: None,
            __saw: None,
            __saw_hurt: None,
            pedant_steady: None,
            pedant_nervy: None,
            pedant_hard: None,
            selfown_steady: None,
            selfown_nervy: None,
            selfown_hard: None,
            folder_steady: None,
            folder_nervy: None,
            folder_hard: None,
            ramble_steady: None,
            ramble_nervy: None,
            ramble_hard: None,
            fan_steady: None,
            fan_nervy: None,
            fan_hard: None,
            stoic_steady: None,
            stoic_nervy: None,
            stoic_hard: None,
            con_steady: None,
            con_nervy: None,
            con_hard: None,
            game_steady: None,
            game_nervy: None,
            game_hard: None,
            menace_steady: None,
            menace_nervy: None,
            menace_hard: None,
        }
    }
}
impl Default for INTELNOWBlackout { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at INTELNOW.door, ../.npm-global/lib/node_modules/typescript/lib/lib.es5.d.ts:1583)
#[derive(Clone, Debug)]
pub struct INTELNOWDoor {
    pub steady: Option<String>,
    pub nervy: Option<String>,
    pub hard: Option<String>,
    pub under: Option<String>,
    pub pedant: Option<String>,
    pub selfown: Option<String>,
    pub folder: Option<String>,
    pub ramble: Option<String>,
    pub fan: Option<String>,
    pub stoic: Option<String>,
    pub con: Option<String>,
    pub game: Option<String>,
    pub menace: Option<String>,
    pub steady_none: Option<String>,
    pub steady_heard: Option<String>,
    pub steady_glimpsed: Option<String>,
    pub steady_saw: Option<String>,
    pub steady_saw_hurt: Option<String>,
    pub nervy_none: Option<String>,
    pub nervy_heard: Option<String>,
    pub nervy_glimpsed: Option<String>,
    pub nervy_saw: Option<String>,
    pub nervy_saw_hurt: Option<String>,
    pub hard_none: Option<String>,
    pub hard_heard: Option<String>,
    pub hard_glimpsed: Option<String>,
    pub hard_saw: Option<String>,
    pub hard_saw_hurt: Option<String>,
    pub __none: Option<String>,
    pub __heard: Option<String>,
    pub __glimpsed: Option<String>,
    pub __saw: Option<String>,
    pub __saw_hurt: Option<String>,
    pub pedant_steady: Option<String>,
    pub pedant_nervy: Option<String>,
    pub pedant_hard: Option<String>,
    pub selfown_steady: Option<String>,
    pub selfown_nervy: Option<String>,
    pub selfown_hard: Option<String>,
    pub folder_steady: Option<String>,
    pub folder_nervy: Option<String>,
    pub folder_hard: Option<String>,
    pub ramble_steady: Option<String>,
    pub ramble_nervy: Option<String>,
    pub ramble_hard: Option<String>,
    pub fan_steady: Option<String>,
    pub fan_nervy: Option<String>,
    pub fan_hard: Option<String>,
    pub stoic_steady: Option<String>,
    pub stoic_nervy: Option<String>,
    pub stoic_hard: Option<String>,
    pub con_steady: Option<String>,
    pub con_nervy: Option<String>,
    pub con_hard: Option<String>,
    pub game_steady: Option<String>,
    pub game_nervy: Option<String>,
    pub game_hard: Option<String>,
    pub menace_steady: Option<String>,
    pub menace_nervy: Option<String>,
    pub menace_hard: Option<String>,
}
impl INTELNOWDoor {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        INTELNOWDoor {
            steady: None,
            nervy: None,
            hard: None,
            under: None,
            pedant: None,
            selfown: None,
            folder: None,
            ramble: None,
            fan: None,
            stoic: None,
            con: None,
            game: None,
            menace: None,
            steady_none: None,
            steady_heard: None,
            steady_glimpsed: None,
            steady_saw: None,
            steady_saw_hurt: None,
            nervy_none: None,
            nervy_heard: None,
            nervy_glimpsed: None,
            nervy_saw: None,
            nervy_saw_hurt: None,
            hard_none: None,
            hard_heard: None,
            hard_glimpsed: None,
            hard_saw: None,
            hard_saw_hurt: None,
            __none: None,
            __heard: None,
            __glimpsed: None,
            __saw: None,
            __saw_hurt: None,
            pedant_steady: None,
            pedant_nervy: None,
            pedant_hard: None,
            selfown_steady: None,
            selfown_nervy: None,
            selfown_hard: None,
            folder_steady: None,
            folder_nervy: None,
            folder_hard: None,
            ramble_steady: None,
            ramble_nervy: None,
            ramble_hard: None,
            fan_steady: None,
            fan_nervy: None,
            fan_hard: None,
            stoic_steady: None,
            stoic_nervy: None,
            stoic_hard: None,
            con_steady: None,
            con_nervy: None,
            con_hard: None,
            game_steady: None,
            game_nervy: None,
            game_hard: None,
            menace_steady: None,
            menace_nervy: None,
            menace_hard: None,
        }
    }
}
impl Default for INTELNOWDoor { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at INTELNOW.code, ../.npm-global/lib/node_modules/typescript/lib/lib.es5.d.ts:1583)
#[derive(Clone, Debug)]
pub struct INTELNOWCode {
    pub steady: Option<String>,
    pub nervy: Option<String>,
    pub hard: Option<String>,
    pub under: Option<String>,
    pub pedant: Option<String>,
    pub selfown: Option<String>,
    pub folder: Option<String>,
    pub ramble: Option<String>,
    pub fan: Option<String>,
    pub stoic: Option<String>,
    pub con: Option<String>,
    pub game: Option<String>,
    pub menace: Option<String>,
    pub steady_none: Option<String>,
    pub steady_heard: Option<String>,
    pub steady_glimpsed: Option<String>,
    pub steady_saw: Option<String>,
    pub steady_saw_hurt: Option<String>,
    pub nervy_none: Option<String>,
    pub nervy_heard: Option<String>,
    pub nervy_glimpsed: Option<String>,
    pub nervy_saw: Option<String>,
    pub nervy_saw_hurt: Option<String>,
    pub hard_none: Option<String>,
    pub hard_heard: Option<String>,
    pub hard_glimpsed: Option<String>,
    pub hard_saw: Option<String>,
    pub hard_saw_hurt: Option<String>,
    pub __none: Option<String>,
    pub __heard: Option<String>,
    pub __glimpsed: Option<String>,
    pub __saw: Option<String>,
    pub __saw_hurt: Option<String>,
    pub pedant_steady: Option<String>,
    pub pedant_nervy: Option<String>,
    pub pedant_hard: Option<String>,
    pub selfown_steady: Option<String>,
    pub selfown_nervy: Option<String>,
    pub selfown_hard: Option<String>,
    pub folder_steady: Option<String>,
    pub folder_nervy: Option<String>,
    pub folder_hard: Option<String>,
    pub ramble_steady: Option<String>,
    pub ramble_nervy: Option<String>,
    pub ramble_hard: Option<String>,
    pub fan_steady: Option<String>,
    pub fan_nervy: Option<String>,
    pub fan_hard: Option<String>,
    pub stoic_steady: Option<String>,
    pub stoic_nervy: Option<String>,
    pub stoic_hard: Option<String>,
    pub con_steady: Option<String>,
    pub con_nervy: Option<String>,
    pub con_hard: Option<String>,
    pub game_steady: Option<String>,
    pub game_nervy: Option<String>,
    pub game_hard: Option<String>,
    pub menace_steady: Option<String>,
    pub menace_nervy: Option<String>,
    pub menace_hard: Option<String>,
}
impl INTELNOWCode {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        INTELNOWCode {
            steady: None,
            nervy: None,
            hard: None,
            under: None,
            pedant: None,
            selfown: None,
            folder: None,
            ramble: None,
            fan: None,
            stoic: None,
            con: None,
            game: None,
            menace: None,
            steady_none: None,
            steady_heard: None,
            steady_glimpsed: None,
            steady_saw: None,
            steady_saw_hurt: None,
            nervy_none: None,
            nervy_heard: None,
            nervy_glimpsed: None,
            nervy_saw: None,
            nervy_saw_hurt: None,
            hard_none: None,
            hard_heard: None,
            hard_glimpsed: None,
            hard_saw: None,
            hard_saw_hurt: None,
            __none: None,
            __heard: None,
            __glimpsed: None,
            __saw: None,
            __saw_hurt: None,
            pedant_steady: None,
            pedant_nervy: None,
            pedant_hard: None,
            selfown_steady: None,
            selfown_nervy: None,
            selfown_hard: None,
            folder_steady: None,
            folder_nervy: None,
            folder_hard: None,
            ramble_steady: None,
            ramble_nervy: None,
            ramble_hard: None,
            fan_steady: None,
            fan_nervy: None,
            fan_hard: None,
            stoic_steady: None,
            stoic_nervy: None,
            stoic_hard: None,
            con_steady: None,
            con_nervy: None,
            con_hard: None,
            game_steady: None,
            game_nervy: None,
            game_hard: None,
            menace_steady: None,
            menace_nervy: None,
            menace_hard: None,
        }
    }
}
impl Default for INTELNOWCode { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at INTELNOW.rollcall, ../.npm-global/lib/node_modules/typescript/lib/lib.es5.d.ts:1583)
#[derive(Clone, Debug)]
pub struct INTELNOWRollcall {
    pub steady: Option<String>,
    pub nervy: Option<String>,
    pub hard: Option<String>,
    pub under: Option<String>,
    pub pedant: Option<String>,
    pub selfown: Option<String>,
    pub folder: Option<String>,
    pub ramble: Option<String>,
    pub fan: Option<String>,
    pub stoic: Option<String>,
    pub con: Option<String>,
    pub game: Option<String>,
    pub menace: Option<String>,
    pub steady_none: Option<String>,
    pub steady_heard: Option<String>,
    pub steady_glimpsed: Option<String>,
    pub steady_saw: Option<String>,
    pub steady_saw_hurt: Option<String>,
    pub nervy_none: Option<String>,
    pub nervy_heard: Option<String>,
    pub nervy_glimpsed: Option<String>,
    pub nervy_saw: Option<String>,
    pub nervy_saw_hurt: Option<String>,
    pub hard_none: Option<String>,
    pub hard_heard: Option<String>,
    pub hard_glimpsed: Option<String>,
    pub hard_saw: Option<String>,
    pub hard_saw_hurt: Option<String>,
    pub __none: Option<String>,
    pub __heard: Option<String>,
    pub __glimpsed: Option<String>,
    pub __saw: Option<String>,
    pub __saw_hurt: Option<String>,
    pub pedant_steady: Option<String>,
    pub pedant_nervy: Option<String>,
    pub pedant_hard: Option<String>,
    pub selfown_steady: Option<String>,
    pub selfown_nervy: Option<String>,
    pub selfown_hard: Option<String>,
    pub folder_steady: Option<String>,
    pub folder_nervy: Option<String>,
    pub folder_hard: Option<String>,
    pub ramble_steady: Option<String>,
    pub ramble_nervy: Option<String>,
    pub ramble_hard: Option<String>,
    pub fan_steady: Option<String>,
    pub fan_nervy: Option<String>,
    pub fan_hard: Option<String>,
    pub stoic_steady: Option<String>,
    pub stoic_nervy: Option<String>,
    pub stoic_hard: Option<String>,
    pub con_steady: Option<String>,
    pub con_nervy: Option<String>,
    pub con_hard: Option<String>,
    pub game_steady: Option<String>,
    pub game_nervy: Option<String>,
    pub game_hard: Option<String>,
    pub menace_steady: Option<String>,
    pub menace_nervy: Option<String>,
    pub menace_hard: Option<String>,
}
impl INTELNOWRollcall {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        INTELNOWRollcall {
            steady: None,
            nervy: None,
            hard: None,
            under: None,
            pedant: None,
            selfown: None,
            folder: None,
            ramble: None,
            fan: None,
            stoic: None,
            con: None,
            game: None,
            menace: None,
            steady_none: None,
            steady_heard: None,
            steady_glimpsed: None,
            steady_saw: None,
            steady_saw_hurt: None,
            nervy_none: None,
            nervy_heard: None,
            nervy_glimpsed: None,
            nervy_saw: None,
            nervy_saw_hurt: None,
            hard_none: None,
            hard_heard: None,
            hard_glimpsed: None,
            hard_saw: None,
            hard_saw_hurt: None,
            __none: None,
            __heard: None,
            __glimpsed: None,
            __saw: None,
            __saw_hurt: None,
            pedant_steady: None,
            pedant_nervy: None,
            pedant_hard: None,
            selfown_steady: None,
            selfown_nervy: None,
            selfown_hard: None,
            folder_steady: None,
            folder_nervy: None,
            folder_hard: None,
            ramble_steady: None,
            ramble_nervy: None,
            ramble_hard: None,
            fan_steady: None,
            fan_nervy: None,
            fan_hard: None,
            stoic_steady: None,
            stoic_nervy: None,
            stoic_hard: None,
            con_steady: None,
            con_nervy: None,
            con_hard: None,
            game_steady: None,
            game_nervy: None,
            game_hard: None,
            menace_steady: None,
            menace_nervy: None,
            menace_hard: None,
        }
    }
}
impl Default for INTELNOWRollcall { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at INTELNOW.lockdownPosts, ../.npm-global/lib/node_modules/typescript/lib/lib.es5.d.ts:1583)
#[derive(Clone, Debug)]
pub struct INTELNOWLockdownPosts {
    pub steady: Option<String>,
    pub nervy: Option<String>,
    pub hard: Option<String>,
    pub under: Option<String>,
    pub pedant: Option<String>,
    pub selfown: Option<String>,
    pub folder: Option<String>,
    pub ramble: Option<String>,
    pub fan: Option<String>,
    pub stoic: Option<String>,
    pub con: Option<String>,
    pub game: Option<String>,
    pub menace: Option<String>,
    pub steady_none: Option<String>,
    pub steady_heard: Option<String>,
    pub steady_glimpsed: Option<String>,
    pub steady_saw: Option<String>,
    pub steady_saw_hurt: Option<String>,
    pub nervy_none: Option<String>,
    pub nervy_heard: Option<String>,
    pub nervy_glimpsed: Option<String>,
    pub nervy_saw: Option<String>,
    pub nervy_saw_hurt: Option<String>,
    pub hard_none: Option<String>,
    pub hard_heard: Option<String>,
    pub hard_glimpsed: Option<String>,
    pub hard_saw: Option<String>,
    pub hard_saw_hurt: Option<String>,
    pub __none: Option<String>,
    pub __heard: Option<String>,
    pub __glimpsed: Option<String>,
    pub __saw: Option<String>,
    pub __saw_hurt: Option<String>,
    pub pedant_steady: Option<String>,
    pub pedant_nervy: Option<String>,
    pub pedant_hard: Option<String>,
    pub selfown_steady: Option<String>,
    pub selfown_nervy: Option<String>,
    pub selfown_hard: Option<String>,
    pub folder_steady: Option<String>,
    pub folder_nervy: Option<String>,
    pub folder_hard: Option<String>,
    pub ramble_steady: Option<String>,
    pub ramble_nervy: Option<String>,
    pub ramble_hard: Option<String>,
    pub fan_steady: Option<String>,
    pub fan_nervy: Option<String>,
    pub fan_hard: Option<String>,
    pub stoic_steady: Option<String>,
    pub stoic_nervy: Option<String>,
    pub stoic_hard: Option<String>,
    pub con_steady: Option<String>,
    pub con_nervy: Option<String>,
    pub con_hard: Option<String>,
    pub game_steady: Option<String>,
    pub game_nervy: Option<String>,
    pub game_hard: Option<String>,
    pub menace_steady: Option<String>,
    pub menace_nervy: Option<String>,
    pub menace_hard: Option<String>,
}
impl INTELNOWLockdownPosts {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        INTELNOWLockdownPosts {
            steady: None,
            nervy: None,
            hard: None,
            under: None,
            pedant: None,
            selfown: None,
            folder: None,
            ramble: None,
            fan: None,
            stoic: None,
            con: None,
            game: None,
            menace: None,
            steady_none: None,
            steady_heard: None,
            steady_glimpsed: None,
            steady_saw: None,
            steady_saw_hurt: None,
            nervy_none: None,
            nervy_heard: None,
            nervy_glimpsed: None,
            nervy_saw: None,
            nervy_saw_hurt: None,
            hard_none: None,
            hard_heard: None,
            hard_glimpsed: None,
            hard_saw: None,
            hard_saw_hurt: None,
            __none: None,
            __heard: None,
            __glimpsed: None,
            __saw: None,
            __saw_hurt: None,
            pedant_steady: None,
            pedant_nervy: None,
            pedant_hard: None,
            selfown_steady: None,
            selfown_nervy: None,
            selfown_hard: None,
            folder_steady: None,
            folder_nervy: None,
            folder_hard: None,
            ramble_steady: None,
            ramble_nervy: None,
            ramble_hard: None,
            fan_steady: None,
            fan_nervy: None,
            fan_hard: None,
            stoic_steady: None,
            stoic_nervy: None,
            stoic_hard: None,
            con_steady: None,
            con_nervy: None,
            con_hard: None,
            game_steady: None,
            game_nervy: None,
            game_hard: None,
            menace_steady: None,
            menace_nervy: None,
            menace_hard: None,
        }
    }
}
impl Default for INTELNOWLockdownPosts { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at INTELNOW.routes, ../.npm-global/lib/node_modules/typescript/lib/lib.es5.d.ts:1583)
#[derive(Clone, Debug)]
pub struct INTELNOWRoutes {
    pub steady: Option<String>,
    pub nervy: Option<String>,
    pub hard: Option<String>,
    pub under: Option<String>,
    pub pedant: Option<String>,
    pub selfown: Option<String>,
    pub folder: Option<String>,
    pub ramble: Option<String>,
    pub fan: Option<String>,
    pub stoic: Option<String>,
    pub con: Option<String>,
    pub game: Option<String>,
    pub menace: Option<String>,
    pub steady_none: Option<String>,
    pub steady_heard: Option<String>,
    pub steady_glimpsed: Option<String>,
    pub steady_saw: Option<String>,
    pub steady_saw_hurt: Option<String>,
    pub nervy_none: Option<String>,
    pub nervy_heard: Option<String>,
    pub nervy_glimpsed: Option<String>,
    pub nervy_saw: Option<String>,
    pub nervy_saw_hurt: Option<String>,
    pub hard_none: Option<String>,
    pub hard_heard: Option<String>,
    pub hard_glimpsed: Option<String>,
    pub hard_saw: Option<String>,
    pub hard_saw_hurt: Option<String>,
    pub __none: Option<String>,
    pub __heard: Option<String>,
    pub __glimpsed: Option<String>,
    pub __saw: Option<String>,
    pub __saw_hurt: Option<String>,
    pub pedant_steady: Option<String>,
    pub pedant_nervy: Option<String>,
    pub pedant_hard: Option<String>,
    pub selfown_steady: Option<String>,
    pub selfown_nervy: Option<String>,
    pub selfown_hard: Option<String>,
    pub folder_steady: Option<String>,
    pub folder_nervy: Option<String>,
    pub folder_hard: Option<String>,
    pub ramble_steady: Option<String>,
    pub ramble_nervy: Option<String>,
    pub ramble_hard: Option<String>,
    pub fan_steady: Option<String>,
    pub fan_nervy: Option<String>,
    pub fan_hard: Option<String>,
    pub stoic_steady: Option<String>,
    pub stoic_nervy: Option<String>,
    pub stoic_hard: Option<String>,
    pub con_steady: Option<String>,
    pub con_nervy: Option<String>,
    pub con_hard: Option<String>,
    pub game_steady: Option<String>,
    pub game_nervy: Option<String>,
    pub game_hard: Option<String>,
    pub menace_steady: Option<String>,
    pub menace_nervy: Option<String>,
    pub menace_hard: Option<String>,
}
impl INTELNOWRoutes {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        INTELNOWRoutes {
            steady: None,
            nervy: None,
            hard: None,
            under: None,
            pedant: None,
            selfown: None,
            folder: None,
            ramble: None,
            fan: None,
            stoic: None,
            con: None,
            game: None,
            menace: None,
            steady_none: None,
            steady_heard: None,
            steady_glimpsed: None,
            steady_saw: None,
            steady_saw_hurt: None,
            nervy_none: None,
            nervy_heard: None,
            nervy_glimpsed: None,
            nervy_saw: None,
            nervy_saw_hurt: None,
            hard_none: None,
            hard_heard: None,
            hard_glimpsed: None,
            hard_saw: None,
            hard_saw_hurt: None,
            __none: None,
            __heard: None,
            __glimpsed: None,
            __saw: None,
            __saw_hurt: None,
            pedant_steady: None,
            pedant_nervy: None,
            pedant_hard: None,
            selfown_steady: None,
            selfown_nervy: None,
            selfown_hard: None,
            folder_steady: None,
            folder_nervy: None,
            folder_hard: None,
            ramble_steady: None,
            ramble_nervy: None,
            ramble_hard: None,
            fan_steady: None,
            fan_nervy: None,
            fan_hard: None,
            stoic_steady: None,
            stoic_nervy: None,
            stoic_hard: None,
            con_steady: None,
            con_nervy: None,
            con_hard: None,
            game_steady: None,
            game_nervy: None,
            game_hard: None,
            menace_steady: None,
            menace_nervy: None,
            menace_hard: None,
        }
    }
}
impl Default for INTELNOWRoutes { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at INTELNOW.missing, ../.npm-global/lib/node_modules/typescript/lib/lib.es5.d.ts:1583)
#[derive(Clone, Debug)]
pub struct INTELNOWMissing {
    pub steady: Option<String>,
    pub nervy: Option<String>,
    pub hard: Option<String>,
    pub under: Option<String>,
    pub pedant: Option<String>,
    pub selfown: Option<String>,
    pub folder: Option<String>,
    pub ramble: Option<String>,
    pub fan: Option<String>,
    pub stoic: Option<String>,
    pub con: Option<String>,
    pub game: Option<String>,
    pub menace: Option<String>,
    pub steady_none: Option<String>,
    pub steady_heard: Option<String>,
    pub steady_glimpsed: Option<String>,
    pub steady_saw: Option<String>,
    pub steady_saw_hurt: Option<String>,
    pub nervy_none: Option<String>,
    pub nervy_heard: Option<String>,
    pub nervy_glimpsed: Option<String>,
    pub nervy_saw: Option<String>,
    pub nervy_saw_hurt: Option<String>,
    pub hard_none: Option<String>,
    pub hard_heard: Option<String>,
    pub hard_glimpsed: Option<String>,
    pub hard_saw: Option<String>,
    pub hard_saw_hurt: Option<String>,
    pub __none: Option<String>,
    pub __heard: Option<String>,
    pub __glimpsed: Option<String>,
    pub __saw: Option<String>,
    pub __saw_hurt: Option<String>,
    pub pedant_steady: Option<String>,
    pub pedant_nervy: Option<String>,
    pub pedant_hard: Option<String>,
    pub selfown_steady: Option<String>,
    pub selfown_nervy: Option<String>,
    pub selfown_hard: Option<String>,
    pub folder_steady: Option<String>,
    pub folder_nervy: Option<String>,
    pub folder_hard: Option<String>,
    pub ramble_steady: Option<String>,
    pub ramble_nervy: Option<String>,
    pub ramble_hard: Option<String>,
    pub fan_steady: Option<String>,
    pub fan_nervy: Option<String>,
    pub fan_hard: Option<String>,
    pub stoic_steady: Option<String>,
    pub stoic_nervy: Option<String>,
    pub stoic_hard: Option<String>,
    pub con_steady: Option<String>,
    pub con_nervy: Option<String>,
    pub con_hard: Option<String>,
    pub game_steady: Option<String>,
    pub game_nervy: Option<String>,
    pub game_hard: Option<String>,
    pub menace_steady: Option<String>,
    pub menace_nervy: Option<String>,
    pub menace_hard: Option<String>,
}
impl INTELNOWMissing {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        INTELNOWMissing {
            steady: None,
            nervy: None,
            hard: None,
            under: None,
            pedant: None,
            selfown: None,
            folder: None,
            ramble: None,
            fan: None,
            stoic: None,
            con: None,
            game: None,
            menace: None,
            steady_none: None,
            steady_heard: None,
            steady_glimpsed: None,
            steady_saw: None,
            steady_saw_hurt: None,
            nervy_none: None,
            nervy_heard: None,
            nervy_glimpsed: None,
            nervy_saw: None,
            nervy_saw_hurt: None,
            hard_none: None,
            hard_heard: None,
            hard_glimpsed: None,
            hard_saw: None,
            hard_saw_hurt: None,
            __none: None,
            __heard: None,
            __glimpsed: None,
            __saw: None,
            __saw_hurt: None,
            pedant_steady: None,
            pedant_nervy: None,
            pedant_hard: None,
            selfown_steady: None,
            selfown_nervy: None,
            selfown_hard: None,
            folder_steady: None,
            folder_nervy: None,
            folder_hard: None,
            ramble_steady: None,
            ramble_nervy: None,
            ramble_hard: None,
            fan_steady: None,
            fan_nervy: None,
            fan_hard: None,
            stoic_steady: None,
            stoic_nervy: None,
            stoic_hard: None,
            con_steady: None,
            con_nervy: None,
            con_hard: None,
            game_steady: None,
            game_nervy: None,
            game_hard: None,
            menace_steady: None,
            menace_nervy: None,
            menace_hard: None,
        }
    }
}
impl Default for INTELNOWMissing { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at INTELNOW.drive, ../.npm-global/lib/node_modules/typescript/lib/lib.es5.d.ts:1583)
#[derive(Clone, Debug)]
pub struct INTELNOWDrive {
    pub steady: Option<String>,
    pub nervy: Option<String>,
    pub hard: Option<String>,
    pub under: Option<String>,
    pub pedant: Option<String>,
    pub selfown: Option<String>,
    pub folder: Option<String>,
    pub ramble: Option<String>,
    pub fan: Option<String>,
    pub stoic: Option<String>,
    pub con: Option<String>,
    pub game: Option<String>,
    pub menace: Option<String>,
    pub steady_none: Option<String>,
    pub steady_heard: Option<String>,
    pub steady_glimpsed: Option<String>,
    pub steady_saw: Option<String>,
    pub steady_saw_hurt: Option<String>,
    pub nervy_none: Option<String>,
    pub nervy_heard: Option<String>,
    pub nervy_glimpsed: Option<String>,
    pub nervy_saw: Option<String>,
    pub nervy_saw_hurt: Option<String>,
    pub hard_none: Option<String>,
    pub hard_heard: Option<String>,
    pub hard_glimpsed: Option<String>,
    pub hard_saw: Option<String>,
    pub hard_saw_hurt: Option<String>,
    pub __none: Option<String>,
    pub __heard: Option<String>,
    pub __glimpsed: Option<String>,
    pub __saw: Option<String>,
    pub __saw_hurt: Option<String>,
    pub pedant_steady: Option<String>,
    pub pedant_nervy: Option<String>,
    pub pedant_hard: Option<String>,
    pub selfown_steady: Option<String>,
    pub selfown_nervy: Option<String>,
    pub selfown_hard: Option<String>,
    pub folder_steady: Option<String>,
    pub folder_nervy: Option<String>,
    pub folder_hard: Option<String>,
    pub ramble_steady: Option<String>,
    pub ramble_nervy: Option<String>,
    pub ramble_hard: Option<String>,
    pub fan_steady: Option<String>,
    pub fan_nervy: Option<String>,
    pub fan_hard: Option<String>,
    pub stoic_steady: Option<String>,
    pub stoic_nervy: Option<String>,
    pub stoic_hard: Option<String>,
    pub con_steady: Option<String>,
    pub con_nervy: Option<String>,
    pub con_hard: Option<String>,
    pub game_steady: Option<String>,
    pub game_nervy: Option<String>,
    pub game_hard: Option<String>,
    pub menace_steady: Option<String>,
    pub menace_nervy: Option<String>,
    pub menace_hard: Option<String>,
}
impl INTELNOWDrive {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        INTELNOWDrive {
            steady: None,
            nervy: None,
            hard: None,
            under: None,
            pedant: None,
            selfown: None,
            folder: None,
            ramble: None,
            fan: None,
            stoic: None,
            con: None,
            game: None,
            menace: None,
            steady_none: None,
            steady_heard: None,
            steady_glimpsed: None,
            steady_saw: None,
            steady_saw_hurt: None,
            nervy_none: None,
            nervy_heard: None,
            nervy_glimpsed: None,
            nervy_saw: None,
            nervy_saw_hurt: None,
            hard_none: None,
            hard_heard: None,
            hard_glimpsed: None,
            hard_saw: None,
            hard_saw_hurt: None,
            __none: None,
            __heard: None,
            __glimpsed: None,
            __saw: None,
            __saw_hurt: None,
            pedant_steady: None,
            pedant_nervy: None,
            pedant_hard: None,
            selfown_steady: None,
            selfown_nervy: None,
            selfown_hard: None,
            folder_steady: None,
            folder_nervy: None,
            folder_hard: None,
            ramble_steady: None,
            ramble_nervy: None,
            ramble_hard: None,
            fan_steady: None,
            fan_nervy: None,
            fan_hard: None,
            stoic_steady: None,
            stoic_nervy: None,
            stoic_hard: None,
            con_steady: None,
            con_nervy: None,
            con_hard: None,
            game_steady: None,
            game_nervy: None,
            game_hard: None,
            menace_steady: None,
            menace_nervy: None,
            menace_hard: None,
        }
    }
}
impl Default for INTELNOWDrive { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/dialogue.ts:342 INTEL_GUIDE, ../.npm-global/lib/node_modules/typescript/lib/lib.es5.d.ts:1583)
#[derive(Clone, Debug)]
pub struct INTELGUIDE {
    pub check_on: Option<crate::dialogue::INTELGUIDECheckOn>,
    pub clearing: Option<crate::dialogue::INTELGUIDEClearing>,
    pub rollcall_live: Option<crate::dialogue::INTELGUIDERollcallLive>,
    pub blackout: Option<crate::dialogue::INTELGUIDEBlackout>,
    pub door: Option<crate::dialogue::INTELGUIDEDoor>,
    pub code: Option<crate::dialogue::INTELGUIDECode>,
    pub rollcall: Option<crate::dialogue::INTELGUIDERollcall>,
    pub lockdown_posts: Option<crate::dialogue::INTELGUIDELockdownPosts>,
    pub routes: Option<crate::dialogue::INTELGUIDERoutes>,
    pub missing: Option<crate::dialogue::INTELGUIDEMissing>,
    pub drive: Option<crate::dialogue::INTELGUIDEDrive>,
}
impl INTELGUIDE {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        INTELGUIDE {
            check_on: None,
            clearing: None,
            rollcall_live: None,
            blackout: None,
            door: None,
            code: None,
            rollcall: None,
            lockdown_posts: None,
            routes: None,
            missing: None,
            drive: None,
        }
    }
}
impl Default for INTELGUIDE { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at INTELGUIDE.checkOn, ../.npm-global/lib/node_modules/typescript/lib/lib.es5.d.ts:1583)
#[derive(Clone, Debug)]
pub struct INTELGUIDECheckOn {
    pub steady: Option<String>,
    pub nervy: Option<String>,
    pub hard: Option<String>,
    pub under: Option<String>,
    pub pedant: Option<String>,
    pub selfown: Option<String>,
    pub folder: Option<String>,
    pub ramble: Option<String>,
    pub fan: Option<String>,
    pub stoic: Option<String>,
    pub con: Option<String>,
    pub game: Option<String>,
    pub menace: Option<String>,
    pub steady_none: Option<String>,
    pub steady_heard: Option<String>,
    pub steady_glimpsed: Option<String>,
    pub steady_saw: Option<String>,
    pub steady_saw_hurt: Option<String>,
    pub nervy_none: Option<String>,
    pub nervy_heard: Option<String>,
    pub nervy_glimpsed: Option<String>,
    pub nervy_saw: Option<String>,
    pub nervy_saw_hurt: Option<String>,
    pub hard_none: Option<String>,
    pub hard_heard: Option<String>,
    pub hard_glimpsed: Option<String>,
    pub hard_saw: Option<String>,
    pub hard_saw_hurt: Option<String>,
    pub __none: Option<String>,
    pub __heard: Option<String>,
    pub __glimpsed: Option<String>,
    pub __saw: Option<String>,
    pub __saw_hurt: Option<String>,
    pub pedant_steady: Option<String>,
    pub pedant_nervy: Option<String>,
    pub pedant_hard: Option<String>,
    pub selfown_steady: Option<String>,
    pub selfown_nervy: Option<String>,
    pub selfown_hard: Option<String>,
    pub folder_steady: Option<String>,
    pub folder_nervy: Option<String>,
    pub folder_hard: Option<String>,
    pub ramble_steady: Option<String>,
    pub ramble_nervy: Option<String>,
    pub ramble_hard: Option<String>,
    pub fan_steady: Option<String>,
    pub fan_nervy: Option<String>,
    pub fan_hard: Option<String>,
    pub stoic_steady: Option<String>,
    pub stoic_nervy: Option<String>,
    pub stoic_hard: Option<String>,
    pub con_steady: Option<String>,
    pub con_nervy: Option<String>,
    pub con_hard: Option<String>,
    pub game_steady: Option<String>,
    pub game_nervy: Option<String>,
    pub game_hard: Option<String>,
    pub menace_steady: Option<String>,
    pub menace_nervy: Option<String>,
    pub menace_hard: Option<String>,
}
impl INTELGUIDECheckOn {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        INTELGUIDECheckOn {
            steady: None,
            nervy: None,
            hard: None,
            under: None,
            pedant: None,
            selfown: None,
            folder: None,
            ramble: None,
            fan: None,
            stoic: None,
            con: None,
            game: None,
            menace: None,
            steady_none: None,
            steady_heard: None,
            steady_glimpsed: None,
            steady_saw: None,
            steady_saw_hurt: None,
            nervy_none: None,
            nervy_heard: None,
            nervy_glimpsed: None,
            nervy_saw: None,
            nervy_saw_hurt: None,
            hard_none: None,
            hard_heard: None,
            hard_glimpsed: None,
            hard_saw: None,
            hard_saw_hurt: None,
            __none: None,
            __heard: None,
            __glimpsed: None,
            __saw: None,
            __saw_hurt: None,
            pedant_steady: None,
            pedant_nervy: None,
            pedant_hard: None,
            selfown_steady: None,
            selfown_nervy: None,
            selfown_hard: None,
            folder_steady: None,
            folder_nervy: None,
            folder_hard: None,
            ramble_steady: None,
            ramble_nervy: None,
            ramble_hard: None,
            fan_steady: None,
            fan_nervy: None,
            fan_hard: None,
            stoic_steady: None,
            stoic_nervy: None,
            stoic_hard: None,
            con_steady: None,
            con_nervy: None,
            con_hard: None,
            game_steady: None,
            game_nervy: None,
            game_hard: None,
            menace_steady: None,
            menace_nervy: None,
            menace_hard: None,
        }
    }
}
impl Default for INTELGUIDECheckOn { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at INTELGUIDE.clearing, ../.npm-global/lib/node_modules/typescript/lib/lib.es5.d.ts:1583)
#[derive(Clone, Debug)]
pub struct INTELGUIDEClearing {
    pub steady: Option<String>,
    pub nervy: Option<String>,
    pub hard: Option<String>,
    pub under: Option<String>,
    pub pedant: Option<String>,
    pub selfown: Option<String>,
    pub folder: Option<String>,
    pub ramble: Option<String>,
    pub fan: Option<String>,
    pub stoic: Option<String>,
    pub con: Option<String>,
    pub game: Option<String>,
    pub menace: Option<String>,
    pub steady_none: Option<String>,
    pub steady_heard: Option<String>,
    pub steady_glimpsed: Option<String>,
    pub steady_saw: Option<String>,
    pub steady_saw_hurt: Option<String>,
    pub nervy_none: Option<String>,
    pub nervy_heard: Option<String>,
    pub nervy_glimpsed: Option<String>,
    pub nervy_saw: Option<String>,
    pub nervy_saw_hurt: Option<String>,
    pub hard_none: Option<String>,
    pub hard_heard: Option<String>,
    pub hard_glimpsed: Option<String>,
    pub hard_saw: Option<String>,
    pub hard_saw_hurt: Option<String>,
    pub __none: Option<String>,
    pub __heard: Option<String>,
    pub __glimpsed: Option<String>,
    pub __saw: Option<String>,
    pub __saw_hurt: Option<String>,
    pub pedant_steady: Option<String>,
    pub pedant_nervy: Option<String>,
    pub pedant_hard: Option<String>,
    pub selfown_steady: Option<String>,
    pub selfown_nervy: Option<String>,
    pub selfown_hard: Option<String>,
    pub folder_steady: Option<String>,
    pub folder_nervy: Option<String>,
    pub folder_hard: Option<String>,
    pub ramble_steady: Option<String>,
    pub ramble_nervy: Option<String>,
    pub ramble_hard: Option<String>,
    pub fan_steady: Option<String>,
    pub fan_nervy: Option<String>,
    pub fan_hard: Option<String>,
    pub stoic_steady: Option<String>,
    pub stoic_nervy: Option<String>,
    pub stoic_hard: Option<String>,
    pub con_steady: Option<String>,
    pub con_nervy: Option<String>,
    pub con_hard: Option<String>,
    pub game_steady: Option<String>,
    pub game_nervy: Option<String>,
    pub game_hard: Option<String>,
    pub menace_steady: Option<String>,
    pub menace_nervy: Option<String>,
    pub menace_hard: Option<String>,
}
impl INTELGUIDEClearing {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        INTELGUIDEClearing {
            steady: None,
            nervy: None,
            hard: None,
            under: None,
            pedant: None,
            selfown: None,
            folder: None,
            ramble: None,
            fan: None,
            stoic: None,
            con: None,
            game: None,
            menace: None,
            steady_none: None,
            steady_heard: None,
            steady_glimpsed: None,
            steady_saw: None,
            steady_saw_hurt: None,
            nervy_none: None,
            nervy_heard: None,
            nervy_glimpsed: None,
            nervy_saw: None,
            nervy_saw_hurt: None,
            hard_none: None,
            hard_heard: None,
            hard_glimpsed: None,
            hard_saw: None,
            hard_saw_hurt: None,
            __none: None,
            __heard: None,
            __glimpsed: None,
            __saw: None,
            __saw_hurt: None,
            pedant_steady: None,
            pedant_nervy: None,
            pedant_hard: None,
            selfown_steady: None,
            selfown_nervy: None,
            selfown_hard: None,
            folder_steady: None,
            folder_nervy: None,
            folder_hard: None,
            ramble_steady: None,
            ramble_nervy: None,
            ramble_hard: None,
            fan_steady: None,
            fan_nervy: None,
            fan_hard: None,
            stoic_steady: None,
            stoic_nervy: None,
            stoic_hard: None,
            con_steady: None,
            con_nervy: None,
            con_hard: None,
            game_steady: None,
            game_nervy: None,
            game_hard: None,
            menace_steady: None,
            menace_nervy: None,
            menace_hard: None,
        }
    }
}
impl Default for INTELGUIDEClearing { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at INTELGUIDE.rollcallLive, ../.npm-global/lib/node_modules/typescript/lib/lib.es5.d.ts:1583)
#[derive(Clone, Debug)]
pub struct INTELGUIDERollcallLive {
    pub steady: Option<String>,
    pub nervy: Option<String>,
    pub hard: Option<String>,
    pub under: Option<String>,
    pub pedant: Option<String>,
    pub selfown: Option<String>,
    pub folder: Option<String>,
    pub ramble: Option<String>,
    pub fan: Option<String>,
    pub stoic: Option<String>,
    pub con: Option<String>,
    pub game: Option<String>,
    pub menace: Option<String>,
    pub steady_none: Option<String>,
    pub steady_heard: Option<String>,
    pub steady_glimpsed: Option<String>,
    pub steady_saw: Option<String>,
    pub steady_saw_hurt: Option<String>,
    pub nervy_none: Option<String>,
    pub nervy_heard: Option<String>,
    pub nervy_glimpsed: Option<String>,
    pub nervy_saw: Option<String>,
    pub nervy_saw_hurt: Option<String>,
    pub hard_none: Option<String>,
    pub hard_heard: Option<String>,
    pub hard_glimpsed: Option<String>,
    pub hard_saw: Option<String>,
    pub hard_saw_hurt: Option<String>,
    pub __none: Option<String>,
    pub __heard: Option<String>,
    pub __glimpsed: Option<String>,
    pub __saw: Option<String>,
    pub __saw_hurt: Option<String>,
    pub pedant_steady: Option<String>,
    pub pedant_nervy: Option<String>,
    pub pedant_hard: Option<String>,
    pub selfown_steady: Option<String>,
    pub selfown_nervy: Option<String>,
    pub selfown_hard: Option<String>,
    pub folder_steady: Option<String>,
    pub folder_nervy: Option<String>,
    pub folder_hard: Option<String>,
    pub ramble_steady: Option<String>,
    pub ramble_nervy: Option<String>,
    pub ramble_hard: Option<String>,
    pub fan_steady: Option<String>,
    pub fan_nervy: Option<String>,
    pub fan_hard: Option<String>,
    pub stoic_steady: Option<String>,
    pub stoic_nervy: Option<String>,
    pub stoic_hard: Option<String>,
    pub con_steady: Option<String>,
    pub con_nervy: Option<String>,
    pub con_hard: Option<String>,
    pub game_steady: Option<String>,
    pub game_nervy: Option<String>,
    pub game_hard: Option<String>,
    pub menace_steady: Option<String>,
    pub menace_nervy: Option<String>,
    pub menace_hard: Option<String>,
}
impl INTELGUIDERollcallLive {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        INTELGUIDERollcallLive {
            steady: None,
            nervy: None,
            hard: None,
            under: None,
            pedant: None,
            selfown: None,
            folder: None,
            ramble: None,
            fan: None,
            stoic: None,
            con: None,
            game: None,
            menace: None,
            steady_none: None,
            steady_heard: None,
            steady_glimpsed: None,
            steady_saw: None,
            steady_saw_hurt: None,
            nervy_none: None,
            nervy_heard: None,
            nervy_glimpsed: None,
            nervy_saw: None,
            nervy_saw_hurt: None,
            hard_none: None,
            hard_heard: None,
            hard_glimpsed: None,
            hard_saw: None,
            hard_saw_hurt: None,
            __none: None,
            __heard: None,
            __glimpsed: None,
            __saw: None,
            __saw_hurt: None,
            pedant_steady: None,
            pedant_nervy: None,
            pedant_hard: None,
            selfown_steady: None,
            selfown_nervy: None,
            selfown_hard: None,
            folder_steady: None,
            folder_nervy: None,
            folder_hard: None,
            ramble_steady: None,
            ramble_nervy: None,
            ramble_hard: None,
            fan_steady: None,
            fan_nervy: None,
            fan_hard: None,
            stoic_steady: None,
            stoic_nervy: None,
            stoic_hard: None,
            con_steady: None,
            con_nervy: None,
            con_hard: None,
            game_steady: None,
            game_nervy: None,
            game_hard: None,
            menace_steady: None,
            menace_nervy: None,
            menace_hard: None,
        }
    }
}
impl Default for INTELGUIDERollcallLive { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at INTELGUIDE.blackout, ../.npm-global/lib/node_modules/typescript/lib/lib.es5.d.ts:1583)
#[derive(Clone, Debug)]
pub struct INTELGUIDEBlackout {
    pub steady: Option<String>,
    pub nervy: Option<String>,
    pub hard: Option<String>,
    pub under: Option<String>,
    pub pedant: Option<String>,
    pub selfown: Option<String>,
    pub folder: Option<String>,
    pub ramble: Option<String>,
    pub fan: Option<String>,
    pub stoic: Option<String>,
    pub con: Option<String>,
    pub game: Option<String>,
    pub menace: Option<String>,
    pub steady_none: Option<String>,
    pub steady_heard: Option<String>,
    pub steady_glimpsed: Option<String>,
    pub steady_saw: Option<String>,
    pub steady_saw_hurt: Option<String>,
    pub nervy_none: Option<String>,
    pub nervy_heard: Option<String>,
    pub nervy_glimpsed: Option<String>,
    pub nervy_saw: Option<String>,
    pub nervy_saw_hurt: Option<String>,
    pub hard_none: Option<String>,
    pub hard_heard: Option<String>,
    pub hard_glimpsed: Option<String>,
    pub hard_saw: Option<String>,
    pub hard_saw_hurt: Option<String>,
    pub __none: Option<String>,
    pub __heard: Option<String>,
    pub __glimpsed: Option<String>,
    pub __saw: Option<String>,
    pub __saw_hurt: Option<String>,
    pub pedant_steady: Option<String>,
    pub pedant_nervy: Option<String>,
    pub pedant_hard: Option<String>,
    pub selfown_steady: Option<String>,
    pub selfown_nervy: Option<String>,
    pub selfown_hard: Option<String>,
    pub folder_steady: Option<String>,
    pub folder_nervy: Option<String>,
    pub folder_hard: Option<String>,
    pub ramble_steady: Option<String>,
    pub ramble_nervy: Option<String>,
    pub ramble_hard: Option<String>,
    pub fan_steady: Option<String>,
    pub fan_nervy: Option<String>,
    pub fan_hard: Option<String>,
    pub stoic_steady: Option<String>,
    pub stoic_nervy: Option<String>,
    pub stoic_hard: Option<String>,
    pub con_steady: Option<String>,
    pub con_nervy: Option<String>,
    pub con_hard: Option<String>,
    pub game_steady: Option<String>,
    pub game_nervy: Option<String>,
    pub game_hard: Option<String>,
    pub menace_steady: Option<String>,
    pub menace_nervy: Option<String>,
    pub menace_hard: Option<String>,
}
impl INTELGUIDEBlackout {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        INTELGUIDEBlackout {
            steady: None,
            nervy: None,
            hard: None,
            under: None,
            pedant: None,
            selfown: None,
            folder: None,
            ramble: None,
            fan: None,
            stoic: None,
            con: None,
            game: None,
            menace: None,
            steady_none: None,
            steady_heard: None,
            steady_glimpsed: None,
            steady_saw: None,
            steady_saw_hurt: None,
            nervy_none: None,
            nervy_heard: None,
            nervy_glimpsed: None,
            nervy_saw: None,
            nervy_saw_hurt: None,
            hard_none: None,
            hard_heard: None,
            hard_glimpsed: None,
            hard_saw: None,
            hard_saw_hurt: None,
            __none: None,
            __heard: None,
            __glimpsed: None,
            __saw: None,
            __saw_hurt: None,
            pedant_steady: None,
            pedant_nervy: None,
            pedant_hard: None,
            selfown_steady: None,
            selfown_nervy: None,
            selfown_hard: None,
            folder_steady: None,
            folder_nervy: None,
            folder_hard: None,
            ramble_steady: None,
            ramble_nervy: None,
            ramble_hard: None,
            fan_steady: None,
            fan_nervy: None,
            fan_hard: None,
            stoic_steady: None,
            stoic_nervy: None,
            stoic_hard: None,
            con_steady: None,
            con_nervy: None,
            con_hard: None,
            game_steady: None,
            game_nervy: None,
            game_hard: None,
            menace_steady: None,
            menace_nervy: None,
            menace_hard: None,
        }
    }
}
impl Default for INTELGUIDEBlackout { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at INTELGUIDE.door, ../.npm-global/lib/node_modules/typescript/lib/lib.es5.d.ts:1583)
#[derive(Clone, Debug)]
pub struct INTELGUIDEDoor {
    pub steady: Option<String>,
    pub nervy: Option<String>,
    pub hard: Option<String>,
    pub under: Option<String>,
    pub pedant: Option<String>,
    pub selfown: Option<String>,
    pub folder: Option<String>,
    pub ramble: Option<String>,
    pub fan: Option<String>,
    pub stoic: Option<String>,
    pub con: Option<String>,
    pub game: Option<String>,
    pub menace: Option<String>,
    pub steady_none: Option<String>,
    pub steady_heard: Option<String>,
    pub steady_glimpsed: Option<String>,
    pub steady_saw: Option<String>,
    pub steady_saw_hurt: Option<String>,
    pub nervy_none: Option<String>,
    pub nervy_heard: Option<String>,
    pub nervy_glimpsed: Option<String>,
    pub nervy_saw: Option<String>,
    pub nervy_saw_hurt: Option<String>,
    pub hard_none: Option<String>,
    pub hard_heard: Option<String>,
    pub hard_glimpsed: Option<String>,
    pub hard_saw: Option<String>,
    pub hard_saw_hurt: Option<String>,
    pub __none: Option<String>,
    pub __heard: Option<String>,
    pub __glimpsed: Option<String>,
    pub __saw: Option<String>,
    pub __saw_hurt: Option<String>,
    pub pedant_steady: Option<String>,
    pub pedant_nervy: Option<String>,
    pub pedant_hard: Option<String>,
    pub selfown_steady: Option<String>,
    pub selfown_nervy: Option<String>,
    pub selfown_hard: Option<String>,
    pub folder_steady: Option<String>,
    pub folder_nervy: Option<String>,
    pub folder_hard: Option<String>,
    pub ramble_steady: Option<String>,
    pub ramble_nervy: Option<String>,
    pub ramble_hard: Option<String>,
    pub fan_steady: Option<String>,
    pub fan_nervy: Option<String>,
    pub fan_hard: Option<String>,
    pub stoic_steady: Option<String>,
    pub stoic_nervy: Option<String>,
    pub stoic_hard: Option<String>,
    pub con_steady: Option<String>,
    pub con_nervy: Option<String>,
    pub con_hard: Option<String>,
    pub game_steady: Option<String>,
    pub game_nervy: Option<String>,
    pub game_hard: Option<String>,
    pub menace_steady: Option<String>,
    pub menace_nervy: Option<String>,
    pub menace_hard: Option<String>,
}
impl INTELGUIDEDoor {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        INTELGUIDEDoor {
            steady: None,
            nervy: None,
            hard: None,
            under: None,
            pedant: None,
            selfown: None,
            folder: None,
            ramble: None,
            fan: None,
            stoic: None,
            con: None,
            game: None,
            menace: None,
            steady_none: None,
            steady_heard: None,
            steady_glimpsed: None,
            steady_saw: None,
            steady_saw_hurt: None,
            nervy_none: None,
            nervy_heard: None,
            nervy_glimpsed: None,
            nervy_saw: None,
            nervy_saw_hurt: None,
            hard_none: None,
            hard_heard: None,
            hard_glimpsed: None,
            hard_saw: None,
            hard_saw_hurt: None,
            __none: None,
            __heard: None,
            __glimpsed: None,
            __saw: None,
            __saw_hurt: None,
            pedant_steady: None,
            pedant_nervy: None,
            pedant_hard: None,
            selfown_steady: None,
            selfown_nervy: None,
            selfown_hard: None,
            folder_steady: None,
            folder_nervy: None,
            folder_hard: None,
            ramble_steady: None,
            ramble_nervy: None,
            ramble_hard: None,
            fan_steady: None,
            fan_nervy: None,
            fan_hard: None,
            stoic_steady: None,
            stoic_nervy: None,
            stoic_hard: None,
            con_steady: None,
            con_nervy: None,
            con_hard: None,
            game_steady: None,
            game_nervy: None,
            game_hard: None,
            menace_steady: None,
            menace_nervy: None,
            menace_hard: None,
        }
    }
}
impl Default for INTELGUIDEDoor { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at INTELGUIDE.code, ../.npm-global/lib/node_modules/typescript/lib/lib.es5.d.ts:1583)
#[derive(Clone, Debug)]
pub struct INTELGUIDECode {
    pub steady: Option<String>,
    pub nervy: Option<String>,
    pub hard: Option<String>,
    pub under: Option<String>,
    pub pedant: Option<String>,
    pub selfown: Option<String>,
    pub folder: Option<String>,
    pub ramble: Option<String>,
    pub fan: Option<String>,
    pub stoic: Option<String>,
    pub con: Option<String>,
    pub game: Option<String>,
    pub menace: Option<String>,
    pub steady_none: Option<String>,
    pub steady_heard: Option<String>,
    pub steady_glimpsed: Option<String>,
    pub steady_saw: Option<String>,
    pub steady_saw_hurt: Option<String>,
    pub nervy_none: Option<String>,
    pub nervy_heard: Option<String>,
    pub nervy_glimpsed: Option<String>,
    pub nervy_saw: Option<String>,
    pub nervy_saw_hurt: Option<String>,
    pub hard_none: Option<String>,
    pub hard_heard: Option<String>,
    pub hard_glimpsed: Option<String>,
    pub hard_saw: Option<String>,
    pub hard_saw_hurt: Option<String>,
    pub __none: Option<String>,
    pub __heard: Option<String>,
    pub __glimpsed: Option<String>,
    pub __saw: Option<String>,
    pub __saw_hurt: Option<String>,
    pub pedant_steady: Option<String>,
    pub pedant_nervy: Option<String>,
    pub pedant_hard: Option<String>,
    pub selfown_steady: Option<String>,
    pub selfown_nervy: Option<String>,
    pub selfown_hard: Option<String>,
    pub folder_steady: Option<String>,
    pub folder_nervy: Option<String>,
    pub folder_hard: Option<String>,
    pub ramble_steady: Option<String>,
    pub ramble_nervy: Option<String>,
    pub ramble_hard: Option<String>,
    pub fan_steady: Option<String>,
    pub fan_nervy: Option<String>,
    pub fan_hard: Option<String>,
    pub stoic_steady: Option<String>,
    pub stoic_nervy: Option<String>,
    pub stoic_hard: Option<String>,
    pub con_steady: Option<String>,
    pub con_nervy: Option<String>,
    pub con_hard: Option<String>,
    pub game_steady: Option<String>,
    pub game_nervy: Option<String>,
    pub game_hard: Option<String>,
    pub menace_steady: Option<String>,
    pub menace_nervy: Option<String>,
    pub menace_hard: Option<String>,
}
impl INTELGUIDECode {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        INTELGUIDECode {
            steady: None,
            nervy: None,
            hard: None,
            under: None,
            pedant: None,
            selfown: None,
            folder: None,
            ramble: None,
            fan: None,
            stoic: None,
            con: None,
            game: None,
            menace: None,
            steady_none: None,
            steady_heard: None,
            steady_glimpsed: None,
            steady_saw: None,
            steady_saw_hurt: None,
            nervy_none: None,
            nervy_heard: None,
            nervy_glimpsed: None,
            nervy_saw: None,
            nervy_saw_hurt: None,
            hard_none: None,
            hard_heard: None,
            hard_glimpsed: None,
            hard_saw: None,
            hard_saw_hurt: None,
            __none: None,
            __heard: None,
            __glimpsed: None,
            __saw: None,
            __saw_hurt: None,
            pedant_steady: None,
            pedant_nervy: None,
            pedant_hard: None,
            selfown_steady: None,
            selfown_nervy: None,
            selfown_hard: None,
            folder_steady: None,
            folder_nervy: None,
            folder_hard: None,
            ramble_steady: None,
            ramble_nervy: None,
            ramble_hard: None,
            fan_steady: None,
            fan_nervy: None,
            fan_hard: None,
            stoic_steady: None,
            stoic_nervy: None,
            stoic_hard: None,
            con_steady: None,
            con_nervy: None,
            con_hard: None,
            game_steady: None,
            game_nervy: None,
            game_hard: None,
            menace_steady: None,
            menace_nervy: None,
            menace_hard: None,
        }
    }
}
impl Default for INTELGUIDECode { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at INTELGUIDE.rollcall, ../.npm-global/lib/node_modules/typescript/lib/lib.es5.d.ts:1583)
#[derive(Clone, Debug)]
pub struct INTELGUIDERollcall {
    pub steady: Option<String>,
    pub nervy: Option<String>,
    pub hard: Option<String>,
    pub under: Option<String>,
    pub pedant: Option<String>,
    pub selfown: Option<String>,
    pub folder: Option<String>,
    pub ramble: Option<String>,
    pub fan: Option<String>,
    pub stoic: Option<String>,
    pub con: Option<String>,
    pub game: Option<String>,
    pub menace: Option<String>,
    pub steady_none: Option<String>,
    pub steady_heard: Option<String>,
    pub steady_glimpsed: Option<String>,
    pub steady_saw: Option<String>,
    pub steady_saw_hurt: Option<String>,
    pub nervy_none: Option<String>,
    pub nervy_heard: Option<String>,
    pub nervy_glimpsed: Option<String>,
    pub nervy_saw: Option<String>,
    pub nervy_saw_hurt: Option<String>,
    pub hard_none: Option<String>,
    pub hard_heard: Option<String>,
    pub hard_glimpsed: Option<String>,
    pub hard_saw: Option<String>,
    pub hard_saw_hurt: Option<String>,
    pub __none: Option<String>,
    pub __heard: Option<String>,
    pub __glimpsed: Option<String>,
    pub __saw: Option<String>,
    pub __saw_hurt: Option<String>,
    pub pedant_steady: Option<String>,
    pub pedant_nervy: Option<String>,
    pub pedant_hard: Option<String>,
    pub selfown_steady: Option<String>,
    pub selfown_nervy: Option<String>,
    pub selfown_hard: Option<String>,
    pub folder_steady: Option<String>,
    pub folder_nervy: Option<String>,
    pub folder_hard: Option<String>,
    pub ramble_steady: Option<String>,
    pub ramble_nervy: Option<String>,
    pub ramble_hard: Option<String>,
    pub fan_steady: Option<String>,
    pub fan_nervy: Option<String>,
    pub fan_hard: Option<String>,
    pub stoic_steady: Option<String>,
    pub stoic_nervy: Option<String>,
    pub stoic_hard: Option<String>,
    pub con_steady: Option<String>,
    pub con_nervy: Option<String>,
    pub con_hard: Option<String>,
    pub game_steady: Option<String>,
    pub game_nervy: Option<String>,
    pub game_hard: Option<String>,
    pub menace_steady: Option<String>,
    pub menace_nervy: Option<String>,
    pub menace_hard: Option<String>,
}
impl INTELGUIDERollcall {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        INTELGUIDERollcall {
            steady: None,
            nervy: None,
            hard: None,
            under: None,
            pedant: None,
            selfown: None,
            folder: None,
            ramble: None,
            fan: None,
            stoic: None,
            con: None,
            game: None,
            menace: None,
            steady_none: None,
            steady_heard: None,
            steady_glimpsed: None,
            steady_saw: None,
            steady_saw_hurt: None,
            nervy_none: None,
            nervy_heard: None,
            nervy_glimpsed: None,
            nervy_saw: None,
            nervy_saw_hurt: None,
            hard_none: None,
            hard_heard: None,
            hard_glimpsed: None,
            hard_saw: None,
            hard_saw_hurt: None,
            __none: None,
            __heard: None,
            __glimpsed: None,
            __saw: None,
            __saw_hurt: None,
            pedant_steady: None,
            pedant_nervy: None,
            pedant_hard: None,
            selfown_steady: None,
            selfown_nervy: None,
            selfown_hard: None,
            folder_steady: None,
            folder_nervy: None,
            folder_hard: None,
            ramble_steady: None,
            ramble_nervy: None,
            ramble_hard: None,
            fan_steady: None,
            fan_nervy: None,
            fan_hard: None,
            stoic_steady: None,
            stoic_nervy: None,
            stoic_hard: None,
            con_steady: None,
            con_nervy: None,
            con_hard: None,
            game_steady: None,
            game_nervy: None,
            game_hard: None,
            menace_steady: None,
            menace_nervy: None,
            menace_hard: None,
        }
    }
}
impl Default for INTELGUIDERollcall { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at INTELGUIDE.lockdownPosts, ../.npm-global/lib/node_modules/typescript/lib/lib.es5.d.ts:1583)
#[derive(Clone, Debug)]
pub struct INTELGUIDELockdownPosts {
    pub steady: Option<String>,
    pub nervy: Option<String>,
    pub hard: Option<String>,
    pub under: Option<String>,
    pub pedant: Option<String>,
    pub selfown: Option<String>,
    pub folder: Option<String>,
    pub ramble: Option<String>,
    pub fan: Option<String>,
    pub stoic: Option<String>,
    pub con: Option<String>,
    pub game: Option<String>,
    pub menace: Option<String>,
    pub steady_none: Option<String>,
    pub steady_heard: Option<String>,
    pub steady_glimpsed: Option<String>,
    pub steady_saw: Option<String>,
    pub steady_saw_hurt: Option<String>,
    pub nervy_none: Option<String>,
    pub nervy_heard: Option<String>,
    pub nervy_glimpsed: Option<String>,
    pub nervy_saw: Option<String>,
    pub nervy_saw_hurt: Option<String>,
    pub hard_none: Option<String>,
    pub hard_heard: Option<String>,
    pub hard_glimpsed: Option<String>,
    pub hard_saw: Option<String>,
    pub hard_saw_hurt: Option<String>,
    pub __none: Option<String>,
    pub __heard: Option<String>,
    pub __glimpsed: Option<String>,
    pub __saw: Option<String>,
    pub __saw_hurt: Option<String>,
    pub pedant_steady: Option<String>,
    pub pedant_nervy: Option<String>,
    pub pedant_hard: Option<String>,
    pub selfown_steady: Option<String>,
    pub selfown_nervy: Option<String>,
    pub selfown_hard: Option<String>,
    pub folder_steady: Option<String>,
    pub folder_nervy: Option<String>,
    pub folder_hard: Option<String>,
    pub ramble_steady: Option<String>,
    pub ramble_nervy: Option<String>,
    pub ramble_hard: Option<String>,
    pub fan_steady: Option<String>,
    pub fan_nervy: Option<String>,
    pub fan_hard: Option<String>,
    pub stoic_steady: Option<String>,
    pub stoic_nervy: Option<String>,
    pub stoic_hard: Option<String>,
    pub con_steady: Option<String>,
    pub con_nervy: Option<String>,
    pub con_hard: Option<String>,
    pub game_steady: Option<String>,
    pub game_nervy: Option<String>,
    pub game_hard: Option<String>,
    pub menace_steady: Option<String>,
    pub menace_nervy: Option<String>,
    pub menace_hard: Option<String>,
}
impl INTELGUIDELockdownPosts {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        INTELGUIDELockdownPosts {
            steady: None,
            nervy: None,
            hard: None,
            under: None,
            pedant: None,
            selfown: None,
            folder: None,
            ramble: None,
            fan: None,
            stoic: None,
            con: None,
            game: None,
            menace: None,
            steady_none: None,
            steady_heard: None,
            steady_glimpsed: None,
            steady_saw: None,
            steady_saw_hurt: None,
            nervy_none: None,
            nervy_heard: None,
            nervy_glimpsed: None,
            nervy_saw: None,
            nervy_saw_hurt: None,
            hard_none: None,
            hard_heard: None,
            hard_glimpsed: None,
            hard_saw: None,
            hard_saw_hurt: None,
            __none: None,
            __heard: None,
            __glimpsed: None,
            __saw: None,
            __saw_hurt: None,
            pedant_steady: None,
            pedant_nervy: None,
            pedant_hard: None,
            selfown_steady: None,
            selfown_nervy: None,
            selfown_hard: None,
            folder_steady: None,
            folder_nervy: None,
            folder_hard: None,
            ramble_steady: None,
            ramble_nervy: None,
            ramble_hard: None,
            fan_steady: None,
            fan_nervy: None,
            fan_hard: None,
            stoic_steady: None,
            stoic_nervy: None,
            stoic_hard: None,
            con_steady: None,
            con_nervy: None,
            con_hard: None,
            game_steady: None,
            game_nervy: None,
            game_hard: None,
            menace_steady: None,
            menace_nervy: None,
            menace_hard: None,
        }
    }
}
impl Default for INTELGUIDELockdownPosts { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at INTELGUIDE.routes, ../.npm-global/lib/node_modules/typescript/lib/lib.es5.d.ts:1583)
#[derive(Clone, Debug)]
pub struct INTELGUIDERoutes {
    pub steady: Option<String>,
    pub nervy: Option<String>,
    pub hard: Option<String>,
    pub under: Option<String>,
    pub pedant: Option<String>,
    pub selfown: Option<String>,
    pub folder: Option<String>,
    pub ramble: Option<String>,
    pub fan: Option<String>,
    pub stoic: Option<String>,
    pub con: Option<String>,
    pub game: Option<String>,
    pub menace: Option<String>,
    pub steady_none: Option<String>,
    pub steady_heard: Option<String>,
    pub steady_glimpsed: Option<String>,
    pub steady_saw: Option<String>,
    pub steady_saw_hurt: Option<String>,
    pub nervy_none: Option<String>,
    pub nervy_heard: Option<String>,
    pub nervy_glimpsed: Option<String>,
    pub nervy_saw: Option<String>,
    pub nervy_saw_hurt: Option<String>,
    pub hard_none: Option<String>,
    pub hard_heard: Option<String>,
    pub hard_glimpsed: Option<String>,
    pub hard_saw: Option<String>,
    pub hard_saw_hurt: Option<String>,
    pub __none: Option<String>,
    pub __heard: Option<String>,
    pub __glimpsed: Option<String>,
    pub __saw: Option<String>,
    pub __saw_hurt: Option<String>,
    pub pedant_steady: Option<String>,
    pub pedant_nervy: Option<String>,
    pub pedant_hard: Option<String>,
    pub selfown_steady: Option<String>,
    pub selfown_nervy: Option<String>,
    pub selfown_hard: Option<String>,
    pub folder_steady: Option<String>,
    pub folder_nervy: Option<String>,
    pub folder_hard: Option<String>,
    pub ramble_steady: Option<String>,
    pub ramble_nervy: Option<String>,
    pub ramble_hard: Option<String>,
    pub fan_steady: Option<String>,
    pub fan_nervy: Option<String>,
    pub fan_hard: Option<String>,
    pub stoic_steady: Option<String>,
    pub stoic_nervy: Option<String>,
    pub stoic_hard: Option<String>,
    pub con_steady: Option<String>,
    pub con_nervy: Option<String>,
    pub con_hard: Option<String>,
    pub game_steady: Option<String>,
    pub game_nervy: Option<String>,
    pub game_hard: Option<String>,
    pub menace_steady: Option<String>,
    pub menace_nervy: Option<String>,
    pub menace_hard: Option<String>,
}
impl INTELGUIDERoutes {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        INTELGUIDERoutes {
            steady: None,
            nervy: None,
            hard: None,
            under: None,
            pedant: None,
            selfown: None,
            folder: None,
            ramble: None,
            fan: None,
            stoic: None,
            con: None,
            game: None,
            menace: None,
            steady_none: None,
            steady_heard: None,
            steady_glimpsed: None,
            steady_saw: None,
            steady_saw_hurt: None,
            nervy_none: None,
            nervy_heard: None,
            nervy_glimpsed: None,
            nervy_saw: None,
            nervy_saw_hurt: None,
            hard_none: None,
            hard_heard: None,
            hard_glimpsed: None,
            hard_saw: None,
            hard_saw_hurt: None,
            __none: None,
            __heard: None,
            __glimpsed: None,
            __saw: None,
            __saw_hurt: None,
            pedant_steady: None,
            pedant_nervy: None,
            pedant_hard: None,
            selfown_steady: None,
            selfown_nervy: None,
            selfown_hard: None,
            folder_steady: None,
            folder_nervy: None,
            folder_hard: None,
            ramble_steady: None,
            ramble_nervy: None,
            ramble_hard: None,
            fan_steady: None,
            fan_nervy: None,
            fan_hard: None,
            stoic_steady: None,
            stoic_nervy: None,
            stoic_hard: None,
            con_steady: None,
            con_nervy: None,
            con_hard: None,
            game_steady: None,
            game_nervy: None,
            game_hard: None,
            menace_steady: None,
            menace_nervy: None,
            menace_hard: None,
        }
    }
}
impl Default for INTELGUIDERoutes { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at INTELGUIDE.missing, ../.npm-global/lib/node_modules/typescript/lib/lib.es5.d.ts:1583)
#[derive(Clone, Debug)]
pub struct INTELGUIDEMissing {
    pub steady: Option<String>,
    pub nervy: Option<String>,
    pub hard: Option<String>,
    pub under: Option<String>,
    pub pedant: Option<String>,
    pub selfown: Option<String>,
    pub folder: Option<String>,
    pub ramble: Option<String>,
    pub fan: Option<String>,
    pub stoic: Option<String>,
    pub con: Option<String>,
    pub game: Option<String>,
    pub menace: Option<String>,
    pub steady_none: Option<String>,
    pub steady_heard: Option<String>,
    pub steady_glimpsed: Option<String>,
    pub steady_saw: Option<String>,
    pub steady_saw_hurt: Option<String>,
    pub nervy_none: Option<String>,
    pub nervy_heard: Option<String>,
    pub nervy_glimpsed: Option<String>,
    pub nervy_saw: Option<String>,
    pub nervy_saw_hurt: Option<String>,
    pub hard_none: Option<String>,
    pub hard_heard: Option<String>,
    pub hard_glimpsed: Option<String>,
    pub hard_saw: Option<String>,
    pub hard_saw_hurt: Option<String>,
    pub __none: Option<String>,
    pub __heard: Option<String>,
    pub __glimpsed: Option<String>,
    pub __saw: Option<String>,
    pub __saw_hurt: Option<String>,
    pub pedant_steady: Option<String>,
    pub pedant_nervy: Option<String>,
    pub pedant_hard: Option<String>,
    pub selfown_steady: Option<String>,
    pub selfown_nervy: Option<String>,
    pub selfown_hard: Option<String>,
    pub folder_steady: Option<String>,
    pub folder_nervy: Option<String>,
    pub folder_hard: Option<String>,
    pub ramble_steady: Option<String>,
    pub ramble_nervy: Option<String>,
    pub ramble_hard: Option<String>,
    pub fan_steady: Option<String>,
    pub fan_nervy: Option<String>,
    pub fan_hard: Option<String>,
    pub stoic_steady: Option<String>,
    pub stoic_nervy: Option<String>,
    pub stoic_hard: Option<String>,
    pub con_steady: Option<String>,
    pub con_nervy: Option<String>,
    pub con_hard: Option<String>,
    pub game_steady: Option<String>,
    pub game_nervy: Option<String>,
    pub game_hard: Option<String>,
    pub menace_steady: Option<String>,
    pub menace_nervy: Option<String>,
    pub menace_hard: Option<String>,
}
impl INTELGUIDEMissing {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        INTELGUIDEMissing {
            steady: None,
            nervy: None,
            hard: None,
            under: None,
            pedant: None,
            selfown: None,
            folder: None,
            ramble: None,
            fan: None,
            stoic: None,
            con: None,
            game: None,
            menace: None,
            steady_none: None,
            steady_heard: None,
            steady_glimpsed: None,
            steady_saw: None,
            steady_saw_hurt: None,
            nervy_none: None,
            nervy_heard: None,
            nervy_glimpsed: None,
            nervy_saw: None,
            nervy_saw_hurt: None,
            hard_none: None,
            hard_heard: None,
            hard_glimpsed: None,
            hard_saw: None,
            hard_saw_hurt: None,
            __none: None,
            __heard: None,
            __glimpsed: None,
            __saw: None,
            __saw_hurt: None,
            pedant_steady: None,
            pedant_nervy: None,
            pedant_hard: None,
            selfown_steady: None,
            selfown_nervy: None,
            selfown_hard: None,
            folder_steady: None,
            folder_nervy: None,
            folder_hard: None,
            ramble_steady: None,
            ramble_nervy: None,
            ramble_hard: None,
            fan_steady: None,
            fan_nervy: None,
            fan_hard: None,
            stoic_steady: None,
            stoic_nervy: None,
            stoic_hard: None,
            con_steady: None,
            con_nervy: None,
            con_hard: None,
            game_steady: None,
            game_nervy: None,
            game_hard: None,
            menace_steady: None,
            menace_nervy: None,
            menace_hard: None,
        }
    }
}
impl Default for INTELGUIDEMissing { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at INTELGUIDE.drive, ../.npm-global/lib/node_modules/typescript/lib/lib.es5.d.ts:1583)
#[derive(Clone, Debug)]
pub struct INTELGUIDEDrive {
    pub steady: Option<String>,
    pub nervy: Option<String>,
    pub hard: Option<String>,
    pub under: Option<String>,
    pub pedant: Option<String>,
    pub selfown: Option<String>,
    pub folder: Option<String>,
    pub ramble: Option<String>,
    pub fan: Option<String>,
    pub stoic: Option<String>,
    pub con: Option<String>,
    pub game: Option<String>,
    pub menace: Option<String>,
    pub steady_none: Option<String>,
    pub steady_heard: Option<String>,
    pub steady_glimpsed: Option<String>,
    pub steady_saw: Option<String>,
    pub steady_saw_hurt: Option<String>,
    pub nervy_none: Option<String>,
    pub nervy_heard: Option<String>,
    pub nervy_glimpsed: Option<String>,
    pub nervy_saw: Option<String>,
    pub nervy_saw_hurt: Option<String>,
    pub hard_none: Option<String>,
    pub hard_heard: Option<String>,
    pub hard_glimpsed: Option<String>,
    pub hard_saw: Option<String>,
    pub hard_saw_hurt: Option<String>,
    pub __none: Option<String>,
    pub __heard: Option<String>,
    pub __glimpsed: Option<String>,
    pub __saw: Option<String>,
    pub __saw_hurt: Option<String>,
    pub pedant_steady: Option<String>,
    pub pedant_nervy: Option<String>,
    pub pedant_hard: Option<String>,
    pub selfown_steady: Option<String>,
    pub selfown_nervy: Option<String>,
    pub selfown_hard: Option<String>,
    pub folder_steady: Option<String>,
    pub folder_nervy: Option<String>,
    pub folder_hard: Option<String>,
    pub ramble_steady: Option<String>,
    pub ramble_nervy: Option<String>,
    pub ramble_hard: Option<String>,
    pub fan_steady: Option<String>,
    pub fan_nervy: Option<String>,
    pub fan_hard: Option<String>,
    pub stoic_steady: Option<String>,
    pub stoic_nervy: Option<String>,
    pub stoic_hard: Option<String>,
    pub con_steady: Option<String>,
    pub con_nervy: Option<String>,
    pub con_hard: Option<String>,
    pub game_steady: Option<String>,
    pub game_nervy: Option<String>,
    pub game_hard: Option<String>,
    pub menace_steady: Option<String>,
    pub menace_nervy: Option<String>,
    pub menace_hard: Option<String>,
}
impl INTELGUIDEDrive {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        INTELGUIDEDrive {
            steady: None,
            nervy: None,
            hard: None,
            under: None,
            pedant: None,
            selfown: None,
            folder: None,
            ramble: None,
            fan: None,
            stoic: None,
            con: None,
            game: None,
            menace: None,
            steady_none: None,
            steady_heard: None,
            steady_glimpsed: None,
            steady_saw: None,
            steady_saw_hurt: None,
            nervy_none: None,
            nervy_heard: None,
            nervy_glimpsed: None,
            nervy_saw: None,
            nervy_saw_hurt: None,
            hard_none: None,
            hard_heard: None,
            hard_glimpsed: None,
            hard_saw: None,
            hard_saw_hurt: None,
            __none: None,
            __heard: None,
            __glimpsed: None,
            __saw: None,
            __saw_hurt: None,
            pedant_steady: None,
            pedant_nervy: None,
            pedant_hard: None,
            selfown_steady: None,
            selfown_nervy: None,
            selfown_hard: None,
            folder_steady: None,
            folder_nervy: None,
            folder_hard: None,
            ramble_steady: None,
            ramble_nervy: None,
            ramble_hard: None,
            fan_steady: None,
            fan_nervy: None,
            fan_hard: None,
            stoic_steady: None,
            stoic_nervy: None,
            stoic_hard: None,
            con_steady: None,
            con_nervy: None,
            con_hard: None,
            game_steady: None,
            game_nervy: None,
            game_hard: None,
            menace_steady: None,
            menace_nervy: None,
            menace_hard: None,
        }
    }
}
impl Default for INTELGUIDEDrive { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/dialogue.ts:349 DRIVE_AWAY, ../.npm-global/lib/node_modules/typescript/lib/lib.es5.d.ts:1583)
#[derive(Clone, Debug)]
pub struct DRIVEAWAY {
    pub steady: Option<String>,
    pub nervy: Option<String>,
    pub hard: Option<String>,
    pub under: Option<String>,
}
impl DRIVEAWAY {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DRIVEAWAY {
            steady: None,
            nervy: None,
            hard: None,
            under: None,
        }
    }
}
impl Default for DRIVEAWAY { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/dialogue.ts:351 DRIVE_AWAY_GUIDE, ../.npm-global/lib/node_modules/typescript/lib/lib.es5.d.ts:1583)
#[derive(Clone, Debug)]
pub struct DRIVEAWAYGUIDE {
    pub steady: Option<String>,
    pub nervy: Option<String>,
    pub hard: Option<String>,
    pub under: Option<String>,
}
impl DRIVEAWAYGUIDE {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DRIVEAWAYGUIDE {
            steady: None,
            nervy: None,
            hard: None,
            under: None,
        }
    }
}
impl Default for DRIVEAWAYGUIDE { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/dialogue.ts:364 SHAPE_LEAD, ../.npm-global/lib/node_modules/typescript/lib/lib.es5.d.ts:1583)
#[derive(Clone, Debug)]
pub struct SHAPELEAD {
    pub pedant: Option<String>,
    pub selfown: Option<String>,
    pub folder: Option<String>,
    pub ramble: Option<String>,
    pub fan: Option<String>,
    pub stoic: Option<String>,
    pub con: Option<String>,
    pub game: Option<String>,
    pub menace: Option<String>,
}
impl SHAPELEAD {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SHAPELEAD {
            pedant: None,
            selfown: None,
            folder: None,
            ramble: None,
            fan: None,
            stoic: None,
            con: None,
            game: None,
            menace: None,
        }
    }
}
impl Default for SHAPELEAD { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/dialogue.ts:374 roomRects, src/game/dialogue.ts:374)
#[derive(Clone, Debug)]
pub struct DialogueModRoomRectsValue {
    pub rooms: Vec<crate::dialogue::Rect>,
    pub building: crate::dialogue::Rect,
    pub office: bool,
}
impl DialogueModRoomRectsValue {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DialogueModRoomRectsValue {
            rooms: Vec::new(),
            building: crate::dialogue::Rect::zeroed(),
            office: false,
        }
    }
}
impl Default for DialogueModRoomRectsValue { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/dialogue.ts:1676 HANDLER_LINES, src/game/dialogue.ts:1676)
#[derive(Clone, Debug)]
pub struct HANDLERLINESValueItem {
    pub h: String,
    pub s: String,
    pub h_own: Option<String>,
    pub s_own: Option<String>,
    pub h_now: Option<String>,
}
impl HANDLERLINESValueItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        HANDLERLINESValueItem {
            h: String::new(),
            s: String::new(),
            h_own: None,
            s_own: None,
            h_now: None,
        }
    }
}
impl Default for HANDLERLINESValueItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/dialogue.ts:1764 AMBIENT, src/game/dialogue.ts:1764)
#[derive(Clone, Copy, Debug)]
pub struct AMBIENT {
    pub pair_m: f64,
    pub cooldown: f64,
    pub first_after: f64,
    pub gap: f64,
    pub face_rate: f64,
}
impl AMBIENT {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        AMBIENT {
            pair_m: 0.0,
            cooldown: 0.0,
            first_after: 0.0,
            gap: 0.0,
            face_rate: 0.0,
        }
    }
}
impl Default for AMBIENT { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/dialogue.ts:1784 NEEDS_LINES, src/game/dialogue.ts:1784)
#[derive(Clone, Debug)]
pub struct NEEDSLINES {
    pub announce: JsMap<String, Vec<String>>,
    pub announce_own: JsMap<String, Vec<String>>,
    pub now: JsMap<String, String>,
    pub reply: Vec<String>,
    pub reply_other: Vec<String>,
    pub radio: String,
    pub short: JsMap<String, String>,
    pub short_own: JsMap<String, String>,
    pub back: Vec<String>,
}
impl NEEDSLINES {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        NEEDSLINES {
            announce: JsMap::new(),
            announce_own: JsMap::new(),
            now: JsMap::new(),
            reply: Vec::new(),
            reply_other: Vec::new(),
            radio: String::new(),
            short: JsMap::new(),
            short_own: JsMap::new(),
            back: Vec::new(),
        }
    }
}
impl Default for NEEDSLINES { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/dialogue.ts:1820 CIV_LINES, src/game/dialogue.ts:1820)
#[derive(Clone, Debug)]
pub struct CIVLINES {
    pub glimpse: Vec<String>,
    pub shrug: Vec<String>,
    pub gasp: JsMap<String, Vec<String>>,
    pub flee: Vec<String>,
    pub report: JsMap<String, Vec<String>>,
    pub guard_ack: crate::dialogue::CIVLINESGuardAck,
    pub guard_radio: JsMap<String, String>,
    pub phone: Vec<String>,
    pub phone_armed: Vec<String>,
    pub control: JsMap<String, String>,
    pub cower: Vec<String>,
    pub held_fear: Vec<String>,
    pub held_fear_worker: Vec<String>,
    pub intel: JsMap<String, Vec<String>>,
    pub dry: Vec<String>,
    pub comply: crate::dialogue::CIVLINESComply,
    pub sam_ask: Vec<String>,
    pub found: Vec<String>,
    pub found_down: crate::dialogue::CIVLINESFoundDown,
    pub outrage: String,
    pub greet: crate::dialogue::CIVLINESGreet,
    pub settle: Vec<String>,
    pub qrf: String,
    pub evac: Vec<String>,
    pub own: crate::dialogue::CIVLINESOwn,
    pub now: crate::dialogue::CIVLINESNow,
    pub guide: crate::dialogue::CIVLINESGuide,
}
impl CIVLINES {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CIVLINES {
            glimpse: Vec::new(),
            shrug: Vec::new(),
            gasp: JsMap::new(),
            flee: Vec::new(),
            report: JsMap::new(),
            guard_ack: crate::dialogue::CIVLINESGuardAck::zeroed(),
            guard_radio: JsMap::new(),
            phone: Vec::new(),
            phone_armed: Vec::new(),
            control: JsMap::new(),
            cower: Vec::new(),
            held_fear: Vec::new(),
            held_fear_worker: Vec::new(),
            intel: JsMap::new(),
            dry: Vec::new(),
            comply: crate::dialogue::CIVLINESComply::zeroed(),
            sam_ask: Vec::new(),
            found: Vec::new(),
            found_down: crate::dialogue::CIVLINESFoundDown::zeroed(),
            outrage: String::new(),
            greet: crate::dialogue::CIVLINESGreet::zeroed(),
            settle: Vec::new(),
            qrf: String::new(),
            evac: Vec::new(),
            own: crate::dialogue::CIVLINESOwn::zeroed(),
            now: crate::dialogue::CIVLINESNow::zeroed(),
            guide: crate::dialogue::CIVLINESGuide::zeroed(),
        }
    }
}
impl Default for CIVLINES { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/dialogue.ts:1854 CIVLINES.guardAck, src/game/dialogue.ts:1854)
#[derive(Clone, Debug)]
pub struct CIVLINESGuardAck {
    pub someone: Vec<String>,
    pub armed: Vec<String>,
}
impl CIVLINESGuardAck {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CIVLINESGuardAck {
            someone: Vec::new(),
            armed: Vec::new(),
        }
    }
}
impl Default for CIVLINESGuardAck { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/dialogue.ts:1885 CIVLINES.comply, src/game/dialogue.ts:1885)
#[derive(Clone, Debug)]
pub struct CIVLINESComply {
    pub sam: Vec<String>,
    pub civ: Vec<String>,
}
impl CIVLINESComply {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CIVLINESComply {
            sam: Vec::new(),
            civ: Vec::new(),
        }
    }
}
impl Default for CIVLINESComply { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/dialogue.ts:1891 CIVLINES.foundDown, src/game/dialogue.ts:1891)
#[derive(Clone, Debug)]
pub struct CIVLINESFoundDown {
    pub cold: String,
    pub dead: String,
}
impl CIVLINESFoundDown {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CIVLINESFoundDown {
            cold: String::new(),
            dead: String::new(),
        }
    }
}
impl Default for CIVLINESFoundDown { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/dialogue.ts:1897 CIVLINES.greet, src/game/dialogue.ts:1897)
#[derive(Clone, Debug)]
pub struct CIVLINESGreet {
    pub civ: Vec<String>,
    pub guard: Vec<String>,
}
impl CIVLINESGreet {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CIVLINESGreet {
            civ: Vec::new(),
            guard: Vec::new(),
        }
    }
}
impl Default for CIVLINESGreet { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/dialogue.ts:1906 CIVLINES.own, src/game/dialogue.ts:1906)
#[derive(Clone, Debug)]
pub struct CIVLINESOwn {
    pub phone: String,
    pub phone_armed: String,
    pub control: JsMap<String, String>,
    pub held_fear: String,
    pub intel_pad: String,
    pub intel: Vec<String>,
    pub sam_ask: String,
    pub found_cold: String,
}
impl CIVLINESOwn {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CIVLINESOwn {
            phone: String::new(),
            phone_armed: String::new(),
            control: JsMap::new(),
            held_fear: String::new(),
            intel_pad: String::new(),
            intel: Vec::new(),
            sam_ask: String::new(),
            found_cold: String::new(),
        }
    }
}
impl Default for CIVLINESOwn { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/dialogue.ts:1920 CIVLINES.now, src/game/dialogue.ts:1920)
#[derive(Clone, Debug)]
pub struct CIVLINESNow {
    pub guard_ack: String,
    pub qrf: String,
}
impl CIVLINESNow {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CIVLINESNow {
            guard_ack: String::new(),
            qrf: String::new(),
        }
    }
}
impl Default for CIVLINESNow { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/dialogue.ts:1922 CIVLINES.guide, src/game/dialogue.ts:1922)
#[derive(Clone, Debug)]
pub struct CIVLINESGuide {
    pub report: JsMap<String, Vec<String>>,
    pub control: JsMap<String, String>,
    pub greet_civ: Vec<String>,
    pub greet_guard: Vec<String>,
    pub own: crate::dialogue::CIVLINESGuideOwn,
}
impl CIVLINESGuide {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CIVLINESGuide {
            report: JsMap::new(),
            control: JsMap::new(),
            greet_civ: Vec::new(),
            greet_guard: Vec::new(),
            own: crate::dialogue::CIVLINESGuideOwn::zeroed(),
        }
    }
}
impl Default for CIVLINESGuide { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/dialogue.ts:1934 CIVLINESGuide.own, src/game/dialogue.ts:1934)
#[derive(Clone, Debug)]
pub struct CIVLINESGuideOwn {
    pub control: JsMap<String, String>,
    pub intel: Vec<String>,
    pub found_cold: String,
}
impl CIVLINESGuideOwn {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CIVLINESGuideOwn {
            control: JsMap::new(),
            intel: Vec::new(),
            found_cold: String::new(),
        }
    }
}
impl Default for CIVLINESGuideOwn { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/dialogue.ts:1958 HOSTAGE_LINES, src/game/dialogue.ts:1958)
#[derive(Clone, Debug)]
pub struct HOSTAGELINES {
    pub wait: Vec<String>,
    pub threat: Vec<String>,
    pub aimed: Vec<String>,
    pub shot: Vec<String>,
    pub entry: Vec<String>,
    pub alone: Vec<String>,
    pub secured: Vec<String>,
    pub team: Vec<String>,
}
impl HOSTAGELINES {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        HOSTAGELINES {
            wait: Vec::new(),
            threat: Vec::new(),
            aimed: Vec::new(),
            shot: Vec::new(),
            entry: Vec::new(),
            alone: Vec::new(),
            secured: Vec::new(),
            team: Vec::new(),
        }
    }
}
impl Default for HOSTAGELINES { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/dialogue.ts:1983 SNATCH_LINES, src/game/dialogue.ts:1983)
#[derive(Clone, Debug)]
pub struct SNATCHLINES {
    pub running: Vec<String>,
    pub cornered: Vec<String>,
    pub hands_up: Vec<String>,
    pub tied: Vec<String>,
    pub held: Vec<String>,
    pub dry: Vec<String>,
}
impl SNATCHLINES {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SNATCHLINES {
            running: Vec::new(),
            cornered: Vec::new(),
            hands_up: Vec::new(),
            tied: Vec::new(),
            held: Vec::new(),
            dry: Vec::new(),
        }
    }
}
impl Default for SNATCHLINES { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/dialogue.ts:121 agitationTerms ->, src/game/dialogue.ts:121)
#[derive(Clone, Debug)]
pub struct AgitationTerms {
    pub a: f64,
    pub why: Vec<String>,
}
impl AgitationTerms {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        AgitationTerms {
            a: 0.0,
            why: Vec::new(),
        }
    }
}
impl Default for AgitationTerms { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/dialogue.ts:208 saidText(l), src/game/dialogue.ts:208)
#[derive(Clone, Debug)]
pub struct SaidTextL {
    pub text: String,
    pub now: Option<String>,
    pub away: Option<String>,
    pub bare: Option<String>,
    pub guide: Option<String>,
}
impl SaidTextL {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SaidTextL {
            text: String::new(),
            now: None,
            away: None,
            bare: None,
            guide: None,
        }
    }
}
impl Default for SaidTextL { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/dialogue.ts:347 intelTable ->, ../.npm-global/lib/node_modules/typescript/lib/lib.es5.d.ts:1583)
#[derive(Clone, Debug)]
pub struct IntelTable {
    pub steady: Option<String>,
    pub nervy: Option<String>,
    pub hard: Option<String>,
    pub under: Option<String>,
    pub pedant: Option<String>,
    pub selfown: Option<String>,
    pub folder: Option<String>,
    pub ramble: Option<String>,
    pub fan: Option<String>,
    pub stoic: Option<String>,
    pub con: Option<String>,
    pub game: Option<String>,
    pub menace: Option<String>,
    pub steady_none: Option<String>,
    pub steady_heard: Option<String>,
    pub steady_glimpsed: Option<String>,
    pub steady_saw: Option<String>,
    pub steady_saw_hurt: Option<String>,
    pub nervy_none: Option<String>,
    pub nervy_heard: Option<String>,
    pub nervy_glimpsed: Option<String>,
    pub nervy_saw: Option<String>,
    pub nervy_saw_hurt: Option<String>,
    pub hard_none: Option<String>,
    pub hard_heard: Option<String>,
    pub hard_glimpsed: Option<String>,
    pub hard_saw: Option<String>,
    pub hard_saw_hurt: Option<String>,
    pub __none: Option<String>,
    pub __heard: Option<String>,
    pub __glimpsed: Option<String>,
    pub __saw: Option<String>,
    pub __saw_hurt: Option<String>,
    pub pedant_steady: Option<String>,
    pub pedant_nervy: Option<String>,
    pub pedant_hard: Option<String>,
    pub selfown_steady: Option<String>,
    pub selfown_nervy: Option<String>,
    pub selfown_hard: Option<String>,
    pub folder_steady: Option<String>,
    pub folder_nervy: Option<String>,
    pub folder_hard: Option<String>,
    pub ramble_steady: Option<String>,
    pub ramble_nervy: Option<String>,
    pub ramble_hard: Option<String>,
    pub fan_steady: Option<String>,
    pub fan_nervy: Option<String>,
    pub fan_hard: Option<String>,
    pub stoic_steady: Option<String>,
    pub stoic_nervy: Option<String>,
    pub stoic_hard: Option<String>,
    pub con_steady: Option<String>,
    pub con_nervy: Option<String>,
    pub con_hard: Option<String>,
    pub game_steady: Option<String>,
    pub game_nervy: Option<String>,
    pub game_hard: Option<String>,
    pub menace_steady: Option<String>,
    pub menace_nervy: Option<String>,
    pub menace_hard: Option<String>,
}
impl IntelTable {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        IntelTable {
            steady: None,
            nervy: None,
            hard: None,
            under: None,
            pedant: None,
            selfown: None,
            folder: None,
            ramble: None,
            fan: None,
            stoic: None,
            con: None,
            game: None,
            menace: None,
            steady_none: None,
            steady_heard: None,
            steady_glimpsed: None,
            steady_saw: None,
            steady_saw_hurt: None,
            nervy_none: None,
            nervy_heard: None,
            nervy_glimpsed: None,
            nervy_saw: None,
            nervy_saw_hurt: None,
            hard_none: None,
            hard_heard: None,
            hard_glimpsed: None,
            hard_saw: None,
            hard_saw_hurt: None,
            __none: None,
            __heard: None,
            __glimpsed: None,
            __saw: None,
            __saw_hurt: None,
            pedant_steady: None,
            pedant_nervy: None,
            pedant_hard: None,
            selfown_steady: None,
            selfown_nervy: None,
            selfown_hard: None,
            folder_steady: None,
            folder_nervy: None,
            folder_hard: None,
            ramble_steady: None,
            ramble_nervy: None,
            ramble_hard: None,
            fan_steady: None,
            fan_nervy: None,
            fan_hard: None,
            stoic_steady: None,
            stoic_nervy: None,
            stoic_hard: None,
            con_steady: None,
            con_nervy: None,
            con_hard: None,
            game_steady: None,
            game_nervy: None,
            game_hard: None,
            menace_steady: None,
            menace_nervy: None,
            menace_hard: None,
        }
    }
}
impl Default for IntelTable { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/dialogue.ts:352 driveAway ->, ../.npm-global/lib/node_modules/typescript/lib/lib.es5.d.ts:1583)
#[derive(Clone, Debug)]
pub struct DriveAway {
    pub steady: Option<String>,
    pub nervy: Option<String>,
    pub hard: Option<String>,
    pub under: Option<String>,
}
impl DriveAway {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DriveAway {
            steady: None,
            nervy: None,
            hard: None,
            under: None,
        }
    }
}
impl Default for DriveAway { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/dialogue.ts:356 shapedIntelText(sc), src/game/dialogue.ts:356)
#[derive(Clone, Debug)]
pub struct ShapedIntelTextSc {
    pub id: String,
    pub shape: crate::dialogue::ShapeKey,
}
impl ShapedIntelTextSc {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ShapedIntelTextSc {
            id: String::new(),
            shape: crate::dialogue::ShapeKey::Pedant,
        }
    }
}
impl Default for ShapedIntelTextSc { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/dialogue.ts:412 ownPlace ->, src/game/dialogue.ts:412)
#[derive(Clone, Debug)]
pub struct OwnPlace {
    pub rect: crate::dialogue::PlaceRect,
    pub d: f64,
}
impl OwnPlace {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        OwnPlace {
            rect: crate::dialogue::PlaceRect::zeroed(),
            d: 0.0,
        }
    }
}
impl Default for OwnPlace { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/dialogue.ts:671 needsHold(facts), ../.npm-global/lib/node_modules/typescript/lib/lib.es5.d.ts:1583)
#[derive(Clone, Copy, Debug)]
pub struct NeedsHoldFacts {
    pub shift: Option<bool>,
    pub rollcall_live: Option<bool>,
    pub blackout: Option<bool>,
    pub body_found: Option<bool>,
    pub floor_up: Option<bool>,
    pub lockdown: Option<bool>,
    pub episode: Option<bool>,
    pub heard_small: Option<bool>,
    pub heard_shots: Option<bool>,
    pub heard_bang: Option<bool>,
    pub body_called: Option<bool>,
    pub body_known: Option<bool>,
    pub missing_known: Option<bool>,
    pub held_long: Option<bool>,
    pub dazzled: Option<bool>,
    pub saw_shot: Option<bool>,
    pub saw_drop: Option<bool>,
    pub alert_now: Option<bool>,
    pub drew_on_you: Option<bool>,
    pub partner_known: Option<bool>,
    pub office: Option<bool>,
}
impl NeedsHoldFacts {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        NeedsHoldFacts {
            shift: None,
            rollcall_live: None,
            blackout: None,
            body_found: None,
            floor_up: None,
            lockdown: None,
            episode: None,
            heard_small: None,
            heard_shots: None,
            heard_bang: None,
            body_called: None,
            body_known: None,
            missing_known: None,
            held_long: None,
            dazzled: None,
            saw_shot: None,
            saw_drop: None,
            alert_now: None,
            drew_on_you: None,
            partner_known: None,
            office: None,
        }
    }
}
impl Default for NeedsHoldFacts { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/dialogue.ts:675 pendingIntel ->, src/game/dialogue.ts:675)
#[derive(Clone, Debug)]
pub struct PendingIntelItem {
    pub kind: crate::dialogue::IntelKind,
    pub text: String,
}
impl PendingIntelItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        PendingIntelItem {
            kind: crate::dialogue::IntelKind::CheckOn,
            text: String::new(),
        }
    }
}
impl Default for PendingIntelItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/dialogue.ts:689 intelFor ->, src/game/dialogue.ts:689)
#[derive(Clone, Debug)]
pub struct IntelFor {
    pub kind: crate::dialogue::IntelKind,
    pub text: String,
}
impl IntelFor {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        IntelFor {
            kind: crate::dialogue::IntelKind::CheckOn,
            text: String::new(),
        }
    }
}
impl Default for IntelFor { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/dialogue.ts:778 promptFor ->, src/game/dialogue.ts:778)
#[derive(Clone, Debug)]
pub struct PromptFor {
    pub text: String,
    pub spent: bool,
}
impl PromptFor {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        PromptFor {
            text: String::new(),
            spent: false,
        }
    }
}
impl Default for PromptFor { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/dialogue.ts:965 chooseSamLine ->, src/game/dialogue.ts:965)
#[derive(Clone, Debug)]
pub struct ChooseSamLine {
    pub l: crate::dialogue::SamLine,
    pub text: String,
    pub fresh: bool,
}
impl ChooseSamLine {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ChooseSamLine {
            l: crate::dialogue::SamLine::zeroed(),
            text: String::new(),
            fresh: false,
        }
    }
}
impl Default for ChooseSamLine { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/dialogue.ts:1007 samKeyLineFor ->, src/game/dialogue.ts:1007)
#[derive(Clone, Debug)]
pub struct SamKeyLineFor {
    pub text: String,
    pub id: String,
}
impl SamKeyLineFor {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SamKeyLineFor {
            text: String::new(),
            id: String::new(),
        }
    }
}
impl Default for SamKeyLineFor { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/dialogue.ts:1016 samRemarkFor ->, src/game/dialogue.ts:1016)
#[derive(Clone, Debug)]
pub struct SamRemarkFor {
    pub text: String,
    pub id: String,
}
impl SamRemarkFor {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SamRemarkFor {
            text: String::new(),
            id: String::new(),
        }
    }
}
impl Default for SamRemarkFor { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/dialogue.ts:1025 synthCtx(over), src/game/dialogue.ts:1025)
#[derive(Clone, Debug)]
pub struct SynthCtxOver {
    pub facts: Option<crate::dialogue::SynthCtxOverFacts>,
    pub tokens: Option<crate::dialogue::SynthCtxOverTokens>,
    pub intel_live: Option<Vec<crate::dialogue::IntelKind>>,
    pub who: Option<String>,
}
impl SynthCtxOver {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SynthCtxOver {
            facts: None,
            tokens: None,
            intel_live: None,
            who: None,
        }
    }
}
impl Default for SynthCtxOver { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/dialogue.ts:1025 SynthCtxOver.facts, ../.npm-global/lib/node_modules/typescript/lib/lib.es5.d.ts:1583)
#[derive(Clone, Copy, Debug)]
pub struct SynthCtxOverFacts {
    pub shift: Option<bool>,
    pub rollcall_live: Option<bool>,
    pub blackout: Option<bool>,
    pub body_found: Option<bool>,
    pub floor_up: Option<bool>,
    pub lockdown: Option<bool>,
    pub episode: Option<bool>,
    pub heard_small: Option<bool>,
    pub heard_shots: Option<bool>,
    pub heard_bang: Option<bool>,
    pub body_called: Option<bool>,
    pub body_known: Option<bool>,
    pub missing_known: Option<bool>,
    pub held_long: Option<bool>,
    pub dazzled: Option<bool>,
    pub saw_shot: Option<bool>,
    pub saw_drop: Option<bool>,
    pub alert_now: Option<bool>,
    pub drew_on_you: Option<bool>,
    pub partner_known: Option<bool>,
    pub office: Option<bool>,
}
impl SynthCtxOverFacts {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SynthCtxOverFacts {
            shift: None,
            rollcall_live: None,
            blackout: None,
            body_found: None,
            floor_up: None,
            lockdown: None,
            episode: None,
            heard_small: None,
            heard_shots: None,
            heard_bang: None,
            body_called: None,
            body_known: None,
            missing_known: None,
            held_long: None,
            dazzled: None,
            saw_shot: None,
            saw_drop: None,
            alert_now: None,
            drew_on_you: None,
            partner_known: None,
            office: None,
        }
    }
}
impl Default for SynthCtxOverFacts { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/dialogue.ts:1025 SynthCtxOver.tokens, ../.npm-global/lib/node_modules/typescript/lib/lib.es5.d.ts:1583)
#[derive(Clone, Debug)]
pub struct SynthCtxOverTokens {
    pub door: Option<String>,
    pub code: Option<String>,
    pub missing: Option<String>,
    pub body: Option<String>,
    pub n: Option<String>,
    pub up: Option<String>,
    pub post: Option<String>,
    pub room: Option<String>,
    pub route: Option<String>,
    pub partner: Option<String>,
    pub victim: Option<String>,
    pub saw_where: Option<String>,
    pub heard_where: Option<String>,
    pub partner_route: Option<String>,
    pub caller: Option<String>,
    pub checker: Option<String>,
    pub checked: Option<String>,
    pub clear_room: Option<String>,
    pub breaker_room: Option<String>,
    pub check_eta: Option<String>,
    pub minutes: Option<String>,
    pub badge: Option<String>,
    pub pad_door: Option<String>,
}
impl SynthCtxOverTokens {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SynthCtxOverTokens {
            door: None,
            code: None,
            missing: None,
            body: None,
            n: None,
            up: None,
            post: None,
            room: None,
            route: None,
            partner: None,
            victim: None,
            saw_where: None,
            heard_where: None,
            partner_route: None,
            caller: None,
            checker: None,
            checked: None,
            clear_room: None,
            breaker_room: None,
            check_eta: None,
            minutes: None,
            badge: None,
            pad_door: None,
        }
    }
}
impl Default for SynthCtxOverTokens { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/dialogue.ts:1214 coverageMatrix ->, src/game/dialogue.ts:1214)
#[derive(Clone, Debug)]
pub struct CoverageMatrix {
    pub rows: Vec<crate::dialogue::CoverageMatrixRowsItem>,
    pub total: f64,
    pub intel: f64,
}
impl CoverageMatrix {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CoverageMatrix {
            rows: Vec::new(),
            total: 0.0,
            intel: 0.0,
        }
    }
}
impl Default for CoverageMatrix { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/dialogue.ts:1214 CoverageMatrix.rows, src/game/dialogue.ts:1214)
#[derive(Clone, Debug)]
pub struct CoverageMatrixRowsItem {
    pub k: String,
    pub counts: JsMap<String, f64>,
    pub total: f64,
}
impl CoverageMatrixRowsItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CoverageMatrixRowsItem {
            k: String::new(),
            counts: JsMap::new(),
            total: 0.0,
        }
    }
}
impl Default for CoverageMatrixRowsItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/dialogue.ts:1220 samCoverage ->, src/game/dialogue.ts:1220)
#[derive(Clone, Debug)]
pub struct SamCoverage {
    pub rows: Vec<crate::dialogue::SamCoverageRowsItem>,
    pub q: crate::dialogue::SamCoverageQ,
    pub key: f64,
    pub total: f64,
}
impl SamCoverage {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SamCoverage {
            rows: Vec::new(),
            q: crate::dialogue::SamCoverageQ::zeroed(),
            key: 0.0,
            total: 0.0,
        }
    }
}
impl Default for SamCoverage { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/dialogue.ts:1220 SamCoverage.rows, src/game/dialogue.ts:1220)
#[derive(Clone, Copy, Debug)]
pub struct SamCoverageRowsItem {
    pub s: crate::dialogue::SamStage,
    pub plain: f64,
    pub gun: f64,
    pub arm: f64,
    pub t: f64,
    pub k: f64,
    pub floor: f64,
    pub total: f64,
}
impl SamCoverageRowsItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SamCoverageRowsItem {
            s: crate::dialogue::SamStage::Press,
            plain: 0.0,
            gun: 0.0,
            arm: 0.0,
            t: 0.0,
            k: 0.0,
            floor: 0.0,
            total: 0.0,
        }
    }
}
impl Default for SamCoverageRowsItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/dialogue.ts:1220 SamCoverage.q, ../.npm-global/lib/node_modules/typescript/lib/lib.es5.d.ts:1583)
#[derive(Clone, Copy, Debug)]
pub struct SamCoverageQ {
    pub check_on: Option<f64>,
    pub clearing: Option<f64>,
    pub rollcall_live: Option<f64>,
    pub blackout: Option<f64>,
    pub door: Option<f64>,
    pub code: Option<f64>,
    pub rollcall: Option<f64>,
    pub lockdown_posts: Option<f64>,
    pub routes: Option<f64>,
    pub missing: Option<f64>,
    pub drive: Option<f64>,
}
impl SamCoverageQ {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SamCoverageQ {
            check_on: None,
            clearing: None,
            rollcall_live: None,
            blackout: None,
            door: None,
            code: None,
            rollcall: None,
            lockdown_posts: None,
            routes: None,
            missing: None,
            drive: None,
        }
    }
}
impl Default for SamCoverageQ { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/dialogue.ts:1226 scriptCoverage ->, src/game/dialogue.ts:1226)
#[derive(Clone, Debug)]
pub struct ScriptCoverageItem {
    pub shape: crate::dialogue::ShapeKey,
    pub by_t: crate::dialogue::ScriptCoverageItemByT,
    pub k: String,
    pub band: String,
    pub pin: String,
    pub total: f64,
}
impl ScriptCoverageItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ScriptCoverageItem {
            shape: crate::dialogue::ShapeKey::Pedant,
            by_t: crate::dialogue::ScriptCoverageItemByT::zeroed(),
            k: String::new(),
            band: String::new(),
            pin: String::new(),
            total: 0.0,
        }
    }
}
impl Default for ScriptCoverageItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/dialogue.ts:1226 ScriptCoverageItem.byT, ../.npm-global/lib/node_modules/typescript/lib/lib.es5.d.ts:1583)
#[derive(Clone, Copy, Debug)]
pub struct ScriptCoverageItemByT {
    pub steady: Option<f64>,
    pub nervy: Option<f64>,
    pub hard: Option<f64>,
}
impl ScriptCoverageItemByT {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ScriptCoverageItemByT {
            steady: None,
            nervy: None,
            hard: None,
        }
    }
}
impl Default for ScriptCoverageItemByT { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/dialogue.ts:1233 lineLengthReport ->, src/game/dialogue.ts:1233)
#[derive(Clone, Debug)]
pub struct LineLengthReport {
    pub men: Vec<crate::dialogue::LineLengthReportMenItem>,
    pub sam: Vec<crate::dialogue::LineLengthReportSamItem>,
}
impl LineLengthReport {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        LineLengthReport {
            men: Vec::new(),
            sam: Vec::new(),
        }
    }
}
impl Default for LineLengthReport { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/dialogue.ts:1233 LineLengthReport.men, src/game/dialogue.ts:1233)
#[derive(Clone, Debug)]
pub struct LineLengthReportMenItem {
    pub where_: String,
    pub text: String,
}
impl LineLengthReportMenItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        LineLengthReportMenItem {
            where_: String::new(),
            text: String::new(),
        }
    }
}
impl Default for LineLengthReportMenItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/dialogue.ts:1233 LineLengthReport.sam, src/game/dialogue.ts:1233)
#[derive(Clone, Debug)]
pub struct LineLengthReportSamItem {
    pub where_: String,
    pub text: String,
}
impl LineLengthReportSamItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        LineLengthReportSamItem {
            where_: String::new(),
            text: String::new(),
        }
    }
}
impl Default for LineLengthReportSamItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/dialogue.ts:1324 himOf ->, src/game/dialogue.ts:1324)
#[derive(Clone, Debug)]
pub struct HimOf {
    pub text: String,
    pub k: crate::dialogue::HimOfK,
}
impl HimOf {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        HimOf {
            text: String::new(),
            k: crate::dialogue::HimOfK::None,
        }
    }
}
impl Default for HimOf { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum HimOfK {
    None,
    Heard,
    Glimpsed,
    Saw,
    SawHurt,
    Empty,
}
impl HimOfK {
    pub const fn as_str(self) -> &'static str {
        match self {
            HimOfK::None => "none",
            HimOfK::Heard => "heard",
            HimOfK::Glimpsed => "glimpsed",
            HimOfK::Saw => "saw",
            HimOfK::SawHurt => "sawHurt",
            HimOfK::Empty => "*",
        }
    }
}

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct DialogueMod {
    /// (src/game/dialogue.ts:374 roomRects)
    pub room_rects: JsMap<u32, crate::dialogue::DialogueModRoomRectsValue>,
    /// (src/game/dialogue.ts:387 ownRects)
    pub own_rects: JsMap<u32, Option<Vec<crate::dialogue::PlaceRect>>>,
    /// (src/game/dialogue.ts:531 recentByGame)
    pub recent_by_game: Option<JsMap<String, f64>>,
    /// (src/game/dialogue.ts:820 SAM_LINES)
    pub sam_lines: Vec<crate::dialogue::SamLine>,
    /// (src/game/dialogue.ts:929 samMemByGame)
    pub sam_mem_by_game: Option<crate::dialogue::SamMemory>,
    /// (src/game/dialogue.ts:1276 scriptByTalk)
    pub script_by_talk: JsMap<GameTalkId, Option<crate::dialogue::Script>>,
    /// (src/game/dialogue.ts:1277 forcedByTalk)
    pub forced_by_talk: JsMap<GameTalkId, Option<String>>,
    /// (src/game/dialogue.ts:1714 radioByGame)
    pub radio_by_game: Option<crate::dialogue::RadioState>,
    /// (src/game/dialogue.ts:2001 ambientByGame)
    pub ambient_by_game: Option<crate::dialogue::AmbientState>,
}
impl DialogueMod {
    pub fn blank() -> Self {
        DialogueMod {
            room_rects: JsMap::new(),
            own_rects: JsMap::new(),
            recent_by_game: None,
            sam_lines: alloc::vec![crate::dialogue::SamLine { id: String::from("So1"), s: crate::dialogue::SamLineS::Open, q: None, t: None, k: None, needs: None, text: String::from("Quiet now. You and I are going to talk."), w: None, now: None, away: None, guide: None }, crate::dialogue::SamLine { id: String::from("So2"), s: crate::dialogue::SamLineS::Open, q: None, t: None, k: None, needs: None, text: String::from("Not a sound. Nod if you understand me."), w: None, now: None, away: None, guide: None }, crate::dialogue::SamLine { id: String::from("So3"), s: crate::dialogue::SamLineS::Open, q: None, t: None, k: None, needs: None, text: String::from("Evening. Let's keep this between us."), w: None, now: None, away: None, guide: None }, crate::dialogue::SamLine { id: String::from("So4"), s: crate::dialogue::SamLineS::Open, q: None, t: None, k: None, needs: None, text: String::from("You're having a bad night. Keep it at bad."), w: None, now: None, away: None, guide: None }, crate::dialogue::SamLine { id: String::from("So5"), s: crate::dialogue::SamLineS::Open, q: None, t: None, k: None, needs: None, text: String::from("Relax. If I wanted you dead, you'd have missed it."), w: None, now: None, away: None, guide: None }, crate::dialogue::SamLine { id: String::from("So6"), s: crate::dialogue::SamLineS::Open, q: None, t: None, k: None, needs: None, text: String::from("Don't turn around. You won't like the view."), w: None, now: None, away: None, guide: None }, crate::dialogue::SamLine { id: String::from("So7"), s: crate::dialogue::SamLineS::Open, q: None, t: None, k: None, needs: None, text: String::from("Hands still. Breathe through your nose. Good."), w: None, now: None, away: None, guide: None }, crate::dialogue::SamLine { id: String::from("Sog1"), s: crate::dialogue::SamLineS::Open, q: None, t: None, k: None, needs: Some(alloc::vec![crate::dialogue::SamNeed::Gun]), text: String::from("Feel that at your temple? Good. Whisper."), w: None, now: None, away: None, guide: None }, crate::dialogue::SamLine { id: String::from("Sog2"), s: crate::dialogue::SamLineS::Open, q: None, t: None, k: None, needs: Some(alloc::vec![crate::dialogue::SamNeed::Gun]), text: String::from("That's a gun behind your ear. Indoor voice."), w: None, now: None, away: None, guide: None }, crate::dialogue::SamLine { id: String::from("Sog3"), s: crate::dialogue::SamLineS::Open, q: None, t: None, k: None, needs: Some(alloc::vec![crate::dialogue::SamNeed::Gun]), text: String::from("One twitch toward that radio and this gets loud."), w: None, now: None, away: None, guide: None }, crate::dialogue::SamLine { id: String::from("Sog4"), s: crate::dialogue::SamLineS::Open, q: None, t: None, k: None, needs: Some(alloc::vec![crate::dialogue::SamNeed::Gun]), text: String::from("The pistol's the polite version. Stay polite."), w: None, now: None, away: None, guide: None }, crate::dialogue::SamLine { id: String::from("Sog5"), s: crate::dialogue::SamLineS::Open, q: None, t: None, k: None, needs: Some(alloc::vec![crate::dialogue::SamNeed::Gun]), text: String::from("Cold, isn't it. Warm up with an easy question."), w: None, now: None, away: None, guide: None }, crate::dialogue::SamLine { id: String::from("Soa1"), s: crate::dialogue::SamLineS::Open, q: None, t: None, k: None, needs: Some(alloc::vec![crate::dialogue::SamNeed::Arm]), text: String::from("That's my arm. It tightens when I get bored."), w: None, now: None, away: None, guide: None }, crate::dialogue::SamLine { id: String::from("Soa2"), s: crate::dialogue::SamLineS::Open, q: None, t: None, k: None, needs: Some(alloc::vec![crate::dialogue::SamNeed::Arm]), text: String::from("Struggle and you sleep. Talk and you sleep later."), w: None, now: None, away: None, guide: None }, crate::dialogue::SamLine { id: String::from("Soa3"), s: crate::dialogue::SamLineS::Open, q: None, t: None, k: None, needs: Some(alloc::vec![crate::dialogue::SamNeed::Arm]), text: String::from("You can breathe or you can shout. Choose breathe."), w: None, now: None, away: None, guide: None }, crate::dialogue::SamLine { id: String::from("Sok1"), s: crate::dialogue::SamLineS::Open, q: None, t: None, k: None, needs: Some(alloc::vec![crate::dialogue::SamNeed::Knife]), text: String::from("Feel the edge? Then we understand each other."), w: None, now: None, away: None, guide: None }, crate::dialogue::SamLine { id: String::from("Sok2"), s: crate::dialogue::SamLineS::Open, q: None, t: None, k: None, needs: Some(alloc::vec![crate::dialogue::SamNeed::Knife]), text: String::from("Easy. The blade only moves if you do."), w: None, now: None, away: None, guide: None }, crate::dialogue::SamLine { id: String::from("Sob1"), s: crate::dialogue::SamLineS::Open, q: None, t: None, k: None, needs: Some(alloc::vec![crate::dialogue::SamNeed::Blackout]), text: String::from("Dark, isn't it. That was me. Now talk."), w: Some(1.4), now: None, away: None, guide: None }, crate::dialogue::SamLine { id: String::from("Sox1"), s: crate::dialogue::SamLineS::Open, q: None, t: None, k: None, needs: Some(alloc::vec![crate::dialogue::SamNeed::Alarm]), text: String::from("Your friends are jumpy tonight. Don't join them."), w: None, now: None, away: None, guide: None }, crate::dialogue::SamLine { id: String::from("Sol1"), s: crate::dialogue::SamLineS::Open, q: None, t: None, k: None, needs: Some(alloc::vec![crate::dialogue::SamNeed::Lockdown]), text: String::from("All locked down, and still nobody heard me coming."), w: Some(1.4), now: None, away: None, guide: None }, crate::dialogue::SamLine { id: String::from("Soh1"), s: crate::dialogue::SamLineS::Open, q: None, t: None, k: Some(crate::dialogue::Knowledge::SawHurt), needs: None, text: String::from("You saw how {victim} went down. Let's do you differently."), w: None, now: None, away: None, guide: None }, crate::dialogue::SamLine { id: String::from("Soh2"), s: crate::dialogue::SamLineS::Open, q: None, t: None, k: Some(crate::dialogue::Knowledge::SawHurt), needs: None, text: String::from("{Victim} didn't get a conversation. You do. Use it."), w: None, now: None, away: None, guide: None }, crate::dialogue::SamLine { id: String::from("Sos1"), s: crate::dialogue::SamLineS::Open, q: None, t: None, k: Some(crate::dialogue::Knowledge::Saw), needs: None, text: String::from("You were looking for me. Congratulations."), w: None, now: None, away: None, guide: None }, crate::dialogue::SamLine { id: String::from("Sos2"), s: crate::dialogue::SamLineS::Open, q: None, t: None, k: Some(crate::dialogue::Knowledge::Saw), needs: None, text: String::from("Found me. Not how you pictured it, is it."), w: None, now: None, away: None, guide: None }, crate::dialogue::SamLine { id: String::from("Soc1"), s: crate::dialogue::SamLineS::Open, q: None, t: None, k: Some(crate::dialogue::Knowledge::Glimpsed), needs: None, text: String::from("That shape in the dark earlier? Hello again."), w: None, now: None, away: None, guide: None }, crate::dialogue::SamLine { id: String::from("Sp1"), s: crate::dialogue::SamLineS::Press, q: None, t: None, k: None, needs: None, text: String::from("Think hard. I only ask things twice."), w: None, now: None, away: None, guide: None }, crate::dialogue::SamLine { id: String::from("Sp2"), s: crate::dialogue::SamLineS::Press, q: None, t: None, k: None, needs: None, text: String::from("You're stalling. I can feel your pulse doing it."), w: None, now: None, away: None, guide: None }, crate::dialogue::SamLine { id: String::from("Sp3"), s: crate::dialogue::SamLineS::Press, q: None, t: None, k: None, needs: None, text: String::from("A wrong answer costs nothing. Silence costs more."), w: None, now: None, away: None, guide: None }, crate::dialogue::SamLine { id: String::from("Sp4"), s: crate::dialogue::SamLineS::Press, q: None, t: None, k: None, needs: None, text: String::from("I've got all night. You've got about a minute."), w: None, now: None, away: None, guide: None }, crate::dialogue::SamLine { id: String::from("Sp5"), s: crate::dialogue::SamLineS::Press, q: None, t: None, k: None, needs: None, text: String::from("Let's try that again, with words in it."), w: None, now: None, away: None, guide: None }, crate::dialogue::SamLine { id: String::from("Sp6"), s: crate::dialogue::SamLineS::Press, q: None, t: None, k: None, needs: None, text: String::from("This is me being patient. It doesn't keep."), w: None, now: None, away: None, guide: None }, crate::dialogue::SamLine { id: String::from("Spg1"), s: crate::dialogue::SamLineS::Press, q: None, t: None, k: None, needs: Some(alloc::vec![crate::dialogue::SamNeed::Gun]), text: String::from("The safety came off before you ever heard me."), w: None, now: None, away: None, guide: None }, crate::dialogue::SamLine { id: String::from("Spg2"), s: crate::dialogue::SamLineS::Press, q: None, t: None, k: None, needs: Some(alloc::vec![crate::dialogue::SamNeed::Gun]), text: String::from("I can ask louder. The gun only has one volume."), w: None, now: None, away: None, guide: None }, crate::dialogue::SamLine { id: String::from("Spg3"), s: crate::dialogue::SamLineS::Press, q: None, t: None, k: None, needs: Some(alloc::vec![crate::dialogue::SamNeed::Gun]), text: String::from("Talk to me, not the gun. It's a poor listener."), w: None, now: None, away: None, guide: None }, crate::dialogue::SamLine { id: String::from("Spg4"), s: crate::dialogue::SamLineS::Press, q: None, t: None, k: None, needs: Some(alloc::vec![crate::dialogue::SamNeed::Gun]), text: String::from("Every stall, the muzzle drifts an inch. Guess where."), w: None, now: None, away: None, guide: None }, crate::dialogue::SamLine { id: String::from("Spg5"), s: crate::dialogue::SamLineS::Press, q: None, t: None, k: None, needs: Some(alloc::vec![crate::dialogue::SamNeed::Gun]), text: String::from("Nod toward the truth. Slowly. My finger's tired."), w: None, now: None, away: None, guide: None }, crate::dialogue::SamLine { id: String::from("Spa1"), s: crate::dialogue::SamLineS::Press, q: None, t: None, k: None, needs: Some(alloc::vec![crate::dialogue::SamNeed::Arm]), text: String::from("Feel that? The arm's asking. Next it insists."), w: None, now: None, away: None, guide: None }, crate::dialogue::SamLine { id: String::from("Spa2"), s: crate::dialogue::SamLineS::Press, q: None, t: None, k: None, needs: Some(alloc::vec![crate::dialogue::SamNeed::Arm]), text: String::from("You go quiet, the arm goes tight. Simple system."), w: None, now: None, away: None, guide: None }, crate::dialogue::SamLine { id: String::from("Spa3"), s: crate::dialogue::SamLineS::Press, q: None, t: None, k: None, needs: Some(alloc::vec![crate::dialogue::SamNeed::Arm]), text: String::from("Air's a privilege from here. Spend it on words."), w: None, now: None, away: None, guide: None }, crate::dialogue::SamLine { id: String::from("Spa4"), s: crate::dialogue::SamLineS::Press, q: None, t: None, k: None, needs: Some(alloc::vec![crate::dialogue::SamNeed::Arm]), text: String::from("I can hold this longer than you can hold out."), w: None, now: None, away: None, guide: None }, crate::dialogue::SamLine { id: String::from("Spk1"), s: crate::dialogue::SamLineS::Press, q: None, t: None, k: None, needs: Some(alloc::vec![crate::dialogue::SamNeed::Knife]), text: String::from("The edge is patient. I'm less so."), w: None, now: None, away: None, guide: None }, crate::dialogue::SamLine { id: String::from("Spk2"), s: crate::dialogue::SamLineS::Press, q: None, t: None, k: None, needs: Some(alloc::vec![crate::dialogue::SamNeed::Knife]), text: String::from("Swallow carefully. Then answer carefully."), w: None, now: None, away: None, guide: None }, crate::dialogue::SamLine { id: String::from("Spn1"), s: crate::dialogue::SamLineS::Press, q: None, t: Some(crate::game::Temperament::Nervy), k: None, needs: None, text: String::from("Breathe. Slower. Now spend the breath on words."), w: None, now: None, away: None, guide: None }, crate::dialogue::SamLine { id: String::from("Spn2"), s: crate::dialogue::SamLineS::Press, q: None, t: Some(crate::game::Temperament::Nervy), k: None, needs: None, text: String::from("Stop shaking. You'll blur the answer."), w: None, now: None, away: None, guide: None }, crate::dialogue::SamLine { id: String::from("Sph1"), s: crate::dialogue::SamLineS::Press, q: None, t: Some(crate::game::Temperament::Hard), k: None, needs: None, text: String::from("Tough is fine. Tough just takes longer."), w: None, now: None, away: None, guide: None }, crate::dialogue::SamLine { id: String::from("Sph2"), s: crate::dialogue::SamLineS::Press, q: None, t: Some(crate::game::Temperament::Hard), k: None, needs: None, text: String::from("Save the attitude. Nobody's grading you."), w: None, now: None, away: None, guide: None }, crate::dialogue::SamLine { id: String::from("Sps1"), s: crate::dialogue::SamLineS::Press, q: None, t: Some(crate::game::Temperament::Steady), k: None, needs: None, text: String::from("You're calm. Good. Calm men remember details."), w: None, now: None, away: None, guide: None }, crate::dialogue::SamLine { id: String::from("Spx1"), s: crate::dialogue::SamLineS::Press, q: None, t: None, k: None, needs: Some(alloc::vec![crate::dialogue::SamNeed::Alarm]), text: String::from("They're hunting me. You're what's slowing me down."), w: None, now: None, away: None, guide: None }, crate::dialogue::SamLine { id: String::from("Spb1"), s: crate::dialogue::SamLineS::Press, q: None, t: None, k: None, needs: Some(alloc::vec![crate::dialogue::SamNeed::Blackout]), text: String::from("Lights are out. Nobody finds you by accident now."), w: None, now: None, away: None, guide: None }, crate::dialogue::SamLine { id: String::from("Spl1"), s: crate::dialogue::SamLineS::Press, q: None, t: None, k: None, needs: Some(alloc::vec![crate::dialogue::SamNeed::Lockdown]), text: String::from("Your floor's sealed. So is this conversation."), w: None, now: Some(String::from("You're sealed in. So is this conversation.")), away: None, guide: None }, crate::dialogue::SamLine { id: String::from("Sa1"), s: crate::dialogue::SamLineS::Ask, q: None, t: None, k: None, needs: None, text: String::from("Tell me something useful and you sleep it off."), w: None, now: None, away: None, guide: None }, crate::dialogue::SamLine { id: String::from("Sa2"), s: crate::dialogue::SamLineS::Ask, q: None, t: None, k: None, needs: None, text: String::from("What's on this floor I should worry about?"), w: None, now: Some(String::from("What's around here I should worry about?")), away: None, guide: None }, crate::dialogue::SamLine { id: String::from("Sa3"), s: crate::dialogue::SamLineS::Ask, q: None, t: None, k: None, needs: None, text: String::from("Last useful thing you know. Make it count."), w: None, now: None, away: None, guide: None }, crate::dialogue::SamLine { id: String::from("Sa4"), s: crate::dialogue::SamLineS::Ask, q: None, t: None, k: None, needs: None, text: String::from("What don't I know yet? Quickly."), w: None, now: None, away: None, guide: None }, crate::dialogue::SamLine { id: String::from("Sa5"), s: crate::dialogue::SamLineS::Ask, q: None, t: None, k: None, needs: None, text: String::from("Make yourself useful. It's a short interview."), w: None, now: None, away: None, guide: None }, crate::dialogue::SamLine { id: String::from("Sa6"), s: crate::dialogue::SamLineS::Ask, q: None, t: None, k: None, needs: None, text: String::from("What would ruin my night up here? Besides you."), w: None, now: Some(String::from("What would ruin my night here? Besides you.")), away: None, guide: None }, crate::dialogue::SamLine { id: String::from("Sah1"), s: crate::dialogue::SamLineS::Ask, q: None, t: Some(crate::game::Temperament::Hard), k: None, needs: None, text: String::from("Here's where you decide how tomorrow feels."), w: None, now: None, away: None, guide: None }, crate::dialogue::SamLine { id: String::from("San1"), s: crate::dialogue::SamLineS::Ask, q: None, t: Some(crate::game::Temperament::Nervy), k: None, needs: None, text: String::from("Small words. One useful thing. Go."), w: None, now: None, away: None, guide: None }, crate::dialogue::SamLine { id: String::from("Sag1"), s: crate::dialogue::SamLineS::Ask, q: None, t: None, k: None, needs: Some(alloc::vec![crate::dialogue::SamNeed::Gun]), text: String::from("The gun's listening too. Impress us both."), w: None, now: None, away: None, guide: None }, crate::dialogue::SamLine { id: String::from("Saa1"), s: crate::dialogue::SamLineS::Ask, q: None, t: None, k: None, needs: Some(alloc::vec![crate::dialogue::SamNeed::Arm]), text: String::from("One good answer buys you a lot of air."), w: None, now: None, away: None, guide: None }, crate::dialogue::SamLine { id: String::from("Sqd1"), s: crate::dialogue::SamLineS::Q, q: Some(crate::dialogue::IntelKind::Door), t: None, k: None, needs: None, text: String::from("The locked doors up here. Who carries keys?"), w: None, now: Some(String::from("The locked doors here. Who carries keys?")), away: None, guide: None }, crate::dialogue::SamLine { id: String::from("Sqd2"), s: crate::dialogue::SamLineS::Q, q: Some(crate::dialogue::IntelKind::Door), t: None, k: None, needs: None, text: String::from("That keyed door on your round. How does it open?"), w: None, now: None, away: None, guide: Some(String::from("That keyed door on your route. How does it open?")) }, crate::dialogue::SamLine { id: String::from("Sqr1"), s: crate::dialogue::SamLineS::Q, q: Some(crate::dialogue::IntelKind::Rollcall), t: None, k: None, needs: None, text: String::from("How long before somebody misses you?"), w: None, now: None, away: None, guide: None }, crate::dialogue::SamLine { id: String::from("Sqr2"), s: crate::dialogue::SamLineS::Q, q: Some(crate::dialogue::IntelKind::Rollcall), t: None, k: None, needs: None, text: String::from("When's your next radio check? Don't round down."), w: None, now: None, away: None, guide: None }, crate::dialogue::SamLine { id: String::from("Sqv1"), s: crate::dialogue::SamLineS::Q, q: Some(crate::dialogue::IntelKind::RollcallLive), t: None, k: None, needs: None, text: String::from("That's your radio wanting you. What happens now?"), w: None, now: None, away: None, guide: None }, crate::dialogue::SamLine { id: String::from("Sqv2"), s: crate::dialogue::SamLineS::Q, q: Some(crate::dialogue::IntelKind::RollcallLive), t: None, k: None, needs: None, text: String::from("They're calling names. What if yours stays quiet?"), w: None, now: None, away: None, guide: None }, crate::dialogue::SamLine { id: String::from("Sqn1"), s: crate::dialogue::SamLineS::Q, q: Some(crate::dialogue::IntelKind::Routes), t: None, k: None, needs: None, text: String::from("How many of you tonight, and who walks where?"), w: None, now: None, away: None, guide: None }, crate::dialogue::SamLine { id: String::from("Sqn2"), s: crate::dialogue::SamLineS::Q, q: Some(crate::dialogue::IntelKind::Routes), t: None, k: None, needs: None, text: String::from("Head count. Routes. Don't leave yourself out."), w: None, now: None, away: None, guide: None }, crate::dialogue::SamLine { id: String::from("Sqb1"), s: crate::dialogue::SamLineS::Q, q: Some(crate::dialogue::IntelKind::Blackout), t: None, k: None, needs: None, text: String::from("Lights die up here — what's the drill?"), w: None, now: Some(String::from("Lights die — what's the drill?")), away: None, guide: None }, crate::dialogue::SamLine { id: String::from("Sqb2"), s: crate::dialogue::SamLineS::Q, q: Some(crate::dialogue::IntelKind::Blackout), t: None, k: None, needs: None, text: String::from("Power drops. Who comes running, and how armed?"), w: None, now: None, away: None, guide: None }, crate::dialogue::SamLine { id: String::from("Sqc1"), s: crate::dialogue::SamLineS::Q, q: Some(crate::dialogue::IntelKind::CheckOn), t: None, k: None, needs: None, text: String::from("Someone's off the air. Who goes looking for him?"), w: None, now: None, away: None, guide: None }, crate::dialogue::SamLine { id: String::from("Sqm1"), s: crate::dialogue::SamLineS::Q, q: Some(crate::dialogue::IntelKind::Missing), t: None, k: None, needs: None, text: String::from("Your net's a man short. What happens next?"), w: None, now: None, away: None, guide: None }, crate::dialogue::SamLine { id: String::from("Sql1"), s: crate::dialogue::SamLineS::Q, q: Some(crate::dialogue::IntelKind::LockdownPosts), t: None, k: None, needs: None, text: String::from("You've locked down. Where does everyone dig in?"), w: None, now: None, away: None, guide: None }, crate::dialogue::SamLine { id: String::from("Sql2"), s: crate::dialogue::SamLineS::Q, q: Some(crate::dialogue::IntelKind::Clearing), t: None, k: None, needs: None, text: String::from("They're sweeping rooms. Which one's first?"), w: None, now: None, away: None, guide: None }, crate::dialogue::SamLine { id: String::from("Sqk1"), s: crate::dialogue::SamLineS::Q, q: Some(crate::dialogue::IntelKind::Drive), t: None, k: None, needs: None, text: String::from("The server room. Anybody watching those racks?"), w: None, now: None, away: Some(String::from("What I took. Anybody noticed it's gone?")), guide: None }, crate::dialogue::SamLine { id: String::from("Sk1"), s: crate::dialogue::SamLineS::Key, q: None, t: None, k: None, needs: None, text: String::from("I'll hold on to this."), w: None, now: None, away: None, guide: None }, crate::dialogue::SamLine { id: String::from("Sk2"), s: crate::dialogue::SamLineS::Key, q: None, t: None, k: None, needs: None, text: String::from("Your key. You won't be needing it."), w: None, now: None, away: None, guide: None }, crate::dialogue::SamLine { id: String::from("Sk3"), s: crate::dialogue::SamLineS::Key, q: None, t: None, k: None, needs: None, text: String::from("This comes with me. Saves me a lock."), w: None, now: None, away: None, guide: None }, crate::dialogue::SamLine { id: String::from("Sk4"), s: crate::dialogue::SamLineS::Key, q: None, t: None, k: None, needs: None, text: String::from("Thanks. I'll let myself in."), w: None, now: None, away: None, guide: None }, crate::dialogue::SamLine { id: String::from("Sk5"), s: crate::dialogue::SamLineS::Key, q: None, t: None, k: None, needs: None, text: String::from("Off your belt, into my pocket. Keep still."), w: None, now: None, away: None, guide: None }, crate::dialogue::SamLine { id: String::from("Skh1"), s: crate::dialogue::SamLineS::Key, q: None, t: Some(crate::game::Temperament::Hard), k: None, needs: None, text: String::from("Generous. I was taking it anyway."), w: None, now: None, away: None, guide: None }, crate::dialogue::SamLine { id: String::from("Skn1"), s: crate::dialogue::SamLineS::Key, q: None, t: Some(crate::game::Temperament::Nervy), k: None, needs: None, text: String::from("Easy. Just the key. Keep breathing."), w: None, now: None, away: None, guide: None }, crate::dialogue::SamLine { id: String::from("Sd1"), s: crate::dialogue::SamLineS::Dry, q: None, t: None, k: None, needs: None, text: String::from("Once more, from the top. Humor me."), w: None, now: None, away: None, guide: None }, crate::dialogue::SamLine { id: String::from("Sd2"), s: crate::dialogue::SamLineS::Dry, q: None, t: None, k: None, needs: None, text: String::from("That had better be all of it."), w: None, now: None, away: None, guide: None }, crate::dialogue::SamLine { id: String::from("Sd3"), s: crate::dialogue::SamLineS::Dry, q: None, t: None, k: None, needs: None, text: String::from("If you're holding out, I'll know before you do."), w: None, now: None, away: None, guide: None }, crate::dialogue::SamLine { id: String::from("Sd4"), s: crate::dialogue::SamLineS::Dry, q: None, t: None, k: None, needs: None, text: String::from("Anything you forgot? Now is cheaper than later."), w: None, now: None, away: None, guide: None }, crate::dialogue::SamLine { id: String::from("Sd5"), s: crate::dialogue::SamLineS::Dry, q: None, t: None, k: None, needs: None, text: String::from("You're sure. Think about how sure."), w: None, now: None, away: None, guide: None }, crate::dialogue::SamLine { id: String::from("Sd6"), s: crate::dialogue::SamLineS::Dry, q: None, t: None, k: None, needs: None, text: String::from("Last chance to be interesting."), w: None, now: None, away: None, guide: None }, crate::dialogue::SamLine { id: String::from("Sd7"), s: crate::dialogue::SamLineS::Dry, q: None, t: None, k: None, needs: None, text: String::from("Then we're about done. Any famous last details?"), w: None, now: None, away: None, guide: None }, crate::dialogue::SamLine { id: String::from("Sd8"), s: crate::dialogue::SamLineS::Dry, q: None, t: None, k: None, needs: None, text: String::from("Nothing else? Then you're just heavy now."), w: None, now: None, away: None, guide: None }, crate::dialogue::SamLine { id: String::from("Sdg1"), s: crate::dialogue::SamLineS::Dry, q: None, t: None, k: None, needs: Some(alloc::vec![crate::dialogue::SamNeed::Gun]), text: String::from("The gun's bored. Bore it again and we're finished."), w: None, now: None, away: None, guide: None }, crate::dialogue::SamLine { id: String::from("Sda1"), s: crate::dialogue::SamLineS::Dry, q: None, t: None, k: None, needs: Some(alloc::vec![crate::dialogue::SamNeed::Arm]), text: String::from("My arm's falling asleep. You're next."), w: None, now: None, away: None, guide: None }, crate::dialogue::SamLine { id: String::from("Sdk1"), s: crate::dialogue::SamLineS::Dry, q: None, t: None, k: None, needs: Some(alloc::vec![crate::dialogue::SamNeed::Knife]), text: String::from("The blade's heard enough. Have I?"), w: None, now: None, away: None, guide: None }, crate::dialogue::SamLine { id: String::from("Sdn1"), s: crate::dialogue::SamLineS::Dry, q: None, t: Some(crate::game::Temperament::Nervy), k: None, needs: None, text: String::from("Easy. Same question, smaller words."), w: None, now: None, away: None, guide: None }, crate::dialogue::SamLine { id: String::from("Sdh1"), s: crate::dialogue::SamLineS::Dry, q: None, t: Some(crate::game::Temperament::Hard), k: None, needs: None, text: String::from("Still no? Consistent. I'll give you that."), w: None, now: None, away: None, guide: None }, crate::dialogue::SamLine { id: String::from("Sdl1"), s: crate::dialogue::SamLineS::Dry, q: None, t: None, k: None, needs: Some(alloc::vec![crate::dialogue::SamNeed::HeldLong]), text: String::from("We've been at this a while. Wrap it up for me."), w: Some(1.4), now: None, away: None, guide: None }, crate::dialogue::SamLine { id: String::from("Sdx1"), s: crate::dialogue::SamLineS::Dry, q: None, t: None, k: None, needs: Some(alloc::vec![crate::dialogue::SamNeed::Alarm]), text: String::from("Your friends are close. Be quick, or be quiet for good."), w: None, now: None, away: None, guide: None }],
            sam_mem_by_game: None,
            script_by_talk: JsMap::new(),
            forced_by_talk: JsMap::new(),
            radio_by_game: None,
            ambient_by_game: None,
        }
    }
}

/// (src/game/dialogue.ts:55 KNOWLEDGE: a table nothing writes)
pub fn knowledge() -> Vec<String> { alloc::vec![String::from("none"), String::from("heard"), String::from("glimpsed"), String::from("saw"), String::from("sawHurt")] }

/// (src/game/dialogue.ts:57 BANDS: a table nothing writes)
pub fn bands() -> Vec<String> { alloc::vec![String::from("low"), String::from("mid"), String::from("high")] }

/// (src/game/dialogue.ts:58 TEMPERAMENTS: a table nothing writes)
pub fn temperaments() -> Vec<String> { alloc::vec![String::from("steady"), String::from("nervy"), String::from("hard")] }

/// (src/game/dialogue.ts:60 STAGES: a table nothing writes)
pub fn stages() -> Vec<crate::dialogue::StageKey> { alloc::vec![crate::dialogue::StageKey::Num(1.0), crate::dialogue::StageKey::Num(2.0), crate::dialogue::StageKey::Num(3.0), crate::dialogue::StageKey::X] }

/// (src/game/dialogue.ts:62 BAND_MID)
pub const BAND_MID: f64 = 0.3;

/// (src/game/dialogue.ts:62 BAND_HIGH)
pub const BAND_HIGH: f64 = 0.6;

/// (src/game/dialogue.ts:66 MAX_CHARS)
pub const MAX_CHARS: f64 = 90.0;

/// (src/game/dialogue.ts:66 AIM_CHARS)
pub const AIM_CHARS: f64 = 60.0;

/// (src/game/dialogue.ts:67 INTERROGATE_REACH)
pub const INTERROGATE_REACH: f64 = 2.0;

/// (src/game/dialogue.ts:70 HELD_LONG_SECS)
pub const HELD_LONG_SECS: f64 = 40.0;

/// (src/game/dialogue.ts:70 HOLD_LAPSE_SECS)
pub const HOLD_LAPSE_SECS: f64 = 30.0;

/// (src/game/dialogue.ts:70 ABORT_RATTLE_SECS)
pub const ABORT_RATTLE_SECS: f64 = 60.0;

/// (src/game/dialogue.ts:152 NEEDS: a table nothing writes)
pub fn needs() -> [String; 21] { [String::from("floorUp"), String::from("lockdown"), String::from("episode"), String::from("heardSmall"), String::from("heardShots"), String::from("heardBang"), String::from("bodyFound"), String::from("bodyCalled"), String::from("bodyKnown"), String::from("missingKnown"), String::from("blackout"), String::from("heldLong"), String::from("rollcallLive"), String::from("dazzled"), String::from("sawShot"), String::from("sawDrop"), String::from("alertNow"), String::from("drewOnYou"), String::from("partnerKnown"), String::from("shift"), String::from("office")] }

/// (src/game/dialogue.ts:177 TOKENS: a table nothing writes)
pub fn tokens() -> [String; 23] { [String::from("partner"), String::from("victim"), String::from("body"), String::from("missing"), String::from("room"), String::from("sawWhere"), String::from("heardWhere"), String::from("route"), String::from("partnerRoute"), String::from("post"), String::from("door"), String::from("caller"), String::from("checker"), String::from("checked"), String::from("clearRoom"), String::from("breakerRoom"), String::from("checkEta"), String::from("minutes"), String::from("n"), String::from("up"), String::from("badge"), String::from("code"), String::from("padDoor")] }

/// (src/game/dialogue.ts:180 TOKEN_WORST: a table nothing writes)
pub fn token_worst_table() -> JsMap<String, String> { { let mut t = JsMap::new(); t.set(String::from("partner"), String::from("Lindqvist")); t.set(String::from("victim"), String::from("Lindqvist")); t.set(String::from("body"), String::from("Lindqvist")); t.set(String::from("missing"), String::from("Lindqvist")); t.set(String::from("caller"), String::from("Lindqvist")); t.set(String::from("checker"), String::from("Lindqvist")); t.set(String::from("checked"), String::from("Lindqvist")); t.set(String::from("room"), String::from("the manager's office")); t.set(String::from("sawWhere"), String::from("the manager's office")); t.set(String::from("heardWhere"), String::from("the manager's office")); t.set(String::from("breakerRoom"), String::from("the manager's office")); t.set(String::from("clearRoom"), String::from("the break room side door")); t.set(String::from("route"), String::from("the lobby loop")); t.set(String::from("partnerRoute"), String::from("the lobby loop")); t.set(String::from("post"), String::from("the cubicle door")); t.set(String::from("door"), String::from("the server room door")); t.set(String::from("checkEta"), String::from("in a couple of minutes")); t.set(String::from("minutes"), String::from("fifteen")); t.set(String::from("n"), String::from("three")); t.set(String::from("up"), String::from("three")); t.set(String::from("badge"), String::from("4471")); t.set(String::from("code"), String::from("7391")); t.set(String::from("padDoor"), String::from("the office side door")); t } }

/// (src/game/dialogue.ts:187 TOKEN_WORST_NOW: a table nothing writes)
pub fn token_worst_now() -> crate::dialogue::TOKENWORSTNOW { crate::dialogue::TOKENWORSTNOW { door: Some(String::from("the phone shop back door")), code: None, missing: None, body: None, n: None, up: None, post: None, room: None, route: None, partner: None, victim: None, saw_where: None, heard_where: None, partner_route: None, caller: None, checker: None, checked: None, clear_room: None, breaker_room: None, check_eta: None, minutes: None, badge: None, pad_door: None } }

/// (src/game/dialogue.ts:190 TOKENS_ALWAYS: a table nothing writes)
pub fn tokens_always() -> Vec<String> { alloc::vec![String::from("room"), String::from("route"), String::from("post"), String::from("breakerRoom"), String::from("checkEta"), String::from("n"), String::from("up"), String::from("badge")] }

/// (src/game/dialogue.ts:192 TOKENS_BY_K: a table nothing writes)
pub fn tokens_by_k() -> JsMap<String, Vec<String>> { { let mut t = JsMap::new(); t.set(String::from("none"), Vec::new()); t.set(String::from("heard"), Vec::new()); t.set(String::from("glimpsed"), alloc::vec![String::from("sawWhere")]); t.set(String::from("saw"), alloc::vec![String::from("sawWhere")]); t.set(String::from("sawHurt"), alloc::vec![String::from("victim")]); t } }

/// (src/game/dialogue.ts:209 LINES: a table nothing writes)
pub fn lines() -> Vec<crate::dialogue::Line> { alloc::vec![crate::dialogue::Line { id: String::from("n1s"), k: crate::dialogue::LineK::None, a: None, t: Some(crate::game::Temperament::Steady), s: crate::dialogue::StageKey::Num(1.0), text: String::from("Easy— easy. I'm not moving. Hands are where you can see them."), needs: None, w: None, only: None, now: None, away: None, guide: None, bare: None }, crate::dialogue::Line { id: String::from("n1n"), k: crate::dialogue::LineK::None, a: None, t: Some(crate::game::Temperament::Nervy), s: crate::dialogue::StageKey::Num(1.0), text: String::from("Wh— who— okay. Okay okay okay. Don't. Please don't."), needs: None, w: None, only: None, now: None, away: None, guide: None, bare: None }, crate::dialogue::Line { id: String::from("n1h"), k: crate::dialogue::LineK::None, a: None, t: Some(crate::game::Temperament::Hard), s: crate::dialogue::StageKey::Num(1.0), text: String::from("…That's a bad idea, friend. Mostly for you."), needs: None, w: None, only: None, now: None, away: None, guide: None, bare: None }, crate::dialogue::Line { id: String::from("n1a"), k: crate::dialogue::LineK::None, a: None, t: None, s: crate::dialogue::StageKey::Num(1.0), text: String::from("Where did you— there was nobody there. I checked. There was nobody."), needs: None, w: None, only: None, now: None, away: None, guide: None, bare: None }, crate::dialogue::Line { id: String::from("n1f"), k: crate::dialogue::LineK::None, a: Some(crate::dialogue::LineA::Mid2), t: None, s: crate::dialogue::StageKey::Num(1.0), text: String::from("So it's real. They said weapons out and I thought someone sat on his radio."), needs: Some(alloc::vec![crate::dialogue::Need::FloorUp]), w: None, only: None, now: None, away: None, guide: None, bare: None }, crate::dialogue::Line { id: String::from("n2s"), k: crate::dialogue::LineK::None, a: None, t: Some(crate::game::Temperament::Steady), s: crate::dialogue::StageKey::Num(2.0), text: String::from("I don't know you. I walk {route} and I sign the sheet. That's the whole job."), needs: None, w: None, only: Some(crate::dialogue::LineOnly::Shift), now: None, away: None, guide: None, bare: None }, crate::dialogue::Line { id: String::from("n2n"), k: crate::dialogue::LineK::None, a: None, t: Some(crate::game::Temperament::Nervy), s: crate::dialogue::StageKey::Num(2.0), text: String::from("I haven't seen your face. I don't want to. I'm looking at the floor— see?"), needs: None, w: None, only: None, now: None, away: None, guide: None, bare: None }, crate::dialogue::Line { id: String::from("n2h"), k: crate::dialogue::LineK::None, a: None, t: Some(crate::game::Temperament::Hard), s: crate::dialogue::StageKey::Num(2.0), text: String::from("You want the tour, reception opens at eight."), needs: None, w: None, only: None, now: None, away: Some(String::from("You want the tour, buy a ticket.")), guide: None, bare: None }, crate::dialogue::Line { id: String::from("n2a"), k: crate::dialogue::LineK::None, a: None, t: None, s: crate::dialogue::StageKey::Num(2.0), text: String::from("Whatever you're here for, it isn't me. I'm nights. I'm nobody."), needs: None, w: None, only: Some(crate::dialogue::LineOnly::Shift), now: None, away: None, guide: None, bare: None }, crate::dialogue::Line { id: String::from("n3a"), k: crate::dialogue::LineK::None, a: None, t: None, s: crate::dialogue::StageKey::Num(3.0), text: String::from("That's it. That's all there is. I do nights and I go home."), needs: None, w: None, only: Some(crate::dialogue::LineOnly::Shift), now: None, away: None, guide: None, bare: None }, crate::dialogue::Line { id: String::from("n3n"), k: crate::dialogue::LineK::None, a: None, t: Some(crate::game::Temperament::Nervy), s: crate::dialogue::StageKey::Num(3.0), text: String::from("I don't KNOW anything— I fill the machine and I walk. That's me. That's all of me."), needs: None, w: None, only: Some(crate::dialogue::LineOnly::Office), now: None, away: None, guide: None, bare: None }, crate::dialogue::Line { id: String::from("n3h"), k: crate::dialogue::LineK::None, a: Some(crate::dialogue::LineA::Low), t: Some(crate::game::Temperament::Hard), s: crate::dialogue::StageKey::Num(3.0), text: String::from("Nothing. You get nothing, and you knew that when you grabbed me."), needs: None, w: Some(2.0), only: None, now: None, away: None, guide: None, bare: None }, crate::dialogue::Line { id: String::from("nxa"), k: crate::dialogue::LineK::None, a: None, t: None, s: crate::dialogue::StageKey::X, text: String::from("I told you. I just do nights."), needs: None, w: None, only: Some(crate::dialogue::LineOnly::Shift), now: None, away: None, guide: None, bare: None }, crate::dialogue::Line { id: String::from("nxb"), k: crate::dialogue::LineK::None, a: None, t: None, s: crate::dialogue::StageKey::X, text: String::from("Same as before. I don't know. I still don't know."), needs: None, w: None, only: None, now: None, away: None, guide: None, bare: None }, crate::dialogue::Line { id: String::from("nxn"), k: crate::dialogue::LineK::None, a: None, t: Some(crate::game::Temperament::Nervy), s: crate::dialogue::StageKey::X, text: String::from("Please. My arm. I answered you."), needs: None, w: None, only: None, now: None, away: None, guide: None, bare: None }, crate::dialogue::Line { id: String::from("nxh"), k: crate::dialogue::LineK::None, a: None, t: Some(crate::game::Temperament::Hard), s: crate::dialogue::StageKey::X, text: String::from("Ask again. Same answer."), needs: None, w: None, only: None, now: None, away: None, guide: None, bare: None }, crate::dialogue::Line { id: String::from("h1s"), k: crate::dialogue::LineK::Heard, a: None, t: Some(crate::game::Temperament::Steady), s: crate::dialogue::StageKey::Num(1.0), text: String::from("So that was you, by {heardWhere}. I wrote it off as the building settling."), needs: Some(alloc::vec![crate::dialogue::Need::HeardSmall]), w: None, only: None, now: Some(String::from("So that was you, by {heardWhere}. I wrote it off as nothing.")), away: None, guide: None, bare: None }, crate::dialogue::Line { id: String::from("h1n"), k: crate::dialogue::LineK::Heard, a: None, t: Some(crate::game::Temperament::Nervy), s: crate::dialogue::StageKey::Num(1.0), text: String::from("I knew it. I KNEW I heard something. Nobody ever believes me."), needs: None, w: None, only: None, now: None, away: None, guide: None, bare: None }, crate::dialogue::Line { id: String::from("h1h"), k: crate::dialogue::LineK::Heard, a: None, t: Some(crate::game::Temperament::Hard), s: crate::dialogue::StageKey::Num(1.0), text: String::from("Should've trusted my ears and come looking for you."), needs: None, w: None, only: None, now: None, away: None, guide: None, bare: None }, crate::dialogue::Line { id: String::from("h1a"), k: crate::dialogue::LineK::Heard, a: None, t: None, s: crate::dialogue::StageKey::Num(1.0), text: String::from("Something's been off all night. So it's a someone. Fine."), needs: None, w: None, only: Some(crate::dialogue::LineOnly::Shift), now: None, away: None, guide: None, bare: None }, crate::dialogue::Line { id: String::from("h1g"), k: crate::dialogue::LineK::Heard, a: None, t: None, s: crate::dialogue::StageKey::Num(1.0), text: String::from("Those were shots. That was you? …Who's down. Tell me who's down."), needs: Some(alloc::vec![crate::dialogue::Need::HeardShots]), w: Some(2.0), only: None, now: None, away: None, guide: None, bare: None }, crate::dialogue::Line { id: String::from("h1b"), k: crate::dialogue::LineK::Heard, a: None, t: None, s: crate::dialogue::StageKey::Num(1.0), text: String::from("They found {body}. That was you. And now it's my turn, is it."), needs: Some(alloc::vec![crate::dialogue::Need::BodyKnown]), w: Some(2.0), only: None, now: None, away: None, guide: None, bare: None }, crate::dialogue::Line { id: String::from("h2s"), k: crate::dialogue::LineK::Heard, a: None, t: Some(crate::game::Temperament::Steady), s: crate::dialogue::StageKey::Num(2.0), text: String::from("{Partner} is going to love this. I told him it was the vending machine."), needs: Some(alloc::vec![crate::dialogue::Need::PartnerKnown]), w: None, only: Some(crate::dialogue::LineOnly::Office), now: None, away: None, guide: None, bare: None }, crate::dialogue::Line { id: String::from("h2n"), k: crate::dialogue::LineK::Heard, a: None, t: Some(crate::game::Temperament::Nervy), s: crate::dialogue::StageKey::Num(2.0), text: String::from("I called it in as nothing. I SAID nothing. They'll put that on me."), needs: None, w: None, only: None, now: None, away: None, guide: None, bare: None }, crate::dialogue::Line { id: String::from("h2h"), k: crate::dialogue::LineK::Heard, a: None, t: Some(crate::game::Temperament::Hard), s: crate::dialogue::StageKey::Num(2.0), text: String::from("You're loud, whoever you are. Loud gets found."), needs: None, w: None, only: None, now: None, away: None, guide: None, bare: None }, crate::dialogue::Line { id: String::from("h2a"), k: crate::dialogue::LineK::Heard, a: None, t: None, s: crate::dialogue::StageKey::Num(2.0), text: String::from("I never saw you. Noises, that's all — and I'm not paid to chase noises."), needs: None, w: None, only: Some(crate::dialogue::LineOnly::Shift), now: None, away: None, guide: None, bare: None }, crate::dialogue::Line { id: String::from("h3a"), k: crate::dialogue::LineK::Heard, a: None, t: None, s: crate::dialogue::StageKey::Num(3.0), text: String::from("I heard things. I didn't see things. That's the truth and it's all of it."), needs: None, w: None, only: None, now: None, away: None, guide: None, bare: None }, crate::dialogue::Line { id: String::from("h3h"), k: crate::dialogue::LineK::Heard, a: Some(crate::dialogue::LineA::Low), t: Some(crate::game::Temperament::Hard), s: crate::dialogue::StageKey::Num(3.0), text: String::from("You've had your answer. It was no."), needs: None, w: Some(2.0), only: None, now: None, away: None, guide: None, bare: None }, crate::dialogue::Line { id: String::from("hxa"), k: crate::dialogue::LineK::Heard, a: None, t: None, s: crate::dialogue::StageKey::X, text: String::from("Noises. All I ever had was noises."), needs: None, w: None, only: None, now: None, away: None, guide: None, bare: None }, crate::dialogue::Line { id: String::from("hxb"), k: crate::dialogue::LineK::Heard, a: None, t: None, s: crate::dialogue::StageKey::X, text: String::from("I've said it. Ears, not eyes."), needs: None, w: None, only: None, now: None, away: None, guide: None, bare: None }, crate::dialogue::Line { id: String::from("g1s"), k: crate::dialogue::LineK::Glimpsed, a: None, t: Some(crate::game::Temperament::Steady), s: crate::dialogue::StageKey::Num(1.0), text: String::from("Something moved, over in {sawWhere}. I told myself it was the light."), needs: None, w: None, only: None, now: None, away: None, guide: None, bare: None }, crate::dialogue::Line { id: String::from("g1n"), k: crate::dialogue::LineK::Glimpsed, a: None, t: Some(crate::game::Temperament::Nervy), s: crate::dialogue::StageKey::Num(1.0), text: String::from("You were right there. I looked RIGHT at you. Oh no. No no no."), needs: None, w: None, only: None, now: None, away: None, guide: None, bare: None }, crate::dialogue::Line { id: String::from("g1h"), k: crate::dialogue::LineK::Glimpsed, a: None, t: Some(crate::game::Temperament::Hard), s: crate::dialogue::StageKey::Num(1.0), text: String::from("Next time the torch stays on you."), needs: None, w: None, only: None, now: Some(String::from("Next time the light stays on you.")), away: None, guide: None, bare: None }, crate::dialogue::Line { id: String::from("g1a"), k: crate::dialogue::LineK::Glimpsed, a: None, t: None, s: crate::dialogue::StageKey::Num(1.0), text: String::from("I saw— I don't know what I saw. You, I suppose. In {sawWhere}."), needs: None, w: None, only: None, now: None, away: None, guide: None, bare: None }, crate::dialogue::Line { id: String::from("g2a"), k: crate::dialogue::LineK::Glimpsed, a: None, t: None, s: crate::dialogue::StageKey::Num(2.0), text: String::from("A shape. That's what goes in the book. A shape, about your size."), needs: None, w: None, only: Some(crate::dialogue::LineOnly::Shift), now: None, away: None, guide: None, bare: None }, crate::dialogue::Line { id: String::from("g2n"), k: crate::dialogue::LineK::Glimpsed, a: Some(crate::dialogue::LineA::Mid2), t: Some(crate::game::Temperament::Nervy), s: crate::dialogue::StageKey::Num(2.0), text: String::from("I nearly called it. Thumb on the button. I nearly— why didn't I."), needs: None, w: None, only: None, now: None, away: None, guide: None, bare: None }, crate::dialogue::Line { id: String::from("g2h"), k: crate::dialogue::LineK::Glimpsed, a: None, t: Some(crate::game::Temperament::Hard), s: crate::dialogue::StageKey::Num(2.0), text: String::from("Half a second sooner with the light and we'd be talking the other way round."), needs: None, w: None, only: None, now: None, away: None, guide: None, bare: None }, crate::dialogue::Line { id: String::from("g3a"), k: crate::dialogue::LineK::Glimpsed, a: None, t: None, s: crate::dialogue::StageKey::Num(3.0), text: String::from("Half a second of you in the dark. That's everything I've got."), needs: None, w: None, only: None, now: None, away: None, guide: None, bare: None }, crate::dialogue::Line { id: String::from("g3h"), k: crate::dialogue::LineK::Glimpsed, a: Some(crate::dialogue::LineA::Low), t: Some(crate::game::Temperament::Hard), s: crate::dialogue::StageKey::Num(3.0), text: String::from("No. Work it out yourself."), needs: None, w: Some(2.0), only: None, now: None, away: None, guide: None, bare: None }, crate::dialogue::Line { id: String::from("gxa"), k: crate::dialogue::LineK::Glimpsed, a: None, t: None, s: crate::dialogue::StageKey::X, text: String::from("A shape. I keep telling you. A shape."), needs: None, w: None, only: None, now: None, away: None, guide: None, bare: None }, crate::dialogue::Line { id: String::from("gxb"), k: crate::dialogue::LineK::Glimpsed, a: None, t: None, s: crate::dialogue::StageKey::X, text: String::from("That's twice you've asked and twice it's the same."), needs: None, w: None, only: None, now: None, away: None, guide: None, bare: None }, crate::dialogue::Line { id: String::from("s1s"), k: crate::dialogue::LineK::Saw, a: None, t: Some(crate::game::Temperament::Steady), s: crate::dialogue::StageKey::Num(1.0), text: String::from("Everyone on this floor is looking for you. You do know that."), needs: None, w: None, only: None, now: Some(String::from("Every man here is looking for you. You do know that.")), away: None, guide: None, bare: None }, crate::dialogue::Line { id: String::from("s1n"), k: crate::dialogue::LineK::Saw, a: None, t: Some(crate::game::Temperament::Nervy), s: crate::dialogue::StageKey::Num(1.0), text: String::from("It's you. From {sawWhere}. Okay. Okay— I'm not doing anything. Look. Nothing."), needs: None, w: None, only: None, now: None, away: None, guide: None, bare: None }, crate::dialogue::Line { id: String::from("s1p"), k: crate::dialogue::LineK::Saw, a: Some(crate::dialogue::LineA::High), t: Some(crate::game::Temperament::Nervy), s: crate::dialogue::StageKey::Num(1.0), text: String::from("They're coming— all of them— you heard the radio same as me—"), needs: None, w: Some(2.0), only: None, now: None, away: None, guide: None, bare: None }, crate::dialogue::Line { id: String::from("s1h"), k: crate::dialogue::LineK::Saw, a: None, t: Some(crate::game::Temperament::Hard), s: crate::dialogue::StageKey::Num(1.0), text: String::from("You. From {sawWhere}. You don't walk out of this building."), needs: None, w: None, only: None, now: Some(String::from("You. From {sawWhere}. You don't walk out of here.")), away: None, guide: None, bare: None }, crate::dialogue::Line { id: String::from("s1a"), k: crate::dialogue::LineK::Saw, a: None, t: None, s: crate::dialogue::StageKey::Num(1.0), text: String::from("I had you. In {sawWhere}. I had you and I lost you."), needs: None, w: None, only: None, now: None, away: None, guide: None, bare: None }, crate::dialogue::Line { id: String::from("s1d"), k: crate::dialogue::LineK::Saw, a: None, t: None, s: crate::dialogue::StageKey::Num(1.0), text: String::from("I put rounds your way and you still got behind me. …Fine. Fine."), needs: Some(alloc::vec![crate::dialogue::Need::DrewOnYou]), w: Some(2.0), only: None, now: None, away: None, guide: None, bare: None }, crate::dialogue::Line { id: String::from("s2s"), k: crate::dialogue::LineK::Saw, a: None, t: Some(crate::game::Temperament::Steady), s: crate::dialogue::StageKey::Num(2.0), text: String::from("Let go now and I count to sixty before I touch the radio. That offer is real."), needs: None, w: None, only: None, now: None, away: None, guide: None, bare: None }, crate::dialogue::Line { id: String::from("s2n"), k: crate::dialogue::LineK::Saw, a: None, t: Some(crate::game::Temperament::Nervy), s: crate::dialogue::StageKey::Num(2.0), text: String::from("What do you want— just say it— say it and I'll say yes—"), needs: None, w: None, only: None, now: None, away: None, guide: None, bare: None }, crate::dialogue::Line { id: String::from("s2h"), k: crate::dialogue::LineK::Saw, a: None, t: Some(crate::game::Temperament::Hard), s: crate::dialogue::StageKey::Num(2.0), text: String::from("Squeeze all you like. I've had worse off my brother."), needs: None, w: None, only: None, now: None, away: None, guide: None, bare: None }, crate::dialogue::Line { id: String::from("s2a"), k: crate::dialogue::LineK::Saw, a: None, t: None, s: crate::dialogue::StageKey::Num(2.0), text: String::from("You've got maybe a minute before someone walks round that corner. Use it."), needs: None, w: None, only: None, now: None, away: None, guide: None, bare: None }, crate::dialogue::Line { id: String::from("s3a"), k: crate::dialogue::LineK::Saw, a: None, t: None, s: crate::dialogue::StageKey::Num(3.0), text: String::from("That's everything. Do what you're going to do."), needs: None, w: None, only: None, now: None, away: None, guide: None, bare: None }, crate::dialogue::Line { id: String::from("s3h"), k: crate::dialogue::LineK::Saw, a: Some(crate::dialogue::LineA::Low), t: Some(crate::game::Temperament::Hard), s: crate::dialogue::StageKey::Num(3.0), text: String::from("From me? Nothing. Not a word."), needs: None, w: Some(2.0), only: None, now: None, away: None, guide: None, bare: None }, crate::dialogue::Line { id: String::from("sxa"), k: crate::dialogue::LineK::Saw, a: None, t: None, s: crate::dialogue::StageKey::X, text: String::from("I've said it all. Twice."), needs: None, w: None, only: None, now: None, away: None, guide: None, bare: None }, crate::dialogue::Line { id: String::from("sxb"), k: crate::dialogue::LineK::Saw, a: None, t: None, s: crate::dialogue::StageKey::X, text: String::from("Still here. Still nothing new."), needs: None, w: None, only: None, now: None, away: None, guide: None, bare: None }, crate::dialogue::Line { id: String::from("k1s"), k: crate::dialogue::LineK::SawHurt, a: None, t: Some(crate::game::Temperament::Steady), s: crate::dialogue::StageKey::Num(1.0), text: String::from("You shot {victim}. I watched you do it. Don't talk to me like we're talking."), needs: Some(alloc::vec![crate::dialogue::Need::SawShot]), w: None, only: None, now: None, away: None, guide: None, bare: None }, crate::dialogue::Line { id: String::from("k1n"), k: crate::dialogue::LineK::SawHurt, a: None, t: Some(crate::game::Temperament::Nervy), s: crate::dialogue::StageKey::Num(1.0), text: String::from("{Victim}— is he— you didn't have to— he'd have let you walk past—"), needs: Some(alloc::vec![crate::dialogue::Need::SawShot]), w: None, only: None, now: None, away: None, guide: None, bare: None }, crate::dialogue::Line { id: String::from("k1h"), k: crate::dialogue::LineK::SawHurt, a: None, t: Some(crate::game::Temperament::Hard), s: crate::dialogue::StageKey::Num(1.0), text: String::from("{Victim} had a name. Remember it. Somebody is going to ask you for it."), needs: Some(alloc::vec![crate::dialogue::Need::SawShot]), w: None, only: None, now: None, away: None, guide: None, bare: Some(String::from("He had a name. Somebody is going to make you learn it.")) }, crate::dialogue::Line { id: String::from("k1a"), k: crate::dialogue::LineK::SawHurt, a: None, t: None, s: crate::dialogue::StageKey::Num(1.0), text: String::from("I saw you shoot {victim}. I saw it. Whatever you say next, I saw it."), needs: Some(alloc::vec![crate::dialogue::Need::SawShot]), w: None, only: None, now: None, away: None, guide: None, bare: Some(String::from("I saw you shoot him. I saw it. Whatever you say next, I saw it.")) }, crate::dialogue::Line { id: String::from("k1d"), k: crate::dialogue::LineK::SawHurt, a: None, t: None, s: crate::dialogue::StageKey::Num(1.0), text: String::from("Is {victim} breathing? …Tell me he's breathing."), needs: Some(alloc::vec![crate::dialogue::Need::SawDrop]), w: None, only: None, now: None, away: None, guide: None, bare: Some(String::from("Is he breathing? …Tell me he's breathing.")) }, crate::dialogue::Line { id: String::from("k1e"), k: crate::dialogue::LineK::SawHurt, a: None, t: Some(crate::game::Temperament::Hard), s: crate::dialogue::StageKey::Num(1.0), text: String::from("I saw what you did to {victim}. Try it from the front some time."), needs: Some(alloc::vec![crate::dialogue::Need::SawDrop]), w: None, only: None, now: None, away: None, guide: None, bare: Some(String::from("I saw what you did to him. Try it from the front some time.")) }, crate::dialogue::Line { id: String::from("k1g"), k: crate::dialogue::LineK::SawHurt, a: None, t: None, s: crate::dialogue::StageKey::Num(1.0), text: String::from("I saw what you did to {victim}. Get your arm off me."), needs: None, w: Some(0.6), only: None, now: None, away: None, guide: None, bare: Some(String::from("I saw what you did to him. Get your arm off me.")) }, crate::dialogue::Line { id: String::from("k2s"), k: crate::dialogue::LineK::SawHurt, a: None, t: Some(crate::game::Temperament::Steady), s: crate::dialogue::StageKey::Num(2.0), text: String::from("What do you want from me? You've shown me what you are."), needs: None, w: None, only: None, now: None, away: None, guide: None, bare: None }, crate::dialogue::Line { id: String::from("k2n"), k: crate::dialogue::LineK::SawHurt, a: None, t: Some(crate::game::Temperament::Nervy), s: crate::dialogue::StageKey::Num(2.0), text: String::from("Okay! Okay. Anything. Just— not like {victim}. Please. Not like that."), needs: None, w: None, only: None, now: None, away: None, guide: None, bare: None }, crate::dialogue::Line { id: String::from("k2h"), k: crate::dialogue::LineK::SawHurt, a: None, t: Some(crate::game::Temperament::Hard), s: crate::dialogue::StageKey::Num(2.0), text: String::from("Every man on this floor heard that. They're not coming to talk."), needs: Some(alloc::vec![crate::dialogue::Need::SawShot]), w: None, only: None, now: Some(String::from("Every man here heard that. They're not coming to talk.")), away: None, guide: None, bare: None }, crate::dialogue::Line { id: String::from("k2i"), k: crate::dialogue::LineK::SawHurt, a: None, t: Some(crate::game::Temperament::Hard), s: crate::dialogue::StageKey::Num(2.0), text: String::from("You put {victim} down in front of me and now you want a chat. No."), needs: None, w: None, only: None, now: None, away: None, guide: None, bare: Some(String::from("You put him down in front of me and now you want a chat. No.")) }, crate::dialogue::Line { id: String::from("k2a"), k: crate::dialogue::LineK::SawHurt, a: None, t: None, s: crate::dialogue::StageKey::Num(2.0), text: String::from("{Victim} first, then me — that the plan? Then get on with it."), needs: None, w: None, only: None, now: None, away: None, guide: None, bare: Some(String::from("Him first, then me — that the plan? Then get on with it.")) }, crate::dialogue::Line { id: String::from("k3a"), k: crate::dialogue::LineK::SawHurt, a: None, t: None, s: crate::dialogue::StageKey::Num(3.0), text: String::from("You've had all I've got. The rest you took off {victim}."), needs: None, w: None, only: None, now: None, away: None, guide: None, bare: Some(String::from("You've had all I've got. The rest you took off him.")) }, crate::dialogue::Line { id: String::from("k3h"), k: crate::dialogue::LineK::SawHurt, a: None, t: Some(crate::game::Temperament::Hard), s: crate::dialogue::StageKey::Num(3.0), text: String::from("No."), needs: None, w: Some(2.0), only: None, now: None, away: None, guide: None, bare: None }, crate::dialogue::Line { id: String::from("kxa"), k: crate::dialogue::LineK::SawHurt, a: None, t: Some(crate::game::Temperament::Nervy), s: crate::dialogue::StageKey::X, text: String::from("Please. I answered. Please."), needs: None, w: None, only: None, now: None, away: None, guide: None, bare: None }, crate::dialogue::Line { id: String::from("kxb"), k: crate::dialogue::LineK::SawHurt, a: None, t: None, s: crate::dialogue::StageKey::X, text: String::from("Nothing else. There's nothing else. Ask {victim}— oh. Right."), needs: None, w: None, only: None, now: None, away: None, guide: None, bare: Some(String::from("Nothing else. There's nothing else. Ask him— oh. Right.")) }, crate::dialogue::Line { id: String::from("kxs"), k: crate::dialogue::LineK::SawHurt, a: None, t: None, s: crate::dialogue::StageKey::X, text: String::from("I've nothing left to tell you and no reason left to lie to you."), needs: None, w: None, only: None, now: None, away: None, guide: None, bare: None }, crate::dialogue::Line { id: String::from("kxh"), k: crate::dialogue::LineK::SawHurt, a: None, t: Some(crate::game::Temperament::Hard), s: crate::dialogue::StageKey::X, text: String::from("Still no."), needs: None, w: None, only: None, now: None, away: None, guide: None, bare: None }, crate::dialogue::Line { id: String::from("f2m"), k: crate::dialogue::LineK::Empty, a: None, t: Some(crate::game::Temperament::Steady), s: crate::dialogue::StageKey::Num(2.0), text: String::from("{Missing} hasn't answered his radio. That was you as well, wasn't it."), needs: Some(alloc::vec![crate::dialogue::Need::MissingKnown]), w: Some(1.2), only: None, now: None, away: None, guide: None, bare: None }, crate::dialogue::Line { id: String::from("f2l"), k: crate::dialogue::LineK::Empty, a: None, t: Some(crate::game::Temperament::Hard), s: crate::dialogue::StageKey::Num(2.0), text: String::from("Hear that? Pairs and doors. You're in a box now."), needs: Some(alloc::vec![crate::dialogue::Need::Lockdown]), w: None, only: None, now: None, away: None, guide: None, bare: None }, crate::dialogue::Line { id: String::from("f2k"), k: crate::dialogue::LineK::Empty, a: None, t: None, s: crate::dialogue::StageKey::Num(2.0), text: String::from("It's locked down. Nobody walks alone and every door gets pulled to. Good luck."), needs: Some(alloc::vec![crate::dialogue::Need::Lockdown]), w: None, only: None, now: None, away: None, guide: None, bare: None }, crate::dialogue::Line { id: String::from("f2b"), k: crate::dialogue::LineK::Empty, a: None, t: Some(crate::game::Temperament::Hard), s: crate::dialogue::StageKey::Num(2.0), text: String::from("I heard them call {body} in. I'm telling you nothing."), needs: Some(alloc::vec![crate::dialogue::Need::BodyCalled]), w: None, only: None, now: None, away: None, guide: None, bare: None }, crate::dialogue::Line { id: String::from("f2c"), k: crate::dialogue::LineK::Empty, a: None, t: Some(crate::game::Temperament::Steady), s: crate::dialogue::StageKey::Num(2.0), text: String::from("After {body} they said weapons out. Should've listened harder."), needs: Some(alloc::vec![crate::dialogue::Need::BodyCalled]), w: None, only: None, now: None, away: None, guide: None, bare: None }, crate::dialogue::Line { id: String::from("f2f"), k: crate::dialogue::LineK::Empty, a: None, t: None, s: crate::dialogue::StageKey::Num(2.0), text: String::from("I stood over {body} not long ago. I know exactly what you are."), needs: Some(alloc::vec![crate::dialogue::Need::BodyFound]), w: None, only: None, now: None, away: None, guide: None, bare: None }, crate::dialogue::Line { id: String::from("f2o"), k: crate::dialogue::LineK::Empty, a: None, t: None, s: crate::dialogue::StageKey::Num(2.0), text: String::from("The lights too? …Course it was you."), needs: Some(alloc::vec![crate::dialogue::Need::Blackout]), w: None, only: None, now: None, away: None, guide: None, bare: None }, crate::dialogue::Line { id: String::from("f2d"), k: crate::dialogue::LineK::Empty, a: None, t: Some(crate::game::Temperament::Nervy), s: crate::dialogue::StageKey::Num(2.0), text: String::from("I still can't see straight. That flash— that was you too, wasn't it—"), needs: Some(alloc::vec![crate::dialogue::Need::Dazzled]), w: None, only: None, now: None, away: None, guide: None, bare: None }, crate::dialogue::Line { id: String::from("f2r"), k: crate::dialogue::LineK::Empty, a: None, t: None, s: crate::dialogue::StageKey::Num(2.0), text: String::from("That's the check going now. I don't answer it, they come down {route}."), needs: Some(alloc::vec![crate::dialogue::Need::RollcallLive]), w: Some(2.0), only: Some(crate::dialogue::LineOnly::Shift), now: None, away: None, guide: None, bare: None }, crate::dialogue::Line { id: String::from("fxl"), k: crate::dialogue::LineK::Empty, a: None, t: Some(crate::game::Temperament::Steady), s: crate::dialogue::StageKey::X, text: String::from("My arm's gone dead. Whatever this is, finish it."), needs: Some(alloc::vec![crate::dialogue::Need::HeldLong]), w: None, only: None, now: None, away: None, guide: None, bare: None }, crate::dialogue::Line { id: String::from("fxn"), k: crate::dialogue::LineK::Empty, a: None, t: Some(crate::game::Temperament::Nervy), s: crate::dialogue::StageKey::X, text: String::from("I can't feel my hand. Please. I've been good. I've answered everything."), needs: Some(alloc::vec![crate::dialogue::Need::HeldLong]), w: None, only: None, now: None, away: None, guide: None, bare: None }, crate::dialogue::Line { id: String::from("fxe"), k: crate::dialogue::LineK::Empty, a: Some(crate::dialogue::LineA::High), t: None, s: crate::dialogue::StageKey::X, text: String::from("You're running out of floor. You know that. I can hear it in you."), needs: None, w: None, only: None, now: None, away: None, guide: None, bare: None }] }

/// (src/game/dialogue.ts:309 INTEL_KINDS: a table nothing writes)
pub fn intel_kinds() -> [String; 11] { [String::from("checkOn"), String::from("clearing"), String::from("rollcallLive"), String::from("blackout"), String::from("door"), String::from("code"), String::from("rollcall"), String::from("lockdownPosts"), String::from("routes"), String::from("missing"), String::from("drive")] }

/// (src/game/dialogue.ts:314 SHAPES: a table nothing writes)
pub fn shapes() -> Vec<String> { alloc::vec![String::from("pedant"), String::from("selfown"), String::from("folder"), String::from("ramble"), String::from("fan"), String::from("stoic"), String::from("con"), String::from("game"), String::from("menace")] }

/// (src/game/dialogue.ts:319 INTEL_TEXT: a table nothing writes)
pub fn intel_text_table() -> JsMap<String, crate::dialogue::INTELTEXTValue> { { let mut t = JsMap::new(); t.set(String::from("checkOn"), crate::dialogue::INTELTEXTValue { steady: None, nervy: Some(String::from("{Checker}'s out looking for {checked}— he went quiet— he'll walk right into us—")), hard: Some(String::from("{Checker}'s walking {checked}'s route. This way. Soon.")), under: Some(String::from("{Checker}'s walking {checked}'s route — he went quiet. He'll come past here.")), pedant: None, selfown: None, folder: None, ramble: None, fan: None, stoic: None, con: None, game: None, menace: None, steady_none: None, steady_heard: None, steady_glimpsed: None, steady_saw: None, steady_saw_hurt: None, nervy_none: None, nervy_heard: None, nervy_glimpsed: None, nervy_saw: None, nervy_saw_hurt: None, hard_none: None, hard_heard: None, hard_glimpsed: None, hard_saw: None, hard_saw_hurt: None, __none: None, __heard: None, __glimpsed: None, __saw: None, __saw_hurt: None, pedant_steady: None, pedant_nervy: None, pedant_hard: None, selfown_steady: None, selfown_nervy: None, selfown_hard: None, folder_steady: None, folder_nervy: None, folder_hard: None, ramble_steady: None, ramble_nervy: None, ramble_hard: None, fan_steady: None, fan_nervy: None, fan_hard: None, stoic_steady: None, stoic_nervy: None, stoic_hard: None, con_steady: None, con_nervy: None, con_hard: None, game_steady: None, game_nervy: None, game_hard: None, menace_steady: None, menace_nervy: None, menace_hard: None }); t.set(String::from("clearing"), crate::dialogue::INTELTEXTValue { steady: None, nervy: None, hard: Some(String::from("Rooms are being cleared. {ClearRoom} first. Then this one.")), under: Some(String::from("They're clearing rooms off your last position. {ClearRoom} first.")), pedant: None, selfown: None, folder: None, ramble: None, fan: None, stoic: None, con: None, game: None, menace: None, steady_none: None, steady_heard: None, steady_glimpsed: None, steady_saw: None, steady_saw_hurt: None, nervy_none: None, nervy_heard: None, nervy_glimpsed: None, nervy_saw: None, nervy_saw_hurt: None, hard_none: None, hard_heard: None, hard_glimpsed: None, hard_saw: None, hard_saw_hurt: None, __none: None, __heard: None, __glimpsed: None, __saw: None, __saw_hurt: None, pedant_steady: None, pedant_nervy: None, pedant_hard: None, selfown_steady: None, selfown_nervy: None, selfown_hard: None, folder_steady: None, folder_nervy: None, folder_hard: None, ramble_steady: None, ramble_nervy: None, ramble_hard: None, fan_steady: None, fan_nervy: None, fan_hard: None, stoic_steady: None, stoic_nervy: None, stoic_hard: None, con_steady: None, con_nervy: None, con_hard: None, game_steady: None, game_nervy: None, game_hard: None, menace_steady: None, menace_nervy: None, menace_hard: None }); t.set(String::from("rollcallLive"), crate::dialogue::INTELTEXTValue { steady: None, nervy: None, hard: Some(String::from("That's the check. I don't answer, {caller} sends a man. Here.")), under: Some(String::from("That's a check now. No answer from a man and {caller} sends someone round.")), pedant: None, selfown: None, folder: None, ramble: None, fan: None, stoic: None, con: None, game: None, menace: None, steady_none: None, steady_heard: None, steady_glimpsed: None, steady_saw: None, steady_saw_hurt: None, nervy_none: None, nervy_heard: None, nervy_glimpsed: None, nervy_saw: None, nervy_saw_hurt: None, hard_none: None, hard_heard: None, hard_glimpsed: None, hard_saw: None, hard_saw_hurt: None, __none: None, __heard: None, __glimpsed: None, __saw: None, __saw_hurt: None, pedant_steady: None, pedant_nervy: None, pedant_hard: None, selfown_steady: None, selfown_nervy: None, selfown_hard: None, folder_steady: None, folder_nervy: None, folder_hard: None, ramble_steady: None, ramble_nervy: None, ramble_hard: None, fan_steady: None, fan_nervy: None, fan_hard: None, stoic_steady: None, stoic_nervy: None, stoic_hard: None, con_steady: None, con_nervy: None, con_hard: None, game_steady: None, game_nervy: None, game_hard: None, menace_steady: None, menace_nervy: None, menace_hard: None }); t.set(String::from("blackout"), crate::dialogue::INTELTEXTValue { steady: None, nervy: None, hard: Some(String::from("Lights out means a man at the breaker, {breakerRoom}. Armed.")), under: Some(String::from("Lights die, one of us goes to the breaker in {breakerRoom} — armed.")), pedant: None, selfown: None, folder: None, ramble: None, fan: None, stoic: None, con: None, game: None, menace: None, steady_none: None, steady_heard: None, steady_glimpsed: None, steady_saw: None, steady_saw_hurt: None, nervy_none: None, nervy_heard: None, nervy_glimpsed: None, nervy_saw: None, nervy_saw_hurt: None, hard_none: None, hard_heard: None, hard_glimpsed: None, hard_saw: None, hard_saw_hurt: None, __none: None, __heard: None, __glimpsed: None, __saw: None, __saw_hurt: None, pedant_steady: None, pedant_nervy: None, pedant_hard: None, selfown_steady: None, selfown_nervy: None, selfown_hard: None, folder_steady: None, folder_nervy: None, folder_hard: None, ramble_steady: None, ramble_nervy: None, ramble_hard: None, fan_steady: None, fan_nervy: None, fan_hard: None, stoic_steady: None, stoic_nervy: None, stoic_hard: None, con_steady: None, con_nervy: None, con_hard: None, game_steady: None, game_nervy: None, game_hard: None, menace_steady: None, menace_nervy: None, menace_hard: None }); t.set(String::from("door"), crate::dialogue::INTELTEXTValue { steady: None, nervy: None, hard: Some(String::from("{Door} is keyed. We all carry one. Take mine and get out.")), under: Some(String::from("{Door} is on a key. We all carry one. Mine's on my belt.")), pedant: None, selfown: None, folder: None, ramble: None, fan: Some(String::from("{Door}'s keyed— key's on my belt. Do yours open anything? No?")), stoic: None, con: Some(String::from("{Door}'s keyed; every belt carries one. Here— inspect mine.")), game: None, menace: None, steady_none: None, steady_heard: None, steady_glimpsed: None, steady_saw: None, steady_saw_hurt: None, nervy_none: None, nervy_heard: None, nervy_glimpsed: None, nervy_saw: None, nervy_saw_hurt: None, hard_none: None, hard_heard: None, hard_glimpsed: None, hard_saw: None, hard_saw_hurt: None, __none: None, __heard: None, __glimpsed: None, __saw: None, __saw_hurt: None, pedant_steady: None, pedant_nervy: None, pedant_hard: None, selfown_steady: Some(String::from("…Belt. The key's on my belt. Well played, I suppose.")), selfown_nervy: Some(String::from("…belt. It's on my belt. Please don't make it a whole thing.")), selfown_hard: None, folder_steady: None, folder_nervy: None, folder_hard: None, ramble_steady: None, ramble_nervy: None, ramble_hard: None, fan_steady: None, fan_nervy: None, fan_hard: None, stoic_steady: None, stoic_nervy: None, stoic_hard: None, con_steady: None, con_nervy: None, con_hard: None, game_steady: None, game_nervy: None, game_hard: None, menace_steady: None, menace_nervy: None, menace_hard: None }); t.set(String::from("code"), crate::dialogue::INTELTEXTValue { steady: None, nervy: Some(String::from("The pad by {padDoor}— it's {code}— {code}— don't make me say it twice—")), hard: Some(String::from("{PadDoor}: {code} on the pad. Now you know what I know.")), under: Some(String::from("{PadDoor} is on a pad — {code}. Same four digits for all of us.")), pedant: None, selfown: None, folder: None, ramble: None, fan: Some(String::from("The pad on {padDoor} is {code}— do YOU get codes, or is it all wire?")), stoic: None, con: Some(String::from("{PadDoor} takes {code} on the pad, sir. Logged monthly, changed never.")), game: None, menace: None, steady_none: None, steady_heard: None, steady_glimpsed: None, steady_saw: None, steady_saw_hurt: None, nervy_none: None, nervy_heard: None, nervy_glimpsed: None, nervy_saw: None, nervy_saw_hurt: None, hard_none: None, hard_heard: None, hard_glimpsed: None, hard_saw: None, hard_saw_hurt: None, __none: None, __heard: None, __glimpsed: None, __saw: None, __saw_hurt: None, pedant_steady: None, pedant_nervy: None, pedant_hard: None, selfown_steady: None, selfown_nervy: None, selfown_hard: None, folder_steady: None, folder_nervy: None, folder_hard: None, ramble_steady: None, ramble_nervy: None, ramble_hard: None, fan_steady: None, fan_nervy: None, fan_hard: None, stoic_steady: None, stoic_nervy: None, stoic_hard: None, con_steady: None, con_nervy: None, con_hard: None, game_steady: None, game_nervy: None, game_hard: None, menace_steady: None, menace_nervy: None, menace_hard: None }); t.set(String::from("rollcall"), crate::dialogue::INTELTEXTValue { steady: None, nervy: Some(String::from("A check— {checkEta}— I don't answer, somebody comes looking.")), hard: Some(String::from("Radio check {checkEta}. Silence gets a man sent. Do the sums.")), under: Some(String::from("Radio check {checkEta}. Miss it, they send a man down your route.")), pedant: None, selfown: None, folder: None, ramble: None, fan: Some(String::from("Radio check's {checkEta}— miss it, they send someone. Do YOU have checks?")), stoic: None, con: Some(String::from("Radio check {checkEta}, sir. No answer, a man walks that route. By the book.")), game: None, menace: None, steady_none: None, steady_heard: None, steady_glimpsed: None, steady_saw: None, steady_saw_hurt: None, nervy_none: None, nervy_heard: None, nervy_glimpsed: None, nervy_saw: None, nervy_saw_hurt: Some(String::from("The check's {checkEta}— {victim} won't answer it— then they ALL come—")), hard_none: None, hard_heard: None, hard_glimpsed: None, hard_saw: None, hard_saw_hurt: None, __none: None, __heard: None, __glimpsed: None, __saw: None, __saw_hurt: None, pedant_steady: None, pedant_nervy: None, pedant_hard: None, selfown_steady: None, selfown_nervy: None, selfown_hard: None, folder_steady: None, folder_nervy: None, folder_hard: None, ramble_steady: None, ramble_nervy: None, ramble_hard: None, fan_steady: None, fan_nervy: None, fan_hard: None, stoic_steady: None, stoic_nervy: None, stoic_hard: None, con_steady: None, con_nervy: None, con_hard: None, game_steady: None, game_nervy: None, game_hard: None, menace_steady: None, menace_nervy: None, menace_hard: None }); t.set(String::from("lockdownPosts"), crate::dialogue::INTELTEXTValue { steady: None, nervy: None, hard: Some(String::from("Pairs. A man on {post}. Nobody alone — except you.")), under: Some(String::from("We're paired now. One man holds {post}. Nobody walks alone.")), pedant: None, selfown: None, folder: None, ramble: None, fan: None, stoic: None, con: None, game: None, menace: None, steady_none: None, steady_heard: None, steady_glimpsed: None, steady_saw: None, steady_saw_hurt: None, nervy_none: None, nervy_heard: None, nervy_glimpsed: None, nervy_saw: None, nervy_saw_hurt: None, hard_none: None, hard_heard: None, hard_glimpsed: None, hard_saw: None, hard_saw_hurt: None, __none: None, __heard: None, __glimpsed: None, __saw: None, __saw_hurt: None, pedant_steady: None, pedant_nervy: None, pedant_hard: None, selfown_steady: None, selfown_nervy: None, selfown_hard: None, folder_steady: None, folder_nervy: None, folder_hard: None, ramble_steady: None, ramble_nervy: None, ramble_hard: None, fan_steady: None, fan_nervy: None, fan_hard: None, stoic_steady: None, stoic_nervy: None, stoic_hard: None, con_steady: None, con_nervy: None, con_hard: None, game_steady: None, game_nervy: None, game_hard: None, menace_steady: None, menace_nervy: None, menace_hard: None }); t.set(String::from("routes"), crate::dialogue::INTELTEXTValue { steady: None, nervy: Some(String::from("It's {n} of us— me on {route}, {partner} on {partnerRoute}— that's it—")), hard: Some(String::from("{N}. Me, {route}. {Partner}, {partnerRoute}. Happy?")), under: Some(String::from("{N} of us tonight. I've got {route}, {partner} has {partnerRoute}.")), pedant: None, selfown: None, folder: None, ramble: None, fan: Some(String::from("{N} of us— I do {route}, {partner} {partnerRoute}. What's YOUR route like?")), stoic: None, con: Some(String::from("{N} on shift: I take {route}, {partner} {partnerRoute}. All logged, sir.")), game: None, menace: None, steady_none: None, steady_heard: None, steady_glimpsed: None, steady_saw: None, steady_saw_hurt: None, nervy_none: None, nervy_heard: None, nervy_glimpsed: None, nervy_saw: None, nervy_saw_hurt: None, hard_none: None, hard_heard: None, hard_glimpsed: None, hard_saw: None, hard_saw_hurt: None, __none: None, __heard: None, __glimpsed: None, __saw: None, __saw_hurt: None, pedant_steady: None, pedant_nervy: None, pedant_hard: None, selfown_steady: None, selfown_nervy: None, selfown_hard: None, folder_steady: None, folder_nervy: None, folder_hard: None, ramble_steady: None, ramble_nervy: None, ramble_hard: None, fan_steady: None, fan_nervy: None, fan_hard: None, stoic_steady: None, stoic_nervy: None, stoic_hard: None, con_steady: None, con_nervy: None, con_hard: None, game_steady: None, game_nervy: None, game_hard: None, menace_steady: None, menace_nervy: None, menace_hard: None }); t.set(String::from("missing"), crate::dialogue::INTELTEXTValue { steady: None, nervy: None, hard: Some(String::from("{Missing} is off the air. His route comes up empty, we lock down.")), under: Some(String::from("Nobody can raise {missing}. His route turns up empty, this all locks down.")), pedant: None, selfown: None, folder: None, ramble: None, fan: None, stoic: None, con: None, game: None, menace: None, steady_none: None, steady_heard: None, steady_glimpsed: None, steady_saw: None, steady_saw_hurt: None, nervy_none: None, nervy_heard: None, nervy_glimpsed: None, nervy_saw: None, nervy_saw_hurt: None, hard_none: None, hard_heard: None, hard_glimpsed: None, hard_saw: None, hard_saw_hurt: None, __none: None, __heard: None, __glimpsed: None, __saw: None, __saw_hurt: None, pedant_steady: None, pedant_nervy: None, pedant_hard: None, selfown_steady: None, selfown_nervy: None, selfown_hard: None, folder_steady: None, folder_nervy: None, folder_hard: None, ramble_steady: None, ramble_nervy: None, ramble_hard: None, fan_steady: None, fan_nervy: None, fan_hard: None, stoic_steady: None, stoic_nervy: None, stoic_hard: None, con_steady: None, con_nervy: None, con_hard: None, game_steady: None, game_nervy: None, game_hard: None, menace_steady: None, menace_nervy: None, menace_hard: None }); t.set(String::from("drive"), crate::dialogue::INTELTEXTValue { steady: None, nervy: None, hard: Some(String::from("Your rack went dark on the board. A man is on his way to it.")), under: Some(String::from("Your rack dropped off the board. Somebody's already been sent to look at it.")), pedant: None, selfown: None, folder: None, ramble: None, fan: None, stoic: None, con: None, game: None, menace: None, steady_none: None, steady_heard: None, steady_glimpsed: None, steady_saw: None, steady_saw_hurt: None, nervy_none: None, nervy_heard: None, nervy_glimpsed: None, nervy_saw: None, nervy_saw_hurt: None, hard_none: None, hard_heard: None, hard_glimpsed: None, hard_saw: None, hard_saw_hurt: None, __none: None, __heard: None, __glimpsed: None, __saw: None, __saw_hurt: None, pedant_steady: None, pedant_nervy: None, pedant_hard: None, selfown_steady: None, selfown_nervy: None, selfown_hard: None, folder_steady: None, folder_nervy: None, folder_hard: None, ramble_steady: None, ramble_nervy: None, ramble_hard: None, fan_steady: None, fan_nervy: None, fan_hard: None, stoic_steady: None, stoic_nervy: None, stoic_hard: None, con_steady: None, con_nervy: None, con_hard: None, game_steady: None, game_nervy: None, game_hard: None, menace_steady: None, menace_nervy: None, menace_hard: None }); t } }

/// (src/game/dialogue.ts:340 INTEL_NOW: a table nothing writes)
pub fn intel_now() -> crate::dialogue::INTELNOW { crate::dialogue::INTELNOW { check_on: None, clearing: None, rollcall_live: None, blackout: None, door: None, code: Some(crate::dialogue::INTELNOWCode { steady: None, nervy: None, hard: None, under: Some(String::from("{PadDoor} is on a pad — {code}. Same four digits every night.")), pedant: None, selfown: None, folder: None, ramble: None, fan: None, stoic: None, con: None, game: None, menace: None, steady_none: None, steady_heard: None, steady_glimpsed: None, steady_saw: None, steady_saw_hurt: None, nervy_none: None, nervy_heard: None, nervy_glimpsed: None, nervy_saw: None, nervy_saw_hurt: None, hard_none: None, hard_heard: None, hard_glimpsed: None, hard_saw: None, hard_saw_hurt: None, __none: None, __heard: None, __glimpsed: None, __saw: None, __saw_hurt: None, pedant_steady: None, pedant_nervy: None, pedant_hard: None, selfown_steady: None, selfown_nervy: None, selfown_hard: None, folder_steady: None, folder_nervy: None, folder_hard: None, ramble_steady: None, ramble_nervy: None, ramble_hard: None, fan_steady: None, fan_nervy: None, fan_hard: None, stoic_steady: None, stoic_nervy: None, stoic_hard: None, con_steady: None, con_nervy: None, con_hard: None, game_steady: None, game_nervy: None, game_hard: None, menace_steady: None, menace_nervy: None, menace_hard: None }), rollcall: None, lockdown_posts: None, routes: None, missing: None, drive: None } }

/// (src/game/dialogue.ts:342 INTEL_GUIDE: a table nothing writes)
pub fn intel_guide() -> crate::dialogue::INTELGUIDE { crate::dialogue::INTELGUIDE { check_on: None, clearing: None, rollcall_live: Some(crate::dialogue::INTELGUIDERollcallLive { steady: None, nervy: None, hard: Some(String::from("That's the check. If I don't answer, {caller} sends somebody here.")), under: Some(String::from("That's a check now. If one of us doesn't answer, {caller} sends someone over.")), pedant: None, selfown: None, folder: None, ramble: None, fan: None, stoic: None, con: None, game: None, menace: None, steady_none: None, steady_heard: None, steady_glimpsed: None, steady_saw: None, steady_saw_hurt: None, nervy_none: None, nervy_heard: None, nervy_glimpsed: None, nervy_saw: None, nervy_saw_hurt: None, hard_none: None, hard_heard: None, hard_glimpsed: None, hard_saw: None, hard_saw_hurt: None, __none: None, __heard: None, __glimpsed: None, __saw: None, __saw_hurt: None, pedant_steady: None, pedant_nervy: None, pedant_hard: None, selfown_steady: None, selfown_nervy: None, selfown_hard: None, folder_steady: None, folder_nervy: None, folder_hard: None, ramble_steady: None, ramble_nervy: None, ramble_hard: None, fan_steady: None, fan_nervy: None, fan_hard: None, stoic_steady: None, stoic_nervy: None, stoic_hard: None, con_steady: None, con_nervy: None, con_hard: None, game_steady: None, game_nervy: None, game_hard: None, menace_steady: None, menace_nervy: None, menace_hard: None }), blackout: Some(crate::dialogue::INTELGUIDEBlackout { steady: None, nervy: None, hard: Some(String::from("If the lights go out, somebody goes to the breaker in {breakerRoom}. Armed.")), under: None, pedant: None, selfown: None, folder: None, ramble: None, fan: None, stoic: None, con: None, game: None, menace: None, steady_none: None, steady_heard: None, steady_glimpsed: None, steady_saw: None, steady_saw_hurt: None, nervy_none: None, nervy_heard: None, nervy_glimpsed: None, nervy_saw: None, nervy_saw_hurt: None, hard_none: None, hard_heard: None, hard_glimpsed: None, hard_saw: None, hard_saw_hurt: None, __none: None, __heard: None, __glimpsed: None, __saw: None, __saw_hurt: None, pedant_steady: None, pedant_nervy: None, pedant_hard: None, selfown_steady: None, selfown_nervy: None, selfown_hard: None, folder_steady: None, folder_nervy: None, folder_hard: None, ramble_steady: None, ramble_nervy: None, ramble_hard: None, fan_steady: None, fan_nervy: None, fan_hard: None, stoic_steady: None, stoic_nervy: None, stoic_hard: None, con_steady: None, con_nervy: None, con_hard: None, game_steady: None, game_nervy: None, game_hard: None, menace_steady: None, menace_nervy: None, menace_hard: None }), door: None, code: None, rollcall: Some(crate::dialogue::INTELGUIDERollcall { steady: None, nervy: None, hard: Some(String::from("Radio check {checkEta}. No answer, they send somebody. Do the math.")), under: None, pedant: None, selfown: None, folder: None, ramble: None, fan: None, stoic: None, con: None, game: None, menace: None, steady_none: None, steady_heard: None, steady_glimpsed: None, steady_saw: None, steady_saw_hurt: None, nervy_none: None, nervy_heard: None, nervy_glimpsed: None, nervy_saw: None, nervy_saw_hurt: None, hard_none: None, hard_heard: None, hard_glimpsed: None, hard_saw: None, hard_saw_hurt: None, __none: None, __heard: None, __glimpsed: None, __saw: None, __saw_hurt: None, pedant_steady: None, pedant_nervy: None, pedant_hard: None, selfown_steady: None, selfown_nervy: None, selfown_hard: None, folder_steady: None, folder_nervy: None, folder_hard: None, ramble_steady: None, ramble_nervy: None, ramble_hard: None, fan_steady: None, fan_nervy: None, fan_hard: None, stoic_steady: None, stoic_nervy: None, stoic_hard: None, con_steady: None, con_nervy: None, con_hard: None, game_steady: None, game_nervy: None, game_hard: None, menace_steady: None, menace_nervy: None, menace_hard: None }), lockdown_posts: None, routes: None, missing: None, drive: Some(crate::dialogue::INTELGUIDEDrive { steady: None, nervy: None, hard: Some(String::from("Your rack went dark on the board. Somebody's on his way to it.")), under: None, pedant: None, selfown: None, folder: None, ramble: None, fan: None, stoic: None, con: None, game: None, menace: None, steady_none: None, steady_heard: None, steady_glimpsed: None, steady_saw: None, steady_saw_hurt: None, nervy_none: None, nervy_heard: None, nervy_glimpsed: None, nervy_saw: None, nervy_saw_hurt: None, hard_none: None, hard_heard: None, hard_glimpsed: None, hard_saw: None, hard_saw_hurt: None, __none: None, __heard: None, __glimpsed: None, __saw: None, __saw_hurt: None, pedant_steady: None, pedant_nervy: None, pedant_hard: None, selfown_steady: None, selfown_nervy: None, selfown_hard: None, folder_steady: None, folder_nervy: None, folder_hard: None, ramble_steady: None, ramble_nervy: None, ramble_hard: None, fan_steady: None, fan_nervy: None, fan_hard: None, stoic_steady: None, stoic_nervy: None, stoic_hard: None, con_steady: None, con_nervy: None, con_hard: None, game_steady: None, game_nervy: None, game_hard: None, menace_steady: None, menace_nervy: None, menace_hard: None }) } }

/// (src/game/dialogue.ts:349 DRIVE_AWAY: a table nothing writes)
pub fn drive_away_table() -> crate::dialogue::DRIVEAWAY { crate::dialogue::DRIVEAWAY { steady: None, nervy: None, hard: Some(String::from("It's been missed. A man is on his way to look.")), under: Some(String::from("It's been missed already. Somebody's been sent to look.")) } }

/// (src/game/dialogue.ts:351 DRIVE_AWAY_GUIDE: a table nothing writes)
pub fn drive_away_guide() -> crate::dialogue::DRIVEAWAYGUIDE { crate::dialogue::DRIVEAWAYGUIDE { steady: None, nervy: None, hard: Some(String::from("They know it's gone. Somebody's on his way to look.")), under: Some(String::from("They already know it's gone. Somebody's been sent to look.")) } }

/// (src/game/dialogue.ts:358 CHECKON_SELF_WAS)
pub const CHECKON_SELF_WAS: &str = "I was sent round {checked}'s route — he'd gone quiet. Now I know why.";

/// (src/game/dialogue.ts:358 CHECKON_SELF_NOW)
pub const CHECKON_SELF_NOW: &str = "I was sent to walk {checked}'s route — he'd gone quiet. Now I know why.";

/// (src/game/dialogue.ts:364 SHAPE_LEAD: a table nothing writes)
pub fn shape_lead() -> crate::dialogue::SHAPELEAD { crate::dialogue::SHAPELEAD { pedant: None, selfown: None, folder: Some(String::from("")), ramble: Some(String::from("—okay— ")), fan: Some(String::from("Oh— and ")), stoic: None, con: Some(String::from("Sir: ")), game: None, menace: None } }

/// (src/game/dialogue.ts:376 OFFICE_AREAS: a table nothing writes)
pub fn office_areas() -> Vec<crate::dialogue::Rect> { alloc::vec![crate::dialogue::Rect { name: String::from("the corridor"), x0: 4.0, x1: 36.0, z0: 10.0, z1: 12.2 }, crate::dialogue::Rect { name: String::from("the lobby"), x0: 4.0, x1: 12.0, z0: 12.2, z1: 24.0 }, crate::dialogue::Rect { name: String::from("the cubicles"), x0: 12.0, x1: 28.0, z0: 12.2, z1: 24.0 }, crate::dialogue::Rect { name: String::from("the break room"), x0: 28.0, x1: 36.0, z0: 12.2, z1: 24.0 }] }

/// (src/game/dialogue.ts:410 PLACE_EDGE)
pub const PLACE_EDGE: f64 = 0.35;

/// (src/game/dialogue.ts:458 OFFICE_WALLS: a table nothing writes)
pub fn office_walls() -> crate::dialogue::Rect { crate::dialogue::Rect { name: String::from("inside"), x0: 4.0, x1: 36.0, z0: 4.0, z1: 24.0 } }

/// (src/game/dialogue.ts:460 NAMELESS)
pub const NAMELESS: &str = "the building";

/// (src/game/dialogue.ts:484 ROUTE_NAMES: a table nothing writes)
pub fn route_names() -> JsMap<String, String> { { let mut t = JsMap::new(); t.set(String::from("corridor"), String::from("the corridor")); t.set(String::from("cubicles"), String::from("the cubicles")); t.set(String::from("lobby_break"), String::from("the lobby loop")); t } }

/// (src/game/dialogue.ts:486 ROUTE_NAMES_NOW: a table nothing writes)
pub fn route_names_now() -> JsMap<String, String> { { let mut t = JsMap::new(); t.set(String::from("laneW"), String::from("the west lane")); t.set(String::from("laneE"), String::from("the east lane")); t.set(String::from("clerk"), String::from("the office")); t.set(String::from("north"), String::from("the north side")); t.set(String::from("south"), String::from("the south side")); t.set(String::from("east"), String::from("the east side")); t.set(String::from("sentry_a"), String::from("the west post")); t.set(String::from("sentry_b"), String::from("the east post")); t.set(String::from("ops"), String::from("the ops room")); t.set(String::from("walker"), String::from("the block")); t.set(String::from("fuel"), String::from("the east gate")); t.set(String::from("mast"), String::from("the mast yard")); t.set(String::from("bunk"), String::from("the bunks")); t.set(String::from("teahouse"), String::from("the tea house")); t.set(String::from("watchman"), String::from("the street end")); t.set(String::from("counting"), String::from("the counting room")); t.set(String::from("counting_b"), String::from("the counting room")); t.set(String::from("yard_round"), String::from("the yard")); t.set(String::from("yard_round_b"), String::from("the yard")); t.set(String::from("south_b"), String::from("the south side")); t.set(String::from("north_b"), String::from("the north side")); t.set(String::from("east_b"), String::from("the east side")); t.set(String::from("cook"), String::from("the cook fire")); t.set(String::from("cook_b"), String::from("the cook fire")); t.set(String::from("roundW"), String::from("the west round")); t.set(String::from("roundE"), String::from("the east round")); t } }

/// (src/game/dialogue.ts:488 ROUTE_NAMES_GUIDE: a table nothing writes)
pub fn route_names_guide() -> JsMap<String, Option<String>> { { let mut t = JsMap::new(); t.set(String::from("roundW"), Some(String::from("the west loop"))); t.set(String::from("roundE"), Some(String::from("the east loop"))); t } }

/// (src/game/dialogue.ts:488 DOOR_NAMES_GUIDE: a table nothing writes)
pub fn door_names_guide() -> JsMap<String, Option<String>> { { let mut t = JsMap::new(); t.set(String::from("armoury_radio"), Some(String::from("the armory side door"))); t } }

/// (src/game/dialogue.ts:490 DOOR_NAMES: a table nothing writes)
pub fn door_names() -> JsMap<String, String> { { let mut t = JsMap::new(); t.set(String::from("server"), String::from("the server room door")); t.set(String::from("manager"), String::from("the manager's door")); t.set(String::from("storage"), String::from("the storage door")); t.set(String::from("conference"), String::from("the conference door")); t.set(String::from("breakroom_n"), String::from("the break room door")); t.set(String::from("breakroom_w"), String::from("the break room side door")); t.set(String::from("breakroom_ext"), String::from("the outside door")); t.set(String::from("fire_exit"), String::from("the fire exit")); t.set(String::from("server_manager"), String::from("the office side door")); t } }

/// (src/game/dialogue.ts:492 DOOR_NAMES_NOW: a table nothing writes)
pub fn door_names_now() -> JsMap<String, String> { { let mut t = JsMap::new(); t.set(String::from("pharm_back"), String::from("the pharmacy back door")); t.set(String::from("pharm_front"), String::from("the pharmacy front door")); t.set(String::from("phone_back"), String::from("the phone shop back door")); t.set(String::from("fence_e"), String::from("the east fence gate")); t.set(String::from("office_garage"), String::from("the garage side door")); t.set(String::from("armoury_radio"), String::from("the armoury side door")); t } }

/// (src/game/dialogue.ts:494 NUM_WORDS: a table nothing writes)
pub fn num_words() -> Vec<String> { alloc::vec![String::from("none"), String::from("one"), String::from("two"), String::from("three"), String::from("four"), String::from("five"), String::from("six"), String::from("seven"), String::from("eight"), String::from("nine"), String::from("ten"), String::from("eleven"), String::from("twelve"), String::from("thirteen"), String::from("fourteen"), String::from("fifteen")] }

/// (src/game/dialogue.ts:497 DOOR_ROUTE_M)
pub const DOOR_ROUTE_M: f64 = 2.5;

/// (src/game/dialogue.ts:533 RECENT_SECS)
pub const RECENT_SECS: f64 = 90.0;

/// (src/game/dialogue.ts:533 RECENT_W)
pub const RECENT_W: f64 = 0.25;

/// (src/game/dialogue.ts:648 TOKEN_RE)
pub const TOKEN_RE: &str = "/\\{([A-Za-z]+)\\}/g";

/// (src/game/dialogue.ts:796 SAM_STAGES: a table nothing writes)
pub fn sam_stages() -> Vec<String> { alloc::vec![String::from("open"), String::from("press"), String::from("ask"), String::from("dry")] }

/// (src/game/dialogue.ts:798 SAM_MAX_CHARS)
pub const SAM_MAX_CHARS: f64 = 60.0;

/// (src/game/dialogue.ts:800 ARM_HOLD_HAS_KNIFE)
pub const ARM_HOLD_HAS_KNIFE: bool = true;

/// (src/game/dialogue.ts:803 SAM_NEEDS: a table nothing writes)
pub fn sam_needs() -> [String; 8] { [String::from("gun"), String::from("arm"), String::from("knife"), String::from("blackout"), String::from("alarm"), String::from("calm"), String::from("lockdown"), String::from("heldLong")] }

/// (src/game/dialogue.ts:931 SAM_RECENT_SECS)
pub const SAM_RECENT_SECS: f64 = 150.0;

/// (src/game/dialogue.ts:1038 ORIGINALITY_BLOCKLIST: a table nothing writes)
pub fn originality_blocklist() -> Vec<String> { alloc::vec![String::from("equation"), String::from("daisies"), String::from("cripple"), String::from("tuxedo"), String::from("monkey"), String::from("ninja"), String::from("pick a number"), String::from("wallet"), String::from("wet myself"), String::from("blowpipe"), String::from("training exercise"), String::from("death before dishonor"), String::from("death before dishonour"), String::from("we all die alone"), String::from("i have two"), String::from("the real kind"), String::from("chit-chat"), String::from("chitchat"), String::from("elvis"), String::from("freaking me out"), String::from("freak me out"), String::from("light switch"), String::from("lightswitch"), String::from("poke you"), String::from("sixth grade"), String::from("grade school"), String::from("fisher"), String::from("lambert"), String::from("grimsdott"), String::from("redding"), String::from("third echelon"), String::from("echelon"), String::from("shetland"), String::from("displace"), String::from("bathhouse"), String::from("lighthouse"), String::from("penthouse"), String::from("hokkaido"), String::from("panama"), String::from("guns are for"), String::from("knives are for"), String::from("necks snap"), String::from("snap your neck"), String::from("two kinds of people"), String::from("coma"), String::from("karate"), String::from("kung fu"), String::from("james bond"), String::from("superhero"), String::from("spy movie")] }

/// (src/game/dialogue.ts:1057 SAM_POOL_MIN)
pub const SAM_POOL_MIN: f64 = 5.0;

/// (src/game/dialogue.ts:1059 SAM_KEY_MIN)
pub const SAM_KEY_MIN: f64 = 3.0;

/// (src/game/dialogue.ts:1061 SCRIPT_VARIANTS_MIN)
pub const SCRIPT_VARIANTS_MIN: f64 = 3.0;

/// (src/game/dialogue.ts:1255 RAMBLE_SWALLOWS)
pub const RAMBLE_SWALLOWS: f64 = 1.0;

/// (src/game/dialogue.ts:1256 REJOINDERS)
pub const REJOINDERS: bool = true;

/// (src/game/dialogue.ts:1257 STOIC_BITE)
pub const STOIC_BITE: f64 = 1.0;

/// (src/game/dialogue.ts:1258 SCRIPT_ROTATE)
pub const SCRIPT_ROTATE: bool = true;

/// (src/game/dialogue.ts:1338 ROOM_MENACE: a table nothing writes)
pub fn room_menace() -> JsMap<String, Vec<crate::dialogue::RoomLine>> { {
    use crate::dialogue::{Him, RoomLine};
    fn s(x: &str) -> String { String::from(x) }
    fn text(x: &str) -> Him { Him::Text(s(x)) }
    fn keyed(rows: &[(&str, &str)]) -> Him { let mut m = JsMap::new(); for (k, v) in rows { m.set(s(k), s(v)); } Him::Arm1(m) }
    let mut m: JsMap<String, Vec<RoomLine>> = JsMap::new();
    m.set(s("the server room"), alloc::vec![
        RoomLine { sam: s("Server room. Know what the fire gas does to lungs?"), him: keyed(&[("*", "They made us watch a video. I'd rather not be in the sequel."), ("hard", "Roughly.")]), after: None },
        RoomLine { sam: s("Cold aisle's four degrees. I can wait in it. Can you?"), him: keyed(&[("*", "I'm in a polo shirt. This was already unfair."), ("hard", "Wore a vest.")]), after: None },
        RoomLine { sam: s("Lot of cable in here. Tidy loops. Strong ties."), him: keyed(&[("*", "I did those loops. Don't make me regret neatness."), ("hard", "I tied them. They hold.")]), after: None },
    ]);
    m.set(s("the break room"), alloc::vec![
        RoomLine { sam: s("That chest freezer takes a man who folds."), him: keyed(&[("*", "It's full of someone's batch cooking. …Mostly full."), ("hard", "It's full. Lunches. I checked.")]), after: None },
        RoomLine { sam: s("I could make you drink from the jug on the left."), him: keyed(&[("*", "That's stewed since six. That's not a threat, that's a grievance."), ("hard", "I drink that. Try harder.")]), after: None },
        RoomLine { sam: s("Microwave still does that sparking thing?"), him: keyed(&[("*", "Only with foil. …You'd find foil, wouldn't you."), ("hard", "Unplugged it myself.")]), after: None },
    ]);
    m.set(s("the car park"), alloc::vec![
        RoomLine { sam: s("Long ramp. Handbrakes fail more than people think."), him: keyed(&[("*", "Mine did, once. Nobody believed me then either."), ("hard", "I take the bus.")]), after: None },
        RoomLine { sam: s("Nobody hears anything down a ramp."), him: keyed(&[("*", "They don't hear anything UP it. I've radioed for a year."), ("hard", "I noticed.")]), after: None },
    ]);
    m.set(s("the conference room"), alloc::vec![
        RoomLine { sam: s("Conference room. I could make you present the quarter."), him: keyed(&[("*", "I've seen the numbers. That's crueller than the arm."), ("hard", "Seen worse decks.")]), after: None },
        RoomLine { sam: s("Forty chairs. I'll sit you in the one that squeaks."), him: keyed(&[("*", "Facing the projector? That's sick. …Sorry. Sir."), ("hard", "I oil that one.")]), after: None },
        RoomLine { sam: s("Whiteboard says Q3 TARGETS. Want to be a bullet point?"), him: keyed(&[("*", "There's no room. They never wipe it. It's from two Q3s ago."), ("hard", "Wipe it first.")]), after: None },
    ]);
    m.set(s("the corridor"), alloc::vec![
        RoomLine { sam: s("There's a shredder by the copier. Cross-cut."), him: keyed(&[("*", "It jams on three sheets. You'd be there all night. …So would I."), ("hard", "It jams.")]), after: None },
        RoomLine { sam: s("Confidential waste bin. Locked lid. Roomy."), him: keyed(&[("*", "They only empty it Thursdays. …What day is it."), ("hard", "Thursdays. Plan ahead.")]), after: None },
        RoomLine { sam: s("Long corridor. Very polished. You ever been curled?"), him: keyed(&[("*", "I signed off that polish. I'd feel involved."), ("hard", "Floor is level. Checked.")]), after: None },
    ]);
    m.set(s("the lobby"), alloc::vec![
        RoomLine { sam: s("Lot of glass in a lobby. All of it loud."), him: keyed(&[("*", "The big pane's already cracked. Facilities know. Facilities 'know'."), ("hard", "Glass is glass.")]), after: None },
        RoomLine { sam: s("Reception has a bell. I could ring it with you."), him: keyed(&[("*", "Nobody comes when you ring it. That's the frightening part."), ("hard", "Nobody comes.")]), after: None },
        RoomLine { sam: s("Revolving door still catching heels?"), him: keyed(&[("*", "It took a courier's shoe in May. We've still got the shoe."), ("hard", "I use the side door.")]), after: None },
    ]);
    m.set(s("the cubicles"), alloc::vec![
        RoomLine { sam: s("Forty desks. Forty staplers. Think about it."), him: keyed(&[("*", "Half are empty. The desks, not the— well. Both."), ("hard", "Thought. Unmoved.")]), after: None },
        RoomLine { sam: s("Partitions. Nobody sees over. Nobody looks."), him: keyed(&[("*", "Someone's put a 'hang in there' kitten up. I'd see that. Last."), ("hard", "I look.")]), after: None },
        RoomLine { sam: s("That laminator runs hot. Takes a while to notice."), him: keyed(&[("*", "It's signed out to me. Everything bad here is signed out to me."), ("hard", "Signed it back in.")]), after: None },
    ]);
    m.set(s("the manager's office"), alloc::vec![
        RoomLine { sam: s("Manager's office. Heavy award on that desk."), him: keyed(&[("*", "'Regional Excellence'. It's acrylic. It'd still hurt."), ("hard", "Acrylic.")]), after: None },
        RoomLine { sam: s("Nice letter opener. She won't miss it till Monday."), him: keyed(&[("*", "She'd miss it. She'd email about it. All staff. Twice."), ("hard", "She'd notice.")]), after: None },
    ]);
    m.set(s("the storage room"), alloc::vec![
        RoomLine { sam: s("Shelving to the ceiling. Badly anchored."), him: keyed(&[("*", "I reported that! There's a ticket! …Don't close the ticket."), ("hard", "I anchored it.")]), after: None },
        RoomLine { sam: s("Lot of toner in here. You know what toner does?"), him: keyed(&[("*", "Gets everywhere? …Oh. You mean worse than everywhere."), ("hard", "Gets on you too.")]), after: None },
    ]);
    m.set(s("the back offices"), alloc::vec![
        RoomLine { sam: s("Drop ceiling. Lots of room up there."), him: keyed(&[("*", "There's a pigeon up there since March. We named it. Don't."), ("hard", "After you.")]), after: None },
        RoomLine { sam: s("Nobody comes down here. I checked the dust."), him: keyed(&[("*", "The cleaners skip it. I don't blame them. I blame them a bit."), ("hard", "I come down here.")]), after: None },
    ]);
    m.set(s("the stairwell"), alloc::vec![
        RoomLine { sam: s("Four flights. Handrail's loose on two of them."), him: keyed(&[("*", "I put the tape on it! The tape IS the fix!"), ("hard", "I use the lift.")]), after: None },
        RoomLine { sam: s("Stairwells echo. Once."), him: keyed(&[("*", "That's— yes. I've dropped keys in here. Everyone knew."), ("hard", "So whisper.")]), after: None },
    ]);
    m
} }

/// (src/game/dialogue.ts:1392 KN: a table nothing writes)
pub fn kn() -> Vec<String> { alloc::vec![String::from("none"), String::from("heard"), String::from("glimpsed")] }

/// (src/game/dialogue.ts:1392 KS: a table nothing writes)
pub fn ks() -> Vec<String> { alloc::vec![String::from("none"), String::from("heard"), String::from("glimpsed"), String::from("saw")] }

/// (src/game/dialogue.ts:1396 SCRIPTS: a table nothing writes)
pub fn scripts() -> Vec<crate::dialogue::Script> { {
    use crate::dialogue::{Band, Beat, Him, IntelKind, Knowledge, Need, Script, ScriptBeats, ShapeKey};
    use crate::game::Temperament;
    fn s(x: &str) -> String { String::from(x) }
    fn text(x: &str) -> Him { Him::Text(s(x)) }
    fn keyed(rows: &[(&str, &str)]) -> Him { let mut m = JsMap::new(); for (k, v) in rows { m.set(s(k), s(v)); } Him::Arm1(m) }
    alloc::vec![
    Script {
        id: s("pedant-s1"), shape: ShapeKey::Pedant, t: Temperament::Steady, k: alloc::vec![Knowledge::None, Knowledge::Heard, Knowledge::Glimpsed],
        band: Some(alloc::vec![Band::Low, Band::Mid]), needs: Some(alloc::vec![Need::Office]), not: None, pin: None,
        beats: ScriptBeats {
            _1: Beat { sam: s("How many of you on tonight? Whisper it."), him: text("On tonight, or on this floor? Those are different lists."), ..Beat::zeroed() },
            _2: Beat { sam: s("The list that gets shorter if you stall."), him: text("…Floor. You mean floor. {Up}, counting me. Sorry— nerves."), ..Beat::zeroed() },
            _3: Beat { sam: s("Good. Now something I cannot count myself."), him: text("That's the lot. I count things. It's most of the job."), ..Beat::zeroed() },
            x: Beat { sam: s("Still {up}?"), him: text("Unless you've been busy. …Please don't answer that."), ..Beat::zeroed() },
        },
    },
    Script {
        id: s("pedant-s2"), shape: ShapeKey::Pedant, t: Temperament::Steady, k: alloc::vec![Knowledge::None, Knowledge::Heard, Knowledge::Glimpsed],
        band: Some(alloc::vec![Band::Low, Band::Mid]), needs: Some(alloc::vec![Need::Office]), not: None, pin: None,
        beats: ScriptBeats {
            _1: Beat { sam: s("Head count. This floor. Now."), him: text("Counting how — bodies present, or badged in? They drift."), ..Beat::zeroed() },
            _2: Beat { sam: s("Count the ones who'd miss you."), him: text("…That's a smaller number. {Up} present. Me included."), ..Beat::zeroed() },
            _3: Beat { sam: s("See, you round beautifully. Next fact."), him: text("There isn't a next fact. I peaked at counting."), ..Beat::zeroed() },
            x: Beat { sam: s("Any advance on {up}?"), him: text("It's not an auction. …{Up}. Final answer. Please stop bidding."), ..Beat::zeroed() },
        },
    },
    Script {
        id: s("pedant-s3"), shape: ShapeKey::Pedant, t: Temperament::Steady, k: alloc::vec![Knowledge::None, Knowledge::Heard, Knowledge::Glimpsed],
        band: Some(alloc::vec![Band::Low, Band::Mid]), needs: Some(alloc::vec![Need::Office]), not: None, pin: None,
        beats: ScriptBeats {
            _1: Beat { sam: s("Numbers. How many walking this floor?"), him: text("Walking right now, or rostered? Two of them sit down a lot."), ..Beat::zeroed() },
            _2: Beat { sam: s("The arm doesn't take footnotes."), him: text("{Up}. Call it {up}. Including the one you're wearing."), ..Beat::zeroed() },
            _3: Beat { sam: s("Now where the other feet go."), him: text("I only do my stretch— {route}. The rest is rumour and biscuits."), ..Beat::zeroed() },
            x: Beat { sam: s("You've gone quiet. Recounting?"), him: text("Checking my working. Still {up}. Still including me, ideally."), ..Beat::zeroed() },
        },
    },
    Script {
        id: s("selfown-s1"), shape: ShapeKey::Selfown, t: Temperament::Steady, k: alloc::vec![Knowledge::None, Knowledge::Heard, Knowledge::Glimpsed, Knowledge::Saw],
        band: None, needs: Some(alloc::vec![Need::Office]), not: None, pin: Some(IntelKind::Door),
        beats: ScriptBeats {
            _1: Beat { sam: s("You're not staff. Staff would have heard me."), him: keyed(&[("*", "I am staff. Eleven years. There's a photo on the lanyard. Look."), ("saw", "Staff enough to chase you round {sawWhere}. Eleven years.")]), ..Beat::zeroed() },
            _2: Beat { sam: s("Anyone can print a lanyard. Staff know the doors."), him: text("I know the doors. {Door}'s keyed and mine's on my— …no."), ..Beat::zeroed() },
            _3: Beat { sam: s("Finish the sentence. Then I finish with the arm."), him: text("There is no end to that sentence. It just stops there. Forever."), ..Beat::zeroed() },
            x: Beat { sam: s("Anything else you'd like to prove?"), him: text("No. I'm done proving things. Clearly not my strength."), ..Beat::zeroed() },
        },
    },
    Script {
        id: s("selfown-s2"), shape: ShapeKey::Selfown, t: Temperament::Steady, k: alloc::vec![Knowledge::None, Knowledge::Heard, Knowledge::Glimpsed, Knowledge::Saw],
        band: None, needs: Some(alloc::vec![Need::Office]), not: None, pin: Some(IntelKind::Door),
        beats: ScriptBeats {
            _1: Beat { sam: s("Contract badge. You don't know this floor."), him: keyed(&[("*", "Better than the day lot. Try me. …On reflection, do not try me."), ("saw", "I knew it well enough to spot you in it. Briefly.")]), ..Beat::zeroed() },
            _2: Beat { sam: s("Bet you can't even open {door}."), him: text("Course I can, it's only the key off my— I see what you did there."), ..Beat::zeroed() },
            _3: Beat { sam: s("So finish it. Slowly. Hands still."), him: text("Nothing to finish. I trail off professionally. Years of practice."), ..Beat::zeroed() },
            x: Beat { sam: s("You've gone shy."), him: text("I'm reviewing everything I've ever said out loud. It's a long list."), ..Beat::zeroed() },
        },
    },
    Script {
        id: s("selfown-s3"), shape: ShapeKey::Selfown, t: Temperament::Steady, k: alloc::vec![Knowledge::None, Knowledge::Heard, Knowledge::Glimpsed, Knowledge::Saw],
        band: None, needs: Some(alloc::vec![Need::Office]), not: None, pin: Some(IntelKind::Door),
        beats: ScriptBeats {
            _1: Beat { sam: s("You're the temp. Temps know nothing."), him: keyed(&[("*", "Permanent nights, thank you. I've a drawer with my name on it."), ("saw", "The temp nearly had you in {sawWhere}. Permanent, actually.")]), ..Beat::zeroed() },
            _2: Beat { sam: s("A drawer. Not a key, though."), him: text("Keys, plural. One for {door}, on my— …that was the whole trick, was it."), ..Beat::zeroed() },
            _3: Beat { sam: s("It was. Where."), him: text("Nowhere. I own a drawer and no keys. Write that down."), ..Beat::zeroed() },
            x: Beat { sam: s("Tell me about the drawer."), him: text("No. The drawer stays between me and the drawer."), ..Beat::zeroed() },
        },
    },
    Script {
        id: s("fan-s1"), shape: ShapeKey::Fan, t: Temperament::Steady, k: alloc::vec![Knowledge::None, Knowledge::Heard, Knowledge::Glimpsed],
        band: Some(alloc::vec![Band::Low]), needs: None, not: None, pin: None,
        beats: ScriptBeats {
            _1: Beat { sam: s("Not a word."), him: text("…You're an actual professional. Aren't you. Sorry— whispering. Wow."), ..Beat::zeroed() },
            _2: Beat { sam: s("This is the part where you're frightened."), him: text("I am! Bit of both. Is the arm thing hard to learn? Roughly?"), ..Beat::zeroed() },
            _3: Beat { sam: s("One useful thing. Then you can gush."), him: text("I don't know anything useful. I know YOU now. That's huge for me."), take: Some(s("There it is. Thank you. Stop smiling.")), ..Beat::zeroed() },
            x: Beat { sam: s("We never met."), him: text("Course not. …Can I say I met 'someone'? No details. A vibe."), ..Beat::zeroed() },
        },
    },
    Script {
        id: s("fan-s2"), shape: ShapeKey::Fan, t: Temperament::Steady, k: alloc::vec![Knowledge::None, Knowledge::Heard, Knowledge::Glimpsed],
        band: Some(alloc::vec![Band::Low]), needs: None, not: None, pin: None,
        beats: ScriptBeats {
            _1: Beat { sam: s("Easy. Hands where they are."), him: text("They're staying. Are those the goggles that— they are. Brilliant."), ..Beat::zeroed() },
            _2: Beat { sam: s("You should be more worried."), him: text("Oh, I'm terrified. Do you lot get dental? Genuine question."), ..Beat::zeroed() },
            _3: Beat { sam: s("Question for question. Mine first."), him: text("I've nothing worth trading. Nights, mate. They give us a torch."), take: Some(s("Good. And no. No dental.")), ..Beat::zeroed() },
            x: Beat { sam: s("Forget my face."), him: text("Never saw it. Felt the forearm, though. Textbook. Shutting up."), ..Beat::zeroed() },
        },
    },
    Script {
        id: s("fan-s3"), shape: ShapeKey::Fan, t: Temperament::Steady, k: alloc::vec![Knowledge::None, Knowledge::Heard, Knowledge::Glimpsed],
        band: Some(alloc::vec![Band::Low]), needs: Some(alloc::vec![Need::Office]), not: None, pin: None,
        beats: ScriptBeats {
            _1: Beat { sam: s("Shh."), him: text("Twenty years reading about this stuff and it happens on MY floor."), ..Beat::zeroed() },
            _2: Beat { sam: s("Focus. You're being held."), him: text("I know! Properly, too. Is there a course? Evening classes?"), ..Beat::zeroed() },
            _3: Beat { sam: s("Tell me something and I'll say there's a course."), him: text("I've got nothing. Ask me anything, though. Love a quiz."), take: Some(s("There's no course.")), ..Beat::zeroed() },
            x: Beat { sam: s("Not a word to anyone."), him: text("Who'd believe me? …I'll know, though. Quietly. For years."), ..Beat::zeroed() },
        },
    },
    Script {
        id: s("con-s1"), shape: ShapeKey::Con, t: Temperament::Steady, k: alloc::vec![Knowledge::None, Knowledge::Heard],
        band: Some(alloc::vec![Band::Low, Band::Mid]), needs: Some(alloc::vec![Need::Office]), not: Some(alloc::vec![Need::HeardShots, Need::BodyKnown, Need::FloorUp, Need::HeardBang]), pin: None,
        beats: ScriptBeats {
            _1: Beat { sam: s("Security audit. Unannounced. Badge number."), him: text("Oh— {badge}. Is this the fire-door thing? I flagged that. Twice."), ..Beat::zeroed() },
            _2: Beat { sam: s("I ask, you answer. That's the audit."), him: text("Sorry. Yes. It's just— nobody audits nights. Ever. Carry on."), ..Beat::zeroed() },
            _3: Beat { sam: s("Item three. Staffing and access. Go."), him: text("Staffing's thin and access is a joke. Quote me. Please quote me."), take: Some(s("Very thorough. Head office will be thrilled.")), ..Beat::zeroed() },
            x: Beat { sam: s("That concludes the audit."), him: text("…Auditors don't usually do the arm, do they. Thinking about it."), ..Beat::zeroed() },
        },
    },
    Script {
        id: s("con-s2"), shape: ShapeKey::Con, t: Temperament::Steady, k: alloc::vec![Knowledge::None, Knowledge::Heard],
        band: Some(alloc::vec![Band::Low, Band::Mid]), needs: Some(alloc::vec![Need::Office]), not: Some(alloc::vec![Need::HeardShots, Need::BodyKnown, Need::FloorUp, Need::HeardBang]), pin: None,
        beats: ScriptBeats {
            _1: Beat { sam: s("Penetration test. You're doing well so far."), him: text("Am I? Brilliant. Do I get a— is there a certificate at the end?"), ..Beat::zeroed() },
            _2: Beat { sam: s("Depends on the next answers."), him: text("Right. Focused. This is my certificate face."), ..Beat::zeroed() },
            _3: Beat { sam: s("Walk me through tonight's cover."), him: text("Cover's me, a torch and optimism. Put 'optimism'. It sounds funded."), take: Some(s("Textbook. I'll mention you by badge.")), ..Beat::zeroed() },
            x: Beat { sam: s("Test complete."), him: text("You're not writing any of this dow— oh. Oh, I see. Oh no."), ..Beat::zeroed() },
        },
    },
    Script {
        id: s("con-s3"), shape: ShapeKey::Con, t: Temperament::Steady, k: alloc::vec![Knowledge::None, Knowledge::Heard],
        band: Some(alloc::vec![Band::Low, Band::Mid]), needs: Some(alloc::vec![Need::Office]), not: Some(alloc::vec![Need::HeardShots, Need::BodyKnown, Need::FloorUp, Need::HeardBang]), pin: None,
        beats: ScriptBeats {
            _1: Beat { sam: s("Fire drill assessment. Don't turn round."), him: text("Finally. I've said for years the night drill is a work of fiction."), sam_now: Some(s("Fire drill assessment. Don't turn around.")), ..Beat::zeroed() },
            _2: Beat { sam: s("Assessors don't chat. Badge."), him: text("{Badge}. Sorry. It's just nice to be assessed for once."), ..Beat::zeroed() },
            _3: Beat { sam: s("Muster procedure. Who's where."), him: text("Muster's the car park. It's always the car park. Rain or rain."), take: Some(s("Exemplary. Gold star.")), ..Beat::zeroed() },
            x: Beat { sam: s("Drill over. As you were."), him: text("Hang on. Assessors come in through the FRONT."), ..Beat::zeroed() },
        },
    },
    Script {
        id: s("game-s1"), shape: ShapeKey::Game, t: Temperament::Steady, k: alloc::vec![Knowledge::None, Knowledge::Heard, Knowledge::Glimpsed, Knowledge::Saw],
        band: None, needs: None, not: None, pin: None,
        beats: ScriptBeats {
            _1: Beat { sam: s("New rule. You don't say 'basically'."), him: keyed(&[("*", "I— fine. I wasn't going to. …Was I? Now I can feel it queuing."), ("saw", "You had a gun on you an hour ago and THIS is the rule. Fine.")]), ..Beat::zeroed() },
            _2: Beat { sam: s("Rule two. Count down from seven. Slowly."), him: text("Seven. Six. Why seven— five. Four."), ..Beat::zeroed() },
            _3: Beat { sam: s("Hold at four. Now something useful."), him: text("Useful. Er. Three? No— that's counting again. Sorry. Four."), take: Some(s("Good. Stay at four.")), ..Beat::zeroed() },
            x: Beat { sam: s("You may finish."), him: text("Three, two, one. So basic— NO. Nearly. So nearly."), ..Beat::zeroed() },
        },
    },
    Script {
        id: s("game-s2"), shape: ShapeKey::Game, t: Temperament::Steady, k: alloc::vec![Knowledge::None, Knowledge::Heard, Knowledge::Glimpsed, Knowledge::Saw],
        band: None, needs: Some(alloc::vec![Need::Office]), not: None, pin: None,
        beats: ScriptBeats {
            _1: Beat { sam: s("Blink twice if you've ever validated parking."), him: keyed(&[("*", "That's— twice. There. What have I just admitted to?"), ("saw", "Hunted you all night and you open with PARKING. …Twice. There.")]), ..Beat::zeroed() },
            _2: Beat { sam: s("Everything. Now hum. One note. Low."), him: text("Mmmm— this is the strangest thing that has ever— mmmm."), ..Beat::zeroed() },
            _3: Beat { sam: s("Stop humming. Talk."), him: text("About what? You've had blinking AND humming. I'm out of face."), take: Some(s("Better than the humming.")), ..Beat::zeroed() },
            x: Beat { sam: s("Resume humming."), him: text("Mmmm. …I've lost the note. Was it this one? Mmmm? Lower?"), ..Beat::zeroed() },
        },
    },
    Script {
        id: s("game-s3"), shape: ShapeKey::Game, t: Temperament::Steady, k: alloc::vec![Knowledge::None, Knowledge::Heard, Knowledge::Glimpsed, Knowledge::Saw],
        band: None, needs: None, not: None, pin: None,
        beats: ScriptBeats {
            _1: Beat { sam: s("Every answer starts with 'respectfully'. Go."), him: keyed(&[("*", "Respectfully— what? Respectfully, what was the question?"), ("saw", "Respectfully, I nearly had you earlier. Respectfully.")]), ..Beat::zeroed() },
            _2: Beat { sam: s("There wasn't one. Well done."), him: text("Respectfully, this is a very odd night for a Tuesday sort of man."), ..Beat::zeroed() },
            _3: Beat { sam: s("Respectfully, tell me something."), him: text("Respectfully, I've got nothing. With the greatest respect. Nothing."), take: Some(s("You dropped the 'respectfully'. I'll allow it.")), ..Beat::zeroed() },
            x: Beat { sam: s("Dismissed."), him: text("Respectfully— can I stop now? It's stuck in my mouth. Respectfully."), ..Beat::zeroed() },
        },
    },
    Script {
        id: s("menace-s1"), shape: ShapeKey::Menace, t: Temperament::Steady, k: alloc::vec![Knowledge::None, Knowledge::Heard, Knowledge::Glimpsed, Knowledge::Saw],
        band: None, needs: Some(alloc::vec![Need::Office]), not: None, pin: None,
        beats: ScriptBeats {
            _1: Beat { sam: s("Look around. Tell me where we are."), him: keyed(&[("*", "{Room}. …Why, where did you think you were?"), ("saw", "{Room}. Same floor I chased you round. Smaller, now.")]), ..Beat::zeroed() },
            _2: Beat { sam: s("Quiet corner. Nobody walks past for hours."), him: text("I walk past. That's the job. …I see the flaw in that now."), room: Some(true), ..Beat::zeroed() },
            _3: Beat { sam: s("So. Something useful, or the tour continues."), him: text("Nothing useful. Continue the tour. I've had worse shifts. Just."), ..Beat::zeroed() },
            x: Beat { sam: s("Enjoy {room}."), him: text("I used to."), ..Beat::zeroed() },
        },
    },
    Script {
        id: s("menace-s2"), shape: ShapeKey::Menace, t: Temperament::Steady, k: alloc::vec![Knowledge::None, Knowledge::Heard, Knowledge::Glimpsed, Knowledge::Saw],
        band: None, needs: Some(alloc::vec![Need::Office]), not: None, pin: None,
        beats: ScriptBeats {
            _1: Beat { sam: s("You know this floor. Every room."), him: keyed(&[("*", "Every bin, every squeaky tile. Small life, but it's indexed."), ("saw", "Well enough to nearly corner you in it. Nearly is the word.")]), ..Beat::zeroed() },
            _2: Beat { sam: s("Then you know what in here would hurt."), him: text("The chairs, long term. Short term— you, I assume."), room: Some(true), ..Beat::zeroed() },
            _3: Beat { sam: s("Or you tell me what I need."), him: text("You need a different hostage. Said with respect. And urgency."), ..Beat::zeroed() },
            x: Beat { sam: s("We'll stay here a while."), him: text("Fine. It's warmer than {route}."), ..Beat::zeroed() },
        },
    },
    Script {
        id: s("menace-s3"), shape: ShapeKey::Menace, t: Temperament::Steady, k: alloc::vec![Knowledge::None, Knowledge::Heard, Knowledge::Glimpsed, Knowledge::Saw],
        band: None, needs: Some(alloc::vec![Need::Office]), not: None, pin: None,
        beats: ScriptBeats {
            _1: Beat { sam: s("Nice spot you picked to stand in."), him: keyed(&[("*", "I didn't pick it. There's a rota. The rota picked it."), ("saw", "I was standing in it looking for YOU. The rota will hear of this.")]), ..Beat::zeroed() },
            _2: Beat { sam: s("Rota didn't mention the sharp edges in here."), him: text("Health and safety did. Laminated sign. Nobody reads laminate."), room: Some(true), ..Beat::zeroed() },
            _3: Beat { sam: s("Talk, and we both leave {room}."), him: text("I've nothing. We live here now. I'll show you where the kettle is."), ..Beat::zeroed() },
            x: Beat { sam: s("Comfortable?"), him: text("In {room}? Nobody ever has been. There was a survey."), ..Beat::zeroed() },
        },
    },
    Script {
        id: s("pedant-n1"), shape: ShapeKey::Pedant, t: Temperament::Nervy, k: alloc::vec![Knowledge::None, Knowledge::Heard, Knowledge::Glimpsed],
        band: Some(alloc::vec![Band::Low, Band::Mid]), needs: None, not: None, pin: None,
        beats: ScriptBeats {
            _1: Beat { sam: s("How many of you tonight? Quietly."), him: text("Including me? Including YOU? Sorry— is that a trick one—"), ..Beat::zeroed() },
            _2: Beat { sam: s("Including anyone with a radio. Go."), him: text("{Up}! {Up}. Including me. Not you. Unless— no. {Up}."), take: Some(s("And breathe. In through the number.")), ..Beat::zeroed() },
            _3: Beat { sam: s("You never actually said the number."), him: text("{Up}! Sorry! {Up}. I panicked about the rounding."), ask: Some(s("Keep going. You're on a roll.")), ..Beat::zeroed() },
            x: Beat { sam: s("Final count?"), him: text("{Up}. Wait— still {up}? You'd tell me. You wouldn't tell me."), ..Beat::zeroed() },
        },
    },
    Script {
        id: s("pedant-n2"), shape: ShapeKey::Pedant, t: Temperament::Nervy, k: alloc::vec![Knowledge::None, Knowledge::Heard, Knowledge::Glimpsed],
        band: Some(alloc::vec![Band::Low, Band::Mid]), needs: Some(alloc::vec![Need::Office]), not: None, pin: None,
        beats: ScriptBeats {
            _1: Beat { sam: s("Staff on this floor. A number."), him: text("A number or THE number? They come apart after midnight—"), ..Beat::zeroed() },
            _2: Beat { sam: s("The one that's true right now."), him: text("{Up}. True right now. Ask again in a minute and I— sorry. {Up}."), take: Some(s("See. True things. More of those.")), ..Beat::zeroed() },
            _3: Beat { sam: s("What else is true right now?"), him: text("My arm's asleep and there's {up} of us. That's everything true."), ask: Some(s("What else is true right now?")), ..Beat::zeroed() },
            x: Beat { sam: s("Recount."), him: text("{Up}. …You're not writing any of this down, are you. Nobody does."), ..Beat::zeroed() },
        },
    },
    Script {
        id: s("pedant-n3"), shape: ShapeKey::Pedant, t: Temperament::Nervy, k: alloc::vec![Knowledge::None, Knowledge::Heard, Knowledge::Glimpsed],
        band: Some(alloc::vec![Band::Low, Band::Mid]), needs: Some(alloc::vec![Need::Office]), not: None, pin: None,
        beats: ScriptBeats {
            _1: Beat { sam: s("How many on shift?"), him: text("Shift or floor? Payroll says one thing, the floor says—"), ..Beat::zeroed() },
            _2: Beat { sam: s("Say the floor thing."), him: text("{Up}. Floor says {up}. Payroll's a liar anyway."), take: Some(s("The floor thing. Good. Stay on the floor.")), ..Beat::zeroed() },
            _3: Beat { sam: s("More floor things."), him: text("That's all the floor says. The floor's not chatty. I am. Sorry."), ask: Some(s("More floor things.")), ..Beat::zeroed() },
            x: Beat { sam: s("Last count."), him: text("{Up}, and I'd like to stay in it. The count. Keep me in the count."), ..Beat::zeroed() },
        },
    },
    Script {
        id: s("selfown-n1"), shape: ShapeKey::Selfown, t: Temperament::Nervy, k: alloc::vec![Knowledge::None, Knowledge::Heard, Knowledge::Glimpsed, Knowledge::Saw],
        band: None, needs: Some(alloc::vec![Need::Office]), not: None, pin: Some(IntelKind::Door),
        beats: ScriptBeats {
            _1: Beat { sam: s("You don't work here. Where's your badge?"), him: keyed(&[("*", "I DO— badge, radio, key to {door}, all here on my— oh no."), ("saw", "I DO— I CHASED you— key to {door} right here on my— oh no.")]), ..Beat::zeroed() },
            _2: Beat { sam: s("You've gone pale. Finish the thought."), him: text("There's no thought. Thought's gone. I'm mostly lanyard now."), ask: Some(s("Go on. You were doing so well.")), ..Beat::zeroed() },
            _3: Beat { sam: s("Anything else on that belt?"), him: text("I walked straight into that. My sister says I do that. At weddings."), ask: Some(s("What else is on you? Keep declaring.")), ..Beat::zeroed() },
            x: Beat { sam: s("Anything else to declare?"), him: text("A packed lunch and poor judgement. That's the lot. That's me."), ..Beat::zeroed() },
        },
    },
    Script {
        id: s("selfown-n2"), shape: ShapeKey::Selfown, t: Temperament::Nervy, k: alloc::vec![Knowledge::None, Knowledge::Heard, Knowledge::Glimpsed, Knowledge::Saw],
        band: None, needs: Some(alloc::vec![Need::Office]), not: None, pin: Some(IntelKind::Door),
        beats: ScriptBeats {
            _1: Beat { sam: s("Agency cover? You've never seen this floor."), him: keyed(&[("*", "Three years! I know every door— {door} sticks, you jiggle it—"), ("saw", "Three years! I nearly HAD you— and {door} sticks, you jiggle—")]), after: Some(s("Do you.")), ..Beat::zeroed() },
            _2: Beat { sam: s("Which key."), him: text("No key. Figure of speech. We jiggle… morale. Nothing else."), ask: Some(s("Which key would that be.")), ..Beat::zeroed() },
            _3: Beat { sam: s("Noted. Anything further?"), him: text("I'd like it noted I was helpful BEFORE I was stupid. In that order."), ask: Some(s("Noted. Keep being helpful.")), ..Beat::zeroed() },
            x: Beat { sam: s("Noted."), him: text("Is it? Is there a form? There is always, always a form."), ..Beat::zeroed() },
        },
    },
    Script {
        id: s("selfown-n3"), shape: ShapeKey::Selfown, t: Temperament::Nervy, k: alloc::vec![Knowledge::None, Knowledge::Heard, Knowledge::Glimpsed, Knowledge::Saw],
        band: None, needs: Some(alloc::vec![Need::Office]), not: None, pin: Some(IntelKind::Door),
        beats: ScriptBeats {
            _1: Beat { sam: s("You're lost. Visitors sign in downstairs."), him: keyed(&[("*", "VISITOR? I lock up! {Door}, nightly, with my own—"), ("saw", "VISITOR? I nearly CAUGHT you! I lock {door} with my own—")]), after: Some(s("Own what.")), ..Beat::zeroed() },
            _2: Beat { sam: s("Say it. You'll feel better."), him: text("…breath. My own breath. I lock up by breathing on it. Firmly."), ask: Some(s("Say 'key'. You'll feel better.")), ..Beat::zeroed() },
            _3: Beat { sam: s("Anything else you lock up?"), him: text("My bike. Badly, it turns out. It's been a year of this."), ask: Some(s("Anything else you lock up?")), ..Beat::zeroed() },
            x: Beat { sam: s("Deep breaths."), him: text("That's what the wellness app says. The app has never met you."), ..Beat::zeroed() },
        },
    },
    Script {
        id: s("folder-n1"), shape: ShapeKey::Folder, t: Temperament::Nervy, k: alloc::vec![Knowledge::None, Knowledge::Heard, Knowledge::Glimpsed, Knowledge::Saw],
        band: Some(alloc::vec![Band::Low, Band::Mid]), needs: Some(alloc::vec![Need::Office]), not: None, pin: None,
        beats: ScriptBeats {
            _1: Beat { sam: s("Right. Here's how this goes—"), him: keyed(&[("*", "Yes. Anything. Everything. You choose where I start. I trust you."), ("saw", "Yes! It's you— fine— anything. Everything. You pick where I start.")]), after: Some(s("…I hadn't finished the threat.")), ..Beat::zeroed() },
            _2: Beat { sam: s("One useful thing. Just one."), him: text("I don't HAVE a useful thing— I've a locker code— it's four ones—"), after: Some(s("Keep it.")), take: Some(s("Huh. I had a whole second half planned.")), ..Beat::zeroed() },
            _3: Beat { sam: s("Since you're in the mood. What else."), him: text("Wifi's Synergy-something, capital S. Unchanged since the merger."), after: Some(s("…Thank you?")), ask: Some(s("Since you're in the mood. What else.")), ..Beat::zeroed() },
            x: Beat { sam: s("We're done."), him: text("Good biscuits are behind the decaf. Nobody looks behind the decaf."), after: Some(s("I'll treasure that.")), ..Beat::zeroed() },
        },
    },
    Script {
        id: s("folder-n2"), shape: ShapeKey::Folder, t: Temperament::Nervy, k: alloc::vec![Knowledge::None, Knowledge::Heard, Knowledge::Glimpsed, Knowledge::Saw],
        band: Some(alloc::vec![Band::Low, Band::Mid]), needs: Some(alloc::vec![Need::Office]), not: None, pin: None,
        beats: ScriptBeats {
            _1: Beat { sam: s("Listen carefully—"), him: keyed(&[("*", "Listening. Agreeing. Whatever it is, I've agreed. Backdate it."), ("saw", "Listening! I saw what you— agreed. Whatever it is. Backdate it.")]), after: Some(s("That's… efficient.")), ..Beat::zeroed() },
            _2: Beat { sam: s("Then tell me the thing I want."), him: text("I want to! I've got nothing! I've got a parking code! It's expired!"), after: Some(s("Tempting.")), take: Some(s("You didn't even haggle.")), ..Beat::zeroed() },
            _3: Beat { sam: s("More."), him: text("Locker forty. Code's my birthday. It's on the group chat. With cake."), ask: Some(s("More.")), ..Beat::zeroed() },
            x: Beat { sam: s("Stop helping."), him: text("The printer on three jams if you look at it. That's my last one."), ..Beat::zeroed() },
        },
    },
    Script {
        id: s("folder-n3"), shape: ShapeKey::Folder, t: Temperament::Nervy, k: alloc::vec![Knowledge::None, Knowledge::Heard, Knowledge::Glimpsed, Knowledge::Saw],
        band: Some(alloc::vec![Band::Low, Band::Mid]), needs: Some(alloc::vec![Need::Office]), not: None, pin: None,
        beats: ScriptBeats {
            _1: Beat { sam: s("You're going to answer some—"), him: keyed(&[("*", "Yes. Yes to the questions. Yes to the tone. All of it. Pre-yes."), ("saw", "Yes— I know who you— yes to all of it. The tone. Pre-yes.")]), after: Some(s("I usually get to finish.")), ..Beat::zeroed() },
            _2: Beat { sam: s("Fine. What should I know?"), him: text("That I'm cooperating? Very hard? Can that go somewhere? On a file?"), after: Some(s("There's no file.")), take: Some(s("…People normally make me work for that.")), ..Beat::zeroed() },
            _3: Beat { sam: s("And?"), him: text("And the coffee machine takes exact change and holds grudges."), ask: Some(s("And?")), ..Beat::zeroed() },
            x: Beat { sam: s("Enough."), him: text("Fridge cake marked 'Dave' isn't Dave's. There is no Dave. Never was."), after: Some(s("Noted. No Dave.")), ..Beat::zeroed() },
        },
    },
    Script {
        id: s("ramble-n1"), shape: ShapeKey::Ramble, t: Temperament::Nervy, k: alloc::vec![Knowledge::None, Knowledge::Heard, Knowledge::Glimpsed, Knowledge::Saw],
        band: Some(alloc::vec![Band::Mid, Band::High]), needs: Some(alloc::vec![Need::Office]), not: None, pin: None,
        beats: ScriptBeats {
            _1: Beat { sam: s("Not a sound. Nod if—"), him: keyed(&[("*", "—sorry, I talk when I'm scared, my podcast's actually ABOUT that—"), ("saw", "—it's you— sorry— I talk when I'm scared, my podcast's ABOUT that—")]), after: Some(s("Your what.")), ..Beat::zeroed() },
            _2: Beat { sam: s("One fact. Small. Go."), him: text("—forty listeners, it's on focus, ironically, and breathing, and—"), after: Some(s("That's not intel. That's a trailer.")), take: Some(s("…Was that in there the whole time?")), ..Beat::zeroed() },
            _3: Beat { sam: s("Another. Smaller."), him: text("—and HR said the chair thing was MY posture— MINE— in writing—"), after: Some(s("I'm putting you down as 'no'.")), ask: Some(s("Another. Smaller.")), ..Beat::zeroed() },
            x: Beat { sam: s("Are you done?"), him: text("—day ninety of the streak and THIS is what my heart rate gets—"), ..Beat::zeroed() },
        },
    },
    Script {
        id: s("ramble-n2"), shape: ShapeKey::Ramble, t: Temperament::Nervy, k: alloc::vec![Knowledge::None, Knowledge::Heard, Knowledge::Glimpsed, Knowledge::Saw],
        band: Some(alloc::vec![Band::Mid, Band::High]), needs: Some(alloc::vec![Need::Office]), not: None, pin: None,
        beats: ScriptBeats {
            _1: Beat { sam: s("Quiet. I'll ask, you'll—"), him: keyed(&[("*", "—is this the parking? Because I appealed the parking, formally—"), ("saw", "—you're the— is this the parking? I APPEALED the parking, formally—")]), after: Some(s("It's not about the parking.")), ..Beat::zeroed() },
            _2: Beat { sam: s("Focus. What's on this floor?"), him: text("—forty a month to park at my own JOB, and the ramp floods, and—"), after: Some(s("Still not the parking.")), take: Some(s("A fact. In among the parking. Fine.")), ..Beat::zeroed() },
            _3: Beat { sam: s("Last try. Something useful."), him: text("—I've photos of the ramp. Timestamped. I'll forward you the—"), after: Some(s("He'll forward me the ramp.")), ask: Some(s("Last try. Something useful.")), ..Beat::zeroed() },
            x: Beat { sam: s("Breathe."), him: text("—and they validate for VISITORS. Visitors! YOU would get validated!"), ..Beat::zeroed() },
        },
    },
    Script {
        id: s("ramble-n3"), shape: ShapeKey::Ramble, t: Temperament::Nervy, k: alloc::vec![Knowledge::None, Knowledge::Heard, Knowledge::Glimpsed, Knowledge::Saw],
        band: Some(alloc::vec![Band::Mid, Band::High]), needs: None, not: None, pin: None,
        beats: ScriptBeats {
            _1: Beat { sam: s("Whisper. Only when I—"), him: keyed(&[("*", "—okay, okay, context: eleven weeks of nights, body clock in a bin—"), ("saw", "—okay— YOU— context: eleven weeks of nights, body clock in a bin—")]), after: Some(s("I said when.")), ..Beat::zeroed() },
            _2: Beat { sam: s("One thing I can use."), him: text("—the app says I slept four hours in PIECES, it draws you a graph—"), after: Some(s("I can't use the graph.")), take: Some(s("Usable. Barely. Go on hyperventilating.")), ..Beat::zeroed() },
            _3: Beat { sam: s("Something about tonight."), him: text("—tonight's three coffees and a sort of oat bar with opinions—"), after: Some(s("We're done here.")), ask: Some(s("Something about tonight.")), ..Beat::zeroed() },
            x: Beat { sam: s("Finished?"), him: text("—it had RAISINS in it, unannounced, which I think legally—"), ..Beat::zeroed() },
        },
    },
    Script {
        id: s("game-n1"), shape: ShapeKey::Game, t: Temperament::Nervy, k: alloc::vec![Knowledge::None, Knowledge::Heard, Knowledge::Glimpsed, Knowledge::Saw],
        band: None, needs: None, not: None, pin: None,
        beats: ScriptBeats {
            _1: Beat { sam: s("Rule one: no vowels louder than this."), him: keyed(&[("*", "wht— sorry— what? okay. okay. tiny vowels. tiny."), ("saw", "it's y— sorry— tiny vowels. okay. i can do tiny.")]), ..Beat::zeroed() },
            _2: Beat { sam: s("Good. Now a fact. Tiny vowels."), him: text("a fct— no fact. i've gt nothing. this is very hard."), take: Some(s("Vowels crept up at the end. Watch it.")), ..Beat::zeroed() },
            _3: Beat { sam: s("Another."), him: text("cn i— can I have one loud vowel. Just an 'oh'. On account."), ask: Some(s("Another.")), ..Beat::zeroed() },
            x: Beat { sam: s("One."), him: text("OH. …thank you. that's been building since the arm."), ..Beat::zeroed() },
        },
    },
    Script {
        id: s("game-n2"), shape: ShapeKey::Game, t: Temperament::Nervy, k: alloc::vec![Knowledge::None, Knowledge::Heard, Knowledge::Glimpsed, Knowledge::Saw],
        band: None, needs: None, not: None, pin: None,
        beats: ScriptBeats {
            _1: Beat { sam: s("From now on you're Unit Two. Say it."), him: keyed(&[("*", "Unit Two. Why Two— who's One— am I not One material?"), ("saw", "Unit Two— I nearly caught— fine. Unit Two. Who's ONE?")]), ..Beat::zeroed() },
            _2: Beat { sam: s("Unit Two, report."), him: text("Unit Two has nothing to report and would like that minuted!"), take: Some(s("Adequate, Unit Two.")), ..Beat::zeroed() },
            _3: Beat { sam: s("Further, Unit Two?"), him: text("Unit Two requests to be One. Once. Before whatever this is ends."), ask: Some(s("Further, Unit Two?")), ..Beat::zeroed() },
            x: Beat { sam: s("Denied."), him: text("Unit Two saw that coming and is still, somehow, hurt."), ..Beat::zeroed() },
        },
    },
    Script {
        id: s("game-n3"), shape: ShapeKey::Game, t: Temperament::Nervy, k: alloc::vec![Knowledge::None, Knowledge::Heard, Knowledge::Glimpsed, Knowledge::Saw],
        band: None, needs: None, not: None, pin: None,
        beats: ScriptBeats {
            _1: Beat { sam: s("You get five words. Total. Budget them."), him: keyed(&[("*", "Five?! That's— wait, do these— oh no."), ("saw", "Five?! But you're the— do these count— oh no.")]), after: Some(s("Seven. I'll restart the meter. Once.")), ..Beat::zeroed() },
            _2: Beat { sam: s("Meter's running. Spend wisely."), him: text("Nothing. Know nothing. Two left. …Help."), take: Some(s("Way over budget. Worth it.")), ..Beat::zeroed() },
            _3: Beat { sam: s("I'll extend your credit. Talk."), him: text("Can unused words roll over? Asking for a— no. Wasted four."), ask: Some(s("I'll extend your credit. Talk.")), ..Beat::zeroed() },
            x: Beat { sam: s("Budget's closed."), him: text("Mm. Mm-mm. …That's humming, not words. Loophole. —Six. Damn."), ..Beat::zeroed() },
        },
    },
    Script {
        id: s("menace-n1"), shape: ShapeKey::Menace, t: Temperament::Nervy, k: alloc::vec![Knowledge::None, Knowledge::Heard, Knowledge::Glimpsed, Knowledge::Saw],
        band: None, needs: Some(alloc::vec![Need::Office]), not: None, pin: None,
        beats: ScriptBeats {
            _1: Beat { sam: s("Bad room to be caught in. Look at it."), him: keyed(&[("*", "It's a bad room to be EMPLOYED in! I've said! There's a survey!"), ("saw", "YOU'RE the bad thing in it! Sorry. Sorry. Also the room, yes.")]), room: Some(true), ..Beat::zeroed() },
            _2: Beat { sam: s("Or you talk. Talking's warmer."), him: text("I'd talk! I've nothing to talk WITH! Threaten me near a plant!"), take: Some(s("See. Warmer already.")), ..Beat::zeroed() },
            _3: Beat { sam: s("Keep talking."), him: text("Can we do this in the lobby? There's a ficus. It calms me."), ask: Some(s("Keep talking.")), ..Beat::zeroed() },
            x: Beat { sam: s("We're staying."), him: text("Then I'm naming things. That vent's Gary. Hi Gary. Help me, Gary."), ..Beat::zeroed() },
        },
    },
    Script {
        id: s("menace-n2"), shape: ShapeKey::Menace, t: Temperament::Nervy, k: alloc::vec![Knowledge::None, Knowledge::Heard, Knowledge::Glimpsed, Knowledge::Saw],
        band: None, needs: Some(alloc::vec![Need::Office]), not: None, pin: None,
        beats: ScriptBeats {
            _1: Beat { sam: s("You know what happens in rooms like this."), him: keyed(&[("*", "Meetings? Oh— you mean— oh, worse than meetings. Right."), ("saw", "You do! You happen! I've SEEN you happen! …Sorry. Go on.")]), room: Some(true), ..Beat::zeroed() },
            _2: Beat { sam: s("Unless you've something better to offer."), him: text("Better than THAT? I've got a bus pass and regrets! Take both!"), take: Some(s("Better. Marginally.")), sam_now: Some(s("Unless you've got something better to offer.")), ..Beat::zeroed() },
            _3: Beat { sam: s("Go on."), him: text("Never liked {room}. Now it is mutual. I feel that."), ask: Some(s("Go on.")), ..Beat::zeroed() },
            x: Beat { sam: s("Settle in."), him: text("I'm telling facilities about you. Anonymously. SO anonymously."), ..Beat::zeroed() },
        },
    },
    Script {
        id: s("menace-n3"), shape: ShapeKey::Menace, t: Temperament::Nervy, k: alloc::vec![Knowledge::None, Knowledge::Heard, Knowledge::Glimpsed, Knowledge::Saw],
        band: None, needs: Some(alloc::vec![Need::Office]), not: None, pin: None,
        beats: ScriptBeats {
            _1: Beat { sam: s("Look where we are. Take your time."), him: keyed(&[("*", "{Room}. I know. I KNOW. I've had the induction. Twice."), ("saw", "{Room}— with YOU— which is the worst version of {room} so far.")]), room: Some(true), ..Beat::zeroed() },
            _2: Beat { sam: s("Talk and I lose interest in the furniture."), him: text("Lose it anyway! Please! It's dreadful furniture! Nobody'd blame you!"), take: Some(s("Interest lost.")), ..Beat::zeroed() },
            _3: Beat { sam: s("And?"), him: text("I'm going to have such SPECIFIC nightmares. With an inventory."), ask: Some(s("What else.")), ..Beat::zeroed() },
            x: Beat { sam: s("Still with me?"), him: text("Physically. The rest of me is in the car park. Running. Badly."), ..Beat::zeroed() },
        },
    },
    Script {
        id: s("stoic-h1"), shape: ShapeKey::Stoic, t: Temperament::Hard, k: alloc::vec![Knowledge::None, Knowledge::Heard, Knowledge::Glimpsed, Knowledge::Saw],
        band: None, needs: None, not: None, pin: None,
        beats: ScriptBeats {
            _1: Beat { sam: s("Quiet."), him: keyed(&[("*", "I was."), ("saw", "You weren't. Earlier.")]), ..Beat::zeroed() },
            _2: Beat { sam: s("It gets tighter from here."), him: text("Arms do."), ..Beat::zeroed() },
            _3: Beat { sam: s("Last time I ask nicely."), him: text("That was nicely?"), ..Beat::zeroed() },
            x: Beat { sam: s("Anything to add?"), him: text("A mint. For you. Urgently."), mild: Some(s("No. And I'd have said.")), ..Beat::zeroed() },
        },
    },
    Script {
        id: s("stoic-h2"), shape: ShapeKey::Stoic, t: Temperament::Hard, k: alloc::vec![Knowledge::None, Knowledge::Heard, Knowledge::Glimpsed, Knowledge::Saw],
        band: None, needs: None, not: None, pin: None,
        beats: ScriptBeats {
            _1: Beat { sam: s("Don't."), him: keyed(&[("*", "Wasn't."), ("saw", "Already did. Missed.")]), ..Beat::zeroed() },
            _2: Beat { sam: s("You're calm, for a man in an arm."), him: text("Not my first arm."), ..Beat::zeroed() },
            _3: Beat { sam: s("One thing, and you keep your evening."), him: text("This was my evening. You've had it."), ..Beat::zeroed() },
            x: Beat { sam: s("Nothing else?"), him: text("You grunt when you crouch. Get that looked at."), mild: Some(s("Nothing. Same as before, only later.")), ..Beat::zeroed() },
        },
    },
    Script {
        id: s("stoic-h3"), shape: ShapeKey::Stoic, t: Temperament::Hard, k: alloc::vec![Knowledge::None, Knowledge::Heard, Knowledge::Glimpsed, Knowledge::Saw],
        band: None, needs: Some(alloc::vec![Need::Shift]), not: None, pin: None,
        beats: ScriptBeats {
            _1: Beat { sam: s("Easy."), him: keyed(&[("*", "For you, apparently."), ("saw", "It was, nearly. For me.")]), ..Beat::zeroed() },
            _2: Beat { sam: s("I can wait."), him: text("So can I. I'm paid to. Badly, but hourly."), ..Beat::zeroed() },
            _3: Beat { sam: s("One fact. Then I'm a rumour."), him: text("You're already a rumour. Short one."), ..Beat::zeroed() },
            x: Beat { sam: s("Last word?"), him: text("You breathe loud. For a rumour."), mild: Some(s("Same answer. Shorter.")), ..Beat::zeroed() },
        },
    },
    Script {
        id: s("stoic-h4"), shape: ShapeKey::Stoic, t: Temperament::Hard, k: alloc::vec![Knowledge::None, Knowledge::Heard, Knowledge::Glimpsed, Knowledge::Saw],
        band: None, needs: None, not: None, pin: None,
        beats: ScriptBeats {
            _1: Beat { sam: s("Not a sound."), him: keyed(&[("*", "Deal."), ("saw", "Bit late for quiet. But deal.")]), ..Beat::zeroed() },
            _2: Beat { sam: s("You've done this before."), him: text("Been done to. Twice. Better both times."), ..Beat::zeroed() },
            _3: Beat { sam: s("Then you know the part where you talk."), him: text("I know the part where you get bored."), ..Beat::zeroed() },
            x: Beat { sam: s("We're finished."), him: text("You were finished at the knees. Years back."), mild: Some(s("We were finished at hello.")), ..Beat::zeroed() },
        },
    },
    Script {
        id: s("game-h1"), shape: ShapeKey::Game, t: Temperament::Hard, k: alloc::vec![Knowledge::None, Knowledge::Heard, Knowledge::Glimpsed, Knowledge::Saw],
        band: None, needs: None, not: None, pin: None,
        beats: ScriptBeats {
            _1: Beat { sam: s("Rule: you answer in under three words."), him: keyed(&[("*", "Suits me."), ("saw", "Missed you. Twice.")]), ..Beat::zeroed() },
            _2: Beat { sam: s("What's your job?"), him: text("Standing. Mostly."), ..Beat::zeroed() },
            _3: Beat { sam: s("What ruins my night in here?"), him: text("Me. Shortly."), take: Some(s("More than three. I'll let it go.")), ..Beat::zeroed() },
            x: Beat { sam: s("Closing remarks?"), him: text("Two words."), mild: Some(s("None. Obviously.")), ..Beat::zeroed() },
        },
    },
    Script {
        id: s("game-h2"), shape: ShapeKey::Game, t: Temperament::Hard, k: alloc::vec![Knowledge::None, Knowledge::Heard, Knowledge::Glimpsed, Knowledge::Saw],
        band: None, needs: None, not: None, pin: None,
        beats: ScriptBeats {
            _1: Beat { sam: s("Nod for yes. Shrug for no."), him: keyed(&[("*", "I'll narrate. This is a shrug."), ("saw", "This is a shrug. That, earlier, was a warning shot.")]), ..Beat::zeroed() },
            _2: Beat { sam: s("Are you going to be difficult?"), him: text("Nod. Big one."), ..Beat::zeroed() },
            _3: Beat { sam: s("Something useful. Use words."), him: text("Shrug. Slow, meaningful shrug."), take: Some(s("See. Words.")), ..Beat::zeroed() },
            x: Beat { sam: s("Anything else?"), him: text("Picture a gesture."), mild: Some(s("Shrug.")), ..Beat::zeroed() },
        },
    },
    Script {
        id: s("game-h3"), shape: ShapeKey::Game, t: Temperament::Hard, k: alloc::vec![Knowledge::None, Knowledge::Heard, Knowledge::Glimpsed, Knowledge::Saw],
        band: None, needs: None, not: None, pin: None,
        beats: ScriptBeats {
            _1: Beat { sam: s("We'll play a game."), him: keyed(&[("*", "You'll play. I'm equipment."), ("saw", "We played earlier. You hid.")]), ..Beat::zeroed() },
            _2: Beat { sam: s("Count the exits you can't reach."), him: text("All of them. Good game. Next."), ..Beat::zeroed() },
            _3: Beat { sam: s("Now one thing I don't know."), him: text("How this ends for you. Nor do I. Fun."), take: Some(s("You lose gracefully.")), ..Beat::zeroed() },
            x: Beat { sam: s("Rematch?"), him: text("Bring a longer arm."), mild: Some(s("Not tonight.")), ..Beat::zeroed() },
        },
    },
    Script {
        id: s("menace-h1"), shape: ShapeKey::Menace, t: Temperament::Hard, k: alloc::vec![Knowledge::None, Knowledge::Heard, Knowledge::Glimpsed, Knowledge::Saw],
        band: None, needs: Some(alloc::vec![Need::Office]), not: None, pin: None,
        beats: ScriptBeats {
            _1: Beat { sam: s("Know where you are?"), him: keyed(&[("*", "Work."), ("saw", "Behind on my paperwork. About you.")]), ..Beat::zeroed() },
            _2: Beat { sam: s("Rooms like this keep secrets."), him: text("This one keeps a draught."), room: Some(true), ..Beat::zeroed() },
            _3: Beat { sam: s("Or one useful sentence."), him: text("Do the room thing. I'm curious now."), ..Beat::zeroed() },
            x: Beat { sam: s("Thoughts?"), him: text("You rehearse these. It shows."), mild: Some(s("None for you.")), ..Beat::zeroed() },
        },
    },
    Script {
        id: s("menace-h2"), shape: ShapeKey::Menace, t: Temperament::Hard, k: alloc::vec![Knowledge::None, Knowledge::Heard, Knowledge::Glimpsed, Knowledge::Saw],
        band: None, needs: Some(alloc::vec![Need::Office]), not: None, pin: None,
        beats: ScriptBeats {
            _1: Beat { sam: s("Look around you."), him: keyed(&[("*", "Seen it. Nightly."), ("saw", "Saw it. Saw you in it. Preferred it empty.")]), ..Beat::zeroed() },
            _2: Beat { sam: s("Plenty in here to regret."), him: text("The carpet, mainly."), room: Some(true), ..Beat::zeroed() },
            _3: Beat { sam: s("Talk instead."), him: text("No. Go on. Redecorate."), ..Beat::zeroed() },
            x: Beat { sam: s("Anything?"), him: text("You'd never get the deposit back."), mild: Some(s("Nothing.")), ..Beat::zeroed() },
        },
    },
    Script {
        id: s("menace-h3"), shape: ShapeKey::Menace, t: Temperament::Hard, k: alloc::vec![Knowledge::None, Knowledge::Heard, Knowledge::Glimpsed, Knowledge::Saw],
        band: None, needs: Some(alloc::vec![Need::Office]), not: None, pin: None,
        beats: ScriptBeats {
            _1: Beat { sam: s("Cosy in here."), him: keyed(&[("*", "Isn't."), ("saw", "Was. Then you.")]), sam_now: Some(s("Cozy in here.")), ..Beat::zeroed() },
            _2: Beat { sam: s("Lot of hard surfaces."), him: text("I mop them. I'd know."), room: Some(true), ..Beat::zeroed() },
            _3: Beat { sam: s("Or you give me something."), him: text("I'll take the surfaces."), ..Beat::zeroed() },
            x: Beat { sam: s("Last chance."), him: text("For you to lift properly. Bend the knees."), mild: Some(s("Pass.")), ..Beat::zeroed() },
        },
    },
    ]
} }

/// (src/game/dialogue.ts:1668 HANDLER_TAG)
pub const HANDLER_TAG: &str = "handler";

/// (src/game/dialogue.ts:1669 RADIO_REPLY_SECS)
pub const RADIO_REPLY_SECS: f64 = 1.7;

/// (src/game/dialogue.ts:1671 RADIO_EDGE_DELAY)
pub const RADIO_EDGE_DELAY: f64 = 2.6;

/// (src/game/dialogue.ts:1672 HANDLER_BEATS: a table nothing writes)
pub fn handler_beats() -> [String; 8] { [String::from("server"), String::from("drive"), String::from("rack"), String::from("alarm"), String::from("lockdown"), String::from("blackout"), String::from("bodyKo"), String::from("bodyDead")] }

/// (src/game/dialogue.ts:1676 HANDLER_LINES: a table nothing writes)
pub fn handler_lines() -> JsMap<String, Vec<crate::dialogue::HANDLERLINESValueItem>> { { let mut t = JsMap::new(); t.set(String::from("server"), alloc::vec![crate::dialogue::HANDLERLINESValueItem { h: String::from("That's you in. Rack {rack}, east bank. Gently."), s: String::from("Gently is all I've got left at this hour."), h_own: None, s_own: None, h_now: Some(String::from("You're in. Rack {rack}, east bank. Gently.")) }, crate::dialogue::HANDLERLINESValueItem { h: String::from("Server room. Try not to lean on anything load-bearing."), s: String::from("At my age everything I lean on is load-bearing."), h_own: None, s_own: None, h_now: None }, crate::dialogue::HANDLERLINESValueItem { h: String::from("Rack {rack}. And lift with the legs this time."), s: String::from("The legs filed a grievance in the lobby."), h_own: None, s_own: None, h_now: None }]); t.set(String::from("drive"), alloc::vec![crate::dialogue::HANDLERLINESValueItem { h: String::from("Drive shows dark on my board. Nicely done."), s: String::from("Tell my knees. They did the crouching."), h_own: None, s_own: None, h_now: None }, crate::dialogue::HANDLERLINESValueItem { h: String::from("That's the drive. You can stop holding your breath."), s: String::from("That was my back, not my breath."), h_own: None, s_own: None, h_now: None }, crate::dialogue::HANDLERLINESValueItem { h: String::from("Got it. Fire exit, east end. No sightseeing."), s: String::from("I've seen it. It's an office. They're all this office."), h_own: None, s_own: None, h_now: None }]); t.set(String::from("rack"), alloc::vec![crate::dialogue::HANDLERLINESValueItem { h: String::from("They've noticed the rack. Someone's coming to frown at it."), s: String::from("Let him frown. I'll be elsewhere."), h_own: None, s_own: None, h_now: None }, crate::dialogue::HANDLERLINESValueItem { h: String::from("Heads up— a ticket just opened on your rack. Fast, for them."), s: String::from("First time for everything."), h_own: None, s_own: None, h_now: None }]); t.set(String::from("alarm"), alloc::vec![crate::dialogue::HANDLERLINESValueItem { h: String::from("That sounded expensive. You in one piece?"), s: String::from("Define piece. I'm upright."), h_own: None, s_own: None, h_now: None }, crate::dialogue::HANDLERLINESValueItem { h: String::from("Floor's awake now. So am I, thanks for that."), s: String::from("You were asleep?"), h_own: Some(String::from("They're awake now. So am I, thanks for that.")), s_own: None, h_now: None }, crate::dialogue::HANDLERLINESValueItem { h: String::from("Overtime just kicked in, if that helps."), s: String::from("Helps the invoice. Not the corridor."), h_own: None, s_own: Some(String::from("Helps the invoice. Not me.")), h_now: None }]); t.set(String::from("lockdown"), alloc::vec![crate::dialogue::HANDLERLINESValueItem { h: String::from("They've paired up and locked down. Old school."), s: String::from("Old school I can do. I am old school."), h_own: None, s_own: None, h_now: None }, crate::dialogue::HANDLERLINESValueItem { h: String::from("Lockdown. Doors, pairs, the lot. Your doing?"), s: String::from("Allegedly."), h_own: None, s_own: None, h_now: Some(String::from("Lockdown. Doors, pairs, the works. Your doing?")) }]); t.set(String::from("blackout"), alloc::vec![crate::dialogue::HANDLERLINESValueItem { h: String::from("Board says you killed the floor's lights. Subtle."), s: String::from("Subtle was twenty years ago. This is efficient."), h_own: Some(String::from("Board says you killed their lights. Subtle.")), s_own: None, h_now: None }, crate::dialogue::HANDLERLINESValueItem { h: String::from("Blackout. Someone's walking to a breaker, armed and annoyed."), s: String::from("Annoyed I can work with."), h_own: None, s_own: None, h_now: None }]); t.set(String::from("bodyKo"), alloc::vec![crate::dialogue::HANDLERLINESValueItem { h: String::from("They've found your sleeper. He'll wake up embarrassed."), s: String::from("We've all been there."), h_own: None, s_own: None, h_now: None }, crate::dialogue::HANDLERLINESValueItem { h: String::from("Net says man down, breathing. Yours?"), s: String::from("Mine breathe. House style."), h_own: None, s_own: None, h_now: None }]); t.set(String::from("bodyDead"), alloc::vec![crate::dialogue::HANDLERLINESValueItem { h: String::from("They've found him. That's on the record now."), s: String::from("So is the rest of tonight."), h_own: None, s_own: None, h_now: None }, crate::dialogue::HANDLERLINESValueItem { h: String::from("Man down on their net. Not breathing."), s: String::from("I heard. Keep the channel clear."), h_own: None, s_own: None, h_now: None }]); t } }

/// (src/game/dialogue.ts:1764 AMBIENT: a table nothing writes)
pub fn ambient() -> crate::dialogue::AMBIENT { crate::dialogue::AMBIENT { pair_m: 4.5, cooldown: 70.0, first_after: 20.0, gap: 0.45, face_rate: 2.5 } }

/// (src/game/dialogue.ts:1769 AMBIENT_PAIRS: a table nothing writes)
pub fn ambient_pairs() -> Vec<crate::dialogue::PairTalk> { alloc::vec![crate::dialogue::PairTalk { id: String::from("sensor"), lines: alloc::vec![crate::dialogue::PairTalkLinesItem { who: 0.0, text: String::from("They've put a sensor in the gents. Little red eye."), own: None }, crate::dialogue::PairTalkLinesItem { who: 1.0, text: String::from("That's the air freshener."), own: None }, crate::dialogue::PairTalkLinesItem { who: 0.0, text: String::from("It clicked at me."), own: None }, crate::dialogue::PairTalkLinesItem { who: 1.0, text: String::from("It does that. It's judging, not recording."), own: None }], note: None, office: Some(true) }, crate::dialogue::PairTalk { id: String::from("thunk"), lines: alloc::vec![crate::dialogue::PairTalkLinesItem { who: 0.0, text: String::from("Hear that? Sort of a thunk."), own: None }, crate::dialogue::PairTalkLinesItem { who: 1.0, text: String::from("Vending machine. Drops one at midnight."), own: None }, crate::dialogue::PairTalkLinesItem { who: 0.0, text: String::from("Drops one what?"), own: None }, crate::dialogue::PairTalkLinesItem { who: 1.0, text: String::from("Nobody's ever gone to look."), own: None }], note: None, office: Some(true) }, crate::dialogue::PairTalk { id: String::from("pencil"), lines: alloc::vec![crate::dialogue::PairTalkLinesItem { who: 0.0, text: String::from("{Other} did the sheet in pencil again."), own: None }, crate::dialogue::PairTalkLinesItem { who: 1.0, text: String::from("So?"), own: None }, crate::dialogue::PairTalkLinesItem { who: 0.0, text: String::from("So Thursday never officially happened."), own: None }, crate::dialogue::PairTalkLinesItem { who: 1.0, text: String::from("Lucky Thursday."), own: None }], note: None, office: None }, crate::dialogue::PairTalk { id: String::from("pin"), lines: alloc::vec![crate::dialogue::PairTalkLinesItem { who: 0.0, text: String::from("They redid the pad on {padDoor}. {Code} now."), own: None }, crate::dialogue::PairTalkLinesItem { who: 1.0, text: String::from("Out loud. In a corridor. Lovely."), own: Some(String::from("Out loud. Lovely.")) }, crate::dialogue::PairTalkLinesItem { who: 0.0, text: String::from("It's on a sticky ON the pad. I wrote it big."), own: None }, crate::dialogue::PairTalkLinesItem { who: 1.0, text: String::from("You ARE security. Somehow."), own: Some(String::from("And you GUARD the thing. Somehow.")) }], note: Some(String::from("the keypad's real digits, said aloud (devices.ts overhear: the player learns them iff the line CARRIES to him — col.sound, the guards' own ears the other way round); {code} / {padDoor} resolve off the pad nearest the pair, and a level with no pad never deals this one")), office: None }, crate::dialogue::PairTalk { id: String::from("overtime"), lines: alloc::vec![crate::dialogue::PairTalkLinesItem { who: 0.0, text: String::from("Overtime sheet's gone digital."), own: None }, crate::dialogue::PairTalkLinesItem { who: 1.0, text: String::from("Meaning?"), own: None }, crate::dialogue::PairTalkLinesItem { who: 0.0, text: String::from("Meaning it rounds down on its own now."), own: None }, crate::dialogue::PairTalkLinesItem { who: 1.0, text: String::from("Progress."), own: None }], note: None, office: Some(true) }, crate::dialogue::PairTalk { id: String::from("poster"), lines: alloc::vec![crate::dialogue::PairTalkLinesItem { who: 0.0, text: String::from("New poster by the lifts. 'Own the outcome.'"), own: None }, crate::dialogue::PairTalkLinesItem { who: 1.0, text: String::from("Which outcome?"), own: None }, crate::dialogue::PairTalkLinesItem { who: 0.0, text: String::from("Doesn't say. Eagle, though. Big eagle."), own: None }, crate::dialogue::PairTalkLinesItem { who: 1.0, text: String::from("I'll own the eagle."), own: None }], note: None, office: Some(true) }, crate::dialogue::PairTalk { id: String::from("jug"), lines: alloc::vec![crate::dialogue::PairTalkLinesItem { who: 0.0, text: String::from("Who made the six o'clock jug?"), own: None }, crate::dialogue::PairTalkLinesItem { who: 1.0, text: String::from("Day shift. As a warning."), own: None }, crate::dialogue::PairTalkLinesItem { who: 0.0, text: String::from("I drank it."), own: None }, crate::dialogue::PairTalkLinesItem { who: 1.0, text: String::from("Then you've been warned."), own: None }], note: None, office: None }, crate::dialogue::PairTalk { id: String::from("photos"), lines: alloc::vec![crate::dialogue::PairTalkLinesItem { who: 0.0, text: String::from("They're redoing badge photos Monday."), own: None }, crate::dialogue::PairTalkLinesItem { who: 1.0, text: String::from("Mine's from nine kilos ago."), own: None }, crate::dialogue::PairTalkLinesItem { who: 0.0, text: String::from("Mine blinks. In a photo. Somehow."), own: None }], note: None, office: Some(true) }, crate::dialogue::PairTalk { id: String::from("costcentre"), lines: alloc::vec![crate::dialogue::PairTalkLinesItem { who: 0.0, text: String::from("Town hall called us 'a cost centre'."), own: None }, crate::dialogue::PairTalkLinesItem { who: 1.0, text: String::from("We're a corridor."), own: None }, crate::dialogue::PairTalkLinesItem { who: 0.0, text: String::from("A cost corridor."), own: None }, crate::dialogue::PairTalkLinesItem { who: 1.0, text: String::from("Walk it, then. Cheaply."), own: None }], note: None, office: Some(true) }] }

/// (src/game/dialogue.ts:1784 NEEDS_LINES: a table nothing writes)
pub fn needs_lines_table() -> crate::dialogue::NEEDSLINES { crate::dialogue::NEEDSLINES { announce: { let mut t = JsMap::new(); t.set(String::from("coffee"), alloc::vec![String::from("Two minutes — grabbing a coffee."), String::from("Coffee. Back in two."), String::from("I'm doing a coffee run. Two minutes.")]); t.set(String::from("smoke"), alloc::vec![String::from("Going for a smoke by the fire door."), String::from("Quick smoke. Fire exit. Back in five."), String::from("Nipping out the fire door for one.")]); t.set(String::from("water"), alloc::vec![String::from("Getting some water. One minute."), String::from("Throat's gone. Water — one minute."), String::from("Cooler run. Back in a sec.")]); t.set(String::from("stretch"), alloc::vec![String::from("Stretching my legs — lobby and back."), String::from("Legs have seized. Quick lap of the lobby."), String::from("Walking the lobby loop. Back in a minute.")]); t }, announce_own: { let mut t = JsMap::new(); t.set(String::from("coffee"), alloc::vec![String::from("Two minutes — grabbing a coffee."), String::from("Coffee. Back in two.")]); t.set(String::from("smoke"), alloc::vec![String::from("Going out for a smoke."), String::from("Quick smoke. Back in five.")]); t.set(String::from("water"), alloc::vec![String::from("Getting some water. One minute."), String::from("Throat's gone. Water — one minute.")]); t.set(String::from("stretch"), alloc::vec![String::from("Stretching my legs. Back in a minute."), String::from("Legs have seized. Quick lap.")]); t }, now: { let mut t = JsMap::new(); t.set(String::from("Nipping out the fire door for one."), String::from("Stepping out the fire door for a smoke.")); t.set(String::from("Legs have seized. Quick lap of the lobby."), String::from("My legs are stiff. Quick lap around the lobby.")); t.set(String::from("Legs have seized. Quick lap."), String::from("My legs are stiff. Quick lap.")); t }, reply: alloc::vec![String::from("Go on then."), String::from("Again?"), String::from("Bring me one."), String::from("Don't be long."), String::from("Clock's running."), String::from("I'll hold the fort. Such as it is.")], reply_other: alloc::vec![String::from("Don't be long — {other} wanders enough for two."), String::from("Fine. If {other} asks, I never saw you.")], radio: String::from("{who} — two minutes, {short}."), short: { let mut t = JsMap::new(); t.set(String::from("coffee"), String::from("coffee")); t.set(String::from("smoke"), String::from("smoke, fire door")); t.set(String::from("water"), String::from("water")); t.set(String::from("stretch"), String::from("stretching my legs")); t }, short_own: { let mut t = JsMap::new(); t.set(String::from("smoke"), String::from("smoke")); t }, back: alloc::vec![String::from("Right. Where was I."), String::from("Okay. Back on it."), String::from("As I was."), String::from("…and back to the glamour.")] } }

/// (src/game/dialogue.ts:1820 CIV_LINES: a table nothing writes)
pub fn civ_lines() -> crate::dialogue::CIVLINES { crate::dialogue::CIVLINES { glimpse: alloc::vec![String::from("Hello…?"), String::from("Is somebody there?"), String::from("…who's that?"), String::from("Hello? …Security?")], shrug: alloc::vec![String::from("…nothing. Jumpy tonight."), String::from("Hearing things."), String::from("…get a grip, {name}."), String::from("This building.")], gasp: { let mut t = JsMap::new(); t.set(String::from("sam"), alloc::vec![String::from("— oh god —"), String::from("Who — WHO ARE YOU —"), String::from("No. No no no —")]); t.set(String::from("gun"), alloc::vec![String::from("He's got a GUN —"), String::from("GUN — oh god, oh god —"), String::from("— that's a GUN —")]); t.set(String::from("body"), alloc::vec![String::from("Oh god — oh GOD —"), String::from("He's — is he DEAD —?"), String::from("Somebody's — oh no. No —")]); t.set(String::from("gunfire"), alloc::vec![String::from("Was that a SHOT —?!"), String::from("— shooting — oh god —"), String::from("That was a GUN —")]); t.set(String::from("shot"), alloc::vec![String::from("What was — was that a SHOT?"), String::from("— that's — no. NO —")]); t.set(String::from("bang"), alloc::vec![String::from("— what the — !"), String::from("JESUS — what WAS that —")]); t.set(String::from("scream"), alloc::vec![String::from("What — what's HAPPENING —?!"), String::from("{other}?! — oh god —")]); t.set(String::from("noise"), alloc::vec![String::from("What was THAT —"), String::from("— okay. No. No.")]); t.set(String::from("grabbed"), alloc::vec![String::from("— he GRABBED me — he —"), String::from("GET AWAY FROM ME —")]); t.set(String::from("panel"), alloc::vec![String::from("— oh god —")]); t }, flee: alloc::vec![String::from("HELP! SECURITY!"), String::from("SECURITY —!"), String::from("Somebody HELP —!"), String::from("HELP —!")], report: { let mut t = JsMap::new(); t.set(String::from("sam"), alloc::vec![String::from("There's someone in {room} — a man — all in black —"), String::from("A MAN — {room} — creeping about in the dark —"), String::from("{room} — somebody's IN there — I saw him —")]); t.set(String::from("gun"), alloc::vec![String::from("A man in {room} — he's got a GUN —"), String::from("{room} — there's a man with a GUN in there —")]); t.set(String::from("body"), alloc::vec![String::from("{room} — there's a man on the FLOOR — one of yours —"), String::from("One of your lot — DOWN — {room} — not moving —")]); t.set(String::from("gunfire"), alloc::vec![String::from("SHOOTING — {room} — I heard it — a gun —"), String::from("That was a gunshot — {room} — it WAS —")]); t.set(String::from("shot"), alloc::vec![String::from("A bang — like a shot — {room} —"), String::from("Something went off in {room} — like a gun but — quiet —")]); t.set(String::from("bang"), alloc::vec![String::from("An EXPLOSION — {room} — a flash and —"), String::from("Something BLEW UP — {room} —")]); t.set(String::from("scream"), alloc::vec![String::from("{room} — screaming — somebody screamed —"), String::from("I heard SCREAMING — {room} —")]); t.set(String::from("noise"), alloc::vec![String::from("{room} — a crash — somebody is IN there —")]); t.set(String::from("grabbed"), alloc::vec![String::from("He GRABBED me — {room} — a man — he had a KNIFE at me —"), String::from("A man — he had me by the THROAT — {room} —")]); t.set(String::from("panel"), alloc::vec![String::from("There's someone in {room} —")]); t }, guard_ack: crate::dialogue::CIVLINESGuardAck { someone: alloc::vec![String::from("All right — all right. Stay here. I'll look."), String::from("Okay. Get behind the desk and stay put."), String::from("Easy. Stay here, stay down. I'll go and see.")], armed: alloc::vec![String::from("DOWN — get down and STAY down."), String::from("Behind me. On the floor. Do NOT move."), String::from("Get down. Stay there. — control —")] }, guard_radio: { let mut t = JsMap::new(); t.set(String::from("someone"), String::from("control, {cs} — {name}'s seen somebody, {room}. walking it now")); t.set(String::from("armed"), String::from("control, {cs} — {name} saw an ARMED man, {room}. weapons out, I'm going")); t.set(String::from("body"), String::from("control, {cs} — {name} says there's a man DOWN, {room}. going now")); t.set(String::from("gunfire"), String::from("control, {cs} — {name} heard it too, {room}. moving")); t.set(String::from("grabbed"), String::from("control, {cs} — he had {name} by the throat, {room}. armed. going now")); t }, phone: alloc::vec![String::from("Security? — it's {name}, fourth floor — there's a MAN up here — {room} —"), String::from("…yes — {room} — I SAW him — please, just — send someone —")], phone_armed: alloc::vec![String::from("Security — {name}, on four — a man with a GUN — {room} —"), String::from("— YES a gun — {room} — hurry — please —")], control: { let mut t = JsMap::new(); t.set(String::from("someone"), String::from("all callsigns, control — {name} on four just rang in a prowler, {room}. somebody walk it. eyes open")); t.set(String::from("armed"), String::from("all callsigns, control — {name} on four rang it in: ARMED intruder, {room}. weapons out, get there")); t.set(String::from("body"), String::from("all callsigns, control — {name} says a man is DOWN, {room}. weapons out, get there now")); t.set(String::from("outside"), String::from("all callsigns, control — {name} just came off your floor at a run. says there is a man up there, {room}. treat it as real")); t }, cower: alloc::vec![String::from("please — please —"), String::from("I didn't see anything — I DIDN'T —"), String::from("oh god oh god oh god"), String::from("don't — don't —"), String::from("— breathe — breathe —")], held_fear: alloc::vec![String::from("Please — I just CLEAN here —"), String::from("Don't — please — I've got kids —"), String::from("okay — okay — OKAY —"), String::from("I won't scream — I won't — please —")], held_fear_worker: alloc::vec![String::from("Please — I'm just — I do the ACCOUNTS —"), String::from("Take the laptop — take anything — please —"), String::from("okay — okay — OKAY —")], intel: { let mut t = JsMap::new(); t.set(String::from("cleaner"), alloc::vec![String::from("The side door — the number's ON it. A sticky. Seven three nine one."), String::from("Three of them tonight. The big one walks the corridor. All night."), String::from("The one in here — Novak — he goes for water. The far corner. Often.")]); t.set(String::from("worker"), alloc::vec![String::from("Their room's downstairs. They come up the west stairs. The steel door."), String::from("Reyes does the front. Sits in the break room half the night."), String::from("There's a camera on the corridor. East end. It swings.")]); t }, dry: alloc::vec![String::from("That's ALL I know — I swear —"), String::from("I don't KNOW anything else —"), String::from("Please — that's everything —")], comply: crate::dialogue::CIVLINESComply { sam: alloc::vec![String::from("Down. Stay down, stay quiet — and you go home tonight."), String::from("On the floor. Not a sound. You never saw me."), String::from("Face the wall. Count to a thousand. Slowly.")], civ: alloc::vec![String::from("okay — okay —"), String::from("I won't — I won't move —"), String::from("…yes. yes. okay.")] }, sam_ask: alloc::vec![String::from("Quietly. Who else is on this floor?"), String::from("Breathe. Now — what do you know that I want?"), String::from("Shh. The guards. Where, and how many?"), String::from("Easy. Talk to me and this ends well.")], found: alloc::vec![String::from("Hey — hey. It's all right. What happened?"), String::from("{name}? — what is it, what did you see?"), String::from("{name} — look at me. What HAPPENED?")], found_down: crate::dialogue::CIVLINESFoundDown { cold: String::from("control, {cs} — it's {name}, the {label} — {she}'s DOWN. out cold, breathing. someone's on this floor"), dead: String::from("control, {cs} — {name}'s DEAD. the {label}. somebody's KILLED {her} — lock it down, all of it") }, outrage: String::from("all callsigns, control — the {label} is dead. whoever this is does not walk out of here. find him"), greet: crate::dialogue::CIVLINESGreet { civ: alloc::vec![String::from("Evening."), String::from("Don't mind me."), String::from("Mind the wet floor."), String::from("Nearly done in here."), String::from("You lot never sleep.")], guard: alloc::vec![String::from("Evening, {name}."), String::from("Late one tonight, {name}?"), String::from("You're all right."), String::from("Careful by the stairs — lift's off again."), String::from("Leave the bins, {name}. Go home.")] }, settle: alloc::vec![String::from("…okay. Okay. Back to it."), String::from("Right. Deep breaths."), String::from("…they've got it. They've got it.")], qrf: String::from("control to all — response team inbound. get the cleaning staff OFF that floor. now"), evac: alloc::vec![String::from("Going — I'm GOING —"), String::from("Out — yes — out —"), String::from("— I'm going, I'm going —")], own: crate::dialogue::CIVLINESOwn { phone: String::from("It's {name} — listen — there's a MAN in here — {room} —"), phone_armed: String::from("It's {name} — a man with a GUN — {room} —"), control: { let mut t = JsMap::new(); t.set(String::from("someone"), String::from("all callsigns, control — {name} just rang in a prowler, {room}. somebody walk it. eyes open")); t.set(String::from("armed"), String::from("all callsigns, control — {name} rang it in: ARMED intruder, {room}. weapons out, get there")); t.set(String::from("outside"), String::from("all callsigns, control — {name} just came out at a run. says there is a man in there, {room}. treat it as real")); t }, held_fear: String::from("Please — I just WORK here —"), intel_pad: String::from("The door with the pad — I've seen them do it. {code}."), intel: alloc::vec![String::from("{n} of them tonight. Walking round. All night.")], sam_ask: String::from("Quietly. Who else is here tonight?"), found_cold: String::from("control, {cs} — it's {name}, the {label} — {she}'s DOWN. out cold, breathing. someone's about") }, now: crate::dialogue::CIVLINESNow { guard_ack: String::from("Okay. Get down somewhere and stay put."), qrf: String::from("control to all — response team inbound. get the civilians OFF that floor. now") }, guide: crate::dialogue::CIVLINESGuide { report: { let mut t = JsMap::new(); t.set(String::from("sam"), alloc::vec![String::from("There's someone in {room} — a man — all in black —"), String::from("A MAN — {room} — sneaking around in the dark —"), String::from("{room} — somebody's IN there — I saw him —")]); t.set(String::from("body"), alloc::vec![String::from("{room} — there's a man on the FLOOR — one of yours —"), String::from("One of your guys — DOWN — {room} — not moving —")]); t }, control: { let mut t = JsMap::new(); t.set(String::from("someone"), String::from("all callsigns, control — {name} on four just called in a prowler, {room}. somebody walk it. eyes open")); t.set(String::from("armed"), String::from("all callsigns, control — {name} on four called it in: ARMED intruder, {room}. weapons out, get there")); t.set(String::from("outside"), String::from("all callsigns, control — {name} just ran off your floor. says there's a man up there, {room}. treat it as real")); t }, greet_civ: alloc::vec![String::from("Evening."), String::from("Don't mind me."), String::from("Mind the wet floor."), String::from("Nearly done in here."), String::from("You guys never sleep.")], greet_guard: alloc::vec![String::from("Evening, {name}."), String::from("Late one tonight, {name}?"), String::from("You're fine."), String::from("Careful by the stairs — elevator's out again."), String::from("Leave the trash, {name}. Go home.")], own: crate::dialogue::CIVLINESGuideOwn { control: { let mut t = JsMap::new(); t.set(String::from("someone"), String::from("all callsigns, control — {name} just called in a prowler, {room}. somebody walk it. eyes open")); t.set(String::from("armed"), String::from("all callsigns, control — {name} called it in: ARMED intruder, {room}. weapons out, get there")); t.set(String::from("outside"), String::from("all callsigns, control — {name} just ran out. says there's a man in there, {room}. treat it as real")); t }, intel: alloc::vec![String::from("{n} of them tonight. Walking around. All night.")], found_cold: String::from("control, {cs} — it's {name}, the {label} — {she}'s DOWN. out cold, breathing. someone's here") } } } }

/// (src/game/dialogue.ts:1945 CIV_COUNT: a table nothing writes)
pub fn civ_count() -> Vec<String> { alloc::vec![String::from("None"), String::from("One"), String::from("Two"), String::from("Three"), String::from("Four"), String::from("Five"), String::from("Six"), String::from("Seven"), String::from("Eight"), String::from("A lot")] }

/// (src/game/dialogue.ts:1958 HOSTAGE_LINES: a table nothing writes)
pub fn hostage_lines_table() -> crate::dialogue::HOSTAGELINES { crate::dialogue::HOSTAGELINES { wait: alloc::vec![String::from("Please — just let us go —"), String::from("We'll do what you say — please —"), String::from("Please. Please."), String::from("I can't feel my hands —"), String::from("— breathe. Just breathe. —"), String::from("I want to go home —"), String::from("We haven't seen your faces — we haven't —")], threat: alloc::vec![String::from("Please — don't — we've done NOTHING —"), String::from("He means it — oh god, he MEANS it —"), String::from("Don't — please, please don't —"), String::from("Somebody — PLEASE —")], aimed: alloc::vec![String::from("No — no, PLEASE —"), String::from("Don't — I'm begging you —"), String::from("Wait — WAIT —")], shot: alloc::vec![String::from("NO —!"), String::from("{other}! — oh god — {other} —"), String::from("He SHOT {other} — he —"), String::from("— no no no no —")], entry: alloc::vec![String::from("— what was THAT —"), String::from("They're shooting — oh god —"), String::from("In HERE — we're in HERE —"), String::from("{other} — stay DOWN — stay down —")], alone: alloc::vec![String::from("Hello? — is somebody there —?"), String::from("We're in here — HELP —"), String::from("In here — please —")], secured: alloc::vec![String::from("Don't shoot — please — we're hostages —"), String::from("Oh thank god — thank god —"), String::from("Is it over? — is it OVER —?"), String::from("Are they — are they gone —?")], team: alloc::vec![String::from("stay down. you're safe — we've got you."), String::from("you're all right. stay where you are."), String::from("friendlies. stay down till we say.")] } }

/// (src/game/dialogue.ts:1983 SNATCH_LINES: a table nothing writes)
pub fn snatch_lines_table() -> crate::dialogue::SNATCHLINES { crate::dialogue::SNATCHLINES { running: alloc::vec![String::from("They're here — they're HERE —"), String::from("Not me — not ME —"), String::from("Out — get me OUT —")], cornered: alloc::vec![String::from("Stay back — stay BACK —"), String::from("I've nothing for you — nothing —"), String::from("— no, no, no —")], hands_up: alloc::vec![String::from("don't shoot! don't shoot!"), String::from("All right — all RIGHT — they're up —"), String::from("I'm not armed — look — nothing —")], tied: alloc::vec![String::from("You've the wrong man —"), String::from("I'm not fighting you — easy —"), String::from("They will come for me. You know that.")], held: alloc::vec![String::from("All right — I'm not fighting — all right —"), String::from("Easy — I have no gun — nothing —")], dry: alloc::vec![String::from("I say nothing to you."), String::from("Ask whoever sent you.")] } }
