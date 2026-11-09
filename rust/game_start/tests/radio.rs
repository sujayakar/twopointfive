//! WHAT IS SAID, AS A HUD IS TOLD IT (`view::tick_view`: `radio`, `bubbles`, `place`, `me`): the page reads these and words nothing of its own.
use game::dialogue::PlaceRect;
use game::game::{DownBy, GuardState};
use game::ids::*;
use game::view::{BubblePlace, BubbleTone, BubbleView, RadioKind};
use game_start::play::{Opts, Play};

const SEEDS: [u32; 3] = [1, 2, 3];

fn line_of(v: &game::view::TickView, man: f64) -> Option<BubbleView> { v.bubbles.iter().find(|b| b.man == Some(man)).cloned() }
fn id_of(p: &Play, gd: GuardId) -> f64 { p.g.ch[p.g.gd[gd].ch].id }
fn says(p: &mut Play, gd: GuardId, text: &str) { game::guards::say(&mut p.g, gd, text, None, Some(true), None); }

/// A FAILURE MEANS: a seat's HUD is told a line it should not show, or not told one it should. A man the fog hides has no line over him, unless he is the seat's own, whose line
/// goes to the edge of the view; a line that has had its time is gone; a man who has gone down has no line, and his squad's call of it stands where he fell.
#[test]
fn a_line_on_the_floor_is_told_as_the_scene_says_it() {
    for seed in SEEDS {
        let mut p = Play::mission("office.json", "?levelFile=office.json&roomclear&squad4", &Opts { seed, ..Default::default() });
        p.g.m.dress.dress.net = true;   // (a machine that shows the game ticks the squad's net: this host's dressing is down)
        for _ in 0..30 { p.frame(); }
        let mate = p.red()[0];
        // the seat's own man: his words and whose they are
        says(&mut p, mate, "with you.");
        let hidden = |p: &Play, gd| game::fogofwar::fog_of_war_hidden(&p.g, gd);
        let b = line_of(&p.view(), id_of(&p, mate)).unwrap_or_else(|| panic!("seed {}: a man of the seat's own said something and the HUD was not told", seed));
        assert_eq!(b.text, "with you.", "seed {}", seed);
        assert_eq!(b.edge, format!("{}: with you.", p.g.gd[mate].callsign), "seed {}", seed);
        assert_eq!(b.place, if hidden(&p, mate) { BubblePlace::Edge } else { BubblePlace::OverOrEdge }, "seed {}", seed);
        assert!(b.age == 0.0 && b.dur > 0.0, "seed {}: {:?}", seed, b);
        // the fog over him: at the edge, wherever he stands
        let ch = p.g.gd[mate].ch;
        let was = p.g.fog_of_war.hidden_chars.clone();
        p.g.fog_of_war.hidden_chars.add(ch);
        assert_eq!(line_of(&p.view(), id_of(&p, mate)).map(|b| b.place), Some(BubblePlace::Edge), "seed {}", seed);
        p.g.fog_of_war.hidden_chars = was;
        // CONTROL: the man beside him said nothing
        let quiet = p.red().iter().copied().find(|&m| m != mate).unwrap();
        assert!(line_of(&p.view(), id_of(&p, quiet)).is_none(), "seed {}: a man who said nothing has a line", seed);
        // a line that has had its time
        let dur = b.dur;
        let said = p.g.time;
        p.until(dur + 2.0, |p| p.g.time - said > dur).unwrap_or_else(|| panic!("seed {}: the clock stands", seed));
        assert!(line_of(&p.view(), id_of(&p, mate)).is_none(), "seed {}: the line outlived its time", seed);
        // a defender: told over him where the team sees him, and not at all where the fog hides him
        let foes = p.hostiles();
        for &h in foes.iter() { says(&mut p, h, "over here!"); }
        let v = p.view();
        let (mut seen, mut unseen) = (0, 0);
        for &h in foes.iter() {
            match (line_of(&v, id_of(&p, h)), hidden(&p, h)) {
                (Some(b), false) => { assert_eq!(b.place, BubblePlace::Over, "seed {}", seed); seen += 1; }
                (None, true) => unseen += 1,
                (b, hid) => panic!("seed {}: a defender with the fog hiding him {}: told {:?}", seed, hid, b),
            }
        }
        assert!(unseen > 0, "seed {}: the fog hid nobody ({} seen), so this proved nothing", seed, seen);
        // a man of the seat's own goes down: his line goes, and the call of it stands where he fell
        says(&mut p, mate, "moving.");
        let at = p.g.ch[p.g.gd[mate].ch].pos;
        p.falls(mate, DownBy::Guard);
        p.frame();
        let v = p.view();
        assert!(line_of(&v, id_of(&p, mate)).is_none(), "seed {}: a dead man has a line", seed);
        let call = v.bubbles.iter().find(|b| b.man.is_none()).unwrap_or_else(|| panic!("seed {}: a man of the team went down and nothing calls it: {:?}", seed, v.bubbles));
        assert!(call.tone == BubbleTone::Alert && call.place == BubblePlace::OverOrEdge && call.edge == call.text, "seed {}: {:?}", seed, call);
        assert!((call.at[0] - at[0]).abs() < 1.0 && (call.at[2] - at[2]).abs() < 1.0, "seed {}: called from {:?}, he fell at {:?}", seed, call.at, at);
    }
}

/// A FAILURE MEANS: a bubble's colour says the wrong thing: a man of your own wears the wary yellow that means a man who may find you out, or a defender's does not change with what he is doing.
#[test]
fn a_line_sounds_as_the_man_is() {
    for seed in SEEDS {
        let mut p = Play::level("kit-yard.json", true, &Opts { seed, ..Default::default() });
        p.frame();
        let mate = p.red()[0];
        let foe = p.hostiles()[0];
        p.g.fog_of_war.hidden_chars = Default::default();   // (nobody hidden: a tone is what is asked here)
        for gd in [mate, foe] { says(&mut p, gd, "what?"); }
        let tone = |p: &mut Play, gd: GuardId, s: GuardState| { p.g.gd[gd].state = s; let id = id_of(p, gd); line_of(&p.view(), id).unwrap_or_else(|| panic!("seed {}: not told", seed)).tone };
        for (s, own, theirs) in [(GuardState::Patrol, BubbleTone::Calm, BubbleTone::Calm), (GuardState::Suspicious, BubbleTone::Calm, BubbleTone::Sus), (GuardState::Search, BubbleTone::Calm, BubbleTone::Sus), (GuardState::Alert, BubbleTone::Alert, BubbleTone::Alert)] {
            assert_eq!(tone(&mut p, mate, s), own, "seed {}: his own man, {:?}", seed, s);
            assert_eq!(tone(&mut p, foe, s), theirs, "seed {}: a defender, {:?}", seed, s);
        }
        drop(p);
        // a civilian by his own state
        let mut q = Play::level("office.json", false, &Opts { seed, ..Default::default() });
        q.frame();
        let c = *q.g.civilians.first().unwrap_or_else(|| panic!("seed {}: the office has no civilian", seed));
        q.g.fog_of_war.hidden_chars = Default::default();
        says(&mut q, c, "oh!");
        let id = id_of(&q, c);
        for (st, want) in [(game::civilians::CivState::Work, BubbleTone::Calm), (game::civilians::CivState::Cower, BubbleTone::Sus), (game::civilians::CivState::Flee, BubbleTone::Alert)] {
            let cv = q.g.gd[c].civ.unwrap();
            q.g.cv[cv].state = st;
            assert_eq!(line_of(&q.view(), id).map(|b| b.tone), Some(want), "seed {}: a civilian {:?}", seed, st);
        }
        let cv = q.g.gd[c].civ.unwrap();
        q.g.cv[cv].state = game::civilians::CivState::Gone;
        assert!(line_of(&q.view(), id).is_none(), "seed {}: a civilian who has left the level still speaks", seed);
    }
}

/// A FAILURE MEANS: the radio strip is told something else than the game's rows: a line that never went on a net, a row past its time, or a shout where a row of the net wants the place.
#[test]
fn the_strip_is_told_the_rows_alive_and_a_shout_in_a_free_place() {
    for seed in SEEDS {
        let mut p = Play::level("kit-yard.json", true, &Opts { seed, ..Default::default() });
        p.frame();
        game::game::game_msg(&mut p.g, "radio: control to all");
        let v = p.view();
        assert_eq!(v.radio.len(), 1, "seed {}: {:?}", seed, v.radio);
        assert!(v.radio[0].kind == RadioKind::Net && v.radio[0].text == "control to all" && v.radio[0].age == 0.0, "seed {}: {:?}", seed, v.radio);
        // CONTROL: a line of the log that went on no net
        game::game::game_msg(&mut p.g, "guards reset");
        assert_eq!(p.view().radio.len(), 1, "seed {}", seed);
        // a shout in a free place, oldest first
        let t = p.g.time;
        p.g.cues.heard_row = Some(game::cues::CuesHeardRow { t, text: String::from("find them!") });
        let v = p.view();
        assert_eq!(v.radio.iter().map(|r| r.kind).collect::<Vec<_>>(), [RadioKind::Net, RadioKind::Heard], "seed {}", seed);
        assert_eq!(v.radio[1].text, "find them!", "seed {}", seed);
        // the strip full: no place for it
        let most = p.g.k.cues.rows as usize;
        for i in 0..most { game::game::game_msg(&mut p.g, &format!("radio: row {}", i)); }
        let v = p.view();
        assert!(v.radio.len() == most && v.radio.iter().all(|r| r.kind == RadioKind::Net), "seed {}: {:?}", seed, v.radio);
        // past its time
        let (secs, said) = (p.g.k.cues.secs, p.g.time);
        p.until(secs + 2.0, |p| p.g.time - said > secs).unwrap_or_else(|| panic!("seed {}: the clock stands", seed));
        assert!(p.view().radio.iter().all(|r| r.text != "row 0"), "seed {}: a row outlived its time", seed);
    }
}

/// A FAILURE MEANS: the strip no longer ranks: a line drawn twice, a class that has just spoken talks over the net, a worse line takes a better row's place, or one class holds the whole strip.
#[test]
fn the_strip_gives_a_row_to_one_line_ranked_and_rate_limited() {
    for seed in SEEDS {
        let mut p = Play::level("kit-yard.json", true, &Opts { seed, ..Default::default() });
        p.frame();
        let foes = p.hostiles();
        let name = |p: &Play, i: usize| p.g.gd[foes[i]].callsign.clone();
        let said = |p: &mut Play, i: usize, text: &str| game::cues::cues_heard(&mut p.g, foes[i], text);
        let rows = |p: &Play| p.view().radio.iter().map(|r| r.text.clone()).collect::<Vec<_>>();
        // CONTROL: the team's own net is not put on the strip
        let mate = p.red()[0];
        game::cues::cues_heard(&mut p.g, mate, "room clear");
        assert!(rows(&p).is_empty(), "seed {}: {:?}", seed, rows(&p));
        // a defender's line, under his callsign
        said(&mut p, 0, "contact — the mess!");
        assert_eq!(rows(&p), [format!("{}: contact — the mess!", name(&p, 0))], "seed {}", seed);
        // the same words from another man: not twice
        p.g.time += 0.5;
        said(&mut p, 1, "contact — the mess!");
        assert_eq!((rows(&p).len(), p.g.cues.stats.dup), (1, 1.0), "seed {}", seed);
        // two rows up: a class that has just spoken waits
        said(&mut p, 1, "shots fired! on me!");
        said(&mut p, 2, "that was a shot — find him!");
        assert_eq!((rows(&p).len(), p.g.cues.stats.gap), (2, 1.0), "seed {}: {:?}", seed, rows(&p));
        // a full strip: a better class takes the place of the worst, a worse one is held back
        said(&mut p, 3, "Bravo: two going round — keep them there!");
        said(&mut p, 4, "heard. they'll find this room ready.");
        let full = rows(&p);
        assert!(full.len() == 3 && full.iter().any(|r| r.ends_with("they'll find this room ready.")) && !full.iter().any(|r| r.ends_with("contact — the mess!")), "seed {}: {:?}", seed, full);
        said(&mut p, 5, "they're still out there — holding the mess");
        assert!(rows(&p) == full && p.g.cues.stats.full == 1.0, "seed {}: {:?}", seed, rows(&p));
    }
}

/// A FAILURE MEANS: in a game of two a HUD is told about the other player's men, his line, or the room he stands in.
#[test]
fn in_a_game_of_two_each_hud_is_told_its_own_men_its_own_line_and_its_own_room() {
    for seed in SEEDS {
        let mut p = Play::level("office.json", true, &Opts { seed, seats: 2, ..Default::default() });
        p.frame();
        // CONTROL: the rule at the place the seat's man stands is the room's own: two men side by side are told one room
        assert_eq!(p.view_of(0).place, p.view_of(1).place, "seed {}: side by side", seed);
        let names: Vec<PlaceRect> = game::dialogue::place_rects(&p.g);
        let mid = |r: &PlaceRect| [(r.x0 + r.x1) / 2.0, (r.z0 + r.z1) / 2.0];
        p.seat_stands(0, mid(&names[0]));
        let here = p.view_of(0).place;
        let far = names.iter().map(mid).find(|&at| { p.seat_stands(1, at); p.view_of(1).place != here }).unwrap_or_else(|| panic!("seed {}: the level has one room", seed));
        p.seat_stands(1, far);
        let (a, b) = (p.view_of(0), p.view_of(1));
        assert_ne!(a.place, b.place, "seed {}: in two rooms each is told his own", seed);
        assert_ne!(a.me, b.me, "seed {}", seed);
        // the second seat's own line reaches the second seat's HUD alone, in his own tone
        let pl = p.g.seats[1];
        let man = game::seats::seat(&p.g, pl).player;
        let time = p.g.time;
        p.g.pl[man].bubble = Some(game::game::PlayerBubble { text: String::from("who sent you?"), t: time, dur: 3.0, radio: false });
        let own = |v: &game::view::TickView| v.bubbles.iter().find(|b| b.tone == BubbleTone::Own).map(|b| (b.text.clone(), b.man));
        assert_eq!(own(&p.view_of(1)), Some((String::from("who sent you?"), Some(b.me))), "seed {}: the second seat", seed);
        assert_eq!(own(&p.view_of(0)), None, "seed {}: the first seat", seed);
        // a teammate of the second seat's: his to find at the edge, and to the first only a man on the floor
        let theirs = p.g.guards.iter().copied().find(|&gd| game::seats::in_seat(&p.g, pl, gd) && p.g.gd[gd].team == game::teams::Team::Operative && p.g.gd[gd].unit.is_some());
        let m = theirs.unwrap_or_else(|| panic!("seed {}: the second seat has no man of its own", seed));
        says(&mut p, m, "on you.");
        let id = id_of(&p, m);
        let (first, second) = (p.view_of(0), p.view_of(1));
        let sec = line_of(&second, id).unwrap_or_else(|| panic!("seed {}: the second seat's own man said it and its HUD was not told", seed));
        assert!(sec.place != BubblePlace::Over, "seed {}: {:?}", seed, sec);
        if let Some(fi) = line_of(&first, id) { assert_eq!(fi.place, BubblePlace::Over, "seed {}: {:?}", seed, fi); }
    }
}
