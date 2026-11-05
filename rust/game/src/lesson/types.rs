//! The types of `lesson`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// a step's done-test: one arm a row of LESSON_TESTS, with the fields that row's `ok` reads (lesson.ts:167-282; a level file's `{ is: '<kind>', … }`, tag from `is`). Unknown: an `is` no row has, which never passes (:109)
#[derive(Clone, Debug)]
pub enum LessonTest {
    Unknown,
    In { area: crate::scene_scenarios::Rect },
    Near { at: V3, r: Option<f64> },
    Flag { flag: String, value: Option<bool> },
    Slot { slot: crate::game::SlotKind, drawn: Option<bool> },
    Pace,
    Meter { which: String, below: Option<f64>, above: Option<f64> },
    Stage { stage: String },
    Door { door: String, state: String },
    Target { name: String, on: bool },
    Men { down: Option<f64> },
    Did { what: String, on: Option<String>, n: Option<f64> },
    Downed { how: String, n: Option<f64> },
    Body { area: crate::scene_scenarios::Rect },
    Calm { route: Option<String> },
    Loaded,
    Reload,
    Launcher { on: String, fired: bool },
    Burst { n: Option<f64> },
    Threw { kind: Option<String>, quick: bool, area: Option<crate::scene_scenarios::Rect> },
    OrderTo { who: crate::orders::Who },
    StackOn { door: String, who: Option<crate::orders::SquadOrdersMenSel>, set: bool },
    PairsOn { doors: Option<f64> },
    Word { doors: Option<f64> },
    Taken { room: String },
    Following,
    Mate { area: Option<crate::scene_scenarios::Rect>, at: Option<V3>, r: Option<f64>, by: Option<f64>, away: Option<f64> },
    Pair { what: String, n: Option<f64> },
    Cue { name: String },
    View { turned: Option<f64>, zoomed: Option<f64>, panned: Option<f64> },
    Squad { area: crate::scene_scenarios::Rect, n: Option<f64> },
    Stacks { doors: Vec<String>, n: Option<f64>, set: bool },
    Watching { at: V3, r: Option<f64>, n: Option<f64> },
    All { of: Vec<crate::lesson::LessonTest> },
    Any { of: Vec<crate::lesson::LessonTest> },
}

/// (TypeScript: StepSeen, src/game/lesson.ts:91)
#[derive(Clone, Debug)]
pub struct StepSeen {
    pub t0: f64,
    pub n: JsMap<String, f64>,
    pub chron: f64,
    pub heard: JsSet<u32>,
    pub held: f64,
    pub done_t: Option<f64>,
    pub idle: Option<bool>,
    pub last_order: Option<u32>,
    pub last_pair: Option<u32>,
    pub chron0: f64,
    pub held_at: f64,
    pub words: JsSet<DoorId>,
    pub yaw0: f64,
    pub dist0: f64,
    pub yaw_last: f64,
    pub turned: f64,
    pub done_i: Option<f64>,
}
impl StepSeen {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        StepSeen {
            t0: 0.0,
            n: JsMap::new(),
            chron: 0.0,
            heard: JsSet::new(),
            held: 0.0,
            done_t: None,
            idle: None,
            last_order: None,
            last_pair: None,
            chron0: 0.0,
            held_at: 0.0,
            words: JsSet::new(),
            yaw0: 0.0,
            dist0: 0.0,
            yaw_last: 0.0,
            turned: 0.0,
            done_i: None,
        }
    }
}
impl Default for StepSeen { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at quiet StepSeen.lastOrder, quiet StepSeen.lastOrder)
#[derive(Clone, Copy, Debug)]
pub struct StepSeenLastOrder {
}
impl StepSeenLastOrder {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        StepSeenLastOrder {
        }
    }
}
impl Default for StepSeenLastOrder { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at quiet StepSeen.lastPair, quiet StepSeen.lastPair)
#[derive(Clone, Copy, Debug)]
pub struct StepSeenLastPair {
}
impl StepSeenLastPair {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        StepSeenLastPair {
        }
    }
}
impl Default for StepSeenLastPair { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: TestKind, src/game/lesson.ts:104)
#[derive(Clone, Copy, Debug)]
pub struct TestKind {
}
impl TestKind {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        TestKind {
        }
    }
}
impl Default for TestKind { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum PairSel {
    All,
    Red,
    Blue,
}
impl PairSel {
    pub const fn as_str(self) -> &'static str {
        match self {
            PairSel::All => "all",
            PairSel::Red => "red",
            PairSel::Blue => "blue",
        }
    }
}

/// (TypeScript: LessonSnap, src/game/lesson.ts:298)
#[derive(Clone, Debug)]
pub struct LessonSnap {
    pub i: f64,
    pub done: Vec<(String, f64)>,
}
impl LessonSnap {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        LessonSnap {
            i: 0.0,
            done: Vec::new(),
        }
    }
}
impl Default for LessonSnap { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Lesson, src/game/lesson.ts:300)
#[derive(Clone, Debug)]
pub struct Lesson {
    pub i: f64,
    pub done_at: JsMap<String, f64>,
    pub log: Vec<String>,
    pub seen: Option<crate::lesson::StepSeen>,
    pub run: Option<u32>,
}
impl Lesson {
    /// what a new one holds: the TypeScript's field initialisers, before its constructor
    pub fn blank() -> Self {
        Lesson {
            i: 0.0,
            done_at: JsMap::new(),
            log: Vec::new(),
            seen: None,
            run: None,
        }
    }
}
impl Default for Lesson { fn default() -> Self { Self::blank() } }

/// (TypeScript: inline at quiet Lesson.run, quiet Lesson.run)
#[derive(Clone, Copy, Debug)]
pub struct LessonRun {
}
impl LessonRun {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        LessonRun {
        }
    }
}
impl Default for LessonRun { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/lesson.ts:392 objective ->, src/game/lesson.ts:392)
#[derive(Clone, Debug)]
pub struct LessonObjective {
    pub text: String,
    pub pos: Option<V3>,
}
impl LessonObjective {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        LessonObjective {
            text: String::new(),
            pos: None,
        }
    }
}
impl Default for LessonObjective { fn default() -> Self { Self::zeroed() } }

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct LessonStats {
    pub started: i32,
    pub steps: i32,
    pub finished: i32,
}

/// (src/game/lesson.ts:37 WHISTLE_LINE)
pub const WHISTLE_LINE: &str = "/\\bV whistles\\b/g";

/// (src/game/lesson.ts:39 WHEEL_LINE)
pub const WHEEL_LINE: &str = "/\\bHold Space, wheel to kick, let go\\b/g";

/// (src/game/lesson.ts:39 WHEEL_RING)
pub const WHEEL_RING: &str = "Hold Space, then 2 kicks it";

/// (src/game/lesson.ts:41 GRAB_BEHIND)
pub const GRAB_BEHIND: &str = "/\\b([Pp])ress \\[?Space\\]? behind (him|the near guard) to grab him\\b/g";

/// (src/game/lesson.ts:41 GRAB_KEY)
pub const GRAB_KEY: &str = "/\\b([Pp])ress \\[?Space\\]? to grab him\\b/g";

/// (src/game/lesson.ts:44 KNIFE_OR)
pub const KNIFE_OR: &str = "/\\bRight click to knock him out, or left click to use the knife\\b/g";

/// (src/game/lesson.ts:44 KNIFE_HIM)
pub const KNIFE_HIM: &str = "/\\bleft click to knife him\\b/g";

/// (src/game/lesson.ts:44 KNIFE_KILL)
pub const KNIFE_KILL: &str = "/\\bleft-click to kill him with the knife\\b/g";

/// (src/game/lesson.ts:51 TOKENS: a table nothing writes)
pub fn tokens() -> Vec<(crate::Opaque, crate::FnField)> { alloc::vec![(crate::Opaque, crate::FnField), (crate::Opaque, crate::FnField), (crate::Opaque, crate::FnField), (crate::Opaque, crate::FnField)] }

/// (src/game/lesson.ts:58 LESSON_TOKENS: a table nothing writes)
pub fn lesson_tokens() -> Vec<String> { alloc::vec![String::from("grab him"), String::from("grab him from behind"), String::from("knock him out"), String::from("take him out")] }

/// (src/game/lesson.ts:59 TOKEN)
pub const TOKEN: &str = "/\\{([A-Za-z][A-Za-z' ]*)\\}/g";

/// (src/game/lesson.ts:129 PAIR_WHATS: a table nothing writes)
pub fn pair_whats() -> Vec<String> { alloc::vec![String::from("mark"), String::from("now"), String::from("set"), String::from("go"), String::from("go:F"), String::from("go:hands"), String::from("down")] }

/// (src/game/lesson.ts:132 FLAGS: a table nothing writes)
pub fn flags() -> JsMap<String, crate::FnField> { { let mut t = JsMap::new(); t.set(String::from("crouch"), crate::FnField); t.set(String::from("sprint"), crate::FnField); t.set(String::from("wall"), crate::FnField); t.set(String::from("peek"), crate::FnField); t.set(String::from("lean"), crate::FnField); t.set(String::from("nv"), crate::FnField); t.set(String::from("light"), crate::FnField); t.set(String::from("holstered"), crate::FnField); t.set(String::from("gun"), crate::FnField); t.set(String::from("ringsOut"), crate::FnField); t.set(String::from("shellsOut"), crate::FnField); t.set(String::from("auto"), crate::FnField); t.set(String::from("setup"), crate::FnField); t.set(String::from("setupShut"), crate::FnField); t.set(String::from("dragging"), crate::FnField); t.set(String::from("dark"), crate::FnField); t.set(String::from("chargeSet"), crate::FnField); t.set(String::from("chargeClear"), crate::FnField); t.set(String::from("wallBlown"), crate::FnField); t.set(String::from("holding"), crate::FnField); t.set(String::from("picking"), crate::FnField); t.set(String::from("coding"), crate::FnField); t.set(String::from("menu"), crate::FnField); t.set(String::from("ownRing"), crate::FnField); t.set(String::from("silent"), crate::FnField); t } }

/// (src/game/lesson.ts:165 LESSON_TESTS: a table nothing writes)
pub fn lesson_tests() -> JsMap<String, crate::lesson::TestKind> { { let mut t = JsMap::new(); t.set(String::from("in"), crate::lesson::TestKind {  }); t.set(String::from("near"), crate::lesson::TestKind {  }); t.set(String::from("flag"), crate::lesson::TestKind {  }); t.set(String::from("slot"), crate::lesson::TestKind {  }); t.set(String::from("pace"), crate::lesson::TestKind {  }); t.set(String::from("meter"), crate::lesson::TestKind {  }); t.set(String::from("stage"), crate::lesson::TestKind {  }); t.set(String::from("door"), crate::lesson::TestKind {  }); t.set(String::from("target"), crate::lesson::TestKind {  }); t.set(String::from("men"), crate::lesson::TestKind {  }); t.set(String::from("did"), crate::lesson::TestKind {  }); t.set(String::from("downed"), crate::lesson::TestKind {  }); t.set(String::from("body"), crate::lesson::TestKind {  }); t.set(String::from("calm"), crate::lesson::TestKind {  }); t.set(String::from("loaded"), crate::lesson::TestKind {  }); t.set(String::from("reload"), crate::lesson::TestKind {  }); t.set(String::from("launcher"), crate::lesson::TestKind {  }); t.set(String::from("burst"), crate::lesson::TestKind {  }); t.set(String::from("threw"), crate::lesson::TestKind {  }); t.set(String::from("orderTo"), crate::lesson::TestKind {  }); t.set(String::from("stackOn"), crate::lesson::TestKind {  }); t.set(String::from("pairsOn"), crate::lesson::TestKind {  }); t.set(String::from("word"), crate::lesson::TestKind {  }); t.set(String::from("taken"), crate::lesson::TestKind {  }); t.set(String::from("following"), crate::lesson::TestKind {  }); t.set(String::from("mate"), crate::lesson::TestKind {  }); t.set(String::from("pair"), crate::lesson::TestKind {  }); t.set(String::from("cue"), crate::lesson::TestKind {  }); t.set(String::from("view"), crate::lesson::TestKind {  }); t.set(String::from("squad"), crate::lesson::TestKind {  }); t.set(String::from("stacks"), crate::lesson::TestKind {  }); t.set(String::from("watching"), crate::lesson::TestKind {  }); t.set(String::from("all"), crate::lesson::TestKind {  }); t.set(String::from("any"), crate::lesson::TestKind {  }); t } }

/// (src/game/lesson.ts:284 STAGES: a table nothing writes)
pub fn stages() -> Vec<String> { alloc::vec![String::from("infiltrate"), String::from("drive"), String::from("exfil"), String::from("done")] }

/// (src/game/lesson.ts:285 HOWS: a table nothing writes)
pub fn hows() -> Vec<String> { alloc::vec![String::from("takedown"), String::from("choke"), String::from("whip"), String::from("ring"), String::from("knife")] }
