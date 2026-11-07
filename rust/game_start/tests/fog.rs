//! THE FOG OF WAR KEEPS TWO RECORDS. What the team knows (`g.fog_of_war`: who is seen, which doors, lamps and loose things are remembered, who is
//! left out) is the game's, and every machine of one game holds it the same. The picture's block (`g.fog_picture`: a lamp's kept glow, the list kept for a
//! viewer's draw with the fog off) is one machine's, made from the game every tick.
//! These tests hold the line between them: nothing in the game reads the picture's block.
use game_start::play::{Opts, Play};

const SEEDS: [u32; 3] = [5, 6, 7];
const LEVEL: &str = "office.json";
const LINK: &str = "?levelFile=office.json&roomclear&squad4";

/// the office's room clear under the fog, as a click on its row starts it
fn office(seed: u32) -> Play { Play::mission(LEVEL, LINK, &Opts { seed, ..Default::default() }) }

/// every part of the game's state, a word a part
fn words(p: &mut Play) -> Vec<u32> {
    let (mut w, mut out) = (game::io::W::new(), Vec::new());
    game::state_words::state_words(&mut p.g, &mut w, &mut out);
    out
}

/// what a host does to its game between two steps
#[derive(Clone, Copy, PartialEq)]
enum Host { Plain, Watching, PictureMadeAnew }

/// the state's words at the end of every frame of one game
fn played(seed: u32, host: Host, frames: usize) -> Vec<Vec<u32>> {
    let mut p = office(seed);
    (0..frames).map(|_| {
        // (a viewer's page raises the switch once; a host whose block is made anew raises it again each frame, as it would have to)
        if host != Host::Plain { p.g.fog_picture.viewer = true; }
        p.frame();
        let row = words(&mut p);
        if host == Host::PictureMadeAnew { p.g.fog_picture = game::fogofwar::FogPicture::blank(); }
        row
    }).collect()
}

/// the first frame at which two games differ, and the parts of the state that do
fn parting(a: &[Vec<u32>], b: &[Vec<u32>]) -> Option<(usize, String)> {
    let k = a.iter().zip(b).position(|(x, y)| x != y)?;
    let parts: Vec<&str> = game::state_words::STATE_NAMES.iter().zip(a[k].iter().zip(&b[k])).filter(|(_, (x, y))| x != y).map(|(n, _)| *n).collect();
    Some((k, parts.join(", ")))
}

/// THE RULE: the picture's block is made from the game and read by nothing in it. A game whose block is thrown away after every frame is, in
/// every part of its state, the game that kept it.
/// A FAILURE MEANS: a rule has come to read the picture's block (or a field that belongs to the team's record was put in the picture's), so two
/// machines of one game that show different seats would part: the lit fog's lamps are cut by distance from the seat a machine shows.
/// HOW FAR IT REACHES, tried by breaking the game three ways: the team's knowledge of a lamp made to wait on the block's `on` turns this red at
/// frame 1 (in `fog_of_war`). `fog_of_war_hidden` and `fog_of_war_ours_on` made to read the block do NOT: on a host with the dressing down nothing
/// that asks them in these four seconds leaves a mark on the state. So this holds the fog's own step and what a quiet start runs, not every reader.
#[test]
fn nothing_in_the_game_reads_the_pictures_block() {
    for s in SEEDS {
        let (kept, anew) = (played(s, Host::Plain, 240), played(s, Host::PictureMadeAnew, 240));
        if let Some((k, parts)) = parting(&kept, &anew) { panic!("seed {s}: with the picture's block made anew every frame, the game first differs at frame {k} in {parts}"); }
    }
}

/// THE RULE: a viewer watching (the list before the fog cuts it kept every frame) is no part of the game: the watched game is the unwatched one.
/// And the list IS kept for the viewer: it is this frame's, and it holds no fewer boxes than the frame's list after the fog's cut less the copies
/// the fog keeps of doors.
/// A FAILURE MEANS: keeping the viewer's list has come to change what the fog's own step does, or the list is no longer kept.
#[test]
fn a_viewer_watching_is_no_part_of_the_game() {
    for s in SEEDS {
        let (plain, watched) = (played(s, Host::Plain, 240), played(s, Host::Watching, 240));
        if let Some((k, parts)) = parting(&plain, &watched) { panic!("seed {s}: watched by a viewer, the game first differs at frame {k} in {parts}"); }

        let mut p = office(s);
        p.g.fog_picture.viewer = true;
        p.run(1.0);
        assert!(game::fogofwar::fog_of_war_active(&p.g), "seed {s}: the mission's link starts the fog of war");
        assert_eq!(p.g.fog_picture.raw_at, p.g.fog_of_war.frame_no, "seed {s}: the list kept for the viewer is not this frame's");
        assert!(!p.g.fog_picture.raw_list.is_empty(), "seed {s}: the list kept for the viewer is empty");
        drop(p);

        let mut p = office(s);
        p.run(1.0);
        assert_eq!(p.g.fog_picture.raw_at, -1.0, "seed {s}: with no viewer a list was kept all the same");
    }
}

/// THE RULE: what the fog's pass draws from is made of a game that stands (rust/game_draw `fog`) and is no state of it: on while the fog is, an eye for
/// each the game has, every lamp the team knows lit, and the same of the same game however often it is asked.
/// A FAILURE MEANS: a page draws no fog, or draws it from other eyes than the game sees by.
#[test]
fn the_fogs_section_says_what_the_game_has() {
    for s in SEEDS {
        let mut p = office(s);
        p.run(1.0);
        let mut d = game_draw::Drawn::default();
        d.take(&p.g, 1);
        let f = d.fog[0];
        assert_eq!((d.fog.len(), f.on), (1, 1), "seed {s}: a second in, the section does not say the fog is on");
        assert_eq!(f.count as usize, p.g.fog_of_war.eyes.len(), "seed {s}: the section's eyes are not the game's");
        assert_eq!([f.eyes[0], f.eyes[2]], [p.g.fog_of_war.eyes[0].at[0] as f32, p.g.fog_of_war.eyes[0].at[2] as f32], "seed {s}: the first eye is somewhere else");
        assert!(f.light_count > 0, "seed {s}: the office's lamps are lit and the lit fog has none");
        assert_eq!(f.thin_count as usize, p.g.engine.world.statics.len(), "seed {s}: the thin statics are of another list");
        let before = words(&mut p);
        d.take(&p.g, 1);
        assert_eq!(d.fog[0], f, "seed {s}: the same game drawn twice is drawn two ways");
        assert_eq!(before, words(&mut p), "seed {s}: drawing the game changed it");
        p.g.fog_of_war.on = false;
        d.take(&p.g, 1);
        assert_eq!((d.fog[0].on, d.fog[0].count), (0, 0), "seed {s}: the fog is off and the section says on");
    }
}
