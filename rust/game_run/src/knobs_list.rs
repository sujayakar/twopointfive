//! Written by tools/draw/gen.ts from tools/blitz/schema.json: every number of the knob tables that is not settled, by its name, where it lies, and whether it is a yes or no.
use core::mem::offset_of;
use game::knobs::Knobs;

pub static KNOBS: &[(&str, usize, bool)] = &[
    ("aim.guard.settle_secs", offset_of!(Knobs, aim.guard.settle_secs), false),
    ("aim.guard.range_ref", offset_of!(Knobs, aim.guard.range_ref), false),
    ("aim.guard.near_k", offset_of!(Knobs, aim.guard.near_k), false),
    ("aim.guard.acquire_min", offset_of!(Knobs, aim.guard.acquire_min), false),
    ("aim.guard.unsettled_k", offset_of!(Knobs, aim.guard.unsettled_k), false),
    ("aim.guard.on_deg", offset_of!(Knobs, aim.guard.on_deg), false),
    ("aim.guard.lose_rate", offset_of!(Knobs, aim.guard.lose_rate), false),
    ("aim.guard.mem_secs", offset_of!(Knobs, aim.guard.mem_secs), false),
    ("aim.guard.jump_deg", offset_of!(Knobs, aim.guard.jump_deg), false),
    ("aim.guard.target_move_k", offset_of!(Knobs, aim.guard.target_move_k), false),
    ("aim.guard.target_move_cap", offset_of!(Knobs, aim.guard.target_move_cap), false),
    ("aim.guard.rate_tau", offset_of!(Knobs, aim.guard.rate_tau), false),
    ("aim.guard.shooter_move_k", offset_of!(Knobs, aim.guard.shooter_move_k), false),
    ("aim.guard.feet_tau", offset_of!(Knobs, aim.guard.feet_tau), false),
    ("aim.guard.steady_pace", offset_of!(Knobs, aim.guard.steady_pace), false),
    ("aim.guard.stop_to_shoot", offset_of!(Knobs, aim.guard.stop_to_shoot), false),
    ("aim.guard.moving_track_cap", offset_of!(Knobs, aim.guard.moving_track_cap), false),
    ("aim.guard.shoot_band", offset_of!(Knobs, aim.guard.shoot_band), false),
    ("aim.guard.brake_lead", offset_of!(Knobs, aim.guard.brake_lead), false),
    ("aim.player.settle_secs", offset_of!(Knobs, aim.player.settle_secs), false),
    ("aim.player.range_ref", offset_of!(Knobs, aim.player.range_ref), false),
    ("aim.player.near_k", offset_of!(Knobs, aim.player.near_k), false),
    ("aim.player.unsettled_k", offset_of!(Knobs, aim.player.unsettled_k), false),
    ("aim.player.lose_rate", offset_of!(Knobs, aim.player.lose_rate), false),
    ("aim.player.base_stand", offset_of!(Knobs, aim.player.base_stand), false),
    ("aim.player.base_crouch", offset_of!(Knobs, aim.player.base_crouch), false),
    ("aim.player.target_move_k", offset_of!(Knobs, aim.player.target_move_k), false),
    ("aim.player.target_move_cap", offset_of!(Knobs, aim.player.target_move_cap), false),
    ("aim.player.rate_tau", offset_of!(Knobs, aim.player.rate_tau), false),
    ("aim.player.jump_deg", offset_of!(Knobs, aim.player.jump_deg), false),
    ("aim.player.shooter_move_k", offset_of!(Knobs, aim.player.shooter_move_k), false),
    ("aim.player.feet_tau", offset_of!(Knobs, aim.player.feet_tau), false),
    ("aim.player.steady_pace", offset_of!(Knobs, aim.player.steady_pace), false),
    ("aim.player.sprint_cap", offset_of!(Knobs, aim.player.sprint_cap), false),
    ("aim.player.walk_cap", offset_of!(Knobs, aim.player.walk_cap), false),
    ("aim.player.moving_above", offset_of!(Knobs, aim.player.moving_above), false),
    ("aim.player.crouch_k", offset_of!(Knobs, aim.player.crouch_k), false),
    ("trained.settle_k", offset_of!(Knobs, trained.settle_k), false),
    ("trained.spread_k", offset_of!(Knobs, trained.spread_k), false),
    ("trained.react_cap", offset_of!(Knobs, trained.react_cap), false),
    ("body_light.lit", offset_of!(Knobs, body_light.lit), false),
    ("body_light.lit_near", offset_of!(Knobs, body_light.lit_near), false),
    ("body_light.near_m", offset_of!(Knobs, body_light.near_m), false),
    ("body_light.far_m", offset_of!(Knobs, body_light.far_m), false),
    ("body_light.smoke", offset_of!(Knobs, body_light.smoke), false),
    ("guard_eye.range", offset_of!(Knobs, guard_eye.range), false),
    ("guard_eye.half_deg", offset_of!(Knobs, guard_eye.half_deg), false),
    ("guard_eye.near_m", offset_of!(Knobs, guard_eye.near_m), false),
    ("guard_eye.near_deg", offset_of!(Knobs, guard_eye.near_deg), false),
    ("guard_eye.far_fix_at", offset_of!(Knobs, guard_eye.far_fix_at), false),
    ("guard_eye.far_range", offset_of!(Knobs, guard_eye.far_range), false),
    ("guard_eye.far_lo", offset_of!(Knobs, guard_eye.far_lo), false),
    ("guard_eye.far_hi", offset_of!(Knobs, guard_eye.far_hi), false),
    ("guard_eye.far_k", offset_of!(Knobs, guard_eye.far_k), false),
    ("needs.rate_k", offset_of!(Knobs, needs.rate_k), false),
    ("needs.max_away", offset_of!(Knobs, needs.max_away), false),
    ("needs.announce_lead", offset_of!(Knobs, needs.announce_lead), false),
    ("needs.say_lead", offset_of!(Knobs, needs.say_lead), false),
    ("needs.stretch_lead", offset_of!(Knobs, needs.stretch_lead), false),
    ("needs.partner_m", offset_of!(Knobs, needs.partner_m), false),
    ("needs.reply_after", offset_of!(Knobs, needs.reply_after), false),
    ("needs.min_wait", offset_of!(Knobs, needs.min_wait), false),
    ("needs.min_trip_m", offset_of!(Knobs, needs.min_trip_m), false),
    ("needs.after_cancel", offset_of!(Knobs, needs.after_cancel), false),
    ("needs.after_alert", offset_of!(Knobs, needs.after_alert), false),
    ("needs.rest_secs", offset_of!(Knobs, needs.rest_secs), false),
    ("needs.floor_gap_secs", offset_of!(Knobs, needs.floor_gap_secs), false),
    ("needs.pace_k", offset_of!(Knobs, needs.pace_k), false),
    ("needs.stuck_secs", offset_of!(Knobs, needs.stuck_secs), false),
    ("whistle.level", offset_of!(Knobs, whistle.level), false),
    ("whistle.reach_k", offset_of!(Knobs, whistle.reach_k), false),
    ("whistle.horizon", offset_of!(Knobs, whistle.horizon), false),
    ("whistle.cooldown", offset_of!(Knobs, whistle.cooldown), false),
    ("whistle.twice_window", offset_of!(Knobs, whistle.twice_window), false),
    ("whistle.awareness", offset_of!(Knobs, whistle.awareness), false),
    ("whistle.react_t", offset_of!(Knobs, whistle.react_t), false),
    ("whistle.walk_floor", offset_of!(Knobs, whistle.walk_floor), false),
    ("whistle.glance_secs", offset_of!(Knobs, whistle.glance_secs), false),
    ("whistle.glance_max", offset_of!(Knobs, whistle.glance_max), false),
    ("whistle.gain", offset_of!(Knobs, whistle.gain), false),
    ("wet.soak_per_sec", offset_of!(Knobs, wet.soak_per_sec), false),
    ("wet.dry_per_sec", offset_of!(Knobs, wet.dry_per_sec), false),
    ("wet.bead_runoff_secs", offset_of!(Knobs, wet.bead_runoff_secs), false),
    ("wet.bead_density", offset_of!(Knobs, wet.bead_density), false),
    ("wet.bead_size", offset_of!(Knobs, wet.bead_size), false),
    ("wet.trail_len", offset_of!(Knobs, wet.trail_len), false),
    ("wet.rough_wet", offset_of!(Knobs, wet.rough_wet), false),
    ("wet.rough_dry", offset_of!(Knobs, wet.rough_dry), false),
    ("wet.darken", offset_of!(Knobs, wet.darken), false),
    ("wet.darken_mul", offset_of!(Knobs, wet.darken_mul), false),
    ("wet.spec_gain", offset_of!(Knobs, wet.spec_gain), false),
    ("wet.drip_secs", offset_of!(Knobs, wet.drip_secs), false),
    ("wet.prints", offset_of!(Knobs, wet.prints), false),
    ("wet.line_band", offset_of!(Knobs, wet.line_band), false),
    ("wet.print_fade_secs", offset_of!(Knobs, wet.print_fade_secs), false),
    ("wind.def", offset_of!(Knobs, wind.def), false),
    ("wind.vent_k", offset_of!(Knobs, wind.vent_k), false),
    ("wind.storm_k", offset_of!(Knobs, wind.storm_k), false),
    ("wind.door_k", offset_of!(Knobs, wind.door_k), false),
    ("wind.door_reach", offset_of!(Knobs, wind.door_reach), false),
    ("wind.door_spread_deg", offset_of!(Knobs, wind.door_spread_deg), false),
    ("wind.door_open_rad", offset_of!(Knobs, wind.door_open_rad), false),
    ("wind.gust_amp", offset_of!(Knobs, wind.gust_amp), false),
    ("wind.gust_period", offset_of!(Knobs, wind.gust_period), false),
    ("wind.storm_gust_amp", offset_of!(Knobs, wind.storm_gust_amp), false),
    ("wind.storm_gust_period", offset_of!(Knobs, wind.storm_gust_period), false),
    ("wind.ramp_up", offset_of!(Knobs, wind.ramp_up), false),
    ("wind.ramp_down", offset_of!(Knobs, wind.ramp_down), false),
    ("wind.smoke_relax", offset_of!(Knobs, wind.smoke_relax), false),
    ("wind.sheet_wake", offset_of!(Knobs, wind.sheet_wake), false),
    ("wind.sheet_kick", offset_of!(Knobs, wind.sheet_kick), false),
    ("wind.debris_k", offset_of!(Knobs, wind.debris_k), false),
    ("wind.near_m", offset_of!(Knobs, wind.near_m), false),
    ("civ.eye.range", offset_of!(Knobs, civ.eye.range), false),
    ("civ.eye.half_deg", offset_of!(Knobs, civ.eye.half_deg), false),
    ("civ.eye.near_m", offset_of!(Knobs, civ.eye.near_m), false),
    ("civ.eye.near_deg", offset_of!(Knobs, civ.eye.near_deg), false),
    ("civ.eye_busy.range", offset_of!(Knobs, civ.eye_busy.range), false),
    ("civ.eye_busy.half_deg", offset_of!(Knobs, civ.eye_busy.half_deg), false),
    ("civ.eye_busy.near_m", offset_of!(Knobs, civ.eye_busy.near_m), false),
    ("civ.eye_busy.near_deg", offset_of!(Knobs, civ.eye_busy.near_deg), false),
    ("civ.eye_alarmed.range", offset_of!(Knobs, civ.eye_alarmed.range), false),
    ("civ.eye_alarmed.half_deg", offset_of!(Knobs, civ.eye_alarmed.half_deg), false),
    ("civ.eye_alarmed.near_m", offset_of!(Knobs, civ.eye_alarmed.near_m), false),
    ("civ.eye_alarmed.near_deg", offset_of!(Knobs, civ.eye_alarmed.near_deg), false),
    ("civ.keen_k", offset_of!(Knobs, civ.keen_k), false),
    ("civ.gun_k", offset_of!(Knobs, civ.gun_k), false),
    ("civ.gun_m", offset_of!(Knobs, civ.gun_m), false),
    ("civ.glimpse_at", offset_of!(Knobs, civ.glimpse_at), false),
    ("civ.spot_at", offset_of!(Knobs, civ.spot_at), false),
    ("civ.startle_secs", offset_of!(Knobs, civ.startle_secs), false),
    ("civ.startle_again_secs", offset_of!(Knobs, civ.startle_again_secs), false),
    ("civ.body_m", offset_of!(Knobs, civ.body_m), false),
    ("civ.freeze.0", offset_of!(Knobs, civ.freeze), false),
    ("civ.freeze.1", offset_of!(Knobs, civ.freeze) + 8, false),
    ("civ.run_speed", offset_of!(Knobs, civ.run_speed), false),
    ("civ.walk_speed", offset_of!(Knobs, civ.walk_speed), false),
    ("civ.creep_speed", offset_of!(Knobs, civ.creep_speed), false),
    ("civ.report_m", offset_of!(Knobs, civ.report_m), false),
    ("civ.report_secs", offset_of!(Knobs, civ.report_secs), false),
    ("civ.phone_secs", offset_of!(Knobs, civ.phone_secs), false),
    ("civ.guard_bonus_m", offset_of!(Knobs, civ.guard_bonus_m), false),
    ("civ.exit_malus_m", offset_of!(Knobs, civ.exit_malus_m), false),
    ("civ.past_threat_m", offset_of!(Knobs, civ.past_threat_m), false),
    ("civ.corner_m", offset_of!(Knobs, civ.corner_m), false),
    ("civ.reflee_m", offset_of!(Knobs, civ.reflee_m), false),
    ("civ.lone_secs", offset_of!(Knobs, civ.lone_secs), false),
    ("civ.settle_secs", offset_of!(Knobs, civ.settle_secs), false),
    ("civ.comply_min", offset_of!(Knobs, civ.comply_min), false),
    ("civ.scream_m", offset_of!(Knobs, civ.scream_m), false),
    ("civ.see_m", offset_of!(Knobs, civ.see_m), false),
    ("civ.see_lit", offset_of!(Knobs, civ.see_lit), false),
    ("civ.found_m", offset_of!(Knobs, civ.found_m), false),
    ("civ.found_secs", offset_of!(Knobs, civ.found_secs), false),
    ("civ.outside_call_secs", offset_of!(Knobs, civ.outside_call_secs), false),
    ("civ.greet_m", offset_of!(Knobs, civ.greet_m), false),
    ("civ.greet_pair_secs", offset_of!(Knobs, civ.greet_pair_secs), false),
    ("civ.greet_floor_secs", offset_of!(Knobs, civ.greet_floor_secs), false),
    ("chaos.def", offset_of!(Knobs, chaos.def), false),
    ("chaos.heat_tau", offset_of!(Knobs, chaos.heat_tau), false),
    ("chaos.cell", offset_of!(Knobs, chaos.cell), false),
    ("chaos.t_decal", offset_of!(Knobs, chaos.t_decal), false),
    ("chaos.t_paper", offset_of!(Knobs, chaos.t_paper), false),
    ("chaos.t_burst", offset_of!(Knobs, chaos.t_burst), false),
    ("chaos.t_puff", offset_of!(Knobs, chaos.t_puff), false),
    ("chaos.t_knock", offset_of!(Knobs, chaos.t_knock), false),
    ("chaos.t_tile", offset_of!(Knobs, chaos.t_tile), false),
    ("chaos.bang_heat", offset_of!(Knobs, chaos.bang_heat), false),
    ("chaos.puffs_per_sec", offset_of!(Knobs, chaos.puffs_per_sec), false),
    ("chaos.puffs_per_frame", offset_of!(Knobs, chaos.puffs_per_frame), false),
    ("chaos.papers_per_sec", offset_of!(Knobs, chaos.papers_per_sec), false),
    ("chaos.knocks_per_sec", offset_of!(Knobs, chaos.knocks_per_sec), false),
    ("chaos.tile_gap_secs", offset_of!(Knobs, chaos.tile_gap_secs), false),
    ("chaos.tiles_per_encounter", offset_of!(Knobs, chaos.tiles_per_encounter), false),
    ("chaos.decals_per_sec", offset_of!(Knobs, chaos.decals_per_sec), false),
    ("chaos.paper_reach_m", offset_of!(Knobs, chaos.paper_reach_m), false),
    ("chaos.knock_reach_m", offset_of!(Knobs, chaos.knock_reach_m), false),
    ("chaos.sheets_per_stack", offset_of!(Knobs, chaos.sheets_per_stack), false),
    ("chaos.puff.radius", offset_of!(Knobs, chaos.puff.radius), false),
    ("chaos.puff.density", offset_of!(Knobs, chaos.puff.density), false),
    ("chaos.puff.ttl", offset_of!(Knobs, chaos.puff.ttl), false),
    ("chaos.puff.speed", offset_of!(Knobs, chaos.puff.speed), false),
    ("chaos.puff.temp", offset_of!(Knobs, chaos.puff.temp), false),
    ("chaos.puff.push", offset_of!(Knobs, chaos.puff.push), false),
    ("chaos.puff.expand", offset_of!(Knobs, chaos.puff.expand), false),
    ("chaos.tile.half.0", offset_of!(Knobs, chaos.tile.half), false),
    ("chaos.tile.half.1", offset_of!(Knobs, chaos.tile.half) + 8, false),
    ("chaos.tile.half.2", offset_of!(Knobs, chaos.tile.half) + 16, false),
    ("chaos.tile.albedo.0", offset_of!(Knobs, chaos.tile.albedo), false),
    ("chaos.tile.albedo.1", offset_of!(Knobs, chaos.tile.albedo) + 8, false),
    ("chaos.tile.albedo.2", offset_of!(Knobs, chaos.tile.albedo) + 16, false),
    ("chaos.vel_low", offset_of!(Knobs, chaos.vel_low), false),
    ("chaos.vel_high_k", offset_of!(Knobs, chaos.vel_high_k), false),
    ("chaos.imp.bang.radius", offset_of!(Knobs, chaos.imp.bang.radius), false),
    ("chaos.imp.bang.strength", offset_of!(Knobs, chaos.imp.bang.strength), false),
    ("chaos.imp.bang.up", offset_of!(Knobs, chaos.imp.bang.up), false),
    ("chaos.imp.impact.radius", offset_of!(Knobs, chaos.imp.impact.radius), false),
    ("chaos.imp.impact.strength", offset_of!(Knobs, chaos.imp.impact.strength), false),
    ("chaos.imp.impact.up", offset_of!(Knobs, chaos.imp.impact.up), false),
    ("chaos.imp.impact.bias_k", offset_of!(Knobs, chaos.imp.impact.bias_k), false),
    ("chaos.imp.impact.debris_m", offset_of!(Knobs, chaos.imp.impact.debris_m), false),
    ("chaos.imp.thud.radius", offset_of!(Knobs, chaos.imp.thud.radius), false),
    ("chaos.imp.thud.strength", offset_of!(Knobs, chaos.imp.thud.strength), false),
    ("chaos.imp.thud.up", offset_of!(Knobs, chaos.imp.thud.up), false),
    ("chaos.imp.thud.per_frame", offset_of!(Knobs, chaos.imp.thud.per_frame), false),
    ("chaos.imp.chaos_k", offset_of!(Knobs, chaos.imp.chaos_k), false),
    ("chaos.loud.fixture_hit_m", offset_of!(Knobs, chaos.loud.fixture_hit_m), false),
    ("chaos.loud.fixture_blast_m", offset_of!(Knobs, chaos.loud.fixture_blast_m), false),
    ("chaos.loud.stun_fixture_m", offset_of!(Knobs, chaos.loud.stun_fixture_m), false),
    ("chaos.loud.fixtures_per_encounter", offset_of!(Knobs, chaos.loud.fixtures_per_encounter), false),
    ("chaos.loud.fixture_gap_secs", offset_of!(Knobs, chaos.loud.fixture_gap_secs), false),
    ("chaos.loud.blast_dust_floor", offset_of!(Knobs, chaos.loud.blast_dust_floor), false),
    ("chaos.loud.blast_dust_puffs", offset_of!(Knobs, chaos.loud.blast_dust_puffs), false),
    ("chaos.loud.heat_k", offset_of!(Knobs, chaos.loud.heat_k), false),
    ("chaos.loud.burst_queue", offset_of!(Knobs, chaos.loud.burst_queue), false),
    ("chaos.loud.burst_secs", offset_of!(Knobs, chaos.loud.burst_secs), false),
    ("chaos.pall.per_window", offset_of!(Knobs, chaos.pall.per_window), false),
    ("chaos.pall.window_secs", offset_of!(Knobs, chaos.pall.window_secs), false),
    ("chaos.pall.slack", offset_of!(Knobs, chaos.pall.slack), false),
    ("chaos.pall.clear", offset_of!(Knobs, chaos.pall.clear), false),
    ("chaos.pall.clear_y", offset_of!(Knobs, chaos.pall.clear_y), false),
    ("chaos.pall.min_radius", offset_of!(Knobs, chaos.pall.min_radius), false),
    ("chaos.pall.max_gain", offset_of!(Knobs, chaos.pall.max_gain), false),
    ("chaos.pall.wall.near", offset_of!(Knobs, chaos.pall.wall.near), false),
    ("chaos.pall.wall.far", offset_of!(Knobs, chaos.pall.wall.far), false),
    ("chaos.pall.wall.radius", offset_of!(Knobs, chaos.pall.wall.radius), false),
    ("chaos.pall.wall.density", offset_of!(Knobs, chaos.pall.wall.density), false),
    ("chaos.pall.wall.ttl", offset_of!(Knobs, chaos.pall.wall.ttl), false),
    ("chaos.pall.wall.speed", offset_of!(Knobs, chaos.pall.wall.speed), false),
    ("chaos.pall.wall.temp", offset_of!(Knobs, chaos.pall.wall.temp), false),
    ("chaos.pall.wall.push", offset_of!(Knobs, chaos.pall.wall.push), false),
    ("chaos.pall.wall.expand", offset_of!(Knobs, chaos.pall.wall.expand), false),
    ("chaos.pall.wall.spread", offset_of!(Knobs, chaos.pall.wall.spread), false),
    ("chaos.pall.wall.out", offset_of!(Knobs, chaos.pall.wall.out), false),
    ("chaos.pall.wall.dust.0", offset_of!(Knobs, chaos.pall.wall.dust), false),
    ("chaos.pall.wall.dust.1", offset_of!(Knobs, chaos.pall.wall.dust) + 8, false),
    ("chaos.pall.wall.dust_life", offset_of!(Knobs, chaos.pall.wall.dust_life), false),
    ("chaos.pall.wall.dust_size", offset_of!(Knobs, chaos.pall.wall.dust_size), false),
    ("chaos.pall.wall.dust_speed", offset_of!(Knobs, chaos.pall.wall.dust_speed), false),
    ("chaos.pall.burst.ring", offset_of!(Knobs, chaos.pall.burst.ring), false),
    ("chaos.pall.burst.core_gain", offset_of!(Knobs, chaos.pall.burst.core_gain), false),
    ("chaos.pall.burst.radius", offset_of!(Knobs, chaos.pall.burst.radius), false),
    ("chaos.pall.burst.density", offset_of!(Knobs, chaos.pall.burst.density), false),
    ("chaos.pall.burst.ttl", offset_of!(Knobs, chaos.pall.burst.ttl), false),
    ("chaos.pall.burst.speed", offset_of!(Knobs, chaos.pall.burst.speed), false),
    ("chaos.pall.burst.temp", offset_of!(Knobs, chaos.pall.burst.temp), false),
    ("chaos.pall.burst.push", offset_of!(Knobs, chaos.pall.burst.push), false),
    ("chaos.pall.burst.expand", offset_of!(Knobs, chaos.pall.burst.expand), false),
    ("chaos.pall.burst.spread", offset_of!(Knobs, chaos.pall.burst.spread), false),
    ("chaos.pall.burst.dust", offset_of!(Knobs, chaos.pall.burst.dust), false),
    ("chaos.pall.burst.dust_life", offset_of!(Knobs, chaos.pall.burst.dust_life), false),
    ("chaos.pall.burst.dust_size", offset_of!(Knobs, chaos.pall.burst.dust_size), false),
    ("chaos.pall.burst.dust_speed", offset_of!(Knobs, chaos.pall.burst.dust_speed), false),
    ("chaos.pall.burst.smoke.0", offset_of!(Knobs, chaos.pall.burst.smoke), false),
    ("chaos.pall.burst.smoke.1", offset_of!(Knobs, chaos.pall.burst.smoke) + 8, false),
    ("chaos.pall.burst.smoke.2", offset_of!(Knobs, chaos.pall.burst.smoke) + 16, false),
    ("chaos.mess.stand_y", offset_of!(Knobs, chaos.mess.stand_y), false),
    ("chaos.mess.clear_y", offset_of!(Knobs, chaos.mess.clear_y), false),
    ("chaos.mess.under", offset_of!(Knobs, chaos.mess.under), false),
    ("chaos.mess.min_radius", offset_of!(Knobs, chaos.mess.min_radius), false),
    ("chaos.mess.crowd", offset_of!(Knobs, chaos.mess.crowd), false),
    ("chaos.mess.roof_y", offset_of!(Knobs, chaos.mess.roof_y), false),
    ("chaos.mess.burst.core", offset_of!(Knobs, chaos.mess.burst.core), false),
    ("chaos.mess.burst.lobes", offset_of!(Knobs, chaos.mess.burst.lobes), false),
    ("chaos.mess.burst.reach", offset_of!(Knobs, chaos.mess.burst.reach), false),
    ("chaos.mess.burst.lobe", offset_of!(Knobs, chaos.mess.burst.lobe), false),
    ("chaos.mess.burst.strength", offset_of!(Knobs, chaos.mess.burst.strength), false),
    ("chaos.mess.wall.out", offset_of!(Knobs, chaos.mess.wall.out), false),
    ("chaos.mess.wall.near.core", offset_of!(Knobs, chaos.mess.wall.near.core), false),
    ("chaos.mess.wall.near.lobes", offset_of!(Knobs, chaos.mess.wall.near.lobes), false),
    ("chaos.mess.wall.near.reach", offset_of!(Knobs, chaos.mess.wall.near.reach), false),
    ("chaos.mess.wall.near.lobe", offset_of!(Knobs, chaos.mess.wall.near.lobe), false),
    ("chaos.mess.wall.near.strength", offset_of!(Knobs, chaos.mess.wall.near.strength), false),
    ("chaos.mess.wall.far.core", offset_of!(Knobs, chaos.mess.wall.far.core), false),
    ("chaos.mess.wall.far.lobes", offset_of!(Knobs, chaos.mess.wall.far.lobes), false),
    ("chaos.mess.wall.far.reach", offset_of!(Knobs, chaos.mess.wall.far.reach), false),
    ("chaos.mess.wall.far.lobe", offset_of!(Knobs, chaos.mess.wall.far.lobe), false),
    ("chaos.mess.wall.far.strength", offset_of!(Knobs, chaos.mess.wall.far.strength), false),
    ("chaos.mess.foot.out", offset_of!(Knobs, chaos.mess.foot.out), false),
    ("chaos.mess.foot.radius", offset_of!(Knobs, chaos.mess.foot.radius), false),
    ("chaos.mess.foot.per_metre", offset_of!(Knobs, chaos.mess.foot.per_metre), false),
    ("chaos.mess.foot.strength", offset_of!(Knobs, chaos.mess.foot.strength), false),
    ("chaos.mess.top.lo", offset_of!(Knobs, chaos.mess.top.lo), false),
    ("chaos.mess.top.hi", offset_of!(Knobs, chaos.mess.top.hi), false),
    ("chaos.mess.top.strength", offset_of!(Knobs, chaos.mess.top.strength), false),
    ("chaos.mess.rubble.least", offset_of!(Knobs, chaos.mess.rubble.least), false),
    ("chaos.mess.rubble.grow", offset_of!(Knobs, chaos.mess.rubble.grow), false),
    ("chaos.mess.rubble.min", offset_of!(Knobs, chaos.mess.rubble.min), false),
    ("chaos.mess.rubble.max", offset_of!(Knobs, chaos.mess.rubble.max), false),
    ("chaos.mess.rubble.crowd", offset_of!(Knobs, chaos.mess.rubble.crowd), false),
    ("chaos.mess.rubble.strength", offset_of!(Knobs, chaos.mess.rubble.strength), false),
    ("ring.speed", offset_of!(Knobs, ring.speed), false),
    ("ring.gravity", offset_of!(Knobs, ring.gravity), false),
    ("ring.drag", offset_of!(Knobs, ring.drag), false),
    ("ring.radius", offset_of!(Knobs, ring.radius), false),
    ("ring.life", offset_of!(Knobs, ring.life), false),
    ("ring.substeps", offset_of!(Knobs, ring.substeps), false),
    ("ring.ko_below_neck", offset_of!(Knobs, ring.ko_below_neck), false),
    ("ring.stagger", offset_of!(Knobs, ring.stagger), false),
    ("ring.stagger_fire_cd", offset_of!(Knobs, ring.stagger_fire_cd), false),
    ("ring.stagger_aware", offset_of!(Knobs, ring.stagger_aware), false),
    ("ring.stagger_back", offset_of!(Knobs, ring.stagger_back), false),
    ("ring.launch_noise", offset_of!(Knobs, ring.launch_noise), false),
    ("ring.clack_noise", offset_of!(Knobs, ring.clack_noise), false),
    ("ring.ko_noise", offset_of!(Knobs, ring.ko_noise), false),
    ("ring.hit_noise", offset_of!(Knobs, ring.hit_noise), false),
    ("ring.prop_j", offset_of!(Knobs, ring.prop_j), false),
    ("ring.restitution", offset_of!(Knobs, ring.restitution), false),
    ("ring.keep_tangent", offset_of!(Knobs, ring.keep_tangent), false),
    ("ring.spent_speed_max", offset_of!(Knobs, ring.spent_speed_max), false),
    ("ring.solve_iters", offset_of!(Knobs, ring.solve_iters), false),
    ("launcher.rings", offset_of!(Knobs, launcher.rings), false),
    ("launcher.cd", offset_of!(Knobs, launcher.cd), false),
    ("he40.speed", offset_of!(Knobs, he40.speed), false),
    ("he40.gravity", offset_of!(Knobs, he40.gravity), false),
    ("he40.drag", offset_of!(Knobs, he40.drag), false),
    ("he40.radius", offset_of!(Knobs, he40.radius), false),
    ("he40.life", offset_of!(Knobs, he40.life), false),
    ("he40.substeps", offset_of!(Knobs, he40.substeps), false),
    ("he40.solve_iters", offset_of!(Knobs, he40.solve_iters), false),
    ("he40.arm_m", offset_of!(Knobs, he40.arm_m), false),
    ("he40.standoff", offset_of!(Knobs, he40.standoff), false),
    ("he40.launch_noise", offset_of!(Knobs, he40.launch_noise), false),
    ("he40.dud_noise", offset_of!(Knobs, he40.dud_noise), false),
    ("he40.dud_hit_noise", offset_of!(Knobs, he40.dud_hit_noise), false),
    ("he40.prop_j", offset_of!(Knobs, he40.prop_j), false),
    ("he40.restitution", offset_of!(Knobs, he40.restitution), false),
    ("he40.keep_tangent", offset_of!(Knobs, he40.keep_tangent), false),
    ("he40.spent_speed_max", offset_of!(Knobs, he40.spent_speed_max), false),
    ("launcher40.rounds", offset_of!(Knobs, launcher40.rounds), false),
    ("launcher40.cd", offset_of!(Knobs, launcher40.cd), false),
    ("frag.issue", offset_of!(Knobs, frag.issue), false),
    ("frag.fuse", offset_of!(Knobs, frag.fuse), false),
    ("frag.order_fuse", offset_of!(Knobs, frag.order_fuse), false),
    ("frag.squad", offset_of!(Knobs, frag.squad), false),
    ("frag.order_in", offset_of!(Knobs, frag.order_in), false),
    ("frag.lethal_m", offset_of!(Knobs, frag.lethal_m), false),
    ("frag.wound_m", offset_of!(Knobs, frag.wound_m), false),
    ("frag.destr_m", offset_of!(Knobs, frag.destr_m), false),
    ("frag.wound_kill", offset_of!(Knobs, frag.wound_kill), false),
    ("frag.stagger.0", offset_of!(Knobs, frag.stagger), false),
    ("frag.stagger.1", offset_of!(Knobs, frag.stagger) + 8, false),
    ("frag.lift", offset_of!(Knobs, frag.lift), false),
    ("frag.glass_m", offset_of!(Knobs, frag.glass_m), false),
    ("frag.glass_power", offset_of!(Knobs, frag.glass_power), false),
    ("frag.pane_energy", offset_of!(Knobs, frag.pane_energy), false),
    ("frag.see_m", offset_of!(Knobs, frag.see_m), false),
    ("frag.hear_m", offset_of!(Knobs, frag.hear_m), false),
    ("frag.react.0", offset_of!(Knobs, frag.react), false),
    ("frag.react.1", offset_of!(Knobs, frag.react) + 8, false),
    ("frag.react_jit", offset_of!(Knobs, frag.react_jit), false),
    ("frag.shout_m", offset_of!(Knobs, frag.shout_m), false),
    ("frag.flee_m", offset_of!(Knobs, frag.flee_m), false),
    ("frag.run_speed", offset_of!(Knobs, frag.run_speed), false),
    ("frag.flee_keep", offset_of!(Knobs, frag.flee_keep), false),
    ("frag.flee_apart_m", offset_of!(Knobs, frag.flee_apart_m), false),
    ("frag.flee_lane_m", offset_of!(Knobs, frag.flee_lane_m), false),
    ("frag.down_shove", offset_of!(Knobs, frag.down_shove), false),
    ("frag.flee_door_open", offset_of!(Knobs, frag.flee_door_open), false),
    ("frag.flee_door_in", offset_of!(Knobs, frag.flee_door_in), false),
    ("frag.flee_door_dip", offset_of!(Knobs, frag.flee_door_dip), false),
    ("frag.flee_door_mouth", offset_of!(Knobs, frag.flee_door_mouth), false),
    ("frag.flee_door_leaf", offset_of!(Knobs, frag.flee_door_leaf), false),
    ("frag.flee_door_via", offset_of!(Knobs, frag.flee_door_via), false),
    ("frag.flee_door_via_m", offset_of!(Knobs, frag.flee_door_via_m), false),
    ("frag.flee_door_new", offset_of!(Knobs, frag.flee_door_new), false),
    ("frag.flee_door_most", offset_of!(Knobs, frag.flee_door_most), false),
    ("frag.flee_door_try", offset_of!(Knobs, frag.flee_door_try), false),
    ("frag.flee_door_nodes", offset_of!(Knobs, frag.flee_door_nodes), false),
    ("frag.def_share", offset_of!(Knobs, frag.def_share), false),
    ("frag.def_max", offset_of!(Knobs, frag.def_max), false),
    ("frag.def_fuse", offset_of!(Knobs, frag.def_fuse), false),
    ("frag.def_wind", offset_of!(Knobs, frag.def_wind), false),
    ("frag.def_near", offset_of!(Knobs, frag.def_near), false),
    ("frag.def_far", offset_of!(Knobs, frag.def_far), false),
    ("frag.def_past", offset_of!(Knobs, frag.def_past), false),
    ("frag.def_fresh", offset_of!(Knobs, frag.def_fresh), false),
    ("frag.def_blocked", offset_of!(Knobs, frag.def_blocked), false),
    ("frag.def_bunch", offset_of!(Knobs, frag.def_bunch), false),
    ("frag.def_bunch_secs", offset_of!(Knobs, frag.def_bunch_secs), false),
    ("frag.def_gap", offset_of!(Knobs, frag.def_gap), false),
    ("frag.def_own_m", offset_of!(Knobs, frag.def_own_m), false),
    ("frag.def_scatter.0", offset_of!(Knobs, frag.def_scatter), false),
    ("frag.def_scatter.1", offset_of!(Knobs, frag.def_scatter) + 8, false),
    ("frag.cue_m", offset_of!(Knobs, frag.cue_m), false),
    ("shotgun.clear", offset_of!(Knobs, shotgun.clear), false),
    ("shotgun.mark_reach", offset_of!(Knobs, shotgun.mark_reach), false),
    ("shotgun.line", offset_of!(Knobs, shotgun.line), false),
    ("shotgun.own_beat", offset_of!(Knobs, shotgun.own_beat), false),
    ("shotgun.own_slack", offset_of!(Knobs, shotgun.own_slack), false),
    ("shotgun.lock_in", offset_of!(Knobs, shotgun.lock_in), false),
    ("shotgun.lock_h", offset_of!(Knobs, shotgun.lock_h), false),
    ("shotgun.shells", offset_of!(Knobs, shotgun.shells), false),
    ("shotgun.aim_cap", offset_of!(Knobs, shotgun.aim_cap), false),
    ("glass.hear_m", offset_of!(Knobs, glass.hear_m), false),
    ("glass.hear_level", offset_of!(Knobs, glass.hear_level), false),
    ("glass.bang_m", offset_of!(Knobs, glass.bang_m), false),
    ("glass.bang_cracks", offset_of!(Knobs, glass.bang_cracks), false),
    ("glass.r_min", offset_of!(Knobs, glass.r_min), false),
    ("glass.r_max", offset_of!(Knobs, glass.r_max), false),
    ("glass.crack_gap", offset_of!(Knobs, glass.crack_gap), false),
    ("glass.live_gap", offset_of!(Knobs, glass.live_gap), false),
    ("glass.ear_off", offset_of!(Knobs, glass.ear_off), false),
    ("glassbreak.max_sites", offset_of!(Knobs, glassbreak.max_sites), false),
    ("glassbreak.break_e.annealed", offset_of!(Knobs, glassbreak.break_e.annealed), false),
    ("glassbreak.break_e.tempered", offset_of!(Knobs, glassbreak.break_e.tempered), false),
    ("glassbreak.punch_e", offset_of!(Knobs, glassbreak.punch_e), false),
    ("glassbreak.run_single_e", offset_of!(Knobs, glassbreak.run_single_e), false),
    ("glassbreak.run_star", offset_of!(Knobs, glassbreak.run_star), false),
    ("glassbreak.run_ring_secs", offset_of!(Knobs, glassbreak.run_ring_secs), false),
    ("glassbreak.crack_gap", offset_of!(Knobs, glassbreak.crack_gap), false),
    ("glassbreak.merge", offset_of!(Knobs, glassbreak.merge), false),
    ("glassbreak.rz_min", offset_of!(Knobs, glassbreak.rz_min), false),
    ("glassbreak.think", offset_of!(Knobs, glassbreak.think), false),
    ("glassbreak.stagger_per_m", offset_of!(Knobs, glassbreak.stagger_per_m), false),
    ("glassbreak.stagger_jit", offset_of!(Knobs, glassbreak.stagger_jit), false),
    ("glassbreak.hang_area", offset_of!(Knobs, glassbreak.hang_area), false),
    ("glassbreak.grip.side", offset_of!(Knobs, glassbreak.grip.side), false),
    ("glassbreak.grip.top", offset_of!(Knobs, glassbreak.grip.top), false),
    ("glassbreak.grip.wedged", offset_of!(Knobs, glassbreak.grip.wedged), false),
    ("glassbreak.stand_ny", offset_of!(Knobs, glassbreak.stand_ny), false),
    ("glassbreak.stand_foot", offset_of!(Knobs, glassbreak.stand_foot), false),
    ("glassbreak.balance_tol", offset_of!(Knobs, glassbreak.balance_tol), false),
    ("glassbreak.blast_m", offset_of!(Knobs, glassbreak.blast_m), false),
    ("glassbreak.bang_power", offset_of!(Knobs, glassbreak.bang_power), false),
    ("glassbreak.tooth_area", offset_of!(Knobs, glassbreak.tooth_area), false),
    ("glassbreak.tooth_k", offset_of!(Knobs, glassbreak.tooth_k), false),
    ("glassbreak.blast_run_k", offset_of!(Knobs, glassbreak.blast_run_k), false),
    ("glassbreak.blast_dice_k", offset_of!(Knobs, glassbreak.blast_dice_k), false),
    ("glassbreak.blast_sheet_k", offset_of!(Knobs, glassbreak.blast_sheet_k), false),
    ("glassbreak.rake_open", offset_of!(Knobs, glassbreak.rake_open), false),
    ("glassbreak.rake_m", offset_of!(Knobs, glassbreak.rake_m), false),
    ("glassbreak.dice_grain", offset_of!(Knobs, glassbreak.dice_grain), false),
    ("glassbreak.clump_m", offset_of!(Knobs, glassbreak.clump_m), false),
    ("glassbreak.dice_hang", offset_of!(Knobs, glassbreak.dice_hang), false),
    ("glassbreak.dice_speed", offset_of!(Knobs, glassbreak.dice_speed), false),
    ("glassbreak.clumps_per_frame", offset_of!(Knobs, glassbreak.clumps_per_frame), false),
    ("glassbreak.releases_per_frame", offset_of!(Knobs, glassbreak.releases_per_frame), false),
    ("glassbreak.out", offset_of!(Knobs, glassbreak.out), false),
    ("glassbreak.spin", offset_of!(Knobs, glassbreak.spin), false),
    ("glassbreak.blast_out", offset_of!(Knobs, glassbreak.blast_out), false),
    ("glassbreak.shard_albedo.0", offset_of!(Knobs, glassbreak.shard_albedo), false),
    ("glassbreak.shard_albedo.1", offset_of!(Knobs, glassbreak.shard_albedo) + 8, false),
    ("glassbreak.shard_albedo.2", offset_of!(Knobs, glassbreak.shard_albedo) + 16, false),
    ("glassbreak.chip_color.0", offset_of!(Knobs, glassbreak.chip_color), false),
    ("glassbreak.chip_color.1", offset_of!(Knobs, glassbreak.chip_color) + 8, false),
    ("glassbreak.chip_color.2", offset_of!(Knobs, glassbreak.chip_color) + 16, false),
    ("glassbreak.chips_per_m2", offset_of!(Knobs, glassbreak.chips_per_m2), false),
    ("glassbreak.chips_min", offset_of!(Knobs, glassbreak.chips_min), false),
    ("glassbreak.dice_chips_per_m2", offset_of!(Knobs, glassbreak.dice_chips_per_m2), false),
    ("glassbreak.crumb_every", offset_of!(Knobs, glassbreak.crumb_every), false),
    ("glassbreak.gust_speed_k", offset_of!(Knobs, glassbreak.gust_speed_k), false),
    ("glassbreak.walk_clear_h", offset_of!(Knobs, glassbreak.walk_clear_h), false),
    ("glasspen.default_energy", offset_of!(Knobs, glasspen.default_energy), false),
    ("glasspen.ref_thick", offset_of!(Knobs, glasspen.ref_thick), false),
    ("glasspen.oblique_max", offset_of!(Knobs, glasspen.oblique_max), false),
    ("glasspen.graze_cos", offset_of!(Knobs, glasspen.graze_cos), false),
    ("glasspen.min_energy", offset_of!(Knobs, glasspen.min_energy), false),
    ("glasspen.max_panes", offset_of!(Knobs, glasspen.max_panes), false),
    ("glasspen.deflect_deg", offset_of!(Knobs, glasspen.deflect_deg), false),
    ("glasspen.bend_k", offset_of!(Knobs, glasspen.bend_k), false),
    ("glasspen.hole_r", offset_of!(Knobs, glasspen.hole_r), false),
    ("glasspen.hole_k", offset_of!(Knobs, glasspen.hole_k), false),
    ("glasspen.holes", offset_of!(Knobs, glasspen.holes), false),
    ("glasspen.shell_r", offset_of!(Knobs, glasspen.shell_r), false),
    ("glasspen.stack_eyes", offset_of!(Knobs, glasspen.stack_eyes), false),
    ("glasspen.stack_energy", offset_of!(Knobs, glasspen.stack_energy), false),
    ("glasspen.stack_least_k", offset_of!(Knobs, glasspen.stack_least_k), false),
    ("glasspen.fracture_k", offset_of!(Knobs, glasspen.fracture_k), false),
    ("shards.mid_per_m2", offset_of!(Knobs, shards.mid_per_m2), false),
    ("shards.mid_min", offset_of!(Knobs, shards.mid_min), false),
    ("shards.mid_max", offset_of!(Knobs, shards.mid_max), false),
    ("shards.mid_size", offset_of!(Knobs, shards.mid_size), false),
    ("shards.mid_life", offset_of!(Knobs, shards.mid_life), false),
    ("shards.fine_per_m2", offset_of!(Knobs, shards.fine_per_m2), false),
    ("shards.fine_min", offset_of!(Knobs, shards.fine_min), false),
    ("shards.fine_max", offset_of!(Knobs, shards.fine_max), false),
    ("shards.fine_size", offset_of!(Knobs, shards.fine_size), false),
    ("shards.fine_life", offset_of!(Knobs, shards.fine_life), false),
    ("shards.back_share", offset_of!(Knobs, shards.back_share), false),
    ("shards.hit_fine_min", offset_of!(Knobs, shards.hit_fine_min), false),
    ("shards.hit_back_share", offset_of!(Knobs, shards.hit_back_share), false),
    ("shards.spall_out", offset_of!(Knobs, shards.spall_out), false),
    ("shards.spall_back", offset_of!(Knobs, shards.spall_back), false),
    ("shards.dice_per_m2", offset_of!(Knobs, shards.dice_per_m2), false),
    ("shards.dice_min", offset_of!(Knobs, shards.dice_min), false),
    ("shards.dice_max", offset_of!(Knobs, shards.dice_max), false),
    ("shards.dice_life", offset_of!(Knobs, shards.dice_life), false),
    ("shards.dice_fine_share", offset_of!(Knobs, shards.dice_fine_share), false),
    ("shards.crater_per_m2", offset_of!(Knobs, shards.crater_per_m2), false),
    ("shards.crater_max", offset_of!(Knobs, shards.crater_max), false),
    ("shards.exit_fines", offset_of!(Knobs, shards.exit_fines), false),
    ("shards.exit_speed", offset_of!(Knobs, shards.exit_speed), false),
    ("shards.speed", offset_of!(Knobs, shards.speed), false),
    ("shards.live_cap", offset_of!(Knobs, shards.live_cap), false),
    ("shards.cap_floor", offset_of!(Knobs, shards.cap_floor), false),
    ("shards.over_cap", offset_of!(Knobs, shards.over_cap), false),
    ("shards.press_life", offset_of!(Knobs, shards.press_life), false),
    ("shards.press_from", offset_of!(Knobs, shards.press_from), false),
    ("shards.recs_per_frame", offset_of!(Knobs, shards.recs_per_frame), false),
    ("shards.defer_max", offset_of!(Knobs, shards.defer_max), false),
    ("shards.plate_min_area", offset_of!(Knobs, shards.plate_min_area), false),
    ("shards.plate_share", offset_of!(Knobs, shards.plate_share), false),
    ("shards.child_min_area", offset_of!(Knobs, shards.child_min_area), false),
    ("shards.shatter_v0", offset_of!(Knobs, shards.shatter_v0), false),
    ("shards.shatter_v1", offset_of!(Knobs, shards.shatter_v1), false),
    ("shards.rest_crumbs", offset_of!(Knobs, shards.rest_crumbs), false),
    ("shards.crumb_size", offset_of!(Knobs, shards.crumb_size), false),
    ("shards.cut_edge", offset_of!(Knobs, shards.cut_edge), false),
    ("shards.cut_see", offset_of!(Knobs, shards.cut_see), false),
    ("shards.cut_rim", offset_of!(Knobs, shards.cut_rim), false),
    ("shards.cut_body", offset_of!(Knobs, shards.cut_body), false),
    ("shards.cut_see_rest", offset_of!(Knobs, shards.cut_see_rest), false),
    ("shards.cut_rest_body", offset_of!(Knobs, shards.cut_rest_body), false),
    ("shards.max_gen", offset_of!(Knobs, shards.max_gen), false),
    ("shards.contact.keep_floor", offset_of!(Knobs, shards.contact.keep_floor), false),
    ("shards.contact.keep_wall", offset_of!(Knobs, shards.contact.keep_wall), false),
    ("shards.contact.rest", offset_of!(Knobs, shards.contact.rest), false),
    ("shards.sounds_per_frame", offset_of!(Knobs, shards.sounds_per_frame), false),
    ("shards.pending_max", offset_of!(Knobs, shards.pending_max), false),
    ("shards.sound_thicken", offset_of!(Knobs, shards.sound_thicken), false),
    ("shards.sound_owed_secs", offset_of!(Knobs, shards.sound_owed_secs), false),
    ("shards.level", offset_of!(Knobs, shards.level), false),
    ("shards.event_gap", offset_of!(Knobs, shards.event_gap), false),
    ("shards.crunch_full", offset_of!(Knobs, shards.crunch_full), false),
    ("shards.crunch_gain", offset_of!(Knobs, shards.crunch_gain), false),
    ("shards.crunch_add", offset_of!(Knobs, shards.crunch_add), false),
    ("shards.grind", offset_of!(Knobs, shards.grind), false),
    ("hold.regrab", offset_of!(Knobs, hold.regrab), false),
    ("hold.bare_grip", offset_of!(Knobs, hold.bare_grip), false),
    ("hold.bare_arch", offset_of!(Knobs, hold.bare_arch), false),
    ("hold.bare_elbow", offset_of!(Knobs, hold.bare_elbow), false),
    ("hold.yank.sink", offset_of!(Knobs, hold.yank.sink), false),
    ("hold.yank.in_", offset_of!(Knobs, hold.yank.in_), false),
    ("hold.yank.hold", offset_of!(Knobs, hold.yank.hold), false),
    ("hold.yank.out", offset_of!(Knobs, hold.yank.out), false),
    ("hold.lunge.t.0", offset_of!(Knobs, hold.lunge.t), false),
    ("hold.lunge.t.1", offset_of!(Knobs, hold.lunge.t) + 8, false),
    ("hold.lunge.t.2", offset_of!(Knobs, hold.lunge.t) + 16, false),
    ("hold.sway.secs", offset_of!(Knobs, hold.sway.secs), false),
    ("hold.sway.his", offset_of!(Knobs, hold.sway.his), false),
    ("hold.sway.in_", offset_of!(Knobs, hold.sway.in_), false),
    ("hold.sway.out", offset_of!(Knobs, hold.sway.out), false),
    ("hold.catch.via.0", offset_of!(Knobs, hold.catch.via), false),
    ("hold.catch.via.1", offset_of!(Knobs, hold.catch.via) + 8, false),
    ("hold.catch.via.2", offset_of!(Knobs, hold.catch.via) + 16, false),
    ("hold.catch.over.0", offset_of!(Knobs, hold.catch.over), false),
    ("hold.catch.over.1", offset_of!(Knobs, hold.catch.over) + 8, false),
    ("hold.catch.over.2", offset_of!(Knobs, hold.catch.over) + 16, false),
    ("hold.catch.at.0", offset_of!(Knobs, hold.catch.at), false),
    ("hold.catch.at.1", offset_of!(Knobs, hold.catch.at) + 8, false),
    ("hold.catch.arm_in", offset_of!(Knobs, hold.catch.arm_in), false),
    ("hold.catch.ready_out.0", offset_of!(Knobs, hold.catch.ready_out), false),
    ("hold.catch.ready_out.1", offset_of!(Knobs, hold.catch.ready_out) + 8, false),
    ("hold.catch.jolt.t.0", offset_of!(Knobs, hold.catch.jolt.t), false),
    ("hold.catch.jolt.t.1", offset_of!(Knobs, hold.catch.jolt.t) + 8, false),
    ("hold.catch.jolt.t.2", offset_of!(Knobs, hold.catch.jolt.t) + 16, false),
    ("hold.catch.jolt.t.3", offset_of!(Knobs, hold.catch.jolt.t) + 24, false),
    ("hold.catch.jolt.fwd", offset_of!(Knobs, hold.catch.jolt.fwd), false),
    ("hold.catch.jolt.head", offset_of!(Knobs, hold.catch.jolt.head), false),
    ("hold.catch.jolt.clav", offset_of!(Knobs, hold.catch.jolt.clav), false),
    ("hold.catch.jolt.dip", offset_of!(Knobs, hold.catch.jolt.dip), false),
    ("hold.catch.fling.t.0", offset_of!(Knobs, hold.catch.fling.t), false),
    ("hold.catch.fling.t.1", offset_of!(Knobs, hold.catch.fling.t) + 8, false),
    ("hold.catch.fling.land.0", offset_of!(Knobs, hold.catch.fling.land), false),
    ("hold.catch.fling.land.1", offset_of!(Knobs, hold.catch.fling.land) + 8, false),
    ("hold.catch.fling.drift.0", offset_of!(Knobs, hold.catch.fling.drift), false),
    ("hold.catch.fling.drift.1", offset_of!(Knobs, hold.catch.fling.drift) + 8, false),
    ("hold.catch.fling.drift.2", offset_of!(Knobs, hold.catch.fling.drift) + 16, false),
    ("hold.catch.fling.arm_in.0", offset_of!(Knobs, hold.catch.fling.arm_in), false),
    ("hold.catch.fling.arm_in.1", offset_of!(Knobs, hold.catch.fling.arm_in) + 8, false),
    ("hold.catch.sink", offset_of!(Knobs, hold.catch.sink), false),
    ("hold.catch.hips", offset_of!(Knobs, hold.catch.hips), false),
    ("hold.catch.heels.0", offset_of!(Knobs, hold.catch.heels), false),
    ("hold.catch.heels.1", offset_of!(Knobs, hold.catch.heels) + 8, false),
    ("hold.catch.seat.0", offset_of!(Knobs, hold.catch.seat), false),
    ("hold.catch.seat.1", offset_of!(Knobs, hold.catch.seat) + 8, false),
    ("hold.catch.seat.2", offset_of!(Knobs, hold.catch.seat) + 16, false),
    ("hold.catch.seat_lean", offset_of!(Knobs, hold.catch.seat_lean), false),
    ("hold.catch.brace.0", offset_of!(Knobs, hold.catch.brace), false),
    ("hold.catch.brace.1", offset_of!(Knobs, hold.catch.brace) + 8, false),
    ("hold.catch.brace_lift", offset_of!(Knobs, hold.catch.brace_lift), false),
    ("hold.catch.yank_sit", offset_of!(Knobs, hold.catch.yank_sit), false),
    ("hold.catch.lift", offset_of!(Knobs, hold.catch.lift), false),
    ("hold.catch.sam_lift", offset_of!(Knobs, hold.catch.sam_lift), false),
    ("hold.catch.settle", offset_of!(Knobs, hold.catch.settle), false),
    ("hold.hit.t.0", offset_of!(Knobs, hold.hit.t), false),
    ("hold.hit.t.1", offset_of!(Knobs, hold.hit.t) + 8, false),
    ("hold.hit.t.2", offset_of!(Knobs, hold.hit.t) + 16, false),
    ("hold.hit.t.3", offset_of!(Knobs, hold.hit.t) + 24, false),
    ("hold.hit.hips", offset_of!(Knobs, hold.hit.hips), false),
    ("hold.hit.dip", offset_of!(Knobs, hold.hit.dip), false),
    ("hold.hit.head", offset_of!(Knobs, hold.hit.head), false),
    ("hold.hit.heave", offset_of!(Knobs, hold.hit.heave), false),
    ("hold.hit.sam_in", offset_of!(Knobs, hold.hit.sam_in), false),
    ("hold.hit.sam_dip", offset_of!(Knobs, hold.hit.sam_dip), false),
    ("hold.first.take", offset_of!(Knobs, hold.first.take), false),
    ("hold.first.drop", offset_of!(Knobs, hold.first.drop), false),
    ("hold.first.catch", offset_of!(Knobs, hold.first.catch), false),
    ("hold.strip.take", offset_of!(Knobs, hold.strip.take), false),
    ("hold.strip.out", offset_of!(Knobs, hold.strip.out), false),
    ("hold.strip.drop", offset_of!(Knobs, hold.strip.drop), false),
    ("hold.strip.follow", offset_of!(Knobs, hold.strip.follow), false),
    ("hold.strip.knife", offset_of!(Knobs, hold.strip.knife), false),
    ("hold.strip.home", offset_of!(Knobs, hold.strip.home), false),
    ("hold.strip.free", offset_of!(Knobs, hold.strip.free), false),
    ("hold.strip.up", offset_of!(Knobs, hold.strip.up), false),
    ("hold.strip.meet.0", offset_of!(Knobs, hold.strip.meet), false),
    ("hold.strip.meet.1", offset_of!(Knobs, hold.strip.meet) + 8, false),
    ("hold.strip.meet.2", offset_of!(Knobs, hold.strip.meet) + 16, false),
    ("hold.strip.torn.0", offset_of!(Knobs, hold.strip.torn), false),
    ("hold.strip.torn.1", offset_of!(Knobs, hold.strip.torn) + 8, false),
    ("hold.strip.torn.2", offset_of!(Knobs, hold.strip.torn) + 16, false),
    ("hold.strip.over.0", offset_of!(Knobs, hold.strip.over), false),
    ("hold.strip.over.1", offset_of!(Knobs, hold.strip.over) + 8, false),
    ("hold.strip.over.2", offset_of!(Knobs, hold.strip.over) + 16, false),
    ("hold.strip.wrench.0", offset_of!(Knobs, hold.strip.wrench), false),
    ("hold.strip.wrench.1", offset_of!(Knobs, hold.strip.wrench) + 8, false),
    ("hold.strip.wrench.2", offset_of!(Knobs, hold.strip.wrench) + 16, false),
    ("hold.strip.low.0", offset_of!(Knobs, hold.strip.low), false),
    ("hold.strip.low.1", offset_of!(Knobs, hold.strip.low) + 8, false),
    ("hold.strip.low.2", offset_of!(Knobs, hold.strip.low) + 16, false),
    ("hold.strip.yaw", offset_of!(Knobs, hold.strip.yaw), false),
    ("hold.strip.rap.0", offset_of!(Knobs, hold.strip.rap), false),
    ("hold.strip.rap.1", offset_of!(Knobs, hold.strip.rap) + 8, false),
    ("hold.strip.rap.2", offset_of!(Knobs, hold.strip.rap) + 16, false),
    ("hold.strip.hold.0", offset_of!(Knobs, hold.strip.hold), false),
    ("hold.strip.hold.1", offset_of!(Knobs, hold.strip.hold) + 8, false),
    ("hold.strip.pitch.0", offset_of!(Knobs, hold.strip.pitch), false),
    ("hold.strip.pitch.1", offset_of!(Knobs, hold.strip.pitch) + 8, false),
    ("hold.strip.pitch.2", offset_of!(Knobs, hold.strip.pitch) + 16, false),
    ("hold.strip.rest", offset_of!(Knobs, hold.strip.rest), false),
    ("hold.strip.push.0", offset_of!(Knobs, hold.strip.push), false),
    ("hold.strip.push.1", offset_of!(Knobs, hold.strip.push) + 8, false),
    ("hold.strip.fall", offset_of!(Knobs, hold.strip.fall), false),
    ("hold.strip.spin", offset_of!(Knobs, hold.strip.spin), false),
    ("hold.dist", offset_of!(Knobs, hold.dist), false),
    ("hold.secs.grab", offset_of!(Knobs, hold.secs.grab), false),
    ("hold.secs.choke_hold", offset_of!(Knobs, hold.secs.choke_hold), false),
    ("hold.secs.choke", offset_of!(Knobs, hold.secs.choke), false),
    ("hold.secs.whip", offset_of!(Knobs, hold.secs.whip), false),
    ("hold.secs.swap", offset_of!(Knobs, hold.secs.swap), false),
    ("hold.secs.kill", offset_of!(Knobs, hold.secs.kill), false),
    ("hold.throat.0", offset_of!(Knobs, hold.throat), false),
    ("hold.throat.1", offset_of!(Knobs, hold.throat) + 8, false),
    ("hold.throat.2", offset_of!(Knobs, hold.throat) + 16, false),
    ("hold.via_l.0", offset_of!(Knobs, hold.via_l), false),
    ("hold.via_l.1", offset_of!(Knobs, hold.via_l) + 8, false),
    ("hold.via_l.2", offset_of!(Knobs, hold.via_l) + 16, false),
    ("hold.over_l.0", offset_of!(Knobs, hold.over_l), false),
    ("hold.over_l.1", offset_of!(Knobs, hold.over_l) + 8, false),
    ("hold.over_l.2", offset_of!(Knobs, hold.over_l) + 16, false),
    ("hold.tight.0", offset_of!(Knobs, hold.tight), false),
    ("hold.tight.1", offset_of!(Knobs, hold.tight) + 8, false),
    ("hold.tight.2", offset_of!(Knobs, hold.tight) + 16, false),
    ("hold.lean", offset_of!(Knobs, hold.lean), false),
    ("hold.choke_lean", offset_of!(Knobs, hold.choke_lean), false),
    ("hold.head_pitch", offset_of!(Knobs, hold.head_pitch), false),
    ("hold.brace", offset_of!(Knobs, hold.brace), false),
    ("hold.knife.hand.0", offset_of!(Knobs, hold.knife.hand), false),
    ("hold.knife.hand.1", offset_of!(Knobs, hold.knife.hand) + 8, false),
    ("hold.knife.hand.2", offset_of!(Knobs, hold.knife.hand) + 16, false),
    ("hold.knife.blade.0", offset_of!(Knobs, hold.knife.blade), false),
    ("hold.knife.blade.1", offset_of!(Knobs, hold.knife.blade) + 8, false),
    ("hold.knife.blade.2", offset_of!(Knobs, hold.knife.blade) + 16, false),
    ("hold.knife.back.0", offset_of!(Knobs, hold.knife.back), false),
    ("hold.knife.back.1", offset_of!(Knobs, hold.knife.back) + 8, false),
    ("hold.knife.back.2", offset_of!(Knobs, hold.knife.back) + 16, false),
    ("hold.knife.via.0", offset_of!(Knobs, hold.knife.via), false),
    ("hold.knife.via.1", offset_of!(Knobs, hold.knife.via) + 8, false),
    ("hold.knife.via.2", offset_of!(Knobs, hold.knife.via) + 16, false),
    ("hold.knife.off_hand.0", offset_of!(Knobs, hold.knife.off_hand), false),
    ("hold.knife.off_hand.1", offset_of!(Knobs, hold.knife.off_hand) + 8, false),
    ("hold.knife.off_hand.2", offset_of!(Knobs, hold.knife.off_hand) + 16, false),
    ("hold.knife.off_blade.0", offset_of!(Knobs, hold.knife.off_blade), false),
    ("hold.knife.off_blade.1", offset_of!(Knobs, hold.knife.off_blade) + 8, false),
    ("hold.knife.off_blade.2", offset_of!(Knobs, hold.knife.off_blade) + 16, false),
    ("hold.knife.off_back.0", offset_of!(Knobs, hold.knife.off_back), false),
    ("hold.knife.off_back.1", offset_of!(Knobs, hold.knife.off_back) + 8, false),
    ("hold.knife.off_back.2", offset_of!(Knobs, hold.knife.off_back) + 16, false),
    ("hold.knife.off_secs", offset_of!(Knobs, hold.knife.off_secs), false),
    ("hold.knife.side", offset_of!(Knobs, hold.knife.side), false),
    ("hold.knife.head_yaw", offset_of!(Knobs, hold.knife.head_yaw), false),
    ("hold.knife.clav", offset_of!(Knobs, hold.knife.clav), false),
    ("hold.rest_l.0", offset_of!(Knobs, hold.rest_l), false),
    ("hold.rest_l.1", offset_of!(Knobs, hold.rest_l) + 8, false),
    ("hold.rest_l.2", offset_of!(Knobs, hold.rest_l) + 16, false),
    ("hold.claw_l.0", offset_of!(Knobs, hold.claw_l), false),
    ("hold.claw_l.1", offset_of!(Knobs, hold.claw_l) + 8, false),
    ("hold.claw_l.2", offset_of!(Knobs, hold.claw_l) + 16, false),
    ("hold.raised_r.0", offset_of!(Knobs, hold.raised_r), false),
    ("hold.raised_r.1", offset_of!(Knobs, hold.raised_r) + 8, false),
    ("hold.raised_r.2", offset_of!(Knobs, hold.raised_r) + 16, false),
    ("hold.via_up.0", offset_of!(Knobs, hold.via_up), false),
    ("hold.via_up.1", offset_of!(Knobs, hold.via_up) + 8, false),
    ("hold.via_up.2", offset_of!(Knobs, hold.via_up) + 16, false),
    ("hold.claw_r.0", offset_of!(Knobs, hold.claw_r), false),
    ("hold.claw_r.1", offset_of!(Knobs, hold.claw_r) + 8, false),
    ("hold.claw_r.2", offset_of!(Knobs, hold.claw_r) + 16, false),
    ("hold.arch", offset_of!(Knobs, hold.arch), false),
    ("hold.chin_up", offset_of!(Knobs, hold.chin_up), false),
    ("hold.aside", offset_of!(Knobs, hold.aside), false),
    ("hold.clav_up", offset_of!(Knobs, hold.clav_up), false),
    ("hold.loll", offset_of!(Knobs, hold.loll), false),
    ("hold.loll_roll", offset_of!(Knobs, hold.loll_roll), false),
    ("hold.lower.from", offset_of!(Knobs, hold.lower.from), false),
    ("hold.lower.by", offset_of!(Knobs, hold.lower.by), false),
    ("hold.lower.up", offset_of!(Knobs, hold.lower.up), false),
    ("hold.lower.sink", offset_of!(Knobs, hold.lower.sink), false),
    ("hold.lower.fwd", offset_of!(Knobs, hold.lower.fwd), false),
    ("hold.lower.hip", offset_of!(Knobs, hold.lower.hip), false),
    ("hold.lower.fold", offset_of!(Knobs, hold.lower.fold), false),
    ("hold.lower.hang.0", offset_of!(Knobs, hold.lower.hang), false),
    ("hold.lower.hang.1", offset_of!(Knobs, hold.lower.hang) + 8, false),
    ("hold.lower.sam_sink", offset_of!(Knobs, hold.lower.sam_sink), false),
    ("hold.lower.sam_fwd", offset_of!(Knobs, hold.lower.sam_fwd), false),
    ("hold.lower.sam_knees", offset_of!(Knobs, hold.lower.sam_knees), false),
    ("hold.lower.sam_lean", offset_of!(Knobs, hold.lower.sam_lean), false),
    ("hold.lower.sam_look", offset_of!(Knobs, hold.lower.sam_look), false),
    ("hold.lower.sam_arm.0", offset_of!(Knobs, hold.lower.sam_arm), false),
    ("hold.lower.sam_arm.1", offset_of!(Knobs, hold.lower.sam_arm) + 8, false),
    ("hold.kill.wind", offset_of!(Knobs, hold.kill.wind), false),
    ("hold.kill.hit", offset_of!(Knobs, hold.kill.hit), false),
    ("hold.kill.out", offset_of!(Knobs, hold.kill.out), false),
    ("hold.kill.go", offset_of!(Knobs, hold.kill.go), false),
    ("hold.kill.after", offset_of!(Knobs, hold.kill.after), false),
    ("hold.kill.mouth.0", offset_of!(Knobs, hold.kill.mouth), false),
    ("hold.kill.mouth.1", offset_of!(Knobs, hold.kill.mouth) + 8, false),
    ("hold.kill.mouth.2", offset_of!(Knobs, hold.kill.mouth) + 16, false),
    ("hold.kill.chest.0", offset_of!(Knobs, hold.kill.chest), false),
    ("hold.kill.chest.1", offset_of!(Knobs, hold.kill.chest) + 8, false),
    ("hold.kill.chest.2", offset_of!(Knobs, hold.kill.chest) + 16, false),
    ("hold.kill.collar.0", offset_of!(Knobs, hold.kill.collar), false),
    ("hold.kill.collar.1", offset_of!(Knobs, hold.kill.collar) + 8, false),
    ("hold.kill.collar.2", offset_of!(Knobs, hold.kill.collar) + 16, false),
    ("hold.kill.set.0", offset_of!(Knobs, hold.kill.set), false),
    ("hold.kill.set.1", offset_of!(Knobs, hold.kill.set) + 8, false),
    ("hold.kill.set.2", offset_of!(Knobs, hold.kill.set) + 16, false),
    ("hold.kill.across.0", offset_of!(Knobs, hold.kill.across), false),
    ("hold.kill.across.1", offset_of!(Knobs, hold.kill.across) + 8, false),
    ("hold.kill.across.2", offset_of!(Knobs, hold.kill.across) + 16, false),
    ("hold.kill.wide.0", offset_of!(Knobs, hold.kill.wide), false),
    ("hold.kill.wide.1", offset_of!(Knobs, hold.kill.wide) + 8, false),
    ("hold.kill.wide.2", offset_of!(Knobs, hold.kill.wide) + 16, false),
    ("hold.kill.low.0", offset_of!(Knobs, hold.kill.low), false),
    ("hold.kill.low.1", offset_of!(Knobs, hold.kill.low) + 8, false),
    ("hold.kill.low.2", offset_of!(Knobs, hold.kill.low) + 16, false),
    ("hold.kill.wide_blade.0", offset_of!(Knobs, hold.kill.wide_blade), false),
    ("hold.kill.wide_blade.1", offset_of!(Knobs, hold.kill.wide_blade) + 8, false),
    ("hold.kill.wide_blade.2", offset_of!(Knobs, hold.kill.wide_blade) + 16, false),
    ("hold.kill.wide_back.0", offset_of!(Knobs, hold.kill.wide_back), false),
    ("hold.kill.wide_back.1", offset_of!(Knobs, hold.kill.wide_back) + 8, false),
    ("hold.kill.wide_back.2", offset_of!(Knobs, hold.kill.wide_back) + 16, false),
    ("hold.kill.lean.0", offset_of!(Knobs, hold.kill.lean), false),
    ("hold.kill.lean.1", offset_of!(Knobs, hold.kill.lean) + 8, false),
    ("hold.kill.yaw.0", offset_of!(Knobs, hold.kill.yaw), false),
    ("hold.kill.yaw.1", offset_of!(Knobs, hold.kill.yaw) + 8, false),
    ("hold.kill.sam_sink", offset_of!(Knobs, hold.kill.sam_sink), false),
    ("hold.kill.look", offset_of!(Knobs, hold.kill.look), false),
    ("hold.kill.chin", offset_of!(Knobs, hold.kill.chin), false),
    ("hold.kill.arch", offset_of!(Knobs, hold.kill.arch), false),
    ("hold.kill.jolt.0", offset_of!(Knobs, hold.kill.jolt), false),
    ("hold.kill.jolt.1", offset_of!(Knobs, hold.kill.jolt) + 8, false),
    ("hold.kill.sink", offset_of!(Knobs, hold.kill.sink), false),
    ("hold.kill.fwd", offset_of!(Knobs, hold.kill.fwd), false),
    ("hold.kill.hip", offset_of!(Knobs, hold.kill.hip), false),
    ("hold.kill.fold", offset_of!(Knobs, hold.kill.fold), false),
    ("hold.kill.hang.0", offset_of!(Knobs, hold.kill.hang), false),
    ("hold.kill.hang.1", offset_of!(Knobs, hold.kill.hang) + 8, false),
    ("hold.beats.clamp.0", offset_of!(Knobs, hold.beats.clamp), false),
    ("hold.beats.clamp.1", offset_of!(Knobs, hold.beats.clamp) + 8, false),
    ("hold.beats.off.0", offset_of!(Knobs, hold.beats.off), false),
    ("hold.beats.off.1", offset_of!(Knobs, hold.beats.off) + 8, false),
    ("hold.beats.tip_by.0", offset_of!(Knobs, hold.beats.tip_by), false),
    ("hold.beats.tip_by.1", offset_of!(Knobs, hold.beats.tip_by) + 8, false),
    ("hold.beats.tip", offset_of!(Knobs, hold.beats.tip), false),
    ("hold.beats.back", offset_of!(Knobs, hold.beats.back), false),
    ("hold.beats.after", offset_of!(Knobs, hold.beats.after), false),
    ("hold.beats.lower.from", offset_of!(Knobs, hold.beats.lower.from), false),
    ("hold.beats.lower.by", offset_of!(Knobs, hold.beats.lower.by), false),
    ("hold.beats.lower.up", offset_of!(Knobs, hold.beats.lower.up), false),
    ("hold.beats.lower.sink", offset_of!(Knobs, hold.beats.lower.sink), false),
    ("hold.beats.lower.fwd", offset_of!(Knobs, hold.beats.lower.fwd), false),
    ("hold.beats.lower.hip", offset_of!(Knobs, hold.beats.lower.hip), false),
    ("hold.beats.lower.fold", offset_of!(Knobs, hold.beats.lower.fold), false),
    ("hold.beats.lower.hang.0", offset_of!(Knobs, hold.beats.lower.hang), false),
    ("hold.beats.lower.hang.1", offset_of!(Knobs, hold.beats.lower.hang) + 8, false),
    ("hold.beats.lower.sam_sink", offset_of!(Knobs, hold.beats.lower.sam_sink), false),
    ("hold.beats.lower.sam_fwd", offset_of!(Knobs, hold.beats.lower.sam_fwd), false),
    ("hold.beats.lower.sam_knees", offset_of!(Knobs, hold.beats.lower.sam_knees), false),
    ("hold.beats.lower.sam_lean", offset_of!(Knobs, hold.beats.lower.sam_lean), false),
    ("hold.beats.lower.sam_look", offset_of!(Knobs, hold.beats.lower.sam_look), false),
    ("hold.beats.lower.sam_arm.0", offset_of!(Knobs, hold.beats.lower.sam_arm), false),
    ("hold.beats.lower.sam_arm.1", offset_of!(Knobs, hold.beats.lower.sam_arm) + 8, false),
    ("hold.haul.in_.0", offset_of!(Knobs, hold.haul.in_), false),
    ("hold.haul.in_.1", offset_of!(Knobs, hold.haul.in_) + 8, false),
    ("hold.haul.out.0", offset_of!(Knobs, hold.haul.out), false),
    ("hold.haul.out.1", offset_of!(Knobs, hold.haul.out) + 8, false),
    ("hold.haul.down", offset_of!(Knobs, hold.haul.down), false),
    ("hold.haul.sam_lean", offset_of!(Knobs, hold.haul.sam_lean), false),
    ("hold.haul.sam_arm.0", offset_of!(Knobs, hold.haul.sam_arm), false),
    ("hold.haul.sam_arm.1", offset_of!(Knobs, hold.haul.sam_arm) + 8, false),
    ("hold.haul.lift", offset_of!(Knobs, hold.haul.lift), false),
    ("hold.haul.back", offset_of!(Knobs, hold.haul.back), false),
    ("hold.haul.recline", offset_of!(Knobs, hold.haul.recline), false),
    ("hold.haul.arch", offset_of!(Knobs, hold.haul.arch), false),
    ("hold.haul.chin", offset_of!(Knobs, hold.haul.chin), false),
    ("hold.haul.feet.0", offset_of!(Knobs, hold.haul.feet), false),
    ("hold.haul.feet.1", offset_of!(Knobs, hold.haul.feet) + 8, false),
    ("hold.haul.toes", offset_of!(Knobs, hold.haul.toes), false),
    ("hold.haul.kick.0", offset_of!(Knobs, hold.haul.kick), false),
    ("hold.haul.kick.1", offset_of!(Knobs, hold.haul.kick) + 8, false),
    ("hold.sag.0", offset_of!(Knobs, hold.sag), false),
    ("hold.sag.1", offset_of!(Knobs, hold.sag) + 8, false),
    ("hold.breath.hz", offset_of!(Knobs, hold.breath.hz), false),
    ("hold.breath.pitch", offset_of!(Knobs, hold.breath.pitch), false),
    ("hold.breath.clav", offset_of!(Knobs, hold.breath.clav), false),
    ("hold.struggle.rise", offset_of!(Knobs, hold.struggle.rise), false),
    ("hold.struggle.fade_from", offset_of!(Knobs, hold.struggle.fade_from), false),
    ("hold.struggle.fade_to", offset_of!(Knobs, hold.struggle.fade_to), false),
    ("hold.struggle.yaw", offset_of!(Knobs, hold.struggle.yaw), false),
    ("hold.struggle.roll", offset_of!(Knobs, hold.struggle.roll), false),
    ("hold.struggle.head", offset_of!(Knobs, hold.struggle.head), false),
    ("hold.struggle.claw", offset_of!(Knobs, hold.struggle.claw), false),
    ("hold.struggle.dip", offset_of!(Knobs, hold.struggle.dip), false),
    ("hold.struggle.hz.0", offset_of!(Knobs, hold.struggle.hz), false),
    ("hold.struggle.hz.1", offset_of!(Knobs, hold.struggle.hz) + 8, false),
    ("hold.struggle.hz.2", offset_of!(Knobs, hold.struggle.hz) + 16, false),
    ("hold.gun.hand.0", offset_of!(Knobs, hold.gun.hand), false),
    ("hold.gun.hand.1", offset_of!(Knobs, hold.gun.hand) + 8, false),
    ("hold.gun.hand.2", offset_of!(Knobs, hold.gun.hand) + 16, false),
    ("hold.gun.bore.0", offset_of!(Knobs, hold.gun.bore), false),
    ("hold.gun.bore.1", offset_of!(Knobs, hold.gun.bore) + 8, false),
    ("hold.gun.bore.2", offset_of!(Knobs, hold.gun.bore) + 16, false),
    ("hold.gun.out_hand.0", offset_of!(Knobs, hold.gun.out_hand), false),
    ("hold.gun.out_hand.1", offset_of!(Knobs, hold.gun.out_hand) + 8, false),
    ("hold.gun.out_hand.2", offset_of!(Knobs, hold.gun.out_hand) + 16, false),
    ("hold.gun.kick_pitch", offset_of!(Knobs, hold.gun.kick_pitch), false),
    ("hold.gun.kick_back", offset_of!(Knobs, hold.gun.kick_back), false),
    ("hold.gun.via_a.0", offset_of!(Knobs, hold.gun.via_a), false),
    ("hold.gun.via_a.1", offset_of!(Knobs, hold.gun.via_a) + 8, false),
    ("hold.gun.via_a.2", offset_of!(Knobs, hold.gun.via_a) + 16, false),
    ("hold.gun.via_b.0", offset_of!(Knobs, hold.gun.via_b), false),
    ("hold.gun.via_b.1", offset_of!(Knobs, hold.gun.via_b) + 8, false),
    ("hold.gun.via_b.2", offset_of!(Knobs, hold.gun.via_b) + 16, false),
    ("hold.gun.via_bore.0", offset_of!(Knobs, hold.gun.via_bore), false),
    ("hold.gun.via_bore.1", offset_of!(Knobs, hold.gun.via_bore) + 8, false),
    ("hold.gun.via_bore.2", offset_of!(Knobs, hold.gun.via_bore) + 16, false),
    ("hold.gun.wind_hand.0", offset_of!(Knobs, hold.gun.wind_hand), false),
    ("hold.gun.wind_hand.1", offset_of!(Knobs, hold.gun.wind_hand) + 8, false),
    ("hold.gun.wind_hand.2", offset_of!(Knobs, hold.gun.wind_hand) + 16, false),
    ("hold.gun.wind_bore.0", offset_of!(Knobs, hold.gun.wind_bore), false),
    ("hold.gun.wind_bore.1", offset_of!(Knobs, hold.gun.wind_bore) + 8, false),
    ("hold.gun.wind_bore.2", offset_of!(Knobs, hold.gun.wind_bore) + 16, false),
    ("hold.gun.hit_hand.0", offset_of!(Knobs, hold.gun.hit_hand), false),
    ("hold.gun.hit_hand.1", offset_of!(Knobs, hold.gun.hit_hand) + 8, false),
    ("hold.gun.hit_hand.2", offset_of!(Knobs, hold.gun.hit_hand) + 16, false),
    ("hold.gun.hit_bore.0", offset_of!(Knobs, hold.gun.hit_bore), false),
    ("hold.gun.hit_bore.1", offset_of!(Knobs, hold.gun.hit_bore) + 8, false),
    ("hold.gun.hit_bore.2", offset_of!(Knobs, hold.gun.hit_bore) + 16, false),
    ("hold.gun.thru_hand.0", offset_of!(Knobs, hold.gun.thru_hand), false),
    ("hold.gun.thru_hand.1", offset_of!(Knobs, hold.gun.thru_hand) + 8, false),
    ("hold.gun.thru_hand.2", offset_of!(Knobs, hold.gun.thru_hand) + 16, false),
    ("hold.gun.lean", offset_of!(Knobs, hold.gun.lean), false),
    ("hold.gun.side", offset_of!(Knobs, hold.gun.side), false),
    ("hold.gun.head_yaw", offset_of!(Knobs, hold.gun.head_yaw), false),
    ("hold.gun.head_pitch", offset_of!(Knobs, hold.gun.head_pitch), false),
    ("hold.gun.wind_yaw", offset_of!(Knobs, hold.gun.wind_yaw), false),
    ("hold.gun.hit_yaw", offset_of!(Knobs, hold.gun.hit_yaw), false),
    ("hold.gun.clav", offset_of!(Knobs, hold.gun.clav), false),
    ("hold.gun.aside", offset_of!(Knobs, hold.gun.aside), false),
    ("hold.gun.away_yaw", offset_of!(Knobs, hold.gun.away_yaw), false),
    ("hold.gun.chin_up", offset_of!(Knobs, hold.gun.chin_up), false),
    ("hold.gun.duck.yaw", offset_of!(Knobs, hold.gun.duck.yaw), false),
    ("hold.gun.duck.roll", offset_of!(Knobs, hold.gun.duck.roll), false),
    ("hold.gun.duck.pitch", offset_of!(Knobs, hold.gun.duck.pitch), false),
    ("hold.gun.duck.hunch", offset_of!(Knobs, hold.gun.duck.hunch), false),
    ("hold.gun.duck.secs", offset_of!(Knobs, hold.gun.duck.secs), false),
    ("hold.gun.knock.yaw", offset_of!(Knobs, hold.gun.knock.yaw), false),
    ("hold.gun.knock.roll", offset_of!(Knobs, hold.gun.knock.roll), false),
    ("hold.gun.knock.pitch", offset_of!(Knobs, hold.gun.knock.pitch), false),
    ("suppression.near_r", offset_of!(Knobs, suppression.near_r), false),
    ("suppression.w_near", offset_of!(Knobs, suppression.w_near), false),
    ("suppression.hit_r", offset_of!(Knobs, suppression.hit_r), false),
    ("suppression.w_hit", offset_of!(Knobs, suppression.w_hit), false),
    ("suppression.min_travel", offset_of!(Knobs, suppression.min_travel), false),
    ("suppression.half_life", offset_of!(Knobs, suppression.half_life), false),
    ("suppression.cap", offset_of!(Knobs, suppression.cap), false),
    ("suppression.kick_decay", offset_of!(Knobs, suppression.kick_decay), false),
    ("suppression.down_accrual", offset_of!(Knobs, suppression.down_accrual), false),
    ("suppression.friendly_k", offset_of!(Knobs, suppression.friendly_k), false),
    ("suppression.flinch", offset_of!(Knobs, suppression.flinch), false),
    ("suppression.pinned", offset_of!(Knobs, suppression.pinned), false),
    ("suppression.release", offset_of!(Knobs, suppression.release), false),
    ("suppression.head_down.0", offset_of!(Knobs, suppression.head_down), false),
    ("suppression.head_down.1", offset_of!(Knobs, suppression.head_down) + 8, false),
    ("suppression.flinch_delay", offset_of!(Knobs, suppression.flinch_delay), false),
    ("suppression.head_down_delay", offset_of!(Knobs, suppression.head_down_delay), false),
    ("suppression.guard_spread", offset_of!(Knobs, suppression.guard_spread), false),
    ("suppression.gap_k", offset_of!(Knobs, suppression.gap_k), false),
    ("suppression.standoff_pace_k", offset_of!(Knobs, suppression.standoff_pace_k), false),
    ("suppression.displace_after", offset_of!(Knobs, suppression.displace_after), false),
    ("suppression.displace_min", offset_of!(Knobs, suppression.displace_min), false),
    ("suppression.displace_max", offset_of!(Knobs, suppression.displace_max), false),
    ("suppression.cover_hold", offset_of!(Knobs, suppression.cover_hold), false),
    ("suppression.displace_cooldown", offset_of!(Knobs, suppression.displace_cooldown), false),
    ("suppression.displace_cap_secs", offset_of!(Knobs, suppression.displace_cap_secs), false),
    ("suppression.displace_pace", offset_of!(Knobs, suppression.displace_pace), false),
    ("suppression.displace_crouch", offset_of!(Knobs, suppression.displace_crouch), false),
    ("suppression.src_memory_secs", offset_of!(Knobs, suppression.src_memory_secs), false),
    ("suppression.bark_secs", offset_of!(Knobs, suppression.bark_secs), false),
    ("suppression.floor_bark_secs", offset_of!(Knobs, suppression.floor_bark_secs), false),
    ("suppression.pair_secs", offset_of!(Knobs, suppression.pair_secs), false),
    ("suppression.floor_line_secs", offset_of!(Knobs, suppression.floor_line_secs), false),
    ("suppression.sam_spread", offset_of!(Knobs, suppression.sam_spread), false),
    ("rifle_spread.still", offset_of!(Knobs, rifle_spread.still), false),
    ("rifle_spread.moving", offset_of!(Knobs, rifle_spread.moving), false),
    ("rifle_spread.move_at", offset_of!(Knobs, rifle_spread.move_at), false),
    ("rifle_spread.crouch_k", offset_of!(Knobs, rifle_spread.crouch_k), false),
    ("rifle_spread.brace_k", offset_of!(Knobs, rifle_spread.brace_k), false),
    ("footfall.travel", offset_of!(Knobs, footfall.travel), false),
    ("footfall.owe", offset_of!(Knobs, footfall.owe), false),
    ("twogears.crouch", offset_of!(Knobs, twogears.crouch), false),
    ("crouchrun.pace", offset_of!(Knobs, crouchrun.pace), false),
    ("crouchrun.loud", offset_of!(Knobs, crouchrun.loud), false),
    ("runready.fwd", offset_of!(Knobs, runready.fwd), false),
    ("runready.up", offset_of!(Knobs, runready.up), false),
    ("runready.pitch", offset_of!(Knobs, runready.pitch), false),
    ("runready.tuck", offset_of!(Knobs, runready.tuck), false),
    ("runarms.at", offset_of!(Knobs, runarms.at), false),
    ("runarms.keep", offset_of!(Knobs, runarms.keep), false),
    ("runarms.mid", offset_of!(Knobs, runarms.mid), false),
    ("runarms.swing", offset_of!(Knobs, runarms.swing), false),
    ("runarms.pump", offset_of!(Knobs, runarms.pump), false),
    ("runarms.lift", offset_of!(Knobs, runarms.lift), false),
    ("runarms.pole.0", offset_of!(Knobs, runarms.pole), false),
    ("runarms.pole.1", offset_of!(Knobs, runarms.pole) + 8, false),
    ("runarms.pole.2", offset_of!(Knobs, runarms.pole) + 16, false),
    ("runarms.pole_w", offset_of!(Knobs, runarms.pole_w), false),
    ("samlook.near_m", offset_of!(Knobs, samlook.near_m), false),
    ("samlook.full_m", offset_of!(Knobs, samlook.full_m), false),
    ("samlook.back_deg", offset_of!(Knobs, samlook.back_deg), false),
    ("samlook.keep_deg", offset_of!(Knobs, samlook.keep_deg), false),
    ("samlook.notice_m", offset_of!(Knobs, samlook.notice_m), false),
    ("samlook.swap_m", offset_of!(Knobs, samlook.swap_m), false),
    ("samlook.slip_from", offset_of!(Knobs, samlook.slip_from), false),
    ("samlook.slip_by", offset_of!(Knobs, samlook.slip_by), false),
    ("samlook.back_cos.0", offset_of!(Knobs, samlook.back_cos), false),
    ("samlook.back_cos.1", offset_of!(Knobs, samlook.back_cos) + 8, false),
    ("dash.stride", offset_of!(Knobs, dash.stride), false),
    ("dash.drop", offset_of!(Knobs, dash.drop), false),
    ("dash.lean", offset_of!(Knobs, dash.lean), false),
    ("dash.rate", offset_of!(Knobs, dash.rate), false),
    ("watch.yaw", offset_of!(Knobs, watch.yaw), false),
    ("watch.down", offset_of!(Knobs, watch.down), false),
    ("watch.up", offset_of!(Knobs, watch.up), false),
    ("watch.share.0", offset_of!(Knobs, watch.share), false),
    ("watch.share.1", offset_of!(Knobs, watch.share) + 8, false),
    ("watch.share.2", offset_of!(Knobs, watch.share) + 16, false),
    ("watch.rate", offset_of!(Knobs, watch.rate), false),
    ("watch.w_rate", offset_of!(Knobs, watch.w_rate), false),
    ("watch.lean", offset_of!(Knobs, watch.lean), false),
    ("watch.drop", offset_of!(Knobs, watch.drop), false),
    ("watch.away", offset_of!(Knobs, watch.away), false),
    ("watch.arm_out", offset_of!(Knobs, watch.arm_out), false),
    ("watch.arm_down", offset_of!(Knobs, watch.arm_down), false),
    ("watch.arm_w", offset_of!(Knobs, watch.arm_w), false),
    ("watch.arm_side.0", offset_of!(Knobs, watch.arm_side), false),
    ("watch.arm_side.1", offset_of!(Knobs, watch.arm_side) + 8, false),
    ("watch.slip_rate", offset_of!(Knobs, watch.slip_rate), false),
    ("watch.squeeze", offset_of!(Knobs, watch.squeeze), false),
    ("stalk.cursor_m", offset_of!(Knobs, stalk.cursor_m), false),
    ("stalk.begin_cos", offset_of!(Knobs, stalk.begin_cos), false),
    ("stalk.out_cos", offset_of!(Knobs, stalk.out_cos), false),
    ("stalk.station_m", offset_of!(Knobs, stalk.station_m), false),
    ("stalk.grab_m", offset_of!(Knobs, stalk.grab_m), false),
    ("stalk.pace", offset_of!(Knobs, stalk.pace), false),
    ("stalk.close_min", offset_of!(Knobs, stalk.close_min), false),
    ("stalk.close_k", offset_of!(Knobs, stalk.close_k), false),
    ("stalk.ease_up", offset_of!(Knobs, stalk.ease_up), false),
    ("stalk.ease_down", offset_of!(Knobs, stalk.ease_down), false),
    ("stalk.reach_from", offset_of!(Knobs, stalk.reach_from), false),
    ("stalk.reach", offset_of!(Knobs, stalk.reach), false),
    ("stalk.rise_from", offset_of!(Knobs, stalk.rise_from), false),
    ("stalk.rise_by", offset_of!(Knobs, stalk.rise_by), false),
    ("stalk.rise", offset_of!(Knobs, stalk.rise), false),
    ("stalk.catch_secs", offset_of!(Knobs, stalk.catch_secs), false),
    ("stalk.catch_m", offset_of!(Knobs, stalk.catch_m), false),
    ("stalk.catch_stride", offset_of!(Knobs, stalk.catch_stride), false),
    ("stalk.yank", offset_of!(Knobs, stalk.yank), false),
    ("stalk.yank_room", offset_of!(Knobs, stalk.yank_room), false),
    ("stalk.corner_m", offset_of!(Knobs, stalk.corner_m), false),
    ("stalk.corner_deg", offset_of!(Knobs, stalk.corner_deg), false),
    ("stalk.jolt_face", offset_of!(Knobs, stalk.jolt_face), false),
    ("stalk.jolt_secs", offset_of!(Knobs, stalk.jolt_secs), false),
    ("stalk.corner_see", offset_of!(Knobs, stalk.corner_see), false),
    ("stalk.corner_reach.0", offset_of!(Knobs, stalk.corner_reach), false),
    ("stalk.corner_reach.1", offset_of!(Knobs, stalk.corner_reach) + 8, false),
    ("stalk.corner_coil", offset_of!(Knobs, stalk.corner_coil), false),
    ("stalk.corner_rise", offset_of!(Knobs, stalk.corner_rise), false),
    ("stalk.key_m", offset_of!(Knobs, stalk.key_m), false),
    ("stalk.low_from", offset_of!(Knobs, stalk.low_from), false),
    ("stalk.low_by", offset_of!(Knobs, stalk.low_by), false),
    ("stalk.low_k", offset_of!(Knobs, stalk.low_k), false),
    ("stalk.low_drop", offset_of!(Knobs, stalk.low_drop), false),
    ("stalk.low_back", offset_of!(Knobs, stalk.low_back), false),
    ("stalk.low_lean", offset_of!(Knobs, stalk.low_lean), false),
    ("stalk.low_hands", offset_of!(Knobs, stalk.low_hands), false),
    ("stalk.low_spring", offset_of!(Knobs, stalk.low_spring), false),
    ("stalk.low_wind", offset_of!(Knobs, stalk.low_wind), false),
    ("stalk.wide_out", offset_of!(Knobs, stalk.wide_out), false),
    ("stalk.wide_down", offset_of!(Knobs, stalk.wide_down), false),
    ("stalk.wide_fwd", offset_of!(Knobs, stalk.wide_fwd), false),
    ("stalk.wide_lead", offset_of!(Knobs, stalk.wide_lead), false),
    ("stalk.wide_by", offset_of!(Knobs, stalk.wide_by), false),
    ("stalk.wide_pole.0", offset_of!(Knobs, stalk.wide_pole), false),
    ("stalk.wide_pole.1", offset_of!(Knobs, stalk.wide_pole) + 8, false),
    ("stalk.wide_pole.2", offset_of!(Knobs, stalk.wide_pole) + 16, false),
    ("stalk.heft_twist", offset_of!(Knobs, stalk.heft_twist), false),
    ("stalk.heft_swing", offset_of!(Knobs, stalk.heft_swing), false),
    ("stalk.heft_side", offset_of!(Knobs, stalk.heft_side), false),
    ("stalk.heft_lean", offset_of!(Knobs, stalk.heft_lean), false),
    ("stalk.heft_dip", offset_of!(Knobs, stalk.heft_dip), false),
    ("stalk.heft_surge", offset_of!(Knobs, stalk.heft_surge), false),
    ("stalk.heft_out", offset_of!(Knobs, stalk.heft_out), false),
    ("stalk.heft_down", offset_of!(Knobs, stalk.heft_down), false),
    ("stalk.heft_fwd", offset_of!(Knobs, stalk.heft_fwd), false),
    ("stalk.heft_lead", offset_of!(Knobs, stalk.heft_lead), false),
    ("stalk.heft_pole.0", offset_of!(Knobs, stalk.heft_pole), false),
    ("stalk.heft_pole.1", offset_of!(Knobs, stalk.heft_pole) + 8, false),
    ("stalk.heft_pole.2", offset_of!(Knobs, stalk.heft_pole) + 16, false),
    ("stalk.orbit_k", offset_of!(Knobs, stalk.orbit_k), false),
    ("stalk.orbit_edge", offset_of!(Knobs, stalk.orbit_edge), false),
    ("stalk.orbit_snap", offset_of!(Knobs, stalk.orbit_snap), false),
    ("stalk.low_begin_m", offset_of!(Knobs, stalk.low_begin_m), false),
    ("stalk.low_out_m", offset_of!(Knobs, stalk.low_out_m), false),
    ("overhear.path_m", offset_of!(Knobs, overhear.path_m), false),
    ("overhear.carry", offset_of!(Knobs, overhear.carry), false),
    ("score.seen", offset_of!(Knobs, score.seen), false),
    ("score.seen_up", offset_of!(Knobs, score.seen_up), false),
    ("score.alarm", offset_of!(Knobs, score.alarm), false),
    ("score.heightened", offset_of!(Knobs, score.heightened), false),
    ("score.lockdown", offset_of!(Knobs, score.lockdown), false),
    ("score.body", offset_of!(Knobs, score.body), false),
    ("score.kill", offset_of!(Knobs, score.kill), false),
    ("score.ko", offset_of!(Knobs, score.ko), false),
    ("score.shot_near", offset_of!(Knobs, score.shot_near), false),
    ("score.shot_near_cap", offset_of!(Knobs, score.shot_near_cap), false),
    ("score.light", offset_of!(Knobs, score.light), false),
    ("score.light_cap", offset_of!(Knobs, score.light_cap), false),
    ("score.hit", offset_of!(Knobs, score.hit), false),
    ("score.down", offset_of!(Knobs, score.down), false),
    ("score.load", offset_of!(Knobs, score.load), false),
    ("score.civ_reported", offset_of!(Knobs, score.civ_reported), false),
    ("score.civ_ko", offset_of!(Knobs, score.civ_ko), false),
    ("score.civ_killed", offset_of!(Knobs, score.civ_killed), false),
    ("score.device", offset_of!(Knobs, score.device), false),
    ("score.device_cap", offset_of!(Knobs, score.device_cap), false),
    ("score.par_secs", offset_of!(Knobs, score.par_secs), false),
    ("score.per_min", offset_of!(Knobs, score.per_min), false),
    ("score.time_cap", offset_of!(Knobs, score.time_cap), false),
    ("badge.lights_out_n", offset_of!(Knobs, badge.lights_out_n), false),
    ("badge.speedrun_secs", offset_of!(Knobs, badge.speedrun_secs), false),
    ("ckpt.quiet_secs", offset_of!(Knobs, ckpt.quiet_secs), false),
    ("ckpt.aware_max", offset_of!(Knobs, ckpt.aware_max), false),
    ("ckpt.debounce_secs", offset_of!(Knobs, ckpt.debounce_secs), false),
    ("ckpt.min_gap_secs", offset_of!(Knobs, ckpt.min_gap_secs), false),
    ("ckpt.pending_secs", offset_of!(Knobs, ckpt.pending_secs), false),
    ("ckpt.body_hide_m", offset_of!(Knobs, ckpt.body_hide_m), false),
    ("ckpt.auto_slots", offset_of!(Knobs, ckpt.auto_slots), false),
    ("ckpt.path_keep", offset_of!(Knobs, ckpt.path_keep), false),
    ("ckpt.flash_secs", offset_of!(Knobs, ckpt.flash_secs), false),
    ("ckpt.regret_secs", offset_of!(Knobs, ckpt.regret_secs), false),
    ("fog.dim", offset_of!(Knobs, fog.dim), false),
    ("fog.fov_deg", offset_of!(Knobs, fog.fov_deg), false),
    ("fog.near_m", offset_of!(Knobs, fog.near_m), false),
    ("fog.cone", offset_of!(Knobs, fog.cone), true),
    ("fog.cone_deg", offset_of!(Knobs, fog.cone_deg), false),
    ("fog.cone_near_m", offset_of!(Knobs, fog.cone_near_m), false),
    ("fog.cone_soft_deg", offset_of!(Knobs, fog.cone_soft_deg), false),
    ("fog.cone_near_soft_m", offset_of!(Knobs, fog.cone_near_soft_m), false),
    ("fog.own", offset_of!(Knobs, fog.own), true),
    ("fog.call_secs", offset_of!(Knobs, fog.call_secs), false),
    ("fog.call_gap", offset_of!(Knobs, fog.call_gap), false),
    ("fog.call_known", offset_of!(Knobs, fog.call_known), false),
    ("fog.smoke", offset_of!(Knobs, fog.smoke), false),
    ("fog.linger", offset_of!(Knobs, fog.linger), false),
    ("fog.top_pad", offset_of!(Knobs, fog.top_pad), false),
    ("fog.squad_doors_m", offset_of!(Knobs, fog.squad_doors_m), false),
    ("fog.squad_fire_secs", offset_of!(Knobs, fog.squad_fire_secs), false),
    ("fog.full_fill", offset_of!(Knobs, fog.full_fill), false),
    ("fog.lit_fill", offset_of!(Knobs, fog.lit_fill), false),
    ("fog.lit_ao", offset_of!(Knobs, fog.lit_ao), false),
    ("fog.lit_ao_m", offset_of!(Knobs, fog.lit_ao_m), false),
    ("fog.lit_cut", offset_of!(Knobs, fog.lit_cut), false),
    ("fog.last_seen", offset_of!(Knobs, fog.last_seen), false),
    ("moves.recoil", offset_of!(Knobs, moves.recoil), false),
    ("moves.toss", offset_of!(Knobs, moves.toss), false),
    ("moves.dazzle", offset_of!(Knobs, moves.dazzle), false),
    ("moves.flinch", offset_of!(Knobs, moves.flinch), false),
    ("moves.gait", offset_of!(Knobs, moves.gait), false),
    ("moves.fall", offset_of!(Knobs, moves.fall), false),
    ("moves.squeeze", offset_of!(Knobs, moves.squeeze), false),
    ("moves.reload", offset_of!(Knobs, moves.reload), false),
    ("moves.bound", offset_of!(Knobs, moves.bound), false),
    ("moves.clutch", offset_of!(Knobs, moves.clutch), false),
    ("moves.entry", offset_of!(Knobs, moves.entry), false),
    ("moves.entry_eyes", offset_of!(Knobs, moves.entry_eyes), false),
    ("moves.fold", offset_of!(Knobs, moves.fold), false),
    ("moves.snap", offset_of!(Knobs, moves.snap), false),
    ("moves.steady", offset_of!(Knobs, moves.steady), false),
    ("moves.steady_run", offset_of!(Knobs, moves.steady_run), false),
    ("mv.recoil.back.0", offset_of!(Knobs, mv.recoil.back), false),
    ("mv.recoil.back.1", offset_of!(Knobs, mv.recoil.back) + 8, false),
    ("mv.recoil.back_tau.0", offset_of!(Knobs, mv.recoil.back_tau), false),
    ("mv.recoil.back_tau.1", offset_of!(Knobs, mv.recoil.back_tau) + 8, false),
    ("mv.recoil.climb.0", offset_of!(Knobs, mv.recoil.climb), false),
    ("mv.recoil.climb.1", offset_of!(Knobs, mv.recoil.climb) + 8, false),
    ("mv.recoil.climb_tau.0", offset_of!(Knobs, mv.recoil.climb_tau), false),
    ("mv.recoil.climb_tau.1", offset_of!(Knobs, mv.recoil.climb_tau) + 8, false),
    ("mv.recoil.yaw.0", offset_of!(Knobs, mv.recoil.yaw), false),
    ("mv.recoil.yaw.1", offset_of!(Knobs, mv.recoil.yaw) + 8, false),
    ("mv.recoil.rock.0", offset_of!(Knobs, mv.recoil.rock), false),
    ("mv.recoil.rock.1", offset_of!(Knobs, mv.recoil.rock) + 8, false),
    ("mv.recoil.shoulder.0", offset_of!(Knobs, mv.recoil.shoulder), false),
    ("mv.recoil.shoulder.1", offset_of!(Knobs, mv.recoil.shoulder) + 8, false),
    ("mv.recoil.rock_tau.0", offset_of!(Knobs, mv.recoil.rock_tau), false),
    ("mv.recoil.rock_tau.1", offset_of!(Knobs, mv.recoil.rock_tau) + 8, false),
    ("mv.recoil.head.0", offset_of!(Knobs, mv.recoil.head), false),
    ("mv.recoil.head.1", offset_of!(Knobs, mv.recoil.head) + 8, false),
    ("mv.flinch.amp.0", offset_of!(Knobs, mv.flinch.amp), false),
    ("mv.flinch.amp.1", offset_of!(Knobs, mv.flinch.amp) + 8, false),
    ("mv.flinch.dur.0", offset_of!(Knobs, mv.flinch.dur), false),
    ("mv.flinch.dur.1", offset_of!(Knobs, mv.flinch.dur) + 8, false),
    ("mv.flinch.pitch", offset_of!(Knobs, mv.flinch.pitch), false),
    ("mv.flinch.head", offset_of!(Knobs, mv.flinch.head), false),
    ("mv.flinch.away", offset_of!(Knobs, mv.flinch.away), false),
    ("mv.flinch.drop", offset_of!(Knobs, mv.flinch.drop), false),
    ("mv.flinch.side", offset_of!(Knobs, mv.flinch.side), false),
    ("mv.hit.dur", offset_of!(Knobs, mv.hit.dur), false),
    ("mv.hit.pitch", offset_of!(Knobs, mv.hit.pitch), false),
    ("mv.hit.yaw", offset_of!(Knobs, mv.hit.yaw), false),
    ("mv.hit.roll", offset_of!(Knobs, mv.hit.roll), false),
    ("mv.hit.shove", offset_of!(Knobs, mv.hit.shove), false),
    ("mv.hit.drop", offset_of!(Knobs, mv.hit.drop), false),
    ("mv.dazzle.amp.0", offset_of!(Knobs, mv.dazzle.amp), false),
    ("mv.dazzle.amp.1", offset_of!(Knobs, mv.dazzle.amp) + 8, false),
    ("mv.dazzle.hunch", offset_of!(Knobs, mv.dazzle.hunch), false),
    ("mv.dazzle.away", offset_of!(Knobs, mv.dazzle.away), false),
    ("mv.dazzle.roll", offset_of!(Knobs, mv.dazzle.roll), false),
    ("mv.dazzle.head", offset_of!(Knobs, mv.dazzle.head), false),
    ("mv.dazzle.head_away", offset_of!(Knobs, mv.dazzle.head_away), false),
    ("mv.dazzle.drop", offset_of!(Knobs, mv.dazzle.drop), false),
    ("mv.dazzle.back", offset_of!(Knobs, mv.dazzle.back), false),
    ("mv.dazzle.sway_x", offset_of!(Knobs, mv.dazzle.sway_x), false),
    ("mv.dazzle.sway_z", offset_of!(Knobs, mv.dazzle.sway_z), false),
    ("mv.dazzle.rate_in", offset_of!(Knobs, mv.dazzle.rate_in), false),
    ("mv.dazzle.rate_out", offset_of!(Knobs, mv.dazzle.rate_out), false),
    ("mv.dazzle.hand_drop", offset_of!(Knobs, mv.dazzle.hand_drop), false),
    ("mv.dazzle.hand_out", offset_of!(Knobs, mv.dazzle.hand_out), false),
    ("mv.dazzle.hand_tilt", offset_of!(Knobs, mv.dazzle.hand_tilt), false),
    ("mv.dazzle.hand_roll", offset_of!(Knobs, mv.dazzle.hand_roll), false),
    ("mv.dazzle.hand_pole.0", offset_of!(Knobs, mv.dazzle.hand_pole), false),
    ("mv.dazzle.hand_pole.1", offset_of!(Knobs, mv.dazzle.hand_pole) + 8, false),
    ("mv.dazzle.hand_pole.2", offset_of!(Knobs, mv.dazzle.hand_pole) + 16, false),
    ("mv.gait.drop", offset_of!(Knobs, mv.gait.drop), false),
    ("mv.gait.aim_drop", offset_of!(Knobs, mv.gait.aim_drop), false),
    ("mv.gait.bob", offset_of!(Knobs, mv.gait.bob), false),
    ("mv.gait.lean", offset_of!(Knobs, mv.gait.lean), false),
    ("mv.gait.aim_lean", offset_of!(Knobs, mv.gait.aim_lean), false),
    ("mv.gait.elbow_in", offset_of!(Knobs, mv.gait.elbow_in), false),
    ("mv.gait.slouch_sway", offset_of!(Knobs, mv.gait.slouch_sway), false),
    ("mv.gait.slouch_roll", offset_of!(Knobs, mv.gait.slouch_roll), false),
    ("mv.gait.back", offset_of!(Knobs, mv.gait.back), false),
    ("mv.gait.chin", offset_of!(Knobs, mv.gait.chin), false),
    ("mv.gait.wing", offset_of!(Knobs, mv.gait.wing), false),
    ("mv.snap.in_", offset_of!(Knobs, mv.snap.in_), false),
    ("mv.snap.ease", offset_of!(Knobs, mv.snap.ease), false),
    ("mv.snap.cover.0", offset_of!(Knobs, mv.snap.cover), false),
    ("mv.snap.cover.1", offset_of!(Knobs, mv.snap.cover) + 8, false),
    ("mv.snap.under", offset_of!(Knobs, mv.snap.under), false),
    ("mv.steady.roll", offset_of!(Knobs, mv.steady.roll), false),
    ("mv.steady.sway", offset_of!(Knobs, mv.steady.sway), false),
    ("mv.steady.feet", offset_of!(Knobs, mv.steady.feet), false),
    ("mv.steady.knee", offset_of!(Knobs, mv.steady.knee), false),
    ("mv.steady.knee_low", offset_of!(Knobs, mv.steady.knee_low), false),
    ("mv.steady.knee_out", offset_of!(Knobs, mv.steady.knee_out), false),
    ("mv.steady.rise", offset_of!(Knobs, mv.steady.rise), false),
    ("mv.steady.reach", offset_of!(Knobs, mv.steady.reach), false),
    ("mv.steady.rise_rate", offset_of!(Knobs, mv.steady.rise_rate), false),
    ("turn.body_share", offset_of!(Knobs, turn.body_share), false),
    ("turn.green", offset_of!(Knobs, turn.green), false),
    ("turn.lead", offset_of!(Knobs, turn.lead), false),
    ("turn.lead_rate", offset_of!(Knobs, turn.lead_rate), false),
    ("turn.jump", offset_of!(Knobs, turn.jump), false),
    ("turn.aim", offset_of!(Knobs, turn.aim), false),
    ("turn.aim_acc", offset_of!(Knobs, turn.aim_acc), false),
    ("turn.body", offset_of!(Knobs, turn.body), false),
    ("turn.body_acc", offset_of!(Knobs, turn.body_acc), false),
    ("wound.centre", offset_of!(Knobs, wound.centre), false),
    ("wound.head", offset_of!(Knobs, wound.head), false),
    ("wound.lead", offset_of!(Knobs, wound.lead), false),
    ("wound.depth", offset_of!(Knobs, wound.depth), false),
    ("wound.back_room", offset_of!(Knobs, wound.back_room), false),
    ("doorpath.at", offset_of!(Knobs, doorpath.at), false),
    ("doorpath.out_m", offset_of!(Knobs, doorpath.out_m), false),
    ("doorpath.in_m", offset_of!(Knobs, doorpath.in_m), false),
    ("doorpath.look", offset_of!(Knobs, doorpath.look), false),
    ("doorpath.leaf_clear", offset_of!(Knobs, doorpath.leaf_clear), false),
    ("doorpath.tip_m", offset_of!(Knobs, doorpath.tip_m), false),
    ("doorpath.gap_m", offset_of!(Knobs, doorpath.gap_m), false),
    ("doorpath.short_m", offset_of!(Knobs, doorpath.short_m), false),
    ("doorpath.cap", offset_of!(Knobs, doorpath.cap), false),
    ("doorpath.move_m", offset_of!(Knobs, doorpath.move_m), false),
    ("doorpath.apart_m", offset_of!(Knobs, doorpath.apart_m), false),
    ("doorpath.body_m", offset_of!(Knobs, doorpath.body_m), false),
    ("doorpath.body_off", offset_of!(Knobs, doorpath.body_off), false),
    ("doorpath.body_move", offset_of!(Knobs, doorpath.body_move), false),
    ("walls.wall_max_thick", offset_of!(Knobs, walls.wall_max_thick), false),
    ("walls.wall_min_top", offset_of!(Knobs, walls.wall_min_top), false),
    ("walls.wall_min_len", offset_of!(Knobs, walls.wall_min_len), false),
    ("walls.col", offset_of!(Knobs, walls.col), false),
    ("walls.sill", offset_of!(Knobs, walls.sill), false),
    ("walls.sill_k", offset_of!(Knobs, walls.sill_k), false),
    ("walls.pistol_k", offset_of!(Knobs, walls.pistol_k), false),
    ("walls.cover_min_top", offset_of!(Knobs, walls.cover_min_top), false),
    ("walls.cover_min_len", offset_of!(Knobs, walls.cover_min_len), false),
    ("walls.cover_max_cells", offset_of!(Knobs, walls.cover_max_cells), false),
    ("walls.blast.radius", offset_of!(Knobs, walls.blast.radius), false),
    ("walls.blast.hole_k", offset_of!(Knobs, walls.blast.hole_k), false),
    ("walls.blast.port_k", offset_of!(Knobs, walls.blast.port_k), false),
    ("walls.blast.cover_k", offset_of!(Knobs, walls.blast.cover_k), false),
    ("walls.blast.door_k", offset_of!(Knobs, walls.blast.door_k), false),
    ("walls.blast.min_hole", offset_of!(Knobs, walls.blast.min_hole), false),
    ("walls.blast.contact_m", offset_of!(Knobs, walls.blast.contact_m), false),
    ("walls.max_pieces", offset_of!(Knobs, walls.max_pieces), false),
    ("walls.max_changes", offset_of!(Knobs, walls.max_changes), false),
    ("walls.chunks_per_cell", offset_of!(Knobs, walls.chunks_per_cell), false),
    ("walls.chunks_per_blast", offset_of!(Knobs, walls.chunks_per_blast), false),
    ("walls.fall_max_vol", offset_of!(Knobs, walls.fall_max_vol), false),
    ("walls.falls_per_change", offset_of!(Knobs, walls.falls_per_change), false),
    ("walls.hung.deep", offset_of!(Knobs, walls.hung.deep), false),
    ("walls.hung.long", offset_of!(Knobs, walls.hung.long), false),
    ("walls.hung.tall", offset_of!(Knobs, walls.hung.tall), false),
    ("wallcharge.pick_m", offset_of!(Knobs, wallcharge.pick_m), false),
    ("wallcharge.height", offset_of!(Knobs, wallcharge.height), false),
    ("wallcharge.end_m", offset_of!(Knobs, wallcharge.end_m), false),
    ("wallcharge.set_off", offset_of!(Knobs, wallcharge.set_off), false),
    ("wallcharge.set_near", offset_of!(Knobs, wallcharge.set_near), false),
    ("wallcharge.sam_reach", offset_of!(Knobs, wallcharge.sam_reach), false),
    ("wallcharge.sam_shift", offset_of!(Knobs, wallcharge.sam_shift), false),
    ("wallcharge.sam_marker_m", offset_of!(Knobs, wallcharge.sam_marker_m), false),
    ("wallcharge.post_off", offset_of!(Knobs, wallcharge.post_off), false),
    ("wallcharge.file_gap", offset_of!(Knobs, wallcharge.file_gap), false),
    ("wallcharge.clear_m", offset_of!(Knobs, wallcharge.clear_m), false),
    ("wallcharge.mate_m", offset_of!(Knobs, wallcharge.mate_m), false),
    ("wallcharge.after_blow", offset_of!(Knobs, wallcharge.after_blow), false),
    ("wallcharge.enter_cap", offset_of!(Knobs, wallcharge.enter_cap), false),
    ("wallcharge.sweep_secs", offset_of!(Knobs, wallcharge.sweep_secs), false),
    ("wallcharge.mouth_in", offset_of!(Knobs, wallcharge.mouth_in), false),
    ("wallcharge.lane", offset_of!(Knobs, wallcharge.lane), false),
    ("wallcharge.lane_watch", offset_of!(Knobs, wallcharge.lane_watch), false),
    ("wallcharge.cost", offset_of!(Knobs, wallcharge.cost), false),
    ("wallcharge.hold_secs", offset_of!(Knobs, wallcharge.hold_secs), false),
    ("wallcharge.mate_line", offset_of!(Knobs, wallcharge.mate_line), false),
    ("wallcharge.off_frame", offset_of!(Knobs, wallcharge.off_frame), false),
    ("wallcharge.off_frame_pod", offset_of!(Knobs, wallcharge.off_frame_pod), false),
    ("wallcharge.outer_thick", offset_of!(Knobs, wallcharge.outer_thick), false),
    ("wallcharge.outer_min_len", offset_of!(Knobs, wallcharge.outer_min_len), false),
    ("wallcharge.far_slack", offset_of!(Knobs, wallcharge.far_slack), false),
    ("wallcharge.far_reach", offset_of!(Knobs, wallcharge.far_reach), false),
    ("wallcharge.hung_m", offset_of!(Knobs, wallcharge.hung_m), false),
    ("wallcharge.safe_standoff.0", offset_of!(Knobs, wallcharge.safe_standoff), false),
    ("wallcharge.safe_standoff.1", offset_of!(Knobs, wallcharge.safe_standoff) + 8, false),
    ("wallcharge.safe_reach.0", offset_of!(Knobs, wallcharge.safe_reach), false),
    ("wallcharge.safe_reach.1", offset_of!(Knobs, wallcharge.safe_reach) + 8, false),
    ("wallcharge.safe_secs.0", offset_of!(Knobs, wallcharge.safe_secs), false),
    ("wallcharge.safe_secs.1", offset_of!(Knobs, wallcharge.safe_secs) + 8, false),
    ("wallcharge.safe_hide_m", offset_of!(Knobs, wallcharge.safe_hide_m), false),
    ("wallcharge.safe_walk_m", offset_of!(Knobs, wallcharge.safe_walk_m), false),
    ("wallcharge.safe_back_m", offset_of!(Knobs, wallcharge.safe_back_m), false),
    ("wallcharge.fire_known_s", offset_of!(Knobs, wallcharge.fire_known_s), false),
    ("wallcharge.fire_m", offset_of!(Knobs, wallcharge.fire_m), false),
    ("wallcharge.fire_step", offset_of!(Knobs, wallcharge.fire_step), false),
    ("wallcharge.fire_grain", offset_of!(Knobs, wallcharge.fire_grain), false),
    ("wallcharge.men_clear_in", offset_of!(Knobs, wallcharge.men_clear_in), false),
    ("wallcharge.men_clear_s", offset_of!(Knobs, wallcharge.men_clear_s), false),
    ("wallcharge.sam_secs.0", offset_of!(Knobs, wallcharge.sam_secs), false),
    ("wallcharge.sam_secs.1", offset_of!(Knobs, wallcharge.sam_secs) + 8, false),
    ("wallcharge.sam_sup", offset_of!(Knobs, wallcharge.sam_sup), false),
    ("wallshot.pall", offset_of!(Knobs, wallshot.pall), false),
    ("wallshot.stagger_secs", offset_of!(Knobs, wallshot.stagger_secs), false),
    ("wallshot.stun_secs.0", offset_of!(Knobs, wallshot.stun_secs), false),
    ("wallshot.stun_secs.1", offset_of!(Knobs, wallshot.stun_secs) + 8, false),
    ("wallshot.room_m", offset_of!(Knobs, wallshot.room_m), false),
    ("wallshot.ours_secs", offset_of!(Knobs, wallshot.ours_secs), false),
    ("wallkill.near_m", offset_of!(Knobs, wallkill.near_m), false),
    ("hardbreath.chest_amp", offset_of!(Knobs, hardbreath.chest_amp), false),
    ("hardbreath.chest_rate", offset_of!(Knobs, hardbreath.chest_rate), false),
    ("hardbreath.chest_up", offset_of!(Knobs, hardbreath.chest_up), false),
    ("hardbreath.chest_down", offset_of!(Knobs, hardbreath.chest_down), false),
    ("hardbreath.every", offset_of!(Knobs, hardbreath.every), false),
    ("hardbreath.puffs", offset_of!(Knobs, hardbreath.puffs), false),
    ("hardbreath.count", offset_of!(Knobs, hardbreath.count), false),
    ("hardbreath.jet", offset_of!(Knobs, hardbreath.jet), false),
    ("hardbreath.size", offset_of!(Knobs, hardbreath.size), false),
    ("hardbreath.carry", offset_of!(Knobs, hardbreath.carry), false),
    ("hardbreath.up", offset_of!(Knobs, hardbreath.up), false),
    ("hardbreath.down", offset_of!(Knobs, hardbreath.down), false),
    ("castor.decel", offset_of!(Knobs, castor.decel), false),
    ("castor.visc", offset_of!(Knobs, castor.visc), false),
    ("castor.spin", offset_of!(Knobs, castor.spin), false),
    ("castor.start_v", offset_of!(Knobs, castor.start_v), false),
    ("castor.stop_v", offset_of!(Knobs, castor.stop_v), false),
    ("wallreact.see_m", offset_of!(Knobs, wallreact.see_m), false),
    ("wallreact.fov_deg", offset_of!(Knobs, wallreact.fov_deg), false),
    ("wallreact.hear_blast_m", offset_of!(Knobs, wallreact.hear_blast_m), false),
    ("wallreact.hear_fire_m", offset_of!(Knobs, wallreact.hear_fire_m), false),
    ("wallreact.hear_fall_m", offset_of!(Knobs, wallreact.hear_fall_m), false),
    ("wallreact.tell_m", offset_of!(Knobs, wallreact.tell_m), false),
    ("wallreact.tell_secs", offset_of!(Knobs, wallreact.tell_secs), false),
    ("wallreact.near_m", offset_of!(Knobs, wallreact.near_m), false),
    ("wallreact.retry_secs", offset_of!(Knobs, wallreact.retry_secs), false),
    ("wallreact.gone_max", offset_of!(Knobs, wallreact.gone_max), false),
    ("wallreact.step_m", offset_of!(Knobs, wallreact.step_m), false),
    ("wallreact.run_m", offset_of!(Knobs, wallreact.run_m), false),
    ("wallreact.beyond_m", offset_of!(Knobs, wallreact.beyond_m), false),
    ("wallreact.ev_open", offset_of!(Knobs, wallreact.ev_open), false),
    ("wallreact.ev_seen", offset_of!(Knobs, wallreact.ev_seen), false),
    ("wallreact.ev_fire", offset_of!(Knobs, wallreact.ev_fire), false),
    ("wallreact.forget", offset_of!(Knobs, wallreact.forget), false),
    ("wallreact.hole_base", offset_of!(Knobs, wallreact.hole_base), false),
    ("wallreact.port_base", offset_of!(Knobs, wallreact.port_base), false),
    ("wallreact.crowd", offset_of!(Knobs, wallreact.crowd), false),
    ("wallreact.sticky", offset_of!(Knobs, wallreact.sticky), false),
    ("wallreact.fresh_secs", offset_of!(Knobs, wallreact.fresh_secs), false),
    ("wallreact.fresh", offset_of!(Knobs, wallreact.fresh), false),
    ("wallreact.watch_m", offset_of!(Knobs, wallreact.watch_m), false),
    ("coverfire.stale_secs", offset_of!(Knobs, coverfire.stale_secs), false),
    ("coverfire.call_secs", offset_of!(Knobs, coverfire.call_secs), false),
    ("coverfire.call_m", offset_of!(Knobs, coverfire.call_m), false),
    ("coverfire.max_layers", offset_of!(Knobs, coverfire.max_layers), false),
    ("coverfire.max_rounds", offset_of!(Knobs, coverfire.max_rounds), false),
    ("coverfire.max_m", offset_of!(Knobs, coverfire.max_m), false),
    ("coverfire.min_m", offset_of!(Knobs, coverfire.min_m), false),
    ("coverfire.near_m", offset_of!(Knobs, coverfire.near_m), false),
    ("coverfire.slack_k", offset_of!(Knobs, coverfire.slack_k), false),
    ("coverfire.slack", offset_of!(Knobs, coverfire.slack), false),
    ("coverfire.give_secs", offset_of!(Knobs, coverfire.give_secs), false),
    ("coverfire.lane", offset_of!(Knobs, coverfire.lane), false),
    ("coverfire.cone_k", offset_of!(Knobs, coverfire.cone_k), false),
    ("coverfire.past_m", offset_of!(Knobs, coverfire.past_m), false),
    ("coverfire.same_m", offset_of!(Knobs, coverfire.same_m), false),
    ("coverfire.linger_secs", offset_of!(Knobs, coverfire.linger_secs), false),
    ("coverfire.lead_secs", offset_of!(Knobs, coverfire.lead_secs), false),
    ("coverfire.lead_m", offset_of!(Knobs, coverfire.lead_m), false),
    ("coverfire.low_y", offset_of!(Knobs, coverfire.low_y), false),
    ("coverfire.round_secs", offset_of!(Knobs, coverfire.round_secs), false),
    ("coverfire.walk_pace", offset_of!(Knobs, coverfire.walk_pace), false),
    ("coverfire.walk_ask_secs", offset_of!(Knobs, coverfire.walk_ask_secs), false),
    ("coverfire.say_secs", offset_of!(Knobs, coverfire.say_secs), false),
    ("doorblast.tear_m.stun", offset_of!(Knobs, doorblast.tear_m.stun), false),
    ("doorblast.tear_m.frag", offset_of!(Knobs, doorblast.tear_m.frag), false),
    ("doorblast.charge_k", offset_of!(Knobs, doorblast.charge_k), false),
    ("doorblast.charge_r", offset_of!(Knobs, doorblast.charge_r), false),
    ("doorblast.speed.0", offset_of!(Knobs, doorblast.speed), false),
    ("doorblast.speed.1", offset_of!(Knobs, doorblast.speed) + 8, false),
    ("doorblast.up", offset_of!(Knobs, doorblast.up), false),
    ("doorblast.tip", offset_of!(Knobs, doorblast.tip), false),
    ("doorblast.spin", offset_of!(Knobs, doorblast.spin), false),
    ("doorblast.scrape", offset_of!(Knobs, doorblast.scrape), false),
    ("doorblast.slam", offset_of!(Knobs, doorblast.slam), false),
    ("doorblast.slide", offset_of!(Knobs, doorblast.slide), false),
    ("doorblast.bounce", offset_of!(Knobs, doorblast.bounce), false),
    ("doorblast.room", offset_of!(Knobs, doorblast.room), false),
    ("doorblast.land_level", offset_of!(Knobs, doorblast.land_level), false),
    ("doorshot.toll", offset_of!(Knobs, doorshot.toll), false),
    ("doorshot.strips", offset_of!(Knobs, doorshot.strips), false),
    ("doorshot.lengths", offset_of!(Knobs, doorshot.lengths), false),
    ("doorshot.uneven", offset_of!(Knobs, doorshot.uneven), false),
    ("doorshot.chunks", offset_of!(Knobs, doorshot.chunks), false),
    ("doorshot.side", offset_of!(Knobs, doorshot.side), false),
    ("doorshot.spin", offset_of!(Knobs, doorshot.spin), false),
    ("doorshot.shed", offset_of!(Knobs, doorshot.shed), false),
    ("doorshot.level", offset_of!(Knobs, doorshot.level), false),
    ("doorthrow.push_secs", offset_of!(Knobs, doorthrow.push_secs), false),
    ("doorthrow.kick_secs", offset_of!(Knobs, doorthrow.kick_secs), false),
    ("doorthrow.startle", offset_of!(Knobs, doorthrow.startle), false),
    ("doorthrow.wary", offset_of!(Knobs, doorthrow.wary), false),
    ("doorthrow.min_speed", offset_of!(Knobs, doorthrow.min_speed), false),
    ("doorthrow.hand_rate", offset_of!(Knobs, doorthrow.hand_rate), false),
    ("doorthrow.floor_rad", offset_of!(Knobs, doorthrow.floor_rad), false),
    ("doorthrow.rate", offset_of!(Knobs, doorthrow.rate), false),
    ("doorthrow.slide", offset_of!(Knobs, doorthrow.slide), false),
    ("doorthrow.again", offset_of!(Knobs, doorthrow.again), false),
    ("doorthrow.aware", offset_of!(Knobs, doorthrow.aware), false),
    ("winbars.per_round", offset_of!(Knobs, winbars.per_round), false),
    ("winbars.pieces", offset_of!(Knobs, winbars.pieces), false),
    ("winbars.uneven", offset_of!(Knobs, winbars.uneven), false),
    ("winbars.fly.0", offset_of!(Knobs, winbars.fly), false),
    ("winbars.fly.1", offset_of!(Knobs, winbars.fly) + 8, false),
    ("winbars.rest_k", offset_of!(Knobs, winbars.rest_k), false),
    ("winbars.up.0", offset_of!(Knobs, winbars.up), false),
    ("winbars.up.1", offset_of!(Knobs, winbars.up) + 8, false),
    ("winbars.side", offset_of!(Knobs, winbars.side), false),
    ("winbars.spin", offset_of!(Knobs, winbars.spin), false),
    ("winbars.chips", offset_of!(Knobs, winbars.chips), false),
    ("winbars.chip_v", offset_of!(Knobs, winbars.chip_v), false),
    ("winbars.blast_k", offset_of!(Knobs, winbars.blast_k), false),
    ("opening.clear_m", offset_of!(Knobs, opening.clear_m), false),
    ("opening.step_m", offset_of!(Knobs, opening.step_m), false),
    ("handle.look", offset_of!(Knobs, handle.look), false),
    ("handle.ajar", offset_of!(Knobs, handle.ajar), false),
    ("handle.reach", offset_of!(Knobs, handle.reach), false),
    ("handle.handle_in", offset_of!(Knobs, handle.handle_in), false),
    ("handle.far_reach", offset_of!(Knobs, handle.far_reach), false),
    ("handle.push_in", offset_of!(Knobs, handle.push_in), false),
    ("handle.push_off", offset_of!(Knobs, handle.push_off), false),
    ("handle.push_thru", offset_of!(Knobs, handle.push_thru), false),
    ("handle.stand_in", offset_of!(Knobs, handle.stand_in), false),
    ("handle.stand_off", offset_of!(Knobs, handle.stand_off), false),
    ("handle.thru_at", offset_of!(Knobs, handle.thru_at), false),
    ("handle.thru_off", offset_of!(Knobs, handle.thru_off), false),
    ("handle.body", offset_of!(Knobs, handle.body), false),
    ("handle.slack", offset_of!(Knobs, handle.slack), false),
    ("handle.near", offset_of!(Knobs, handle.near), false),
    ("handle.turn", offset_of!(Knobs, handle.turn), false),
    ("handle.clear", offset_of!(Knobs, handle.clear), false),
    ("handle.wait", offset_of!(Knobs, handle.wait), false),
    ("handle.press", offset_of!(Knobs, handle.press), false),
    ("handle.tool_post", offset_of!(Knobs, handle.tool_post), false),
    ("handle.near_body", offset_of!(Knobs, handle.near_body), false),
    ("handle.near_post", offset_of!(Knobs, handle.near_post), false),
    ("handle.go_cap", offset_of!(Knobs, handle.go_cap), false),
    ("handle.rest", offset_of!(Knobs, handle.rest), false),
    ("doormanners.reach", offset_of!(Knobs, doormanners.reach), false),
    ("doormanners.half", offset_of!(Knobs, doormanners.half), false),
    ("doormanners.throat", offset_of!(Knobs, doormanners.throat), false),
    ("doormanners.gap", offset_of!(Knobs, doormanners.gap), false),
    ("doormanners.gap_min", offset_of!(Knobs, doormanners.gap_min), false),
    ("doormanners.brake", offset_of!(Knobs, doormanners.brake), false),
    ("doormanners.patience", offset_of!(Knobs, doormanners.patience), false),
    ("doormanners.rest", offset_of!(Knobs, doormanners.rest), false),
    ("doormanners.beyond", offset_of!(Knobs, doormanners.beyond), false),
    ("doormanners.aside", offset_of!(Knobs, doormanners.aside), false),
    ("doormanners.jamb_out", offset_of!(Knobs, doormanners.jamb_out), false),
    ("doormanners.jamb_past", offset_of!(Knobs, doormanners.jamb_past), false),
    ("doormanners.file_gap", offset_of!(Knobs, doormanners.file_gap), false),
    ("doormanners.apart", offset_of!(Knobs, doormanners.apart), false),
    ("doormanners.step", offset_of!(Knobs, doormanners.step), false),
    ("doormanners.round", offset_of!(Knobs, doormanners.round), false),
    ("doormanners.round_out", offset_of!(Knobs, doormanners.round_out), false),
    ("doormanners.own_m", offset_of!(Knobs, doormanners.own_m), false),
    ("doormanners.own_end_m", offset_of!(Knobs, doormanners.own_end_m), false),
    ("doormanners.leaf_stall_m", offset_of!(Knobs, doormanners.leaf_stall_m), false),
    ("doormanners.leaf_stall_secs", offset_of!(Knobs, doormanners.leaf_stall_secs), false),
    ("doormanners.leaf_stall_reach", offset_of!(Knobs, doormanners.leaf_stall_reach), false),
    ("doormanners.leaf_stall_ahead", offset_of!(Knobs, doormanners.leaf_stall_ahead), false),
    ("doormanners.leaf_stall_push", offset_of!(Knobs, doormanners.leaf_stall_push), false),
    ("doormanners.sweep", offset_of!(Knobs, doormanners.sweep), false),
    ("doormanners.lane_ajar", offset_of!(Knobs, doormanners.lane_ajar), false),
    ("doormanners.lane_m", offset_of!(Knobs, doormanners.lane_m), false),
    ("doormanners.lane_out", offset_of!(Knobs, doormanners.lane_out), false),
    ("doormanners.lane_flare", offset_of!(Knobs, doormanners.lane_flare), false),
    ("doormanners.lane_clear", offset_of!(Knobs, doormanners.lane_clear), false),
    ("doormanners.lane_apart", offset_of!(Knobs, doormanners.lane_apart), false),
    ("doormanners.cover_m", offset_of!(Knobs, doormanners.cover_m), false),
    ("doormanners.cover_deep", offset_of!(Knobs, doormanners.cover_deep), false),
    ("doormanners.cover_look", offset_of!(Knobs, doormanners.cover_look), false),
    ("doormanners.cover_quiet", offset_of!(Knobs, doormanners.cover_quiet), false),
    ("doormanners.cover_cap", offset_of!(Knobs, doormanners.cover_cap), false),
    ("doormanners.lane_dear", offset_of!(Knobs, doormanners.lane_dear), false),
    ("doormanners.lane_strip", offset_of!(Knobs, doormanners.lane_strip), false),
    ("doormanners.lane_long", offset_of!(Knobs, doormanners.lane_long), false),
    ("doormanners.funnel", offset_of!(Knobs, doormanners.funnel), false),
    ("doormanners.funnel_rest", offset_of!(Knobs, doormanners.funnel_rest), false),
    ("doormanners.through", offset_of!(Knobs, doormanners.through), false),
    ("doormanners.past", offset_of!(Knobs, doormanners.past), false),
    ("doormanners.plane_deg", offset_of!(Knobs, doormanners.plane_deg), false),
    ("doormanners.rest_secs", offset_of!(Knobs, doormanners.rest_secs), false),
    ("doormanners.clear_m", offset_of!(Knobs, doormanners.clear_m), false),
    ("doormanners.held_cap", offset_of!(Knobs, doormanners.held_cap), false),
    ("breaks.share.big.thrown", offset_of!(Knobs, breaks.share.big.thrown), false),
    ("breaks.share.big.heap", offset_of!(Knobs, breaks.share.big.heap), false),
    ("breaks.share.big.furniture", offset_of!(Knobs, breaks.share.big.furniture), false),
    ("breaks.refuse_chips.0", offset_of!(Knobs, breaks.refuse_chips), false),
    ("breaks.refuse_chips.1", offset_of!(Knobs, breaks.refuse_chips) + 8, false),
    ("breaks.crumble_chips.0", offset_of!(Knobs, breaks.crumble_chips), false),
    ("breaks.crumble_chips.1", offset_of!(Knobs, breaks.crumble_chips) + 8, false),
    ("breaks.sized_chips.0", offset_of!(Knobs, breaks.sized_chips), false),
    ("breaks.sized_chips.1", offset_of!(Knobs, breaks.sized_chips) + 8, false),
    ("breaks.sized_span.0", offset_of!(Knobs, breaks.sized_span), false),
    ("breaks.sized_span.1", offset_of!(Knobs, breaks.sized_span) + 8, false),
    ("breaks.china_land", offset_of!(Knobs, breaks.china_land), false),
    ("breaks.china_max", offset_of!(Knobs, breaks.china_max), false),
    ("breaks.core_tint", offset_of!(Knobs, breaks.core_tint), false),
    ("breaks.chip_tint", offset_of!(Knobs, breaks.chip_tint), false),
    ("breaks.heap.span.0", offset_of!(Knobs, breaks.heap.span), false),
    ("breaks.heap.span.1", offset_of!(Knobs, breaks.heap.span) + 8, false),
    ("breaks.heap.out.0", offset_of!(Knobs, breaks.heap.out), false),
    ("breaks.heap.out.1", offset_of!(Knobs, breaks.heap.out) + 8, false),
    ("breaks.heap.tip.0", offset_of!(Knobs, breaks.heap.tip), false),
    ("breaks.heap.tip.1", offset_of!(Knobs, breaks.heap.tip) + 8, false),
    ("breaks.max_standing_planes", offset_of!(Knobs, breaks.max_standing_planes), false),
    ("breaks.across_deg.0", offset_of!(Knobs, breaks.across_deg), false),
    ("breaks.across_deg.1", offset_of!(Knobs, breaks.across_deg) + 8, false),
    ("breaks.loose_min", offset_of!(Knobs, breaks.loose_min), false),
    ("breaks.kink.deg.0", offset_of!(Knobs, breaks.kink.deg), false),
    ("breaks.kink.deg.1", offset_of!(Knobs, breaks.kink.deg) + 8, false),
    ("breaks.kink.skew", offset_of!(Knobs, breaks.kink.skew), false),
    ("breaks.kink.off", offset_of!(Knobs, breaks.kink.off), false),
    ("breaks.kink.min_long", offset_of!(Knobs, breaks.kink.min_long), false),
    ("breaks.kink.gap_secs", offset_of!(Knobs, breaks.kink.gap_secs), false),
    ("breaks.head.from", offset_of!(Knobs, breaks.head.from), false),
    ("breaks.head.deep.0", offset_of!(Knobs, breaks.head.deep), false),
    ("breaks.head.deep.1", offset_of!(Knobs, breaks.head.deep) + 8, false),
    ("breaks.head.planes", offset_of!(Knobs, breaks.head.planes), false),
    ("breaks.head.across.0", offset_of!(Knobs, breaks.head.across), false),
    ("breaks.head.across.1", offset_of!(Knobs, breaks.head.across) + 8, false),
    ("breaks.head.min_room", offset_of!(Knobs, breaks.head.min_room), false),
    ("breaks.head.most_of_width", offset_of!(Knobs, breaks.head.most_of_width), false),
    ("breaks.steel.sheet", offset_of!(Knobs, breaks.steel.sheet), false),
    ("breaks.steel.kink.0", offset_of!(Knobs, breaks.steel.kink), false),
    ("breaks.steel.kink.1", offset_of!(Knobs, breaks.steel.kink) + 8, false),
    ("breaks.steel.dents.0", offset_of!(Knobs, breaks.steel.dents), false),
    ("breaks.steel.dents.1", offset_of!(Knobs, breaks.steel.dents) + 8, false),
    ("breaks.steel.dent.0", offset_of!(Knobs, breaks.steel.dent), false),
    ("breaks.steel.dent.1", offset_of!(Knobs, breaks.steel.dent) + 8, false),
    ("breaks.steel.dent_most", offset_of!(Knobs, breaks.steel.dent_most), false),
    ("breaks.steel.rim", offset_of!(Knobs, breaks.steel.rim), false),
    ("breaks.steel.lip", offset_of!(Knobs, breaks.steel.lip), false),
    ("breaks.steel.inside", offset_of!(Knobs, breaks.steel.inside), false),
    ("breaks.stud.half", offset_of!(Knobs, breaks.stud.half), false),
    ("breaks.stud.depth_k", offset_of!(Knobs, breaks.stud.depth_k), false),
    ("breaks.stud.foot", offset_of!(Knobs, breaks.stud.foot), false),
    ("breaks.stud.albedo.0", offset_of!(Knobs, breaks.stud.albedo), false),
    ("breaks.stud.albedo.1", offset_of!(Knobs, breaks.stud.albedo) + 8, false),
    ("breaks.stud.albedo.2", offset_of!(Knobs, breaks.stud.albedo) + 16, false),
    ("breaks.stud.rough", offset_of!(Knobs, breaks.stud.rough), false),
    ("furn.shatter_m", offset_of!(Knobs, furn.shatter_m), false),
    ("furn.apart_m", offset_of!(Knobs, furn.apart_m), false),
    ("furn.throw_m", offset_of!(Knobs, furn.throw_m), false),
    ("furn.throw_v", offset_of!(Knobs, furn.throw_v), false),
    ("furn.prop_break_m", offset_of!(Knobs, furn.prop_break_m), false),
    ("furn.chunks_per_blast", offset_of!(Knobs, furn.chunks_per_blast), false),
    ("furn.mouth_m", offset_of!(Knobs, furn.mouth_m), false),
    ("furn.merge_m", offset_of!(Knobs, furn.merge_m), false),
    ("furn.mouth.depth", offset_of!(Knobs, furn.mouth.depth), false),
    ("furn.mouth.top", offset_of!(Knobs, furn.mouth.top), false),
    ("furn.mouth.vol", offset_of!(Knobs, furn.mouth.vol), false),
    ("furn.mouth.chunks", offset_of!(Knobs, furn.mouth.chunks), false),
    ("furn.mouth.base_thick", offset_of!(Knobs, furn.mouth.base_thick), false),
    ("furn.hung.len", offset_of!(Knobs, furn.hung.len), false),
    ("furn.hung.max", offset_of!(Knobs, furn.hung.max), false),
    ("furn.hung.strips", offset_of!(Knobs, furn.hung.strips), false),
    ("furn.hung.under_m", offset_of!(Knobs, furn.hung.under_m), false),
    ("furn.hung.side", offset_of!(Knobs, furn.hung.side), false),
    ("furn.hung.spin", offset_of!(Knobs, furn.hung.spin), false),
    ("furn.hung.chunks", offset_of!(Knobs, furn.hung.chunks), false),
    ("furn.hung.hole_m", offset_of!(Knobs, furn.hung.hole_m), false),
    ("furn.root.min_top", offset_of!(Knobs, furn.root.min_top), false),
    ("furn.root.max_top", offset_of!(Knobs, furn.root.max_top), false),
    ("furn.root.max_long", offset_of!(Knobs, furn.root.max_long), false),
    ("furn.root.max_vol", offset_of!(Knobs, furn.root.max_vol), false),
    ("furn.root.apart_max_vol", offset_of!(Knobs, furn.root.apart_max_vol), false),
    ("furn.rider.max_top", offset_of!(Knobs, furn.rider.max_top), false),
    ("furn.rider.max_vol", offset_of!(Knobs, furn.rider.max_vol), false),
    ("furn.touch", offset_of!(Knobs, furn.touch), false),
    ("furn.row_m", offset_of!(Knobs, furn.row_m), false),
    ("snowbog.dead", offset_of!(Knobs, snowbog.dead), false),
    ("snowbog.grip", offset_of!(Knobs, snowbog.grip), false),
    ("snowbog.stick", offset_of!(Knobs, snowbog.stick), false),
    ("snowbog.packed", offset_of!(Knobs, snowbog.packed), false),
    ("snowbog.old", offset_of!(Knobs, snowbog.old), false),
    ("snowbog.top", offset_of!(Knobs, snowbog.top), false),
    ("snowbog.bank_m", offset_of!(Knobs, snowbog.bank_m), false),
    ("snowbog.run", offset_of!(Knobs, snowbog.run), false),
    ("rubble.radius", offset_of!(Knobs, rubble.radius), false),
    ("rubble.reach", offset_of!(Knobs, rubble.reach), false),
    ("rubble.lead", offset_of!(Knobs, rubble.lead), false),
    ("rubble.inside", offset_of!(Knobs, rubble.inside), false),
    ("rubble.floor", offset_of!(Knobs, rubble.floor), false),
    ("rubble.heavy", offset_of!(Knobs, rubble.heavy), false),
    ("rubble.walk", offset_of!(Knobs, rubble.walk), false),
    ("rubble.base", offset_of!(Knobs, rubble.base), false),
    ("rubble.gain", offset_of!(Knobs, rubble.gain), false),
    ("rubble.aside", offset_of!(Knobs, rubble.aside), false),
    ("rubble.v_max", offset_of!(Knobs, rubble.v_max), false),
    ("rubble.off", offset_of!(Knobs, rubble.off), false),
    ("rubble.slide", offset_of!(Knobs, rubble.slide), false),
    ("rubble.spin", offset_of!(Knobs, rubble.spin), false),
    ("rubble.per_frame", offset_of!(Knobs, rubble.per_frame), false),
    ("rubble.cooldown", offset_of!(Knobs, rubble.cooldown), false),
    ("rubble.give_up", offset_of!(Knobs, rubble.give_up), false),
    ("rubble.give_up_secs", offset_of!(Knobs, rubble.give_up_secs), false),
    ("rubble.scuff", offset_of!(Knobs, rubble.scuff), false),
    ("debris.min_speed", offset_of!(Knobs, debris.min_speed), false),
    ("lightwall.gap", offset_of!(Knobs, lightwall.gap), false),
    ("airdust.diffuse", offset_of!(Knobs, airdust.diffuse), false),
    ("airdust.settle", offset_of!(Knobs, airdust.settle), false),
    ("airdust.step", offset_of!(Knobs, airdust.step), false),
    ("airdust.recheck_secs", offset_of!(Knobs, airdust.recheck_secs), false),
    ("wallsnap.on", offset_of!(Knobs, wallsnap.on), true),
    ("wallsnap.take_base", offset_of!(Knobs, wallsnap.take_base), false),
    ("wallsnap.take_per_m", offset_of!(Knobs, wallsnap.take_per_m), false),
    ("wallsnap.take_per_turn", offset_of!(Knobs, wallsnap.take_per_turn), false),
    ("wallsnap.take_min", offset_of!(Knobs, wallsnap.take_min), false),
    ("wallsnap.take_max", offset_of!(Knobs, wallsnap.take_max), false),
    ("wallsnap.ease_pow", offset_of!(Knobs, wallsnap.ease_pow), false),
    ("wallsnap.yaw_pow", offset_of!(Knobs, wallsnap.yaw_pow), false),
    ("wallsnap.reach_moving", offset_of!(Knobs, wallsnap.reach_moving), false),
    ("wallsnap.heading_min", offset_of!(Knobs, wallsnap.heading_min), false),
    ("wallsnap.heading_bonus", offset_of!(Knobs, wallsnap.heading_bonus), false),
    ("wallsnap.buffer", offset_of!(Knobs, wallsnap.buffer), false),
    ("wallsnap.dip", offset_of!(Knobs, wallsnap.dip), false),
    ("wallsnap.nod", offset_of!(Knobs, wallsnap.nod), false),
    ("wallsnap.hit_peak", offset_of!(Knobs, wallsnap.hit_peak), false),
    ("wallsnap.scuff", offset_of!(Knobs, wallsnap.scuff), false),
    ("wallsnap.step_lift", offset_of!(Knobs, wallsnap.step_lift), false),
    ("wallsnap.step_trail", offset_of!(Knobs, wallsnap.step_trail), false),
    ("wallsnap.cam_room", offset_of!(Knobs, wallsnap.cam_room), false),
    ("wallsnap.cam_side", offset_of!(Knobs, wallsnap.cam_side), false),
    ("wallsnap.along_deg", offset_of!(Knobs, wallsnap.along_deg), false),
    ("wallsnap.away_secs", offset_of!(Knobs, wallsnap.away_secs), false),
    ("wallsnap.away_diag", offset_of!(Knobs, wallsnap.away_diag), false),
    ("wallsnap.off_push", offset_of!(Knobs, wallsnap.off_push), false),
    ("wallsnap.out_rate", offset_of!(Knobs, wallsnap.out_rate), false),
    ("wallsnap.up_secs", offset_of!(Knobs, wallsnap.up_secs), false),
    ("wallsnap.up_rise", offset_of!(Knobs, wallsnap.up_rise), false),
    ("wallsnap.bore", offset_of!(Knobs, wallsnap.bore), false),
    ("wallsnap.lean_along", offset_of!(Knobs, wallsnap.lean_along), false),
    ("wallsnap.lean_out", offset_of!(Knobs, wallsnap.lean_out), false),
    ("wallsnap.lean_back", offset_of!(Knobs, wallsnap.lean_back), false),
    ("wallsnap.lean_on_deg", offset_of!(Knobs, wallsnap.lean_on_deg), false),
    ("wallsnap.lean_off_deg", offset_of!(Knobs, wallsnap.lean_off_deg), false),
    ("wallsnap.lean_min_dist", offset_of!(Knobs, wallsnap.lean_min_dist), false),
    ("wallsnap.lean_max_deg", offset_of!(Knobs, wallsnap.lean_max_deg), false),
    ("wallsnap.lean_keep_deg", offset_of!(Knobs, wallsnap.lean_keep_deg), false),
    ("wallsnap.lean_keep_dist", offset_of!(Knobs, wallsnap.lean_keep_dist), false),
    ("wallsnap.lean_over_y", offset_of!(Knobs, wallsnap.lean_over_y), false),
    ("wallsnap.lean_clear_m", offset_of!(Knobs, wallsnap.lean_clear_m), false),
    ("wallsnap.lean_gun", offset_of!(Knobs, wallsnap.lean_gun), false),
    ("wallsnap.lean_gun_step", offset_of!(Knobs, wallsnap.lean_gun_step), false),
    ("wallsnap.dwell_out", offset_of!(Knobs, wallsnap.dwell_out), false),
    ("wallsnap.dwell_back", offset_of!(Knobs, wallsnap.dwell_back), false),
    ("wallsnap.min_out", offset_of!(Knobs, wallsnap.min_out), false),
    ("wallsnap.after_shot", offset_of!(Knobs, wallsnap.after_shot), false),
    ("wallsnap.press", offset_of!(Knobs, wallsnap.press), true),
    ("wallsnap.press_secs", offset_of!(Knobs, wallsnap.press_secs), false),
    ("wallsnap.press_flow", offset_of!(Knobs, wallsnap.press_flow), false),
    ("wallsnap.press_near", offset_of!(Knobs, wallsnap.press_near), false),
    ("wallsnap.press_deg", offset_of!(Knobs, wallsnap.press_deg), false),
    ("wallsnap.press_touch", offset_of!(Knobs, wallsnap.press_touch), false),
    ("wallsnap.press_thing", offset_of!(Knobs, wallsnap.press_thing), false),
    ("wallsnap.press_thing_at", offset_of!(Knobs, wallsnap.press_thing_at), false),
    ("wallsnap.press_turn", offset_of!(Knobs, wallsnap.press_turn), false),
    ("wallsnap.press_again", offset_of!(Knobs, wallsnap.press_again), false),
    ("wallsnap.press_after_shot", offset_of!(Knobs, wallsnap.press_after_shot), false),
    ("wallsnap.voice_on", offset_of!(Knobs, wallsnap.voice_on), false),
    ("wallsnap.voice_off", offset_of!(Knobs, wallsnap.voice_off), false),
    ("wallsnap.slow_take", offset_of!(Knobs, wallsnap.slow_take), false),
    ("wallsnap.slow_in", offset_of!(Knobs, wallsnap.slow_in), false),
    ("wallsnap.slow_out", offset_of!(Knobs, wallsnap.slow_out), false),
    ("gunwall.down_m", offset_of!(Knobs, gunwall.down_m), false),
    ("gunwall.up_m", offset_of!(Knobs, gunwall.up_m), false),
    ("gunwall.lead_s", offset_of!(Knobs, gunwall.lead_s), false),
    ("gunwall.at_m", offset_of!(Knobs, gunwall.at_m), false),
    ("gunwall.tuck_m", offset_of!(Knobs, gunwall.tuck_m), false),
    ("gunwall.tuck_lead_s", offset_of!(Knobs, gunwall.tuck_lead_s), false),
    ("gunwall.fight_s", offset_of!(Knobs, gunwall.fight_s), false),
    ("close.rmb_tap_secs", offset_of!(Knobs, close.rmb_tap_secs), false),
    ("close.rmb_loud", offset_of!(Knobs, close.rmb_loud), false),
    ("close.rmb_wait_secs", offset_of!(Knobs, close.rmb_wait_secs), false),
    ("close.hands_low_neck_m", offset_of!(Knobs, close.hands_low_neck_m), false),
    ("close.mode_m", offset_of!(Knobs, close.mode_m), false),
    ("close.mode_out_m", offset_of!(Knobs, close.mode_out_m), false),
    ("close.dark_at", offset_of!(Knobs, close.dark_at), false),
    ("close.lit_at", offset_of!(Knobs, close.lit_at), false),
    ("close.touch_m", offset_of!(Knobs, close.touch_m), false),
    ("close.fade_m", offset_of!(Knobs, close.fade_m), false),
    ("close.reach.behind", offset_of!(Knobs, close.reach.behind), false),
    ("close.reach.side", offset_of!(Knobs, close.reach.side), false),
    ("close.reach.front", offset_of!(Knobs, close.reach.front), false),
    ("close.reach.seat", offset_of!(Knobs, close.reach.seat), false),
    ("close.reach.seat_front", offset_of!(Knobs, close.reach.seat_front), false),
    ("close.side_from_deg", offset_of!(Knobs, close.side_from_deg), false),
    ("close.inner", offset_of!(Knobs, close.inner), false),
    ("close.step_deg", offset_of!(Knobs, close.step_deg), false),
    ("close.strike_dist", offset_of!(Knobs, close.strike_dist), false),
    ("close.seat_dist", offset_of!(Knobs, close.seat_dist), false),
    ("close.noise.behind", offset_of!(Knobs, close.noise.behind), false),
    ("close.noise.side", offset_of!(Knobs, close.noise.side), false),
    ("close.noise.front", offset_of!(Knobs, close.noise.front), false),
    ("close.noise.seat", offset_of!(Knobs, close.noise.seat), false),
    ("close.alert_noise", offset_of!(Knobs, close.alert_noise), false),
    ("plan.fresh_secs", offset_of!(Knobs, plan.fresh_secs), false),
    ("plan.walk_dim", offset_of!(Knobs, plan.walk_dim), false),
    ("plan.rest_dim", offset_of!(Knobs, plan.rest_dim), false),
    ("plan.ring_m", offset_of!(Knobs, plan.ring_m), false),
    ("plan.sector_m", offset_of!(Knobs, plan.sector_m), false),
    ("plan.sector_deg", offset_of!(Knobs, plan.sector_deg), false),
    ("plan.sight_m", offset_of!(Knobs, plan.sight_m), false),
    ("plan.reask_secs", offset_of!(Knobs, plan.reask_secs), false),
    ("plan.pair_bar_m", offset_of!(Knobs, plan.pair_bar_m), false),
    ("plan.others", offset_of!(Knobs, plan.others), false),
    ("reload.delay", offset_of!(Knobs, reload.delay), false),
    ("samstun.fade", offset_of!(Knobs, samstun.fade), false),
    ("recap.secs", offset_of!(Knobs, recap.secs), false),
    ("recap.after", offset_of!(Knobs, recap.after), false),
    ("recap.hz", offset_of!(Knobs, recap.hz), false),
    ("recap.keep", offset_of!(Knobs, recap.keep), false),
    ("recap.live_see_m", offset_of!(Knobs, recap.live_see_m), false),
    ("recap.live_saw_secs", offset_of!(Knobs, recap.live_saw_secs), false),
    ("recap.live_hear_m", offset_of!(Knobs, recap.live_hear_m), false),
    ("recap.live_beat", offset_of!(Knobs, recap.live_beat), false),
    ("recap.live_side_m", offset_of!(Knobs, recap.live_side_m), false),
    ("recap.live_mark_secs", offset_of!(Knobs, recap.live_mark_secs), false),
    ("aimweight.rate", offset_of!(Knobs, aimweight.rate), false),
    ("aimweight.smooth", offset_of!(Knobs, aimweight.smooth), false),
    ("aimweight.heft", offset_of!(Knobs, aimweight.heft), false),
    ("aimweight.bare", offset_of!(Knobs, aimweight.bare), false),
    ("aimweight.on_m", offset_of!(Knobs, aimweight.on_m), false),
    ("aimweight.reach", offset_of!(Knobs, aimweight.reach), false),
    ("aimweight.launch_wait", offset_of!(Knobs, aimweight.launch_wait), false),
    ("aimcost.bare", offset_of!(Knobs, aimcost.bare), false),
    ("aimcost.pad", offset_of!(Knobs, aimcost.pad), false),
    ("aimcost.slack", offset_of!(Knobs, aimcost.slack), false),
    ("aimcost.move_k", offset_of!(Knobs, aimcost.move_k), false),
    ("aimcost.stop_tau", offset_of!(Knobs, aimcost.stop_tau), false),
    ("aimcost.stop_bleed", offset_of!(Knobs, aimcost.stop_bleed), false),
    ("aimcost.brace_feet", offset_of!(Knobs, aimcost.brace_feet), false),
    ("aimcost.bare_device", offset_of!(Knobs, aimcost.bare_device), false),
    ("aimcost.plant_at", offset_of!(Knobs, aimcost.plant_at), false),
    ("aimcost.flank_at", offset_of!(Knobs, aimcost.flank_at), false),
    ("aimcost.ring_px", offset_of!(Knobs, aimcost.ring_px), false),
    ("friends.reach_m", offset_of!(Knobs, friends.reach_m), false),
    ("friends.past_m", offset_of!(Knobs, friends.past_m), false),
    ("friends.clear_m", offset_of!(Knobs, friends.clear_m), false),
    ("friends.lead_s", offset_of!(Knobs, friends.lead_s), false),
    ("friends.lead_max_m", offset_of!(Knobs, friends.lead_max_m), false),
    ("friends.up_secs", offset_of!(Knobs, friends.up_secs), false),
    ("friends.quiet_s", offset_of!(Knobs, friends.quiet_s), false),
    ("friendlyfire.cone_k", offset_of!(Knobs, friendlyfire.cone_k), false),
    ("friendlyfire.body_m", offset_of!(Knobs, friendlyfire.body_m), false),
    ("friendlyfire.sure_m", offset_of!(Knobs, friendlyfire.sure_m), false),
    ("friendlyfire.near_m", offset_of!(Knobs, friendlyfire.near_m), false),
    ("lightswitch.reach", offset_of!(Knobs, lightswitch.reach), false),
    ("lightswitch.height", offset_of!(Knobs, lightswitch.height), false),
    ("lightswitch.stand_m", offset_of!(Knobs, lightswitch.stand_m), false),
    ("lightswitch.per_room", offset_of!(Knobs, lightswitch.per_room), false),
    ("lightswitch.apart_m", offset_of!(Knobs, lightswitch.apart_m), false),
    ("lightswitch.scan_m", offset_of!(Knobs, lightswitch.scan_m), false),
    ("lightswitch.guard_secs", offset_of!(Knobs, lightswitch.guard_secs), false),
    ("lightswitch.hand_secs", offset_of!(Knobs, lightswitch.hand_secs), false),
    ("carlamps.dark.0", offset_of!(Knobs, carlamps.dark), false),
    ("carlamps.dark.1", offset_of!(Knobs, carlamps.dark) + 8, false),
    ("carlamps.dark.2", offset_of!(Knobs, carlamps.dark) + 16, false),
    ("carlamps.fit", offset_of!(Knobs, carlamps.fit), false),
    ("carlamps.tone", offset_of!(Knobs, carlamps.tone), false),
    ("carlamps.twin", offset_of!(Knobs, carlamps.twin), false),
    ("reach_pace.steps", offset_of!(Knobs, reach_pace.steps), false),
    ("reach_pace.whole", offset_of!(Knobs, reach_pace.whole), true),
    ("reach_pace.whole_cells", offset_of!(Knobs, reach_pace.whole_cells), false),
    ("coatfinish.on", offset_of!(Knobs, coatfinish.on), true),
    ("solidglass.on", offset_of!(Knobs, solidglass.on), true),
    ("solidglass.lum", offset_of!(Knobs, solidglass.lum), false),
    ("solidglass.rough", offset_of!(Knobs, solidglass.rough), false),
    ("wallskin.on", offset_of!(Knobs, wallskin.on), true),
    ("wallskin.thick", offset_of!(Knobs, wallskin.thick), false),
    ("wallskin.off", offset_of!(Knobs, wallskin.off), false),
    ("wallskin.per_skin", offset_of!(Knobs, wallskin.per_skin), false),
    ("wallskin.max_pieces", offset_of!(Knobs, wallskin.max_pieces), false),
    ("wallskin.box_slack", offset_of!(Knobs, wallskin.box_slack), false),
    ("fireline.fire_s", offset_of!(Knobs, fireline.fire_s), false),
    ("fireline.reload_s", offset_of!(Knobs, fireline.reload_s), false),
    ("fireline.pace_s", offset_of!(Knobs, fireline.pace_s), false),
    ("fireline.pace_k", offset_of!(Knobs, fireline.pace_k), false),
    ("fireline.lapse_s", offset_of!(Knobs, fireline.lapse_s), false),
    ("fireline.spread", offset_of!(Knobs, fireline.spread), false),
    ("fireline.clear_m", offset_of!(Knobs, fireline.clear_m), false),
    ("fireline.short_m", offset_of!(Knobs, fireline.short_m), false),
    ("fireline.keep_m", offset_of!(Knobs, fireline.keep_m), false),
    ("fireline.gap_m", offset_of!(Knobs, fireline.gap_m), false),
    ("fireline.look_m", offset_of!(Knobs, fireline.look_m), false),
    ("fireline.cue_s", offset_of!(Knobs, fireline.cue_s), false),
    ("fireline.max_s", offset_of!(Knobs, fireline.max_s), false),
    ("fireline.again_s", offset_of!(Knobs, fireline.again_s), false),
    ("fireline.way_more_m", offset_of!(Knobs, fireline.way_more_m), false),
    ("fireline.berth_m", offset_of!(Knobs, fireline.berth_m), false),
    ("fireline.edge_m", offset_of!(Knobs, fireline.edge_m), false),
    ("fireline.reach_m", offset_of!(Knobs, fireline.reach_m), false),
    ("savetimeline.manual", offset_of!(Knobs, savetimeline.manual), false),
    ("savetimeline.auto", offset_of!(Knobs, savetimeline.auto), false),
    ("savetimeline.pic_px", offset_of!(Knobs, savetimeline.pic_px), false),
    ("savetimeline.pic_q", offset_of!(Knobs, savetimeline.pic_q), false),
    ("handring.hold_secs", offset_of!(Knobs, handring.hold_secs), false),
    ("handring.turn_max", offset_of!(Knobs, handring.turn_max), false),
    ("sprintdoor.keep", offset_of!(Knobs, sprintdoor.keep), false),
    ("sprintdoor.bash_vel", offset_of!(Knobs, sprintdoor.bash_vel), false),
    ("sprintdoor.lean", offset_of!(Knobs, sprintdoor.lean), false),
    ("sprintdoor.lean_secs", offset_of!(Knobs, sprintdoor.lean_secs), false),
    ("sprintdoor.rip_pace", offset_of!(Knobs, sprintdoor.rip_pace), false),
    ("sprintdoor.rip_secs", offset_of!(Knobs, sprintdoor.rip_secs), false),
    ("sprintdoor.rip_vel", offset_of!(Knobs, sprintdoor.rip_vel), false),
    ("sprintdoor.rip_level", offset_of!(Knobs, sprintdoor.rip_level), false),
    ("sprintdoor.kick_k", offset_of!(Knobs, sprintdoor.kick_k), false),
    ("sprintdoor.kick_into", offset_of!(Knobs, sprintdoor.kick_into), false),
    ("sprintdoor.kick_free", offset_of!(Knobs, sprintdoor.kick_free), false),
    ("teampace.k", offset_of!(Knobs, teampace.k), false),
    ("teampace.follow_k", offset_of!(Knobs, teampace.follow_k), false),
    ("teampace.behind_m", offset_of!(Knobs, teampace.behind_m), false),
    ("teampace.fight_secs", offset_of!(Knobs, teampace.fight_secs), false),
    ("teampace.back_secs", offset_of!(Knobs, teampace.back_secs), false),
    ("teampace.dash_m", offset_of!(Knobs, teampace.dash_m), false),
    ("teampace.dash_seen", offset_of!(Knobs, teampace.dash_seen), false),
    ("teampace.dash_every", offset_of!(Knobs, teampace.dash_every), false),
    ("teampace.dash_hot", offset_of!(Knobs, teampace.dash_hot), false),
    ("teampace.door_brisk", offset_of!(Knobs, teampace.door_brisk), true),
    ("teampace.pacing", offset_of!(Knobs, teampace.pacing), true),
    ("teampace.pace_near", offset_of!(Knobs, teampace.pace_near), false),
    ("teampace.pace_gap", offset_of!(Knobs, teampace.pace_gap), false),
    ("teampace.pace_cos", offset_of!(Knobs, teampace.pace_cos), false),
    ("teampace.walk_aim_off", offset_of!(Knobs, teampace.walk_aim_off), false),
    ("teampace.walk_aim_on", offset_of!(Knobs, teampace.walk_aim_on), false),
    ("teampace.run_at", offset_of!(Knobs, teampace.run_at), false),
    ("teampace.accel", offset_of!(Knobs, teampace.accel), false),
    ("teampace.step", offset_of!(Knobs, teampace.step), false),
    ("teampace.gap", offset_of!(Knobs, teampace.gap), false),
    ("noshove.ring", offset_of!(Knobs, noshove.ring), false),
    ("noshove.back", offset_of!(Knobs, noshove.back), false),
    ("rails.pan_speed", offset_of!(Knobs, rails.pan_speed), false),
    ("rails.pan_reach", offset_of!(Knobs, rails.pan_reach), false),
    ("rails.snap_px", offset_of!(Knobs, rails.snap_px), false),
    ("rails.keep", offset_of!(Knobs, rails.keep), false),
    ("rails.keep_speed", offset_of!(Knobs, rails.keep_speed), false),
    ("rails.keep_deep", offset_of!(Knobs, rails.keep_deep), false),
    ("rails.keep_up", offset_of!(Knobs, rails.keep_up), false),
    ("rails.way_m", offset_of!(Knobs, rails.way_m), false),
    ("rails.after_secs", offset_of!(Knobs, rails.after_secs), false),
    ("rails.after_max", offset_of!(Knobs, rails.after_max), false),
    ("rails.card_beat_secs", offset_of!(Knobs, rails.card_beat_secs), false),
    ("rails.card_quiet", offset_of!(Knobs, rails.card_quiet), false),
    ("rails.card_max", offset_of!(Knobs, rails.card_max), false),
    ("rails.spare_secs", offset_of!(Knobs, rails.spare_secs), false),
    ("doorforce.sure", offset_of!(Knobs, doorforce.sure), false),
    ("doorforce.keep", offset_of!(Knobs, doorforce.keep), false),
    ("doorforce.fade_secs", offset_of!(Knobs, doorforce.fade_secs), false),
    ("doorforce.stale_secs", offset_of!(Knobs, doorforce.stale_secs), false),
    ("doorforce.gain", offset_of!(Knobs, doorforce.gain), false),
    ("doorforce.hear_min", offset_of!(Knobs, doorforce.hear_min), false),
    ("doorforce.shot", offset_of!(Knobs, doorforce.shot), false),
    ("doorforce.handle", offset_of!(Knobs, doorforce.handle), false),
    ("doorforce.seen", offset_of!(Knobs, doorforce.seen), false),
    ("doorforce.shut_carry", offset_of!(Knobs, doorforce.shut_carry), false),
    ("doorforce.behind_m", offset_of!(Knobs, doorforce.behind_m), false),
    ("doorforce.reach_m", offset_of!(Knobs, doorforce.reach_m), false),
    ("doorforce.seen_secs", offset_of!(Knobs, doorforce.seen_secs), false),
    ("doorforce.heard_err_m", offset_of!(Knobs, doorforce.heard_err_m), false),
    ("doorforce.chest_y", offset_of!(Knobs, doorforce.chest_y), false),
    ("doorforce.near_leaf_m", offset_of!(Knobs, doorforce.near_leaf_m), false),
    ("doorforce.past_leaf_m", offset_of!(Knobs, doorforce.past_leaf_m), false),
    ("doorforce.rounds", offset_of!(Knobs, doorforce.rounds), false),
    ("doorforce.gap_secs", offset_of!(Knobs, doorforce.gap_secs), false),
    ("doorforce.kick_secs", offset_of!(Knobs, doorforce.kick_secs), false),
    ("doorforce.kick_secs_no_shot", offset_of!(Knobs, doorforce.kick_secs_no_shot), false),
    ("doorforce.walk_secs", offset_of!(Knobs, doorforce.walk_secs), false),
    ("doorforce.rest_secs", offset_of!(Knobs, doorforce.rest_secs), false),
    ("slotsetup.secs", offset_of!(Knobs, slotsetup.secs), false),
    ("slotsetup.settle", offset_of!(Knobs, slotsetup.settle), false),
    ("glassreal.hang", offset_of!(Knobs, glassreal.hang), false),
    ("glassreal.blocks", offset_of!(Knobs, glassreal.blocks), false),
    ("glassreal.low_m", offset_of!(Knobs, glassreal.low_m), false),
    ("glassreal.door_m", offset_of!(Knobs, glassreal.door_m), false),
    ("glassreal.door_foot_m", offset_of!(Knobs, glassreal.door_foot_m), false),
    ("told.guided", offset_of!(Knobs, told.guided), false),
    ("told.standard", offset_of!(Knobs, told.standard), false),
    ("kitanim.sling", offset_of!(Knobs, kitanim.sling), false),
    ("kitanim.sling_up", offset_of!(Knobs, kitanim.sling_up), false),
    ("kitanim.sling_down", offset_of!(Knobs, kitanim.sling_down), false),
    ("kitanim.early", offset_of!(Knobs, kitanim.early), false),
    ("kitanim.click_keep", offset_of!(Knobs, kitanim.click_keep), false),
    ("kitanim.throw_keep", offset_of!(Knobs, kitanim.throw_keep), false),
    ("kitanim.plain_lock", offset_of!(Knobs, kitanim.plain_lock), false),
    ("kitanim.beats.pistol.ref_", offset_of!(Knobs, kitanim.beats.pistol.ref_), false),
    ("kitanim.beats.pistol.empty.dur", offset_of!(Knobs, kitanim.beats.pistol.empty.dur), false),
    ("kitanim.beats.pistol.empty.out", offset_of!(Knobs, kitanim.beats.pistol.empty.out), false),
    ("kitanim.beats.pistol.empty.in_", offset_of!(Knobs, kitanim.beats.pistol.empty.in_), false),
    ("kitanim.beats.pistol.dry.dur", offset_of!(Knobs, kitanim.beats.pistol.dry.dur), false),
    ("kitanim.beats.pistol.dry.out", offset_of!(Knobs, kitanim.beats.pistol.dry.out), false),
    ("kitanim.beats.pistol.dry.in_", offset_of!(Knobs, kitanim.beats.pistol.dry.in_), false),
    ("kitanim.beats.pistol.tactical.dur", offset_of!(Knobs, kitanim.beats.pistol.tactical.dur), false),
    ("kitanim.beats.pistol.tactical.out", offset_of!(Knobs, kitanim.beats.pistol.tactical.out), false),
    ("kitanim.beats.pistol.tactical.in_", offset_of!(Knobs, kitanim.beats.pistol.tactical.in_), false),
    ("kitanim.beats.rifle.ref_", offset_of!(Knobs, kitanim.beats.rifle.ref_), false),
    ("kitanim.beats.rifle.empty.dur", offset_of!(Knobs, kitanim.beats.rifle.empty.dur), false),
    ("kitanim.beats.rifle.empty.out", offset_of!(Knobs, kitanim.beats.rifle.empty.out), false),
    ("kitanim.beats.rifle.empty.in_", offset_of!(Knobs, kitanim.beats.rifle.empty.in_), false),
    ("kitanim.beats.rifle.dry.dur", offset_of!(Knobs, kitanim.beats.rifle.dry.dur), false),
    ("kitanim.beats.rifle.dry.out", offset_of!(Knobs, kitanim.beats.rifle.dry.out), false),
    ("kitanim.beats.rifle.dry.in_", offset_of!(Knobs, kitanim.beats.rifle.dry.in_), false),
    ("kitanim.beats.rifle.tactical.dur", offset_of!(Knobs, kitanim.beats.rifle.tactical.dur), false),
    ("kitanim.beats.rifle.tactical.out", offset_of!(Knobs, kitanim.beats.rifle.tactical.out), false),
    ("kitanim.beats.rifle.tactical.in_", offset_of!(Knobs, kitanim.beats.rifle.tactical.in_), false),
    ("kitanim.close_near", offset_of!(Knobs, kitanim.close_near), false),
    ("kit.away", offset_of!(Knobs, kit.away), false),
    ("kit.swap_k", offset_of!(Knobs, kit.swap_k), false),
    ("kit.far", offset_of!(Knobs, kit.far), false),
    ("kit.apex_at", offset_of!(Knobs, kit.apex_at), false),
    ("kit.sling_at", offset_of!(Knobs, kit.sling_at), false),
    ("kit.join", offset_of!(Knobs, kit.join), false),
    ("kit.let_go", offset_of!(Knobs, kit.let_go), false),
    ("kit.apex.at.0", offset_of!(Knobs, kit.apex.at), false),
    ("kit.apex.at.1", offset_of!(Knobs, kit.apex.at) + 8, false),
    ("kit.apex.at.2", offset_of!(Knobs, kit.apex.at) + 16, false),
    ("kit.apex.fing.0", offset_of!(Knobs, kit.apex.fing), false),
    ("kit.apex.fing.1", offset_of!(Knobs, kit.apex.fing) + 8, false),
    ("kit.apex.fing.2", offset_of!(Knobs, kit.apex.fing) + 16, false),
    ("kit.apex.back.0", offset_of!(Knobs, kit.apex.back), false),
    ("kit.apex.back.1", offset_of!(Knobs, kit.apex.back) + 8, false),
    ("kit.apex.back.2", offset_of!(Knobs, kit.apex.back) + 16, false),
    ("kit.apex.pole.0", offset_of!(Knobs, kit.apex.pole), false),
    ("kit.apex.pole.1", offset_of!(Knobs, kit.apex.pole) + 8, false),
    ("kit.apex.pole.2", offset_of!(Knobs, kit.apex.pole) + 16, false),
    ("kit.elbow_min", offset_of!(Knobs, kit.elbow_min), false),
    ("kit.wrist_knee", offset_of!(Knobs, kit.wrist_knee), false),
    ("kit.wrist_max", offset_of!(Knobs, kit.wrist_max), false),
    ("kit.roll_in", offset_of!(Knobs, kit.roll_in), false),
    ("kit.over.0", offset_of!(Knobs, kit.over), false),
    ("kit.over.1", offset_of!(Knobs, kit.over) + 8, false),
    ("kit.over.2", offset_of!(Knobs, kit.over) + 16, false),
    ("kit.up.0", offset_of!(Knobs, kit.up), false),
    ("kit.up.1", offset_of!(Knobs, kit.up) + 8, false),
    ("kit.up.2", offset_of!(Knobs, kit.up) + 16, false),
    ("kit.top.fing.0", offset_of!(Knobs, kit.top.fing), false),
    ("kit.top.fing.1", offset_of!(Knobs, kit.top.fing) + 8, false),
    ("kit.top.fing.2", offset_of!(Knobs, kit.top.fing) + 16, false),
    ("kit.top.back.0", offset_of!(Knobs, kit.top.back), false),
    ("kit.top.back.1", offset_of!(Knobs, kit.top.back) + 8, false),
    ("kit.top.back.2", offset_of!(Knobs, kit.top.back) + 16, false),
    ("kit.trunk", offset_of!(Knobs, kit.trunk), false),
    ("kit.belt.at.0", offset_of!(Knobs, kit.belt.at), false),
    ("kit.belt.at.1", offset_of!(Knobs, kit.belt.at) + 8, false),
    ("kit.belt.at.2", offset_of!(Knobs, kit.belt.at) + 16, false),
    ("kit.belt.fing.0", offset_of!(Knobs, kit.belt.fing), false),
    ("kit.belt.fing.1", offset_of!(Knobs, kit.belt.fing) + 8, false),
    ("kit.belt.fing.2", offset_of!(Knobs, kit.belt.fing) + 16, false),
    ("kit.belt.back.0", offset_of!(Knobs, kit.belt.back), false),
    ("kit.belt.back.1", offset_of!(Knobs, kit.belt.back) + 8, false),
    ("kit.belt.back.2", offset_of!(Knobs, kit.belt.back) + 16, false),
    ("kit.belt.pole.0", offset_of!(Knobs, kit.belt.pole), false),
    ("kit.belt.pole.1", offset_of!(Knobs, kit.belt.pole) + 8, false),
    ("kit.belt.pole.2", offset_of!(Knobs, kit.belt.pole) + 16, false),
    ("kit.holster_pole.0", offset_of!(Knobs, kit.holster_pole), false),
    ("kit.holster_pole.1", offset_of!(Knobs, kit.holster_pole) + 8, false),
    ("kit.holster_pole.2", offset_of!(Knobs, kit.holster_pole) + 16, false),
    ("kit.sheath_pole", offset_of!(Knobs, kit.sheath_pole), false),
    ("kit.rest.0", offset_of!(Knobs, kit.rest), false),
    ("kit.rest.1", offset_of!(Knobs, kit.rest) + 8, false),
    ("kit.rest.2", offset_of!(Knobs, kit.rest) + 16, false),
    ("kit.cross.0", offset_of!(Knobs, kit.cross), false),
    ("kit.cross.1", offset_of!(Knobs, kit.cross) + 8, false),
    ("kit.cross.2", offset_of!(Knobs, kit.cross) + 16, false),
    ("kit.front.stow", offset_of!(Knobs, kit.front.stow), false),
    ("kit.front.reach", offset_of!(Knobs, kit.front.reach), false),
    ("kit.front.draw", offset_of!(Knobs, kit.front.draw), false),
    ("kit.front.hand_at", offset_of!(Knobs, kit.front.hand_at), false),
    ("kit.front.sling_at", offset_of!(Knobs, kit.front.sling_at), false),
    ("kit.front.arc.0", offset_of!(Knobs, kit.front.arc), false),
    ("kit.front.arc.1", offset_of!(Knobs, kit.front.arc) + 8, false),
    ("kit.front.arc.2", offset_of!(Knobs, kit.front.arc) + 16, false),
    ("kit.front.pole.0", offset_of!(Knobs, kit.front.pole), false),
    ("kit.front.pole.1", offset_of!(Knobs, kit.front.pole) + 8, false),
    ("kit.front.pole.2", offset_of!(Knobs, kit.front.pole) + 16, false),
    ("kit.nv.hand.dur", offset_of!(Knobs, kit.nv.hand.dur), false),
    ("kit.nv.hand.reach", offset_of!(Knobs, kit.nv.hand.reach), false),
    ("kit.nv.hand.out", offset_of!(Knobs, kit.nv.hand.out), false),
    ("kit.nv.hand.flip.0", offset_of!(Knobs, kit.nv.hand.flip), false),
    ("kit.nv.hand.flip.1", offset_of!(Knobs, kit.nv.hand.flip) + 8, false),
    ("kit.nv.hand.up.0", offset_of!(Knobs, kit.nv.hand.up), false),
    ("kit.nv.hand.up.1", offset_of!(Knobs, kit.nv.hand.up) + 8, false),
    ("kit.nv.hand.up.2", offset_of!(Knobs, kit.nv.hand.up) + 16, false),
    ("kit.nv.hand.down.0", offset_of!(Knobs, kit.nv.hand.down), false),
    ("kit.nv.hand.down.1", offset_of!(Knobs, kit.nv.hand.down) + 8, false),
    ("kit.nv.hand.down.2", offset_of!(Knobs, kit.nv.hand.down) + 16, false),
    ("kit.nv.hand.fingers.0", offset_of!(Knobs, kit.nv.hand.fingers), false),
    ("kit.nv.hand.fingers.1", offset_of!(Knobs, kit.nv.hand.fingers) + 8, false),
    ("kit.nv.hand.fingers.2", offset_of!(Knobs, kit.nv.hand.fingers) + 16, false),
    ("kit.nv.hand.back.0", offset_of!(Knobs, kit.nv.hand.back), false),
    ("kit.nv.hand.back.1", offset_of!(Knobs, kit.nv.hand.back) + 8, false),
    ("kit.nv.hand.back.2", offset_of!(Knobs, kit.nv.hand.back) + 16, false),
    ("kit.nv.hand.pole.0", offset_of!(Knobs, kit.nv.hand.pole), false),
    ("kit.nv.hand.pole.1", offset_of!(Knobs, kit.nv.hand.pole) + 8, false),
    ("kit.nv.hand.pole.2", offset_of!(Knobs, kit.nv.hand.pole) + 16, false),
    ("kit.nv.hand.shrug", offset_of!(Knobs, kit.nv.hand.shrug), false),
    ("kit.nv.hand.curl.0", offset_of!(Knobs, kit.nv.hand.curl), false),
    ("kit.nv.hand.curl.1", offset_of!(Knobs, kit.nv.hand.curl) + 8, false),
    ("kit.nv.hand.curl.2", offset_of!(Knobs, kit.nv.hand.curl) + 16, false),
    ("kit.nv.hand.curl.3", offset_of!(Knobs, kit.nv.hand.curl) + 24, false),
    ("kit.nv.hand.thumb", offset_of!(Knobs, kit.nv.hand.thumb), false),
    ("kit.nv.hand.rate", offset_of!(Knobs, kit.nv.hand.rate), false),
    ("kit.nv.hand.away", offset_of!(Knobs, kit.nv.hand.away), false),
    ("kit.nv.hand.clear", offset_of!(Knobs, kit.nv.hand.clear), false),
    ("kit.nv.nod.dur", offset_of!(Knobs, kit.nv.nod.dur), false),
    ("kit.nv.nod.pre", offset_of!(Knobs, kit.nv.nod.pre), false),
    ("kit.nv.nod.peak", offset_of!(Knobs, kit.nv.nod.peak), false),
    ("kit.nv.nod.flip.0", offset_of!(Knobs, kit.nv.nod.flip), false),
    ("kit.nv.nod.flip.1", offset_of!(Knobs, kit.nv.nod.flip) + 8, false),
    ("kit.nv.nod.on.0", offset_of!(Knobs, kit.nv.nod.on), false),
    ("kit.nv.nod.on.1", offset_of!(Knobs, kit.nv.nod.on) + 8, false),
    ("kit.nv.nod.off.0", offset_of!(Knobs, kit.nv.nod.off), false),
    ("kit.nv.nod.off.1", offset_of!(Knobs, kit.nv.nod.off) + 8, false),
    ("kit.nv.nod.carry", offset_of!(Knobs, kit.nv.nod.carry), false),
    ("kit.mag.well.0", offset_of!(Knobs, kit.mag.well), false),
    ("kit.mag.well.1", offset_of!(Knobs, kit.mag.well) + 8, false),
    ("kit.mag.well.2", offset_of!(Knobs, kit.mag.well) + 16, false),
    ("kit.mag.strip.0", offset_of!(Knobs, kit.mag.strip), false),
    ("kit.mag.strip.1", offset_of!(Knobs, kit.mag.strip) + 8, false),
    ("kit.mag.strip.2", offset_of!(Knobs, kit.mag.strip) + 16, false),
    ("kit.mag.slide.0", offset_of!(Knobs, kit.mag.slide), false),
    ("kit.mag.slide.1", offset_of!(Knobs, kit.mag.slide) + 8, false),
    ("kit.mag.slide.2", offset_of!(Knobs, kit.mag.slide) + 16, false),
    ("kit.mag.pull.0", offset_of!(Knobs, kit.mag.pull), false),
    ("kit.mag.pull.1", offset_of!(Knobs, kit.mag.pull) + 8, false),
    ("kit.mag.pull.2", offset_of!(Knobs, kit.mag.pull) + 16, false),
    ("kit.mag.pouch.0", offset_of!(Knobs, kit.mag.pouch), false),
    ("kit.mag.pouch.1", offset_of!(Knobs, kit.mag.pouch) + 8, false),
    ("kit.mag.pouch.2", offset_of!(Knobs, kit.mag.pouch) + 16, false),
    ("kit.mag.dump.0", offset_of!(Knobs, kit.mag.dump), false),
    ("kit.mag.dump.1", offset_of!(Knobs, kit.mag.dump) + 8, false),
    ("kit.mag.dump.2", offset_of!(Knobs, kit.mag.dump) + 16, false),
    ("kit.mag.via.0", offset_of!(Knobs, kit.mag.via), false),
    ("kit.mag.via.1", offset_of!(Knobs, kit.mag.via) + 8, false),
    ("kit.mag.via.2", offset_of!(Knobs, kit.mag.via) + 16, false),
    ("kit.mag.pole.0", offset_of!(Knobs, kit.mag.pole), false),
    ("kit.mag.pole.1", offset_of!(Knobs, kit.mag.pole) + 8, false),
    ("kit.mag.pole.2", offset_of!(Knobs, kit.mag.pole) + 16, false),
    ("kit.mag.pole_w", offset_of!(Knobs, kit.mag.pole_w), false),
    ("kit.mag.wf.0", offset_of!(Knobs, kit.mag.wf), false),
    ("kit.mag.wf.1", offset_of!(Knobs, kit.mag.wf) + 8, false),
    ("kit.mag.wf.2", offset_of!(Knobs, kit.mag.wf) + 16, false),
    ("kit.mag.wb.0", offset_of!(Knobs, kit.mag.wb), false),
    ("kit.mag.wb.1", offset_of!(Knobs, kit.mag.wb) + 8, false),
    ("kit.mag.wb.2", offset_of!(Knobs, kit.mag.wb) + 16, false),
    ("kit.mag.sf.0", offset_of!(Knobs, kit.mag.sf), false),
    ("kit.mag.sf.1", offset_of!(Knobs, kit.mag.sf) + 8, false),
    ("kit.mag.sf.2", offset_of!(Knobs, kit.mag.sf) + 16, false),
    ("kit.mag.sb.0", offset_of!(Knobs, kit.mag.sb), false),
    ("kit.mag.sb.1", offset_of!(Knobs, kit.mag.sb) + 8, false),
    ("kit.mag.sb.2", offset_of!(Knobs, kit.mag.sb) + 16, false),
    ("kit.close.far", offset_of!(Knobs, kit.close.far), false),
    ("kit.close.hover", offset_of!(Knobs, kit.close.hover), false),
    ("kit.close.bow.0", offset_of!(Knobs, kit.close.bow), false),
    ("kit.close.bow.1", offset_of!(Knobs, kit.close.bow) + 8, false),
    ("kit.close.bow.2", offset_of!(Knobs, kit.close.bow) + 16, false),
    ("kit.close.lead.0", offset_of!(Knobs, kit.close.lead), false),
    ("kit.close.lead.1", offset_of!(Knobs, kit.close.lead) + 8, false),
    ("kit.close.lead.2", offset_of!(Knobs, kit.close.lead) + 16, false),
    ("kit.close.left", offset_of!(Knobs, kit.close.left), false),
    ("kit.close.left_at", offset_of!(Knobs, kit.close.left_at), false),
    ("kit.close.left_in", offset_of!(Knobs, kit.close.left_in), false),
    ("kit.close.left_out", offset_of!(Knobs, kit.close.left_out), false),
    ("kit.close.rate", offset_of!(Knobs, kit.close.rate), false),
    ("kit.close.flex", offset_of!(Knobs, kit.close.flex), false),
    ("kit.close.flex_amp", offset_of!(Knobs, kit.close.flex_amp), false),
    ("kit.close.drift", offset_of!(Knobs, kit.close.drift), false),
    ("kit.close.regrip", offset_of!(Knobs, kit.close.regrip), false),
    ("kit.close.curl", offset_of!(Knobs, kit.close.curl), false),
    ("throw_by.toss", offset_of!(Knobs, throw_by.toss), false),
    ("throw_by.toss_fast", offset_of!(Knobs, throw_by.toss_fast), false),
    ("throw_by.toss_in", offset_of!(Knobs, throw_by.toss_in), false),
    ("throw_by.toss_min", offset_of!(Knobs, throw_by.toss_min), false),
    ("throw_by.through", offset_of!(Knobs, throw_by.through), false),
    ("throw_by.toss_pivot.0", offset_of!(Knobs, throw_by.toss_pivot), false),
    ("throw_by.toss_pivot.1", offset_of!(Knobs, throw_by.toss_pivot) + 8, false),
    ("throw_by.toss_pivot.2", offset_of!(Knobs, throw_by.toss_pivot) + 16, false),
    ("throw_by.full", offset_of!(Knobs, throw_by.full), false),
    ("throw_by.turn_min", offset_of!(Knobs, throw_by.turn_min), false),
    ("throw_by.arm_min", offset_of!(Knobs, throw_by.arm_min), false),
    ("throw_by.off_min", offset_of!(Knobs, throw_by.off_min), false),
    ("throw_by.sit", offset_of!(Knobs, throw_by.sit), false),
    ("throw_by.pivot.0", offset_of!(Knobs, throw_by.pivot), false),
    ("throw_by.pivot.1", offset_of!(Knobs, throw_by.pivot) + 8, false),
    ("throw_by.pivot.2", offset_of!(Knobs, throw_by.pivot) + 16, false),
    ("hands.reach_secs", offset_of!(Knobs, hands.reach_secs), false),
    ("hands.lift_secs", offset_of!(Knobs, hands.lift_secs), false),
    ("hands.set_secs", offset_of!(Knobs, hands.set_secs), false),
    ("hands.grip_m", offset_of!(Knobs, hands.grip_m), false),
    ("hands.step_secs", offset_of!(Knobs, hands.step_secs), false),
    ("hands.head_bias", offset_of!(Knobs, hands.head_bias), false),
    ("hands.ahead", offset_of!(Knobs, hands.ahead), false),
    ("hands.ahead_head", offset_of!(Knobs, hands.ahead_head), false),
    ("hands.carry_feet", offset_of!(Knobs, hands.carry_feet), false),
    ("hands.carry_head", offset_of!(Knobs, hands.carry_head), false),
    ("hands.slack_m", offset_of!(Knobs, hands.slack_m), false),
    ("hands.hoist_secs", offset_of!(Knobs, hands.hoist_secs), false),
    ("hands.lower_secs", offset_of!(Knobs, hands.lower_secs), false),
    ("hands.carry_step_secs", offset_of!(Knobs, hands.carry_step_secs), false),
    ("hands.carry_near_m", offset_of!(Knobs, hands.carry_near_m), false),
    ("hands.carry_side", offset_of!(Knobs, hands.carry_side), false),
    ("hands.carry_fwd", offset_of!(Knobs, hands.carry_fwd), false),
    ("hands.carry_up", offset_of!(Knobs, hands.carry_up), false),
    ("hands.carry_back", offset_of!(Knobs, hands.carry_back), false),
    ("hands.carry_legs", offset_of!(Knobs, hands.carry_legs), false),
    ("hands.carry_pace", offset_of!(Knobs, hands.carry_pace), false),
    ("hands.carry_turn", offset_of!(Knobs, hands.carry_turn), false),
    ("hands.carry_ride", offset_of!(Knobs, hands.carry_ride), false),
    ("hands.lay_ahead", offset_of!(Knobs, hands.lay_ahead), false),
    ("hands.door_reach", offset_of!(Knobs, hands.door_reach), false),
    ("hands.hand_secs", offset_of!(Knobs, hands.hand_secs), false),
    ("hands.door_rate", offset_of!(Knobs, hands.door_rate), false),
    ("hands.open_to", offset_of!(Knobs, hands.open_to), false),
    ("hands.clear_m", offset_of!(Knobs, hands.clear_m), false),
    ("hands.step_m", offset_of!(Knobs, hands.step_m), false),
    ("hands.round_m", offset_of!(Knobs, hands.round_m), false),
    ("hands.hand_m", offset_of!(Knobs, hands.hand_m), false),
    ("hands.in_m", offset_of!(Knobs, hands.in_m), false),
    ("hands.stall_secs", offset_of!(Knobs, hands.stall_secs), false),
    ("hands.aim_arc", offset_of!(Knobs, hands.aim_arc), false),
    ("hands.team_pre_m", offset_of!(Knobs, hands.team_pre_m), false),
    ("hands.team_reach_m", offset_of!(Knobs, hands.team_reach_m), false),
    ("hands.team_pace", offset_of!(Knobs, hands.team_pace), false),
    ("hands.team_vel", offset_of!(Knobs, hands.team_vel), false),
    ("work.pick_m", offset_of!(Knobs, work.pick_m), false),
    ("work.pick_wide_m", offset_of!(Knobs, work.pick_wide_m), false),
    ("work.behind_m", offset_of!(Knobs, work.behind_m), false),
    ("work.behind_panel_m", offset_of!(Knobs, work.behind_panel_m), false),
    ("work.stand_m", offset_of!(Knobs, work.stand_m), false),
    ("work.buddy_m", offset_of!(Knobs, work.buddy_m), false),
    ("work.hand_secs", offset_of!(Knobs, work.hand_secs), false),
    ("work.breaker_secs", offset_of!(Knobs, work.breaker_secs), false),
    ("work.fade_secs", offset_of!(Knobs, work.fade_secs), false),
    ("work.hand_m", offset_of!(Knobs, work.hand_m), false),
    ("work.walk_m", offset_of!(Knobs, work.walk_m), false),
    ("wand.feed_secs", offset_of!(Knobs, wand.feed_secs), false),
    ("wand.stow_secs", offset_of!(Knobs, wand.stow_secs), false),
    ("wand.fov_deg", offset_of!(Knobs, wand.fov_deg), false),
    ("wand.blind_deg", offset_of!(Knobs, wand.blind_deg), false),
    ("wand.pan_rate", offset_of!(Knobs, wand.pan_rate), false),
    ("wand.eye_y", offset_of!(Knobs, wand.eye_y), false),
    ("wand.past_m", offset_of!(Knobs, wand.past_m), false),
    ("wand.mark_secs", offset_of!(Knobs, wand.mark_secs), false),
    ("wand.shut_rad", offset_of!(Knobs, wand.shut_rad), false),
    ("wand.give_up_secs", offset_of!(Knobs, wand.give_up_secs), false),
    ("wand.sweep_secs", offset_of!(Knobs, wand.sweep_secs), false),
    ("wand.quick_secs", offset_of!(Knobs, wand.quick_secs), false),
    ("wand.quick_down", offset_of!(Knobs, wand.quick_down), false),
    ("sticky.carry", offset_of!(Knobs, sticky.carry), false),
    ("sticky.max", offset_of!(Knobs, sticky.max), false),
    ("sticky.live", offset_of!(Knobs, sticky.live), false),
    ("sticky.issue", offset_of!(Knobs, sticky.issue), false),
    ("sticky.speed", offset_of!(Knobs, sticky.speed), false),
    ("sticky.gravity", offset_of!(Knobs, sticky.gravity), false),
    ("sticky.drag", offset_of!(Knobs, sticky.drag), false),
    ("sticky.radius", offset_of!(Knobs, sticky.radius), false),
    ("sticky.life", offset_of!(Knobs, sticky.life), false),
    ("sticky.substeps", offset_of!(Knobs, sticky.substeps), false),
    ("sticky.solve_iters", offset_of!(Knobs, sticky.solve_iters), false),
    ("sticky.cd", offset_of!(Knobs, sticky.cd), false),
    ("sticky.aim_y", offset_of!(Knobs, sticky.aim_y), false),
    ("sticky.fov_deg", offset_of!(Knobs, sticky.fov_deg), false),
    ("sticky.off", offset_of!(Knobs, sticky.off), false),
    ("sticky.flat_y", offset_of!(Knobs, sticky.flat_y), false),
    ("sticky.launch_noise", offset_of!(Knobs, sticky.launch_noise), false),
    ("sticky.stick_noise", offset_of!(Knobs, sticky.stick_noise), false),
    ("sticky.take_reach", offset_of!(Knobs, sticky.take_reach), false),
    ("sticky.take_noise", offset_of!(Knobs, sticky.take_noise), false),
    ("peeksight.out", offset_of!(Knobs, peeksight.out), false),
    ("peeksight.off", offset_of!(Knobs, peeksight.off), false),
    ("peeksight.turn_deg", offset_of!(Knobs, peeksight.turn_deg), false),
    ("gunslots.near", offset_of!(Knobs, gunslots.near), false),
    ("gunslots.out", offset_of!(Knobs, gunslots.out), false),
    ("gunslots.lull", offset_of!(Knobs, gunslots.lull), false),
    ("gunslots.back", offset_of!(Knobs, gunslots.back), false),
    ("gunroles.akms", offset_of!(Knobs, gunroles.akms), false),
    ("gunroles.belt_lane", offset_of!(Knobs, gunroles.belt_lane), false),
    ("gunroles.belt_min", offset_of!(Knobs, gunroles.belt_min), false),
    ("gunroles.belt_squad", offset_of!(Knobs, gunroles.belt_squad), false),
    ("gunroles.line_max", offset_of!(Knobs, gunroles.line_max), false),
    ("gunroles.pistol_near", offset_of!(Knobs, gunroles.pistol_near), false),
    ("gunroles.pistol_share", offset_of!(Knobs, gunroles.pistol_share), false),
    ("gunroles.pistol_max", offset_of!(Knobs, gunroles.pistol_max), false),
    ("gunroles.lane_k", offset_of!(Knobs, gunroles.lane_k), false),
    ("gunroles.file_cost", offset_of!(Knobs, gunroles.file_cost), false),
    ("kittree.rings", offset_of!(Knobs, kittree.rings), false),
    ("kittree.lend", offset_of!(Knobs, kittree.lend), false),
    ("squadthrow.on", offset_of!(Knobs, squadthrow.on), true),
    ("squadthrow.frag_roll_k", offset_of!(Knobs, squadthrow.frag_roll_k), false),
    ("squadthrow.again_secs", offset_of!(Knobs, squadthrow.again_secs), false),
    ("squadthrow.shown_secs", offset_of!(Knobs, squadthrow.shown_secs), false),
    ("squadthrow.slack_m", offset_of!(Knobs, squadthrow.slack_m), false),
    ("squadthrow.walk_m", offset_of!(Knobs, squadthrow.walk_m), false),
    ("squadthrow.min_m", offset_of!(Knobs, squadthrow.min_m), false),
    ("squadthrow.max_m", offset_of!(Knobs, squadthrow.max_m), false),
    ("squadthrow.tries", offset_of!(Knobs, squadthrow.tries), false),
    ("squadthrow.keep_secs", offset_of!(Knobs, squadthrow.keep_secs), false),
    ("squadthrow.walk_secs", offset_of!(Knobs, squadthrow.walk_secs), false),
    ("squadthrow.turn_secs", offset_of!(Knobs, squadthrow.turn_secs), false),
    ("squadthrow.toss_secs", offset_of!(Knobs, squadthrow.toss_secs), false),
    ("squadthrow.lob_speed", offset_of!(Knobs, squadthrow.lob_speed), false),
    ("squadthrow.lob_near_m", offset_of!(Knobs, squadthrow.lob_near_m), false),
    ("squadthrow.lob_up_m", offset_of!(Knobs, squadthrow.lob_up_m), false),
    ("squadthrow.lob_walls", offset_of!(Knobs, squadthrow.lob_walls), false),
    ("squadthrow.lob_wall_m", offset_of!(Knobs, squadthrow.lob_wall_m), false),
    ("squadthrow.lob_wall_h", offset_of!(Knobs, squadthrow.lob_wall_h), false),
    ("squadthrow.lob_sims", offset_of!(Knobs, squadthrow.lob_sims), false),
    ("squadthrow.lob_cells", offset_of!(Knobs, squadthrow.lob_cells), false),
    ("squadthrow.lob_keep_secs", offset_of!(Knobs, squadthrow.lob_keep_secs), false),
    ("squadthrow.settle_secs", offset_of!(Knobs, squadthrow.settle_secs), false),
    ("squadthrow.duck_secs", offset_of!(Knobs, squadthrow.duck_secs), false),
    ("squadthrow.duck_throw_secs", offset_of!(Knobs, squadthrow.duck_throw_secs), false),
    ("squadthrow.duck_cells", offset_of!(Knobs, squadthrow.duck_cells), false),
    ("squadthrow.duck_apart_m", offset_of!(Knobs, squadthrow.duck_apart_m), false),
    ("squadthrow.duck_min_m", offset_of!(Knobs, squadthrow.duck_min_m), false),
    ("squad40.near_m", offset_of!(Knobs, squad40.near_m), false),
    ("squad40.arm_slack", offset_of!(Knobs, squad40.arm_slack), false),
    ("squad40.reach_slack", offset_of!(Knobs, squad40.reach_slack), false),
    ("squad40.muzzle_ahead", offset_of!(Knobs, squad40.muzzle_ahead), false),
    ("squad40.muzzle_up", offset_of!(Knobs, squad40.muzzle_up), false),
    ("squad40.head_drop", offset_of!(Knobs, squad40.head_drop), false),
    ("squad40.aim_rad", offset_of!(Knobs, squad40.aim_rad), false),
    ("squad40.aim_secs", offset_of!(Knobs, squad40.aim_secs), false),
    ("squad40.raise_secs", offset_of!(Knobs, squad40.raise_secs), false),
    ("squad40.after_secs", offset_of!(Knobs, squad40.after_secs), false),
    ("squad40.shown_secs", offset_of!(Knobs, squad40.shown_secs), false),
    ("autopilot.show_secs", offset_of!(Knobs, autopilot.show_secs), false),
    ("autopilot.keep_bangs", offset_of!(Knobs, autopilot.keep_bangs), false),
    ("autopilot.contact_m", offset_of!(Knobs, autopilot.contact_m), false),
    ("cmdring.wedges", offset_of!(Knobs, cmdring.wedges), false),
    ("cmdring.flick_px", offset_of!(Knobs, cmdring.flick_px), false),
    ("cmdring.hold_secs", offset_of!(Knobs, cmdring.hold_secs), false),
    ("cmdring.stay_hold_secs", offset_of!(Knobs, cmdring.stay_hold_secs), false),
    ("cmdring.tap_secs", offset_of!(Knobs, cmdring.tap_secs), false),
    ("cmdring.edge_x", offset_of!(Knobs, cmdring.edge_x), false),
    ("cmdring.edge_y", offset_of!(Knobs, cmdring.edge_y), false),
    ("cmdring.man_px", offset_of!(Knobs, cmdring.man_px), false),
    ("cmdring.ref_.0", offset_of!(Knobs, cmdring.ref_), false),
    ("cmdring.ref_.1", offset_of!(Knobs, cmdring.ref_) + 8, false),
    ("cmdring.hint", offset_of!(Knobs, cmdring.hint), true),
    ("cmdring.goal_half", offset_of!(Knobs, cmdring.goal_half), true),
    ("cmdring.twice", offset_of!(Knobs, cmdring.twice), true),
    ("cmdring.twice_secs", offset_of!(Knobs, cmdring.twice_secs), false),
    ("cmdring.twice_px", offset_of!(Knobs, cmdring.twice_px), false),
    ("cmdring.hint_wall_m", offset_of!(Knobs, cmdring.hint_wall_m), false),
    ("cmdring.hint_spot_m", offset_of!(Knobs, cmdring.hint_spot_m), false),
    ("cmdring.hint_man_m", offset_of!(Knobs, cmdring.hint_man_m), false),
    ("torch.lit_at", offset_of!(Knobs, torch.lit_at), false),
    ("torch.off_k", offset_of!(Knobs, torch.off_k), false),
    ("torch.spill_m", offset_of!(Knobs, torch.spill_m), false),
    ("torch.day_m", offset_of!(Knobs, torch.day_m), false),
    ("torch.ahead_m", offset_of!(Knobs, torch.ahead_m), false),
    ("torch.on_secs", offset_of!(Knobs, torch.on_secs), false),
    ("torch.off_secs", offset_of!(Knobs, torch.off_secs), false),
    ("torch.hold_secs", offset_of!(Knobs, torch.hold_secs), false),
    ("torch.ask_secs", offset_of!(Knobs, torch.ask_secs), false),
    ("torch.calm_aw", offset_of!(Knobs, torch.calm_aw), false),
    ("torch.wall_m", offset_of!(Knobs, torch.wall_m), false),
    ("torch.wall_back_m", offset_of!(Knobs, torch.wall_back_m), false),
    ("torch.low_tan", offset_of!(Knobs, torch.low_tan), false),
    ("torch.low_rate", offset_of!(Knobs, torch.low_rate), false),
    ("morale.half_life", offset_of!(Knobs, morale.half_life), false),
    ("morale.ask_secs", offset_of!(Knobs, morale.ask_secs), false),
    ("morale.alone", offset_of!(Knobs, morale.alone), false),
    ("morale.alone_m", offset_of!(Knobs, morale.alone_m), false),
    ("morale.dark", offset_of!(Knobs, morale.dark), false),
    ("morale.lit_asks", offset_of!(Knobs, morale.lit_asks), false),
    ("morale.blast", offset_of!(Knobs, morale.blast), false),
    ("morale.shock_cap", offset_of!(Knobs, morale.shock_cap), false),
    ("morale.blast_m", offset_of!(Knobs, morale.blast_m), false),
    ("morale.near_m", offset_of!(Knobs, morale.near_m), false),
    ("morale.far_k", offset_of!(Knobs, morale.far_k), false),
    ("morale.jumpy", offset_of!(Knobs, morale.jumpy), false),
    ("morale.broken", offset_of!(Knobs, morale.broken), false),
    ("morale.hyst", offset_of!(Knobs, morale.hyst), false),
    ("morale.duck_secs", offset_of!(Knobs, morale.duck_secs), false),
    ("morale.down_secs", offset_of!(Knobs, morale.down_secs), false),
    ("morale.back_until", offset_of!(Knobs, morale.back_until), false),
    ("morale.back_m", offset_of!(Knobs, morale.back_m), false),
    ("morale.back_speed", offset_of!(Knobs, morale.back_speed), false),
    ("morale.burst_rounds", offset_of!(Knobs, morale.burst_rounds), false),
    ("morale.burst_every", offset_of!(Knobs, morale.burst_every), false),
    ("morale.spread", offset_of!(Knobs, morale.spread), false),
    ("morale.call2_secs", offset_of!(Knobs, morale.call2_secs), false),
    ("morale.bait_m", offset_of!(Knobs, morale.bait_m), false),
    ("morale.bait_short", offset_of!(Knobs, morale.bait_short), false),
    ("morale.bait_every", offset_of!(Knobs, morale.bait_every), false),
    ("morale.reload_secs", offset_of!(Knobs, morale.reload_secs), false),
    ("morale.bark_secs", offset_of!(Knobs, morale.bark_secs), false),
    ("morale.fan_deg", offset_of!(Knobs, morale.fan_deg), false),
    ("morale.fan_step", offset_of!(Knobs, morale.fan_step), false),
    ("morale.straight_m", offset_of!(Knobs, morale.straight_m), false),
    ("morale.green_broken", offset_of!(Knobs, morale.green_broken), false),
    ("morale.leader_down", offset_of!(Knobs, morale.leader_down), false),
    ("morale.leader_shock", offset_of!(Knobs, morale.leader_shock), false),
    ("morale.flinch_secs", offset_of!(Knobs, morale.flinch_secs), false),
    ("morale.shout_gap", offset_of!(Knobs, morale.shout_gap), false),
    ("morale.order_m", offset_of!(Knobs, morale.order_m), false),
    ("morale.order_beat", offset_of!(Knobs, morale.order_beat), false),
    ("morale.order_secs", offset_of!(Knobs, morale.order_secs), false),
    ("morale.order_gap", offset_of!(Knobs, morale.order_gap), false),
    ("morale.stand_secs", offset_of!(Knobs, morale.stand_secs), false),
    ("morale.side_m", offset_of!(Knobs, morale.side_m), false),
    ("morale.steady_duck", offset_of!(Knobs, morale.steady_duck), false),
    ("morale.hold_shock", offset_of!(Knobs, morale.hold_shock), false),
    ("morale.hold_max", offset_of!(Knobs, morale.hold_max), false),
    ("morale.nerve_k.brittle", offset_of!(Knobs, morale.nerve_k.brittle), false),
    ("morale.nerve_k.normal", offset_of!(Knobs, morale.nerve_k.normal), false),
    ("morale.nerve_k.steady", offset_of!(Knobs, morale.nerve_k.steady), false),
    ("condition.hurt_pace", offset_of!(Knobs, condition.hurt_pace), false),
    ("condition.hurt_sup", offset_of!(Knobs, condition.hurt_sup), false),
    ("condition.hurt_secs", offset_of!(Knobs, condition.hurt_secs), false),
    ("inhand.from", offset_of!(Knobs, inhand.from), false),
    ("inhand.to", offset_of!(Knobs, inhand.to), false),
    ("inhand.wrist", offset_of!(Knobs, inhand.wrist), false),
    ("inhand.both", offset_of!(Knobs, inhand.both), false),
    ("inhand.rate", offset_of!(Knobs, inhand.rate), false),
    ("inhand.holster_out", offset_of!(Knobs, inhand.holster_out), false),
    ("packside.gap", offset_of!(Knobs, packside.gap), false),
    ("packside.handle", offset_of!(Knobs, packside.handle), false),
    ("packside.gun", offset_of!(Knobs, packside.gun), false),
    ("packside.over", offset_of!(Knobs, packside.over), false),
    ("packside.head_min", offset_of!(Knobs, packside.head_min), false),
    ("packside.butt", offset_of!(Knobs, packside.butt), false),
    ("packside.rest", offset_of!(Knobs, packside.rest), false),
    ("packside.pitch0", offset_of!(Knobs, packside.pitch0), false),
    ("packside.pitch1", offset_of!(Knobs, packside.pitch1), false),
    ("runner.close_m", offset_of!(Knobs, runner.close_m), false),
    ("runner.out_secs", offset_of!(Knobs, runner.out_secs), false),
    ("runner.beat_secs", offset_of!(Knobs, runner.beat_secs), false),
    ("runner.tell_m", offset_of!(Knobs, runner.tell_m), false),
    ("runner.told_m", offset_of!(Knobs, runner.told_m), false),
    ("runner.tell_beat", offset_of!(Knobs, runner.tell_beat), false),
    ("runner.tell_aware", offset_of!(Knobs, runner.tell_aware), false),
    ("runner.shy_m", offset_of!(Knobs, runner.shy_m), false),
    ("runner.mate_m", offset_of!(Knobs, runner.mate_m), false),
    ("runner.mate_secs", offset_of!(Knobs, runner.mate_secs), false),
    ("runner.clear_m", offset_of!(Knobs, runner.clear_m), false),
    ("runner.edge_m", offset_of!(Knobs, runner.edge_m), false),
    ("runner.corner_m", offset_of!(Knobs, runner.corner_m), false),
    ("runner.gone_secs", offset_of!(Knobs, runner.gone_secs), false),
    ("runner.stall_secs", offset_of!(Knobs, runner.stall_secs), false),
    ("runner.retry_secs", offset_of!(Knobs, runner.retry_secs), false),
    ("runner.retries", offset_of!(Knobs, runner.retries), false),
    ("runner.replan_secs", offset_of!(Knobs, runner.replan_secs), false),
    ("runner.ahead_m", offset_of!(Knobs, runner.ahead_m), false),
    ("runner.ask_cells", offset_of!(Knobs, runner.ask_cells), false),
    ("runner.ask_wide", offset_of!(Knobs, runner.ask_wide), false),
    ("startle.beat", offset_of!(Knobs, startle.beat), false),
    ("startle.draw", offset_of!(Knobs, startle.draw), false),
    ("startle.fresh", offset_of!(Knobs, startle.fresh), false),
    ("startle.late", offset_of!(Knobs, startle.late), false),
    ("startle.run_m", offset_of!(Knobs, startle.run_m), false),
    ("startle.near_m", offset_of!(Knobs, startle.near_m), false),
    ("startle.clear_secs", offset_of!(Knobs, startle.clear_secs), false),
    ("startle.run_secs", offset_of!(Knobs, startle.run_secs), false),
    ("startle.stuck_secs", offset_of!(Knobs, startle.stuck_secs), false),
    ("startle.stuck_m", offset_of!(Knobs, startle.stuck_m), false),
    ("smokebreak.light_secs", offset_of!(Knobs, smokebreak.light_secs), false),
    ("smokebreak.light_up.0", offset_of!(Knobs, smokebreak.light_up), false),
    ("smokebreak.light_up.1", offset_of!(Knobs, smokebreak.light_up) + 8, false),
    ("smokebreak.flame_at", offset_of!(Knobs, smokebreak.flame_at), false),
    ("smokebreak.flame", offset_of!(Knobs, smokebreak.flame), false),
    ("smokebreak.every", offset_of!(Knobs, smokebreak.every), false),
    ("smokebreak.draw", offset_of!(Knobs, smokebreak.draw), false),
    ("smokebreak.out_after", offset_of!(Knobs, smokebreak.out_after), false),
    ("smokebreak.out", offset_of!(Knobs, smokebreak.out), false),
    ("smokebreak.flame_color.0", offset_of!(Knobs, smokebreak.flame_color), false),
    ("smokebreak.flame_color.1", offset_of!(Knobs, smokebreak.flame_color) + 8, false),
    ("smokebreak.flame_color.2", offset_of!(Knobs, smokebreak.flame_color) + 16, false),
    ("smokebreak.flame_k", offset_of!(Knobs, smokebreak.flame_k), false),
    ("smokebreak.flame_range", offset_of!(Knobs, smokebreak.flame_range), false),
    ("smokebreak.ember_color.0", offset_of!(Knobs, smokebreak.ember_color), false),
    ("smokebreak.ember_color.1", offset_of!(Knobs, smokebreak.ember_color) + 8, false),
    ("smokebreak.ember_color.2", offset_of!(Knobs, smokebreak.ember_color) + 16, false),
    ("smokebreak.ember_k", offset_of!(Knobs, smokebreak.ember_k), false),
    ("smokebreak.ember_draw_k", offset_of!(Knobs, smokebreak.ember_draw_k), false),
    ("smokebreak.ember_range", offset_of!(Knobs, smokebreak.ember_range), false),
    ("smokebreak.ember_every", offset_of!(Knobs, smokebreak.ember_every), false),
    ("smokebreak.tip_out", offset_of!(Knobs, smokebreak.tip_out), false),
    ("smokebreak.tip_up", offset_of!(Knobs, smokebreak.tip_up), false),
    ("smokebreak.thread.radius", offset_of!(Knobs, smokebreak.thread.radius), false),
    ("smokebreak.thread.cells", offset_of!(Knobs, smokebreak.thread.cells), false),
    ("smokebreak.thread.density", offset_of!(Knobs, smokebreak.thread.density), false),
    ("smokebreak.thread.temp", offset_of!(Knobs, smokebreak.thread.temp), false),
    ("smokebreak.thread.speed", offset_of!(Knobs, smokebreak.thread.speed), false),
    ("smokebreak.thread.push", offset_of!(Knobs, smokebreak.thread.push), false),
    ("smokebreak.thread.jitter", offset_of!(Knobs, smokebreak.thread.jitter), false),
    ("smokebreak.thread_gone", offset_of!(Knobs, smokebreak.thread_gone), false),
    ("smokebreak.breath.radius", offset_of!(Knobs, smokebreak.breath.radius), false),
    ("smokebreak.breath.cells", offset_of!(Knobs, smokebreak.breath.cells), false),
    ("smokebreak.breath.density", offset_of!(Knobs, smokebreak.breath.density), false),
    ("smokebreak.breath.temp", offset_of!(Knobs, smokebreak.breath.temp), false),
    ("smokebreak.breath.speed", offset_of!(Knobs, smokebreak.breath.speed), false),
    ("smokebreak.breath.push", offset_of!(Knobs, smokebreak.breath.push), false),
    ("smokebreak.breath.expand", offset_of!(Knobs, smokebreak.breath.expand), false),
    ("smokebreak.breath.attack", offset_of!(Knobs, smokebreak.breath.attack), false),
    ("smokebreak.breath.jitter", offset_of!(Knobs, smokebreak.breath.jitter), false),
    ("smokebreak.breath.down", offset_of!(Knobs, smokebreak.breath.down), false),
    ("smokebreak.puffs.records", offset_of!(Knobs, smokebreak.puffs.records), false),
    ("smokebreak.puffs.count", offset_of!(Knobs, smokebreak.puffs.count), false),
    ("smokebreak.puffs.speed", offset_of!(Knobs, smokebreak.puffs.speed), false),
    ("smokebreak.puffs.spread", offset_of!(Knobs, smokebreak.puffs.spread), false),
    ("smokebreak.puffs.life", offset_of!(Knobs, smokebreak.puffs.life), false),
    ("smokebreak.puffs.size", offset_of!(Knobs, smokebreak.puffs.size), false),
    ("smokebreak.puffs.opacity", offset_of!(Knobs, smokebreak.puffs.opacity), false),
    ("smokebreak.color.0", offset_of!(Knobs, smokebreak.color), false),
    ("smokebreak.color.1", offset_of!(Knobs, smokebreak.color) + 8, false),
    ("smokebreak.color.2", offset_of!(Knobs, smokebreak.color) + 16, false),
    ("smokebreak.mouth_ahead", offset_of!(Knobs, smokebreak.mouth_ahead), false),
    ("smokebreak.mouth_down", offset_of!(Knobs, smokebreak.mouth_down), false),
    ("showwhy.line_after", offset_of!(Knobs, showwhy.line_after), false),
    ("showwhy.line_gap", offset_of!(Knobs, showwhy.line_gap), false),
    ("showwhy.wave_every", offset_of!(Knobs, showwhy.wave_every), false),
    ("showwhy.wave_later", offset_of!(Knobs, showwhy.wave_later), false),
    ("showwhy.line_min", offset_of!(Knobs, showwhy.line_min), false),
    ("showwhy.line_say_gap", offset_of!(Knobs, showwhy.line_say_gap), false),
    ("showwhy.mate_half", offset_of!(Knobs, showwhy.mate_half), false),
    ("showwhy.squeeze_gap", offset_of!(Knobs, showwhy.squeeze_gap), false),
    ("showwhy.squeeze_reach", offset_of!(Knobs, showwhy.squeeze_reach), false),
    ("showwhy.fist_after", offset_of!(Knobs, showwhy.fist_after), false),
    ("showwhy.hold_every", offset_of!(Knobs, showwhy.hold_every), false),
    ("showwhy.hold_max", offset_of!(Knobs, showwhy.hold_max), false),
    ("showwhy.door_after", offset_of!(Knobs, showwhy.door_after), false),
    ("showwhy.door_every", offset_of!(Knobs, showwhy.door_every), false),
    ("showwhy.after_you_gap", offset_of!(Knobs, showwhy.after_you_gap), false),
    ("showwhy.hurt_late", offset_of!(Knobs, showwhy.hurt_late), false),
    ("showwhy.lag_m", offset_of!(Knobs, showwhy.lag_m), false),
    ("showwhy.lag_every", offset_of!(Knobs, showwhy.lag_every), false),
    ("showwhy.lag_max", offset_of!(Knobs, showwhy.lag_max), false),
    ("showwhy.keep_after", offset_of!(Knobs, showwhy.keep_after), false),
    ("showwhy.keep_gap", offset_of!(Knobs, showwhy.keep_gap), false),
    ("showwhy.keep_every", offset_of!(Knobs, showwhy.keep_every), false),
    ("showwhy.owed_for", offset_of!(Knobs, showwhy.owed_for), false),
    ("fire.height", offset_of!(Knobs, fire.height), false),
    ("fire.mouth", offset_of!(Knobs, fire.mouth), false),
    ("fire.coals", offset_of!(Knobs, fire.coals), false),
    ("fire.coal_swing", offset_of!(Knobs, fire.coal_swing), false),
    ("fire.bed_r", offset_of!(Knobs, fire.bed_r), false),
    ("fire.depth", offset_of!(Knobs, fire.depth), false),
    ("fire.least", offset_of!(Knobs, fire.least), false),
    ("fire.most", offset_of!(Knobs, fire.most), false),
    ("fire.wander", offset_of!(Knobs, fire.wander), false),
    ("fire.wander_up", offset_of!(Knobs, fire.wander_up), false),
    ("fire.lift", offset_of!(Knobs, fire.lift), false),
    ("fire.kick", offset_of!(Knobs, fire.kick), false),
    ("fire.kick_secs", offset_of!(Knobs, fire.kick_secs), false),
    ("fire.bright", offset_of!(Knobs, fire.bright), false),
    ("fire.reach", offset_of!(Knobs, fire.reach), false),
    ("fire.stir_gap", offset_of!(Knobs, fire.stir_gap), false),
    ("fire.size", offset_of!(Knobs, fire.size), false),
    ("fire.pop_rate", offset_of!(Knobs, fire.pop_rate), false),
    ("fire.pop_slot", offset_of!(Knobs, fire.pop_slot), false),
    ("fire.embers", offset_of!(Knobs, fire.embers), false),
    ("fire.ember_life", offset_of!(Knobs, fire.ember_life), false),
    ("fire.ember_speed", offset_of!(Knobs, fire.ember_speed), false),
    ("fire.ember_size", offset_of!(Knobs, fire.ember_size), false),
    ("fire.ember_gain", offset_of!(Knobs, fire.ember_gain), false),
    ("fire.ember_crack", offset_of!(Knobs, fire.ember_crack), false),
    ("fire.ember_r", offset_of!(Knobs, fire.ember_r), false),
    ("fire.look.gain", offset_of!(Knobs, fire.look.gain), false),
    ("fire.look.core", offset_of!(Knobs, fire.look.core), false),
    ("fire.look.sigma", offset_of!(Knobs, fire.look.sigma), false),
    ("fire.look.speed", offset_of!(Knobs, fire.look.speed), false),
    ("fire.look.tall", offset_of!(Knobs, fire.look.tall), false),
    ("fire.look.torn", offset_of!(Knobs, fire.look.torn), false),
    ("fire.look.plume", offset_of!(Knobs, fire.look.plume), false),
    ("fire.look.shimmer", offset_of!(Knobs, fire.look.shimmer), false),
    ("fire.look.soot", offset_of!(Knobs, fire.look.soot), false),
    ("fire.look.breathe", offset_of!(Knobs, fire.look.breathe), false),
    ("fire.look.react", offset_of!(Knobs, fire.look.react), false),
    ("fire.look.lean", offset_of!(Knobs, fire.look.lean), false),
    ("fire.look.steps", offset_of!(Knobs, fire.look.steps), false),
    ("fire.smoke.up", offset_of!(Knobs, fire.smoke.up), false),
    ("fire.smoke.radius", offset_of!(Knobs, fire.smoke.radius), false),
    ("fire.smoke.cells", offset_of!(Knobs, fire.smoke.cells), false),
    ("fire.smoke.density", offset_of!(Knobs, fire.smoke.density), false),
    ("fire.smoke.temp", offset_of!(Knobs, fire.smoke.temp), false),
    ("fire.smoke.speed", offset_of!(Knobs, fire.smoke.speed), false),
    ("fire.smoke.push", offset_of!(Knobs, fire.smoke.push), false),
    ("fire.smoke.jitter", offset_of!(Knobs, fire.smoke.jitter), false),
    ("fire.smoke.every", offset_of!(Knobs, fire.smoke.every), false),
    ("fire.smoke.view_r", offset_of!(Knobs, fire.smoke.view_r), false),
    ("fire.smoke.lean", offset_of!(Knobs, fire.smoke.lean), false),
    ("fire.gas.up", offset_of!(Knobs, fire.gas.up), false),
    ("fire.gas.radius", offset_of!(Knobs, fire.gas.radius), false),
    ("fire.gas.cells", offset_of!(Knobs, fire.gas.cells), false),
    ("fire.gas.density", offset_of!(Knobs, fire.gas.density), false),
    ("fire.gas.temp", offset_of!(Knobs, fire.gas.temp), false),
    ("fire.gas.speed", offset_of!(Knobs, fire.gas.speed), false),
    ("fire.gas.push", offset_of!(Knobs, fire.gas.push), false),
    ("fire.gas.jitter", offset_of!(Knobs, fire.gas.jitter), false),
    ("fire.gas.every", offset_of!(Knobs, fire.gas.every), false),
    ("fire.gas.view_r", offset_of!(Knobs, fire.gas.view_r), false),
    ("fire.gas.keep", offset_of!(Knobs, fire.gas.keep), false),
    ("fire.gas.ref_", offset_of!(Knobs, fire.gas.ref_), false),
    ("fire.gas.lo", offset_of!(Knobs, fire.gas.lo), false),
    ("fire.gas.ease", offset_of!(Knobs, fire.gas.ease), false),
    ("fire.bed", offset_of!(Knobs, fire.bed), false),
    ("fire.crack", offset_of!(Knobs, fire.crack), false),
    ("fire.bed_secs", offset_of!(Knobs, fire.bed_secs), false),
    ("fire.max", offset_of!(Knobs, fire.max), false),
    ("fire.draw_r", offset_of!(Knobs, fire.draw_r), false),
    ("fire.wind_max", offset_of!(Knobs, fire.wind_max), false),
    ("flinch.least", offset_of!(Knobs, flinch.least), false),
    ("flinch.full", offset_of!(Knobs, flinch.full), false),
    ("flinch.far", offset_of!(Knobs, flinch.far), false),
    ("flinch.min_travel", offset_of!(Knobs, flinch.min_travel), false),
    ("flinch.steady_k", offset_of!(Knobs, flinch.steady_k), false),
    ("flinch.break_secs", offset_of!(Knobs, flinch.break_secs), false),
    ("flinch.hot_secs", offset_of!(Knobs, flinch.hot_secs), false),
    ("flinch.ease_k", offset_of!(Knobs, flinch.ease_k), false),
    ("flinch.ask_secs", offset_of!(Knobs, flinch.ask_secs), false),
    ("darkdoor.stop_m", offset_of!(Knobs, darkdoor.stop_m), false),
    ("darkdoor.pair_m", offset_of!(Knobs, darkdoor.pair_m), false),
    ("darkdoor.dark_at", offset_of!(Knobs, darkdoor.dark_at), false),
    ("darkdoor.dark_share", offset_of!(Knobs, darkdoor.dark_share), false),
    ("darkdoor.lit_by", offset_of!(Knobs, darkdoor.lit_by), false),
    ("darkdoor.samples", offset_of!(Knobs, darkdoor.samples), false),
    ("darkdoor.ask_secs", offset_of!(Knobs, darkdoor.ask_secs), false),
    ("darkdoor.open_rad", offset_of!(Knobs, darkdoor.open_rad), false),
    ("darkdoor.arm_secs", offset_of!(Knobs, darkdoor.arm_secs), false),
    ("darkdoor.stand_m", offset_of!(Knobs, darkdoor.stand_m), false),
    ("darkdoor.side_m", offset_of!(Knobs, darkdoor.side_m), false),
    ("darkdoor.sweep_secs", offset_of!(Knobs, darkdoor.sweep_secs), false),
    ("darkdoor.ear_m", offset_of!(Knobs, darkdoor.ear_m), false),
    ("darkdoor.mate_m", offset_of!(Knobs, darkdoor.mate_m), false),
    ("darkdoor.mate_try", offset_of!(Knobs, darkdoor.mate_try), false),
    ("darkdoor.call_secs", offset_of!(Knobs, darkdoor.call_secs), false),
    ("darkdoor.copy_secs", offset_of!(Knobs, darkdoor.copy_secs), false),
    ("darkdoor.lag_secs", offset_of!(Knobs, darkdoor.lag_secs), false),
    ("darkdoor.with_secs", offset_of!(Knobs, darkdoor.with_secs), false),
    ("darkdoor.keep_aw", offset_of!(Knobs, darkdoor.keep_aw), false),
    ("darkdoor.keep_secs", offset_of!(Knobs, darkdoor.keep_secs), false),
    ("darkdoor.resume_secs", offset_of!(Knobs, darkdoor.resume_secs), false),
    ("darkdoor.seen_secs", offset_of!(Knobs, darkdoor.seen_secs), false),
    ("darkdoor.wait_secs", offset_of!(Knobs, darkdoor.wait_secs), false),
    ("darkdoor.again_secs", offset_of!(Knobs, darkdoor.again_secs), false),
    ("darkdoor.fed_up_secs", offset_of!(Knobs, darkdoor.fed_up_secs), false),
    ("darkdoor.fed_up_n", offset_of!(Knobs, darkdoor.fed_up_n), false),
    ("darkdoor.nerve_secs", offset_of!(Knobs, darkdoor.nerve_secs), false),
    ("darkdoor.slow_k", offset_of!(Knobs, darkdoor.slow_k), false),
    ("darkfoe.carry", offset_of!(Knobs, darkfoe.carry), false),
    ("darkfoe.lit_at", offset_of!(Knobs, darkfoe.lit_at), false),
    ("darkfoe.look_m", offset_of!(Knobs, darkfoe.look_m), false),
    ("darkfoe.step_m", offset_of!(Knobs, darkfoe.step_m), false),
    ("darkfoe.fan_rad", offset_of!(Knobs, darkfoe.fan_rad), false),
    ("darkfoe.see_share", offset_of!(Knobs, darkfoe.see_share), false),
    ("darkfoe.ask_secs", offset_of!(Knobs, darkfoe.ask_secs), false),
    ("darkfoe.on_secs", offset_of!(Knobs, darkfoe.on_secs), false),
    ("darkfoe.on_jitter", offset_of!(Knobs, darkfoe.on_jitter), false),
    ("darkfoe.off_secs", offset_of!(Knobs, darkfoe.off_secs), false),
    ("darkfoe.off_jitter", offset_of!(Knobs, darkfoe.off_jitter), false),
    ("darkfoe.hold_secs", offset_of!(Knobs, darkfoe.hold_secs), false),
    ("darkfoe.lamp_m", offset_of!(Knobs, darkfoe.lamp_m), false),
    ("darkfoe.blind_secs", offset_of!(Knobs, darkfoe.blind_secs), false),
    ("darkfoe.seek_m", offset_of!(Knobs, darkfoe.seek_m), false),
    ("darkfoe.walk_m", offset_of!(Knobs, darkfoe.walk_m), false),
    ("darkfoe.edge_m", offset_of!(Knobs, darkfoe.edge_m), false),
    ("darkfoe.stumble_k", offset_of!(Knobs, darkfoe.stumble_k), false),
    ("darkfoe.again_secs", offset_of!(Knobs, darkfoe.again_secs), false),
    ("darkfoe.moves", offset_of!(Knobs, darkfoe.moves), false),
    ("darkfoe.quiet_secs", offset_of!(Knobs, darkfoe.quiet_secs), false),
    ("darkfoe.calm_secs", offset_of!(Knobs, darkfoe.calm_secs), false),
    ("powerout.see_m", offset_of!(Knobs, powerout.see_m), false),
    ("powerout.think.0", offset_of!(Knobs, powerout.think), false),
    ("powerout.think.1", offset_of!(Knobs, powerout.think) + 8, false),
    ("powerout.beat", offset_of!(Knobs, powerout.beat), false),
    ("powerout.look_m", offset_of!(Knobs, powerout.look_m), false),
    ("powerout.about_m", offset_of!(Knobs, powerout.about_m), false),
    ("powerout.step_m", offset_of!(Knobs, powerout.step_m), false),
    ("powerout.fan_rad", offset_of!(Knobs, powerout.fan_rad), false),
    ("powerout.lit_at", offset_of!(Knobs, powerout.lit_at), false),
    ("powerout.drop_k", offset_of!(Knobs, powerout.drop_k), false),
    ("powerout.dark_after", offset_of!(Knobs, powerout.dark_after), false),
    ("powerout.men", offset_of!(Knobs, powerout.men), false),
    ("powerout.max_m", offset_of!(Knobs, powerout.max_m), false),
    ("powerout.parties", offset_of!(Knobs, powerout.parties), false),
    ("powerout.look_secs", offset_of!(Knobs, powerout.look_secs), false),
    ("powerout.tries", offset_of!(Knobs, powerout.tries), false),
    ("powerout.lever_secs", offset_of!(Knobs, powerout.lever_secs), false),
    ("powerout.overdue", offset_of!(Knobs, powerout.overdue), false),
    ("powerout.ask", offset_of!(Knobs, powerout.ask), false),
    ("powerout.cover_m.0", offset_of!(Knobs, powerout.cover_m), false),
    ("powerout.cover_m.1", offset_of!(Knobs, powerout.cover_m) + 8, false),
    ("squaddeal.one", offset_of!(Knobs, squaddeal.one), false),
    ("squaddeal.two", offset_of!(Knobs, squaddeal.two), false),
    ("stayask.men", offset_of!(Knobs, stayask.men), false),
    ("rescuecheck.first.0", offset_of!(Knobs, rescuecheck.first), false),
    ("rescuecheck.first.1", offset_of!(Knobs, rescuecheck.first) + 8, false),
    ("rescuecheck.every.0", offset_of!(Knobs, rescuecheck.every), false),
    ("rescuecheck.every.1", offset_of!(Knobs, rescuecheck.every) + 8, false),
    ("rescuecheck.ask_secs", offset_of!(Knobs, rescuecheck.ask_secs), false),
    ("rescuecheck.go_secs", offset_of!(Knobs, rescuecheck.go_secs), false),
    ("rescuecheck.wait_secs", offset_of!(Knobs, rescuecheck.wait_secs), false),
    ("rescuecheck.tries", offset_of!(Knobs, rescuecheck.tries), false),
    ("rescuecheck.feet_m", offset_of!(Knobs, rescuecheck.feet_m), false),
    ("rescuecheck.word_secs", offset_of!(Knobs, rescuecheck.word_secs), false),
    ("kitload.gun_kg", offset_of!(Knobs, kitload.gun_kg), false),
    ("kitload.gun_bulk.0", offset_of!(Knobs, kitload.gun_bulk), false),
    ("kitload.gun_bulk.1", offset_of!(Knobs, kitload.gun_bulk) + 8, false),
    ("kitload.pace", offset_of!(Knobs, kitload.pace), false),
    ("kitload.seen", offset_of!(Knobs, kitload.seen), false),
    ("kitload.heard", offset_of!(Knobs, kitload.heard), false),
    ("kitload.sprint", offset_of!(Knobs, kitload.sprint), false),
    ("kitload.pace_top", offset_of!(Knobs, kitload.pace_top), false),
    ("rescueend.out_m", offset_of!(Knobs, rescueend.out_m), false),
    ("rescueend.clear_secs", offset_of!(Knobs, rescueend.clear_secs), false),
    ("rescueend.marker_m", offset_of!(Knobs, rescueend.marker_m), false),
    ("rescueend.reach_m", offset_of!(Knobs, rescueend.reach_m), false),
    ("rescueend.gap_m", offset_of!(Knobs, rescueend.gap_m), false),
    ("rescueend.gap_step_m", offset_of!(Knobs, rescueend.gap_step_m), false),
    ("rescueflush.after_secs", offset_of!(Knobs, rescueflush.after_secs), false),
    ("rescueflush.gap_secs", offset_of!(Knobs, rescueflush.gap_secs), false),
    ("rescueflush.again_secs", offset_of!(Knobs, rescueflush.again_secs), false),
    ("rescueflush.reach_m", offset_of!(Knobs, rescueflush.reach_m), false),
    ("rescueflush.kill_secs", offset_of!(Knobs, rescueflush.kill_secs), false),
    ("rescueflush.busy_secs", offset_of!(Knobs, rescueflush.busy_secs), false),
    ("rescueflush.smoke_k", offset_of!(Knobs, rescueflush.smoke_k), false),
    ("rescueflush.at_door_m", offset_of!(Knobs, rescueflush.at_door_m), false),
    ("posemath.singles", offset_of!(Knobs, posemath.singles), true),
    ("sense.glow.sweep", offset_of!(Knobs, sense.glow.sweep), false),
    ("sense.smoke.sigma", offset_of!(Knobs, sense.smoke.sigma), false),
    ("sense.smoke.decay", offset_of!(Knobs, sense.smoke.decay), false),
    ("sense.smoke.spread", offset_of!(Knobs, sense.smoke.spread), false),
    ("sense.smoke.widest", offset_of!(Knobs, sense.smoke.widest), false),
    ("sense.smoke.rise", offset_of!(Knobs, sense.smoke.rise), false),
    ("sense.smoke.rise_rate", offset_of!(Knobs, sense.smoke.rise_rate), false),
    ("sense.smoke.in_wind", offset_of!(Knobs, sense.smoke.in_wind), false),
    ("sense.smoke.least", offset_of!(Knobs, sense.smoke.least), false),
    ("sense.smoke.gone", offset_of!(Knobs, sense.smoke.gone), false),
    ("sense.smoke.cap", offset_of!(Knobs, sense.smoke.cap), false),
    ("sense.smoke.emitters", offset_of!(Knobs, sense.smoke.emitters), false),
    ("sense.smoke.reserve", offset_of!(Knobs, sense.smoke.reserve), false),
    ("cmdcheck.keys", offset_of!(Knobs, cmdcheck.keys), false),
    ("cmdcheck.key_len", offset_of!(Knobs, cmdcheck.key_len), false),
    ("cmdcheck.span", offset_of!(Knobs, cmdcheck.span), false),
    ("cmdcheck.wheel", offset_of!(Knobs, cmdcheck.wheel), false),
    ("cmdcheck.far", offset_of!(Knobs, cmdcheck.far), false),
    ("cmdcheck.dist_min", offset_of!(Knobs, cmdcheck.dist_min), false),
    ("cmdcheck.dist_max", offset_of!(Knobs, cmdcheck.dist_max), false),
    ("cmdcheck.fov_min", offset_of!(Knobs, cmdcheck.fov_min), false),
    ("cmdcheck.fov_max", offset_of!(Knobs, cmdcheck.fov_max), false),
    ("cmdcheck.aspect_min", offset_of!(Knobs, cmdcheck.aspect_min), false),
    ("cmdcheck.aspect_max", offset_of!(Knobs, cmdcheck.aspect_max), false),
    ("cmdcheck.reach", offset_of!(Knobs, cmdcheck.reach), false),
    ("cmdcheck.steep", offset_of!(Knobs, cmdcheck.steep), false),
    ("seats.max", offset_of!(Knobs, seats.max), false),
    ("seats.apart", offset_of!(Knobs, seats.apart), false),
    ("seats.keep", offset_of!(Knobs, seats.keep), false),
    ("doze.min_wait", offset_of!(Knobs, doze.min_wait), false),
    ("doze.max_wait", offset_of!(Knobs, doze.max_wait), false),
    ("doze.chair_m", offset_of!(Knobs, doze.chair_m), false),
    ("doze.nod_secs", offset_of!(Knobs, doze.nod_secs), false),
    ("doze.rise_secs", offset_of!(Knobs, doze.rise_secs), false),
    ("doze.hear_k", offset_of!(Knobs, doze.hear_k), false),
    ("doze.light_at", offset_of!(Knobs, doze.light_at), false),
    ("doze.light_secs", offset_of!(Knobs, doze.light_secs), false),
    ("doze.ask_secs", offset_of!(Knobs, doze.ask_secs), false),
    ("doze.wake_aw", offset_of!(Knobs, doze.wake_aw), false),
    ("doze.bump_aw", offset_of!(Knobs, doze.bump_aw), false),
    ("doze.stir_secs", offset_of!(Knobs, doze.stir_secs), false),
    ("doze.breath_secs", offset_of!(Knobs, doze.breath_secs), false),
    ("doze.screen_m", offset_of!(Knobs, doze.screen_m), false),
    ("doze.pull_m", offset_of!(Knobs, doze.pull_m), false),
    ("doze.nap_wait", offset_of!(Knobs, doze.nap_wait), false),
    ("doze.settle_secs", offset_of!(Knobs, doze.settle_secs), false),
    ("doze.ears", offset_of!(Knobs, doze.ears), false),
    ("doze.noises", offset_of!(Knobs, doze.noises), false),
    ("doze.idle_secs", offset_of!(Knobs, doze.idle_secs), false),
    ("netquiet.unanswered", offset_of!(Knobs, netquiet.unanswered), false),
    ("netquiet.heard_it", offset_of!(Knobs, netquiet.heard_it), false),
    ("netquiet.body", offset_of!(Knobs, netquiet.body), false),
    ("netquiet.body_told", offset_of!(Knobs, netquiet.body_told), false),
    ("netquiet.fall_seen", offset_of!(Knobs, netquiet.fall_seen), false),
    ("netquiet.fall_unseen", offset_of!(Knobs, netquiet.fall_unseen), false),
    ("netquiet.steady_k", offset_of!(Knobs, netquiet.steady_k), false),
    ("netquiet.ask_beat", offset_of!(Knobs, netquiet.ask_beat), false),
    ("netquiet.body_beat", offset_of!(Knobs, netquiet.body_beat), false),
    ("netquiet.bark_secs", offset_of!(Knobs, netquiet.bark_secs), false),
    ("netquiet.say_hold", offset_of!(Knobs, netquiet.say_hold), false),
    ("netquiet.shooter_secs", offset_of!(Knobs, netquiet.shooter_secs), false),
    ("netquiet.body_m", offset_of!(Knobs, netquiet.body_m), false),
    ("netquiet.whip_shift", offset_of!(Knobs, netquiet.whip_shift), false),
    ("netquiet.whip_secs", offset_of!(Knobs, netquiet.whip_secs), false),
    ("netquiet.whip_hold", offset_of!(Knobs, netquiet.whip_hold), false),
    ("netquiet.whip_turn", offset_of!(Knobs, netquiet.whip_turn), false),
    ("netquiet.whip_turn_broken", offset_of!(Knobs, netquiet.whip_turn_broken), false),
    ("netquiet.whip_m", offset_of!(Knobs, netquiet.whip_m), false),
    ("netquiet.whip_near_m", offset_of!(Knobs, netquiet.whip_near_m), false),
    ("netquiet.last_beat", offset_of!(Knobs, netquiet.last_beat), false),
    ("netquiet.calm_secs", offset_of!(Knobs, netquiet.calm_secs), false),
    ("netquiet.alert_secs", offset_of!(Knobs, netquiet.alert_secs), false),
    ("netquiet.give_up", offset_of!(Knobs, netquiet.give_up), false),
    ("netquiet.mate_m", offset_of!(Knobs, netquiet.mate_m), false),
    ("netquiet.arc_deg", offset_of!(Knobs, netquiet.arc_deg), false),
    ("netquiet.eye_deg", offset_of!(Knobs, netquiet.eye_deg), false),
    ("netquiet.eye_slack", offset_of!(Knobs, netquiet.eye_slack), false),
    ("netquiet.crouch", offset_of!(Knobs, netquiet.crouch), false),
    ("netquiet.wary", offset_of!(Knobs, netquiet.wary), false),
    ("netquiet.back_speed", offset_of!(Knobs, netquiet.back_speed), false),
    ("netquiet.max_m", offset_of!(Knobs, netquiet.max_m), false),
    ("netquiet.door_m", offset_of!(Knobs, netquiet.door_m), false),
    ("netquiet.eye_m", offset_of!(Knobs, netquiet.eye_m), false),
    ("netquiet.stay_m", offset_of!(Knobs, netquiet.stay_m), false),
    ("attention.focus_deg", offset_of!(Knobs, attention.focus_deg), false),
    ("attention.side_k", offset_of!(Knobs, attention.side_k), false),
    ("attention.ear_k", offset_of!(Knobs, attention.ear_k), false),
    ("attention.break_gain", offset_of!(Knobs, attention.break_gain), false),
    ("attention.calm_aw", offset_of!(Knobs, attention.calm_aw), false),
    ("attention.draw_aw", offset_of!(Knobs, attention.draw_aw), false),
    ("attention.floor", offset_of!(Knobs, attention.floor), false),
    ("attention.whistle_secs", offset_of!(Knobs, attention.whistle_secs), false),
    ("attention.noise_secs", offset_of!(Knobs, attention.noise_secs), false),
    ("attention.door_secs", offset_of!(Knobs, attention.door_secs), false),
    ("attention.light_secs", offset_of!(Knobs, attention.light_secs), false),
    ("attention.tell_secs", offset_of!(Knobs, attention.tell_secs), false),
    ("attention.dwell_sweep", offset_of!(Knobs, attention.dwell_sweep), false),
    ("attention.walk_cap", offset_of!(Knobs, attention.walk_cap), false),
    ("attention.wear_k", offset_of!(Knobs, attention.wear_k), false),
    ("attention.memory_secs", offset_of!(Knobs, attention.memory_secs), false),
    ("attention.spent", offset_of!(Knobs, attention.spent), false),
    ("attention.same_secs", offset_of!(Knobs, attention.same_secs), false),
    ("attention.lent_back_secs", offset_of!(Knobs, attention.lent_back_secs), false),
    ("attention.ring_m", offset_of!(Knobs, attention.ring_m), false),
    ("attention.way_reask", offset_of!(Knobs, attention.way_reask), false),
    ("picture.look_secs", offset_of!(Knobs, picture.look_secs), false),
    ("picture.glimpse", offset_of!(Knobs, picture.glimpse), false),
    ("picture.tell_secs", offset_of!(Knobs, picture.tell_secs), false),
    ("picture.shout_m", offset_of!(Knobs, picture.shout_m), false),
    ("picture.pop_m", offset_of!(Knobs, picture.pop_m), false),
    ("picture.made_out", offset_of!(Knobs, picture.made_out), false),
    ("picture.made_out_m", offset_of!(Knobs, picture.made_out_m), false),
    ("picture.fresh_secs", offset_of!(Knobs, picture.fresh_secs), false),
    ("picture.stale_secs", offset_of!(Knobs, picture.stale_secs), false),
    ("picture.stale_k", offset_of!(Knobs, picture.stale_k), false),
    ("picture.forget_secs", offset_of!(Knobs, picture.forget_secs), false),
    ("picture.lost_k", offset_of!(Knobs, picture.lost_k), false),
    ("picture.walk_mps", offset_of!(Knobs, picture.walk_mps), false),
    ("picture.walk_max", offset_of!(Knobs, picture.walk_max), false),
    ("picture.heard_m", offset_of!(Knobs, picture.heard_m), false),
    ("picture.maybe_k", offset_of!(Knobs, picture.maybe_k), false),
    ("picture.reach_m", offset_of!(Knobs, picture.reach_m), false),
    ("picture.eye_h", offset_of!(Knobs, picture.eye_h), false),
    ("picture.chest_h", offset_of!(Knobs, picture.chest_h), false),
    ("picture.low_h", offset_of!(Knobs, picture.low_h), false),
    ("picture.head_up", offset_of!(Knobs, picture.head_up), false),
    ("picture.flee_cost", offset_of!(Knobs, picture.flee_cost), false),
    ("picture.stack_k", offset_of!(Knobs, picture.stack_k), false),
    ("picture.step_at", offset_of!(Knobs, picture.step_at), false),
    ("picture.step_free", offset_of!(Knobs, picture.step_free), false),
    ("picture.step_m", offset_of!(Knobs, picture.step_m), false),
    ("picture.step_after", offset_of!(Knobs, picture.step_after), false),
    ("picture.step_rest", offset_of!(Knobs, picture.step_rest), false),
    ("picture.step_secs", offset_of!(Knobs, picture.step_secs), false),
    ("picture.show_m", offset_of!(Knobs, picture.show_m), false),
    ("response.clear_m", offset_of!(Knobs, response.clear_m), false),
    ("response.eta_step", offset_of!(Knobs, response.eta_step), false),
    ("response.word_secs", offset_of!(Knobs, response.word_secs), false),
    ("response.call_secs", offset_of!(Knobs, response.call_secs), false),
    ("handwork.stand_m", offset_of!(Knobs, handwork.stand_m), false),
    ("handwork.step_rate", offset_of!(Knobs, handwork.step_rate), false),
    ("handwork.step_pace", offset_of!(Knobs, handwork.step_pace), false),
    ("handwork.stand_side", offset_of!(Knobs, handwork.stand_side), false),
    ("handwork.wait_secs", offset_of!(Knobs, handwork.wait_secs), false),
    ("handwork.lever_secs", offset_of!(Knobs, handwork.lever_secs), false),
    ("handwork.lever_at", offset_of!(Knobs, handwork.lever_at), false),
    ("handwork.stow_secs", offset_of!(Knobs, handwork.stow_secs), false),
    ("handwork.radio_k", offset_of!(Knobs, handwork.radio_k), false),
    ("handwork.radio_min", offset_of!(Knobs, handwork.radio_min), false),
    ("handwork.radio_max", offset_of!(Knobs, handwork.radio_max), false),
    ("workstow.mate_down", offset_of!(Knobs, workstow.mate_down), false),
    ("workstow.mate_up", offset_of!(Knobs, workstow.mate_up), false),
    ("workstow.show", offset_of!(Knobs, workstow.show), false),
    ("workstow.down_max", offset_of!(Knobs, workstow.down_max), false),
    ("workstow.sam_up", offset_of!(Knobs, workstow.sam_up), false),
    ("workstow.fight_cap", offset_of!(Knobs, workstow.fight_cap), false),
    ("squadnet.near", offset_of!(Knobs, squadnet.near), false),
    ("squadnet.edge_in", offset_of!(Knobs, squadnet.edge_in), false),
    ("squadnet.click", offset_of!(Knobs, squadnet.click), false),
    ("squadnet.click_rate", offset_of!(Knobs, squadnet.click_rate), false),
    ("squadnet.down_gain", offset_of!(Knobs, squadnet.down_gain), false),
    ("squadnet.down_rate", offset_of!(Knobs, squadnet.down_rate), false),
    ("squadnet.down_secs", offset_of!(Knobs, squadnet.down_secs), false),
    ("takedown.partner_hit", offset_of!(Knobs, takedown.partner_hit), false),
    ("takedown.partner_end", offset_of!(Knobs, takedown.partner_end), false),
    ("takedown.breath_hz", offset_of!(Knobs, takedown.breath_hz), false),
    ("takedown.breath_rise", offset_of!(Knobs, takedown.breath_rise), false),
    ("takedown.breath_in", offset_of!(Knobs, takedown.breath_in), false),
    ("stealthpose.sway_secs", offset_of!(Knobs, stealthpose.sway_secs), false),
    ("stealthpose.sway_m", offset_of!(Knobs, stealthpose.sway_m), false),
    ("stealthpose.drift_m", offset_of!(Knobs, stealthpose.drift_m), false),
    ("stealthpose.rise_secs", offset_of!(Knobs, stealthpose.rise_secs), false),
    ("stealthpose.cable_in", offset_of!(Knobs, stealthpose.cable_in), false),
    ("stealthpose.cable_bow", offset_of!(Knobs, stealthpose.cable_bow), false),
    ("stealthpose.cable_step", offset_of!(Knobs, stealthpose.cable_step), false),
    ("look.on", offset_of!(Knobs, look.on), true),
    ("lived.on", offset_of!(Knobs, lived.on), true),
    ("env.exposure", offset_of!(Knobs, env.exposure), false),
    ("env.tube_gain", offset_of!(Knobs, env.tube_gain), false),
    ("shapes.on", offset_of!(Knobs, shapes.on), true),
    ("flash.secs.0", offset_of!(Knobs, flash.secs), false),
    ("flash.secs.1", offset_of!(Knobs, flash.secs) + 8, false),
    ("flash.stagger.0", offset_of!(Knobs, flash.stagger), false),
    ("flash.stagger.1", offset_of!(Knobs, flash.stagger) + 8, false),
    ("flash.lift", offset_of!(Knobs, flash.lift), false),
    ("flash.concuss_m", offset_of!(Knobs, flash.concuss_m), false),
    ("flash.concuss_k", offset_of!(Knobs, flash.concuss_k), false),
    ("flash.own_m", offset_of!(Knobs, flash.own_m), false),
    ("flash.own_secs", offset_of!(Knobs, flash.own_secs), false),
    ("flash.fuse", offset_of!(Knobs, flash.fuse), false),
    ("pace.max", offset_of!(Knobs, pace.max), false),
    ("pace.accel", offset_of!(Knobs, pace.accel), false),
    ("pace.turn", offset_of!(Knobs, pace.turn), false),
    ("pace.swing", offset_of!(Knobs, pace.swing), false),
    ("lights.after_secs", offset_of!(Knobs, lights.after_secs), false),
    ("lights.hot_secs", offset_of!(Knobs, lights.hot_secs), false),
    ("lights.step_secs", offset_of!(Knobs, lights.step_secs), false),
    ("lights.tail", offset_of!(Knobs, lights.tail), false),
    ("lights.on_m", offset_of!(Knobs, lights.on_m), false),
    ("lights.alone_m", offset_of!(Knobs, lights.alone_m), false),
    ("lights.look_lit", offset_of!(Knobs, lights.look_lit), false),
    ("lights.look_below", offset_of!(Knobs, lights.look_below), false),
    ("lights.look_m", offset_of!(Knobs, lights.look_m), false),
    ("lights.lit_at", offset_of!(Knobs, lights.lit_at), false),
    ("lights.lit_m", offset_of!(Knobs, lights.lit_m), false),
    ("lights.lit_mark_m", offset_of!(Knobs, lights.lit_mark_m), false),
    ("lights.wall_m", offset_of!(Knobs, lights.wall_m), false),
    ("lights.wall_back_m", offset_of!(Knobs, lights.wall_back_m), false),
    ("lights.wall_secs", offset_of!(Knobs, lights.wall_secs), false),
    ("nvg.quiet_secs", offset_of!(Knobs, nvg.quiet_secs), false),
    ("nvg.lead_tail", offset_of!(Knobs, nvg.lead_tail), false),
    ("nvg.dark_at", offset_of!(Knobs, nvg.dark_at), false),
    ("nvg.black_at", offset_of!(Knobs, nvg.black_at), false),
    ("nvg.black_reads", offset_of!(Knobs, nvg.black_reads), false),
    ("nvg.clear_reads", offset_of!(Knobs, nvg.clear_reads), false),
    ("nvg.albedo", offset_of!(Knobs, nvg.albedo), false),
    ("nvg.noise", offset_of!(Knobs, nvg.noise), false),
    ("nvg.down_secs", offset_of!(Knobs, nvg.down_secs), false),
    ("nvg.up_secs", offset_of!(Knobs, nvg.up_secs), false),
    ("nvg.call_gap", offset_of!(Knobs, nvg.call_gap), false),
    ("irlight.show", offset_of!(Knobs, irlight.show), false),
    ("irlight.hold", offset_of!(Knobs, irlight.hold), false),
    ("irlight.power", offset_of!(Knobs, irlight.power), false),
    ("irlight.reach", offset_of!(Knobs, irlight.reach), false),
    ("irlight.cone", offset_of!(Knobs, irlight.cone), false),
    ("irlight.core", offset_of!(Knobs, irlight.core), false),
    ("irlight.dip", offset_of!(Knobs, irlight.dip), false),
    ("irlight.near_m", offset_of!(Knobs, irlight.near_m), false),
    ("irlight.near_min", offset_of!(Knobs, irlight.near_min), false),
    ("irlight.near_up", offset_of!(Knobs, irlight.near_up), false),
    ("irlight.near_down", offset_of!(Knobs, irlight.near_down), false),
    ("wallpen.min_energy", offset_of!(Knobs, wallpen.min_energy), false),
    ("wallpen.max_things", offset_of!(Knobs, wallpen.max_things), false),
    ("wallpen.deflect_deg", offset_of!(Knobs, wallpen.deflect_deg), false),
    ("wallpen.behind_m", offset_of!(Knobs, wallpen.behind_m), false),
    ("wallpen.thru_rounds", offset_of!(Knobs, wallpen.thru_rounds), false),
    ("wallpen.thru_rest_secs", offset_of!(Knobs, wallpen.thru_rest_secs), false),
    ("doortools.charges", offset_of!(Knobs, doortools.charges), false),
    ("doortools.bangs", offset_of!(Knobs, doortools.bangs), false),
    ("doortools.word_tap", offset_of!(Knobs, doortools.word_tap), false),
    ("doortools.stack_cap", offset_of!(Knobs, doortools.stack_cap), false),
    ("doortools.still_secs", offset_of!(Knobs, doortools.still_secs), false),
    ("doortools.kick_cap", offset_of!(Knobs, doortools.kick_cap), false),
    ("doortools.in_cap", offset_of!(Knobs, doortools.in_cap), false),
    ("doortools.shove", offset_of!(Knobs, doortools.shove), false),
    ("doortools.hold_open", offset_of!(Knobs, doortools.hold_open), false),
    ("doortools.toss_after_open", offset_of!(Knobs, doortools.toss_after_open), false),
    ("doortools.bang_in", offset_of!(Knobs, doortools.bang_in), false),
    ("doortools.fuse_after_land", offset_of!(Knobs, doortools.fuse_after_land), false),
    ("doortools.air_in", offset_of!(Knobs, doortools.air_in), false),
    ("doortools.air_h", offset_of!(Knobs, doortools.air_h), false),
    ("doortools.toss_cap", offset_of!(Knobs, doortools.toss_cap), false),
    ("doortools.toss_after_kick", offset_of!(Knobs, doortools.toss_after_kick), false),
    ("doortools.throw_reach", offset_of!(Knobs, doortools.throw_reach), false),
    ("doortools.toss_turn", offset_of!(Knobs, doortools.toss_turn), false),
    ("doortools.frag_way_wait", offset_of!(Knobs, doortools.frag_way_wait), false),
    ("doortools.frag_way_open", offset_of!(Knobs, doortools.frag_way_open), false),
    ("doortools.frag_way_ajar", offset_of!(Knobs, doortools.frag_way_ajar), false),
    ("doortools.last_out_m", offset_of!(Knobs, doortools.last_out_m), false),
    ("doortools.after_bang", offset_of!(Knobs, doortools.after_bang), false),
    ("doortools.quiet", offset_of!(Knobs, doortools.quiet), false),
    ("doortools.entry_cap", offset_of!(Knobs, doortools.entry_cap), false),
    ("doortools.second", offset_of!(Knobs, doortools.second), false),
    ("doortools.sweep_rad", offset_of!(Knobs, doortools.sweep_rad), false),
    ("doortools.sweep_secs", offset_of!(Knobs, doortools.sweep_secs), false),
    ("doortools.corner_secs", offset_of!(Knobs, doortools.corner_secs), false),
    ("doortools.corner_rad", offset_of!(Knobs, doortools.corner_rad), false),
    ("doortools.corner_cap", offset_of!(Knobs, doortools.corner_cap), false),
    ("doortools.lead_seen_min", offset_of!(Knobs, doortools.lead_seen_min), false),
    ("doortools.lead_seen_secs", offset_of!(Knobs, doortools.lead_seen_secs), false),
    ("doortools.lead_seen_m", offset_of!(Knobs, doortools.lead_seen_m), false),
    ("doortools.sector_secs", offset_of!(Knobs, doortools.sector_secs), false),
    ("doortools.scan_rate", offset_of!(Knobs, doortools.scan_rate), false),
    ("doortools.look_rad", offset_of!(Knobs, doortools.look_rad), false),
    ("doortools.look_m", offset_of!(Knobs, doortools.look_m), false),
    ("doortools.dead_cells", offset_of!(Knobs, doortools.dead_cells), false),
    ("doortools.search_moves", offset_of!(Knobs, doortools.search_moves), false),
    ("doortools.search_cap", offset_of!(Knobs, doortools.search_cap), false),
    ("doortools.search_cells", offset_of!(Knobs, doortools.search_cells), false),
    ("doortools.tuck", offset_of!(Knobs, doortools.tuck), false),
    ("doortools.edge_out", offset_of!(Knobs, doortools.edge_out), false),
    ("doortools.bang_file", offset_of!(Knobs, doortools.bang_file), false),
    ("doortools.set_secs", offset_of!(Knobs, doortools.set_secs), false),
    ("doortools.knock_m", offset_of!(Knobs, doortools.knock_m), false),
    ("doortools.stun_m", offset_of!(Knobs, doortools.stun_m), false),
    ("doortools.stagger_secs", offset_of!(Knobs, doortools.stagger_secs), false),
    ("doortools.backblast_m", offset_of!(Knobs, doortools.backblast_m), false),
    ("doortools.backblast_secs", offset_of!(Knobs, doortools.backblast_secs), false),
    ("doortools.blow_vel", offset_of!(Knobs, doortools.blow_vel), false),
    ("doortools.slice_back", offset_of!(Knobs, doortools.slice_back), false),
    ("doortools.slice_look", offset_of!(Knobs, doortools.slice_look), false),
    ("doortools.slice_edge", offset_of!(Knobs, doortools.slice_edge), false),
    ("doortools.slice_deep", offset_of!(Knobs, doortools.slice_deep), false),
    ("doortools.slice_pace", offset_of!(Knobs, doortools.slice_pace), false),
    ("doortools.word_stack", offset_of!(Knobs, doortools.word_stack), false),
    ("doortools.word_in", offset_of!(Knobs, doortools.word_in), false),
    ("doortools.pod_apart", offset_of!(Knobs, doortools.pod_apart), false),
    ("doortools.thr_min", offset_of!(Knobs, doortools.thr_min), false),
    ("doortools.thr_quiet", offset_of!(Knobs, doortools.thr_quiet), false),
    ("doortools.thr_cap", offset_of!(Knobs, doortools.thr_cap), false),
    ("doortools.thr_bangs", offset_of!(Knobs, doortools.thr_bangs), false),
    ("doortools.thr_pocket", offset_of!(Knobs, doortools.thr_pocket), false),
    ("doortools.thr_shot_m", offset_of!(Knobs, doortools.thr_shot_m), false),
    ("doortools.thr_throw_m", offset_of!(Knobs, doortools.thr_throw_m), false),
    ("doortools.thr_flash_m", offset_of!(Knobs, doortools.thr_flash_m), false),
    ("doortools.thr_turn", offset_of!(Knobs, doortools.thr_turn), false),
    ("doortools.thr_next", offset_of!(Knobs, doortools.thr_next), false),
    ("doortools.thr_light", offset_of!(Knobs, doortools.thr_light), false),
    ("doortools.thr_in", offset_of!(Knobs, doortools.thr_in), false),
    ("doortools.thr_pie_apart", offset_of!(Knobs, doortools.thr_pie_apart), false),
    ("doortools.thr_lean_m", offset_of!(Knobs, doortools.thr_lean_m), false),
    ("doortools.thr_knee_in", offset_of!(Knobs, doortools.thr_knee_in), false),
    ("doortools.thr_lean_look", offset_of!(Knobs, doortools.thr_lean_look), false),
    ("doortools.thr_doors_m", offset_of!(Knobs, doortools.thr_doors_m), false),
    ("doortools.thr_doors_walk", offset_of!(Knobs, doortools.thr_doors_walk), false),
    ("doortools.thr_doors_stay", offset_of!(Knobs, doortools.thr_doors_stay), false),
    ("doortools.other_door_m", offset_of!(Knobs, doortools.other_door_m), false),
    ("doortools.buddy_m", offset_of!(Knobs, doortools.buddy_m), false),
    ("doortools.buddy_pair_m", offset_of!(Knobs, doortools.buddy_pair_m), false),
    ("doortools.buddy_left_m", offset_of!(Knobs, doortools.buddy_left_m), false),
    ("doortools.flow_cells", offset_of!(Knobs, doortools.flow_cells), false),
    ("doortools.flow_moves", offset_of!(Knobs, doortools.flow_moves), false),
    ("doortools.flow_reach", offset_of!(Knobs, doortools.flow_reach), false),
    ("doortools.flow_deep", offset_of!(Knobs, doortools.flow_deep), false),
    ("doortools.flow_bound", offset_of!(Knobs, doortools.flow_bound), false),
    ("doortools.flow_secs", offset_of!(Knobs, doortools.flow_secs), false),
    ("doortools.flow_fight", offset_of!(Knobs, doortools.flow_fight), false),
    ("doortools.flow_max", offset_of!(Knobs, doortools.flow_max), false),
    ("doortools.word_cuts_cap", offset_of!(Knobs, doortools.word_cuts_cap), false),
    ("doortools.bang_into_min", offset_of!(Knobs, doortools.bang_into_min), false),
    ("doortools.bang_into_lit", offset_of!(Knobs, doortools.bang_into_lit), false),
    ("doortools.bang_into_deep", offset_of!(Knobs, doortools.bang_into_deep), false),
    ("doortools.bang_into_wide", offset_of!(Knobs, doortools.bang_into_wide), false),
    ("doortools.bang_hand_m", offset_of!(Knobs, doortools.bang_hand_m), false),
    ("doortools.bang_pass_v", offset_of!(Knobs, doortools.bang_pass_v), false),
    ("doortools.pod_frame_m", offset_of!(Knobs, doortools.pod_frame_m), false),
    ("doortools.thr_hand_wait", offset_of!(Knobs, doortools.thr_hand_wait), false),
    ("doortools.thr_hand_every", offset_of!(Knobs, doortools.thr_hand_every), false),
    ("doortools.frag_hold_cap", offset_of!(Knobs, doortools.frag_hold_cap), false),
    ("doortools.threat_secs", offset_of!(Knobs, doortools.threat_secs), false),
    ("doortools.threat_cap", offset_of!(Knobs, doortools.threat_cap), false),
    ("doortools.threat_near_m", offset_of!(Knobs, doortools.threat_near_m), false),
    ("doortools.threat_stuck", offset_of!(Knobs, doortools.threat_stuck), false),
    ("doortools.thr_set_secs", offset_of!(Knobs, doortools.thr_set_secs), false),
    ("doortools.thr_first", offset_of!(Knobs, doortools.thr_first), false),
    ("doortools.thr_first_min", offset_of!(Knobs, doortools.thr_first_min), false),
    ("doortools.thr_first_stale", offset_of!(Knobs, doortools.thr_first_stale), false),
    ("doortools.thr_first_in", offset_of!(Knobs, doortools.thr_first_in), false),
    ("doortools.thr_last_m", offset_of!(Knobs, doortools.thr_last_m), false),
    ("doortools.thr_last_fresh", offset_of!(Knobs, doortools.thr_last_fresh), false),
    ("roomfight.evidence_secs", offset_of!(Knobs, roomfight.evidence_secs), false),
    ("roomfight.sticky", offset_of!(Knobs, roomfight.sticky), false),
    ("roomfight.crowd", offset_of!(Knobs, roomfight.crowd), false),
    ("roomfight.fear_min", offset_of!(Knobs, roomfight.fear_min), false),
    ("roomfight.close_m", offset_of!(Knobs, roomfight.close_m), false),
    ("roomfight.no_shot", offset_of!(Knobs, roomfight.no_shot), false),
    ("roomfight.point_blank", offset_of!(Knobs, roomfight.point_blank), false),
    ("roomfight.no_shot_go", offset_of!(Knobs, roomfight.no_shot_go), false),
    ("roomfight.no_shot_pass", offset_of!(Knobs, roomfight.no_shot_pass), false),
    ("roomfight.rally_k", offset_of!(Knobs, roomfight.rally_k), false),
    ("roomfight.arc_half_deg", offset_of!(Knobs, roomfight.arc_half_deg), false),
    ("roomfight.cover_half_deg", offset_of!(Knobs, roomfight.cover_half_deg), false),
    ("roomfight.cover_near_m", offset_of!(Knobs, roomfight.cover_near_m), false),
    ("roomfight.team_hot", offset_of!(Knobs, roomfight.team_hot), false),
    ("roomfight.let_go", offset_of!(Knobs, roomfight.let_go), false),
    ("roomfight.step_look_secs", offset_of!(Knobs, roomfight.step_look_secs), false),
    ("roomfight.door_look_secs", offset_of!(Knobs, roomfight.door_look_secs), false),
    ("roomfight.door_shot_m", offset_of!(Knobs, roomfight.door_shot_m), false),
    ("roomfight.hold_short_secs", offset_of!(Knobs, roomfight.hold_short_secs), false),
    ("roomfight.move_engage_m", offset_of!(Knobs, roomfight.move_engage_m), false),
    ("roomfight.lamp", offset_of!(Knobs, roomfight.lamp), false),
    ("roomfight.preaim_deg", offset_of!(Knobs, roomfight.preaim_deg), false),
    ("orders.refresh_secs", offset_of!(Knobs, orders.refresh_secs), false),
    ("orders.door_pick_m", offset_of!(Knobs, orders.door_pick_m), false),
    ("orders.self_m", offset_of!(Knobs, orders.self_m), false),
    ("orders.man_m", offset_of!(Knobs, orders.man_m), false),
    ("orders.door_ray_pad", offset_of!(Knobs, orders.door_ray_pad), false),
    ("orders.door_ray_reach_m", offset_of!(Knobs, orders.door_ray_reach_m), false),
    ("orders.floor_snap_m", offset_of!(Knobs, orders.floor_snap_m), false),
    ("orders.jamb_out", offset_of!(Knobs, orders.jamb_out), false),
    ("orders.jamb_past", offset_of!(Knobs, orders.jamb_past), false),
    ("orders.file_gap", offset_of!(Knobs, orders.file_gap), false),
    ("orders.seam_m", offset_of!(Knobs, orders.seam_m), false),
    ("orders.work_seen_m", offset_of!(Knobs, orders.work_seen_m), false),
    ("orders.crumb", offset_of!(Knobs, orders.crumb), false),
    ("orders.trail_m", offset_of!(Knobs, orders.trail_m), false),
    ("orders.run", offset_of!(Knobs, orders.run), false),
    ("orders.walk", offset_of!(Knobs, orders.walk), false),
    ("orders.gain", offset_of!(Knobs, orders.gain), false),
    ("orders.near_m", offset_of!(Knobs, orders.near_m), false),
    ("orders.band_m", offset_of!(Knobs, orders.band_m), false),
    ("orders.word_far_m", offset_of!(Knobs, orders.word_far_m), false),
    ("orders.word_hold_s", offset_of!(Knobs, orders.word_hold_s), false),
    ("orders.file_apart", offset_of!(Knobs, orders.file_apart), false),
    ("orders.file_tight", offset_of!(Knobs, orders.file_tight), false),
    ("orders.off_door", offset_of!(Knobs, orders.off_door), false),
    ("orders.aside_m", offset_of!(Knobs, orders.aside_m), false),
    ("orders.aside_lane", offset_of!(Knobs, orders.aside_lane), false),
    ("orders.aside_off", offset_of!(Knobs, orders.aside_off), false),
    ("orders.file_queue_gap", offset_of!(Knobs, orders.file_queue_gap), false),
    ("orders.file_queue_cost", offset_of!(Knobs, orders.file_queue_cost), false),
    ("orders.prop_round_pad", offset_of!(Knobs, orders.prop_round_pad), false),
    ("orders.prop_round_cost", offset_of!(Knobs, orders.prop_round_cost), false),
    ("orders.prop_round_more", offset_of!(Knobs, orders.prop_round_more), false),
    ("orders.prop_round_door", offset_of!(Knobs, orders.prop_round_door), false),
    ("orders.aside_hold", offset_of!(Knobs, orders.aside_hold), false),
    ("orders.brisk", offset_of!(Knobs, orders.brisk), false),
    ("orders.brisk_in", offset_of!(Knobs, orders.brisk_in), false),
    ("orders.onto", offset_of!(Knobs, orders.onto), false),
    ("orders.entry", offset_of!(Knobs, orders.entry), false),
    ("orders.move_spread", offset_of!(Knobs, orders.move_spread), false),
    ("orders.cold_secs", offset_of!(Knobs, orders.cold_secs), false),
    ("orders.lane_in", offset_of!(Knobs, orders.lane_in), false),
    ("orders.lane_out", offset_of!(Knobs, orders.lane_out), false),
    ("orders.hold_stride", offset_of!(Knobs, orders.hold_stride), false),
    ("orders.stuck_secs", offset_of!(Knobs, orders.stuck_secs), false),
    ("orders.hold_group_m", offset_of!(Knobs, orders.hold_group_m), false),
    ("orders.hold_door_m", offset_of!(Knobs, orders.hold_door_m), false),
    ("orders.cleared_dear", offset_of!(Knobs, orders.cleared_dear), false),
    ("orders.same_near_m", offset_of!(Knobs, orders.same_near_m), false),
    ("orders.same_apart_m", offset_of!(Knobs, orders.same_apart_m), false),
    ("orders.same_long", offset_of!(Knobs, orders.same_long), false),
    ("orders.trail_reach_m", offset_of!(Knobs, orders.trail_reach_m), false),
    ("orders.trail_long", offset_of!(Knobs, orders.trail_long), false),
    ("cover.rear_read_in", offset_of!(Knobs, cover.rear_read_in), false),
    ("cover.rear_read_m", offset_of!(Knobs, cover.rear_read_m), false),
    ("cover.rear_read_deg", offset_of!(Knobs, cover.rear_read_deg), false),
    ("cover.rear_read_secs", offset_of!(Knobs, cover.rear_read_secs), false),
    ("cover.rear_read_sticky", offset_of!(Knobs, cover.rear_read_sticky), false),
    ("cover.rear_look", offset_of!(Knobs, cover.rear_look), false),
    ("cover.rear_walk_m", offset_of!(Knobs, cover.rear_walk_m), false),
    ("cover.reach", offset_of!(Knobs, cover.reach), false),
    ("cover.deal_secs", offset_of!(Knobs, cover.deal_secs), false),
    ("cover.gap_cells", offset_of!(Knobs, cover.gap_cells), false),
    ("cover.through_rows", offset_of!(Knobs, cover.through_rows), false),
    ("cover.alcove_rows", offset_of!(Knobs, cover.alcove_rows), false),
    ("cover.bridge", offset_of!(Knobs, cover.bridge), false),
    ("cover.in_m", offset_of!(Knobs, cover.in_m), false),
    ("cover.lane", offset_of!(Knobs, cover.lane), false),
    ("cover.lane_keep", offset_of!(Knobs, cover.lane_keep), false),
    ("cover.lane_secs", offset_of!(Knobs, cover.lane_secs), false),
    ("cover.sam_deg", offset_of!(Knobs, cover.sam_deg), false),
    ("cover.sam_k", offset_of!(Knobs, cover.sam_k), false),
    ("cover.held_k", offset_of!(Knobs, cover.held_k), false),
    ("cover.shut_k", offset_of!(Knobs, cover.shut_k), false),
    ("cover.rear_k", offset_of!(Knobs, cover.rear_k), false),
    ("cover.near_m", offset_of!(Knobs, cover.near_m), false),
    ("cover.contact_secs", offset_of!(Knobs, cover.contact_secs), false),
    ("cover.rear_m", offset_of!(Knobs, cover.rear_m), false),
    ("cover.rear_from", offset_of!(Knobs, cover.rear_from), false),
    ("cover.rear_deg", offset_of!(Knobs, cover.rear_deg), false),
    ("cover.sticky", offset_of!(Knobs, cover.sticky), false),
    ("cover.keep_secs", offset_of!(Knobs, cover.keep_secs), false),
    ("cover.rear_keep", offset_of!(Knobs, cover.rear_keep), false),
    ("cover.behind_k", offset_of!(Knobs, cover.behind_k), false),
    ("cover.twist", offset_of!(Knobs, cover.twist), false),
    ("cover.back_in", offset_of!(Knobs, cover.back_in), false),
    ("cover.back_out", offset_of!(Knobs, cover.back_out), false),
    ("cover.back_body", offset_of!(Knobs, cover.back_body), false),
    ("cover.back_turn_body", offset_of!(Knobs, cover.back_turn_body), false),
    ("cover.back_turn_secs", offset_of!(Knobs, cover.back_turn_secs), false),
    ("cover.swing", offset_of!(Knobs, cover.swing), false),
    ("cover.step_m", offset_of!(Knobs, cover.step_m), false),
    ("cover.step_after", offset_of!(Knobs, cover.step_after), false),
    ("cover.step_off_m", offset_of!(Knobs, cover.step_off_m), false),
    ("cover.step_reach", offset_of!(Knobs, cover.step_reach), false),
    ("cover.step_secs", offset_of!(Knobs, cover.step_secs), false),
    ("cover.step_place_m", offset_of!(Knobs, cover.step_place_m), false),
    ("roomfit.spread0", offset_of!(Knobs, roomfit.spread0), false),
    ("roomfit.spread_per", offset_of!(Knobs, roomfit.spread_per), false),
    ("roomfit.apart", offset_of!(Knobs, roomfit.apart), false),
    ("roomfit.loose", offset_of!(Knobs, roomfit.loose), false),
    ("roomfit.loose_k", offset_of!(Knobs, roomfit.loose_k), false),
    ("roomfit.near_k", offset_of!(Knobs, roomfit.near_k), false),
    ("roomfit.near_free", offset_of!(Knobs, roomfit.near_free), false),
    ("roomfit.wall_k", offset_of!(Knobs, roomfit.wall_k), false),
    ("roomfit.corner_k", offset_of!(Knobs, roomfit.corner_k), false),
    ("roomfit.funnel_m", offset_of!(Knobs, roomfit.funnel_m), false),
    ("roomfit.funnel_pad", offset_of!(Knobs, roomfit.funnel_pad), false),
    ("roomfit.funnel_flare", offset_of!(Knobs, roomfit.funnel_flare), false),
    ("roomfit.funnel_k", offset_of!(Knobs, roomfit.funnel_k), false),
    ("roomfit.own_k", offset_of!(Knobs, roomfit.own_k), false),
    ("roomfit.known_funnel_k", offset_of!(Knobs, roomfit.known_funnel_k), false),
    ("roomfit.shut_funnel_k", offset_of!(Knobs, roomfit.shut_funnel_k), false),
    ("roomfit.ang_k", offset_of!(Knobs, roomfit.ang_k), false),
    ("roomfit.ang_lo", offset_of!(Knobs, roomfit.ang_lo), false),
    ("roomfit.ang_hi", offset_of!(Knobs, roomfit.ang_hi), false),
    ("roomfit.ang_max", offset_of!(Knobs, roomfit.ang_max), false),
    ("roomfit.close_m", offset_of!(Knobs, roomfit.close_m), false),
    ("roomfit.close_k", offset_of!(Knobs, roomfit.close_k), false),
    ("roomfit.lane_m", offset_of!(Knobs, roomfit.lane_m), false),
    ("roomfit.lane_k", offset_of!(Knobs, roomfit.lane_k), false),
    ("roomfit.reach", offset_of!(Knobs, roomfit.reach), false),
    ("roomfit.near_m", offset_of!(Knobs, roomfit.near_m), false),
    ("roomfit.known_k", offset_of!(Knobs, roomfit.known_k), false),
    ("roomfit.shut_k", offset_of!(Knobs, roomfit.shut_k), false),
    ("roomfit.locked_k", offset_of!(Knobs, roomfit.locked_k), false),
    ("roomfit.front_k", offset_of!(Knobs, roomfit.front_k), false),
    ("roomfit.second_k", offset_of!(Knobs, roomfit.second_k), false),
    ("roomfit.sector_m", offset_of!(Knobs, roomfit.sector_m), false),
    ("roomfit.sector_k", offset_of!(Knobs, roomfit.sector_k), false),
    ("roomfit.sector_deg", offset_of!(Knobs, roomfit.sector_deg), false),
    ("roomfit.sector_across", offset_of!(Knobs, roomfit.sector_across), false),
    ("roomfit.sector_look", offset_of!(Knobs, roomfit.sector_look), false),
    ("roomfit.sector_far_m", offset_of!(Knobs, roomfit.sector_far_m), false),
    ("roomfit.sector_far_k", offset_of!(Knobs, roomfit.sector_far_k), false),
    ("roomfit.watch_in_m", offset_of!(Knobs, roomfit.watch_in_m), false),
    ("roomfit.again_secs", offset_of!(Knobs, roomfit.again_secs), false),
    ("roomfit.sticky", offset_of!(Knobs, roomfit.sticky), false),
    ("entry.near_m", offset_of!(Knobs, entry.near_m), false),
    ("entry.line_deg", offset_of!(Knobs, entry.line_deg), false),
    ("entry.known_m", offset_of!(Knobs, entry.known_m), false),
    ("entry.move_m", offset_of!(Knobs, entry.move_m), false),
    ("entry.apart", offset_of!(Knobs, entry.apart), false),
    ("entry.mouth_m", offset_of!(Knobs, entry.mouth_m), false),
    ("rear.secs", offset_of!(Knobs, rear.secs), false),
    ("silent.on", offset_of!(Knobs, silent.on), true),
    ("silent.start", offset_of!(Knobs, silent.start), true),
    ("silent.pace", offset_of!(Knobs, silent.pace), false),
    ("silent.step", offset_of!(Knobs, silent.step), false),
    ("silent.noticed", offset_of!(Knobs, silent.noticed), false),
    ("pair.on", offset_of!(Knobs, pair.on), true),
    ("pair.max_squad", offset_of!(Knobs, pair.max_squad), false),
    ("pair.pick_m", offset_of!(Knobs, pair.pick_m), false),
    ("pair.stage_m", offset_of!(Knobs, pair.stage_m), false),
    ("pair.reach", offset_of!(Knobs, pair.reach), false),
    ("pair.secs", offset_of!(Knobs, pair.secs), false),
    ("pair.scuffle", offset_of!(Knobs, pair.scuffle), false),
    ("pair.pace", offset_of!(Knobs, pair.pace), false),
    ("pair.go_pace", offset_of!(Knobs, pair.go_pace), false),
    ("pair.stalk_every", offset_of!(Knobs, pair.stalk_every), false),
    ("pair.stalk_wait", offset_of!(Knobs, pair.stalk_wait), false),
    ("pair.ring_m", offset_of!(Knobs, pair.ring_m), false),
    ("pair.steady_face_secs", offset_of!(Knobs, pair.steady_face_secs), false),
    ("pair.front_m", offset_of!(Knobs, pair.front_m), false),
    ("pair.cone_m", offset_of!(Knobs, pair.cone_m), false),
    ("pair.cone_lit", offset_of!(Knobs, pair.cone_lit), false),
    ("pair.watch_secs", offset_of!(Knobs, pair.watch_secs), false),
    ("pair.watch_aw", offset_of!(Knobs, pair.watch_aw), false),
    ("pair.heel_ring", offset_of!(Knobs, pair.heel_ring), false),
    ("pair.heel_fresh", offset_of!(Knobs, pair.heel_fresh), false),
    ("pair.heel_wait", offset_of!(Knobs, pair.heel_wait), false),
    ("pair.take_secs", offset_of!(Knobs, pair.take_secs), false),
    ("pair.dark_vis", offset_of!(Knobs, pair.dark_vis), false),
    ("pair.dark_m", offset_of!(Knobs, pair.dark_m), false),
    ("pair.dark_h", offset_of!(Knobs, pair.dark_h), false),
    ("pair.lit_secs", offset_of!(Knobs, pair.lit_secs), false),
    ("pair.lit_every", offset_of!(Knobs, pair.lit_every), false),
    ("rescue.resolve_secs", offset_of!(Knobs, rescue.resolve_secs), false),
    ("rescue.resolve_spread", offset_of!(Knobs, rescue.resolve_spread), false),
    ("rescue.second_hostage_secs", offset_of!(Knobs, rescue.second_hostage_secs), false),
    ("rescue.aim_secs", offset_of!(Knobs, rescue.aim_secs), false),
    ("rescue.set_secs", offset_of!(Knobs, rescue.set_secs), false),
    ("rescue.unset_react", offset_of!(Knobs, rescue.unset_react), false),
    ("rescue.hold_secs", offset_of!(Knobs, rescue.hold_secs), false),
    ("rescue.roamers", offset_of!(Knobs, rescue.roamers), false),
    ("rescue.crew_call_secs", offset_of!(Knobs, rescue.crew_call_secs), false),
    ("rescue.squad", offset_of!(Knobs, rescue.squad), false),
    ("rescue.sam_at.0", offset_of!(Knobs, rescue.sam_at), false),
    ("rescue.sam_at.1", offset_of!(Knobs, rescue.sam_at) + 8, false),
    ("rescue.sam_at.2", offset_of!(Knobs, rescue.sam_at) + 16, false),
    ("rescue.sam_yaw", offset_of!(Knobs, rescue.sam_yaw), false),
    ("digin.on", offset_of!(Knobs, digin.on), true),
    ("digin.reach_m", offset_of!(Knobs, digin.reach_m), false),
    ("digin.keeper", offset_of!(Knobs, digin.keeper), true),
    ("digin.noticed_secs", offset_of!(Knobs, digin.noticed_secs), false),
    ("digin.shut_secs", offset_of!(Knobs, digin.shut_secs), false),
    ("digin.shut_home_rad", offset_of!(Knobs, digin.shut_home_rad), false),
    ("digin.shut_doors", offset_of!(Knobs, digin.shut_doors), false),
    ("digin.gather_m", offset_of!(Knobs, digin.gather_m), false),
    ("digin.gather_shy_m", offset_of!(Knobs, digin.gather_shy_m), false),
    ("digin.think.0", offset_of!(Knobs, digin.think), false),
    ("digin.think.1", offset_of!(Knobs, digin.think) + 8, false),
    ("digin.wedge_secs.0", offset_of!(Knobs, digin.wedge_secs), false),
    ("digin.wedge_secs.1", offset_of!(Knobs, digin.wedge_secs) + 8, false),
    ("digin.wedges", offset_of!(Knobs, digin.wedges), false),
    ("digin.caught.0", offset_of!(Knobs, digin.caught), false),
    ("digin.caught.1", offset_of!(Knobs, digin.caught) + 8, false),
    ("digin.try_secs", offset_of!(Knobs, digin.try_secs), false),
    ("digin.pace", offset_of!(Knobs, digin.pace), false),
    ("digin.close_m", offset_of!(Knobs, digin.close_m), false),
    ("digin.prop_m", offset_of!(Knobs, digin.prop_m), false),
    ("digin.haul_speed", offset_of!(Knobs, digin.haul_speed), false),
    ("digin.scrape_level", offset_of!(Knobs, digin.scrape_level), false),
    ("digin.scrape_every", offset_of!(Knobs, digin.scrape_every), false),
    ("digin.shout_m", offset_of!(Knobs, digin.shout_m), false),
    ("digin.alt_m", offset_of!(Knobs, digin.alt_m), false),
    ("digin.better", offset_of!(Knobs, digin.better), false),
    ("digin.fear", offset_of!(Knobs, digin.fear), false),
    ("digin.hold_secs", offset_of!(Knobs, digin.hold_secs), false),
    ("digin.redig_secs", offset_of!(Knobs, digin.redig_secs), false),
    ("digin.lane_secs", offset_of!(Knobs, digin.lane_secs), false),
    ("pressure.on", offset_of!(Knobs, pressure.on), true),
    ("pressure.second", offset_of!(Knobs, pressure.second), true),
    ("pressure.max_men", offset_of!(Knobs, pressure.max_men), false),
    ("pressure.memory_secs", offset_of!(Knobs, pressure.memory_secs), false),
    ("pressure.organise_k", offset_of!(Knobs, pressure.organise_k), false),
    ("pressure.warned", offset_of!(Knobs, pressure.warned), true),
    ("pressure.report_secs", offset_of!(Knobs, pressure.report_secs), false),
    ("pressure.stand_secs", offset_of!(Knobs, pressure.stand_secs), false),
    ("pressure.stand_grace", offset_of!(Knobs, pressure.stand_grace), false),
    ("pressure.again_secs", offset_of!(Knobs, pressure.again_secs), false),
    ("pressure.stuns", offset_of!(Knobs, pressure.stuns), false),
    ("pressure.stun_know_secs", offset_of!(Knobs, pressure.stun_know_secs), false),
    ("pressure.stun_moved_secs", offset_of!(Knobs, pressure.stun_moved_secs), false),
    ("pressure.by_way", offset_of!(Knobs, pressure.by_way), true),
    ("pressure.rescue_crew", offset_of!(Knobs, pressure.rescue_crew), true),
    ("pressure.drilled", offset_of!(Knobs, pressure.drilled), true),
    ("pressure.flank_max_m", offset_of!(Knobs, pressure.flank_max_m), false),
    ("pressure.flank_k", offset_of!(Knobs, pressure.flank_k), false),
    ("pressure.flank_slack_m", offset_of!(Knobs, pressure.flank_slack_m), false),
    ("pressure.last_man_secs.0", offset_of!(Knobs, pressure.last_man_secs), false),
    ("pressure.last_man_secs.1", offset_of!(Knobs, pressure.last_man_secs) + 8, false),
    ("pressure.loud_secs", offset_of!(Knobs, pressure.loud_secs), false),
    ("pressure.out_secs.0", offset_of!(Knobs, pressure.out_secs), false),
    ("pressure.out_secs.1", offset_of!(Knobs, pressure.out_secs) + 8, false),
    ("pressure.gather_secs", offset_of!(Knobs, pressure.gather_secs), false),
    ("pressure.flank_secs", offset_of!(Knobs, pressure.flank_secs), false),
    ("pressure.siege_secs", offset_of!(Knobs, pressure.siege_secs), false),
    ("pressure.word_secs", offset_of!(Knobs, pressure.word_secs), false),
    ("pressure.way_out", offset_of!(Knobs, pressure.way_out), true),
    ("pressure.way_out_m", offset_of!(Knobs, pressure.way_out_m), false),
    ("pressure.way_out_secs", offset_of!(Knobs, pressure.way_out_secs), false),
    ("pressure.sweep", offset_of!(Knobs, pressure.sweep), true),
    ("pressure.sweep_secs", offset_of!(Knobs, pressure.sweep_secs), false),
    ("pressure.sweep_men", offset_of!(Knobs, pressure.sweep_men), false),
    ("pressure.sweep_beat", offset_of!(Knobs, pressure.sweep_beat), false),
    ("pressure.sweep_pair_m", offset_of!(Knobs, pressure.sweep_pair_m), false),
    ("lanes.secs", offset_of!(Knobs, lanes.secs), false),
    ("lanes.reach_m", offset_of!(Knobs, lanes.reach_m), false),
    ("lanes.half_deg", offset_of!(Knobs, lanes.half_deg), false),
    ("lanes.width_m", offset_of!(Knobs, lanes.width_m), false),
    ("lanes.field_m", offset_of!(Knobs, lanes.field_m), false),
    ("lanes.own_m", offset_of!(Knobs, lanes.own_m), false),
    ("lanes.min_m", offset_of!(Knobs, lanes.min_m), false),
    ("lanes.round_k", offset_of!(Knobs, lanes.round_k), false),
    ("lanes.near_m", offset_of!(Knobs, lanes.near_m), false),
    ("lanes.react_secs", offset_of!(Knobs, lanes.react_secs), false),
    ("lanes.call_secs", offset_of!(Knobs, lanes.call_secs), false),
    ("lanes.see_m", offset_of!(Knobs, lanes.see_m), false),
    ("lanes.body_m", offset_of!(Knobs, lanes.body_m), false),
    ("lanes.dear", offset_of!(Knobs, lanes.dear), false),
    ("lanes.ahead_m", offset_of!(Knobs, lanes.ahead_m), false),
    ("lanes.close_m", offset_of!(Knobs, lanes.close_m), false),
    ("lanes.hot_secs", offset_of!(Knobs, lanes.hot_secs), false),
    ("lanes.round_men", offset_of!(Knobs, lanes.round_men), false),
    ("lanes.think_secs", offset_of!(Knobs, lanes.think_secs), false),
    ("lanes.round_max_m", offset_of!(Knobs, lanes.round_max_m), false),
    ("lanes.round_gap", offset_of!(Knobs, lanes.round_gap), false),
    ("lanes.round_side_deg", offset_of!(Knobs, lanes.round_side_deg), false),
    ("lanes.round_show_m", offset_of!(Knobs, lanes.round_show_m), false),
    ("lanes.shift_m", offset_of!(Knobs, lanes.shift_m), false),
    ("lanes.stun_m", offset_of!(Knobs, lanes.stun_m), false),
    ("lanes.bang_m", offset_of!(Knobs, lanes.bang_m), false),
    ("lanes.push_secs", offset_of!(Knobs, lanes.push_secs), false),
    ("lanes.push_m", offset_of!(Knobs, lanes.push_m), false),
    ("lanes.push_fast", offset_of!(Knobs, lanes.push_fast), false),
    ("lanes.stun_gap", offset_of!(Knobs, lanes.stun_gap), false),
    ("lanes.moved_m", offset_of!(Knobs, lanes.moved_m), false),
    ("lanes.moved_secs", offset_of!(Knobs, lanes.moved_secs), false),
    ("lanes.peek_quiet", offset_of!(Knobs, lanes.peek_quiet), false),
    ("lanes.peek_gap", offset_of!(Knobs, lanes.peek_gap), false),
    ("lanes.peek_max", offset_of!(Knobs, lanes.peek_max), false),
    ("lanes.peek_lost", offset_of!(Knobs, lanes.peek_lost), false),
    ("lanes.peek_reach_m", offset_of!(Knobs, lanes.peek_reach_m), false),
    ("lanes.peek_near_m", offset_of!(Knobs, lanes.peek_near_m), false),
    ("lanes.peek_look", offset_of!(Knobs, lanes.peek_look), false),
    ("lanes.peek_sure", offset_of!(Knobs, lanes.peek_sure), false),
    ("lanes.peek_keep", offset_of!(Knobs, lanes.peek_keep), false),
    ("lanes.peek_pace", offset_of!(Knobs, lanes.peek_pace), false),
    ("lanes.peek_lean", offset_of!(Knobs, lanes.peek_lean), false),
    ("lanes.pin_rounds", offset_of!(Knobs, lanes.pin_rounds), false),
    ("lanes.pin_every", offset_of!(Knobs, lanes.pin_every), false),
    ("lanes.pin_snap", offset_of!(Knobs, lanes.pin_snap), false),
    ("lanes.pin_gap", offset_of!(Knobs, lanes.pin_gap), false),
    ("lanes.pin_max", offset_of!(Knobs, lanes.pin_max), false),
    ("lanes.pin_rest", offset_of!(Knobs, lanes.pin_rest), false),
    ("lanes.pin_spread", offset_of!(Knobs, lanes.pin_spread), false),
    ("lanes.pin_aim_deg", offset_of!(Knobs, lanes.pin_aim_deg), false),
    ("lanes.pin_pace", offset_of!(Knobs, lanes.pin_pace), false),
    ("lanes.pin_show_m", offset_of!(Knobs, lanes.pin_show_m), false),
    ("lanes.pin_min_m", offset_of!(Knobs, lanes.pin_min_m), false),
    ("lanes.look_m", offset_of!(Knobs, lanes.look_m), false),
    ("lanes.frag_think", offset_of!(Knobs, lanes.frag_think), false),
    ("lanes.frag_gap", offset_of!(Knobs, lanes.frag_gap), false),
    ("lanes.frag_look", offset_of!(Knobs, lanes.frag_look), false),
    ("lanes.frag_step_m", offset_of!(Knobs, lanes.frag_step_m), false),
    ("lanes.max", offset_of!(Knobs, lanes.max), false),
    ("gunhold.cover_m", offset_of!(Knobs, gunhold.cover_m), false),
    ("gunhold.near_m", offset_of!(Knobs, gunhold.near_m), false),
    ("gunhold.rear_deg", offset_of!(Knobs, gunhold.rear_deg), false),
    ("gunhold.keep_secs", offset_of!(Knobs, gunhold.keep_secs), false),
    ("gunhold.peek_every", offset_of!(Knobs, gunhold.peek_every), false),
    ("gunhold.peek_secs", offset_of!(Knobs, gunhold.peek_secs), false),
    ("gunhold.show_m", offset_of!(Knobs, gunhold.show_m), false),
    ("gunhold.shy_s", offset_of!(Knobs, gunhold.shy_s), false),
    ("gunhold.dash_m", offset_of!(Knobs, gunhold.dash_m), false),
    ("gunhold.face_deg", offset_of!(Knobs, gunhold.face_deg), false),
    ("gunhold.flank_max_m", offset_of!(Knobs, gunhold.flank_max_m), false),
    ("gunhold.react.0", offset_of!(Knobs, gunhold.react), false),
    ("gunhold.react.1", offset_of!(Knobs, gunhold.react) + 8, false),
    ("hard.reach_m", offset_of!(Knobs, hard.reach_m), false),
    ("hard.stuns", offset_of!(Knobs, hard.stuns), false),
    ("drilled.shaken", offset_of!(Knobs, drilled.shaken), false),
    ("squads.think.0", offset_of!(Knobs, squads.think), false),
    ("squads.think.1", offset_of!(Knobs, squads.think) + 8, false),
    ("squads.net_lag", offset_of!(Knobs, squads.net_lag), false),
    ("squads.help_men", offset_of!(Knobs, squads.help_men), false),
    ("squads.flank_men", offset_of!(Knobs, squads.flank_men), false),
    ("squads.again_secs", offset_of!(Knobs, squads.again_secs), false),
    ("squads.settle_secs", offset_of!(Knobs, squads.settle_secs), false),
    ("squads.quiet_secs", offset_of!(Knobs, squads.quiet_secs), false),
    ("squads.recall_secs", offset_of!(Knobs, squads.recall_secs), false),
    ("squads.reinforce.0", offset_of!(Knobs, squads.reinforce), false),
    ("squads.reinforce.1", offset_of!(Knobs, squads.reinforce) + 8, false),
    ("squads.ready_beat", offset_of!(Knobs, squads.ready_beat), false),
    ("squads.fire_m", offset_of!(Knobs, squads.fire_m), false),
    ("squads.rear_m", offset_of!(Knobs, squads.rear_m), false),
    ("squads.soft_m", offset_of!(Knobs, squads.soft_m), false),
    ("squads.mouth_m", offset_of!(Knobs, squads.mouth_m), false),
    ("squads.hold_back_m", offset_of!(Knobs, squads.hold_back_m), false),
    ("squads.hold_secs", offset_of!(Knobs, squads.hold_secs), false),
    ("squads.drill_net", offset_of!(Knobs, squads.drill_net), true),
    ("squads.net_k", offset_of!(Knobs, squads.net_k), false),
    ("squads.reinforce_k", offset_of!(Knobs, squads.reinforce_k), false),
    ("squads.react_k", offset_of!(Knobs, squads.react_k), false),
    ("squads.nerve_k", offset_of!(Knobs, squads.nerve_k), false),
    ("squads.follow_m", offset_of!(Knobs, squads.follow_m), false),
    ("squads.drill_own", offset_of!(Knobs, squads.drill_own), true),
    ("squads.own_up", offset_of!(Knobs, squads.own_up), false),
    ("squads.drill_back", offset_of!(Knobs, squads.drill_back), true),
    ("squads.back_beat", offset_of!(Knobs, squads.back_beat), false),
    ("squads.sweep_secs", offset_of!(Knobs, squads.sweep_secs), false),
    ("squads.sweep_men", offset_of!(Knobs, squads.sweep_men), false),
    ("squads.sweep_beat", offset_of!(Knobs, squads.sweep_beat), false),
    ("squads.check_every", offset_of!(Knobs, squads.check_every), false),
    ("squads.check_jitter", offset_of!(Knobs, squads.check_jitter), false),
    ("squads.check_alert_k", offset_of!(Knobs, squads.check_alert_k), false),
    ("squads.check_ask", offset_of!(Knobs, squads.check_ask), false),
    ("squads.check_send", offset_of!(Knobs, squads.check_send), false),
    ("squads.check_heard.0", offset_of!(Knobs, squads.check_heard), false),
    ("squads.check_heard.1", offset_of!(Knobs, squads.check_heard) + 8, false),
    ("squads.check_hear_m", offset_of!(Knobs, squads.check_hear_m), false),
    ("squads.look_men", offset_of!(Knobs, squads.look_men), false),
    ("squads.call_secs.0", offset_of!(Knobs, squads.call_secs), false),
    ("squads.call_secs.1", offset_of!(Knobs, squads.call_secs) + 8, false),
    ("squads.call_cut_ask.0", offset_of!(Knobs, squads.call_cut_ask), false),
    ("squads.call_cut_ask.1", offset_of!(Knobs, squads.call_cut_ask) + 8, false),
    ("squads.net_every", offset_of!(Knobs, squads.net_every), false),
    ("squads.net_beat.0", offset_of!(Knobs, squads.net_beat), false),
    ("squads.net_beat.1", offset_of!(Knobs, squads.net_beat) + 8, false),
    ("squads.net_look_men", offset_of!(Knobs, squads.net_look_men), false),
    ("squads.net_stops", offset_of!(Knobs, squads.net_stops), false),
    ("cues.rows", offset_of!(Knobs, cues.rows), false),
    ("cues.secs", offset_of!(Knobs, cues.secs), false),
    ("cues.min_show", offset_of!(Knobs, cues.min_show), false),
    ("cues.per_class", offset_of!(Knobs, cues.per_class), false),
    ("cues.dup_secs", offset_of!(Knobs, cues.dup_secs), false),
    ("cues.squelch", offset_of!(Knobs, cues.squelch), false),
    ("cues.man_secs", offset_of!(Knobs, cues.man_secs), false),
    ("cues.again_secs", offset_of!(Knobs, cues.again_secs), false),
    ("cues.still_secs", offset_of!(Knobs, cues.still_secs), false),
    ("cues.still_quiet", offset_of!(Knobs, cues.still_quiet), false),
    ("cues.stay_secs", offset_of!(Knobs, cues.stay_secs), false),
    ("cues.overheard_gap", offset_of!(Knobs, cues.overheard_gap), false),
    ("wx_sim.rain_k", offset_of!(Knobs, wx_sim.rain_k), false),
    ("wx_sim.thunder_k", offset_of!(Knobs, wx_sim.thunder_k), false),
    ("wx_sim.thunder_secs", offset_of!(Knobs, wx_sim.thunder_secs), false),
    ("wx_sim.thunder_loud", offset_of!(Knobs, wx_sim.thunder_loud), false),
    ("wx_sim.roof_bed", offset_of!(Knobs, wx_sim.roof_bed), false),
    ("air_sim.on", offset_of!(Knobs, air_sim.on), true),
    ("air_sim.ground", offset_of!(Knobs, air_sim.ground), false),
    ("air_sim.min_top", offset_of!(Knobs, air_sim.min_top), false),
    ("air_sim.max_foot", offset_of!(Knobs, air_sim.max_foot), false),
    ("air_sim.roof_share", offset_of!(Knobs, air_sim.roof_share), false),
    ("air_sim.glass_open", offset_of!(Knobs, air_sim.glass_open), false),
    ("air_sim.leaf_step", offset_of!(Knobs, air_sim.leaf_step), false),
    ("air_sim.vent_out", offset_of!(Knobs, air_sim.vent_out), false),
    ("air_sim.gust_k", offset_of!(Knobs, air_sim.gust_k), false),
    ("air_sim.rate", offset_of!(Knobs, air_sim.rate), false),
    ("air_sim.sigma_min", offset_of!(Knobs, air_sim.sigma_min), false),
    ("air_sim.sigma_max", offset_of!(Knobs, air_sim.sigma_max), false),
    ("air_sim.puff_rate", offset_of!(Knobs, air_sim.puff_rate), false),
    ("air_sim.settle", offset_of!(Knobs, air_sim.settle), false),
    ("laststand.moved_secs", offset_of!(Knobs, laststand.moved_secs), false),
    ("laststand.still_secs", offset_of!(Knobs, laststand.still_secs), false),
    ("laststand.moved_gap", offset_of!(Knobs, laststand.moved_gap), false),
    ("laststand.few", offset_of!(Knobs, laststand.few), false),
    ("laststand.quiet", offset_of!(Knobs, laststand.quiet), false),
    ("laststand.first.0", offset_of!(Knobs, laststand.first), false),
    ("laststand.first.1", offset_of!(Knobs, laststand.first) + 8, false),
    ("laststand.every", offset_of!(Knobs, laststand.every), false),
    ("laststand.again", offset_of!(Knobs, laststand.again), false),
    ("laststand.answer.0", offset_of!(Knobs, laststand.answer), false),
    ("laststand.answer.1", offset_of!(Knobs, laststand.answer) + 8, false),
    ("laststand.between.0", offset_of!(Knobs, laststand.between), false),
    ("laststand.between.1", offset_of!(Knobs, laststand.between) + 8, false),
    ("laststand.listen.0", offset_of!(Knobs, laststand.listen), false),
    ("laststand.listen.1", offset_of!(Knobs, laststand.listen) + 8, false),
    ("laststand.max_m", offset_of!(Knobs, laststand.max_m), false),
    ("laststand.clear_m", offset_of!(Knobs, laststand.clear_m), false),
    ("laststand.known_secs", offset_of!(Knobs, laststand.known_secs), false),
    ("laststand.go.0", offset_of!(Knobs, laststand.go), false),
    ("laststand.go.1", offset_of!(Knobs, laststand.go) + 8, false),
    ("laststand.legs", offset_of!(Knobs, laststand.legs), false),
    ("laststand.dig.0", offset_of!(Knobs, laststand.dig), false),
    ("laststand.dig.1", offset_of!(Knobs, laststand.dig) + 8, false),
    ("laststand.thump.0", offset_of!(Knobs, laststand.thump), false),
    ("laststand.thump.1", offset_of!(Knobs, laststand.thump) + 8, false),
    ("laststand.thump_level", offset_of!(Knobs, laststand.thump_level), false),
    ("stackroom.apart", offset_of!(Knobs, stackroom.apart), false),
    ("stackroom.squeeze", offset_of!(Knobs, stackroom.squeeze), false),
    ("stackroom.round_m", offset_of!(Knobs, stackroom.round_m), false),
    ("stackroom.round_k", offset_of!(Knobs, stackroom.round_k), false),
    ("stackroom.walk_m", offset_of!(Knobs, stackroom.walk_m), false),
    ("stackroom.next_walk_m", offset_of!(Knobs, stackroom.next_walk_m), false),
    ("stackroom.next_m", offset_of!(Knobs, stackroom.next_m), false),
    ("stackroom.lane_back", offset_of!(Knobs, stackroom.lane_back), false),
    ("stackroom.view_ajar", offset_of!(Knobs, stackroom.view_ajar), false),
    ("stackroom.view_cells", offset_of!(Knobs, stackroom.view_cells), false),
    ("stackroom.view_far", offset_of!(Knobs, stackroom.view_far), false),
    ("stackroom.view_eye", offset_of!(Knobs, stackroom.view_eye), false),
    ("stackroom.view_step", offset_of!(Knobs, stackroom.view_step), false),
    ("stackroom.view_past", offset_of!(Knobs, stackroom.view_past), false),
    ("stackroom.view_off", offset_of!(Knobs, stackroom.view_off), false),
    ("stackroom.view_apart", offset_of!(Knobs, stackroom.view_apart), false),
    ("stackroom.view_out", offset_of!(Knobs, stackroom.view_out), false),
    ("stackroom.view_men", offset_of!(Knobs, stackroom.view_men), false),
    ("stackroom.view_wall", offset_of!(Knobs, stackroom.view_wall), false),
    ("rearwatch.reach", offset_of!(Knobs, rearwatch.reach), false),
    ("rearwatch.ajar", offset_of!(Knobs, rearwatch.ajar), false),
    ("rearwatch.grace", offset_of!(Knobs, rearwatch.grace), false),
    ("rearwatch.short", offset_of!(Knobs, rearwatch.short), false),
    ("rearwatch.room_secs", offset_of!(Knobs, rearwatch.room_secs), false),
    ("rearwatch.room_m", offset_of!(Knobs, rearwatch.room_m), false),
    ("stackline.ajar", offset_of!(Knobs, stackline.ajar), false),
    ("stackline.bins", offset_of!(Knobs, stackline.bins), false),
    ("stackline.eye", offset_of!(Knobs, stackline.eye), false),
    ("stackline.inset", offset_of!(Knobs, stackline.inset), false),
    ("stackline.on_mark", offset_of!(Knobs, stackline.on_mark), false),
    ("sneak.on", offset_of!(Knobs, sneak.on), true),
    ("sneak.min_pace", offset_of!(Knobs, sneak.min_pace), false),
    ("sneak.low_pace", offset_of!(Knobs, sneak.low_pace), false),
    ("sneak.cross_pace", offset_of!(Knobs, sneak.cross_pace), false),
    ("sneak.step", offset_of!(Knobs, sneak.step), false),
    ("sneak.trail_m", offset_of!(Knobs, sneak.trail_m), false),
    ("sneak.back", offset_of!(Knobs, sneak.back), false),
    ("sneak.ahead_m", offset_of!(Knobs, sneak.ahead_m), false),
    ("sneak.lit_at", offset_of!(Knobs, sneak.lit_at), false),
    ("sneak.seen_rate", offset_of!(Knobs, sneak.seen_rate), false),
    ("sneak.cone_deg", offset_of!(Knobs, sneak.cone_deg), false),
    ("sneak.near_m", offset_of!(Knobs, sneak.near_m), false),
    ("sneak.cross_m", offset_of!(Knobs, sneak.cross_m), false),
    ("sneak.patience", offset_of!(Knobs, sneak.patience), false),
    ("sneak.cross_secs", offset_of!(Knobs, sneak.cross_secs), false),
    ("sneak.budget", offset_of!(Knobs, sneak.budget), false),
    ("sneak.margin", offset_of!(Knobs, sneak.margin), false),
    ("sneak.memory", offset_of!(Knobs, sneak.memory), false),
    ("sneak.nag_secs", offset_of!(Knobs, sneak.nag_secs), false),
    ("sneak.cue_gap", offset_of!(Knobs, sneak.cue_gap), false),
    ("sneak.stand_m", offset_of!(Knobs, sneak.stand_m), false),
    ("sneak.stand_near", offset_of!(Knobs, sneak.stand_near), false),
    ("sneak.side_m", offset_of!(Knobs, sneak.side_m), false),
    ("sneak.side_stop", offset_of!(Knobs, sneak.side_stop), false),
    ("sneak.door_m", offset_of!(Knobs, sneak.door_m), false),
    ("sneak.door_pad", offset_of!(Knobs, sneak.door_pad), false),
    ("sneak.door_settle", offset_of!(Knobs, sneak.door_settle), false),
    ("sneak.hide_gap", offset_of!(Knobs, sneak.hide_gap), false),
    ("sneak.noise_walk_m", offset_of!(Knobs, sneak.noise_walk_m), false),
    ("sneak.plan_secs", offset_of!(Knobs, sneak.plan_secs), false),
    ("sneak.skill", offset_of!(Knobs, sneak.skill), false),
    ("pairup.on", offset_of!(Knobs, pairup.on), true),
    ("spotted.on", offset_of!(Knobs, spotted.on), true),
    ("spotted.leash_m", offset_of!(Knobs, spotted.leash_m), false),
    ("spotted.slip_m", offset_of!(Knobs, spotted.slip_m), false),
    ("spotted.lit_at", offset_of!(Knobs, spotted.lit_at), false),
    ("spotted.slip_wait", offset_of!(Knobs, spotted.slip_wait), false),
    ("spotted.wary_m", offset_of!(Knobs, spotted.wary_m), false),
    ("spotted.ear_m", offset_of!(Knobs, spotted.ear_m), false),
    ("spotted.linger_secs", offset_of!(Knobs, spotted.linger_secs), false),
    ("spotted.brace_secs", offset_of!(Knobs, spotted.brace_secs), false),
    ("spotted.brace_m", offset_of!(Knobs, spotted.brace_m), false),
    ("spotted.take_m", offset_of!(Knobs, spotted.take_m), false),
    ("spotted.take_reach", offset_of!(Knobs, spotted.take_reach), false),
    ("spotted.take_secs", offset_of!(Knobs, spotted.take_secs), false),
    ("spotted.fresh_secs", offset_of!(Knobs, spotted.fresh_secs), false),
    ("spotted.caught_m", offset_of!(Knobs, spotted.caught_m), false),
    ("spotted.last_secs", offset_of!(Knobs, spotted.last_secs), false),
    ("spotted.step_m", offset_of!(Knobs, spotted.step_m), false),
    ("spotted.face_deg", offset_of!(Knobs, spotted.face_deg), false),
    ("spotted.close_m", offset_of!(Knobs, spotted.close_m), false),
    ("nightbreach.sam_at.0", offset_of!(Knobs, nightbreach.sam_at), false),
    ("nightbreach.sam_at.1", offset_of!(Knobs, nightbreach.sam_at) + 8, false),
    ("nightbreach.sam_at.2", offset_of!(Knobs, nightbreach.sam_at) + 16, false),
    ("nightbreach.sam_yaw", offset_of!(Knobs, nightbreach.sam_yaw), false),
    ("nightbreach.squad", offset_of!(Knobs, nightbreach.squad), false),
    ("roomclear.seed", offset_of!(Knobs, roomclear.seed), false),
    ("roomclear.way", offset_of!(Knobs, roomclear.way), false),
    ("verdict.win_secs", offset_of!(Knobs, verdict.win_secs), false),
    ("verdict.loss_secs", offset_of!(Knobs, verdict.loss_secs), false),
    ("unseen.pace_at", offset_of!(Knobs, unseen.pace_at), false),
    ("unseen.linger", offset_of!(Knobs, unseen.linger), false),
    ("unseen.min_pace", offset_of!(Knobs, unseen.min_pace), false),
    ("unseen.low_pace", offset_of!(Knobs, unseen.low_pace), false),
    ("unseen.gain", offset_of!(Knobs, unseen.gain), false),
    ("unseen.cap", offset_of!(Knobs, unseen.cap), false),
    ("unseen.wall_off", offset_of!(Knobs, unseen.wall_off), false),
    ("unseen.wall_out", offset_of!(Knobs, unseen.wall_out), false),
    ("unseen.wall_cross", offset_of!(Knobs, unseen.wall_cross), false),
    ("unseen.wall_end", offset_of!(Knobs, unseen.wall_end), false),
    ("unseen.low_step", offset_of!(Knobs, unseen.low_step), false),
    ("unseen.walk_step", offset_of!(Knobs, unseen.walk_step), false),
    ("unseen.cue_gap", offset_of!(Knobs, unseen.cue_gap), false),
    ("unseen.wall_secs", offset_of!(Knobs, unseen.wall_secs), false),
    ("seenwalk.lit_at", offset_of!(Knobs, seenwalk.lit_at), false),
    ("seenwalk.dear", offset_of!(Knobs, seenwalk.dear), false),
    ("seenwalk.long", offset_of!(Knobs, seenwalk.long), false),
    ("seenwalk.less", offset_of!(Knobs, seenwalk.less), false),
    ("seenwalk.eye_step", offset_of!(Knobs, seenwalk.eye_step), false),
    ("seenwalk.eye_y", offset_of!(Knobs, seenwalk.eye_y), false),
    ("seenwalk.margin", offset_of!(Knobs, seenwalk.margin), false),
    ("holdfire.lift_secs", offset_of!(Knobs, holdfire.lift_secs), false),
    ("slots.on", offset_of!(Knobs, slots.on), true),
    ("slots.ok", offset_of!(Knobs, slots.ok), true),
    ("slots.table.v", offset_of!(Knobs, slots.table.v), false),
    ("bigslot.on", offset_of!(Knobs, bigslot.on), true),
    ("bigslot.swings", offset_of!(Knobs, bigslot.swings), false),
    ("bigslot.swing", offset_of!(Knobs, bigslot.swing), false),
    ("bigslot.windup", offset_of!(Knobs, bigslot.windup), false),
    ("bigslot.draw", offset_of!(Knobs, bigslot.draw), false),
    ("bigslot.shells", offset_of!(Knobs, bigslot.shells), false),
    ("bigslot.equip", offset_of!(Knobs, bigslot.equip), false),
    ("bigslot.stow", offset_of!(Knobs, bigslot.stow), false),
    ("bigslot.reach", offset_of!(Knobs, bigslot.reach), false),
    ("bigslot.heft", offset_of!(Knobs, bigslot.heft), false),
    ("bigslot.equip_w", offset_of!(Knobs, bigslot.equip_w), false),
    ("bigslot.draw_w", offset_of!(Knobs, bigslot.draw_w), false),
    ("bigslot.stow_w", offset_of!(Knobs, bigslot.stow_w), false),
    ("bigslot.flow", offset_of!(Knobs, bigslot.flow), false),
];

/// each is of the type the list's reader takes it for
fn typed(k: &Knobs) {
    let _: &f64 = &k.aim.guard.settle_secs;
    let _: &f64 = &k.aim.guard.range_ref;
    let _: &f64 = &k.aim.guard.near_k;
    let _: &f64 = &k.aim.guard.acquire_min;
    let _: &f64 = &k.aim.guard.unsettled_k;
    let _: &f64 = &k.aim.guard.on_deg;
    let _: &f64 = &k.aim.guard.lose_rate;
    let _: &f64 = &k.aim.guard.mem_secs;
    let _: &f64 = &k.aim.guard.jump_deg;
    let _: &f64 = &k.aim.guard.target_move_k;
    let _: &f64 = &k.aim.guard.target_move_cap;
    let _: &f64 = &k.aim.guard.rate_tau;
    let _: &f64 = &k.aim.guard.shooter_move_k;
    let _: &f64 = &k.aim.guard.feet_tau;
    let _: &f64 = &k.aim.guard.steady_pace;
    let _: &f64 = &k.aim.guard.stop_to_shoot;
    let _: &f64 = &k.aim.guard.moving_track_cap;
    let _: &f64 = &k.aim.guard.shoot_band;
    let _: &f64 = &k.aim.guard.brake_lead;
    let _: &f64 = &k.aim.player.settle_secs;
    let _: &f64 = &k.aim.player.range_ref;
    let _: &f64 = &k.aim.player.near_k;
    let _: &f64 = &k.aim.player.unsettled_k;
    let _: &f64 = &k.aim.player.lose_rate;
    let _: &f64 = &k.aim.player.base_stand;
    let _: &f64 = &k.aim.player.base_crouch;
    let _: &f64 = &k.aim.player.target_move_k;
    let _: &f64 = &k.aim.player.target_move_cap;
    let _: &f64 = &k.aim.player.rate_tau;
    let _: &f64 = &k.aim.player.jump_deg;
    let _: &f64 = &k.aim.player.shooter_move_k;
    let _: &f64 = &k.aim.player.feet_tau;
    let _: &f64 = &k.aim.player.steady_pace;
    let _: &f64 = &k.aim.player.sprint_cap;
    let _: &f64 = &k.aim.player.walk_cap;
    let _: &f64 = &k.aim.player.moving_above;
    let _: &f64 = &k.aim.player.crouch_k;
    let _: &f64 = &k.trained.settle_k;
    let _: &f64 = &k.trained.spread_k;
    let _: &f64 = &k.trained.react_cap;
    let _: &f64 = &k.body_light.lit;
    let _: &f64 = &k.body_light.lit_near;
    let _: &f64 = &k.body_light.near_m;
    let _: &f64 = &k.body_light.far_m;
    let _: &f64 = &k.body_light.smoke;
    let _: &f64 = &k.guard_eye.range;
    let _: &f64 = &k.guard_eye.half_deg;
    let _: &f64 = &k.guard_eye.near_m;
    let _: &f64 = &k.guard_eye.near_deg;
    let _: &f64 = &k.guard_eye.far_fix_at;
    let _: &f64 = &k.guard_eye.far_range;
    let _: &f64 = &k.guard_eye.far_lo;
    let _: &f64 = &k.guard_eye.far_hi;
    let _: &f64 = &k.guard_eye.far_k;
    let _: &f64 = &k.needs.rate_k;
    let _: &f64 = &k.needs.max_away;
    let _: &f64 = &k.needs.announce_lead;
    let _: &f64 = &k.needs.say_lead;
    let _: &f64 = &k.needs.stretch_lead;
    let _: &f64 = &k.needs.partner_m;
    let _: &f64 = &k.needs.reply_after;
    let _: &f64 = &k.needs.min_wait;
    let _: &f64 = &k.needs.min_trip_m;
    let _: &f64 = &k.needs.after_cancel;
    let _: &f64 = &k.needs.after_alert;
    let _: &f64 = &k.needs.rest_secs;
    let _: &f64 = &k.needs.floor_gap_secs;
    let _: &f64 = &k.needs.pace_k;
    let _: &f64 = &k.needs.stuck_secs;
    let _: &f64 = &k.whistle.level;
    let _: &f64 = &k.whistle.reach_k;
    let _: &f64 = &k.whistle.horizon;
    let _: &f64 = &k.whistle.cooldown;
    let _: &f64 = &k.whistle.twice_window;
    let _: &f64 = &k.whistle.awareness;
    let _: &f64 = &k.whistle.react_t;
    let _: &f64 = &k.whistle.walk_floor;
    let _: &f64 = &k.whistle.glance_secs;
    let _: &f64 = &k.whistle.glance_max;
    let _: &f64 = &k.whistle.gain;
    let _: &f64 = &k.wet.soak_per_sec;
    let _: &f64 = &k.wet.dry_per_sec;
    let _: &f64 = &k.wet.bead_runoff_secs;
    let _: &f64 = &k.wet.bead_density;
    let _: &f64 = &k.wet.bead_size;
    let _: &f64 = &k.wet.trail_len;
    let _: &f64 = &k.wet.rough_wet;
    let _: &f64 = &k.wet.rough_dry;
    let _: &f64 = &k.wet.darken;
    let _: &f64 = &k.wet.darken_mul;
    let _: &f64 = &k.wet.spec_gain;
    let _: &f64 = &k.wet.drip_secs;
    let _: &f64 = &k.wet.prints;
    let _: &f64 = &k.wet.line_band;
    let _: &f64 = &k.wet.print_fade_secs;
    let _: &f64 = &k.wind.def;
    let _: &f64 = &k.wind.vent_k;
    let _: &f64 = &k.wind.storm_k;
    let _: &f64 = &k.wind.door_k;
    let _: &f64 = &k.wind.door_reach;
    let _: &f64 = &k.wind.door_spread_deg;
    let _: &f64 = &k.wind.door_open_rad;
    let _: &f64 = &k.wind.gust_amp;
    let _: &f64 = &k.wind.gust_period;
    let _: &f64 = &k.wind.storm_gust_amp;
    let _: &f64 = &k.wind.storm_gust_period;
    let _: &f64 = &k.wind.ramp_up;
    let _: &f64 = &k.wind.ramp_down;
    let _: &f64 = &k.wind.smoke_relax;
    let _: &f64 = &k.wind.sheet_wake;
    let _: &f64 = &k.wind.sheet_kick;
    let _: &f64 = &k.wind.debris_k;
    let _: &f64 = &k.wind.near_m;
    let _: &f64 = &k.civ.eye.range;
    let _: &f64 = &k.civ.eye.half_deg;
    let _: &f64 = &k.civ.eye.near_m;
    let _: &f64 = &k.civ.eye.near_deg;
    let _: &f64 = &k.civ.eye_busy.range;
    let _: &f64 = &k.civ.eye_busy.half_deg;
    let _: &f64 = &k.civ.eye_busy.near_m;
    let _: &f64 = &k.civ.eye_busy.near_deg;
    let _: &f64 = &k.civ.eye_alarmed.range;
    let _: &f64 = &k.civ.eye_alarmed.half_deg;
    let _: &f64 = &k.civ.eye_alarmed.near_m;
    let _: &f64 = &k.civ.eye_alarmed.near_deg;
    let _: &f64 = &k.civ.keen_k;
    let _: &f64 = &k.civ.gun_k;
    let _: &f64 = &k.civ.gun_m;
    let _: &f64 = &k.civ.glimpse_at;
    let _: &f64 = &k.civ.spot_at;
    let _: &f64 = &k.civ.startle_secs;
    let _: &f64 = &k.civ.startle_again_secs;
    let _: &f64 = &k.civ.body_m;
    let _: &[f64; 2] = &k.civ.freeze;
    let _: &f64 = &k.civ.run_speed;
    let _: &f64 = &k.civ.walk_speed;
    let _: &f64 = &k.civ.creep_speed;
    let _: &f64 = &k.civ.report_m;
    let _: &f64 = &k.civ.report_secs;
    let _: &f64 = &k.civ.phone_secs;
    let _: &f64 = &k.civ.guard_bonus_m;
    let _: &f64 = &k.civ.exit_malus_m;
    let _: &f64 = &k.civ.past_threat_m;
    let _: &f64 = &k.civ.corner_m;
    let _: &f64 = &k.civ.reflee_m;
    let _: &f64 = &k.civ.lone_secs;
    let _: &f64 = &k.civ.settle_secs;
    let _: &f64 = &k.civ.comply_min;
    let _: &f64 = &k.civ.scream_m;
    let _: &f64 = &k.civ.see_m;
    let _: &f64 = &k.civ.see_lit;
    let _: &f64 = &k.civ.found_m;
    let _: &f64 = &k.civ.found_secs;
    let _: &f64 = &k.civ.outside_call_secs;
    let _: &f64 = &k.civ.greet_m;
    let _: &f64 = &k.civ.greet_pair_secs;
    let _: &f64 = &k.civ.greet_floor_secs;
    let _: &f64 = &k.chaos.def;
    let _: &f64 = &k.chaos.heat_tau;
    let _: &f64 = &k.chaos.cell;
    let _: &f64 = &k.chaos.t_decal;
    let _: &f64 = &k.chaos.t_paper;
    let _: &f64 = &k.chaos.t_burst;
    let _: &f64 = &k.chaos.t_puff;
    let _: &f64 = &k.chaos.t_knock;
    let _: &f64 = &k.chaos.t_tile;
    let _: &f64 = &k.chaos.bang_heat;
    let _: &f64 = &k.chaos.puffs_per_sec;
    let _: &f64 = &k.chaos.puffs_per_frame;
    let _: &f64 = &k.chaos.papers_per_sec;
    let _: &f64 = &k.chaos.knocks_per_sec;
    let _: &f64 = &k.chaos.tile_gap_secs;
    let _: &f64 = &k.chaos.tiles_per_encounter;
    let _: &f64 = &k.chaos.decals_per_sec;
    let _: &f64 = &k.chaos.paper_reach_m;
    let _: &f64 = &k.chaos.knock_reach_m;
    let _: &f64 = &k.chaos.sheets_per_stack;
    let _: &f64 = &k.chaos.puff.radius;
    let _: &f64 = &k.chaos.puff.density;
    let _: &f64 = &k.chaos.puff.ttl;
    let _: &f64 = &k.chaos.puff.speed;
    let _: &f64 = &k.chaos.puff.temp;
    let _: &f64 = &k.chaos.puff.push;
    let _: &f64 = &k.chaos.puff.expand;
    let _: &[f64; 3] = &k.chaos.tile.half;
    let _: &[f64; 3] = &k.chaos.tile.albedo;
    let _: &f64 = &k.chaos.vel_low;
    let _: &f64 = &k.chaos.vel_high_k;
    let _: &f64 = &k.chaos.imp.bang.radius;
    let _: &f64 = &k.chaos.imp.bang.strength;
    let _: &f64 = &k.chaos.imp.bang.up;
    let _: &f64 = &k.chaos.imp.impact.radius;
    let _: &f64 = &k.chaos.imp.impact.strength;
    let _: &f64 = &k.chaos.imp.impact.up;
    let _: &f64 = &k.chaos.imp.impact.bias_k;
    let _: &f64 = &k.chaos.imp.impact.debris_m;
    let _: &f64 = &k.chaos.imp.thud.radius;
    let _: &f64 = &k.chaos.imp.thud.strength;
    let _: &f64 = &k.chaos.imp.thud.up;
    let _: &f64 = &k.chaos.imp.thud.per_frame;
    let _: &f64 = &k.chaos.imp.chaos_k;
    let _: &f64 = &k.chaos.loud.fixture_hit_m;
    let _: &f64 = &k.chaos.loud.fixture_blast_m;
    let _: &f64 = &k.chaos.loud.stun_fixture_m;
    let _: &f64 = &k.chaos.loud.fixtures_per_encounter;
    let _: &f64 = &k.chaos.loud.fixture_gap_secs;
    let _: &f64 = &k.chaos.loud.blast_dust_floor;
    let _: &f64 = &k.chaos.loud.blast_dust_puffs;
    let _: &f64 = &k.chaos.loud.heat_k;
    let _: &f64 = &k.chaos.loud.burst_queue;
    let _: &f64 = &k.chaos.loud.burst_secs;
    let _: &f64 = &k.chaos.pall.per_window;
    let _: &f64 = &k.chaos.pall.window_secs;
    let _: &f64 = &k.chaos.pall.slack;
    let _: &f64 = &k.chaos.pall.clear;
    let _: &f64 = &k.chaos.pall.clear_y;
    let _: &f64 = &k.chaos.pall.min_radius;
    let _: &f64 = &k.chaos.pall.max_gain;
    let _: &f64 = &k.chaos.pall.wall.near;
    let _: &f64 = &k.chaos.pall.wall.far;
    let _: &f64 = &k.chaos.pall.wall.radius;
    let _: &f64 = &k.chaos.pall.wall.density;
    let _: &f64 = &k.chaos.pall.wall.ttl;
    let _: &f64 = &k.chaos.pall.wall.speed;
    let _: &f64 = &k.chaos.pall.wall.temp;
    let _: &f64 = &k.chaos.pall.wall.push;
    let _: &f64 = &k.chaos.pall.wall.expand;
    let _: &f64 = &k.chaos.pall.wall.spread;
    let _: &f64 = &k.chaos.pall.wall.out;
    let _: &[f64; 2] = &k.chaos.pall.wall.dust;
    let _: &f64 = &k.chaos.pall.wall.dust_life;
    let _: &f64 = &k.chaos.pall.wall.dust_size;
    let _: &f64 = &k.chaos.pall.wall.dust_speed;
    let _: &f64 = &k.chaos.pall.burst.ring;
    let _: &f64 = &k.chaos.pall.burst.core_gain;
    let _: &f64 = &k.chaos.pall.burst.radius;
    let _: &f64 = &k.chaos.pall.burst.density;
    let _: &f64 = &k.chaos.pall.burst.ttl;
    let _: &f64 = &k.chaos.pall.burst.speed;
    let _: &f64 = &k.chaos.pall.burst.temp;
    let _: &f64 = &k.chaos.pall.burst.push;
    let _: &f64 = &k.chaos.pall.burst.expand;
    let _: &f64 = &k.chaos.pall.burst.spread;
    let _: &f64 = &k.chaos.pall.burst.dust;
    let _: &f64 = &k.chaos.pall.burst.dust_life;
    let _: &f64 = &k.chaos.pall.burst.dust_size;
    let _: &f64 = &k.chaos.pall.burst.dust_speed;
    let _: &[f64; 3] = &k.chaos.pall.burst.smoke;
    let _: &f64 = &k.chaos.mess.stand_y;
    let _: &f64 = &k.chaos.mess.clear_y;
    let _: &f64 = &k.chaos.mess.under;
    let _: &f64 = &k.chaos.mess.min_radius;
    let _: &f64 = &k.chaos.mess.crowd;
    let _: &f64 = &k.chaos.mess.roof_y;
    let _: &f64 = &k.chaos.mess.burst.core;
    let _: &f64 = &k.chaos.mess.burst.lobes;
    let _: &f64 = &k.chaos.mess.burst.reach;
    let _: &f64 = &k.chaos.mess.burst.lobe;
    let _: &f64 = &k.chaos.mess.burst.strength;
    let _: &f64 = &k.chaos.mess.wall.out;
    let _: &f64 = &k.chaos.mess.wall.near.core;
    let _: &f64 = &k.chaos.mess.wall.near.lobes;
    let _: &f64 = &k.chaos.mess.wall.near.reach;
    let _: &f64 = &k.chaos.mess.wall.near.lobe;
    let _: &f64 = &k.chaos.mess.wall.near.strength;
    let _: &f64 = &k.chaos.mess.wall.far.core;
    let _: &f64 = &k.chaos.mess.wall.far.lobes;
    let _: &f64 = &k.chaos.mess.wall.far.reach;
    let _: &f64 = &k.chaos.mess.wall.far.lobe;
    let _: &f64 = &k.chaos.mess.wall.far.strength;
    let _: &f64 = &k.chaos.mess.foot.out;
    let _: &f64 = &k.chaos.mess.foot.radius;
    let _: &f64 = &k.chaos.mess.foot.per_metre;
    let _: &f64 = &k.chaos.mess.foot.strength;
    let _: &f64 = &k.chaos.mess.top.lo;
    let _: &f64 = &k.chaos.mess.top.hi;
    let _: &f64 = &k.chaos.mess.top.strength;
    let _: &f64 = &k.chaos.mess.rubble.least;
    let _: &f64 = &k.chaos.mess.rubble.grow;
    let _: &f64 = &k.chaos.mess.rubble.min;
    let _: &f64 = &k.chaos.mess.rubble.max;
    let _: &f64 = &k.chaos.mess.rubble.crowd;
    let _: &f64 = &k.chaos.mess.rubble.strength;
    let _: &f64 = &k.ring.speed;
    let _: &f64 = &k.ring.gravity;
    let _: &f64 = &k.ring.drag;
    let _: &f64 = &k.ring.radius;
    let _: &f64 = &k.ring.life;
    let _: &f64 = &k.ring.substeps;
    let _: &f64 = &k.ring.ko_below_neck;
    let _: &f64 = &k.ring.stagger;
    let _: &f64 = &k.ring.stagger_fire_cd;
    let _: &f64 = &k.ring.stagger_aware;
    let _: &f64 = &k.ring.stagger_back;
    let _: &f64 = &k.ring.launch_noise;
    let _: &f64 = &k.ring.clack_noise;
    let _: &f64 = &k.ring.ko_noise;
    let _: &f64 = &k.ring.hit_noise;
    let _: &f64 = &k.ring.prop_j;
    let _: &f64 = &k.ring.restitution;
    let _: &f64 = &k.ring.keep_tangent;
    let _: &f64 = &k.ring.spent_speed_max;
    let _: &f64 = &k.ring.solve_iters;
    let _: &f64 = &k.launcher.rings;
    let _: &f64 = &k.launcher.cd;
    let _: &f64 = &k.he40.speed;
    let _: &f64 = &k.he40.gravity;
    let _: &f64 = &k.he40.drag;
    let _: &f64 = &k.he40.radius;
    let _: &f64 = &k.he40.life;
    let _: &f64 = &k.he40.substeps;
    let _: &f64 = &k.he40.solve_iters;
    let _: &f64 = &k.he40.arm_m;
    let _: &f64 = &k.he40.standoff;
    let _: &f64 = &k.he40.launch_noise;
    let _: &f64 = &k.he40.dud_noise;
    let _: &f64 = &k.he40.dud_hit_noise;
    let _: &f64 = &k.he40.prop_j;
    let _: &f64 = &k.he40.restitution;
    let _: &f64 = &k.he40.keep_tangent;
    let _: &f64 = &k.he40.spent_speed_max;
    let _: &f64 = &k.launcher40.rounds;
    let _: &f64 = &k.launcher40.cd;
    let _: &f64 = &k.frag.issue;
    let _: &f64 = &k.frag.fuse;
    let _: &f64 = &k.frag.order_fuse;
    let _: &f64 = &k.frag.squad;
    let _: &f64 = &k.frag.order_in;
    let _: &f64 = &k.frag.lethal_m;
    let _: &f64 = &k.frag.wound_m;
    let _: &f64 = &k.frag.destr_m;
    let _: &f64 = &k.frag.wound_kill;
    let _: &[f64; 2] = &k.frag.stagger;
    let _: &f64 = &k.frag.lift;
    let _: &f64 = &k.frag.glass_m;
    let _: &f64 = &k.frag.glass_power;
    let _: &f64 = &k.frag.pane_energy;
    let _: &f64 = &k.frag.see_m;
    let _: &f64 = &k.frag.hear_m;
    let _: &[f64; 2] = &k.frag.react;
    let _: &f64 = &k.frag.react_jit;
    let _: &f64 = &k.frag.shout_m;
    let _: &f64 = &k.frag.flee_m;
    let _: &f64 = &k.frag.run_speed;
    let _: &f64 = &k.frag.flee_keep;
    let _: &f64 = &k.frag.flee_apart_m;
    let _: &f64 = &k.frag.flee_lane_m;
    let _: &f64 = &k.frag.down_shove;
    let _: &f64 = &k.frag.flee_door_open;
    let _: &f64 = &k.frag.flee_door_in;
    let _: &f64 = &k.frag.flee_door_dip;
    let _: &f64 = &k.frag.flee_door_mouth;
    let _: &f64 = &k.frag.flee_door_leaf;
    let _: &f64 = &k.frag.flee_door_via;
    let _: &f64 = &k.frag.flee_door_via_m;
    let _: &f64 = &k.frag.flee_door_new;
    let _: &f64 = &k.frag.flee_door_most;
    let _: &f64 = &k.frag.flee_door_try;
    let _: &f64 = &k.frag.flee_door_nodes;
    let _: &f64 = &k.frag.def_share;
    let _: &f64 = &k.frag.def_max;
    let _: &f64 = &k.frag.def_fuse;
    let _: &f64 = &k.frag.def_wind;
    let _: &f64 = &k.frag.def_near;
    let _: &f64 = &k.frag.def_far;
    let _: &f64 = &k.frag.def_past;
    let _: &f64 = &k.frag.def_fresh;
    let _: &f64 = &k.frag.def_blocked;
    let _: &f64 = &k.frag.def_bunch;
    let _: &f64 = &k.frag.def_bunch_secs;
    let _: &f64 = &k.frag.def_gap;
    let _: &f64 = &k.frag.def_own_m;
    let _: &[f64; 2] = &k.frag.def_scatter;
    let _: &f64 = &k.frag.cue_m;
    let _: &f64 = &k.shotgun.clear;
    let _: &f64 = &k.shotgun.mark_reach;
    let _: &f64 = &k.shotgun.line;
    let _: &f64 = &k.shotgun.own_beat;
    let _: &f64 = &k.shotgun.own_slack;
    let _: &f64 = &k.shotgun.lock_in;
    let _: &f64 = &k.shotgun.lock_h;
    let _: &f64 = &k.shotgun.shells;
    let _: &f64 = &k.shotgun.aim_cap;
    let _: &f64 = &k.glass.hear_m;
    let _: &f64 = &k.glass.hear_level;
    let _: &f64 = &k.glass.bang_m;
    let _: &f64 = &k.glass.bang_cracks;
    let _: &f64 = &k.glass.r_min;
    let _: &f64 = &k.glass.r_max;
    let _: &f64 = &k.glass.crack_gap;
    let _: &f64 = &k.glass.live_gap;
    let _: &f64 = &k.glass.ear_off;
    let _: &f64 = &k.glassbreak.max_sites;
    let _: &f64 = &k.glassbreak.break_e.annealed;
    let _: &f64 = &k.glassbreak.break_e.tempered;
    let _: &f64 = &k.glassbreak.punch_e;
    let _: &f64 = &k.glassbreak.run_single_e;
    let _: &f64 = &k.glassbreak.run_star;
    let _: &f64 = &k.glassbreak.run_ring_secs;
    let _: &f64 = &k.glassbreak.crack_gap;
    let _: &f64 = &k.glassbreak.merge;
    let _: &f64 = &k.glassbreak.rz_min;
    let _: &f64 = &k.glassbreak.think;
    let _: &f64 = &k.glassbreak.stagger_per_m;
    let _: &f64 = &k.glassbreak.stagger_jit;
    let _: &f64 = &k.glassbreak.hang_area;
    let _: &f64 = &k.glassbreak.grip.side;
    let _: &f64 = &k.glassbreak.grip.top;
    let _: &f64 = &k.glassbreak.grip.wedged;
    let _: &f64 = &k.glassbreak.stand_ny;
    let _: &f64 = &k.glassbreak.stand_foot;
    let _: &f64 = &k.glassbreak.balance_tol;
    let _: &f64 = &k.glassbreak.blast_m;
    let _: &f64 = &k.glassbreak.bang_power;
    let _: &f64 = &k.glassbreak.tooth_area;
    let _: &f64 = &k.glassbreak.tooth_k;
    let _: &f64 = &k.glassbreak.blast_run_k;
    let _: &f64 = &k.glassbreak.blast_dice_k;
    let _: &f64 = &k.glassbreak.blast_sheet_k;
    let _: &f64 = &k.glassbreak.rake_open;
    let _: &f64 = &k.glassbreak.rake_m;
    let _: &f64 = &k.glassbreak.dice_grain;
    let _: &f64 = &k.glassbreak.clump_m;
    let _: &f64 = &k.glassbreak.dice_hang;
    let _: &f64 = &k.glassbreak.dice_speed;
    let _: &f64 = &k.glassbreak.clumps_per_frame;
    let _: &f64 = &k.glassbreak.releases_per_frame;
    let _: &f64 = &k.glassbreak.out;
    let _: &f64 = &k.glassbreak.spin;
    let _: &f64 = &k.glassbreak.blast_out;
    let _: &[f64; 3] = &k.glassbreak.shard_albedo;
    let _: &[f64; 3] = &k.glassbreak.chip_color;
    let _: &f64 = &k.glassbreak.chips_per_m2;
    let _: &f64 = &k.glassbreak.chips_min;
    let _: &f64 = &k.glassbreak.dice_chips_per_m2;
    let _: &f64 = &k.glassbreak.crumb_every;
    let _: &f64 = &k.glassbreak.gust_speed_k;
    let _: &f64 = &k.glassbreak.walk_clear_h;
    let _: &f64 = &k.glasspen.default_energy;
    let _: &f64 = &k.glasspen.ref_thick;
    let _: &f64 = &k.glasspen.oblique_max;
    let _: &f64 = &k.glasspen.graze_cos;
    let _: &f64 = &k.glasspen.min_energy;
    let _: &f64 = &k.glasspen.max_panes;
    let _: &f64 = &k.glasspen.deflect_deg;
    let _: &f64 = &k.glasspen.bend_k;
    let _: &f64 = &k.glasspen.hole_r;
    let _: &f64 = &k.glasspen.hole_k;
    let _: &f64 = &k.glasspen.holes;
    let _: &f64 = &k.glasspen.shell_r;
    let _: &f64 = &k.glasspen.stack_eyes;
    let _: &f64 = &k.glasspen.stack_energy;
    let _: &f64 = &k.glasspen.stack_least_k;
    let _: &f64 = &k.glasspen.fracture_k;
    let _: &f64 = &k.shards.mid_per_m2;
    let _: &f64 = &k.shards.mid_min;
    let _: &f64 = &k.shards.mid_max;
    let _: &f64 = &k.shards.mid_size;
    let _: &f64 = &k.shards.mid_life;
    let _: &f64 = &k.shards.fine_per_m2;
    let _: &f64 = &k.shards.fine_min;
    let _: &f64 = &k.shards.fine_max;
    let _: &f64 = &k.shards.fine_size;
    let _: &f64 = &k.shards.fine_life;
    let _: &f64 = &k.shards.back_share;
    let _: &f64 = &k.shards.hit_fine_min;
    let _: &f64 = &k.shards.hit_back_share;
    let _: &f64 = &k.shards.spall_out;
    let _: &f64 = &k.shards.spall_back;
    let _: &f64 = &k.shards.dice_per_m2;
    let _: &f64 = &k.shards.dice_min;
    let _: &f64 = &k.shards.dice_max;
    let _: &f64 = &k.shards.dice_life;
    let _: &f64 = &k.shards.dice_fine_share;
    let _: &f64 = &k.shards.crater_per_m2;
    let _: &f64 = &k.shards.crater_max;
    let _: &f64 = &k.shards.exit_fines;
    let _: &f64 = &k.shards.exit_speed;
    let _: &f64 = &k.shards.speed;
    let _: &f64 = &k.shards.live_cap;
    let _: &f64 = &k.shards.cap_floor;
    let _: &f64 = &k.shards.over_cap;
    let _: &f64 = &k.shards.press_life;
    let _: &f64 = &k.shards.press_from;
    let _: &f64 = &k.shards.recs_per_frame;
    let _: &f64 = &k.shards.defer_max;
    let _: &f64 = &k.shards.plate_min_area;
    let _: &f64 = &k.shards.plate_share;
    let _: &f64 = &k.shards.child_min_area;
    let _: &f64 = &k.shards.shatter_v0;
    let _: &f64 = &k.shards.shatter_v1;
    let _: &f64 = &k.shards.rest_crumbs;
    let _: &f64 = &k.shards.crumb_size;
    let _: &f64 = &k.shards.cut_edge;
    let _: &f64 = &k.shards.cut_see;
    let _: &f64 = &k.shards.cut_rim;
    let _: &f64 = &k.shards.cut_body;
    let _: &f64 = &k.shards.cut_see_rest;
    let _: &f64 = &k.shards.cut_rest_body;
    let _: &f64 = &k.shards.max_gen;
    let _: &f64 = &k.shards.contact.keep_floor;
    let _: &f64 = &k.shards.contact.keep_wall;
    let _: &f64 = &k.shards.contact.rest;
    let _: &f64 = &k.shards.sounds_per_frame;
    let _: &f64 = &k.shards.pending_max;
    let _: &f64 = &k.shards.sound_thicken;
    let _: &f64 = &k.shards.sound_owed_secs;
    let _: &f64 = &k.shards.level;
    let _: &f64 = &k.shards.event_gap;
    let _: &f64 = &k.shards.crunch_full;
    let _: &f64 = &k.shards.crunch_gain;
    let _: &f64 = &k.shards.crunch_add;
    let _: &f64 = &k.shards.grind;
    let _: &f64 = &k.hold.regrab;
    let _: &f64 = &k.hold.bare_grip;
    let _: &f64 = &k.hold.bare_arch;
    let _: &f64 = &k.hold.bare_elbow;
    let _: &f64 = &k.hold.yank.sink;
    let _: &f64 = &k.hold.yank.in_;
    let _: &f64 = &k.hold.yank.hold;
    let _: &f64 = &k.hold.yank.out;
    let _: &[f64; 3] = &k.hold.lunge.t;
    let _: &f64 = &k.hold.sway.secs;
    let _: &f64 = &k.hold.sway.his;
    let _: &f64 = &k.hold.sway.in_;
    let _: &f64 = &k.hold.sway.out;
    let _: &[f64; 3] = &k.hold.catch.via;
    let _: &[f64; 3] = &k.hold.catch.over;
    let _: &[f64; 2] = &k.hold.catch.at;
    let _: &f64 = &k.hold.catch.arm_in;
    let _: &[f64; 2] = &k.hold.catch.ready_out;
    let _: &[f64; 4] = &k.hold.catch.jolt.t;
    let _: &f64 = &k.hold.catch.jolt.fwd;
    let _: &f64 = &k.hold.catch.jolt.head;
    let _: &f64 = &k.hold.catch.jolt.clav;
    let _: &f64 = &k.hold.catch.jolt.dip;
    let _: &[f64; 2] = &k.hold.catch.fling.t;
    let _: &[f64; 2] = &k.hold.catch.fling.land;
    let _: &[f64; 3] = &k.hold.catch.fling.drift;
    let _: &[f64; 2] = &k.hold.catch.fling.arm_in;
    let _: &f64 = &k.hold.catch.sink;
    let _: &f64 = &k.hold.catch.hips;
    let _: &[f64; 2] = &k.hold.catch.heels;
    let _: &[f64; 3] = &k.hold.catch.seat;
    let _: &f64 = &k.hold.catch.seat_lean;
    let _: &[f64; 2] = &k.hold.catch.brace;
    let _: &f64 = &k.hold.catch.brace_lift;
    let _: &f64 = &k.hold.catch.yank_sit;
    let _: &f64 = &k.hold.catch.lift;
    let _: &f64 = &k.hold.catch.sam_lift;
    let _: &f64 = &k.hold.catch.settle;
    let _: &[f64; 4] = &k.hold.hit.t;
    let _: &f64 = &k.hold.hit.hips;
    let _: &f64 = &k.hold.hit.dip;
    let _: &f64 = &k.hold.hit.head;
    let _: &f64 = &k.hold.hit.heave;
    let _: &f64 = &k.hold.hit.sam_in;
    let _: &f64 = &k.hold.hit.sam_dip;
    let _: &f64 = &k.hold.first.take;
    let _: &f64 = &k.hold.first.drop;
    let _: &f64 = &k.hold.first.catch;
    let _: &f64 = &k.hold.strip.take;
    let _: &f64 = &k.hold.strip.out;
    let _: &f64 = &k.hold.strip.drop;
    let _: &f64 = &k.hold.strip.follow;
    let _: &f64 = &k.hold.strip.knife;
    let _: &f64 = &k.hold.strip.home;
    let _: &f64 = &k.hold.strip.free;
    let _: &f64 = &k.hold.strip.up;
    let _: &[f64; 3] = &k.hold.strip.meet;
    let _: &[f64; 3] = &k.hold.strip.torn;
    let _: &[f64; 3] = &k.hold.strip.over;
    let _: &[f64; 3] = &k.hold.strip.wrench;
    let _: &[f64; 3] = &k.hold.strip.low;
    let _: &f64 = &k.hold.strip.yaw;
    let _: &[f64; 3] = &k.hold.strip.rap;
    let _: &[f64; 2] = &k.hold.strip.hold;
    let _: &[f64; 3] = &k.hold.strip.pitch;
    let _: &f64 = &k.hold.strip.rest;
    let _: &[f64; 2] = &k.hold.strip.push;
    let _: &f64 = &k.hold.strip.fall;
    let _: &f64 = &k.hold.strip.spin;
    let _: &f64 = &k.hold.dist;
    let _: &f64 = &k.hold.secs.grab;
    let _: &f64 = &k.hold.secs.choke_hold;
    let _: &f64 = &k.hold.secs.choke;
    let _: &f64 = &k.hold.secs.whip;
    let _: &f64 = &k.hold.secs.swap;
    let _: &f64 = &k.hold.secs.kill;
    let _: &[f64; 3] = &k.hold.throat;
    let _: &[f64; 3] = &k.hold.via_l;
    let _: &[f64; 3] = &k.hold.over_l;
    let _: &[f64; 3] = &k.hold.tight;
    let _: &f64 = &k.hold.lean;
    let _: &f64 = &k.hold.choke_lean;
    let _: &f64 = &k.hold.head_pitch;
    let _: &f64 = &k.hold.brace;
    let _: &[f64; 3] = &k.hold.knife.hand;
    let _: &[f64; 3] = &k.hold.knife.blade;
    let _: &[f64; 3] = &k.hold.knife.back;
    let _: &[f64; 3] = &k.hold.knife.via;
    let _: &[f64; 3] = &k.hold.knife.off_hand;
    let _: &[f64; 3] = &k.hold.knife.off_blade;
    let _: &[f64; 3] = &k.hold.knife.off_back;
    let _: &f64 = &k.hold.knife.off_secs;
    let _: &f64 = &k.hold.knife.side;
    let _: &f64 = &k.hold.knife.head_yaw;
    let _: &f64 = &k.hold.knife.clav;
    let _: &[f64; 3] = &k.hold.rest_l;
    let _: &[f64; 3] = &k.hold.claw_l;
    let _: &[f64; 3] = &k.hold.raised_r;
    let _: &[f64; 3] = &k.hold.via_up;
    let _: &[f64; 3] = &k.hold.claw_r;
    let _: &f64 = &k.hold.arch;
    let _: &f64 = &k.hold.chin_up;
    let _: &f64 = &k.hold.aside;
    let _: &f64 = &k.hold.clav_up;
    let _: &f64 = &k.hold.loll;
    let _: &f64 = &k.hold.loll_roll;
    let _: &f64 = &k.hold.lower.from;
    let _: &f64 = &k.hold.lower.by;
    let _: &f64 = &k.hold.lower.up;
    let _: &f64 = &k.hold.lower.sink;
    let _: &f64 = &k.hold.lower.fwd;
    let _: &f64 = &k.hold.lower.hip;
    let _: &f64 = &k.hold.lower.fold;
    let _: &[f64; 2] = &k.hold.lower.hang;
    let _: &f64 = &k.hold.lower.sam_sink;
    let _: &f64 = &k.hold.lower.sam_fwd;
    let _: &f64 = &k.hold.lower.sam_knees;
    let _: &f64 = &k.hold.lower.sam_lean;
    let _: &f64 = &k.hold.lower.sam_look;
    let _: &[f64; 2] = &k.hold.lower.sam_arm;
    let _: &f64 = &k.hold.kill.wind;
    let _: &f64 = &k.hold.kill.hit;
    let _: &f64 = &k.hold.kill.out;
    let _: &f64 = &k.hold.kill.go;
    let _: &f64 = &k.hold.kill.after;
    let _: &[f64; 3] = &k.hold.kill.mouth;
    let _: &[f64; 3] = &k.hold.kill.chest;
    let _: &[f64; 3] = &k.hold.kill.collar;
    let _: &[f64; 3] = &k.hold.kill.set;
    let _: &[f64; 3] = &k.hold.kill.across;
    let _: &[f64; 3] = &k.hold.kill.wide;
    let _: &[f64; 3] = &k.hold.kill.low;
    let _: &[f64; 3] = &k.hold.kill.wide_blade;
    let _: &[f64; 3] = &k.hold.kill.wide_back;
    let _: &[f64; 2] = &k.hold.kill.lean;
    let _: &[f64; 2] = &k.hold.kill.yaw;
    let _: &f64 = &k.hold.kill.sam_sink;
    let _: &f64 = &k.hold.kill.look;
    let _: &f64 = &k.hold.kill.chin;
    let _: &f64 = &k.hold.kill.arch;
    let _: &[f64; 2] = &k.hold.kill.jolt;
    let _: &f64 = &k.hold.kill.sink;
    let _: &f64 = &k.hold.kill.fwd;
    let _: &f64 = &k.hold.kill.hip;
    let _: &f64 = &k.hold.kill.fold;
    let _: &[f64; 2] = &k.hold.kill.hang;
    let _: &[f64; 2] = &k.hold.beats.clamp;
    let _: &[f64; 2] = &k.hold.beats.off;
    let _: &[f64; 2] = &k.hold.beats.tip_by;
    let _: &f64 = &k.hold.beats.tip;
    let _: &f64 = &k.hold.beats.back;
    let _: &f64 = &k.hold.beats.after;
    let _: &f64 = &k.hold.beats.lower.from;
    let _: &f64 = &k.hold.beats.lower.by;
    let _: &f64 = &k.hold.beats.lower.up;
    let _: &f64 = &k.hold.beats.lower.sink;
    let _: &f64 = &k.hold.beats.lower.fwd;
    let _: &f64 = &k.hold.beats.lower.hip;
    let _: &f64 = &k.hold.beats.lower.fold;
    let _: &[f64; 2] = &k.hold.beats.lower.hang;
    let _: &f64 = &k.hold.beats.lower.sam_sink;
    let _: &f64 = &k.hold.beats.lower.sam_fwd;
    let _: &f64 = &k.hold.beats.lower.sam_knees;
    let _: &f64 = &k.hold.beats.lower.sam_lean;
    let _: &f64 = &k.hold.beats.lower.sam_look;
    let _: &[f64; 2] = &k.hold.beats.lower.sam_arm;
    let _: &[f64; 2] = &k.hold.haul.in_;
    let _: &[f64; 2] = &k.hold.haul.out;
    let _: &f64 = &k.hold.haul.down;
    let _: &f64 = &k.hold.haul.sam_lean;
    let _: &[f64; 2] = &k.hold.haul.sam_arm;
    let _: &f64 = &k.hold.haul.lift;
    let _: &f64 = &k.hold.haul.back;
    let _: &f64 = &k.hold.haul.recline;
    let _: &f64 = &k.hold.haul.arch;
    let _: &f64 = &k.hold.haul.chin;
    let _: &[f64; 2] = &k.hold.haul.feet;
    let _: &f64 = &k.hold.haul.toes;
    let _: &[f64; 2] = &k.hold.haul.kick;
    let _: &[f64; 2] = &k.hold.sag;
    let _: &f64 = &k.hold.breath.hz;
    let _: &f64 = &k.hold.breath.pitch;
    let _: &f64 = &k.hold.breath.clav;
    let _: &f64 = &k.hold.struggle.rise;
    let _: &f64 = &k.hold.struggle.fade_from;
    let _: &f64 = &k.hold.struggle.fade_to;
    let _: &f64 = &k.hold.struggle.yaw;
    let _: &f64 = &k.hold.struggle.roll;
    let _: &f64 = &k.hold.struggle.head;
    let _: &f64 = &k.hold.struggle.claw;
    let _: &f64 = &k.hold.struggle.dip;
    let _: &[f64; 3] = &k.hold.struggle.hz;
    let _: &[f64; 3] = &k.hold.gun.hand;
    let _: &[f64; 3] = &k.hold.gun.bore;
    let _: &[f64; 3] = &k.hold.gun.out_hand;
    let _: &f64 = &k.hold.gun.kick_pitch;
    let _: &f64 = &k.hold.gun.kick_back;
    let _: &[f64; 3] = &k.hold.gun.via_a;
    let _: &[f64; 3] = &k.hold.gun.via_b;
    let _: &[f64; 3] = &k.hold.gun.via_bore;
    let _: &[f64; 3] = &k.hold.gun.wind_hand;
    let _: &[f64; 3] = &k.hold.gun.wind_bore;
    let _: &[f64; 3] = &k.hold.gun.hit_hand;
    let _: &[f64; 3] = &k.hold.gun.hit_bore;
    let _: &[f64; 3] = &k.hold.gun.thru_hand;
    let _: &f64 = &k.hold.gun.lean;
    let _: &f64 = &k.hold.gun.side;
    let _: &f64 = &k.hold.gun.head_yaw;
    let _: &f64 = &k.hold.gun.head_pitch;
    let _: &f64 = &k.hold.gun.wind_yaw;
    let _: &f64 = &k.hold.gun.hit_yaw;
    let _: &f64 = &k.hold.gun.clav;
    let _: &f64 = &k.hold.gun.aside;
    let _: &f64 = &k.hold.gun.away_yaw;
    let _: &f64 = &k.hold.gun.chin_up;
    let _: &f64 = &k.hold.gun.duck.yaw;
    let _: &f64 = &k.hold.gun.duck.roll;
    let _: &f64 = &k.hold.gun.duck.pitch;
    let _: &f64 = &k.hold.gun.duck.hunch;
    let _: &f64 = &k.hold.gun.duck.secs;
    let _: &f64 = &k.hold.gun.knock.yaw;
    let _: &f64 = &k.hold.gun.knock.roll;
    let _: &f64 = &k.hold.gun.knock.pitch;
    let _: &f64 = &k.suppression.near_r;
    let _: &f64 = &k.suppression.w_near;
    let _: &f64 = &k.suppression.hit_r;
    let _: &f64 = &k.suppression.w_hit;
    let _: &f64 = &k.suppression.min_travel;
    let _: &f64 = &k.suppression.half_life;
    let _: &f64 = &k.suppression.cap;
    let _: &f64 = &k.suppression.kick_decay;
    let _: &f64 = &k.suppression.down_accrual;
    let _: &f64 = &k.suppression.friendly_k;
    let _: &f64 = &k.suppression.flinch;
    let _: &f64 = &k.suppression.pinned;
    let _: &f64 = &k.suppression.release;
    let _: &[f64; 2] = &k.suppression.head_down;
    let _: &f64 = &k.suppression.flinch_delay;
    let _: &f64 = &k.suppression.head_down_delay;
    let _: &f64 = &k.suppression.guard_spread;
    let _: &f64 = &k.suppression.gap_k;
    let _: &f64 = &k.suppression.standoff_pace_k;
    let _: &f64 = &k.suppression.displace_after;
    let _: &f64 = &k.suppression.displace_min;
    let _: &f64 = &k.suppression.displace_max;
    let _: &f64 = &k.suppression.cover_hold;
    let _: &f64 = &k.suppression.displace_cooldown;
    let _: &f64 = &k.suppression.displace_cap_secs;
    let _: &f64 = &k.suppression.displace_pace;
    let _: &f64 = &k.suppression.displace_crouch;
    let _: &f64 = &k.suppression.src_memory_secs;
    let _: &f64 = &k.suppression.bark_secs;
    let _: &f64 = &k.suppression.floor_bark_secs;
    let _: &f64 = &k.suppression.pair_secs;
    let _: &f64 = &k.suppression.floor_line_secs;
    let _: &f64 = &k.suppression.sam_spread;
    let _: &f64 = &k.rifle_spread.still;
    let _: &f64 = &k.rifle_spread.moving;
    let _: &f64 = &k.rifle_spread.move_at;
    let _: &f64 = &k.rifle_spread.crouch_k;
    let _: &f64 = &k.rifle_spread.brace_k;
    let _: &f64 = &k.footfall.travel;
    let _: &f64 = &k.footfall.owe;
    let _: &f64 = &k.twogears.crouch;
    let _: &f64 = &k.crouchrun.pace;
    let _: &f64 = &k.crouchrun.loud;
    let _: &f64 = &k.runready.fwd;
    let _: &f64 = &k.runready.up;
    let _: &f64 = &k.runready.pitch;
    let _: &f64 = &k.runready.tuck;
    let _: &f64 = &k.runarms.at;
    let _: &f64 = &k.runarms.keep;
    let _: &f64 = &k.runarms.mid;
    let _: &f64 = &k.runarms.swing;
    let _: &f64 = &k.runarms.pump;
    let _: &f64 = &k.runarms.lift;
    let _: &[f64; 3] = &k.runarms.pole;
    let _: &f64 = &k.runarms.pole_w;
    let _: &f64 = &k.samlook.near_m;
    let _: &f64 = &k.samlook.full_m;
    let _: &f64 = &k.samlook.back_deg;
    let _: &f64 = &k.samlook.keep_deg;
    let _: &f64 = &k.samlook.notice_m;
    let _: &f64 = &k.samlook.swap_m;
    let _: &f64 = &k.samlook.slip_from;
    let _: &f64 = &k.samlook.slip_by;
    let _: &[f64; 2] = &k.samlook.back_cos;
    let _: &f64 = &k.dash.stride;
    let _: &f64 = &k.dash.drop;
    let _: &f64 = &k.dash.lean;
    let _: &f64 = &k.dash.rate;
    let _: &f64 = &k.watch.yaw;
    let _: &f64 = &k.watch.down;
    let _: &f64 = &k.watch.up;
    let _: &[f64; 3] = &k.watch.share;
    let _: &f64 = &k.watch.rate;
    let _: &f64 = &k.watch.w_rate;
    let _: &f64 = &k.watch.lean;
    let _: &f64 = &k.watch.drop;
    let _: &f64 = &k.watch.away;
    let _: &f64 = &k.watch.arm_out;
    let _: &f64 = &k.watch.arm_down;
    let _: &f64 = &k.watch.arm_w;
    let _: &[f64; 2] = &k.watch.arm_side;
    let _: &f64 = &k.watch.slip_rate;
    let _: &f64 = &k.watch.squeeze;
    let _: &f64 = &k.stalk.cursor_m;
    let _: &f64 = &k.stalk.begin_cos;
    let _: &f64 = &k.stalk.out_cos;
    let _: &f64 = &k.stalk.station_m;
    let _: &f64 = &k.stalk.grab_m;
    let _: &f64 = &k.stalk.pace;
    let _: &f64 = &k.stalk.close_min;
    let _: &f64 = &k.stalk.close_k;
    let _: &f64 = &k.stalk.ease_up;
    let _: &f64 = &k.stalk.ease_down;
    let _: &f64 = &k.stalk.reach_from;
    let _: &f64 = &k.stalk.reach;
    let _: &f64 = &k.stalk.rise_from;
    let _: &f64 = &k.stalk.rise_by;
    let _: &f64 = &k.stalk.rise;
    let _: &f64 = &k.stalk.catch_secs;
    let _: &f64 = &k.stalk.catch_m;
    let _: &f64 = &k.stalk.catch_stride;
    let _: &f64 = &k.stalk.yank;
    let _: &f64 = &k.stalk.yank_room;
    let _: &f64 = &k.stalk.corner_m;
    let _: &f64 = &k.stalk.corner_deg;
    let _: &f64 = &k.stalk.jolt_face;
    let _: &f64 = &k.stalk.jolt_secs;
    let _: &f64 = &k.stalk.corner_see;
    let _: &[f64; 2] = &k.stalk.corner_reach;
    let _: &f64 = &k.stalk.corner_coil;
    let _: &f64 = &k.stalk.corner_rise;
    let _: &f64 = &k.stalk.key_m;
    let _: &f64 = &k.stalk.low_from;
    let _: &f64 = &k.stalk.low_by;
    let _: &f64 = &k.stalk.low_k;
    let _: &f64 = &k.stalk.low_drop;
    let _: &f64 = &k.stalk.low_back;
    let _: &f64 = &k.stalk.low_lean;
    let _: &f64 = &k.stalk.low_hands;
    let _: &f64 = &k.stalk.low_spring;
    let _: &f64 = &k.stalk.low_wind;
    let _: &f64 = &k.stalk.wide_out;
    let _: &f64 = &k.stalk.wide_down;
    let _: &f64 = &k.stalk.wide_fwd;
    let _: &f64 = &k.stalk.wide_lead;
    let _: &f64 = &k.stalk.wide_by;
    let _: &[f64; 3] = &k.stalk.wide_pole;
    let _: &f64 = &k.stalk.heft_twist;
    let _: &f64 = &k.stalk.heft_swing;
    let _: &f64 = &k.stalk.heft_side;
    let _: &f64 = &k.stalk.heft_lean;
    let _: &f64 = &k.stalk.heft_dip;
    let _: &f64 = &k.stalk.heft_surge;
    let _: &f64 = &k.stalk.heft_out;
    let _: &f64 = &k.stalk.heft_down;
    let _: &f64 = &k.stalk.heft_fwd;
    let _: &f64 = &k.stalk.heft_lead;
    let _: &[f64; 3] = &k.stalk.heft_pole;
    let _: &f64 = &k.stalk.orbit_k;
    let _: &f64 = &k.stalk.orbit_edge;
    let _: &f64 = &k.stalk.orbit_snap;
    let _: &f64 = &k.stalk.low_begin_m;
    let _: &f64 = &k.stalk.low_out_m;
    let _: &f64 = &k.overhear.path_m;
    let _: &f64 = &k.overhear.carry;
    let _: &f64 = &k.score.seen;
    let _: &f64 = &k.score.seen_up;
    let _: &f64 = &k.score.alarm;
    let _: &f64 = &k.score.heightened;
    let _: &f64 = &k.score.lockdown;
    let _: &f64 = &k.score.body;
    let _: &f64 = &k.score.kill;
    let _: &f64 = &k.score.ko;
    let _: &f64 = &k.score.shot_near;
    let _: &f64 = &k.score.shot_near_cap;
    let _: &f64 = &k.score.light;
    let _: &f64 = &k.score.light_cap;
    let _: &f64 = &k.score.hit;
    let _: &f64 = &k.score.down;
    let _: &f64 = &k.score.load;
    let _: &f64 = &k.score.civ_reported;
    let _: &f64 = &k.score.civ_ko;
    let _: &f64 = &k.score.civ_killed;
    let _: &f64 = &k.score.device;
    let _: &f64 = &k.score.device_cap;
    let _: &f64 = &k.score.par_secs;
    let _: &f64 = &k.score.per_min;
    let _: &f64 = &k.score.time_cap;
    let _: &f64 = &k.badge.lights_out_n;
    let _: &f64 = &k.badge.speedrun_secs;
    let _: &f64 = &k.ckpt.quiet_secs;
    let _: &f64 = &k.ckpt.aware_max;
    let _: &f64 = &k.ckpt.debounce_secs;
    let _: &f64 = &k.ckpt.min_gap_secs;
    let _: &f64 = &k.ckpt.pending_secs;
    let _: &f64 = &k.ckpt.body_hide_m;
    let _: &f64 = &k.ckpt.auto_slots;
    let _: &f64 = &k.ckpt.path_keep;
    let _: &f64 = &k.ckpt.flash_secs;
    let _: &f64 = &k.ckpt.regret_secs;
    let _: &f64 = &k.fog.dim;
    let _: &f64 = &k.fog.fov_deg;
    let _: &f64 = &k.fog.near_m;
    let _: &bool = &k.fog.cone;
    let _: &f64 = &k.fog.cone_deg;
    let _: &f64 = &k.fog.cone_near_m;
    let _: &f64 = &k.fog.cone_soft_deg;
    let _: &f64 = &k.fog.cone_near_soft_m;
    let _: &bool = &k.fog.own;
    let _: &f64 = &k.fog.call_secs;
    let _: &f64 = &k.fog.call_gap;
    let _: &f64 = &k.fog.call_known;
    let _: &f64 = &k.fog.smoke;
    let _: &f64 = &k.fog.linger;
    let _: &f64 = &k.fog.top_pad;
    let _: &f64 = &k.fog.squad_doors_m;
    let _: &f64 = &k.fog.squad_fire_secs;
    let _: &f64 = &k.fog.full_fill;
    let _: &f64 = &k.fog.lit_fill;
    let _: &f64 = &k.fog.lit_ao;
    let _: &f64 = &k.fog.lit_ao_m;
    let _: &f64 = &k.fog.lit_cut;
    let _: &f64 = &k.fog.last_seen;
    let _: &f64 = &k.moves.recoil;
    let _: &f64 = &k.moves.toss;
    let _: &f64 = &k.moves.dazzle;
    let _: &f64 = &k.moves.flinch;
    let _: &f64 = &k.moves.gait;
    let _: &f64 = &k.moves.fall;
    let _: &f64 = &k.moves.squeeze;
    let _: &f64 = &k.moves.reload;
    let _: &f64 = &k.moves.bound;
    let _: &f64 = &k.moves.clutch;
    let _: &f64 = &k.moves.entry;
    let _: &f64 = &k.moves.entry_eyes;
    let _: &f64 = &k.moves.fold;
    let _: &f64 = &k.moves.snap;
    let _: &f64 = &k.moves.steady;
    let _: &f64 = &k.moves.steady_run;
    let _: &[f64; 2] = &k.mv.recoil.back;
    let _: &[f64; 2] = &k.mv.recoil.back_tau;
    let _: &[f64; 2] = &k.mv.recoil.climb;
    let _: &[f64; 2] = &k.mv.recoil.climb_tau;
    let _: &[f64; 2] = &k.mv.recoil.yaw;
    let _: &[f64; 2] = &k.mv.recoil.rock;
    let _: &[f64; 2] = &k.mv.recoil.shoulder;
    let _: &[f64; 2] = &k.mv.recoil.rock_tau;
    let _: &[f64; 2] = &k.mv.recoil.head;
    let _: &[f64; 2] = &k.mv.flinch.amp;
    let _: &[f64; 2] = &k.mv.flinch.dur;
    let _: &f64 = &k.mv.flinch.pitch;
    let _: &f64 = &k.mv.flinch.head;
    let _: &f64 = &k.mv.flinch.away;
    let _: &f64 = &k.mv.flinch.drop;
    let _: &f64 = &k.mv.flinch.side;
    let _: &f64 = &k.mv.hit.dur;
    let _: &f64 = &k.mv.hit.pitch;
    let _: &f64 = &k.mv.hit.yaw;
    let _: &f64 = &k.mv.hit.roll;
    let _: &f64 = &k.mv.hit.shove;
    let _: &f64 = &k.mv.hit.drop;
    let _: &[f64; 2] = &k.mv.dazzle.amp;
    let _: &f64 = &k.mv.dazzle.hunch;
    let _: &f64 = &k.mv.dazzle.away;
    let _: &f64 = &k.mv.dazzle.roll;
    let _: &f64 = &k.mv.dazzle.head;
    let _: &f64 = &k.mv.dazzle.head_away;
    let _: &f64 = &k.mv.dazzle.drop;
    let _: &f64 = &k.mv.dazzle.back;
    let _: &f64 = &k.mv.dazzle.sway_x;
    let _: &f64 = &k.mv.dazzle.sway_z;
    let _: &f64 = &k.mv.dazzle.rate_in;
    let _: &f64 = &k.mv.dazzle.rate_out;
    let _: &f64 = &k.mv.dazzle.hand_drop;
    let _: &f64 = &k.mv.dazzle.hand_out;
    let _: &f64 = &k.mv.dazzle.hand_tilt;
    let _: &f64 = &k.mv.dazzle.hand_roll;
    let _: &[f64; 3] = &k.mv.dazzle.hand_pole;
    let _: &f64 = &k.mv.gait.drop;
    let _: &f64 = &k.mv.gait.aim_drop;
    let _: &f64 = &k.mv.gait.bob;
    let _: &f64 = &k.mv.gait.lean;
    let _: &f64 = &k.mv.gait.aim_lean;
    let _: &f64 = &k.mv.gait.elbow_in;
    let _: &f64 = &k.mv.gait.slouch_sway;
    let _: &f64 = &k.mv.gait.slouch_roll;
    let _: &f64 = &k.mv.gait.back;
    let _: &f64 = &k.mv.gait.chin;
    let _: &f64 = &k.mv.gait.wing;
    let _: &f64 = &k.mv.snap.in_;
    let _: &f64 = &k.mv.snap.ease;
    let _: &[f64; 2] = &k.mv.snap.cover;
    let _: &f64 = &k.mv.snap.under;
    let _: &f64 = &k.mv.steady.roll;
    let _: &f64 = &k.mv.steady.sway;
    let _: &f64 = &k.mv.steady.feet;
    let _: &f64 = &k.mv.steady.knee;
    let _: &f64 = &k.mv.steady.knee_low;
    let _: &f64 = &k.mv.steady.knee_out;
    let _: &f64 = &k.mv.steady.rise;
    let _: &f64 = &k.mv.steady.reach;
    let _: &f64 = &k.mv.steady.rise_rate;
    let _: &f64 = &k.turn.body_share;
    let _: &f64 = &k.turn.green;
    let _: &f64 = &k.turn.lead;
    let _: &f64 = &k.turn.lead_rate;
    let _: &f64 = &k.turn.jump;
    let _: &f64 = &k.turn.aim;
    let _: &f64 = &k.turn.aim_acc;
    let _: &f64 = &k.turn.body;
    let _: &f64 = &k.turn.body_acc;
    let _: &f64 = &k.wound.centre;
    let _: &f64 = &k.wound.head;
    let _: &f64 = &k.wound.lead;
    let _: &f64 = &k.wound.depth;
    let _: &f64 = &k.wound.back_room;
    let _: &f64 = &k.doorpath.at;
    let _: &f64 = &k.doorpath.out_m;
    let _: &f64 = &k.doorpath.in_m;
    let _: &f64 = &k.doorpath.look;
    let _: &f64 = &k.doorpath.leaf_clear;
    let _: &f64 = &k.doorpath.tip_m;
    let _: &f64 = &k.doorpath.gap_m;
    let _: &f64 = &k.doorpath.short_m;
    let _: &f64 = &k.doorpath.cap;
    let _: &f64 = &k.doorpath.move_m;
    let _: &f64 = &k.doorpath.apart_m;
    let _: &f64 = &k.doorpath.body_m;
    let _: &f64 = &k.doorpath.body_off;
    let _: &f64 = &k.doorpath.body_move;
    let _: &f64 = &k.walls.wall_max_thick;
    let _: &f64 = &k.walls.wall_min_top;
    let _: &f64 = &k.walls.wall_min_len;
    let _: &f64 = &k.walls.col;
    let _: &f64 = &k.walls.sill;
    let _: &f64 = &k.walls.sill_k;
    let _: &f64 = &k.walls.pistol_k;
    let _: &f64 = &k.walls.cover_min_top;
    let _: &f64 = &k.walls.cover_min_len;
    let _: &f64 = &k.walls.cover_max_cells;
    let _: &f64 = &k.walls.blast.radius;
    let _: &f64 = &k.walls.blast.hole_k;
    let _: &f64 = &k.walls.blast.port_k;
    let _: &f64 = &k.walls.blast.cover_k;
    let _: &f64 = &k.walls.blast.door_k;
    let _: &f64 = &k.walls.blast.min_hole;
    let _: &f64 = &k.walls.blast.contact_m;
    let _: &f64 = &k.walls.max_pieces;
    let _: &f64 = &k.walls.max_changes;
    let _: &f64 = &k.walls.chunks_per_cell;
    let _: &f64 = &k.walls.chunks_per_blast;
    let _: &f64 = &k.walls.fall_max_vol;
    let _: &f64 = &k.walls.falls_per_change;
    let _: &f64 = &k.walls.hung.deep;
    let _: &f64 = &k.walls.hung.long;
    let _: &f64 = &k.walls.hung.tall;
    let _: &f64 = &k.wallcharge.pick_m;
    let _: &f64 = &k.wallcharge.height;
    let _: &f64 = &k.wallcharge.end_m;
    let _: &f64 = &k.wallcharge.set_off;
    let _: &f64 = &k.wallcharge.set_near;
    let _: &f64 = &k.wallcharge.sam_reach;
    let _: &f64 = &k.wallcharge.sam_shift;
    let _: &f64 = &k.wallcharge.sam_marker_m;
    let _: &f64 = &k.wallcharge.post_off;
    let _: &f64 = &k.wallcharge.file_gap;
    let _: &f64 = &k.wallcharge.clear_m;
    let _: &f64 = &k.wallcharge.mate_m;
    let _: &f64 = &k.wallcharge.after_blow;
    let _: &f64 = &k.wallcharge.enter_cap;
    let _: &f64 = &k.wallcharge.sweep_secs;
    let _: &f64 = &k.wallcharge.mouth_in;
    let _: &f64 = &k.wallcharge.lane;
    let _: &f64 = &k.wallcharge.lane_watch;
    let _: &f64 = &k.wallcharge.cost;
    let _: &f64 = &k.wallcharge.hold_secs;
    let _: &f64 = &k.wallcharge.mate_line;
    let _: &f64 = &k.wallcharge.off_frame;
    let _: &f64 = &k.wallcharge.off_frame_pod;
    let _: &f64 = &k.wallcharge.outer_thick;
    let _: &f64 = &k.wallcharge.outer_min_len;
    let _: &f64 = &k.wallcharge.far_slack;
    let _: &f64 = &k.wallcharge.far_reach;
    let _: &f64 = &k.wallcharge.hung_m;
    let _: &[f64; 2] = &k.wallcharge.safe_standoff;
    let _: &[f64; 2] = &k.wallcharge.safe_reach;
    let _: &[f64; 2] = &k.wallcharge.safe_secs;
    let _: &f64 = &k.wallcharge.safe_hide_m;
    let _: &f64 = &k.wallcharge.safe_walk_m;
    let _: &f64 = &k.wallcharge.safe_back_m;
    let _: &f64 = &k.wallcharge.fire_known_s;
    let _: &f64 = &k.wallcharge.fire_m;
    let _: &f64 = &k.wallcharge.fire_step;
    let _: &f64 = &k.wallcharge.fire_grain;
    let _: &f64 = &k.wallcharge.men_clear_in;
    let _: &f64 = &k.wallcharge.men_clear_s;
    let _: &[f64; 2] = &k.wallcharge.sam_secs;
    let _: &f64 = &k.wallcharge.sam_sup;
    let _: &f64 = &k.wallshot.pall;
    let _: &f64 = &k.wallshot.stagger_secs;
    let _: &[f64; 2] = &k.wallshot.stun_secs;
    let _: &f64 = &k.wallshot.room_m;
    let _: &f64 = &k.wallshot.ours_secs;
    let _: &f64 = &k.wallkill.near_m;
    let _: &f64 = &k.hardbreath.chest_amp;
    let _: &f64 = &k.hardbreath.chest_rate;
    let _: &f64 = &k.hardbreath.chest_up;
    let _: &f64 = &k.hardbreath.chest_down;
    let _: &f64 = &k.hardbreath.every;
    let _: &f64 = &k.hardbreath.puffs;
    let _: &f64 = &k.hardbreath.count;
    let _: &f64 = &k.hardbreath.jet;
    let _: &f64 = &k.hardbreath.size;
    let _: &f64 = &k.hardbreath.carry;
    let _: &f64 = &k.hardbreath.up;
    let _: &f64 = &k.hardbreath.down;
    let _: &f64 = &k.castor.decel;
    let _: &f64 = &k.castor.visc;
    let _: &f64 = &k.castor.spin;
    let _: &f64 = &k.castor.start_v;
    let _: &f64 = &k.castor.stop_v;
    let _: &f64 = &k.wallreact.see_m;
    let _: &f64 = &k.wallreact.fov_deg;
    let _: &f64 = &k.wallreact.hear_blast_m;
    let _: &f64 = &k.wallreact.hear_fire_m;
    let _: &f64 = &k.wallreact.hear_fall_m;
    let _: &f64 = &k.wallreact.tell_m;
    let _: &f64 = &k.wallreact.tell_secs;
    let _: &f64 = &k.wallreact.near_m;
    let _: &f64 = &k.wallreact.retry_secs;
    let _: &f64 = &k.wallreact.gone_max;
    let _: &f64 = &k.wallreact.step_m;
    let _: &f64 = &k.wallreact.run_m;
    let _: &f64 = &k.wallreact.beyond_m;
    let _: &f64 = &k.wallreact.ev_open;
    let _: &f64 = &k.wallreact.ev_seen;
    let _: &f64 = &k.wallreact.ev_fire;
    let _: &f64 = &k.wallreact.forget;
    let _: &f64 = &k.wallreact.hole_base;
    let _: &f64 = &k.wallreact.port_base;
    let _: &f64 = &k.wallreact.crowd;
    let _: &f64 = &k.wallreact.sticky;
    let _: &f64 = &k.wallreact.fresh_secs;
    let _: &f64 = &k.wallreact.fresh;
    let _: &f64 = &k.wallreact.watch_m;
    let _: &f64 = &k.coverfire.stale_secs;
    let _: &f64 = &k.coverfire.call_secs;
    let _: &f64 = &k.coverfire.call_m;
    let _: &f64 = &k.coverfire.max_layers;
    let _: &f64 = &k.coverfire.max_rounds;
    let _: &f64 = &k.coverfire.max_m;
    let _: &f64 = &k.coverfire.min_m;
    let _: &f64 = &k.coverfire.near_m;
    let _: &f64 = &k.coverfire.slack_k;
    let _: &f64 = &k.coverfire.slack;
    let _: &f64 = &k.coverfire.give_secs;
    let _: &f64 = &k.coverfire.lane;
    let _: &f64 = &k.coverfire.cone_k;
    let _: &f64 = &k.coverfire.past_m;
    let _: &f64 = &k.coverfire.same_m;
    let _: &f64 = &k.coverfire.linger_secs;
    let _: &f64 = &k.coverfire.lead_secs;
    let _: &f64 = &k.coverfire.lead_m;
    let _: &f64 = &k.coverfire.low_y;
    let _: &f64 = &k.coverfire.round_secs;
    let _: &f64 = &k.coverfire.walk_pace;
    let _: &f64 = &k.coverfire.walk_ask_secs;
    let _: &f64 = &k.coverfire.say_secs;
    let _: &f64 = &k.doorblast.tear_m.stun;
    let _: &f64 = &k.doorblast.tear_m.frag;
    let _: &f64 = &k.doorblast.charge_k;
    let _: &f64 = &k.doorblast.charge_r;
    let _: &[f64; 2] = &k.doorblast.speed;
    let _: &f64 = &k.doorblast.up;
    let _: &f64 = &k.doorblast.tip;
    let _: &f64 = &k.doorblast.spin;
    let _: &f64 = &k.doorblast.scrape;
    let _: &f64 = &k.doorblast.slam;
    let _: &f64 = &k.doorblast.slide;
    let _: &f64 = &k.doorblast.bounce;
    let _: &f64 = &k.doorblast.room;
    let _: &f64 = &k.doorblast.land_level;
    let _: &f64 = &k.doorshot.toll;
    let _: &f64 = &k.doorshot.strips;
    let _: &f64 = &k.doorshot.lengths;
    let _: &f64 = &k.doorshot.uneven;
    let _: &f64 = &k.doorshot.chunks;
    let _: &f64 = &k.doorshot.side;
    let _: &f64 = &k.doorshot.spin;
    let _: &f64 = &k.doorshot.shed;
    let _: &f64 = &k.doorshot.level;
    let _: &f64 = &k.doorthrow.push_secs;
    let _: &f64 = &k.doorthrow.kick_secs;
    let _: &f64 = &k.doorthrow.startle;
    let _: &f64 = &k.doorthrow.wary;
    let _: &f64 = &k.doorthrow.min_speed;
    let _: &f64 = &k.doorthrow.hand_rate;
    let _: &f64 = &k.doorthrow.floor_rad;
    let _: &f64 = &k.doorthrow.rate;
    let _: &f64 = &k.doorthrow.slide;
    let _: &f64 = &k.doorthrow.again;
    let _: &f64 = &k.doorthrow.aware;
    let _: &f64 = &k.winbars.per_round;
    let _: &f64 = &k.winbars.pieces;
    let _: &f64 = &k.winbars.uneven;
    let _: &[f64; 2] = &k.winbars.fly;
    let _: &f64 = &k.winbars.rest_k;
    let _: &[f64; 2] = &k.winbars.up;
    let _: &f64 = &k.winbars.side;
    let _: &f64 = &k.winbars.spin;
    let _: &f64 = &k.winbars.chips;
    let _: &f64 = &k.winbars.chip_v;
    let _: &f64 = &k.winbars.blast_k;
    let _: &f64 = &k.opening.clear_m;
    let _: &f64 = &k.opening.step_m;
    let _: &f64 = &k.handle.look;
    let _: &f64 = &k.handle.ajar;
    let _: &f64 = &k.handle.reach;
    let _: &f64 = &k.handle.handle_in;
    let _: &f64 = &k.handle.far_reach;
    let _: &f64 = &k.handle.push_in;
    let _: &f64 = &k.handle.push_off;
    let _: &f64 = &k.handle.push_thru;
    let _: &f64 = &k.handle.stand_in;
    let _: &f64 = &k.handle.stand_off;
    let _: &f64 = &k.handle.thru_at;
    let _: &f64 = &k.handle.thru_off;
    let _: &f64 = &k.handle.body;
    let _: &f64 = &k.handle.slack;
    let _: &f64 = &k.handle.near;
    let _: &f64 = &k.handle.turn;
    let _: &f64 = &k.handle.clear;
    let _: &f64 = &k.handle.wait;
    let _: &f64 = &k.handle.press;
    let _: &f64 = &k.handle.tool_post;
    let _: &f64 = &k.handle.near_body;
    let _: &f64 = &k.handle.near_post;
    let _: &f64 = &k.handle.go_cap;
    let _: &f64 = &k.handle.rest;
    let _: &f64 = &k.doormanners.reach;
    let _: &f64 = &k.doormanners.half;
    let _: &f64 = &k.doormanners.throat;
    let _: &f64 = &k.doormanners.gap;
    let _: &f64 = &k.doormanners.gap_min;
    let _: &f64 = &k.doormanners.brake;
    let _: &f64 = &k.doormanners.patience;
    let _: &f64 = &k.doormanners.rest;
    let _: &f64 = &k.doormanners.beyond;
    let _: &f64 = &k.doormanners.aside;
    let _: &f64 = &k.doormanners.jamb_out;
    let _: &f64 = &k.doormanners.jamb_past;
    let _: &f64 = &k.doormanners.file_gap;
    let _: &f64 = &k.doormanners.apart;
    let _: &f64 = &k.doormanners.step;
    let _: &f64 = &k.doormanners.round;
    let _: &f64 = &k.doormanners.round_out;
    let _: &f64 = &k.doormanners.own_m;
    let _: &f64 = &k.doormanners.own_end_m;
    let _: &f64 = &k.doormanners.leaf_stall_m;
    let _: &f64 = &k.doormanners.leaf_stall_secs;
    let _: &f64 = &k.doormanners.leaf_stall_reach;
    let _: &f64 = &k.doormanners.leaf_stall_ahead;
    let _: &f64 = &k.doormanners.leaf_stall_push;
    let _: &f64 = &k.doormanners.sweep;
    let _: &f64 = &k.doormanners.lane_ajar;
    let _: &f64 = &k.doormanners.lane_m;
    let _: &f64 = &k.doormanners.lane_out;
    let _: &f64 = &k.doormanners.lane_flare;
    let _: &f64 = &k.doormanners.lane_clear;
    let _: &f64 = &k.doormanners.lane_apart;
    let _: &f64 = &k.doormanners.cover_m;
    let _: &f64 = &k.doormanners.cover_deep;
    let _: &f64 = &k.doormanners.cover_look;
    let _: &f64 = &k.doormanners.cover_quiet;
    let _: &f64 = &k.doormanners.cover_cap;
    let _: &f64 = &k.doormanners.lane_dear;
    let _: &f64 = &k.doormanners.lane_strip;
    let _: &f64 = &k.doormanners.lane_long;
    let _: &f64 = &k.doormanners.funnel;
    let _: &f64 = &k.doormanners.funnel_rest;
    let _: &f64 = &k.doormanners.through;
    let _: &f64 = &k.doormanners.past;
    let _: &f64 = &k.doormanners.plane_deg;
    let _: &f64 = &k.doormanners.rest_secs;
    let _: &f64 = &k.doormanners.clear_m;
    let _: &f64 = &k.doormanners.held_cap;
    let _: &f64 = &k.breaks.share.big.thrown;
    let _: &f64 = &k.breaks.share.big.heap;
    let _: &f64 = &k.breaks.share.big.furniture;
    let _: &[f64; 2] = &k.breaks.refuse_chips;
    let _: &[f64; 2] = &k.breaks.crumble_chips;
    let _: &[f64; 2] = &k.breaks.sized_chips;
    let _: &[f64; 2] = &k.breaks.sized_span;
    let _: &f64 = &k.breaks.china_land;
    let _: &f64 = &k.breaks.china_max;
    let _: &f64 = &k.breaks.core_tint;
    let _: &f64 = &k.breaks.chip_tint;
    let _: &[f64; 2] = &k.breaks.heap.span;
    let _: &[f64; 2] = &k.breaks.heap.out;
    let _: &[f64; 2] = &k.breaks.heap.tip;
    let _: &f64 = &k.breaks.max_standing_planes;
    let _: &[f64; 2] = &k.breaks.across_deg;
    let _: &f64 = &k.breaks.loose_min;
    let _: &[f64; 2] = &k.breaks.kink.deg;
    let _: &f64 = &k.breaks.kink.skew;
    let _: &f64 = &k.breaks.kink.off;
    let _: &f64 = &k.breaks.kink.min_long;
    let _: &f64 = &k.breaks.kink.gap_secs;
    let _: &f64 = &k.breaks.head.from;
    let _: &[f64; 2] = &k.breaks.head.deep;
    let _: &f64 = &k.breaks.head.planes;
    let _: &[f64; 2] = &k.breaks.head.across;
    let _: &f64 = &k.breaks.head.min_room;
    let _: &f64 = &k.breaks.head.most_of_width;
    let _: &f64 = &k.breaks.steel.sheet;
    let _: &[f64; 2] = &k.breaks.steel.kink;
    let _: &[f64; 2] = &k.breaks.steel.dents;
    let _: &[f64; 2] = &k.breaks.steel.dent;
    let _: &f64 = &k.breaks.steel.dent_most;
    let _: &f64 = &k.breaks.steel.rim;
    let _: &f64 = &k.breaks.steel.lip;
    let _: &f64 = &k.breaks.steel.inside;
    let _: &f64 = &k.breaks.stud.half;
    let _: &f64 = &k.breaks.stud.depth_k;
    let _: &f64 = &k.breaks.stud.foot;
    let _: &[f64; 3] = &k.breaks.stud.albedo;
    let _: &f64 = &k.breaks.stud.rough;
    let _: &f64 = &k.furn.shatter_m;
    let _: &f64 = &k.furn.apart_m;
    let _: &f64 = &k.furn.throw_m;
    let _: &f64 = &k.furn.throw_v;
    let _: &f64 = &k.furn.prop_break_m;
    let _: &f64 = &k.furn.chunks_per_blast;
    let _: &f64 = &k.furn.mouth_m;
    let _: &f64 = &k.furn.merge_m;
    let _: &f64 = &k.furn.mouth.depth;
    let _: &f64 = &k.furn.mouth.top;
    let _: &f64 = &k.furn.mouth.vol;
    let _: &f64 = &k.furn.mouth.chunks;
    let _: &f64 = &k.furn.mouth.base_thick;
    let _: &f64 = &k.furn.hung.len;
    let _: &f64 = &k.furn.hung.max;
    let _: &f64 = &k.furn.hung.strips;
    let _: &f64 = &k.furn.hung.under_m;
    let _: &f64 = &k.furn.hung.side;
    let _: &f64 = &k.furn.hung.spin;
    let _: &f64 = &k.furn.hung.chunks;
    let _: &f64 = &k.furn.hung.hole_m;
    let _: &f64 = &k.furn.root.min_top;
    let _: &f64 = &k.furn.root.max_top;
    let _: &f64 = &k.furn.root.max_long;
    let _: &f64 = &k.furn.root.max_vol;
    let _: &f64 = &k.furn.root.apart_max_vol;
    let _: &f64 = &k.furn.rider.max_top;
    let _: &f64 = &k.furn.rider.max_vol;
    let _: &f64 = &k.furn.touch;
    let _: &f64 = &k.furn.row_m;
    let _: &f64 = &k.snowbog.dead;
    let _: &f64 = &k.snowbog.grip;
    let _: &f64 = &k.snowbog.stick;
    let _: &f64 = &k.snowbog.packed;
    let _: &f64 = &k.snowbog.old;
    let _: &f64 = &k.snowbog.top;
    let _: &f64 = &k.snowbog.bank_m;
    let _: &f64 = &k.snowbog.run;
    let _: &f64 = &k.rubble.radius;
    let _: &f64 = &k.rubble.reach;
    let _: &f64 = &k.rubble.lead;
    let _: &f64 = &k.rubble.inside;
    let _: &f64 = &k.rubble.floor;
    let _: &f64 = &k.rubble.heavy;
    let _: &f64 = &k.rubble.walk;
    let _: &f64 = &k.rubble.base;
    let _: &f64 = &k.rubble.gain;
    let _: &f64 = &k.rubble.aside;
    let _: &f64 = &k.rubble.v_max;
    let _: &f64 = &k.rubble.off;
    let _: &f64 = &k.rubble.slide;
    let _: &f64 = &k.rubble.spin;
    let _: &f64 = &k.rubble.per_frame;
    let _: &f64 = &k.rubble.cooldown;
    let _: &f64 = &k.rubble.give_up;
    let _: &f64 = &k.rubble.give_up_secs;
    let _: &f64 = &k.rubble.scuff;
    let _: &f64 = &k.debris.min_speed;
    let _: &f64 = &k.lightwall.gap;
    let _: &f64 = &k.airdust.diffuse;
    let _: &f64 = &k.airdust.settle;
    let _: &f64 = &k.airdust.step;
    let _: &f64 = &k.airdust.recheck_secs;
    let _: &bool = &k.wallsnap.on;
    let _: &f64 = &k.wallsnap.take_base;
    let _: &f64 = &k.wallsnap.take_per_m;
    let _: &f64 = &k.wallsnap.take_per_turn;
    let _: &f64 = &k.wallsnap.take_min;
    let _: &f64 = &k.wallsnap.take_max;
    let _: &f64 = &k.wallsnap.ease_pow;
    let _: &f64 = &k.wallsnap.yaw_pow;
    let _: &f64 = &k.wallsnap.reach_moving;
    let _: &f64 = &k.wallsnap.heading_min;
    let _: &f64 = &k.wallsnap.heading_bonus;
    let _: &f64 = &k.wallsnap.buffer;
    let _: &f64 = &k.wallsnap.dip;
    let _: &f64 = &k.wallsnap.nod;
    let _: &f64 = &k.wallsnap.hit_peak;
    let _: &f64 = &k.wallsnap.scuff;
    let _: &f64 = &k.wallsnap.step_lift;
    let _: &f64 = &k.wallsnap.step_trail;
    let _: &f64 = &k.wallsnap.cam_room;
    let _: &f64 = &k.wallsnap.cam_side;
    let _: &f64 = &k.wallsnap.along_deg;
    let _: &f64 = &k.wallsnap.away_secs;
    let _: &f64 = &k.wallsnap.away_diag;
    let _: &f64 = &k.wallsnap.off_push;
    let _: &f64 = &k.wallsnap.out_rate;
    let _: &f64 = &k.wallsnap.up_secs;
    let _: &f64 = &k.wallsnap.up_rise;
    let _: &f64 = &k.wallsnap.bore;
    let _: &f64 = &k.wallsnap.lean_along;
    let _: &f64 = &k.wallsnap.lean_out;
    let _: &f64 = &k.wallsnap.lean_back;
    let _: &f64 = &k.wallsnap.lean_on_deg;
    let _: &f64 = &k.wallsnap.lean_off_deg;
    let _: &f64 = &k.wallsnap.lean_min_dist;
    let _: &f64 = &k.wallsnap.lean_max_deg;
    let _: &f64 = &k.wallsnap.lean_keep_deg;
    let _: &f64 = &k.wallsnap.lean_keep_dist;
    let _: &f64 = &k.wallsnap.lean_over_y;
    let _: &f64 = &k.wallsnap.lean_clear_m;
    let _: &f64 = &k.wallsnap.lean_gun;
    let _: &f64 = &k.wallsnap.lean_gun_step;
    let _: &f64 = &k.wallsnap.dwell_out;
    let _: &f64 = &k.wallsnap.dwell_back;
    let _: &f64 = &k.wallsnap.min_out;
    let _: &f64 = &k.wallsnap.after_shot;
    let _: &bool = &k.wallsnap.press;
    let _: &f64 = &k.wallsnap.press_secs;
    let _: &f64 = &k.wallsnap.press_flow;
    let _: &f64 = &k.wallsnap.press_near;
    let _: &f64 = &k.wallsnap.press_deg;
    let _: &f64 = &k.wallsnap.press_touch;
    let _: &f64 = &k.wallsnap.press_thing;
    let _: &f64 = &k.wallsnap.press_thing_at;
    let _: &f64 = &k.wallsnap.press_turn;
    let _: &f64 = &k.wallsnap.press_again;
    let _: &f64 = &k.wallsnap.press_after_shot;
    let _: &f64 = &k.wallsnap.voice_on;
    let _: &f64 = &k.wallsnap.voice_off;
    let _: &f64 = &k.wallsnap.slow_take;
    let _: &f64 = &k.wallsnap.slow_in;
    let _: &f64 = &k.wallsnap.slow_out;
    let _: &f64 = &k.gunwall.down_m;
    let _: &f64 = &k.gunwall.up_m;
    let _: &f64 = &k.gunwall.lead_s;
    let _: &f64 = &k.gunwall.at_m;
    let _: &f64 = &k.gunwall.tuck_m;
    let _: &f64 = &k.gunwall.tuck_lead_s;
    let _: &f64 = &k.gunwall.fight_s;
    let _: &f64 = &k.close.rmb_tap_secs;
    let _: &f64 = &k.close.rmb_loud;
    let _: &f64 = &k.close.rmb_wait_secs;
    let _: &f64 = &k.close.hands_low_neck_m;
    let _: &f64 = &k.close.mode_m;
    let _: &f64 = &k.close.mode_out_m;
    let _: &f64 = &k.close.dark_at;
    let _: &f64 = &k.close.lit_at;
    let _: &f64 = &k.close.touch_m;
    let _: &f64 = &k.close.fade_m;
    let _: &f64 = &k.close.reach.behind;
    let _: &f64 = &k.close.reach.side;
    let _: &f64 = &k.close.reach.front;
    let _: &f64 = &k.close.reach.seat;
    let _: &f64 = &k.close.reach.seat_front;
    let _: &f64 = &k.close.side_from_deg;
    let _: &f64 = &k.close.inner;
    let _: &f64 = &k.close.step_deg;
    let _: &f64 = &k.close.strike_dist;
    let _: &f64 = &k.close.seat_dist;
    let _: &f64 = &k.close.noise.behind;
    let _: &f64 = &k.close.noise.side;
    let _: &f64 = &k.close.noise.front;
    let _: &f64 = &k.close.noise.seat;
    let _: &f64 = &k.close.alert_noise;
    let _: &f64 = &k.plan.fresh_secs;
    let _: &f64 = &k.plan.walk_dim;
    let _: &f64 = &k.plan.rest_dim;
    let _: &f64 = &k.plan.ring_m;
    let _: &f64 = &k.plan.sector_m;
    let _: &f64 = &k.plan.sector_deg;
    let _: &f64 = &k.plan.sight_m;
    let _: &f64 = &k.plan.reask_secs;
    let _: &f64 = &k.plan.pair_bar_m;
    let _: &f64 = &k.plan.others;
    let _: &f64 = &k.reload.delay;
    let _: &f64 = &k.samstun.fade;
    let _: &f64 = &k.recap.secs;
    let _: &f64 = &k.recap.after;
    let _: &f64 = &k.recap.hz;
    let _: &f64 = &k.recap.keep;
    let _: &f64 = &k.recap.live_see_m;
    let _: &f64 = &k.recap.live_saw_secs;
    let _: &f64 = &k.recap.live_hear_m;
    let _: &f64 = &k.recap.live_beat;
    let _: &f64 = &k.recap.live_side_m;
    let _: &f64 = &k.recap.live_mark_secs;
    let _: &f64 = &k.aimweight.rate;
    let _: &f64 = &k.aimweight.smooth;
    let _: &f64 = &k.aimweight.heft;
    let _: &f64 = &k.aimweight.bare;
    let _: &f64 = &k.aimweight.on_m;
    let _: &f64 = &k.aimweight.reach;
    let _: &f64 = &k.aimweight.launch_wait;
    let _: &f64 = &k.aimcost.bare;
    let _: &f64 = &k.aimcost.pad;
    let _: &f64 = &k.aimcost.slack;
    let _: &f64 = &k.aimcost.move_k;
    let _: &f64 = &k.aimcost.stop_tau;
    let _: &f64 = &k.aimcost.stop_bleed;
    let _: &f64 = &k.aimcost.brace_feet;
    let _: &f64 = &k.aimcost.bare_device;
    let _: &f64 = &k.aimcost.plant_at;
    let _: &f64 = &k.aimcost.flank_at;
    let _: &f64 = &k.aimcost.ring_px;
    let _: &f64 = &k.friends.reach_m;
    let _: &f64 = &k.friends.past_m;
    let _: &f64 = &k.friends.clear_m;
    let _: &f64 = &k.friends.lead_s;
    let _: &f64 = &k.friends.lead_max_m;
    let _: &f64 = &k.friends.up_secs;
    let _: &f64 = &k.friends.quiet_s;
    let _: &f64 = &k.friendlyfire.cone_k;
    let _: &f64 = &k.friendlyfire.body_m;
    let _: &f64 = &k.friendlyfire.sure_m;
    let _: &f64 = &k.friendlyfire.near_m;
    let _: &f64 = &k.lightswitch.reach;
    let _: &f64 = &k.lightswitch.height;
    let _: &f64 = &k.lightswitch.stand_m;
    let _: &f64 = &k.lightswitch.per_room;
    let _: &f64 = &k.lightswitch.apart_m;
    let _: &f64 = &k.lightswitch.scan_m;
    let _: &f64 = &k.lightswitch.guard_secs;
    let _: &f64 = &k.lightswitch.hand_secs;
    let _: &[f64; 3] = &k.carlamps.dark;
    let _: &f64 = &k.carlamps.fit;
    let _: &f64 = &k.carlamps.tone;
    let _: &f64 = &k.carlamps.twin;
    let _: &f64 = &k.reach_pace.steps;
    let _: &bool = &k.reach_pace.whole;
    let _: &f64 = &k.reach_pace.whole_cells;
    let _: &bool = &k.coatfinish.on;
    let _: &bool = &k.solidglass.on;
    let _: &f64 = &k.solidglass.lum;
    let _: &f64 = &k.solidglass.rough;
    let _: &bool = &k.wallskin.on;
    let _: &f64 = &k.wallskin.thick;
    let _: &f64 = &k.wallskin.off;
    let _: &f64 = &k.wallskin.per_skin;
    let _: &f64 = &k.wallskin.max_pieces;
    let _: &f64 = &k.wallskin.box_slack;
    let _: &f64 = &k.fireline.fire_s;
    let _: &f64 = &k.fireline.reload_s;
    let _: &f64 = &k.fireline.pace_s;
    let _: &f64 = &k.fireline.pace_k;
    let _: &f64 = &k.fireline.lapse_s;
    let _: &f64 = &k.fireline.spread;
    let _: &f64 = &k.fireline.clear_m;
    let _: &f64 = &k.fireline.short_m;
    let _: &f64 = &k.fireline.keep_m;
    let _: &f64 = &k.fireline.gap_m;
    let _: &f64 = &k.fireline.look_m;
    let _: &f64 = &k.fireline.cue_s;
    let _: &f64 = &k.fireline.max_s;
    let _: &f64 = &k.fireline.again_s;
    let _: &f64 = &k.fireline.way_more_m;
    let _: &f64 = &k.fireline.berth_m;
    let _: &f64 = &k.fireline.edge_m;
    let _: &f64 = &k.fireline.reach_m;
    let _: &f64 = &k.savetimeline.manual;
    let _: &f64 = &k.savetimeline.auto;
    let _: &f64 = &k.savetimeline.pic_px;
    let _: &f64 = &k.savetimeline.pic_q;
    let _: &f64 = &k.handring.hold_secs;
    let _: &f64 = &k.handring.turn_max;
    let _: &f64 = &k.sprintdoor.keep;
    let _: &f64 = &k.sprintdoor.bash_vel;
    let _: &f64 = &k.sprintdoor.lean;
    let _: &f64 = &k.sprintdoor.lean_secs;
    let _: &f64 = &k.sprintdoor.rip_pace;
    let _: &f64 = &k.sprintdoor.rip_secs;
    let _: &f64 = &k.sprintdoor.rip_vel;
    let _: &f64 = &k.sprintdoor.rip_level;
    let _: &f64 = &k.sprintdoor.kick_k;
    let _: &f64 = &k.sprintdoor.kick_into;
    let _: &f64 = &k.sprintdoor.kick_free;
    let _: &f64 = &k.teampace.k;
    let _: &f64 = &k.teampace.follow_k;
    let _: &f64 = &k.teampace.behind_m;
    let _: &f64 = &k.teampace.fight_secs;
    let _: &f64 = &k.teampace.back_secs;
    let _: &f64 = &k.teampace.dash_m;
    let _: &f64 = &k.teampace.dash_seen;
    let _: &f64 = &k.teampace.dash_every;
    let _: &f64 = &k.teampace.dash_hot;
    let _: &bool = &k.teampace.door_brisk;
    let _: &bool = &k.teampace.pacing;
    let _: &f64 = &k.teampace.pace_near;
    let _: &f64 = &k.teampace.pace_gap;
    let _: &f64 = &k.teampace.pace_cos;
    let _: &f64 = &k.teampace.walk_aim_off;
    let _: &f64 = &k.teampace.walk_aim_on;
    let _: &f64 = &k.teampace.run_at;
    let _: &f64 = &k.teampace.accel;
    let _: &f64 = &k.teampace.step;
    let _: &f64 = &k.teampace.gap;
    let _: &f64 = &k.noshove.ring;
    let _: &f64 = &k.noshove.back;
    let _: &f64 = &k.rails.pan_speed;
    let _: &f64 = &k.rails.pan_reach;
    let _: &f64 = &k.rails.snap_px;
    let _: &f64 = &k.rails.keep;
    let _: &f64 = &k.rails.keep_speed;
    let _: &f64 = &k.rails.keep_deep;
    let _: &f64 = &k.rails.keep_up;
    let _: &f64 = &k.rails.way_m;
    let _: &f64 = &k.rails.after_secs;
    let _: &f64 = &k.rails.after_max;
    let _: &f64 = &k.rails.card_beat_secs;
    let _: &f64 = &k.rails.card_quiet;
    let _: &f64 = &k.rails.card_max;
    let _: &f64 = &k.rails.spare_secs;
    let _: &f64 = &k.doorforce.sure;
    let _: &f64 = &k.doorforce.keep;
    let _: &f64 = &k.doorforce.fade_secs;
    let _: &f64 = &k.doorforce.stale_secs;
    let _: &f64 = &k.doorforce.gain;
    let _: &f64 = &k.doorforce.hear_min;
    let _: &f64 = &k.doorforce.shot;
    let _: &f64 = &k.doorforce.handle;
    let _: &f64 = &k.doorforce.seen;
    let _: &f64 = &k.doorforce.shut_carry;
    let _: &f64 = &k.doorforce.behind_m;
    let _: &f64 = &k.doorforce.reach_m;
    let _: &f64 = &k.doorforce.seen_secs;
    let _: &f64 = &k.doorforce.heard_err_m;
    let _: &f64 = &k.doorforce.chest_y;
    let _: &f64 = &k.doorforce.near_leaf_m;
    let _: &f64 = &k.doorforce.past_leaf_m;
    let _: &f64 = &k.doorforce.rounds;
    let _: &f64 = &k.doorforce.gap_secs;
    let _: &f64 = &k.doorforce.kick_secs;
    let _: &f64 = &k.doorforce.kick_secs_no_shot;
    let _: &f64 = &k.doorforce.walk_secs;
    let _: &f64 = &k.doorforce.rest_secs;
    let _: &f64 = &k.slotsetup.secs;
    let _: &f64 = &k.slotsetup.settle;
    let _: &f64 = &k.glassreal.hang;
    let _: &f64 = &k.glassreal.blocks;
    let _: &f64 = &k.glassreal.low_m;
    let _: &f64 = &k.glassreal.door_m;
    let _: &f64 = &k.glassreal.door_foot_m;
    let _: &f64 = &k.told.guided;
    let _: &f64 = &k.told.standard;
    let _: &f64 = &k.kitanim.sling;
    let _: &f64 = &k.kitanim.sling_up;
    let _: &f64 = &k.kitanim.sling_down;
    let _: &f64 = &k.kitanim.early;
    let _: &f64 = &k.kitanim.click_keep;
    let _: &f64 = &k.kitanim.throw_keep;
    let _: &f64 = &k.kitanim.plain_lock;
    let _: &f64 = &k.kitanim.beats.pistol.ref_;
    let _: &f64 = &k.kitanim.beats.pistol.empty.dur;
    let _: &f64 = &k.kitanim.beats.pistol.empty.out;
    let _: &f64 = &k.kitanim.beats.pistol.empty.in_;
    let _: &f64 = &k.kitanim.beats.pistol.dry.dur;
    let _: &f64 = &k.kitanim.beats.pistol.dry.out;
    let _: &f64 = &k.kitanim.beats.pistol.dry.in_;
    let _: &f64 = &k.kitanim.beats.pistol.tactical.dur;
    let _: &f64 = &k.kitanim.beats.pistol.tactical.out;
    let _: &f64 = &k.kitanim.beats.pistol.tactical.in_;
    let _: &f64 = &k.kitanim.beats.rifle.ref_;
    let _: &f64 = &k.kitanim.beats.rifle.empty.dur;
    let _: &f64 = &k.kitanim.beats.rifle.empty.out;
    let _: &f64 = &k.kitanim.beats.rifle.empty.in_;
    let _: &f64 = &k.kitanim.beats.rifle.dry.dur;
    let _: &f64 = &k.kitanim.beats.rifle.dry.out;
    let _: &f64 = &k.kitanim.beats.rifle.dry.in_;
    let _: &f64 = &k.kitanim.beats.rifle.tactical.dur;
    let _: &f64 = &k.kitanim.beats.rifle.tactical.out;
    let _: &f64 = &k.kitanim.beats.rifle.tactical.in_;
    let _: &f64 = &k.kitanim.close_near;
    let _: &f64 = &k.kit.away;
    let _: &f64 = &k.kit.swap_k;
    let _: &f64 = &k.kit.far;
    let _: &f64 = &k.kit.apex_at;
    let _: &f64 = &k.kit.sling_at;
    let _: &f64 = &k.kit.join;
    let _: &f64 = &k.kit.let_go;
    let _: &[f64; 3] = &k.kit.apex.at;
    let _: &[f64; 3] = &k.kit.apex.fing;
    let _: &[f64; 3] = &k.kit.apex.back;
    let _: &[f64; 3] = &k.kit.apex.pole;
    let _: &f64 = &k.kit.elbow_min;
    let _: &f64 = &k.kit.wrist_knee;
    let _: &f64 = &k.kit.wrist_max;
    let _: &f64 = &k.kit.roll_in;
    let _: &[f64; 3] = &k.kit.over;
    let _: &[f64; 3] = &k.kit.up;
    let _: &[f64; 3] = &k.kit.top.fing;
    let _: &[f64; 3] = &k.kit.top.back;
    let _: &f64 = &k.kit.trunk;
    let _: &[f64; 3] = &k.kit.belt.at;
    let _: &[f64; 3] = &k.kit.belt.fing;
    let _: &[f64; 3] = &k.kit.belt.back;
    let _: &[f64; 3] = &k.kit.belt.pole;
    let _: &[f64; 3] = &k.kit.holster_pole;
    let _: &f64 = &k.kit.sheath_pole;
    let _: &[f64; 3] = &k.kit.rest;
    let _: &[f64; 3] = &k.kit.cross;
    let _: &f64 = &k.kit.front.stow;
    let _: &f64 = &k.kit.front.reach;
    let _: &f64 = &k.kit.front.draw;
    let _: &f64 = &k.kit.front.hand_at;
    let _: &f64 = &k.kit.front.sling_at;
    let _: &[f64; 3] = &k.kit.front.arc;
    let _: &[f64; 3] = &k.kit.front.pole;
    let _: &f64 = &k.kit.nv.hand.dur;
    let _: &f64 = &k.kit.nv.hand.reach;
    let _: &f64 = &k.kit.nv.hand.out;
    let _: &[f64; 2] = &k.kit.nv.hand.flip;
    let _: &[f64; 3] = &k.kit.nv.hand.up;
    let _: &[f64; 3] = &k.kit.nv.hand.down;
    let _: &[f64; 3] = &k.kit.nv.hand.fingers;
    let _: &[f64; 3] = &k.kit.nv.hand.back;
    let _: &[f64; 3] = &k.kit.nv.hand.pole;
    let _: &f64 = &k.kit.nv.hand.shrug;
    let _: &[f64; 4] = &k.kit.nv.hand.curl;
    let _: &f64 = &k.kit.nv.hand.thumb;
    let _: &f64 = &k.kit.nv.hand.rate;
    let _: &f64 = &k.kit.nv.hand.away;
    let _: &f64 = &k.kit.nv.hand.clear;
    let _: &f64 = &k.kit.nv.nod.dur;
    let _: &f64 = &k.kit.nv.nod.pre;
    let _: &f64 = &k.kit.nv.nod.peak;
    let _: &[f64; 2] = &k.kit.nv.nod.flip;
    let _: &[f64; 2] = &k.kit.nv.nod.on;
    let _: &[f64; 2] = &k.kit.nv.nod.off;
    let _: &f64 = &k.kit.nv.nod.carry;
    let _: &[f64; 3] = &k.kit.mag.well;
    let _: &[f64; 3] = &k.kit.mag.strip;
    let _: &[f64; 3] = &k.kit.mag.slide;
    let _: &[f64; 3] = &k.kit.mag.pull;
    let _: &[f64; 3] = &k.kit.mag.pouch;
    let _: &[f64; 3] = &k.kit.mag.dump;
    let _: &[f64; 3] = &k.kit.mag.via;
    let _: &[f64; 3] = &k.kit.mag.pole;
    let _: &f64 = &k.kit.mag.pole_w;
    let _: &[f64; 3] = &k.kit.mag.wf;
    let _: &[f64; 3] = &k.kit.mag.wb;
    let _: &[f64; 3] = &k.kit.mag.sf;
    let _: &[f64; 3] = &k.kit.mag.sb;
    let _: &f64 = &k.kit.close.far;
    let _: &f64 = &k.kit.close.hover;
    let _: &[f64; 3] = &k.kit.close.bow;
    let _: &[f64; 3] = &k.kit.close.lead;
    let _: &f64 = &k.kit.close.left;
    let _: &f64 = &k.kit.close.left_at;
    let _: &f64 = &k.kit.close.left_in;
    let _: &f64 = &k.kit.close.left_out;
    let _: &f64 = &k.kit.close.rate;
    let _: &f64 = &k.kit.close.flex;
    let _: &f64 = &k.kit.close.flex_amp;
    let _: &f64 = &k.kit.close.drift;
    let _: &f64 = &k.kit.close.regrip;
    let _: &f64 = &k.kit.close.curl;
    let _: &f64 = &k.throw_by.toss;
    let _: &f64 = &k.throw_by.toss_fast;
    let _: &f64 = &k.throw_by.toss_in;
    let _: &f64 = &k.throw_by.toss_min;
    let _: &f64 = &k.throw_by.through;
    let _: &[f64; 3] = &k.throw_by.toss_pivot;
    let _: &f64 = &k.throw_by.full;
    let _: &f64 = &k.throw_by.turn_min;
    let _: &f64 = &k.throw_by.arm_min;
    let _: &f64 = &k.throw_by.off_min;
    let _: &f64 = &k.throw_by.sit;
    let _: &[f64; 3] = &k.throw_by.pivot;
    let _: &f64 = &k.hands.reach_secs;
    let _: &f64 = &k.hands.lift_secs;
    let _: &f64 = &k.hands.set_secs;
    let _: &f64 = &k.hands.grip_m;
    let _: &f64 = &k.hands.step_secs;
    let _: &f64 = &k.hands.head_bias;
    let _: &f64 = &k.hands.ahead;
    let _: &f64 = &k.hands.ahead_head;
    let _: &f64 = &k.hands.carry_feet;
    let _: &f64 = &k.hands.carry_head;
    let _: &f64 = &k.hands.slack_m;
    let _: &f64 = &k.hands.hoist_secs;
    let _: &f64 = &k.hands.lower_secs;
    let _: &f64 = &k.hands.carry_step_secs;
    let _: &f64 = &k.hands.carry_near_m;
    let _: &f64 = &k.hands.carry_side;
    let _: &f64 = &k.hands.carry_fwd;
    let _: &f64 = &k.hands.carry_up;
    let _: &f64 = &k.hands.carry_back;
    let _: &f64 = &k.hands.carry_legs;
    let _: &f64 = &k.hands.carry_pace;
    let _: &f64 = &k.hands.carry_turn;
    let _: &f64 = &k.hands.carry_ride;
    let _: &f64 = &k.hands.lay_ahead;
    let _: &f64 = &k.hands.door_reach;
    let _: &f64 = &k.hands.hand_secs;
    let _: &f64 = &k.hands.door_rate;
    let _: &f64 = &k.hands.open_to;
    let _: &f64 = &k.hands.clear_m;
    let _: &f64 = &k.hands.step_m;
    let _: &f64 = &k.hands.round_m;
    let _: &f64 = &k.hands.hand_m;
    let _: &f64 = &k.hands.in_m;
    let _: &f64 = &k.hands.stall_secs;
    let _: &f64 = &k.hands.aim_arc;
    let _: &f64 = &k.hands.team_pre_m;
    let _: &f64 = &k.hands.team_reach_m;
    let _: &f64 = &k.hands.team_pace;
    let _: &f64 = &k.hands.team_vel;
    let _: &f64 = &k.work.pick_m;
    let _: &f64 = &k.work.pick_wide_m;
    let _: &f64 = &k.work.behind_m;
    let _: &f64 = &k.work.behind_panel_m;
    let _: &f64 = &k.work.stand_m;
    let _: &f64 = &k.work.buddy_m;
    let _: &f64 = &k.work.hand_secs;
    let _: &f64 = &k.work.breaker_secs;
    let _: &f64 = &k.work.fade_secs;
    let _: &f64 = &k.work.hand_m;
    let _: &f64 = &k.work.walk_m;
    let _: &f64 = &k.wand.feed_secs;
    let _: &f64 = &k.wand.stow_secs;
    let _: &f64 = &k.wand.fov_deg;
    let _: &f64 = &k.wand.blind_deg;
    let _: &f64 = &k.wand.pan_rate;
    let _: &f64 = &k.wand.eye_y;
    let _: &f64 = &k.wand.past_m;
    let _: &f64 = &k.wand.mark_secs;
    let _: &f64 = &k.wand.shut_rad;
    let _: &f64 = &k.wand.give_up_secs;
    let _: &f64 = &k.wand.sweep_secs;
    let _: &f64 = &k.wand.quick_secs;
    let _: &f64 = &k.wand.quick_down;
    let _: &f64 = &k.sticky.carry;
    let _: &f64 = &k.sticky.max;
    let _: &f64 = &k.sticky.live;
    let _: &f64 = &k.sticky.issue;
    let _: &f64 = &k.sticky.speed;
    let _: &f64 = &k.sticky.gravity;
    let _: &f64 = &k.sticky.drag;
    let _: &f64 = &k.sticky.radius;
    let _: &f64 = &k.sticky.life;
    let _: &f64 = &k.sticky.substeps;
    let _: &f64 = &k.sticky.solve_iters;
    let _: &f64 = &k.sticky.cd;
    let _: &f64 = &k.sticky.aim_y;
    let _: &f64 = &k.sticky.fov_deg;
    let _: &f64 = &k.sticky.off;
    let _: &f64 = &k.sticky.flat_y;
    let _: &f64 = &k.sticky.launch_noise;
    let _: &f64 = &k.sticky.stick_noise;
    let _: &f64 = &k.sticky.take_reach;
    let _: &f64 = &k.sticky.take_noise;
    let _: &f64 = &k.peeksight.out;
    let _: &f64 = &k.peeksight.off;
    let _: &f64 = &k.peeksight.turn_deg;
    let _: &f64 = &k.gunslots.near;
    let _: &f64 = &k.gunslots.out;
    let _: &f64 = &k.gunslots.lull;
    let _: &f64 = &k.gunslots.back;
    let _: &f64 = &k.gunroles.akms;
    let _: &f64 = &k.gunroles.belt_lane;
    let _: &f64 = &k.gunroles.belt_min;
    let _: &f64 = &k.gunroles.belt_squad;
    let _: &f64 = &k.gunroles.line_max;
    let _: &f64 = &k.gunroles.pistol_near;
    let _: &f64 = &k.gunroles.pistol_share;
    let _: &f64 = &k.gunroles.pistol_max;
    let _: &f64 = &k.gunroles.lane_k;
    let _: &f64 = &k.gunroles.file_cost;
    let _: &f64 = &k.kittree.rings;
    let _: &f64 = &k.kittree.lend;
    let _: &bool = &k.squadthrow.on;
    let _: &f64 = &k.squadthrow.frag_roll_k;
    let _: &f64 = &k.squadthrow.again_secs;
    let _: &f64 = &k.squadthrow.shown_secs;
    let _: &f64 = &k.squadthrow.slack_m;
    let _: &f64 = &k.squadthrow.walk_m;
    let _: &f64 = &k.squadthrow.min_m;
    let _: &f64 = &k.squadthrow.max_m;
    let _: &f64 = &k.squadthrow.tries;
    let _: &f64 = &k.squadthrow.keep_secs;
    let _: &f64 = &k.squadthrow.walk_secs;
    let _: &f64 = &k.squadthrow.turn_secs;
    let _: &f64 = &k.squadthrow.toss_secs;
    let _: &f64 = &k.squadthrow.lob_speed;
    let _: &f64 = &k.squadthrow.lob_near_m;
    let _: &f64 = &k.squadthrow.lob_up_m;
    let _: &f64 = &k.squadthrow.lob_walls;
    let _: &f64 = &k.squadthrow.lob_wall_m;
    let _: &f64 = &k.squadthrow.lob_wall_h;
    let _: &f64 = &k.squadthrow.lob_sims;
    let _: &f64 = &k.squadthrow.lob_cells;
    let _: &f64 = &k.squadthrow.lob_keep_secs;
    let _: &f64 = &k.squadthrow.settle_secs;
    let _: &f64 = &k.squadthrow.duck_secs;
    let _: &f64 = &k.squadthrow.duck_throw_secs;
    let _: &f64 = &k.squadthrow.duck_cells;
    let _: &f64 = &k.squadthrow.duck_apart_m;
    let _: &f64 = &k.squadthrow.duck_min_m;
    let _: &f64 = &k.squad40.near_m;
    let _: &f64 = &k.squad40.arm_slack;
    let _: &f64 = &k.squad40.reach_slack;
    let _: &f64 = &k.squad40.muzzle_ahead;
    let _: &f64 = &k.squad40.muzzle_up;
    let _: &f64 = &k.squad40.head_drop;
    let _: &f64 = &k.squad40.aim_rad;
    let _: &f64 = &k.squad40.aim_secs;
    let _: &f64 = &k.squad40.raise_secs;
    let _: &f64 = &k.squad40.after_secs;
    let _: &f64 = &k.squad40.shown_secs;
    let _: &f64 = &k.autopilot.show_secs;
    let _: &f64 = &k.autopilot.keep_bangs;
    let _: &f64 = &k.autopilot.contact_m;
    let _: &f64 = &k.cmdring.wedges;
    let _: &f64 = &k.cmdring.flick_px;
    let _: &f64 = &k.cmdring.hold_secs;
    let _: &f64 = &k.cmdring.stay_hold_secs;
    let _: &f64 = &k.cmdring.tap_secs;
    let _: &f64 = &k.cmdring.edge_x;
    let _: &f64 = &k.cmdring.edge_y;
    let _: &f64 = &k.cmdring.man_px;
    let _: &[f64; 2] = &k.cmdring.ref_;
    let _: &bool = &k.cmdring.hint;
    let _: &bool = &k.cmdring.goal_half;
    let _: &bool = &k.cmdring.twice;
    let _: &f64 = &k.cmdring.twice_secs;
    let _: &f64 = &k.cmdring.twice_px;
    let _: &f64 = &k.cmdring.hint_wall_m;
    let _: &f64 = &k.cmdring.hint_spot_m;
    let _: &f64 = &k.cmdring.hint_man_m;
    let _: &f64 = &k.torch.lit_at;
    let _: &f64 = &k.torch.off_k;
    let _: &f64 = &k.torch.spill_m;
    let _: &f64 = &k.torch.day_m;
    let _: &f64 = &k.torch.ahead_m;
    let _: &f64 = &k.torch.on_secs;
    let _: &f64 = &k.torch.off_secs;
    let _: &f64 = &k.torch.hold_secs;
    let _: &f64 = &k.torch.ask_secs;
    let _: &f64 = &k.torch.calm_aw;
    let _: &f64 = &k.torch.wall_m;
    let _: &f64 = &k.torch.wall_back_m;
    let _: &f64 = &k.torch.low_tan;
    let _: &f64 = &k.torch.low_rate;
    let _: &f64 = &k.morale.half_life;
    let _: &f64 = &k.morale.ask_secs;
    let _: &f64 = &k.morale.alone;
    let _: &f64 = &k.morale.alone_m;
    let _: &f64 = &k.morale.dark;
    let _: &f64 = &k.morale.lit_asks;
    let _: &f64 = &k.morale.blast;
    let _: &f64 = &k.morale.shock_cap;
    let _: &f64 = &k.morale.blast_m;
    let _: &f64 = &k.morale.near_m;
    let _: &f64 = &k.morale.far_k;
    let _: &f64 = &k.morale.jumpy;
    let _: &f64 = &k.morale.broken;
    let _: &f64 = &k.morale.hyst;
    let _: &f64 = &k.morale.duck_secs;
    let _: &f64 = &k.morale.down_secs;
    let _: &f64 = &k.morale.back_until;
    let _: &f64 = &k.morale.back_m;
    let _: &f64 = &k.morale.back_speed;
    let _: &f64 = &k.morale.burst_rounds;
    let _: &f64 = &k.morale.burst_every;
    let _: &f64 = &k.morale.spread;
    let _: &f64 = &k.morale.call2_secs;
    let _: &f64 = &k.morale.bait_m;
    let _: &f64 = &k.morale.bait_short;
    let _: &f64 = &k.morale.bait_every;
    let _: &f64 = &k.morale.reload_secs;
    let _: &f64 = &k.morale.bark_secs;
    let _: &f64 = &k.morale.fan_deg;
    let _: &f64 = &k.morale.fan_step;
    let _: &f64 = &k.morale.straight_m;
    let _: &f64 = &k.morale.green_broken;
    let _: &f64 = &k.morale.leader_down;
    let _: &f64 = &k.morale.leader_shock;
    let _: &f64 = &k.morale.flinch_secs;
    let _: &f64 = &k.morale.shout_gap;
    let _: &f64 = &k.morale.order_m;
    let _: &f64 = &k.morale.order_beat;
    let _: &f64 = &k.morale.order_secs;
    let _: &f64 = &k.morale.order_gap;
    let _: &f64 = &k.morale.stand_secs;
    let _: &f64 = &k.morale.side_m;
    let _: &f64 = &k.morale.steady_duck;
    let _: &f64 = &k.morale.hold_shock;
    let _: &f64 = &k.morale.hold_max;
    let _: &f64 = &k.morale.nerve_k.brittle;
    let _: &f64 = &k.morale.nerve_k.normal;
    let _: &f64 = &k.morale.nerve_k.steady;
    let _: &f64 = &k.condition.hurt_pace;
    let _: &f64 = &k.condition.hurt_sup;
    let _: &f64 = &k.condition.hurt_secs;
    let _: &f64 = &k.inhand.from;
    let _: &f64 = &k.inhand.to;
    let _: &f64 = &k.inhand.wrist;
    let _: &f64 = &k.inhand.both;
    let _: &f64 = &k.inhand.rate;
    let _: &f64 = &k.inhand.holster_out;
    let _: &f64 = &k.packside.gap;
    let _: &f64 = &k.packside.handle;
    let _: &f64 = &k.packside.gun;
    let _: &f64 = &k.packside.over;
    let _: &f64 = &k.packside.head_min;
    let _: &f64 = &k.packside.butt;
    let _: &f64 = &k.packside.rest;
    let _: &f64 = &k.packside.pitch0;
    let _: &f64 = &k.packside.pitch1;
    let _: &f64 = &k.runner.close_m;
    let _: &f64 = &k.runner.out_secs;
    let _: &f64 = &k.runner.beat_secs;
    let _: &f64 = &k.runner.tell_m;
    let _: &f64 = &k.runner.told_m;
    let _: &f64 = &k.runner.tell_beat;
    let _: &f64 = &k.runner.tell_aware;
    let _: &f64 = &k.runner.shy_m;
    let _: &f64 = &k.runner.mate_m;
    let _: &f64 = &k.runner.mate_secs;
    let _: &f64 = &k.runner.clear_m;
    let _: &f64 = &k.runner.edge_m;
    let _: &f64 = &k.runner.corner_m;
    let _: &f64 = &k.runner.gone_secs;
    let _: &f64 = &k.runner.stall_secs;
    let _: &f64 = &k.runner.retry_secs;
    let _: &f64 = &k.runner.retries;
    let _: &f64 = &k.runner.replan_secs;
    let _: &f64 = &k.runner.ahead_m;
    let _: &f64 = &k.runner.ask_cells;
    let _: &f64 = &k.runner.ask_wide;
    let _: &f64 = &k.startle.beat;
    let _: &f64 = &k.startle.draw;
    let _: &f64 = &k.startle.fresh;
    let _: &f64 = &k.startle.late;
    let _: &f64 = &k.startle.run_m;
    let _: &f64 = &k.startle.near_m;
    let _: &f64 = &k.startle.clear_secs;
    let _: &f64 = &k.startle.run_secs;
    let _: &f64 = &k.startle.stuck_secs;
    let _: &f64 = &k.startle.stuck_m;
    let _: &f64 = &k.smokebreak.light_secs;
    let _: &[f64; 2] = &k.smokebreak.light_up;
    let _: &f64 = &k.smokebreak.flame_at;
    let _: &f64 = &k.smokebreak.flame;
    let _: &f64 = &k.smokebreak.every;
    let _: &f64 = &k.smokebreak.draw;
    let _: &f64 = &k.smokebreak.out_after;
    let _: &f64 = &k.smokebreak.out;
    let _: &[f64; 3] = &k.smokebreak.flame_color;
    let _: &f64 = &k.smokebreak.flame_k;
    let _: &f64 = &k.smokebreak.flame_range;
    let _: &[f64; 3] = &k.smokebreak.ember_color;
    let _: &f64 = &k.smokebreak.ember_k;
    let _: &f64 = &k.smokebreak.ember_draw_k;
    let _: &f64 = &k.smokebreak.ember_range;
    let _: &f64 = &k.smokebreak.ember_every;
    let _: &f64 = &k.smokebreak.tip_out;
    let _: &f64 = &k.smokebreak.tip_up;
    let _: &f64 = &k.smokebreak.thread.radius;
    let _: &f64 = &k.smokebreak.thread.cells;
    let _: &f64 = &k.smokebreak.thread.density;
    let _: &f64 = &k.smokebreak.thread.temp;
    let _: &f64 = &k.smokebreak.thread.speed;
    let _: &f64 = &k.smokebreak.thread.push;
    let _: &f64 = &k.smokebreak.thread.jitter;
    let _: &f64 = &k.smokebreak.thread_gone;
    let _: &f64 = &k.smokebreak.breath.radius;
    let _: &f64 = &k.smokebreak.breath.cells;
    let _: &f64 = &k.smokebreak.breath.density;
    let _: &f64 = &k.smokebreak.breath.temp;
    let _: &f64 = &k.smokebreak.breath.speed;
    let _: &f64 = &k.smokebreak.breath.push;
    let _: &f64 = &k.smokebreak.breath.expand;
    let _: &f64 = &k.smokebreak.breath.attack;
    let _: &f64 = &k.smokebreak.breath.jitter;
    let _: &f64 = &k.smokebreak.breath.down;
    let _: &f64 = &k.smokebreak.puffs.records;
    let _: &f64 = &k.smokebreak.puffs.count;
    let _: &f64 = &k.smokebreak.puffs.speed;
    let _: &f64 = &k.smokebreak.puffs.spread;
    let _: &f64 = &k.smokebreak.puffs.life;
    let _: &f64 = &k.smokebreak.puffs.size;
    let _: &f64 = &k.smokebreak.puffs.opacity;
    let _: &[f64; 3] = &k.smokebreak.color;
    let _: &f64 = &k.smokebreak.mouth_ahead;
    let _: &f64 = &k.smokebreak.mouth_down;
    let _: &f64 = &k.showwhy.line_after;
    let _: &f64 = &k.showwhy.line_gap;
    let _: &f64 = &k.showwhy.wave_every;
    let _: &f64 = &k.showwhy.wave_later;
    let _: &f64 = &k.showwhy.line_min;
    let _: &f64 = &k.showwhy.line_say_gap;
    let _: &f64 = &k.showwhy.mate_half;
    let _: &f64 = &k.showwhy.squeeze_gap;
    let _: &f64 = &k.showwhy.squeeze_reach;
    let _: &f64 = &k.showwhy.fist_after;
    let _: &f64 = &k.showwhy.hold_every;
    let _: &f64 = &k.showwhy.hold_max;
    let _: &f64 = &k.showwhy.door_after;
    let _: &f64 = &k.showwhy.door_every;
    let _: &f64 = &k.showwhy.after_you_gap;
    let _: &f64 = &k.showwhy.hurt_late;
    let _: &f64 = &k.showwhy.lag_m;
    let _: &f64 = &k.showwhy.lag_every;
    let _: &f64 = &k.showwhy.lag_max;
    let _: &f64 = &k.showwhy.keep_after;
    let _: &f64 = &k.showwhy.keep_gap;
    let _: &f64 = &k.showwhy.keep_every;
    let _: &f64 = &k.showwhy.owed_for;
    let _: &f64 = &k.fire.height;
    let _: &f64 = &k.fire.mouth;
    let _: &f64 = &k.fire.coals;
    let _: &f64 = &k.fire.coal_swing;
    let _: &f64 = &k.fire.bed_r;
    let _: &f64 = &k.fire.depth;
    let _: &f64 = &k.fire.least;
    let _: &f64 = &k.fire.most;
    let _: &f64 = &k.fire.wander;
    let _: &f64 = &k.fire.wander_up;
    let _: &f64 = &k.fire.lift;
    let _: &f64 = &k.fire.kick;
    let _: &f64 = &k.fire.kick_secs;
    let _: &f64 = &k.fire.bright;
    let _: &f64 = &k.fire.reach;
    let _: &f64 = &k.fire.stir_gap;
    let _: &f64 = &k.fire.size;
    let _: &f64 = &k.fire.pop_rate;
    let _: &f64 = &k.fire.pop_slot;
    let _: &f64 = &k.fire.embers;
    let _: &f64 = &k.fire.ember_life;
    let _: &f64 = &k.fire.ember_speed;
    let _: &f64 = &k.fire.ember_size;
    let _: &f64 = &k.fire.ember_gain;
    let _: &f64 = &k.fire.ember_crack;
    let _: &f64 = &k.fire.ember_r;
    let _: &f64 = &k.fire.look.gain;
    let _: &f64 = &k.fire.look.core;
    let _: &f64 = &k.fire.look.sigma;
    let _: &f64 = &k.fire.look.speed;
    let _: &f64 = &k.fire.look.tall;
    let _: &f64 = &k.fire.look.torn;
    let _: &f64 = &k.fire.look.plume;
    let _: &f64 = &k.fire.look.shimmer;
    let _: &f64 = &k.fire.look.soot;
    let _: &f64 = &k.fire.look.breathe;
    let _: &f64 = &k.fire.look.react;
    let _: &f64 = &k.fire.look.lean;
    let _: &f64 = &k.fire.look.steps;
    let _: &f64 = &k.fire.smoke.up;
    let _: &f64 = &k.fire.smoke.radius;
    let _: &f64 = &k.fire.smoke.cells;
    let _: &f64 = &k.fire.smoke.density;
    let _: &f64 = &k.fire.smoke.temp;
    let _: &f64 = &k.fire.smoke.speed;
    let _: &f64 = &k.fire.smoke.push;
    let _: &f64 = &k.fire.smoke.jitter;
    let _: &f64 = &k.fire.smoke.every;
    let _: &f64 = &k.fire.smoke.view_r;
    let _: &f64 = &k.fire.smoke.lean;
    let _: &f64 = &k.fire.gas.up;
    let _: &f64 = &k.fire.gas.radius;
    let _: &f64 = &k.fire.gas.cells;
    let _: &f64 = &k.fire.gas.density;
    let _: &f64 = &k.fire.gas.temp;
    let _: &f64 = &k.fire.gas.speed;
    let _: &f64 = &k.fire.gas.push;
    let _: &f64 = &k.fire.gas.jitter;
    let _: &f64 = &k.fire.gas.every;
    let _: &f64 = &k.fire.gas.view_r;
    let _: &f64 = &k.fire.gas.keep;
    let _: &f64 = &k.fire.gas.ref_;
    let _: &f64 = &k.fire.gas.lo;
    let _: &f64 = &k.fire.gas.ease;
    let _: &f64 = &k.fire.bed;
    let _: &f64 = &k.fire.crack;
    let _: &f64 = &k.fire.bed_secs;
    let _: &f64 = &k.fire.max;
    let _: &f64 = &k.fire.draw_r;
    let _: &f64 = &k.fire.wind_max;
    let _: &f64 = &k.flinch.least;
    let _: &f64 = &k.flinch.full;
    let _: &f64 = &k.flinch.far;
    let _: &f64 = &k.flinch.min_travel;
    let _: &f64 = &k.flinch.steady_k;
    let _: &f64 = &k.flinch.break_secs;
    let _: &f64 = &k.flinch.hot_secs;
    let _: &f64 = &k.flinch.ease_k;
    let _: &f64 = &k.flinch.ask_secs;
    let _: &f64 = &k.darkdoor.stop_m;
    let _: &f64 = &k.darkdoor.pair_m;
    let _: &f64 = &k.darkdoor.dark_at;
    let _: &f64 = &k.darkdoor.dark_share;
    let _: &f64 = &k.darkdoor.lit_by;
    let _: &f64 = &k.darkdoor.samples;
    let _: &f64 = &k.darkdoor.ask_secs;
    let _: &f64 = &k.darkdoor.open_rad;
    let _: &f64 = &k.darkdoor.arm_secs;
    let _: &f64 = &k.darkdoor.stand_m;
    let _: &f64 = &k.darkdoor.side_m;
    let _: &f64 = &k.darkdoor.sweep_secs;
    let _: &f64 = &k.darkdoor.ear_m;
    let _: &f64 = &k.darkdoor.mate_m;
    let _: &f64 = &k.darkdoor.mate_try;
    let _: &f64 = &k.darkdoor.call_secs;
    let _: &f64 = &k.darkdoor.copy_secs;
    let _: &f64 = &k.darkdoor.lag_secs;
    let _: &f64 = &k.darkdoor.with_secs;
    let _: &f64 = &k.darkdoor.keep_aw;
    let _: &f64 = &k.darkdoor.keep_secs;
    let _: &f64 = &k.darkdoor.resume_secs;
    let _: &f64 = &k.darkdoor.seen_secs;
    let _: &f64 = &k.darkdoor.wait_secs;
    let _: &f64 = &k.darkdoor.again_secs;
    let _: &f64 = &k.darkdoor.fed_up_secs;
    let _: &f64 = &k.darkdoor.fed_up_n;
    let _: &f64 = &k.darkdoor.nerve_secs;
    let _: &f64 = &k.darkdoor.slow_k;
    let _: &f64 = &k.darkfoe.carry;
    let _: &f64 = &k.darkfoe.lit_at;
    let _: &f64 = &k.darkfoe.look_m;
    let _: &f64 = &k.darkfoe.step_m;
    let _: &f64 = &k.darkfoe.fan_rad;
    let _: &f64 = &k.darkfoe.see_share;
    let _: &f64 = &k.darkfoe.ask_secs;
    let _: &f64 = &k.darkfoe.on_secs;
    let _: &f64 = &k.darkfoe.on_jitter;
    let _: &f64 = &k.darkfoe.off_secs;
    let _: &f64 = &k.darkfoe.off_jitter;
    let _: &f64 = &k.darkfoe.hold_secs;
    let _: &f64 = &k.darkfoe.lamp_m;
    let _: &f64 = &k.darkfoe.blind_secs;
    let _: &f64 = &k.darkfoe.seek_m;
    let _: &f64 = &k.darkfoe.walk_m;
    let _: &f64 = &k.darkfoe.edge_m;
    let _: &f64 = &k.darkfoe.stumble_k;
    let _: &f64 = &k.darkfoe.again_secs;
    let _: &f64 = &k.darkfoe.moves;
    let _: &f64 = &k.darkfoe.quiet_secs;
    let _: &f64 = &k.darkfoe.calm_secs;
    let _: &f64 = &k.powerout.see_m;
    let _: &[f64; 2] = &k.powerout.think;
    let _: &f64 = &k.powerout.beat;
    let _: &f64 = &k.powerout.look_m;
    let _: &f64 = &k.powerout.about_m;
    let _: &f64 = &k.powerout.step_m;
    let _: &f64 = &k.powerout.fan_rad;
    let _: &f64 = &k.powerout.lit_at;
    let _: &f64 = &k.powerout.drop_k;
    let _: &f64 = &k.powerout.dark_after;
    let _: &f64 = &k.powerout.men;
    let _: &f64 = &k.powerout.max_m;
    let _: &f64 = &k.powerout.parties;
    let _: &f64 = &k.powerout.look_secs;
    let _: &f64 = &k.powerout.tries;
    let _: &f64 = &k.powerout.lever_secs;
    let _: &f64 = &k.powerout.overdue;
    let _: &f64 = &k.powerout.ask;
    let _: &[f64; 2] = &k.powerout.cover_m;
    let _: &f64 = &k.squaddeal.one;
    let _: &f64 = &k.squaddeal.two;
    let _: &f64 = &k.stayask.men;
    let _: &[f64; 2] = &k.rescuecheck.first;
    let _: &[f64; 2] = &k.rescuecheck.every;
    let _: &f64 = &k.rescuecheck.ask_secs;
    let _: &f64 = &k.rescuecheck.go_secs;
    let _: &f64 = &k.rescuecheck.wait_secs;
    let _: &f64 = &k.rescuecheck.tries;
    let _: &f64 = &k.rescuecheck.feet_m;
    let _: &f64 = &k.rescuecheck.word_secs;
    let _: &f64 = &k.kitload.gun_kg;
    let _: &[f64; 2] = &k.kitload.gun_bulk;
    let _: &f64 = &k.kitload.pace;
    let _: &f64 = &k.kitload.seen;
    let _: &f64 = &k.kitload.heard;
    let _: &f64 = &k.kitload.sprint;
    let _: &f64 = &k.kitload.pace_top;
    let _: &f64 = &k.rescueend.out_m;
    let _: &f64 = &k.rescueend.clear_secs;
    let _: &f64 = &k.rescueend.marker_m;
    let _: &f64 = &k.rescueend.reach_m;
    let _: &f64 = &k.rescueend.gap_m;
    let _: &f64 = &k.rescueend.gap_step_m;
    let _: &f64 = &k.rescueflush.after_secs;
    let _: &f64 = &k.rescueflush.gap_secs;
    let _: &f64 = &k.rescueflush.again_secs;
    let _: &f64 = &k.rescueflush.reach_m;
    let _: &f64 = &k.rescueflush.kill_secs;
    let _: &f64 = &k.rescueflush.busy_secs;
    let _: &f64 = &k.rescueflush.smoke_k;
    let _: &f64 = &k.rescueflush.at_door_m;
    let _: &bool = &k.posemath.singles;
    let _: &f64 = &k.sense.glow.sweep;
    let _: &f64 = &k.sense.smoke.sigma;
    let _: &f64 = &k.sense.smoke.decay;
    let _: &f64 = &k.sense.smoke.spread;
    let _: &f64 = &k.sense.smoke.widest;
    let _: &f64 = &k.sense.smoke.rise;
    let _: &f64 = &k.sense.smoke.rise_rate;
    let _: &f64 = &k.sense.smoke.in_wind;
    let _: &f64 = &k.sense.smoke.least;
    let _: &f64 = &k.sense.smoke.gone;
    let _: &f64 = &k.sense.smoke.cap;
    let _: &f64 = &k.sense.smoke.emitters;
    let _: &f64 = &k.sense.smoke.reserve;
    let _: &f64 = &k.cmdcheck.keys;
    let _: &f64 = &k.cmdcheck.key_len;
    let _: &f64 = &k.cmdcheck.span;
    let _: &f64 = &k.cmdcheck.wheel;
    let _: &f64 = &k.cmdcheck.far;
    let _: &f64 = &k.cmdcheck.dist_min;
    let _: &f64 = &k.cmdcheck.dist_max;
    let _: &f64 = &k.cmdcheck.fov_min;
    let _: &f64 = &k.cmdcheck.fov_max;
    let _: &f64 = &k.cmdcheck.aspect_min;
    let _: &f64 = &k.cmdcheck.aspect_max;
    let _: &f64 = &k.cmdcheck.reach;
    let _: &f64 = &k.cmdcheck.steep;
    let _: &f64 = &k.seats.max;
    let _: &f64 = &k.seats.apart;
    let _: &f64 = &k.seats.keep;
    let _: &f64 = &k.doze.min_wait;
    let _: &f64 = &k.doze.max_wait;
    let _: &f64 = &k.doze.chair_m;
    let _: &f64 = &k.doze.nod_secs;
    let _: &f64 = &k.doze.rise_secs;
    let _: &f64 = &k.doze.hear_k;
    let _: &f64 = &k.doze.light_at;
    let _: &f64 = &k.doze.light_secs;
    let _: &f64 = &k.doze.ask_secs;
    let _: &f64 = &k.doze.wake_aw;
    let _: &f64 = &k.doze.bump_aw;
    let _: &f64 = &k.doze.stir_secs;
    let _: &f64 = &k.doze.breath_secs;
    let _: &f64 = &k.doze.screen_m;
    let _: &f64 = &k.doze.pull_m;
    let _: &f64 = &k.doze.nap_wait;
    let _: &f64 = &k.doze.settle_secs;
    let _: &f64 = &k.doze.ears;
    let _: &f64 = &k.doze.noises;
    let _: &f64 = &k.doze.idle_secs;
    let _: &f64 = &k.netquiet.unanswered;
    let _: &f64 = &k.netquiet.heard_it;
    let _: &f64 = &k.netquiet.body;
    let _: &f64 = &k.netquiet.body_told;
    let _: &f64 = &k.netquiet.fall_seen;
    let _: &f64 = &k.netquiet.fall_unseen;
    let _: &f64 = &k.netquiet.steady_k;
    let _: &f64 = &k.netquiet.ask_beat;
    let _: &f64 = &k.netquiet.body_beat;
    let _: &f64 = &k.netquiet.bark_secs;
    let _: &f64 = &k.netquiet.say_hold;
    let _: &f64 = &k.netquiet.shooter_secs;
    let _: &f64 = &k.netquiet.body_m;
    let _: &f64 = &k.netquiet.whip_shift;
    let _: &f64 = &k.netquiet.whip_secs;
    let _: &f64 = &k.netquiet.whip_hold;
    let _: &f64 = &k.netquiet.whip_turn;
    let _: &f64 = &k.netquiet.whip_turn_broken;
    let _: &f64 = &k.netquiet.whip_m;
    let _: &f64 = &k.netquiet.whip_near_m;
    let _: &f64 = &k.netquiet.last_beat;
    let _: &f64 = &k.netquiet.calm_secs;
    let _: &f64 = &k.netquiet.alert_secs;
    let _: &f64 = &k.netquiet.give_up;
    let _: &f64 = &k.netquiet.mate_m;
    let _: &f64 = &k.netquiet.arc_deg;
    let _: &f64 = &k.netquiet.eye_deg;
    let _: &f64 = &k.netquiet.eye_slack;
    let _: &f64 = &k.netquiet.crouch;
    let _: &f64 = &k.netquiet.wary;
    let _: &f64 = &k.netquiet.back_speed;
    let _: &f64 = &k.netquiet.max_m;
    let _: &f64 = &k.netquiet.door_m;
    let _: &f64 = &k.netquiet.eye_m;
    let _: &f64 = &k.netquiet.stay_m;
    let _: &f64 = &k.attention.focus_deg;
    let _: &f64 = &k.attention.side_k;
    let _: &f64 = &k.attention.ear_k;
    let _: &f64 = &k.attention.break_gain;
    let _: &f64 = &k.attention.calm_aw;
    let _: &f64 = &k.attention.draw_aw;
    let _: &f64 = &k.attention.floor;
    let _: &f64 = &k.attention.whistle_secs;
    let _: &f64 = &k.attention.noise_secs;
    let _: &f64 = &k.attention.door_secs;
    let _: &f64 = &k.attention.light_secs;
    let _: &f64 = &k.attention.tell_secs;
    let _: &f64 = &k.attention.dwell_sweep;
    let _: &f64 = &k.attention.walk_cap;
    let _: &f64 = &k.attention.wear_k;
    let _: &f64 = &k.attention.memory_secs;
    let _: &f64 = &k.attention.spent;
    let _: &f64 = &k.attention.same_secs;
    let _: &f64 = &k.attention.lent_back_secs;
    let _: &f64 = &k.attention.ring_m;
    let _: &f64 = &k.attention.way_reask;
    let _: &f64 = &k.picture.look_secs;
    let _: &f64 = &k.picture.glimpse;
    let _: &f64 = &k.picture.tell_secs;
    let _: &f64 = &k.picture.shout_m;
    let _: &f64 = &k.picture.pop_m;
    let _: &f64 = &k.picture.made_out;
    let _: &f64 = &k.picture.made_out_m;
    let _: &f64 = &k.picture.fresh_secs;
    let _: &f64 = &k.picture.stale_secs;
    let _: &f64 = &k.picture.stale_k;
    let _: &f64 = &k.picture.forget_secs;
    let _: &f64 = &k.picture.lost_k;
    let _: &f64 = &k.picture.walk_mps;
    let _: &f64 = &k.picture.walk_max;
    let _: &f64 = &k.picture.heard_m;
    let _: &f64 = &k.picture.maybe_k;
    let _: &f64 = &k.picture.reach_m;
    let _: &f64 = &k.picture.eye_h;
    let _: &f64 = &k.picture.chest_h;
    let _: &f64 = &k.picture.low_h;
    let _: &f64 = &k.picture.head_up;
    let _: &f64 = &k.picture.flee_cost;
    let _: &f64 = &k.picture.stack_k;
    let _: &f64 = &k.picture.step_at;
    let _: &f64 = &k.picture.step_free;
    let _: &f64 = &k.picture.step_m;
    let _: &f64 = &k.picture.step_after;
    let _: &f64 = &k.picture.step_rest;
    let _: &f64 = &k.picture.step_secs;
    let _: &f64 = &k.picture.show_m;
    let _: &f64 = &k.response.clear_m;
    let _: &f64 = &k.response.eta_step;
    let _: &f64 = &k.response.word_secs;
    let _: &f64 = &k.response.call_secs;
    let _: &f64 = &k.handwork.stand_m;
    let _: &f64 = &k.handwork.step_rate;
    let _: &f64 = &k.handwork.step_pace;
    let _: &f64 = &k.handwork.stand_side;
    let _: &f64 = &k.handwork.wait_secs;
    let _: &f64 = &k.handwork.lever_secs;
    let _: &f64 = &k.handwork.lever_at;
    let _: &f64 = &k.handwork.stow_secs;
    let _: &f64 = &k.handwork.radio_k;
    let _: &f64 = &k.handwork.radio_min;
    let _: &f64 = &k.handwork.radio_max;
    let _: &f64 = &k.workstow.mate_down;
    let _: &f64 = &k.workstow.mate_up;
    let _: &f64 = &k.workstow.show;
    let _: &f64 = &k.workstow.down_max;
    let _: &f64 = &k.workstow.sam_up;
    let _: &f64 = &k.workstow.fight_cap;
    let _: &f64 = &k.squadnet.near;
    let _: &f64 = &k.squadnet.edge_in;
    let _: &f64 = &k.squadnet.click;
    let _: &f64 = &k.squadnet.click_rate;
    let _: &f64 = &k.squadnet.down_gain;
    let _: &f64 = &k.squadnet.down_rate;
    let _: &f64 = &k.squadnet.down_secs;
    let _: &f64 = &k.takedown.partner_hit;
    let _: &f64 = &k.takedown.partner_end;
    let _: &f64 = &k.takedown.breath_hz;
    let _: &f64 = &k.takedown.breath_rise;
    let _: &f64 = &k.takedown.breath_in;
    let _: &f64 = &k.stealthpose.sway_secs;
    let _: &f64 = &k.stealthpose.sway_m;
    let _: &f64 = &k.stealthpose.drift_m;
    let _: &f64 = &k.stealthpose.rise_secs;
    let _: &f64 = &k.stealthpose.cable_in;
    let _: &f64 = &k.stealthpose.cable_bow;
    let _: &f64 = &k.stealthpose.cable_step;
    let _: &bool = &k.look.on;
    let _: &bool = &k.lived.on;
    let _: &f64 = &k.env.exposure;
    let _: &f64 = &k.env.tube_gain;
    let _: &bool = &k.shapes.on;
    let _: &[f64; 2] = &k.flash.secs;
    let _: &[f64; 2] = &k.flash.stagger;
    let _: &f64 = &k.flash.lift;
    let _: &f64 = &k.flash.concuss_m;
    let _: &f64 = &k.flash.concuss_k;
    let _: &f64 = &k.flash.own_m;
    let _: &f64 = &k.flash.own_secs;
    let _: &f64 = &k.flash.fuse;
    let _: &f64 = &k.pace.max;
    let _: &f64 = &k.pace.accel;
    let _: &f64 = &k.pace.turn;
    let _: &f64 = &k.pace.swing;
    let _: &f64 = &k.lights.after_secs;
    let _: &f64 = &k.lights.hot_secs;
    let _: &f64 = &k.lights.step_secs;
    let _: &f64 = &k.lights.tail;
    let _: &f64 = &k.lights.on_m;
    let _: &f64 = &k.lights.alone_m;
    let _: &f64 = &k.lights.look_lit;
    let _: &f64 = &k.lights.look_below;
    let _: &f64 = &k.lights.look_m;
    let _: &f64 = &k.lights.lit_at;
    let _: &f64 = &k.lights.lit_m;
    let _: &f64 = &k.lights.lit_mark_m;
    let _: &f64 = &k.lights.wall_m;
    let _: &f64 = &k.lights.wall_back_m;
    let _: &f64 = &k.lights.wall_secs;
    let _: &f64 = &k.nvg.quiet_secs;
    let _: &f64 = &k.nvg.lead_tail;
    let _: &f64 = &k.nvg.dark_at;
    let _: &f64 = &k.nvg.black_at;
    let _: &f64 = &k.nvg.black_reads;
    let _: &f64 = &k.nvg.clear_reads;
    let _: &f64 = &k.nvg.albedo;
    let _: &f64 = &k.nvg.noise;
    let _: &f64 = &k.nvg.down_secs;
    let _: &f64 = &k.nvg.up_secs;
    let _: &f64 = &k.nvg.call_gap;
    let _: &f64 = &k.irlight.show;
    let _: &f64 = &k.irlight.hold;
    let _: &f64 = &k.irlight.power;
    let _: &f64 = &k.irlight.reach;
    let _: &f64 = &k.irlight.cone;
    let _: &f64 = &k.irlight.core;
    let _: &f64 = &k.irlight.dip;
    let _: &f64 = &k.irlight.near_m;
    let _: &f64 = &k.irlight.near_min;
    let _: &f64 = &k.irlight.near_up;
    let _: &f64 = &k.irlight.near_down;
    let _: &f64 = &k.wallpen.min_energy;
    let _: &f64 = &k.wallpen.max_things;
    let _: &f64 = &k.wallpen.deflect_deg;
    let _: &f64 = &k.wallpen.behind_m;
    let _: &f64 = &k.wallpen.thru_rounds;
    let _: &f64 = &k.wallpen.thru_rest_secs;
    let _: &f64 = &k.doortools.charges;
    let _: &f64 = &k.doortools.bangs;
    let _: &f64 = &k.doortools.word_tap;
    let _: &f64 = &k.doortools.stack_cap;
    let _: &f64 = &k.doortools.still_secs;
    let _: &f64 = &k.doortools.kick_cap;
    let _: &f64 = &k.doortools.in_cap;
    let _: &f64 = &k.doortools.shove;
    let _: &f64 = &k.doortools.hold_open;
    let _: &f64 = &k.doortools.toss_after_open;
    let _: &f64 = &k.doortools.bang_in;
    let _: &f64 = &k.doortools.fuse_after_land;
    let _: &f64 = &k.doortools.air_in;
    let _: &f64 = &k.doortools.air_h;
    let _: &f64 = &k.doortools.toss_cap;
    let _: &f64 = &k.doortools.toss_after_kick;
    let _: &f64 = &k.doortools.throw_reach;
    let _: &f64 = &k.doortools.toss_turn;
    let _: &f64 = &k.doortools.frag_way_wait;
    let _: &f64 = &k.doortools.frag_way_open;
    let _: &f64 = &k.doortools.frag_way_ajar;
    let _: &f64 = &k.doortools.last_out_m;
    let _: &f64 = &k.doortools.after_bang;
    let _: &f64 = &k.doortools.quiet;
    let _: &f64 = &k.doortools.entry_cap;
    let _: &f64 = &k.doortools.second;
    let _: &f64 = &k.doortools.sweep_rad;
    let _: &f64 = &k.doortools.sweep_secs;
    let _: &f64 = &k.doortools.corner_secs;
    let _: &f64 = &k.doortools.corner_rad;
    let _: &f64 = &k.doortools.corner_cap;
    let _: &f64 = &k.doortools.lead_seen_min;
    let _: &f64 = &k.doortools.lead_seen_secs;
    let _: &f64 = &k.doortools.lead_seen_m;
    let _: &f64 = &k.doortools.sector_secs;
    let _: &f64 = &k.doortools.scan_rate;
    let _: &f64 = &k.doortools.look_rad;
    let _: &f64 = &k.doortools.look_m;
    let _: &f64 = &k.doortools.dead_cells;
    let _: &f64 = &k.doortools.search_moves;
    let _: &f64 = &k.doortools.search_cap;
    let _: &f64 = &k.doortools.search_cells;
    let _: &f64 = &k.doortools.tuck;
    let _: &f64 = &k.doortools.edge_out;
    let _: &f64 = &k.doortools.bang_file;
    let _: &f64 = &k.doortools.set_secs;
    let _: &f64 = &k.doortools.knock_m;
    let _: &f64 = &k.doortools.stun_m;
    let _: &f64 = &k.doortools.stagger_secs;
    let _: &f64 = &k.doortools.backblast_m;
    let _: &f64 = &k.doortools.backblast_secs;
    let _: &f64 = &k.doortools.blow_vel;
    let _: &f64 = &k.doortools.slice_back;
    let _: &f64 = &k.doortools.slice_look;
    let _: &f64 = &k.doortools.slice_edge;
    let _: &f64 = &k.doortools.slice_deep;
    let _: &f64 = &k.doortools.slice_pace;
    let _: &f64 = &k.doortools.word_stack;
    let _: &f64 = &k.doortools.word_in;
    let _: &f64 = &k.doortools.pod_apart;
    let _: &f64 = &k.doortools.thr_min;
    let _: &f64 = &k.doortools.thr_quiet;
    let _: &f64 = &k.doortools.thr_cap;
    let _: &f64 = &k.doortools.thr_bangs;
    let _: &f64 = &k.doortools.thr_pocket;
    let _: &f64 = &k.doortools.thr_shot_m;
    let _: &f64 = &k.doortools.thr_throw_m;
    let _: &f64 = &k.doortools.thr_flash_m;
    let _: &f64 = &k.doortools.thr_turn;
    let _: &f64 = &k.doortools.thr_next;
    let _: &f64 = &k.doortools.thr_light;
    let _: &f64 = &k.doortools.thr_in;
    let _: &f64 = &k.doortools.thr_pie_apart;
    let _: &f64 = &k.doortools.thr_lean_m;
    let _: &f64 = &k.doortools.thr_knee_in;
    let _: &f64 = &k.doortools.thr_lean_look;
    let _: &f64 = &k.doortools.thr_doors_m;
    let _: &f64 = &k.doortools.thr_doors_walk;
    let _: &f64 = &k.doortools.thr_doors_stay;
    let _: &f64 = &k.doortools.other_door_m;
    let _: &f64 = &k.doortools.buddy_m;
    let _: &f64 = &k.doortools.buddy_pair_m;
    let _: &f64 = &k.doortools.buddy_left_m;
    let _: &f64 = &k.doortools.flow_cells;
    let _: &f64 = &k.doortools.flow_moves;
    let _: &f64 = &k.doortools.flow_reach;
    let _: &f64 = &k.doortools.flow_deep;
    let _: &f64 = &k.doortools.flow_bound;
    let _: &f64 = &k.doortools.flow_secs;
    let _: &f64 = &k.doortools.flow_fight;
    let _: &f64 = &k.doortools.flow_max;
    let _: &f64 = &k.doortools.word_cuts_cap;
    let _: &f64 = &k.doortools.bang_into_min;
    let _: &f64 = &k.doortools.bang_into_lit;
    let _: &f64 = &k.doortools.bang_into_deep;
    let _: &f64 = &k.doortools.bang_into_wide;
    let _: &f64 = &k.doortools.bang_hand_m;
    let _: &f64 = &k.doortools.bang_pass_v;
    let _: &f64 = &k.doortools.pod_frame_m;
    let _: &f64 = &k.doortools.thr_hand_wait;
    let _: &f64 = &k.doortools.thr_hand_every;
    let _: &f64 = &k.doortools.frag_hold_cap;
    let _: &f64 = &k.doortools.threat_secs;
    let _: &f64 = &k.doortools.threat_cap;
    let _: &f64 = &k.doortools.threat_near_m;
    let _: &f64 = &k.doortools.threat_stuck;
    let _: &f64 = &k.doortools.thr_set_secs;
    let _: &f64 = &k.doortools.thr_first;
    let _: &f64 = &k.doortools.thr_first_min;
    let _: &f64 = &k.doortools.thr_first_stale;
    let _: &f64 = &k.doortools.thr_first_in;
    let _: &f64 = &k.doortools.thr_last_m;
    let _: &f64 = &k.doortools.thr_last_fresh;
    let _: &f64 = &k.roomfight.evidence_secs;
    let _: &f64 = &k.roomfight.sticky;
    let _: &f64 = &k.roomfight.crowd;
    let _: &f64 = &k.roomfight.fear_min;
    let _: &f64 = &k.roomfight.close_m;
    let _: &f64 = &k.roomfight.no_shot;
    let _: &f64 = &k.roomfight.point_blank;
    let _: &f64 = &k.roomfight.no_shot_go;
    let _: &f64 = &k.roomfight.no_shot_pass;
    let _: &f64 = &k.roomfight.rally_k;
    let _: &f64 = &k.roomfight.arc_half_deg;
    let _: &f64 = &k.roomfight.cover_half_deg;
    let _: &f64 = &k.roomfight.cover_near_m;
    let _: &f64 = &k.roomfight.team_hot;
    let _: &f64 = &k.roomfight.let_go;
    let _: &f64 = &k.roomfight.step_look_secs;
    let _: &f64 = &k.roomfight.door_look_secs;
    let _: &f64 = &k.roomfight.door_shot_m;
    let _: &f64 = &k.roomfight.hold_short_secs;
    let _: &f64 = &k.roomfight.move_engage_m;
    let _: &f64 = &k.roomfight.lamp;
    let _: &f64 = &k.roomfight.preaim_deg;
    let _: &f64 = &k.orders.refresh_secs;
    let _: &f64 = &k.orders.door_pick_m;
    let _: &f64 = &k.orders.self_m;
    let _: &f64 = &k.orders.man_m;
    let _: &f64 = &k.orders.door_ray_pad;
    let _: &f64 = &k.orders.door_ray_reach_m;
    let _: &f64 = &k.orders.floor_snap_m;
    let _: &f64 = &k.orders.jamb_out;
    let _: &f64 = &k.orders.jamb_past;
    let _: &f64 = &k.orders.file_gap;
    let _: &f64 = &k.orders.seam_m;
    let _: &f64 = &k.orders.work_seen_m;
    let _: &f64 = &k.orders.crumb;
    let _: &f64 = &k.orders.trail_m;
    let _: &f64 = &k.orders.run;
    let _: &f64 = &k.orders.walk;
    let _: &f64 = &k.orders.gain;
    let _: &f64 = &k.orders.near_m;
    let _: &f64 = &k.orders.band_m;
    let _: &f64 = &k.orders.word_far_m;
    let _: &f64 = &k.orders.word_hold_s;
    let _: &f64 = &k.orders.file_apart;
    let _: &f64 = &k.orders.file_tight;
    let _: &f64 = &k.orders.off_door;
    let _: &f64 = &k.orders.aside_m;
    let _: &f64 = &k.orders.aside_lane;
    let _: &f64 = &k.orders.aside_off;
    let _: &f64 = &k.orders.file_queue_gap;
    let _: &f64 = &k.orders.file_queue_cost;
    let _: &f64 = &k.orders.prop_round_pad;
    let _: &f64 = &k.orders.prop_round_cost;
    let _: &f64 = &k.orders.prop_round_more;
    let _: &f64 = &k.orders.prop_round_door;
    let _: &f64 = &k.orders.aside_hold;
    let _: &f64 = &k.orders.brisk;
    let _: &f64 = &k.orders.brisk_in;
    let _: &f64 = &k.orders.onto;
    let _: &f64 = &k.orders.entry;
    let _: &f64 = &k.orders.move_spread;
    let _: &f64 = &k.orders.cold_secs;
    let _: &f64 = &k.orders.lane_in;
    let _: &f64 = &k.orders.lane_out;
    let _: &f64 = &k.orders.hold_stride;
    let _: &f64 = &k.orders.stuck_secs;
    let _: &f64 = &k.orders.hold_group_m;
    let _: &f64 = &k.orders.hold_door_m;
    let _: &f64 = &k.orders.cleared_dear;
    let _: &f64 = &k.orders.same_near_m;
    let _: &f64 = &k.orders.same_apart_m;
    let _: &f64 = &k.orders.same_long;
    let _: &f64 = &k.orders.trail_reach_m;
    let _: &f64 = &k.orders.trail_long;
    let _: &f64 = &k.cover.rear_read_in;
    let _: &f64 = &k.cover.rear_read_m;
    let _: &f64 = &k.cover.rear_read_deg;
    let _: &f64 = &k.cover.rear_read_secs;
    let _: &f64 = &k.cover.rear_read_sticky;
    let _: &f64 = &k.cover.rear_look;
    let _: &f64 = &k.cover.rear_walk_m;
    let _: &f64 = &k.cover.reach;
    let _: &f64 = &k.cover.deal_secs;
    let _: &f64 = &k.cover.gap_cells;
    let _: &f64 = &k.cover.through_rows;
    let _: &f64 = &k.cover.alcove_rows;
    let _: &f64 = &k.cover.bridge;
    let _: &f64 = &k.cover.in_m;
    let _: &f64 = &k.cover.lane;
    let _: &f64 = &k.cover.lane_keep;
    let _: &f64 = &k.cover.lane_secs;
    let _: &f64 = &k.cover.sam_deg;
    let _: &f64 = &k.cover.sam_k;
    let _: &f64 = &k.cover.held_k;
    let _: &f64 = &k.cover.shut_k;
    let _: &f64 = &k.cover.rear_k;
    let _: &f64 = &k.cover.near_m;
    let _: &f64 = &k.cover.contact_secs;
    let _: &f64 = &k.cover.rear_m;
    let _: &f64 = &k.cover.rear_from;
    let _: &f64 = &k.cover.rear_deg;
    let _: &f64 = &k.cover.sticky;
    let _: &f64 = &k.cover.keep_secs;
    let _: &f64 = &k.cover.rear_keep;
    let _: &f64 = &k.cover.behind_k;
    let _: &f64 = &k.cover.twist;
    let _: &f64 = &k.cover.back_in;
    let _: &f64 = &k.cover.back_out;
    let _: &f64 = &k.cover.back_body;
    let _: &f64 = &k.cover.back_turn_body;
    let _: &f64 = &k.cover.back_turn_secs;
    let _: &f64 = &k.cover.swing;
    let _: &f64 = &k.cover.step_m;
    let _: &f64 = &k.cover.step_after;
    let _: &f64 = &k.cover.step_off_m;
    let _: &f64 = &k.cover.step_reach;
    let _: &f64 = &k.cover.step_secs;
    let _: &f64 = &k.cover.step_place_m;
    let _: &f64 = &k.roomfit.spread0;
    let _: &f64 = &k.roomfit.spread_per;
    let _: &f64 = &k.roomfit.apart;
    let _: &f64 = &k.roomfit.loose;
    let _: &f64 = &k.roomfit.loose_k;
    let _: &f64 = &k.roomfit.near_k;
    let _: &f64 = &k.roomfit.near_free;
    let _: &f64 = &k.roomfit.wall_k;
    let _: &f64 = &k.roomfit.corner_k;
    let _: &f64 = &k.roomfit.funnel_m;
    let _: &f64 = &k.roomfit.funnel_pad;
    let _: &f64 = &k.roomfit.funnel_flare;
    let _: &f64 = &k.roomfit.funnel_k;
    let _: &f64 = &k.roomfit.own_k;
    let _: &f64 = &k.roomfit.known_funnel_k;
    let _: &f64 = &k.roomfit.shut_funnel_k;
    let _: &f64 = &k.roomfit.ang_k;
    let _: &f64 = &k.roomfit.ang_lo;
    let _: &f64 = &k.roomfit.ang_hi;
    let _: &f64 = &k.roomfit.ang_max;
    let _: &f64 = &k.roomfit.close_m;
    let _: &f64 = &k.roomfit.close_k;
    let _: &f64 = &k.roomfit.lane_m;
    let _: &f64 = &k.roomfit.lane_k;
    let _: &f64 = &k.roomfit.reach;
    let _: &f64 = &k.roomfit.near_m;
    let _: &f64 = &k.roomfit.known_k;
    let _: &f64 = &k.roomfit.shut_k;
    let _: &f64 = &k.roomfit.locked_k;
    let _: &f64 = &k.roomfit.front_k;
    let _: &f64 = &k.roomfit.second_k;
    let _: &f64 = &k.roomfit.sector_m;
    let _: &f64 = &k.roomfit.sector_k;
    let _: &f64 = &k.roomfit.sector_deg;
    let _: &f64 = &k.roomfit.sector_across;
    let _: &f64 = &k.roomfit.sector_look;
    let _: &f64 = &k.roomfit.sector_far_m;
    let _: &f64 = &k.roomfit.sector_far_k;
    let _: &f64 = &k.roomfit.watch_in_m;
    let _: &f64 = &k.roomfit.again_secs;
    let _: &f64 = &k.roomfit.sticky;
    let _: &f64 = &k.entry.near_m;
    let _: &f64 = &k.entry.line_deg;
    let _: &f64 = &k.entry.known_m;
    let _: &f64 = &k.entry.move_m;
    let _: &f64 = &k.entry.apart;
    let _: &f64 = &k.entry.mouth_m;
    let _: &f64 = &k.rear.secs;
    let _: &bool = &k.silent.on;
    let _: &bool = &k.silent.start;
    let _: &f64 = &k.silent.pace;
    let _: &f64 = &k.silent.step;
    let _: &f64 = &k.silent.noticed;
    let _: &bool = &k.pair.on;
    let _: &f64 = &k.pair.max_squad;
    let _: &f64 = &k.pair.pick_m;
    let _: &f64 = &k.pair.stage_m;
    let _: &f64 = &k.pair.reach;
    let _: &f64 = &k.pair.secs;
    let _: &f64 = &k.pair.scuffle;
    let _: &f64 = &k.pair.pace;
    let _: &f64 = &k.pair.go_pace;
    let _: &f64 = &k.pair.stalk_every;
    let _: &f64 = &k.pair.stalk_wait;
    let _: &f64 = &k.pair.ring_m;
    let _: &f64 = &k.pair.steady_face_secs;
    let _: &f64 = &k.pair.front_m;
    let _: &f64 = &k.pair.cone_m;
    let _: &f64 = &k.pair.cone_lit;
    let _: &f64 = &k.pair.watch_secs;
    let _: &f64 = &k.pair.watch_aw;
    let _: &f64 = &k.pair.heel_ring;
    let _: &f64 = &k.pair.heel_fresh;
    let _: &f64 = &k.pair.heel_wait;
    let _: &f64 = &k.pair.take_secs;
    let _: &f64 = &k.pair.dark_vis;
    let _: &f64 = &k.pair.dark_m;
    let _: &f64 = &k.pair.dark_h;
    let _: &f64 = &k.pair.lit_secs;
    let _: &f64 = &k.pair.lit_every;
    let _: &f64 = &k.rescue.resolve_secs;
    let _: &f64 = &k.rescue.resolve_spread;
    let _: &f64 = &k.rescue.second_hostage_secs;
    let _: &f64 = &k.rescue.aim_secs;
    let _: &f64 = &k.rescue.set_secs;
    let _: &f64 = &k.rescue.unset_react;
    let _: &f64 = &k.rescue.hold_secs;
    let _: &f64 = &k.rescue.roamers;
    let _: &f64 = &k.rescue.crew_call_secs;
    let _: &f64 = &k.rescue.squad;
    let _: &[f64; 3] = &k.rescue.sam_at;
    let _: &f64 = &k.rescue.sam_yaw;
    let _: &bool = &k.digin.on;
    let _: &f64 = &k.digin.reach_m;
    let _: &bool = &k.digin.keeper;
    let _: &f64 = &k.digin.noticed_secs;
    let _: &f64 = &k.digin.shut_secs;
    let _: &f64 = &k.digin.shut_home_rad;
    let _: &f64 = &k.digin.shut_doors;
    let _: &f64 = &k.digin.gather_m;
    let _: &f64 = &k.digin.gather_shy_m;
    let _: &[f64; 2] = &k.digin.think;
    let _: &[f64; 2] = &k.digin.wedge_secs;
    let _: &f64 = &k.digin.wedges;
    let _: &[f64; 2] = &k.digin.caught;
    let _: &f64 = &k.digin.try_secs;
    let _: &f64 = &k.digin.pace;
    let _: &f64 = &k.digin.close_m;
    let _: &f64 = &k.digin.prop_m;
    let _: &f64 = &k.digin.haul_speed;
    let _: &f64 = &k.digin.scrape_level;
    let _: &f64 = &k.digin.scrape_every;
    let _: &f64 = &k.digin.shout_m;
    let _: &f64 = &k.digin.alt_m;
    let _: &f64 = &k.digin.better;
    let _: &f64 = &k.digin.fear;
    let _: &f64 = &k.digin.hold_secs;
    let _: &f64 = &k.digin.redig_secs;
    let _: &f64 = &k.digin.lane_secs;
    let _: &bool = &k.pressure.on;
    let _: &bool = &k.pressure.second;
    let _: &f64 = &k.pressure.max_men;
    let _: &f64 = &k.pressure.memory_secs;
    let _: &f64 = &k.pressure.organise_k;
    let _: &bool = &k.pressure.warned;
    let _: &f64 = &k.pressure.report_secs;
    let _: &f64 = &k.pressure.stand_secs;
    let _: &f64 = &k.pressure.stand_grace;
    let _: &f64 = &k.pressure.again_secs;
    let _: &f64 = &k.pressure.stuns;
    let _: &f64 = &k.pressure.stun_know_secs;
    let _: &f64 = &k.pressure.stun_moved_secs;
    let _: &bool = &k.pressure.by_way;
    let _: &bool = &k.pressure.rescue_crew;
    let _: &bool = &k.pressure.drilled;
    let _: &f64 = &k.pressure.flank_max_m;
    let _: &f64 = &k.pressure.flank_k;
    let _: &f64 = &k.pressure.flank_slack_m;
    let _: &[f64; 2] = &k.pressure.last_man_secs;
    let _: &f64 = &k.pressure.loud_secs;
    let _: &[f64; 2] = &k.pressure.out_secs;
    let _: &f64 = &k.pressure.gather_secs;
    let _: &f64 = &k.pressure.flank_secs;
    let _: &f64 = &k.pressure.siege_secs;
    let _: &f64 = &k.pressure.word_secs;
    let _: &bool = &k.pressure.way_out;
    let _: &f64 = &k.pressure.way_out_m;
    let _: &f64 = &k.pressure.way_out_secs;
    let _: &bool = &k.pressure.sweep;
    let _: &f64 = &k.pressure.sweep_secs;
    let _: &f64 = &k.pressure.sweep_men;
    let _: &f64 = &k.pressure.sweep_beat;
    let _: &f64 = &k.pressure.sweep_pair_m;
    let _: &f64 = &k.lanes.secs;
    let _: &f64 = &k.lanes.reach_m;
    let _: &f64 = &k.lanes.half_deg;
    let _: &f64 = &k.lanes.width_m;
    let _: &f64 = &k.lanes.field_m;
    let _: &f64 = &k.lanes.own_m;
    let _: &f64 = &k.lanes.min_m;
    let _: &f64 = &k.lanes.round_k;
    let _: &f64 = &k.lanes.near_m;
    let _: &f64 = &k.lanes.react_secs;
    let _: &f64 = &k.lanes.call_secs;
    let _: &f64 = &k.lanes.see_m;
    let _: &f64 = &k.lanes.body_m;
    let _: &f64 = &k.lanes.dear;
    let _: &f64 = &k.lanes.ahead_m;
    let _: &f64 = &k.lanes.close_m;
    let _: &f64 = &k.lanes.hot_secs;
    let _: &f64 = &k.lanes.round_men;
    let _: &f64 = &k.lanes.think_secs;
    let _: &f64 = &k.lanes.round_max_m;
    let _: &f64 = &k.lanes.round_gap;
    let _: &f64 = &k.lanes.round_side_deg;
    let _: &f64 = &k.lanes.round_show_m;
    let _: &f64 = &k.lanes.shift_m;
    let _: &f64 = &k.lanes.stun_m;
    let _: &f64 = &k.lanes.bang_m;
    let _: &f64 = &k.lanes.push_secs;
    let _: &f64 = &k.lanes.push_m;
    let _: &f64 = &k.lanes.push_fast;
    let _: &f64 = &k.lanes.stun_gap;
    let _: &f64 = &k.lanes.moved_m;
    let _: &f64 = &k.lanes.moved_secs;
    let _: &f64 = &k.lanes.peek_quiet;
    let _: &f64 = &k.lanes.peek_gap;
    let _: &f64 = &k.lanes.peek_max;
    let _: &f64 = &k.lanes.peek_lost;
    let _: &f64 = &k.lanes.peek_reach_m;
    let _: &f64 = &k.lanes.peek_near_m;
    let _: &f64 = &k.lanes.peek_look;
    let _: &f64 = &k.lanes.peek_sure;
    let _: &f64 = &k.lanes.peek_keep;
    let _: &f64 = &k.lanes.peek_pace;
    let _: &f64 = &k.lanes.peek_lean;
    let _: &f64 = &k.lanes.pin_rounds;
    let _: &f64 = &k.lanes.pin_every;
    let _: &f64 = &k.lanes.pin_snap;
    let _: &f64 = &k.lanes.pin_gap;
    let _: &f64 = &k.lanes.pin_max;
    let _: &f64 = &k.lanes.pin_rest;
    let _: &f64 = &k.lanes.pin_spread;
    let _: &f64 = &k.lanes.pin_aim_deg;
    let _: &f64 = &k.lanes.pin_pace;
    let _: &f64 = &k.lanes.pin_show_m;
    let _: &f64 = &k.lanes.pin_min_m;
    let _: &f64 = &k.lanes.look_m;
    let _: &f64 = &k.lanes.frag_think;
    let _: &f64 = &k.lanes.frag_gap;
    let _: &f64 = &k.lanes.frag_look;
    let _: &f64 = &k.lanes.frag_step_m;
    let _: &f64 = &k.lanes.max;
    let _: &f64 = &k.gunhold.cover_m;
    let _: &f64 = &k.gunhold.near_m;
    let _: &f64 = &k.gunhold.rear_deg;
    let _: &f64 = &k.gunhold.keep_secs;
    let _: &f64 = &k.gunhold.peek_every;
    let _: &f64 = &k.gunhold.peek_secs;
    let _: &f64 = &k.gunhold.show_m;
    let _: &f64 = &k.gunhold.shy_s;
    let _: &f64 = &k.gunhold.dash_m;
    let _: &f64 = &k.gunhold.face_deg;
    let _: &f64 = &k.gunhold.flank_max_m;
    let _: &[f64; 2] = &k.gunhold.react;
    let _: &f64 = &k.hard.reach_m;
    let _: &f64 = &k.hard.stuns;
    let _: &f64 = &k.drilled.shaken;
    let _: &[f64; 2] = &k.squads.think;
    let _: &f64 = &k.squads.net_lag;
    let _: &f64 = &k.squads.help_men;
    let _: &f64 = &k.squads.flank_men;
    let _: &f64 = &k.squads.again_secs;
    let _: &f64 = &k.squads.settle_secs;
    let _: &f64 = &k.squads.quiet_secs;
    let _: &f64 = &k.squads.recall_secs;
    let _: &[f64; 2] = &k.squads.reinforce;
    let _: &f64 = &k.squads.ready_beat;
    let _: &f64 = &k.squads.fire_m;
    let _: &f64 = &k.squads.rear_m;
    let _: &f64 = &k.squads.soft_m;
    let _: &f64 = &k.squads.mouth_m;
    let _: &f64 = &k.squads.hold_back_m;
    let _: &f64 = &k.squads.hold_secs;
    let _: &bool = &k.squads.drill_net;
    let _: &f64 = &k.squads.net_k;
    let _: &f64 = &k.squads.reinforce_k;
    let _: &f64 = &k.squads.react_k;
    let _: &f64 = &k.squads.nerve_k;
    let _: &f64 = &k.squads.follow_m;
    let _: &bool = &k.squads.drill_own;
    let _: &f64 = &k.squads.own_up;
    let _: &bool = &k.squads.drill_back;
    let _: &f64 = &k.squads.back_beat;
    let _: &f64 = &k.squads.sweep_secs;
    let _: &f64 = &k.squads.sweep_men;
    let _: &f64 = &k.squads.sweep_beat;
    let _: &f64 = &k.squads.check_every;
    let _: &f64 = &k.squads.check_jitter;
    let _: &f64 = &k.squads.check_alert_k;
    let _: &f64 = &k.squads.check_ask;
    let _: &f64 = &k.squads.check_send;
    let _: &[f64; 2] = &k.squads.check_heard;
    let _: &f64 = &k.squads.check_hear_m;
    let _: &f64 = &k.squads.look_men;
    let _: &[f64; 2] = &k.squads.call_secs;
    let _: &[f64; 2] = &k.squads.call_cut_ask;
    let _: &f64 = &k.squads.net_every;
    let _: &[f64; 2] = &k.squads.net_beat;
    let _: &f64 = &k.squads.net_look_men;
    let _: &f64 = &k.squads.net_stops;
    let _: &f64 = &k.cues.rows;
    let _: &f64 = &k.cues.secs;
    let _: &f64 = &k.cues.min_show;
    let _: &f64 = &k.cues.per_class;
    let _: &f64 = &k.cues.dup_secs;
    let _: &f64 = &k.cues.squelch;
    let _: &f64 = &k.cues.man_secs;
    let _: &f64 = &k.cues.again_secs;
    let _: &f64 = &k.cues.still_secs;
    let _: &f64 = &k.cues.still_quiet;
    let _: &f64 = &k.cues.stay_secs;
    let _: &f64 = &k.cues.overheard_gap;
    let _: &f64 = &k.wx_sim.rain_k;
    let _: &f64 = &k.wx_sim.thunder_k;
    let _: &f64 = &k.wx_sim.thunder_secs;
    let _: &f64 = &k.wx_sim.thunder_loud;
    let _: &f64 = &k.wx_sim.roof_bed;
    let _: &bool = &k.air_sim.on;
    let _: &f64 = &k.air_sim.ground;
    let _: &f64 = &k.air_sim.min_top;
    let _: &f64 = &k.air_sim.max_foot;
    let _: &f64 = &k.air_sim.roof_share;
    let _: &f64 = &k.air_sim.glass_open;
    let _: &f64 = &k.air_sim.leaf_step;
    let _: &f64 = &k.air_sim.vent_out;
    let _: &f64 = &k.air_sim.gust_k;
    let _: &f64 = &k.air_sim.rate;
    let _: &f64 = &k.air_sim.sigma_min;
    let _: &f64 = &k.air_sim.sigma_max;
    let _: &f64 = &k.air_sim.puff_rate;
    let _: &f64 = &k.air_sim.settle;
    let _: &f64 = &k.laststand.moved_secs;
    let _: &f64 = &k.laststand.still_secs;
    let _: &f64 = &k.laststand.moved_gap;
    let _: &f64 = &k.laststand.few;
    let _: &f64 = &k.laststand.quiet;
    let _: &[f64; 2] = &k.laststand.first;
    let _: &f64 = &k.laststand.every;
    let _: &f64 = &k.laststand.again;
    let _: &[f64; 2] = &k.laststand.answer;
    let _: &[f64; 2] = &k.laststand.between;
    let _: &[f64; 2] = &k.laststand.listen;
    let _: &f64 = &k.laststand.max_m;
    let _: &f64 = &k.laststand.clear_m;
    let _: &f64 = &k.laststand.known_secs;
    let _: &[f64; 2] = &k.laststand.go;
    let _: &f64 = &k.laststand.legs;
    let _: &[f64; 2] = &k.laststand.dig;
    let _: &[f64; 2] = &k.laststand.thump;
    let _: &f64 = &k.laststand.thump_level;
    let _: &f64 = &k.stackroom.apart;
    let _: &f64 = &k.stackroom.squeeze;
    let _: &f64 = &k.stackroom.round_m;
    let _: &f64 = &k.stackroom.round_k;
    let _: &f64 = &k.stackroom.walk_m;
    let _: &f64 = &k.stackroom.next_walk_m;
    let _: &f64 = &k.stackroom.next_m;
    let _: &f64 = &k.stackroom.lane_back;
    let _: &f64 = &k.stackroom.view_ajar;
    let _: &f64 = &k.stackroom.view_cells;
    let _: &f64 = &k.stackroom.view_far;
    let _: &f64 = &k.stackroom.view_eye;
    let _: &f64 = &k.stackroom.view_step;
    let _: &f64 = &k.stackroom.view_past;
    let _: &f64 = &k.stackroom.view_off;
    let _: &f64 = &k.stackroom.view_apart;
    let _: &f64 = &k.stackroom.view_out;
    let _: &f64 = &k.stackroom.view_men;
    let _: &f64 = &k.stackroom.view_wall;
    let _: &f64 = &k.rearwatch.reach;
    let _: &f64 = &k.rearwatch.ajar;
    let _: &f64 = &k.rearwatch.grace;
    let _: &f64 = &k.rearwatch.short;
    let _: &f64 = &k.rearwatch.room_secs;
    let _: &f64 = &k.rearwatch.room_m;
    let _: &f64 = &k.stackline.ajar;
    let _: &f64 = &k.stackline.bins;
    let _: &f64 = &k.stackline.eye;
    let _: &f64 = &k.stackline.inset;
    let _: &f64 = &k.stackline.on_mark;
    let _: &bool = &k.sneak.on;
    let _: &f64 = &k.sneak.min_pace;
    let _: &f64 = &k.sneak.low_pace;
    let _: &f64 = &k.sneak.cross_pace;
    let _: &f64 = &k.sneak.step;
    let _: &f64 = &k.sneak.trail_m;
    let _: &f64 = &k.sneak.back;
    let _: &f64 = &k.sneak.ahead_m;
    let _: &f64 = &k.sneak.lit_at;
    let _: &f64 = &k.sneak.seen_rate;
    let _: &f64 = &k.sneak.cone_deg;
    let _: &f64 = &k.sneak.near_m;
    let _: &f64 = &k.sneak.cross_m;
    let _: &f64 = &k.sneak.patience;
    let _: &f64 = &k.sneak.cross_secs;
    let _: &f64 = &k.sneak.budget;
    let _: &f64 = &k.sneak.margin;
    let _: &f64 = &k.sneak.memory;
    let _: &f64 = &k.sneak.nag_secs;
    let _: &f64 = &k.sneak.cue_gap;
    let _: &f64 = &k.sneak.stand_m;
    let _: &f64 = &k.sneak.stand_near;
    let _: &f64 = &k.sneak.side_m;
    let _: &f64 = &k.sneak.side_stop;
    let _: &f64 = &k.sneak.door_m;
    let _: &f64 = &k.sneak.door_pad;
    let _: &f64 = &k.sneak.door_settle;
    let _: &f64 = &k.sneak.hide_gap;
    let _: &f64 = &k.sneak.noise_walk_m;
    let _: &f64 = &k.sneak.plan_secs;
    let _: &f64 = &k.sneak.skill;
    let _: &bool = &k.pairup.on;
    let _: &bool = &k.spotted.on;
    let _: &f64 = &k.spotted.leash_m;
    let _: &f64 = &k.spotted.slip_m;
    let _: &f64 = &k.spotted.lit_at;
    let _: &f64 = &k.spotted.slip_wait;
    let _: &f64 = &k.spotted.wary_m;
    let _: &f64 = &k.spotted.ear_m;
    let _: &f64 = &k.spotted.linger_secs;
    let _: &f64 = &k.spotted.brace_secs;
    let _: &f64 = &k.spotted.brace_m;
    let _: &f64 = &k.spotted.take_m;
    let _: &f64 = &k.spotted.take_reach;
    let _: &f64 = &k.spotted.take_secs;
    let _: &f64 = &k.spotted.fresh_secs;
    let _: &f64 = &k.spotted.caught_m;
    let _: &f64 = &k.spotted.last_secs;
    let _: &f64 = &k.spotted.step_m;
    let _: &f64 = &k.spotted.face_deg;
    let _: &f64 = &k.spotted.close_m;
    let _: &[f64; 3] = &k.nightbreach.sam_at;
    let _: &f64 = &k.nightbreach.sam_yaw;
    let _: &f64 = &k.nightbreach.squad;
    let _: &f64 = &k.roomclear.seed;
    let _: &f64 = &k.roomclear.way;
    let _: &f64 = &k.verdict.win_secs;
    let _: &f64 = &k.verdict.loss_secs;
    let _: &f64 = &k.unseen.pace_at;
    let _: &f64 = &k.unseen.linger;
    let _: &f64 = &k.unseen.min_pace;
    let _: &f64 = &k.unseen.low_pace;
    let _: &f64 = &k.unseen.gain;
    let _: &f64 = &k.unseen.cap;
    let _: &f64 = &k.unseen.wall_off;
    let _: &f64 = &k.unseen.wall_out;
    let _: &f64 = &k.unseen.wall_cross;
    let _: &f64 = &k.unseen.wall_end;
    let _: &f64 = &k.unseen.low_step;
    let _: &f64 = &k.unseen.walk_step;
    let _: &f64 = &k.unseen.cue_gap;
    let _: &f64 = &k.unseen.wall_secs;
    let _: &f64 = &k.seenwalk.lit_at;
    let _: &f64 = &k.seenwalk.dear;
    let _: &f64 = &k.seenwalk.long;
    let _: &f64 = &k.seenwalk.less;
    let _: &f64 = &k.seenwalk.eye_step;
    let _: &f64 = &k.seenwalk.eye_y;
    let _: &f64 = &k.seenwalk.margin;
    let _: &f64 = &k.holdfire.lift_secs;
    let _: &bool = &k.slots.on;
    let _: &bool = &k.slots.ok;
    let _: &f64 = &k.slots.table.v;
    let _: &bool = &k.bigslot.on;
    let _: &f64 = &k.bigslot.swings;
    let _: &f64 = &k.bigslot.swing;
    let _: &f64 = &k.bigslot.windup;
    let _: &f64 = &k.bigslot.draw;
    let _: &f64 = &k.bigslot.shells;
    let _: &f64 = &k.bigslot.equip;
    let _: &f64 = &k.bigslot.stow;
    let _: &f64 = &k.bigslot.reach;
    let _: &f64 = &k.bigslot.heft;
    let _: &f64 = &k.bigslot.equip_w;
    let _: &f64 = &k.bigslot.draw_w;
    let _: &f64 = &k.bigslot.stow_w;
    let _: &f64 = &k.bigslot.flow;
}
const _: fn(&Knobs) = typed;
