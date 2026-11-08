//! WHAT A PLAYER OR A PANEL DOES THAT IS NOT INPUT, as one typed list: a host hands the list for a seat before a step, and a replay or another machine is handed the
//! same bytes. A command is declared here once (`commands!`); the page's writer of it is made from the declaration by `bun run tools/draw/gen.ts`.
//!
//! The bytes: a command's number (u32), then its fields in order, little end first: u32, i32, f64, or text as its length (u32) and its bytes.
use game::game::Game;
use game::savestate::control as saves;

/// bytes being read
pub struct Bytes<'a> { b: &'a [u8], at: usize }
impl<'a> Bytes<'a> {
    pub fn of(b: &'a [u8]) -> Bytes<'a> { Bytes { b, at: 0 } }
    fn take(&mut self, n: usize) -> &'a [u8] { let r = &self.b[self.at..self.at + n]; self.at += n; r }
}
pub trait Field<'a>: Sized { fn read(b: &mut Bytes<'a>) -> Self; }
impl<'a> Field<'a> for u32 { fn read(b: &mut Bytes<'a>) -> u32 { u32::from_le_bytes(b.take(4).try_into().unwrap()) } }
impl<'a> Field<'a> for i32 { fn read(b: &mut Bytes<'a>) -> i32 { i32::from_le_bytes(b.take(4).try_into().unwrap()) } }
impl<'a> Field<'a> for f64 { fn read(b: &mut Bytes<'a>) -> f64 { f64::from_le_bytes(b.take(8).try_into().unwrap()) } }
impl<'a> Field<'a> for &'a str { fn read(b: &mut Bytes<'a>) -> &'a str { let n = u32::read(b) as usize; core::str::from_utf8(b.take(n)).unwrap_or("") } }

macro_rules! commands {
    ($($(#[$m:meta])* $i:literal $name:ident { $($f:ident: $t:ty),* } = $do:expr;)*) => {
        /// the list in `b`, done to the game in its order; a number no command has ends it, and says so
        pub fn apply(g: &mut Game, b: &[u8]) -> Result<(), String> {
            let mut r = Bytes::of(b);
            while r.at < b.len() {
                match u32::read(&mut r) {
                    $($i => { $(let $f = <$t as Field>::read(&mut r);)* let f: fn(&mut Game, $($t),*) = $do; f(g, $($f),*); })*
                    n => return Err(format!("no command has the number {n}")),
                }
            }
            Ok(())
        }

        /// the bytes of the commands in `b`, by their numbers, done to nothing
        fn each(b: &[u8], mut f: impl FnMut(u32, &[u8])) {
            let mut r = Bytes::of(b);
            while r.at < b.len() {
                let n = u32::read(&mut r);
                let from = r.at;
                match n {
                    $($i => { $(let _ = <$t as Field>::read(&mut r);)* })*
                    _ => return,
                }
                f(n, &b[from..r.at]);
            }
        }
    };
}

/// the lever words in a list of commands, in its order: (key, rung)
pub fn levers_said(b: &[u8]) -> Vec<(String, String)> {
    let mut out = Vec::new();
    each(b, |n, args| if n == LEVER {
        let mut r = Bytes::of(args);
        let key = <&str as Field>::read(&mut r);
        let rung = <&str as Field>::read(&mut r);
        out.push((String::from(key), String::from(rung)));
    });
    out
}
const LEVER: u32 = 13;

fn lever(key: &str) -> Option<game::difficulty::LeverKey> {
    use game::difficulty::LeverKey as K;
    [K::Fog, K::Told, K::Ai, K::Kit].into_iter().find(|k| k.as_str() == key)
}

commands! {
    /// the player cannot be hurt
    0 God { on: u32 } = |g, on| g.god_mode = on != 0;
    /// the others think
    1 Ai { on: u32 } = |g, on| g.ai_enabled = on != 0;
    /// rounds are not counted
    2 Ammo { on: u32 } = |g, on| g.infinite_ammo = on != 0;
    /// nobody sees the player
    3 Unseen { on: u32 } = |g, on| g.player_invisible = on != 0;
    /// the fog of war
    4 Fog { on: u32 } = |g, on| g.fog_of_war.on = on != 0;
    /// the view is this man's, by his place in the list; under 0 the player's own
    5 Spectate { who: i32 } = |g, who| g.spectate = who as f64;
    /// the mission from its start
    6 Restart { said: &str } = |g, said| game::game::game_restart(g, if said.is_empty() { None } else { Some(said) });
    /// a save by hand, as the next step ends
    7 Save {} = |g| saves::ask_save(g);
    /// a save goes back, as the next step ends, by its key
    8 Load { key: &str } = |g, key| if let Some(why) = saves::ask_load(g, key) { game::game::game_msg(g, &format!("checkpoint: {why}")); };
    /// the run saves itself
    9 Autosave { on: u32 } = |g, on| g.saves.enabled = on != 0;
    /// a card came up over the game: what the hands were lining up is let go unsent
    10 LetGo {} = |g| { let pl = g.player; g.pl[pl].launch_held = None; };
    /// whatever is up is backed out of
    11 Escape {} = |g| { let _ = game::game::game_escape(g); };
    /// how hard the defenders are, every lever back on it (before a game is made: what it is staged by)
    12 Difficulty { hard: u32 } = |g, hard| game::difficulty::set_difficulty(g, if hard != 0 { game::difficulty::Difficulty::Hard } else { game::difficulty::Difficulty::Standard }, Some(false));
    /// one lever over that, by its key and its rung (an empty rung: back to the level's own)
    13 Lever { key: &str, rung: &str } = |g, key, rung| if let Some(k) = lever(key) { game::difficulty::set_lever(g, k, if rung.is_empty() { None } else { Some(rung) }, Some(false), None); };
    /// one of the two tables under the difficulty, by itself
    14 Pressure { on: u32 } = |g, on| g.k.pressure.on = on != 0;
    15 DigIn { on: u32 } = |g, on| g.k.digin.on = on != 0;
    /// the fog of war's cone, whatever the level says, for a visit
    16 FogCone { on: u32 } = |g, on| game::difficulty::set_cone_visit(g, Some(on != 0));
    /// one knob, by its name (knobs.rs); a name no knob has is nothing
    17 Knob { name: &str, value: f64 } = |g, name, value| { let _ = crate::knobs::set(g, name, value); };
    /// the cards as the player's machine keeps them (before a game is made: what its men are staged in)
    18 Kit { kept: &str } = |g, kept| crate::kept::read(g, kept);
    /// the rung a quiet mission is played under, as this machine keeps it (a word that is no rung is nothing)
    19 QuietFog { rung: &str } = |g, rung| game::quietfog::quiet_fog_kept(g, rung);
    /// the cameras a quiet mission hands out, as this machine keeps them
    20 QuietCams { on: u32 } = |g, on| game::quietfog::quiet_cams_kept(g, on != 0);
    /// nobody plays, and the floor is looked at: the player stands aside, out of everybody's senses, and what is done is none of the mission's, the log's or the alarm's; 0 puts a fresh
    /// encounter back
    21 Aside { on: u32, x: f64, z: f64 } = |g, on, x, z| {
        (g.quiet_utility, g.touring, g.player_invisible) = (on != 0, on != 0, on != 0);
        if on == 0 { game::game::game_restart(g, None); }
        else { game::game::game_clear_aftermath(g); game::game::game_teleport_player(g, [x, 0.0, z]); }
    };
}
