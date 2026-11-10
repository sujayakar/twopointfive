//! WHAT A SQUAD THAT FOLLOWS LOOKS AT IN A ROOM (rust/game_rules/src/teammates/cleared/looked.rs, dealt by cover/): the floor nobody of the team has looked over.
use game::orders::Who;
use game_start::play::{dist, Opts, Play};

const SEEDS: [u32; 3] = [1, 2, 3];
/// a level, a door, which way is out of the room by it, and where in the room Sam stands
const ROOMS: [(&str, &str, [f64; 2], [f64; 2]); 2] = [("barracks", "n4b", [0.0, 1.0], [31.75, 6.75]), ("hall", "bunks", [0.0, 1.0], [14.75, 10.75])];
const HALF: f64 = 45.0;

fn off(a: f64, b: f64) -> f64 { ((a - b + std::f64::consts::PI).rem_euclid(std::f64::consts::TAU) - std::f64::consts::PI).abs().to_degrees() }

/// Square metres of the room's floor that neither Sam nor a man of the squad has had in his eyes, `secs` after the squad, in file at the door, is left to follow Sam into it. Nobody else is in or near.
fn unseen(seed: u32, (level, door, out, at): (&str, &str, [f64; 2], [f64; 2]), secs: usize) -> f64 {
    let mut p = Play::mission(&format!("{level}.json"), &format!("?levelFile={level}.json&roomclear&squad4"), &Opts { seed, ..Default::default() });
    p.sam_unhurt();
    p.sam_unseen();
    p.run(0.3);
    let r = p.g.fight.rooms.iter().copied().find(|&r| { let room = &p.g.tab.roomfight_room[r]; let (w, cs) = (p.g.col.nav.w, p.g.col.nav.cell); room.cells.has(game::Num((at[1] / cs).floor() * w + (at[0] / cs).floor())) }).expect("the room");
    let (w, cs) = (p.g.col.nav.w, p.g.col.nav.cell);
    let cells: Vec<[f64; 2]> = p.g.tab.roomfight_room[r].cells.iter().map(|c| [((c.0 % w) + 0.5) * cs, ((c.0 / w).floor() + 0.5) * cs]).collect();
    let d = game::doors::doors_by_name(&p.g, door).expect("the door");
    let c = p.g.dr[d].frame_centre;
    let off_door = |k: f64| [c[0] + out[0] * k, c[2] + out[1] * k];
    let far = p.g.guards.iter().copied().filter(|&m| p.g.gd[m].team == game::teams::Team::Hostile).max_by(|&x, &y| dist(p.at(x), at).partial_cmp(&dist(p.at(y), at)).unwrap()).unwrap();
    p.drop_all_but(&[far]);
    p.run(1.0);
    let men = p.mates();
    p.sam_put(off_door(4.2));
    for (i, &m) in men.iter().enumerate() { p.man_stands(m, off_door(1.3 + 0.75 * i as f64)); }
    p.order("fallIn", p.sam_at(), Who::All);
    p.run(0.5);
    for (i, &m) in men.iter().enumerate() { p.man_stands(m, off_door(1.3 + 0.75 * i as f64)); }
    p.sam_put(at);
    let mut looked = vec![false; cells.len()];
    for f in 0..secs * 60 {
        p.frame();
        if f % 4 != 0 { continue; }
        let mut eyes: Vec<_> = men.iter().filter(|&&m| p.up(m)).map(|&m| p.g.gd[m].ch).collect();
        eyes.push(p.g.pl[p.g.player].ch);
        for ch in eyes {
            let (q, g) = (p.g.ch[ch].pos, game::fogofwar::gaze_of(&p.g, ch));
            for (i, c) in cells.iter().enumerate() {
                if looked[i] { continue; }
                let far = dist(*c, [q[0], q[2]]);
                if far > 14.0 || (far > 1.2 && off((c[0] - q[0]).atan2(c[1] - q[2]), g[0].atan2(g[2])) > HALF) { continue; }
                looked[i] = !game::scene_collision::segment_blocked(&mut p.g, [q[0], 1.5, q[2]], [c[0], 1.0, c[1]], None, None, None);
            }
        }
    }
    looked.iter().filter(|x| !**x).count() as f64 * cs * cs
}

/// THE RULE: a squad that follows Sam into a room looks over it: a man with nothing better to watch turns to the floor nobody has looked at.
/// A FAILURE MEANS: the men stand in the room beside Sam with a corner of it at their backs that nobody has ever looked into (before: 4.8 and 2.5 to 3 square metres, for as long as they stood there).
#[test]
fn a_squad_that_follows_looks_into_the_corners() {
    for s in SEEDS {
        for room in ROOMS {
            let m2 = unseen(s, room, 10);
            assert!(m2 <= 1.5, "seed {s} {} {}: {m2:.1} square metres of the room nobody has looked at", room.0, room.1);
        }
    }
}

/// Two ways into the block's rooms from its hall, where the doors of other rooms stand all round: the door, and where in the room they are sent
const WAYS_IN: [(&str, &str, [f64; 2], [f64; 2]); 2] = [("block", "m_s", [0.0, 1.0], [14.0, 6.0]), ("block", "b_s", [0.0, 1.0], [30.0, 6.0])];

/// THE RULE: a man whose walk takes him into a room nobody has looked over has his gun where he goes, whatever opening was his.
/// A FAILURE MEANS: men of a squad sent into a room come through the frame sideways or backwards, their guns on the doors of other rooms (before: the second, third and fourth
/// 35 to 151 degrees off their way, a dozen square metres of the room not looked over).
#[test]
fn a_man_who_walks_into_a_room_looks_where_he_goes() {
    let mut went = 0;
    for s in SEEDS {
        for (level, door, out, at) in WAYS_IN {
            let mut p = Play::mission(&format!("{level}.json"), &format!("?levelFile={level}.json&roomclear&squad4"), &Opts { seed: s, ..Default::default() });
            p.sam_unhurt();
            p.sam_unseen();
            p.run(0.3);
            let d = game::doors::doors_by_name(&p.g, door).expect("the door");
            let c = p.g.dr[d].frame_centre;
            let far = p.g.guards.iter().copied().filter(|&m| p.g.gd[m].team == game::teams::Team::Hostile).max_by(|&x, &y| dist(p.at(x), at).partial_cmp(&dist(p.at(y), at)).unwrap()).unwrap();
            p.drop_all_but(&[far]);
            p.run(1.0);
            let men = p.mates();
            p.sam_put([c[0] - 4.0, c[2] + 1.6]);
            for (i, &m) in men.iter().enumerate() { p.man_stands(m, [c[0] - 1.5 + 1.0 * i as f64, c[2] + 1.8]); }
            p.order("fallIn", p.sam_at(), Who::All);
            p.run(1.5);
            for (i, &m) in men.iter().enumerate() { p.man_stands(m, [c[0] - 1.5 + 1.0 * i as f64, c[2] + 1.8]); }
            p.order("move", at, Who::All);
            let mut was: Vec<f64> = men.iter().map(|&m| (p.at(m)[1] - c[2]) * out[1]).collect();
            for _ in 0..12 * 60 {
                p.frame();
                for (i, &m) in men.iter().enumerate() {
                    let now = (p.at(m)[1] - c[2]) * out[1];
                    if was[i] > 0.0 && now <= 0.0 && (p.at(m)[0] - c[0]).abs() < 1.0 {
                        let ch = p.g.gd[m].ch;
                        let o = off(p.g.ch[ch].aim_yaw, (-out[0]).atan2(-out[1]));
                        let (w, cs) = (p.g.col.nav.w as usize, p.g.col.nav.cell);
                        let rooms = p.g.cleared.room_of_cell.clone().unwrap();
                        let room = rooms[(at[1] / cs) as usize * w + (at[0] / cs) as usize];
                        let unlooked = (0..rooms.len()).filter(|&k| rooms[k] == room && p.g.col.nav.blocked[k] == 0 && p.g.cleared.looked[k] == 0 && dist([((k % w) as f64 + 0.5) * cs, ((k / w) as f64 + 0.5) * cs], p.at(m)) < 11.0).count();
                        if unlooked as f64 * cs * cs < 4.0 { was[i] = now; continue; }
                        assert!(o < 45.0, "seed {s} {door}: {} comes in with his gun {o:.0} degrees off his way, {unlooked} cells of the room not looked over", p.g.gd[m].callsign);
                        went += 1;
                    }
                    was[i] = now;
                }
            }
        }
    }
    assert!(went >= 12, "only {went} of them went in");
}
