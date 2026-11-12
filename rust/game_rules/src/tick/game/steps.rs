use crate::game::Game;
use crate::ids::*;
use crate::m;
use alloc::vec::Vec;

/// What every step of a tick is given.
pub struct Tick {
    pub dt: f64,
    /// the players' records, as they came
    pub cmds: Vec<crate::cmd::PlayerCmd>,
}

/// A step of the tick: its name, and what it does.
pub type Step = (&'static str, fn(&mut Game, &Tick));

pub enum Run {
    /// once a tick, with no seat's view open
    World(Step),
    /// once a seat, over that seat's view: all of them for one seat, then for the next
    Seats(&'static [Step]),
}
use Run::{Seats, World};

/// Who is told each step of a tick as it ends.
pub type Watch<'w> = &'w mut dyn FnMut(&mut Game, &'static str);

/// The hands of the seat whose view is open: its keys, its lens, its window.
fn hands(g: &Game) -> (InputId, CameraId, crate::host_hud::View) {
    let s = &g.tab.cmd_seat[g.seat];
    (s.input, s.cam, s.canvas)
}

/// The hands of the first seat: what a one-player part of the tick reads.
fn first(g: &Game) -> (InputId, CameraId) {
    let s = &g.tab.cmd_seat[g.seats[0]];
    (s.input, s.cam)
}

/// THE ORDER OF THESE ROWS IS THE GAME.
pub static STEPS: &[Run] = &[
    // (on every host at the same moment: this tick's rays meet the last tick's boxes and whatever was written since, and the rows of the frames gone by are free
    // again, after whoever showed the last frame has read it)
    World(("box world listed", |g, _| crate::scene_boxes::box_world_listed(g))),
    World(("quiet mission", |g, _| crate::quiet::quiet_mission_current(g))),
    World(("smoke", |g, t| crate::sense::smoke_step(g, t.dt))),
    World(("commands in", commands_in)),
    World(("rails' gate", |g, _| { let (input, _) = first(g); crate::rails::rails_gate(g, input); })),
    World(("opening", |g, _| { crate::opening::opening_start(g); })),
    World(("seats dealt", |g, _| if g.seats.len() > 1 { crate::seats::deal_seats(g); })),
    Seats(&[("lens", lens)]),
    World(("clock", |g, t| g.time += t.dt)),
    World(("sound paths", |g, _| { let now = g.time; crate::scene_collision::sound_paths_tick(g, now); })),
    World(("duties anew", |g, _| crate::belief::duties_anew(g))),
    Seats(&[
        ("aim", |g, _| { let (input, cam, canvas) = hands(g); crate::player::update_aim(g, input, cam, canvas); }),
        ("orders' keys", |g, t| { let (input, cam, canvas) = hands(g); crate::orders::squad_orders_input(g, t.dt, input, cam, canvas); }),
        ("wand order", |g, _| crate::wandorder::wand_order_frame(g)),
        ("squad throw", |g, _| crate::squadthrow::squad_throw_frame(g)),
        ("order rows", |g, _| crate::orders::rows::frame(g)),
        ("orders shown", |g, _| { let (input, cam, canvas) = hands(g); crate::orders::shown::keep(g, input, cam, canvas); }),
        ("his own ring", |g, t| { let (input, cam, canvas) = hands(g); crate::interact::update_interact(g, t.dt, input, Some(canvas), Some(cam)); }),
        ("aim settled", |g, t| crate::aim::track_player(g, t.dt)),
        ("threat", |g, _| crate::game::game_threat_tick(g)),
    ]),
    World(("suppression", |g, t| crate::suppression::update_suppression(g, t.dt))),
    Seats(&[
        ("player", |g, t| { let (input, cam, _) = hands(g); crate::player::update_player(g, t.dt, input, cam); }),
        ("player's look", |g, _| crate::samlook::sam_look_step(g)),
    ]),
    World(("sponge rounds", |g, t| if !g.rings.is_empty() { crate::launcher::update_rings(g, t.dt); })),
    Seats(&[("sticky cameras", |g, t| if !g.pl[g.player].cams.is_empty() { crate::stickycam::update_stickies(g, t.dt); })]),
    World(("grenades", |g, t| if !g.shells.is_empty() { crate::he40::update_shells(g, t.dt); })),
    World(("weapons dealt", |g, t| { g.search_probe_n = 0.0; crate::game::game_deal_weapons(g, Some(t.dt)); })),
    World(("director", |g, t| crate::director::director_tick(g, t.dt, None, None))),
    Seats(&[("pair's key", |g, _| { let (input, ..) = hands(g); crate::pairorders::pair_orders_input(g, input); })]),
    Seats(&[("charges' key", |g, _| {
        let (input, ..) = hands(g);
        crate::wallcharge::wall_charge_input(g, input);
        crate::doortools::door_tools_input(g, input);
        crate::wallcharge::wall_charge_input_after(g, input);
    })]),
    World(("mission's rules", |g, t| { let (input, cam) = first(g); crate::mission::mission_tick(g, t.dt, input, Some(cam)); })),
    World(("wall react", |g, _| crate::wallreact::wall_react_tick_2(g))),
    World(("entry points", |g, _| crate::entrypoints::entry_points_tick(g))),
    World(("cleared rooms", |g, _| crate::cleared::cleared_rooms_tick(g))),
    Seats(&[
        ("door tools", |g, _| crate::doortools::door_tools_tick(g)),
        ("wall charges", |g, _| crate::wallcharge::wall_charge_tick(g)),
    ]),
    Seats(&[("rear watch", |g, _| crate::rearwatch::rear_watch_tick(g))]),
    World(("cover", |g, _| { crate::cover::cover_tick(g); crate::cover::posts_tick(g); })),
    Seats(&[("silent", |g, _| crate::silent::silent_tick(g))]),
    World(("pairs", |g, t| crate::pairorders::pair_orders_tick(g, t.dt))),
    Seats(&[("spotted", |g, _| crate::spotted::spotted_tick(g))]),
    Seats(&[("squad orders", |g, _| crate::orders::squad_orders_tick(g))]),
    World(("their orders", |g, t| crate::group::groups_tick(g, t.dt))),
    World(("door paths", |g, t| crate::doorpath::door_path_tick(g, t.dt))),
    World(("power out", |g, t| crate::powerout::power_out_tick(g, t.dt))),
    World(("room fight", |g, t| crate::roomfight::room_fight_tick(g, t.dt))),
    Seats(&[("orders' reasons", |g, _| crate::showwhy::show_why_tick(g))]),
    World(("morale", |g, t| crate::morale::morale_tick(g, t.dt))),
    World(("flinch", |g, t| crate::flinch::flinch_tick(g, t.dt))),
    World(("runner", |g, t| crate::runner::runner_tick(g, t.dt))),
    World(("dark door", |g, t| crate::darkdoor::dark_door_tick(g, t.dt))),
    World(("net quiet", |g, t| crate::netquiet::net_quiet_tick(g, t.dt))),
    World(("lanes", |g, t| crate::lanes::lanes_tick(g, t.dt))),
    World(("dark foe", |g, t| crate::darkfoe::dark_foe_tick(g, t.dt))),
    World(("night vision", |g, _| crate::nvg::nvg_tick(g))),
    World(("tactics", |g, _| crate::tactics::tactics_tick(g))),
    World(("doze", |g, t| crate::doze::doze_tick(g, t.dt))),
    World(("close work", |g, t| crate::close::close_seat_tick(g, t.dt))),
    World(("needs", |g, t| crate::needs::needs_tick(g, t.dt))),
    World(("frag throws", |g, t| crate::frag::frag_throw_tick(g, t.dt))),
    World(("frags", |g, t| crate::frag::frag_tick(g, t.dt))),
    World(("guards", guards)),
    World(("picture", |g, t| crate::picture::picture_tick(g, t.dt))),
    World(("doors forced", |g, t| crate::doorforce::door_force_tick(g, t.dt))),
    World(("cover fire", |g, t| crate::coverfire::coverfire_tick(g, t.dt))),
    World(("civilians", |g, t| crate::civilians::update_civilians(g, t.dt))),
    World(("escalation", |g, t| crate::guards::update_escalation(g, t.dt))),
    World(("response team", |g, t| crate::qrf::update_qrf(g, t.dt))),
    World(("doors", |g, t| crate::game::game_update_doors(g, t.dt))),
    World(("lenses", |g, _| crate::lighting::lens_recheck(g))),
    World(("infrared", |g, _| crate::irlight::ir_light_tick(g))),
    World(("mission", |g, t| { let s = g.seats[0]; crate::seats::on_seat(g, Some(s), &mut |g| crate::mission::update_mission(g, t.dt)); })),
    World(("par", |g, _| crate::par::par_tick(g))),
    World(("lesson", |g, t| crate::lesson::lesson_tick(g, t.dt))),
    World(("rails", |g, _| crate::rails::rails_tick(g))),
    World(("props", |g, t| crate::game::game_update_props(g, t.dt))),
    World(("destruction", |g, t| crate::destruction::destruction_update(g, t.dt))),
    World(("air dust", |g, t| crate::air_dust::air_dust_update(g, t.dt))),
    World(("events forgotten", |g, _| { let now = g.time; g.events.retain(|e| now - e.time < 0.5); })),
    World(("light switches", |g, _| crate::lightswitch::light_switch_tick(g))),
    World(("fixtures", |g, t| crate::lighting::update_targets(g, t.dt))),
    World(("fire", |g, t| crate::fire::update_fire(g, t.dt))),
    World(("devices", |g, t| crate::devices::update_devices(g, t.dt))),
    World(("weather", |g, t| crate::weather::weather_update(g, t.dt))),
    World(("takedowns", |g, _| crate::takedown::takedown_tick(g))),
    Seats(&[("disarm", |g, t| crate::disarm::disarm_tick(g, t.dt))]),
    // (the soak's ask stands the rule's copy of the box world and may cast the sky's rays)
    World(("wetness", |g, t| crate::wet::update_wetness(g, t.dt))),
    // the dressing the tick itself reads: the wind pulls the moths, and the moths are boxes of the frame's list, which the fog of war's records count
    World(("wind", |g, t| if g.m.dress.dress.wind { crate::wind::wind_update(g, t.dt); })),
    World(("moths", |g, t| if g.m.dress.dress.moths { crate::moths::moths_update(g, t.dt); })),
    World(("items", |g, t| crate::combat::update_items(g, t.dt))),
    World(("glass", |g, _| crate::glass::update_glass(g))),
    World(("glass breaks", |g, _| crate::glassbreak::glass_break_update(g))),
    World(("shards", |g, t| crate::shards::glass_shards_update(g, t.dt))),
    World(("fog of war", |g, t| crate::fogofwar::fog_of_war_update(g, t.dt))),
    World(("men and boxes", |g, t| {
        let all = men(g);
        crate::game::game_step_fx_and_dyn_boxes(g, &all, t.dt);
        crate::game::game_draw_characters(g, &all);
    })),
    World(("smoke breaks", |g, t| crate::smokebreak::update_smoke_break(g, t.dt))),
    World(("light meter", |g, t| crate::game::game_light_meter(g, t.dt))),
    // (every seat's list, before anybody's orders are read: what a point or a cursor is on is asked of it, inside a tick and between two)
    Seats(&[("work things", |g, _| crate::workthings::work_things_keep(g))]),
    Seats(&[("seat's end", |g, t| {
        let pl = g.player;
        g.pl[pl].hit_flash = m::max(0.0, g.pl[pl].hit_flash - t.dt * 3.0);
        g.sets[g.seat].act = None;   // (this frame's verb has been read by everyone who reads it)
    })]),
    World(("squad's net", |g, _| if g.m.dress.dress.net { crate::squadnet::squad_net_tick(g); })),
    World(("cues", |g, _| {
        let news = core::mem::take(&mut g.fight.news);
        if g.m.dress.dress.cues { crate::cues::cues_tick(g, &news); }
    })),
    World(("team tally", |g, _| crate::teamtally::team_tally_tick(g))),
    World(("death recap", |g, _| crate::deathrecap::recap_tick(g))),
    World(("marks", |g, _| crate::render_decals::marks_settle(g))),
    // (with the first seat still standing, so the end it latches is the same on every machine)
    World(("verdict", |g, _| crate::verdict::verdict_tick(g))),
    // (last: every flash's envelope and its end. The next tick's light reads what it leaves)
    World(("flashes", |g, t| crate::render_lights::light_set_update(g, t.dt))),
];

/// Every row of `STEPS`, `after` told each as it ends.
pub(super) fn run_steps(g: &mut Game, tick: &Tick, after: Watch) {
    let seats = g.seats.clone();
    for run in STEPS {
        match run {
            World((name, f)) => { f(g, tick); after(g, name); }
            Seats(rows) => for &s in &seats {
                crate::seats::on_seat(g, Some(s), &mut |g| for (name, f) in rows.iter() { f(g, tick); after(g, name); });
            }
        }
    }
    debug_assert_eq!(crate::scene_collision::astray(g), None, "the box world is not the boxes as they stand");
}

/// A record from anyone made one an honest seat could have sent, before anything reads it.
fn commands_in(g: &mut Game, t: &Tick) {
    let seats = g.seats.clone();
    let mut checked = Vec::with_capacity(seats.len());
    for (&s, cmd) in seats.iter().zip(&t.cmds) {
        let near = if g.tab.cmd_seat[s].remote { let pl = crate::cmd::seat_man(g, s); Some(g.player_pos(pl)) } else { None };
        checked.push(crate::cmd::check_cmd(g, Some(cmd), near));
    }
    for (&s, cmd) in seats.iter().zip(&checked) { crate::cmd::seat_load(g, s, cmd); }
}

fn lens(g: &mut Game, _: &Tick) {
    let (_, cam, _) = hands(g);
    let vp = g.tab.camera[cam].view_proj;
    g.sets[g.seat].view_proj = Some(crate::game::GameViewProj::Float32Array(vp.to_vec()));
}

/// Each guard's own step. (A live index: a man's update may grow the list.)
fn guards(g: &mut Game, t: &Tick) {
    let mut i = 0;
    while i < g.guards.len() {
        let gd = g.guards[i];
        crate::seats::standing_for(g, gd, &mut |g| crate::guards::update_guard(g, gd, t.dt));
        i += 1;
    }
}

/// Everybody's body: the seats' men, the guards, the civilians last.
fn men(g: &mut Game) -> Vec<CharId> {
    let mut all: Vec<CharId> = Vec::new();
    for &s in &g.seats { all.push(g.pl[crate::cmd::seat_man(g, s)].ch); }
    for &gd in &g.guards { all.push(g.gd[gd].ch); }
    for gd in crate::civilians::civilians_shown(g) { all.push(g.gd[gd].ch); }
    all
}
