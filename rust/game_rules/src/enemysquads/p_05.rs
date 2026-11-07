//! src/game/enemysquads.ts lines 1536 to 1572
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

fn sign_of(g: &Game, m: GuardId) -> String { g.gd[m].callsign.clone() }
fn name_of(g: &Game, q: EnemysquadsSquadId) -> String { g.tab.enemysquads_squad[q].name.clone() }
fn door_name(g: &Game, d: DoorId) -> String { g.dr[d].def.name.clone() }
fn def_gd(g: &Game, d: RoomfightDefId) -> GuardId { g.tab.roomfight_def[d].gd }

/// `r3`: to three places; what is not a finite number is not given   (squadWaysRound)
fn r3(x: Option<f64>) -> Option<f64> {
    match x { Some(v) if v.is_finite() => Some(m::round(v * 1000.0) / 1000.0), _ => None }
}
/// the team heard through the walls anywhere in the room it has taken: a place every two metres of its floor, each a soft one as a noise is   (squadWaysRound's heard)
fn heard_in(g: &Game, heard_of: &mut JsMap<RoomfightRoomId, Vec<Known>>, t: RoomfightRoomId) -> Vec<Known> {
    if let Some(h) = heard_of.get(t) { return h.clone(); }
    let (cell, w) = (g.col.nav.cell, g.col.nav.w);
    let step = m::max(1.0, m::round(2.0 / cell));
    let mut h = Vec::new();
    for i in g.tab.roomfight_room[t].cells.iter() {
        let i = i.0;
        let ix = js::rem(i, w);
        let iz = (i - ix) / w;
        if js::rem(ix, step) == 0.0 && js::rem(iz, step) == 0.0 { h.push(Known { p: [(ix + 0.5) * cell, 0.0, (iz + 0.5) * cell], firm: false }); }
    }
    heard_of.set(t, h.clone());
    h
}

/// a static ask for a check: for every man at home of every squad that may leave its rooms, every other squad's room and door, would he find a way round. His position is moved and put back; so is the search's memo   (enemysquads.ts squadWaysRound)
pub fn squad_ways_round(g: &mut crate::game::Game) -> Vec<crate::enemysquads::SquadWaysRoundItem> {
    let mut out = Vec::new();
    let squads = match g.m.enemysquads.states.as_ref() { Some(st) => st.squads.clone(), None => return out };
    let keep = g.m.enemysquads.ctx.take();
    let mut heard_of: JsMap<RoomfightRoomId, Vec<Known>> = JsMap::new();
    for &q in squads.iter() {
        if g.tab.enemysquads_squad[q].def.may_leave == Some(false) { continue; }
        for d in home(g, q) {
            let gd = def_gd(g, d);
            let c = g.gd[gd].ch;
            let post = g.ch[c].pos;
            let mut from: Vec<(String, V3)> = vec![(String::from("his post"), post)];
            let entries = match g.tab.roomfight_def[d].room { Some(r) => g.tab.roomfight_room[r].entries.clone(), None => Vec::new() };
            for en in entries.iter() {
                let stride: V3 = [en.at[0] - en.out[0], 0.0, en.at[2] - en.out[1]];
                if !crate::scene_collision::nav_grid_is_blocked(g, stride[0], stride[2]) { from.push((door_name(g, en.door), stride)); }
            }
            for (from_name, p) in from.iter() {
                g.ch[c].pos[0] = p[0];
                g.ch[c].pos[2] = p[2];
                for &x in squads.iter() {
                    if x == q { continue; }
                    let rooms = g.tab.enemysquads_squad[x].rooms.clone();
                    for room in rooms.iter() {
                        let at: V3 = [(room.bounds.x0 + room.bounds.x1) / 2.0, 0.0, (room.bounds.z0 + room.bounds.z1) / 2.0];
                        let t = crate::roomfight::room_fight_room_at(g, at);
                        let doors = match t { Some(t) => g.tab.roomfight_room[t].entries.clone(), None => Vec::new() };
                        for e in doors.iter() {
                            for team in [false, true] {
                                for hold in [false, true] {
                                    let mut says = WayRound2Says { limit: None, best: None };
                                    let known: Vec<Known> = if team {
                                        let mut k = vec![
                                            Known { p: [e.at[0] + e.out[0], 0.0, e.at[2] + e.out[1]], firm: true },
                                            Known { p: [e.at[0] - e.out[0] * 3.0, 0.0, e.at[2] - e.out[1] * 3.0], firm: true },
                                        ];
                                        k.extend(heard_in(g, &mut heard_of, t.unwrap()));
                                        k
                                    } else { Vec::new() };
                                    g.m.enemysquads.ctx = None;   // (the memo's key counts the known places, it does not tell them apart)
                                    let back = if hold { g.k.squads.hold_back_m } else { 0.0 };
                                    let w = way_round2(g, d, at, &known, &[e.door], &[], &[], Some(back), Some(&mut says));
                                    let way = w.as_ref().map(|w| match w.door { Some(x) => door_name(g, x), None => String::from("open floor") });
                                    // (`r3(w.L)!` of a way with no finite length has no number in the TypeScript: NaN here)
                                    let l = match &w { Some(w) => r3(Some(w.l)).unwrap_or(f64::NAN), None => 0.0 };
                                    out.push(SquadWaysRoundItem { squad: name_of(g, q), man: sign_of(g, gd), from: from_name.clone(), room: room.name.clone(), door: door_name(g, e.door), hold, team, way, l, limit: r3(says.limit), best: r3(says.best) });
                                }
                            }
                        }
                    }
                }
            }
            g.ch[c].pos[0] = post[0];
            g.ch[c].pos[2] = post[2];
        }
    }
    g.m.enemysquads.ctx = keep;
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    /// the TypeScript's own bits, under bun with EXACT on: `Math.round(x * 1000) / 1000`
    #[test]
    fn r3_rounds_to_the_typescripts_bits() {
        for &(x, want) in &[
            (0x0000000000000000u64, 0x0000000000000000u64),
            (0x3ff3c083126e978d, 0x3ff3c28f5c28f5c3),
            (0x3ff0020c49ba5e35, 0x3ff004189374bc6a),
            (0x4004000000000000, 0x4004000000000000),
            (0x40300020c2ee26ba, 0x4030000000000000),
            (0xbff0020c49ba5e35, 0xbff0000000000000),
            (0xbf3a36e2eb1c432d, 0x8000000000000000),
            (0x40934a456d5cfaad, 0x40934a45a1cac083),
            (0x3fd3333333333334, 0x3fd3333333333333),
            (0x3e112e0be826d695, 0x0000000000000000),
        ] {
            assert_eq!(r3(Some(f64::from_bits(x))).unwrap().to_bits(), want);
        }
    }
    #[test]
    fn r3_gives_nothing_for_what_is_not_a_finite_number() {
        assert_eq!(r3(None), None);
        assert_eq!(r3(Some(f64::INFINITY)), None);
        assert_eq!(r3(Some(f64::NAN)), None);
    }
}
