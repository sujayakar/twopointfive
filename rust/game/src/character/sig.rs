#![allow(unused_variables)]
//! Every function and method of `character`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/character.ts:83 workKeyBeat)
pub fn work_key_beat(k: f64, dur: f64) -> f64 { todo!() }

/// (src/game/character.ts:338 holdPolish)
pub fn hold_polish(g: &mut crate::game::Game, up: bool) { todo!() }

/// (src/game/character.ts:347 holdOutRate)
pub fn hold_out_rate(g: &mut crate::game::Game, last: crate::character::HoldPose, speed: f64, busy: bool) -> f64 { todo!() }

/// (src/game/character.ts:438 nodLift)
pub fn nod_lift(n: f64) -> f64 { todo!() }

/// (src/game/character.ts:441 qYaw)
pub fn q_yaw(a: f64) -> Quat { todo!() }

/// (src/game/character.ts:459 Spring.constructor)
pub fn spring_new(x: Option<f64>) -> crate::character::Spring { todo!() }

/// (src/game/character.ts:460 Spring.step)
pub fn spring_step(this: &mut crate::character::Spring, target: f64, w: f64, dt: f64) -> f64 { todo!() }

/// (src/game/character.ts:465 curve)
pub fn curve(t: f64, k: &[[f64; 2]]) -> f64 { todo!() }

/// (src/game/character.ts:479 sampleAt0)
pub fn sample_at0(g: &mut crate::game::Game, clip: ClipId, out: &mut crate::anim_skeleton::Pose) { todo!() }

/// (src/game/character.ts:492 addPoseDelta)
pub fn add_pose_delta(p: &mut crate::anim_skeleton::Pose, from: &crate::anim_skeleton::Pose, to: &crate::anim_skeleton::Pose, w: f64, only: &[f32]) { todo!() }

/// (src/game/character.ts:510 bump)
pub fn bump(t: f64, a: f64, b: f64, c_2: f64, d: f64) -> f64 { todo!() }

/// (src/game/character.ts:648 cloneShape)
pub fn clone_shape(s: crate::character::LongShape) -> crate::character::LongShape { todo!() }

/// (src/game/character.ts:650 longFlat)
pub fn long_flat(s: crate::character::LongShape) -> Vec<f64> { todo!() }

/// (src/game/character.ts:652 longTargets)
pub fn long_targets(stance: crate::character::LongStance, crouch_w: f64) -> Vec<f64> { todo!() }

/// (src/game/character.ts:707 longTuck)
pub fn long_tuck(t: &mut Vec<f64>, gun_wall: f64) { todo!() }

/// (src/game/character.ts:722 animTables)
pub fn anim_tables() -> JsMap<String, crate::character::AnimTablesValue> { todo!() }

/// (src/game/character.ts:786 foldOn)
pub fn fold_on(g: &mut crate::game::Game, too: Option<bool>) -> bool { todo!() }

/// (src/game/character.ts:787 setMoves)
pub fn set_moves(g: &mut crate::game::Game, on: bool) { todo!() }

/// (src/game/character.ts:789 mvOn)
pub fn mv_on(g: &mut crate::game::Game, k: crate::character::MvOnK) -> bool { todo!() }

/// (src/game/character.ts:806 qAng)
pub fn q_ang(a: &[f64], i: f64, s: f64, b: &[f64], k: f64) -> f64 { todo!() }

/// (src/game/character.ts:809 inDipLane)
pub fn in_dip_lane(dx: f64, dz: f64, gun_yaw: f64) -> bool { todo!() }

/// (src/game/character.ts:811 reloadMarks)
pub fn reload_marks(back: bool) -> crate::character::MV2ReloadBox { todo!() }

/// (src/game/character.ts:890 setTurn)
pub fn set_turn(g: &mut crate::game::Game, spec: &str, dflt: Option<f64>) -> bool { todo!() }

/// (src/game/character.ts:900 turnFeel)
pub fn turn_feel(g: &mut crate::game::Game) -> String { todo!() }

/// (src/game/character.ts:1116 blowsNear)
pub fn blows_near(b: &crate::character::BlowsNearB) -> Vec<f64> { todo!() }

/// (src/game/character.ts:1123 layerTablesFit)
pub fn layer_tables_fit() { todo!() }

/// (src/game/character.ts:1132 throwTables)
pub fn throw_tables(low: bool) -> crate::character::ThrowTables { todo!() }

/// (src/game/character.ts:1400 bigRebound)
pub fn big_rebound(kind: crate::character::BigReboundKind) -> f64 { todo!() }

/// (src/game/character.ts:1402 bigOff)
pub fn big_off(b: crate::character::BigOffB) -> bool { todo!() }

/// (src/game/character.ts:1411 columnEntry)
pub fn column_entry(ro: V3, rd: V3, tmax: f64, cx: f64, cz: f64, r: f64, bot: f64, top: f64) -> f64 { todo!() }

/// (src/game/character.ts:1594 CharacterAnimator.woundShown)
pub fn anim_wound_shown(g: &mut crate::game::Game, c: CharId) -> f64 { todo!() }

/// (src/game/character.ts:1596 CharacterAnimator.woundClock)
pub fn anim_wound_clock(g: &mut crate::game::Game, c: CharId) -> f64 { todo!() }

/// (src/game/character.ts:1597 CharacterAnimator.woundDown)
pub fn anim_wound_down(g: &mut crate::game::Game, c: CharId) -> f64 { todo!() }

/// (src/game/character.ts:1601 CharacterAnimator.actShown)
pub fn anim_act_shown(g: &mut crate::game::Game, c: CharId) -> f64 { todo!() }

/// (src/game/character.ts:1621 CharacterAnimator.mvBusy)
pub fn anim_mv_busy(g: &mut crate::game::Game, c: CharId) -> bool { todo!() }

/// (src/game/character.ts:1623 CharacterAnimator.mvHands)
pub fn anim_mv_hands(g: &mut crate::game::Game, c: CharId) -> bool { todo!() }

/// (src/game/character.ts:1754 CharacterAnimator.underPose)
pub fn anim_under_pose(g: &mut crate::game::Game, c: CharId) -> Option<crate::anim_skeleton::Pose> { todo!() }

/// (src/game/character.ts:1831 CharacterAnimator.constructor)
pub fn anim_new(g: &mut crate::game::Game, ch: RigId) -> crate::character::Anim { todo!() }

/// (src/game/character.ts:1927 CharacterAnimator.desync)
pub fn anim_desync(g: &mut crate::game::Game, c: CharId, seed: f64) { todo!() }

/// (src/game/character.ts:1933 CharacterAnimator.fire)
pub fn anim_fire(g: &mut crate::game::Game, c: CharId) { todo!() }

/// (src/game/character.ts:1942 CharacterAnimator.hit)
pub fn anim_hit(g: &mut crate::game::Game, c: CharId) { todo!() }

/// (src/game/character.ts:1944 CharacterAnimator.flinch)
pub fn anim_flinch(g: &mut crate::game::Game, c: CharId) { todo!() }

/// (src/game/character.ts:1948 CharacterAnimator.flinchFrom)
pub fn anim_flinch_from(g: &mut crate::game::Game, c: CharId, toward: Option<V3>, k: Option<f64>, soft: Option<bool>) { todo!() }

/// (src/game/character.ts:1959 CharacterAnimator.flinchShown)
pub fn anim_flinch_shown(g: &mut crate::game::Game, c: CharId) -> f64 { todo!() }

/// (src/game/character.ts:1963 CharacterAnimator.startleFrom)
pub fn anim_startle_from(g: &mut crate::game::Game, c: CharId, toward: Option<V3>) { todo!() }

/// (src/game/character.ts:1969 CharacterAnimator.startleShown)
pub fn anim_startle_shown(g: &mut crate::game::Game, c: CharId) -> f64 { todo!() }

/// (src/game/character.ts:1972 CharacterAnimator.hitFrom)
pub fn anim_hit_from(g: &mut crate::game::Game, c: CharId, along: Option<V3>) { todo!() }

/// (src/game/character.ts:1984 CharacterAnimator.tossItem)
pub fn anim_toss_item(g: &mut crate::game::Game, c: CharId, release: Option<f64>) { todo!() }

/// (src/game/character.ts:1988 CharacterAnimator.tossing)
pub fn anim_tossing(g: &mut crate::game::Game, c: CharId) -> bool { todo!() }

/// (src/game/character.ts:1990 CharacterAnimator.flatModel)
pub fn anim_flat_model(g: &mut crate::game::Game, c: CharId, w: V3, fb: V3) -> V3 { todo!() }

/// (src/game/character.ts:1995 CharacterAnimator.strike)
pub fn anim_strike(g: &mut crate::game::Game, c: CharId, kind: Option<crate::character::StrikeKind>, lethal: Option<bool>) { todo!() }

/// (src/game/character.ts:1997 CharacterAnimator.strikeLethal)
pub fn anim_strike_lethal(g: &mut crate::game::Game, c: CharId) -> bool { todo!() }

/// (src/game/character.ts:1998 CharacterAnimator.striking)
pub fn anim_striking(g: &mut crate::game::Game, c: CharId) -> bool { todo!() }

/// (src/game/character.ts:2002 CharacterAnimator.blow)
pub fn anim_blow(g: &mut crate::game::Game, c: CharId, style: crate::character::BlowStyle, at: V3, at2: Option<V3>, o: Option<&crate::character::BlowOpts>) { todo!() }

/// (src/game/character.ts:2007 CharacterAnimator.blowCancel)
pub fn anim_blow_cancel(g: &mut crate::game::Game, c: CharId) { todo!() }

/// (src/game/character.ts:2009 CharacterAnimator.takenCancel)
pub fn anim_taken_cancel(g: &mut crate::game::Game, c: CharId) { todo!() }

/// (src/game/character.ts:2012 CharacterAnimator.taken)
pub fn anim_taken(g: &mut crate::game::Game, c: CharId, style: crate::character::TakenStyle, delay: Option<f64>) { todo!() }

/// (src/game/character.ts:2013 CharacterAnimator.blowing)
pub fn anim_blowing(g: &mut crate::game::Game, c: CharId) -> bool { todo!() }

/// (src/game/character.ts:2015 CharacterAnimator.reload)
pub fn anim_reload(g: &mut crate::game::Game, c: CharId, dur: Option<f64>, beat: Option<crate::character::ReloadBeat>) { todo!() }

/// (src/game/character.ts:2021 CharacterAnimator.rlBeat)
pub fn anim_rl_beat(g: &mut crate::game::Game, c: CharId, b: crate::character::ReloadBeat) { todo!() }

/// (src/game/character.ts:2032 CharacterAnimator.kitSwap)
pub fn anim_kit_swap(g: &mut crate::game::Game, c: CharId, from: Option<crate::character::KitItem>, to: Option<crate::character::KitItem>, shown: Option<crate::character::AnimKitSwapShown>, same: Option<bool>) -> f64 { todo!() }

/// (src/game/character.ts:2051 CharacterAnimator.kitCancel)
pub fn anim_kit_cancel(g: &mut crate::game::Game, c: CharId) { todo!() }

/// (src/game/character.ts:2052 CharacterAnimator.kitBusy)
pub fn anim_kit_busy(g: &mut crate::game::Game, c: CharId) -> bool { todo!() }

/// (src/game/character.ts:2053 CharacterAnimator.nvBusy)
pub fn anim_nv_busy(g: &mut crate::game::Game, c: CharId) -> bool { todo!() }

/// (src/game/character.ts:2064 CharacterAnimator.nvFlip)
pub fn anim_nv_flip(g: &mut crate::game::Game, c: CharId, down: bool, hand: bool) { todo!() }

/// (src/game/character.ts:2074 CharacterAnimator.nvCancel)
pub fn anim_nv_cancel(g: &mut crate::game::Game, c: CharId) { todo!() }

/// (src/game/character.ts:2076 CharacterAnimator.kitRifle)
pub fn anim_kit_rifle(g: &mut crate::game::Game, c: CharId) -> bool { todo!() }

/// (src/game/character.ts:2078 CharacterAnimator.kitHandsOn)
pub fn anim_kit_hands_on(g: &mut crate::game::Game, c: CharId) -> bool { todo!() }

/// (src/game/character.ts:2081 CharacterAnimator.reloadShow)
pub fn anim_reload_show(g: &mut crate::game::Game, c: CharId) -> Option<crate::character::AnimReloadShow> { todo!() }

/// (src/game/character.ts:2088 CharacterAnimator.feedClock)
pub fn anim_feed_clock(g: &mut crate::game::Game, c: CharId) -> Option<crate::character::AnimFeedClock> { todo!() }

/// (src/game/character.ts:2093 CharacterAnimator.cancelReload)
pub fn anim_cancel_reload(g: &mut crate::game::Game, c: CharId) { todo!() }

/// (src/game/character.ts:2102 CharacterAnimator.throwItem)
pub fn anim_throw_item(g: &mut crate::game::Game, c: CharId, dist: Option<f64>, speed: Option<f64>) { todo!() }

/// (src/game/character.ts:2109 CharacterAnimator.reloading)
pub fn anim_reloading(g: &mut crate::game::Game, c: CharId) -> bool { todo!() }

/// (src/game/character.ts:2111 CharacterAnimator.throwRead)
pub fn anim_throw_read(g: &mut crate::game::Game, c: CharId) -> Option<crate::character::AnimThrowRead> { todo!() }

/// (src/game/character.ts:2113 CharacterAnimator.throwing)
pub fn anim_throwing(g: &mut crate::game::Game, c: CharId) -> bool { todo!() }

/// (src/game/character.ts:2115 CharacterAnimator.inThrow)
pub fn anim_in_throw(g: &mut crate::game::Game, c: CharId) -> bool { todo!() }

/// (src/game/character.ts:2118 CharacterAnimator.throwOff)
pub fn anim_throw_off(g: &mut crate::game::Game, c: CharId) -> bool { todo!() }

/// (src/game/character.ts:2120 CharacterAnimator.aimShown)
pub fn anim_aim_shown(g: &mut crate::game::Game, c: CharId) -> f64 { todo!() }

/// (src/game/character.ts:2123 CharacterAnimator.torchUp)
pub fn anim_torch_up(g: &mut crate::game::Game, c: CharId) -> f64 { todo!() }

/// (src/game/character.ts:2126 CharacterAnimator.atWell)
pub fn anim_at_well(g: &mut crate::game::Game, c: CharId) -> bool { todo!() }

/// (src/game/character.ts:2129 CharacterAnimator.gunArmOwn)
pub fn anim_gun_arm_own(g: &mut crate::game::Game, c: CharId) -> bool { todo!() }

/// (src/game/character.ts:2134 CharacterAnimator.throwTime)
pub fn anim_throw_time(g: &mut crate::game::Game, c: CharId) -> f64 { todo!() }

/// (src/game/character.ts:2135 CharacterAnimator.shooting)
pub fn anim_shooting(g: &mut crate::game::Game, c: CharId) -> bool { todo!() }

/// (src/game/character.ts:2142 CharacterAnimator.kickDoor)
pub fn anim_kick_door(g: &mut crate::game::Game, c: CharId, opts: Option<crate::character::AnimKickDoorOpts>) { todo!() }

/// (src/game/character.ts:2143 CharacterAnimator.kicking)
pub fn anim_kicking(g: &mut crate::game::Game, c: CharId) -> bool { todo!() }

/// (src/game/character.ts:2145 CharacterAnimator.kickingLeg)
pub fn anim_kicking_leg(g: &mut crate::game::Game, c: CharId) -> crate::character::AnimKickingLeg { todo!() }

/// (src/game/character.ts:2147 CharacterAnimator.kickTime)
pub fn anim_kick_time(g: &mut crate::game::Game, c: CharId) -> f64 { todo!() }

/// (src/game/character.ts:2149 CharacterAnimator.kickImpact)
pub fn anim_kick_impact(g: &mut crate::game::Game, c: CharId) -> bool { todo!() }

/// (src/game/character.ts:2154 CharacterAnimator.signal)
pub fn anim_signal(g: &mut crate::game::Game, c: CharId, kind: crate::character::SignalKind, side: Option<f64>) { todo!() }

/// (src/game/character.ts:2155 CharacterAnimator.signalling)
pub fn anim_signalling(g: &mut crate::game::Game, c: CharId) -> bool { todo!() }

/// (src/game/character.ts:2159 CharacterAnimator.signalFree)
pub fn anim_signal_free(g: &mut crate::game::Game, c: CharId) -> bool { todo!() }

/// (src/game/character.ts:2167 CharacterAnimator.tell)
pub fn anim_tell(g: &mut crate::game::Game, c: CharId, kind: crate::character::TellKind) { todo!() }

/// (src/game/character.ts:2169 CharacterAnimator.cancelTell)
pub fn anim_cancel_tell(g: &mut crate::game::Game, c: CharId) { todo!() }

/// (src/game/character.ts:2170 CharacterAnimator.telling)
pub fn anim_telling(g: &mut crate::game::Game, c: CharId) -> bool { todo!() }

/// (src/game/character.ts:2172 CharacterAnimator.tellKind)
pub fn anim_tell_kind(g: &mut crate::game::Game, c: CharId) -> crate::character::TellKind { todo!() }

/// (src/game/character.ts:2175 CharacterAnimator.update)
pub fn anim_update(g: &mut crate::game::Game, c: CharId, dt: f64) { todo!() }

/// (src/game/character.ts:2176 CharacterAnimator.updateStanding)
pub fn anim_update_standing(g: &mut crate::game::Game, c: CharId, dt: f64) { todo!() }

/// (src/game/character.ts:2626 CharacterAnimator.actLayer)
pub fn anim_act_layer(g: &mut crate::game::Game, c: CharId, dt: f64) -> f64 { todo!() }

/// (src/game/character.ts:2640 CharacterAnimator.actPose)
pub fn anim_act_pose(g: &mut crate::game::Game, c: CharId, clip: ClipId, w: f64, dt: f64) { todo!() }

/// (src/game/character.ts:2646 CharacterAnimator.actPoseAsk)
pub fn anim_act_pose_ask(g: &mut crate::game::Game, c: CharId, g_2: f64, clip: ClipId, w: f64, dt: f64) { todo!() }

/// (src/game/character.ts:2653 CharacterAnimator.woundLayer)
pub fn anim_wound_layer(g: &mut crate::game::Game, c: CharId, dt: f64) -> f64 { todo!() }

/// (src/game/character.ts:2667 CharacterAnimator.woundBody)
pub fn anim_wound_body(g: &mut crate::game::Game, c: CharId, clip: ClipId, tt: f64, w: f64, down: f64, head: f64) { todo!() }

/// (src/game/character.ts:2672 CharacterAnimator.woundBodyAsk)
pub fn anim_wound_body_ask(g: &mut crate::game::Game, c: CharId, g_2: f64, clip: ClipId, tt: f64, w: f64, down: f64, head: f64) { todo!() }

/// (src/game/character.ts:2678 CharacterAnimator.woundLimbs)
pub fn anim_wound_limbs(g: &mut crate::game::Game, c: CharId, w: f64, wd: crate::character::AnimWoundLimbsWd) { todo!() }

/// (src/game/character.ts:2683 CharacterAnimator.woundLimbsAsk)
pub fn anim_wound_limbs_ask(g: &mut crate::game::Game, c: CharId, g_2: f64, w: f64, wd: crate::character::AnimWoundLimbsAskWd) { todo!() }

/// (src/game/character.ts:2689 CharacterAnimator.actTime)
pub fn anim_act_time(g: &mut crate::game::Game, c: CharId) -> f64 { todo!() }

/// (src/game/character.ts:2693 CharacterAnimator.mvLive)
pub fn anim_mv_live(g: &mut crate::game::Game, c: CharId) -> bool { todo!() }

/// (src/game/character.ts:2694 CharacterAnimator.mvReset)
pub fn anim_mv_reset(g: &mut crate::game::Game, c: CharId) { todo!() }

/// (src/game/character.ts:2699 CharacterAnimator.mvPre)
pub fn anim_mv_pre(g: &mut crate::game::Game, c: CharId, dt: f64, move_w: f64, stand_w: f64) -> crate::character::MvGait { todo!() }

/// (src/game/character.ts:2707 CharacterAnimator.leanBody)
pub fn anim_lean_body(g: &mut crate::game::Game, c: CharId, dt: f64) { todo!() }

/// (src/game/character.ts:2712 CharacterAnimator.leanBodyAsk)
pub fn anim_lean_body_ask(g: &mut crate::game::Game, c: CharId, g_2: f64, dt: f64) { todo!() }

/// (src/game/character.ts:2723 CharacterAnimator.gazeKeep)
pub fn anim_gaze_keep(g: &mut crate::game::Game, c: CharId, dt: f64, want_w: f64, crouch_pose: f64, w_sprint: f64) -> f64 { todo!() }

/// (src/game/character.ts:2729 CharacterAnimator.gazeKeepAsk)
pub fn anim_gaze_keep_ask(g: &mut crate::game::Game, c: CharId, g_2: f64, dt: f64, want_w: f64, crouch_pose: f64, w_sprint: f64) -> f64 { todo!() }

/// (src/game/character.ts:2737 CharacterAnimator.ruleSlot)
pub fn anim_rule_slot(g: &mut crate::game::Game, c: CharId) -> f64 { todo!() }

/// (src/game/character.ts:2738 CharacterAnimator.ruleRan)
pub fn anim_rule_ran(g: &mut crate::game::Game, c: CharId, code: f64) { todo!() }

/// (src/game/character.ts:2739 CharacterAnimator.ruleWatch)
pub fn anim_rule_watch(g: &mut crate::game::Game, c: CharId, name: &str, cols: crate::anim_skeleton::ColsOf) { todo!() }

/// (src/game/character.ts:2740 CharacterAnimator.mvPreAsk)
pub fn anim_mv_pre_ask(g: &mut crate::game::Game, c: CharId, g_2: f64, dt: f64, move_w: f64, stand_w: f64) { todo!() }

/// (src/game/character.ts:2763 CharacterAnimator.mvTrunk)
pub fn anim_mv_trunk(g: &mut crate::game::Game, c: CharId, w: f64, o: crate::character::MvGait, aim_pass: bool) { todo!() }

/// (src/game/character.ts:2769 CharacterAnimator.mvTrunkAsk)
pub fn anim_mv_trunk_ask(g: &mut crate::game::Game, c: CharId, g_2: f64, w: f64, o: crate::character::MvGait, aim_pass: bool) { todo!() }

/// (src/game/character.ts:2778 CharacterAnimator.mvArms)
pub fn anim_mv_arms(g: &mut crate::game::Game, c: CharId, dt: f64, gate: f64) { todo!() }

/// (src/game/character.ts:2802 CharacterAnimator.trunkFrame)
pub fn anim_trunk_frame(g: &mut crate::game::Game, c: CharId) -> crate::character::AnimTrunkFrame { todo!() }

/// (src/game/character.ts:2807 CharacterAnimator.reloadArm)
pub fn anim_reload_arm(g: &mut crate::game::Game, c: CharId, t: f64, w: f64) { todo!() }

/// (src/game/character.ts:2815 CharacterAnimator.beatToBench)
pub fn anim_beat_to_bench(g: &mut crate::game::Game, c: CharId, a: &mut Vec<f64>, o: f64) { todo!() }

/// (src/game/character.ts:2820 CharacterAnimator.reloadArmAsk)
pub fn anim_reload_arm_ask(g: &mut crate::game::Game, c: CharId, g_2: f64, t: f64, w: f64, own: Option<&crate::character::AnimReloadArmAskOwn>) { todo!() }

/// (src/game/character.ts:2833 CharacterAnimator.pistolReloadArm)
pub fn anim_pistol_reload_arm(g: &mut crate::game::Game, c: CharId, t: f64, w: f64) { todo!() }

/// (src/game/character.ts:2838 CharacterAnimator.pistolReloadArmAsk)
pub fn anim_pistol_reload_arm_ask(g: &mut crate::game::Game, c: CharId, g_2: f64, t: f64, w: f64) { todo!() }

/// (src/game/character.ts:2842 CharacterAnimator.clutchArm)
pub fn anim_clutch_arm(g: &mut crate::game::Game, c: CharId, w: f64) { todo!() }

/// (src/game/character.ts:2847 CharacterAnimator.clutchArmAsk)
pub fn anim_clutch_arm_ask(g: &mut crate::game::Game, c: CharId, g_2: f64, w: f64) { todo!() }

/// (src/game/character.ts:2851 CharacterAnimator.boundArms)
pub fn anim_bound_arms(g: &mut crate::game::Game, c: CharId, w: f64, kind: f64) { todo!() }

/// (src/game/character.ts:2856 CharacterAnimator.boundArmsAsk)
pub fn anim_bound_arms_ask(g: &mut crate::game::Game, c: CharId, g_2: f64, w: f64, kind: f64) { todo!() }

/// (src/game/character.ts:2862 CharacterAnimator.mvPost)
pub fn anim_mv_post(g: &mut crate::game::Game, c: CharId, dt: f64, gate: f64) { todo!() }

/// (src/game/character.ts:2887 CharacterAnimator.mvPostBody)
pub fn anim_mv_post_body(g: &mut crate::game::Game, c: CharId, dt: f64, gate: f64) { todo!() }

/// (src/game/character.ts:2892 CharacterAnimator.mvPostBodyAsk)
pub fn anim_mv_post_body_ask(g: &mut crate::game::Game, c: CharId, g_2: f64, dt: f64, gate: f64) { todo!() }

/// (src/game/character.ts:2901 CharacterAnimator.mvPostArms)
pub fn anim_mv_post_arms(g: &mut crate::game::Game, c: CharId, dt: f64, gate: f64) { todo!() }

/// (src/game/character.ts:2913 CharacterAnimator.withHands)
pub fn anim_with_hands(g: &mut crate::game::Game, c: CharId, c_2: crate::anim_skeleton::LayerCols) -> crate::anim_skeleton::ColsOf { todo!() }

/// (src/game/character.ts:2914 CharacterAnimator.mvPostArmsAsk)
pub fn anim_mv_post_arms_ask(g: &mut crate::game::Game, c: CharId, g_2: f64, dt: f64, gate: f64) { todo!() }

/// (src/game/character.ts:2930 CharacterAnimator.fkPart)
pub fn anim_fk_part(g: &mut crate::game::Game, c: CharId, pose: &crate::anim_skeleton::Pose, list: &[i32]) { todo!() }

/// (src/game/character.ts:2945 CharacterAnimator.fkList)
pub fn anim_fk_list(g: &mut crate::game::Game, c: CharId, ends: &[f64]) -> Vec<i32> { todo!() }

/// (src/game/character.ts:2956 CharacterAnimator.hand)
pub fn anim_hand(g: &mut crate::game::Game, c: CharId, pose: &mut crate::anim_skeleton::Pose, side: f64, curl: &[f64], thumb: f64, spread: Option<f64>) { todo!() }

/// (src/game/character.ts:2975 CharacterAnimator.toesUp)
pub fn anim_toes_up(g: &mut crate::game::Game, c: CharId, w: f64) { todo!() }

/// (src/game/character.ts:2980 CharacterAnimator.toesUpAsk)
pub fn anim_toes_up_ask(g: &mut crate::game::Game, c: CharId, g_2: f64, w: f64) { todo!() }

/// (src/game/character.ts:2982 CharacterAnimator.handsBuild)
pub fn anim_hands_build(g: &mut crate::game::Game, c: CharId) { todo!() }

/// (src/game/character.ts:3006 CharacterAnimator.handsPass)
pub fn anim_hands_pass(g: &mut crate::game::Game, c: CharId, dt: f64, full_l: f64, full_r: f64, gun_r: f64) { todo!() }

/// (src/game/character.ts:3019 CharacterAnimator.handTables)
pub fn anim_hand_tables(g: &mut crate::game::Game, c: CharId) -> Vec<f64> { todo!() }

/// (src/game/character.ts:3025 CharacterAnimator.handsPassAsk)
pub fn anim_hands_pass_ask(g: &mut crate::game::Game, c: CharId, g_2: f64, dt: f64, full_l: f64, full_r: f64, gun_r: f64) { todo!() }

/// (src/game/character.ts:3035 CharacterAnimator.stanceBody)
pub fn anim_stance_body(g: &mut crate::game::Game, c: CharId, w: f64, speed: f64, stand: f64) { todo!() }

/// (src/game/character.ts:3040 CharacterAnimator.stanceBodyAsk)
pub fn anim_stance_body_ask(g: &mut crate::game::Game, c: CharId, g_2: f64, w: f64, speed: f64, stand: f64) { todo!() }

/// (src/game/character.ts:3066 CharacterAnimator.standPose)
pub fn anim_stand_pose(g: &mut crate::game::Game, c: CharId, ph: f64, w_idle: f64, w_walk: f64, w_jog: f64, w_sprint: f64) { todo!() }

/// (src/game/character.ts:3071 CharacterAnimator.standPoseAsk)
pub fn anim_stand_pose_ask(g: &mut crate::game::Game, c: CharId, g_2: f64, ph: f64, w_idle: f64, w_walk: f64, w_jog: f64, w_sprint: f64) { todo!() }

/// (src/game/character.ts:3081 CharacterAnimator.crouchLaid)
pub fn anim_crouch_laid(g: &mut crate::game::Game, c: CharId, ph: f64, w_move: f64, crouch_pose: f64) { todo!() }

/// (src/game/character.ts:3086 CharacterAnimator.crouchLaidAsk)
pub fn anim_crouch_laid_ask(g: &mut crate::game::Game, c: CharId, g_2: f64, ph: f64, w_move: f64, crouch_pose: f64) { todo!() }

/// (src/game/character.ts:3094 CharacterAnimator.runArms)
pub fn anim_run_arms(g: &mut crate::game::Game, c: CharId, pose: &crate::anim_skeleton::Pose, w: f64, ws: f64) { todo!() }

/// (src/game/character.ts:3099 CharacterAnimator.runArmsAsk)
pub fn anim_run_arms_ask(g: &mut crate::game::Game, c: CharId, g_2: f64, pose: &crate::anim_skeleton::Pose, w: f64, ws: f64) { todo!() }

/// (src/game/character.ts:3106 CharacterAnimator.stanceOf)
pub fn anim_stance_of(g: &mut crate::game::Game, c: CharId, ty: &[Vec<f64>], tz: &[Vec<f64>]) -> Option<crate::character::AnimStanceOf> { todo!() }

/// (src/game/character.ts:3116 CharacterAnimator.sampleJog)
pub fn anim_sample_jog(g: &mut crate::game::Game, c: CharId, ph: f64, out: &mut crate::anim_skeleton::Pose) { todo!() }

/// (src/game/character.ts:3125 CharacterAnimator.softKnees)
pub fn anim_soft_knees(g: &mut crate::game::Game, c: CharId, pose: &crate::anim_skeleton::Pose, w: f64) { todo!() }

/// (src/game/character.ts:3130 CharacterAnimator.softKneesAsk)
pub fn anim_soft_knees_ask(g: &mut crate::game::Game, c: CharId, g_2: f64, pose: &crate::anim_skeleton::Pose, w: f64) { todo!() }

/// (src/game/character.ts:3142 CharacterAnimator.runPlant)
pub fn anim_run_plant(g: &mut crate::game::Game, c: CharId, ph: f64, s: f64, dt: f64, w_run: f64, w_sprint: f64) { todo!() }

/// (src/game/character.ts:3147 CharacterAnimator.runPlantAsk)
pub fn anim_run_plant_ask(g: &mut crate::game::Game, c: CharId, g_2: f64, ph: f64, s: f64, dt: f64, w_run: f64, w_sprint: f64) { todo!() }

/// (src/game/character.ts:3166 CharacterAnimator.lifeLayer)
pub fn anim_life_layer(g: &mut crate::game::Game, c: CharId, dt: f64, s: f64, crouch_pose: f64, w: f64, w_f: f64) { todo!() }

/// (src/game/character.ts:3171 CharacterAnimator.lifeLayerAsk)
pub fn anim_life_layer_ask(g: &mut crate::game::Game, c: CharId, g_2: f64, dt: f64, s: f64, crouch_pose: f64, w: f64, w_f: f64) { todo!() }

/// (src/game/character.ts:3195 CharacterAnimator.sneakLayer)
pub fn anim_sneak_layer(g: &mut crate::game::Game, c: CharId, dl: f64, pace: f64, soft: f64, arms_in: f64, surge: f64, dsh: Option<f64>) { todo!() }

/// (src/game/character.ts:3200 CharacterAnimator.sneakBody)
pub fn anim_sneak_body(g: &mut crate::game::Game, c: CharId, dl: f64, pace: f64, soft: f64, surge: f64, dsh: f64) { todo!() }

/// (src/game/character.ts:3205 CharacterAnimator.sneakBodyAsk)
pub fn anim_sneak_body_ask(g: &mut crate::game::Game, c: CharId, g_2: f64, dl: f64, pace: f64, soft: f64, surge: f64, dsh: f64) { todo!() }

/// (src/game/character.ts:3211 CharacterAnimator.sneakArms)
pub fn anim_sneak_arms(g: &mut crate::game::Game, c: CharId, arms_in: f64) { todo!() }

/// (src/game/character.ts:3216 CharacterAnimator.sneakArmsAsk)
pub fn anim_sneak_arms_ask(g: &mut crate::game::Game, c: CharId, g_2: f64, arms_in: f64) { todo!() }

/// (src/game/character.ts:3229 CharacterAnimator.steadyLayer)
pub fn anim_steady_layer(g: &mut crate::game::Game, c: CharId, w: f64, base: Option<f64>, sunk: Option<f64>) { todo!() }

/// (src/game/character.ts:3240 CharacterAnimator.steadyLayerAsk)
pub fn anim_steady_layer_ask(g: &mut crate::game::Game, c: CharId, g_2: f64, w: f64, base: f64, sunk: f64) { todo!() }

/// (src/game/character.ts:3250 CharacterAnimator.carryArms)
pub fn anim_carry_arms(g: &mut crate::game::Game, c: CharId, w: f64, kick_t: f64) { todo!() }

/// (src/game/character.ts:3255 CharacterAnimator.carryArmsAsk)
pub fn anim_carry_arms_ask(g: &mut crate::game::Game, c: CharId, g_2: f64, w: f64, kick_t: f64) { todo!() }

/// (src/game/character.ts:3261 CharacterAnimator.rlRow)
pub fn anim_rl_row(g: &mut crate::game::Game, c: CharId) -> crate::character::LONGRELOADBox { todo!() }

/// (src/game/character.ts:3263 CharacterAnimator.lgChannels)
pub fn anim_lg_channels(g: &mut crate::game::Game, c: CharId) -> Vec<crate::character::AnimLgChannelsItem> { todo!() }

/// (src/game/character.ts:3268 CharacterAnimator.reloadCarry)
pub fn anim_reload_carry(g: &mut crate::game::Game, c: CharId, dt: f64, want: crate::character::LongStance) { todo!() }

/// (src/game/character.ts:3301 CharacterAnimator.longBody)
pub fn anim_long_body(g: &mut crate::game::Game, c: CharId, w: f64, run: Option<f64>, sprint: Option<f64>) { todo!() }

/// (src/game/character.ts:3307 CharacterAnimator.longBodyAsk)
pub fn anim_long_body_ask(g: &mut crate::game::Game, c: CharId, g_2: f64, w: f64, run: f64, sprint: f64) { todo!() }

/// (src/game/character.ts:3322 CharacterAnimator.longArms)
pub fn anim_long_arms(g: &mut crate::game::Game, c: CharId, w: f64, kick_t: f64, dt: f64, yield_door: bool, keep: bool) { todo!() }

/// (src/game/character.ts:3332 CharacterAnimator.longArmsAsk)
pub fn anim_long_arms_ask(g: &mut crate::game::Game, c: CharId, g_2: f64, w: f64, kick_t: f64, dt: f64, yield_door: bool, keep: bool) { todo!() }

/// (src/game/character.ts:3358 CharacterAnimator.wallBody)
pub fn anim_wall_body(g: &mut crate::game::Game, c: CharId, w: f64) { todo!() }

/// (src/game/character.ts:3363 CharacterAnimator.wallBodyAsk)
pub fn anim_wall_body_ask(g: &mut crate::game::Game, c: CharId, g_2: f64, w: f64) { todo!() }

/// (src/game/character.ts:3390 CharacterAnimator.wallArms)
pub fn anim_wall_arms(g: &mut crate::game::Game, c: CharId, w: f64) { todo!() }

/// (src/game/character.ts:3395 CharacterAnimator.wallArmsAsk)
pub fn anim_wall_arms_ask(g: &mut crate::game::Game, c: CharId, g_2: f64, w: f64) { todo!() }

/// (src/game/character.ts:3407 CharacterAnimator.handsAsk)
pub fn anim_hands_ask(g: &mut crate::game::Game, c: CharId, layer: &mut dyn FnMut(&mut crate::game::Game) -> f64) { todo!() }

/// (src/game/character.ts:3417 CharacterAnimator.kickBody)
pub fn anim_kick_body(g: &mut crate::game::Game, c: CharId, t: f64, w: f64) { todo!() }

/// (src/game/character.ts:3422 CharacterAnimator.kickBodyAsk)
pub fn anim_kick_body_ask(g: &mut crate::game::Game, c: CharId, g_2: f64, t: f64, w: f64) { todo!() }

/// (src/game/character.ts:3440 CharacterAnimator.signalLayer)
pub fn anim_signal_layer(g: &mut crate::game::Game, c: CharId, t: f64) { todo!() }

/// (src/game/character.ts:3445 CharacterAnimator.signalLayerAsk)
pub fn anim_signal_layer_ask(g: &mut crate::game::Game, c: CharId, g_2: f64, t: f64) { todo!() }

/// (src/game/character.ts:3456 CharacterAnimator.smokeLayer)
pub fn anim_smoke_layer(g: &mut crate::game::Game, c: CharId, dt: f64, free: bool) { todo!() }

/// (src/game/character.ts:3464 CharacterAnimator.smokePose)
pub fn anim_smoke_pose(g: &mut crate::game::Game, c: CharId) { todo!() }

/// (src/game/character.ts:3470 CharacterAnimator.headHandToBench)
pub fn anim_head_hand_to_bench(g: &mut crate::game::Game, c: CharId, o: f64) { todo!() }

/// (src/game/character.ts:3474 CharacterAnimator.smokePoseAsk)
pub fn anim_smoke_pose_ask(g: &mut crate::game::Game, c: CharId, g_2: f64) { todo!() }

/// (src/game/character.ts:3478 CharacterAnimator.nvLayer)
pub fn anim_nv_layer(g: &mut crate::game::Game, c: CharId, dt: f64, free: bool) { todo!() }

/// (src/game/character.ts:3493 CharacterAnimator.nvPose)
pub fn anim_nv_pose(g: &mut crate::game::Game, c: CharId, k: f64) { todo!() }

/// (src/game/character.ts:3498 CharacterAnimator.nvPoseAsk)
pub fn anim_nv_pose_ask(g: &mut crate::game::Game, c: CharId, g_2: f64, k: f64) { todo!() }

/// (src/game/character.ts:3504 CharacterAnimator.nvTip)
pub fn anim_nv_tip(g: &mut crate::game::Game, c: CharId, pose: &mut crate::anim_skeleton::Pose, nod: f64, g_2: Option<Quat>) { todo!() }

/// (src/game/character.ts:3510 CharacterAnimator.nvHead)
pub fn anim_nv_head(g: &mut crate::game::Game, c: CharId, pose: &crate::anim_skeleton::Pose) -> Quat { todo!() }

/// (src/game/character.ts:3521 CharacterAnimator.tellLayer)
pub fn anim_tell_layer(g: &mut crate::game::Game, c: CharId, t: f64) { todo!() }

/// (src/game/character.ts:3528 CharacterAnimator.tellLayerAsk)
pub fn anim_tell_layer_ask(g: &mut crate::game::Game, c: CharId, g_2: f64, t: f64) { todo!() }

/// (src/game/character.ts:3535 CharacterAnimator.heldGaitSnap)
pub fn anim_held_gait_snap(g: &mut crate::game::Game, c: CharId) -> Option<Vec<f64>> { todo!() }

/// (src/game/character.ts:3536 CharacterAnimator.heldGaitBack)
pub fn anim_held_gait_back(g: &mut crate::game::Game, c: CharId, a: Option<&[f64]>) { todo!() }

/// (src/game/character.ts:3543 CharacterAnimator.creepGait)
pub fn anim_creep_gait(g: &mut crate::game::Game, c: CharId, dt: f64) -> f64 { todo!() }

/// (src/game/character.ts:3553 CharacterAnimator.creepPlant)
pub fn anim_creep_plant(g: &mut crate::game::Game, c: CharId) -> [f64; 2] { todo!() }

/// (src/game/character.ts:3567 CharacterAnimator.watchLayer)
pub fn anim_watch_layer(g: &mut crate::game::Game, c: CharId, dt: f64, free: f64, w_l: f64, w_r: f64, tw_y: f64, tw_p: f64) { todo!() }

/// (src/game/character.ts:3582 CharacterAnimator.watchSlip)
pub fn anim_watch_slip(g: &mut crate::game::Game, c: CharId, dt: f64, free: f64, m: Option<V3>) { todo!() }

/// (src/game/character.ts:3587 CharacterAnimator.watchSlipAsk)
pub fn anim_watch_slip_ask(g: &mut crate::game::Game, c: CharId, g_2: f64, dt: f64, free: f64, m: Option<V3>) { todo!() }

/// (src/game/character.ts:3593 CharacterAnimator.watchArm)
pub fn anim_watch_arm(g: &mut crate::game::Game, c: CharId, w_l: f64, w_r: f64) { todo!() }

/// (src/game/character.ts:3599 CharacterAnimator.watchArmAsk)
pub fn anim_watch_arm_ask(g: &mut crate::game::Game, c: CharId, g_2: f64, w_l: f64, w_r: f64) { todo!() }

/// (src/game/character.ts:3606 CharacterAnimator.watchLook)
pub fn anim_watch_look(g: &mut crate::game::Game, c: CharId, dt: f64, free: f64, m: Option<V3>) { todo!() }

/// (src/game/character.ts:3611 CharacterAnimator.watchLookAsk)
pub fn anim_watch_look_ask(g: &mut crate::game::Game, c: CharId, g_2: f64, dt: f64, free: f64, m: Option<V3>) { todo!() }

/// (src/game/character.ts:3616 CharacterAnimator.creepLayer)
pub fn anim_creep_layer(g: &mut crate::game::Game, c: CharId, w: f64, w_l: f64, w_r: f64, tw_y: f64, tw_p: f64, dt: f64) { todo!() }

/// (src/game/character.ts:3629 CharacterAnimator.creepArms)
pub fn anim_creep_arms(g: &mut crate::game::Game, c: CharId, w: f64, w_l: f64, w_r: f64, e: f64, cr: f64, ck: f64, cke: f64, ws: f64, ha: f64, hg: f64) { todo!() }

/// (src/game/character.ts:3635 CharacterAnimator.creepArmsAsk)
pub fn anim_creep_arms_ask(g: &mut crate::game::Game, c: CharId, g_2: f64, w: f64, w_l: f64, w_r: f64, e: f64, cr: f64, ck: f64, cke: f64, ws: f64, ha: f64, hg: f64) { todo!() }

/// (src/game/character.ts:3644 CharacterAnimator.creepBody)
pub fn anim_creep_body(g: &mut crate::game::Game, c: CharId, e: f64, free: f64, cr: f64, gw: f64, tw_y: f64, tw_p: f64, dt: f64) -> Vec<f64> { todo!() }

/// (src/game/character.ts:3650 CharacterAnimator.creepBodyAsk)
pub fn anim_creep_body_ask(g: &mut crate::game::Game, c: CharId, g_2: f64, e: f64, free: f64, cr: f64, gw: f64, tw_y: f64, tw_p: f64, dt: f64) -> Vec<f64> { todo!() }

/// (src/game/character.ts:3669 CharacterAnimator.stealthLayer)
pub fn anim_stealth_layer(g: &mut crate::game::Game, c: CharId, dt: f64, busy: bool, crouch_pose: f64, w_move: f64, tw_y: f64, tw_p: f64) { todo!() }

/// (src/game/character.ts:3679 CharacterAnimator.stealthLayerAsk)
pub fn anim_stealth_layer_ask(g: &mut crate::game::Game, c: CharId, g_2: f64, dt: f64, busy: bool, crouch_pose: f64, w_move: f64, tw_y: f64, tw_p: f64) { todo!() }

/// (src/game/character.ts:3690 CharacterAnimator.stealthPicture)
pub fn anim_stealth_picture(g: &mut crate::game::Game, c: CharId, dt: f64) { todo!() }

/// (src/game/character.ts:3701 CharacterAnimator.cablePose)
pub fn anim_cable_pose(g: &mut crate::game::Game, c: CharId, pic: &crate::anim_skeleton::Pose, e: f64, eh: f64) { todo!() }

/// (src/game/character.ts:3710 CharacterAnimator.cablePoseAsk)
pub fn anim_cable_pose_ask(g: &mut crate::game::Game, c: CharId, g_2: f64, pic: &crate::anim_skeleton::Pose, e: f64, eh: f64) { todo!() }

/// (src/game/character.ts:3719 CharacterAnimator.untwist)
pub fn anim_untwist(g: &mut crate::game::Game, c: CharId, tw_y: f64, tw_p: f64) -> crate::character::Untwist { todo!() }

/// (src/game/character.ts:3733 CharacterAnimator.kitLayer)
pub fn anim_kit_layer(g: &mut crate::game::Game, c: CharId, dt: f64, tw_y: f64, tw_p: f64) { todo!() }

/// (src/game/character.ts:3743 CharacterAnimator.kitLayerAsk)
pub fn anim_kit_layer_ask(g: &mut crate::game::Game, c: CharId, g_2: f64, tw_y: f64, tw_p: f64) { todo!() }

/// (src/game/character.ts:3769 CharacterAnimator.catchArm)
pub fn anim_catch_arm(g: &mut crate::game::Game, c: CharId, x: crate::character::AnimHdFix) { todo!() }

/// (src/game/character.ts:3774 CharacterAnimator.catchArmAsk)
pub fn anim_catch_arm_ask(g: &mut crate::game::Game, c: CharId, g_2: f64, x: crate::character::AnimHdFix) { todo!() }

/// (src/game/character.ts:3779 CharacterAnimator.catchFix)
pub fn anim_catch_fix(g: &mut crate::game::Game, c: CharId) { todo!() }

/// (src/game/character.ts:3786 CharacterAnimator.readyLayer)
pub fn anim_ready_layer(g: &mut crate::game::Game, c: CharId, w: f64, kind: crate::character::ReadyKind, live: bool, dt: f64, tw_y: f64, tw_p: f64) { todo!() }

/// (src/game/character.ts:3796 CharacterAnimator.readyLayerAsk)
pub fn anim_ready_layer_ask(g: &mut crate::game::Game, c: CharId, g_2: f64, w: f64, kind: crate::character::ReadyKind, live: bool, dt: f64, tw_y: f64, tw_p: f64) { todo!() }

/// (src/game/character.ts:3813 CharacterAnimator.sheathRestInv)
pub fn anim_sheath_rest_inv(g: &mut crate::game::Game, c: CharId) -> Quat { todo!() }

/// (src/game/character.ts:3820 CharacterAnimator.takePre)
pub fn anim_take_pre(g: &mut crate::game::Game, c: CharId, dt: f64, cp: f64) -> f64 { todo!() }

/// (src/game/character.ts:3840 CharacterAnimator.blowTail)
pub fn anim_blow_tail(g: &mut crate::game::Game, c: CharId, b: &crate::character::BlowRun) -> f64 { todo!() }

/// (src/game/character.ts:3847 CharacterAnimator.blowLegs)
pub fn anim_blow_legs(g: &mut crate::game::Game, c: CharId, b: &crate::character::BlowRun) { todo!() }

/// (src/game/character.ts:3853 CharacterAnimator.blowLegsAsk)
pub fn anim_blow_legs_ask(g: &mut crate::game::Game, c: CharId, g_2: f64, b: &crate::character::BlowRun) { todo!() }

/// (src/game/character.ts:3860 CharacterAnimator.blowTrunk)
pub fn anim_blow_trunk(g: &mut crate::game::Game, c: CharId, b: &crate::character::BlowRun) { todo!() }

/// (src/game/character.ts:3865 CharacterAnimator.blowTrunkAsk)
pub fn anim_blow_trunk_ask(g: &mut crate::game::Game, c: CharId, g_2: f64, b: &crate::character::BlowRun, tl: f64) { todo!() }

/// (src/game/character.ts:3874 CharacterAnimator.blowArms)
pub fn anim_blow_arms(g: &mut crate::game::Game, c: CharId, b: &crate::character::BlowRun, tw_y: f64, tw_p: f64) -> f64 { todo!() }

/// (src/game/character.ts:3880 CharacterAnimator.blowArmsAsk)
pub fn anim_blow_arms_ask(g: &mut crate::game::Game, c: CharId, g_2: f64, b: &crate::character::BlowRun, tw_y: f64, tw_p: f64) -> f64 { todo!() }

/// (src/game/character.ts:3889 CharacterAnimator.takenLayer)
pub fn anim_taken_layer(g: &mut crate::game::Game, c: CharId, t: crate::character::AnimTakenLayerT) { todo!() }

/// (src/game/character.ts:3895 CharacterAnimator.takenLayerAsk)
pub fn anim_taken_layer_ask(g: &mut crate::game::Game, c: CharId, g_2: f64, t: crate::character::AnimTakenLayerAskT, u: f64) { todo!() }

/// (src/game/character.ts:3904 CharacterAnimator.knifeBlow)
pub fn anim_knife_blow(g: &mut crate::game::Game, c: CharId, kind: crate::character::StrikeKind, dt: f64, total: f64, tw_y: f64, tw_p: f64) { todo!() }

/// (src/game/character.ts:3912 CharacterAnimator.knifePose)
pub fn anim_knife_pose(g: &mut crate::game::Game, c: CharId, k: &crate::character::AnimKnifePoseK, t: f64, t0: f64, total: f64, tw_y: f64, tw_p: f64) { todo!() }

/// (src/game/character.ts:3917 CharacterAnimator.knifePoseAsk)
pub fn anim_knife_pose_ask(g: &mut crate::game::Game, c: CharId, g_2: f64, k: &crate::character::AnimKnifePoseAskK, t: f64, t0: f64, total: f64, tw_y: f64, tw_p: f64) { todo!() }

/// (src/game/character.ts:3929 CharacterAnimator.lockpickLayer)
pub fn anim_lockpick_layer(g: &mut crate::game::Game, c: CharId, w: f64) { todo!() }

/// (src/game/character.ts:3934 CharacterAnimator.lockpickLayerAsk)
pub fn anim_lockpick_layer_ask(g: &mut crate::game::Game, c: CharId, g_2: f64, w: f64) { todo!() }

/// (src/game/character.ts:3944 CharacterAnimator.haulLayer)
pub fn anim_haul_layer(g: &mut crate::game::Game, c: CharId, w: f64, tw_y: f64, tw_p: f64) { todo!() }

/// (src/game/character.ts:3950 CharacterAnimator.haulLayerAsk)
pub fn anim_haul_layer_ask(g: &mut crate::game::Game, c: CharId, g_2: f64, h: crate::character::HaulPose, w: f64, tw_y: f64, tw_p: f64) { todo!() }

/// (src/game/character.ts:3958 CharacterAnimator.toModelP)
pub fn anim_to_model_p(g: &mut crate::game::Game, c: CharId, wp: V3) -> V3 { todo!() }

/// (src/game/character.ts:3959 CharacterAnimator.toModelD)
pub fn anim_to_model_d(g: &mut crate::game::Game, c: CharId, wd: V3) -> V3 { todo!() }

/// (src/game/character.ts:3961 CharacterAnimator.doorHands)
pub fn anim_door_hands(g: &mut crate::game::Game, c: CharId) -> crate::character::AnimDoorHands { todo!() }

/// (src/game/character.ts:3974 CharacterAnimator.doorLayer)
pub fn anim_door_layer(g: &mut crate::game::Game, c: CharId, dt: f64, intent: f64, armed: bool, tw_y: f64, tw_p: f64) { todo!() }

/// (src/game/character.ts:3986 CharacterAnimator.doorLayerAsk)
pub fn anim_door_layer_ask(g: &mut crate::game::Game, c: CharId, g_2: f64, l: crate::character::DoorPush, dt: f64, intent: f64, armed: bool, tw_y: f64, tw_p: f64) { todo!() }

/// (src/game/character.ts:3994 CharacterAnimator.hwReset)
pub fn anim_hw_reset(g: &mut crate::game::Game, c: CharId) { todo!() }

/// (src/game/character.ts:3997 CharacterAnimator.workInfo)
pub fn anim_work_info(g: &mut crate::game::Game, c: CharId) -> crate::character::AnimWorkInfo { todo!() }

/// (src/game/character.ts:3999 CharacterAnimator.workAfter)
pub fn anim_work_after(g: &mut crate::game::Game, c: CharId, secs: f64) { todo!() }

/// (src/game/character.ts:4001 CharacterAnimator.radio)
pub fn anim_radio(g: &mut crate::game::Game, c: CharId, secs: f64) { todo!() }

/// (src/game/character.ts:4003 CharacterAnimator.radioShown)
pub fn anim_radio_shown(g: &mut crate::game::Game, c: CharId) -> f64 { todo!() }

/// (src/game/character.ts:4006 CharacterAnimator.swapHeld)
pub fn anim_swap_held(g: &mut crate::game::Game, c: CharId, rule: Option<crate::character::AnimSwapHeldRule>) -> Option<crate::character::AnimSwapHeld> { todo!() }

/// (src/game/character.ts:4010 CharacterAnimator.swapBelt)
pub fn anim_swap_belt(g: &mut crate::game::Game, c: CharId) -> f64 { todo!() }

/// (src/game/character.ts:4011 CharacterAnimator.swapInfo)
pub fn anim_swap_info(g: &mut crate::game::Game, c: CharId) -> crate::character::AnimSwapInfo { todo!() }

/// (src/game/character.ts:4018 CharacterAnimator.swapLayer)
pub fn anim_swap_layer(g: &mut crate::game::Game, c: CharId, dt: f64, taken: bool, tw_y: f64, tw_p: f64) { todo!() }

/// (src/game/character.ts:4028 CharacterAnimator.swapPose)
pub fn anim_swap_pose(g: &mut crate::game::Game, c: CharId, e: f64, k: f64, weapon: bool, tw_y: f64, tw_p: f64, sw: &crate::anim_skeleton::Pose) { todo!() }

/// (src/game/character.ts:4033 CharacterAnimator.swapPoseAsk)
pub fn anim_swap_pose_ask(g: &mut crate::game::Game, c: CharId, g_2: f64, e: f64, k: f64, weapon: bool, tw_y: f64, tw_p: f64, sw: &crate::anim_skeleton::Pose) { todo!() }

/// (src/game/character.ts:4044 CharacterAnimator.workLayer)
pub fn anim_work_layer(g: &mut crate::game::Game, c: CharId, dt: f64, free: bool, armed: bool, tw_y: f64, tw_p: f64) { todo!() }

/// (src/game/character.ts:4057 CharacterAnimator.workLayerAsk)
pub fn anim_work_layer_ask(g: &mut crate::game::Game, c: CharId, g_2: f64, l: &crate::character::WorkPose, dt: f64, live: bool, armed: bool, tw_y: f64, tw_p: f64) { todo!() }

/// (src/game/character.ts:4073 CharacterAnimator.torchUpLayer)
pub fn anim_torch_up_layer(g: &mut crate::game::Game, c: CharId, e: f64) { todo!() }

/// (src/game/character.ts:4078 CharacterAnimator.torchUpLayerAsk)
pub fn anim_torch_up_layer_ask(g: &mut crate::game::Game, c: CharId, g_2: f64, e: f64) { todo!() }

/// (src/game/character.ts:4086 CharacterAnimator.radioLayer)
pub fn anim_radio_layer(g: &mut crate::game::Game, c: CharId, dt: f64, free: bool) { todo!() }

/// (src/game/character.ts:4095 CharacterAnimator.radioPose)
pub fn anim_radio_pose(g: &mut crate::game::Game, c: CharId, w: f64, hw: f64) { todo!() }

/// (src/game/character.ts:4100 CharacterAnimator.radioPoseAsk)
pub fn anim_radio_pose_ask(g: &mut crate::game::Game, c: CharId, g_2: f64, w: f64, hw: f64) { todo!() }

/// (src/game/character.ts:4117 CharacterAnimator.holdLayer)
pub fn anim_hold_layer(g: &mut crate::game::Game, c: CharId, w: f64, dt: f64) { todo!() }

/// (src/game/character.ts:4131 CharacterAnimator.holdLayerAsk)
pub fn anim_hold_layer_ask(g: &mut crate::game::Game, c: CharId, g_2: f64, w: f64, dt: f64) { todo!() }

/// (src/game/character.ts:4151 CharacterAnimator.heldLayer)
pub fn anim_held_layer(g: &mut crate::game::Game, c: CharId, w: f64, dt: f64) { todo!() }

/// (src/game/character.ts:4163 CharacterAnimator.heldLayerAsk)
pub fn anim_held_layer_ask(g: &mut crate::game::Game, c: CharId, g_2: f64, w: f64, dt: f64) { todo!() }

/// (src/game/character.ts:4177 CharacterAnimator.holdBusy)
pub fn anim_hold_busy(g: &mut crate::game::Game, c: CharId) -> bool { todo!() }

/// (src/game/character.ts:4198 CharacterAnimator.bigLayer)
pub fn anim_big_layer(g: &mut crate::game::Game, c: CharId, b: &crate::character::AnimBigTool, tw_y: f64, tw_p: f64) { todo!() }

/// (src/game/character.ts:4213 CharacterAnimator.bigLayerAsk)
pub fn anim_big_layer_ask(g: &mut crate::game::Game, c: CharId, g_2: f64, b: &crate::character::AnimBigTool, blows: &[f64], tw_y: f64, tw_p: f64) { todo!() }

/// (src/game/character.ts:4237 CharacterAnimator.throwLayer)
pub fn anim_throw_layer(g: &mut crate::game::Game, c: CharId, t: f64) { todo!() }

/// (src/game/character.ts:4249 CharacterAnimator.throwLayerAsk)
pub fn anim_throw_layer_ask(g: &mut crate::game::Game, c: CharId, g_2: f64, t: f64, k1: &[Vec<[f64; 2]>], k3: &[Vec<(f64, Vec<f64>)>]) { todo!() }

/// (src/game/character.ts:4282 capRec)
pub fn cap_rec(a: V3, c_2: V3, r: f64, extend_b: Option<f64>) -> Vec<f64> { todo!() }

/// (src/game/character.ts:4289 capSole)
pub fn cap_sole(foot: V3, toe: V3) -> Vec<f64> { todo!() }

/// (src/game/character.ts:4355 Character.constructor)
pub fn character_new(g: &mut crate::game::Game, ch: RigId, d: crate::character::CharacterDesc) -> CharId { todo!() }

/// (src/game/character.ts:4371 Character.forward)
pub fn character_forward(g: &mut crate::game::Game, id: CharId) -> V3 { todo!() }

/// (src/game/character.ts:4372 Character.aimDir)
pub fn character_aim_dir(g: &mut crate::game::Game, id: CharId) -> V3 { todo!() }

/// (src/game/character.ts:4375 Character.handFrameR)
pub fn character_hand_frame_r(g: &mut crate::game::Game, id: CharId, sim: Option<bool>) -> crate::character::CharacterHandFrameR { todo!() }

/// (src/game/character.ts:4381 Character.gripFrameR)
pub fn character_grip_frame_r(g: &mut crate::game::Game, id: CharId) -> crate::character::CharacterGripFrameR { todo!() }

/// (src/game/character.ts:4387 Character.kitLieWorld)
pub fn character_kit_lie_world(g: &mut crate::game::Game, id: CharId) -> Option<Quat> { todo!() }

/// (src/game/character.ts:4393 Character.handFrameL)
pub fn character_hand_frame_l(g: &mut crate::game::Game, id: CharId) -> crate::character::CharacterHandFrameL { todo!() }

/// (src/game/character.ts:4400 Character.chestFrame)
pub fn character_chest_frame(g: &mut crate::game::Game, id: CharId) -> Option<crate::character::CharacterChestFrame> { todo!() }

/// (src/game/character.ts:4407 Character.update)
pub fn character_update(g: &mut crate::game::Game, id: CharId, dt: f64) { todo!() }

/// (src/game/character.ts:4408 Character.updateStanding)
pub fn character_update_standing(g: &mut crate::game::Game, id: CharId, dt: f64) { todo!() }

/// (src/game/character.ts:4436 Character.packSidePass)
pub fn character_pack_side_pass(g: &mut crate::game::Game, id: CharId) { todo!() }

/// (src/game/character.ts:4451 Character.reachOv)
pub fn character_reach_ov(g: &mut crate::game::Game, id: CharId, ov: &mut Vec<Option<crate::anim_skeleton::NodeOverride>>, right: bool, t: V3, rot: Quat) { todo!() }

/// (src/game/character.ts:4472 Character.inHandPass)
pub fn character_in_hand_pass(g: &mut crate::game::Game, id: CharId, dt: f64) { todo!() }

/// (src/game/character.ts:4488 Character.inHandWord)
pub fn character_in_hand_word(g: &mut crate::game::Game, id: CharId, dt: f64, cur: Option<crate::character::CharacterInHandWordCur>, hr: Quat, r: V3, l: V3, ov: &mut Vec<Option<crate::anim_skeleton::NodeOverride>>, reach: &mut dyn FnMut(&mut crate::game::Game, bool, V3, Quat)) { todo!() }

/// (src/game/character.ts:4526 Character.turnWeigh)
pub fn character_turn_weigh(g: &mut crate::game::Game, id: CharId, dt: f64) { todo!() }

/// (src/game/character.ts:4531 Character.turnWeighAsk)
pub fn character_turn_weigh_ask(g: &mut crate::game::Game, id: CharId, dt: f64) { todo!() }

/// (src/game/character.ts:4554 Character.turnSnap)
pub fn character_turn_snap(g: &mut crate::game::Game, id: CharId) -> Option<Vec<f64>> { todo!() }

/// (src/game/character.ts:4557 Character.turnPut)
pub fn character_turn_put(g: &mut crate::game::Game, id: CharId, s: Option<&[f64]>) { todo!() }

/// (src/game/character.ts:4566 Character.bakeLoaded)
pub fn character_bake_loaded(g: &mut crate::game::Game, id: CharId) { todo!() }

/// (src/game/character.ts:4572 Character.turnDrop)
pub fn character_turn_drop(g: &mut crate::game::Game, id: CharId) { todo!() }

/// (src/game/character.ts:4577 Character.turnBegin)
pub fn character_turn_begin(g: &mut crate::game::Game, id: CharId) { todo!() }

/// (src/game/character.ts:4580 Character.gunYaw)
pub fn character_gun_yaw(g: &mut crate::game::Game, id: CharId) -> f64 { todo!() }

/// (src/game/character.ts:4582 Character.wantBody)
pub fn character_want_body(g: &mut crate::game::Game, id: CharId) -> f64 { todo!() }

/// (src/game/character.ts:4587 Character.jointUnder)
pub fn character_joint_under(g: &mut crate::game::Game, id: CharId, name: &str) -> Option<V3> { todo!() }

/// (src/game/character.ts:4588 Character.joints)
pub fn character_joints(g: &mut crate::game::Game, id: CharId, b: &mut JsMap<String, V3>) -> JsMap<String, V3> { todo!() }

/// (src/game/character.ts:4598 Character.readBones)
pub fn character_read_bones(g: &mut crate::game::Game, id: CharId, dt: f64) { todo!() }

/// (src/game/character.ts:4624 Character.ragdollize)
pub fn character_ragdollize(g: &mut crate::game::Game, id: CharId, world: Option<&crate::ragdoll::RagdollWorld>, dir: V3, quiet: bool) { todo!() }

/// (src/game/character.ts:4665 Character.updateRagdoll)
pub fn character_update_ragdoll(g: &mut crate::game::Game, id: CharId, dt: f64) { todo!() }

/// (src/game/character.ts:4697 Character.fallGuide)
pub fn character_fall_guide(g: &mut crate::game::Game, id: CharId, rag: RagdollId, kind: crate::character::FallStyle, f: V3, h: V3, j: f64, side: f64) -> crate::character::FallGuide { todo!() }

/// (src/game/character.ts:4774 Character.guideStep)
pub fn character_guide_step(g: &mut crate::game::Game, id: CharId, rag: RagdollId, dt: f64) { todo!() }

/// (src/game/character.ts:4788 Character.breathFrame)
pub fn character_breath_frame(g: &mut crate::game::Game, id: CharId, dt: f64, k: crate::character::CharacterBreathFrameK) { todo!() }

/// (src/game/character.ts:4803 Character.breathEnd)
pub fn character_breath_end(g: &mut crate::game::Game, id: CharId, pose: bool) { todo!() }

/// (src/game/character.ts:4810 Character.gunUnder)
pub fn character_gun_under(g: &mut crate::game::Game, id: CharId) -> V3 { todo!() }

/// (src/game/character.ts:4817 Character.headRot)
pub fn character_head_rot(g: &mut crate::game::Game, id: CharId) -> Quat { todo!() }

/// (src/game/character.ts:4819 Character.headDir)
pub fn character_head_dir(g: &mut crate::game::Game, id: CharId) -> V3 { todo!() }

/// (src/game/character.ts:4822 Character.capsules)
pub fn character_capsules(g: &mut crate::game::Game, id: CharId) -> crate::character::CharacterCapsules { todo!() }

/// (src/game/character.ts:4846 Character.insideBody)
pub fn character_inside_body(g: &mut crate::game::Game, id: CharId, p: V3, crown_pad: Option<f64>) -> bool { todo!() }

/// (src/game/character.ts:4858 Character.rayHit)
pub fn character_ray_hit(g: &mut crate::game::Game, id: CharId, ro: V3, rd: V3, tmax: f64) -> f64 { todo!() }

pub mod fold_with {
    #[allow(unused_imports)]
    use crate::{ids::*, js::*, Quat, V3};
    #[allow(unused_imports)]
    use alloc::{string::String, vec::Vec};
    /// (src/game/character.ts:785 FOLD_WITH.on)
    pub fn on() -> bool { todo!() }

}
