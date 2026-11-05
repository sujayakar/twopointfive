#![allow(unused_variables)]
//! Every function and method of `lesson`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/lesson.ts:42 stalkLine)
pub fn stalk_line(line: &str) -> String { todo!() }

/// (src/game/lesson.ts:45 bareLine)
pub fn bare_line(line: &str) -> String { todo!() }

/// (src/game/lesson.ts:59 lower)
pub fn lower(name: &str) -> String { todo!() }

/// (src/game/lesson.ts:61 lessonToken)
pub fn lesson_token(name: &str) -> bool { todo!() }

/// (src/game/lesson.ts:63 tokenLine)
pub fn token_line(line: &str, held: Option<bool>, bare: Option<bool>) -> String { todo!() }

/// (src/game/lesson.ts:69 lessonTokenFaults)
pub fn lesson_token_faults(w: &str, line: &str, cap: f64, row: Option<&str>, always: Option<bool>) -> Vec<String> { todo!() }

/// (src/game/lesson.ts:78 lessonLine)
pub fn lesson_line(g: &mut crate::game::Game, written: &str) -> String { todo!() }

/// (src/game/lesson.ts:108 passes)
pub fn passes(g: &mut crate::game::Game, t: &crate::lesson::LessonTest, s: &crate::lesson::StepSeen) -> bool { todo!() }

/// (src/game/lesson.ts:113 after)
pub fn after(log: &[u32], last: Option<u32>) -> usize { todo!() }

/// (src/game/lesson.ts:114 num)
pub fn num(v: crate::Opaque) { todo!() }

/// (src/game/lesson.ts:115 str)
pub fn str(v: crate::Opaque) { todo!() }

/// (src/game/lesson.ts:116 rect)
pub fn rect(v: crate::Opaque) { todo!() }

/// (src/game/lesson.ts:117 vec)
pub fn vec(v: crate::Opaque) { todo!() }

/// (src/game/lesson.ts:118 inside)
pub fn inside(r: crate::scene_scenarios::Rect, p: V3) -> bool { todo!() }

/// (src/game/lesson.ts:120 who)
pub fn who(g: &mut crate::game::Game) -> PlayerId { todo!() }

/// (src/game/lesson.ts:123 onStack)
pub fn on_stack(g: &mut crate::game::Game, m: GuardId) -> Option<String> { todo!() }

/// (src/game/lesson.ts:125 downed)
pub fn downed(g: &mut crate::game::Game, x: GuardId) -> bool { todo!() }

/// (src/game/lesson.ts:127 partnerOf)
pub fn partner_of(g: &mut crate::game::Game) -> Option<GuardId> { todo!() }

/// (src/game/lesson.ts:286 band)
pub fn band(v: f64, t: &crate::lesson::LessonTest) -> bool { todo!() }

/// (src/game/lesson.ts:287 bandCheck)
pub fn band_check(t: &crate::lesson::LessonTest) -> Option<String> { todo!() }

/// (src/game/lesson.ts:288 ofCheck)
pub fn of_check(g: &mut crate::game::Game, t: &crate::lesson::LessonTest) -> Option<String> { todo!() }

/// (src/game/lesson.ts:289 freeIn)
pub fn free_in(g: &mut crate::game::Game, r: crate::scene_scenarios::Rect) -> bool { todo!() }

/// (src/game/lesson.ts:291 testFault)
pub fn test_fault(g: &mut crate::game::Game, t: &crate::lesson::LessonTest) -> Option<String> { todo!() }

/// (src/game/lesson.ts:308 Lesson.constructor)
pub fn lesson_new(g: &mut crate::game::Game) -> crate::lesson::Lesson { todo!() }

/// (src/game/lesson.ts:311 Lesson.data)
pub fn lesson_data(g: &mut crate::game::Game) -> Option<crate::scene_scenarios::LessonData> { todo!() }

/// (src/game/lesson.ts:320 Lesson.at)
pub fn lesson_at(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/game/lesson.ts:322 Lesson.index)
pub fn lesson_index(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/game/lesson.ts:323 Lesson.step)
pub fn lesson_step(g: &mut crate::game::Game) -> Option<crate::scene_scenarios::LessonStep> { todo!() }

/// (src/game/lesson.ts:324 Lesson.finished)
pub fn lesson_finished(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/lesson.ts:327 Lesson.tick)
pub fn lesson_tick(g: &mut crate::game::Game, dt: f64) { todo!() }

/// (src/game/lesson.ts:348 Lesson.settling)
pub fn lesson_settling(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/lesson.ts:349 Lesson.advance)
pub fn lesson_advance(g: &mut crate::game::Game, l: &crate::scene_scenarios::LessonData, t: &crate::scene_scenarios::LessonStep) { todo!() }

/// (src/game/lesson.ts:360 Lesson.fresh)
pub fn lesson_fresh(g: &mut crate::game::Game, back: Option<f64>) -> crate::lesson::StepSeen { todo!() }

/// (src/game/lesson.ts:363 Lesson.count)
pub fn lesson_count(g: &mut crate::game::Game, s: &mut crate::lesson::StepSeen) { todo!() }

/// (src/game/lesson.ts:379 Lesson.snapshot)
pub fn lesson_snapshot(g: &mut crate::game::Game) -> crate::lesson::LessonSnap { todo!() }

/// (src/game/lesson.ts:382 Lesson.restore)
pub fn lesson_restore(g: &mut crate::game::Game, s: Option<&crate::lesson::LessonSnap>) { todo!() }

/// (src/game/lesson.ts:390 Lesson.over)
pub fn lesson_over(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/lesson.ts:392 Lesson.objective)
pub fn lesson_objective(g: &mut crate::game::Game) -> Option<crate::lesson::LessonObjective> { todo!() }

/// (src/game/lesson.ts:398 Lesson.summary)
pub fn lesson_summary(g: &mut crate::game::Game) -> String { todo!() }

/// (src/game/lesson.ts:403 Lesson.note)
pub fn lesson_note(g: &mut crate::game::Game, s: &str) { todo!() }

/// (src/game/lesson.ts:408 lineOf)
pub fn line_of(g: &mut crate::game::Game, t: &crate::scene_scenarios::LessonStep) -> String { todo!() }

/// (src/game/lesson.ts:410 stepLine)
pub fn step_line(g: &mut crate::game::Game, t: &crate::scene_scenarios::LessonStep) -> String { todo!() }

/// (src/game/lesson.ts:415 checkLesson)
pub fn check_lesson(g: &mut crate::game::Game) -> Vec<String> { todo!() }

pub mod flags {
    #[allow(unused_imports)]
    use crate::{ids::*, js::*, Quat, V3};
    #[allow(unused_imports)]
    use alloc::{string::String, vec::Vec};
    /// (src/game/lesson.ts:133 FLAGS.crouch)
    pub fn crouch(g: &mut crate::game::Game) -> bool { todo!() }

    /// (src/game/lesson.ts:134 FLAGS.sprint)
    pub fn sprint(g: &mut crate::game::Game) -> bool { todo!() }

    /// (src/game/lesson.ts:135 FLAGS.wall)
    pub fn wall(g: &mut crate::game::Game) -> bool { todo!() }

    /// (src/game/lesson.ts:136 FLAGS.peek)
    pub fn peek(g: &mut crate::game::Game) -> bool { todo!() }

    /// (src/game/lesson.ts:137 FLAGS.lean)
    pub fn lean(g: &mut crate::game::Game) -> bool { todo!() }

    /// (src/game/lesson.ts:138 FLAGS.nv)
    pub fn nv(g: &mut crate::game::Game) -> bool { todo!() }

    /// (src/game/lesson.ts:139 FLAGS.light)
    pub fn light(g: &mut crate::game::Game) -> bool { todo!() }

    /// (src/game/lesson.ts:140 FLAGS.holstered)
    pub fn holstered(g: &mut crate::game::Game) -> bool { todo!() }

    /// (src/game/lesson.ts:141 FLAGS.gun)
    pub fn gun(g: &mut crate::game::Game) -> bool { todo!() }

    /// (src/game/lesson.ts:142 FLAGS.ringsOut)
    pub fn rings_out(g: &mut crate::game::Game) -> bool { todo!() }

    /// (src/game/lesson.ts:143 FLAGS.shellsOut)
    pub fn shells_out(g: &mut crate::game::Game) -> bool { todo!() }

    /// (src/game/lesson.ts:144 FLAGS.auto)
    pub fn auto(g: &mut crate::game::Game) -> bool { todo!() }

    /// (src/game/lesson.ts:147 FLAGS.setup)
    pub fn setup(g: &mut crate::game::Game) -> bool { todo!() }

    /// (src/game/lesson.ts:148 FLAGS.setupShut)
    pub fn setup_shut(g: &mut crate::game::Game) -> bool { todo!() }

    /// (src/game/lesson.ts:149 FLAGS.dragging)
    pub fn dragging(g: &mut crate::game::Game) -> bool { todo!() }

    /// (src/game/lesson.ts:150 FLAGS.dark)
    pub fn dark(g: &mut crate::game::Game) -> bool { todo!() }

    /// (src/game/lesson.ts:151 FLAGS.chargeSet)
    pub fn charge_set(g: &mut crate::game::Game) -> bool { todo!() }

    /// (src/game/lesson.ts:153 FLAGS.chargeClear)
    pub fn charge_clear(g: &mut crate::game::Game) -> bool { todo!() }

    /// (src/game/lesson.ts:155 FLAGS.wallBlown)
    pub fn wall_blown(g: &mut crate::game::Game) -> bool { todo!() }

    /// (src/game/lesson.ts:156 FLAGS.holding)
    pub fn holding(g: &mut crate::game::Game) -> bool { todo!() }

    /// (src/game/lesson.ts:157 FLAGS.picking)
    pub fn picking(g: &mut crate::game::Game) -> bool { todo!() }

    /// (src/game/lesson.ts:158 FLAGS.coding)
    pub fn coding(g: &mut crate::game::Game) -> bool { todo!() }

    /// (src/game/lesson.ts:159 FLAGS.menu)
    pub fn menu(g: &mut crate::game::Game) -> bool { todo!() }

    /// (src/game/lesson.ts:160 FLAGS.ownRing)
    pub fn own_ring(g: &mut crate::game::Game) -> bool { todo!() }

    /// (src/game/lesson.ts:161 FLAGS.silent)
    pub fn silent(g: &mut crate::game::Game) -> bool { todo!() }

}
