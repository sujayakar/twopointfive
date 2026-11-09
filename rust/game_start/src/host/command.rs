//! WHAT A PLAYER OR A PANEL DOES THAT IS NOT INPUT, as one typed list: a host hands the list for a seat before a step, and a replay or another machine is handed the
//! same bytes. A command is declared here once (`commands!`); the page's writer of it is made from the declaration by `bun run tools/draw/gen.ts`.
//!
//! The bytes: a command's number (u32), then its fields in order, little end first: u32, i32, f64, or text as its length (u32) and its bytes.
use alloc::{format, string::String};
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

    };
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
    /// a save goes back, as the next step ends, by its key (one that will not is said then)
    8 Load { key: &str } = |g, key| g.saves.due = Some(String::from(key));
    /// the run saves itself
    9 Autosave { on: u32 } = |g, on| g.saves.enabled = on != 0;
    /// a card came up over the game: what the hands were lining up is let go unsent
    10 LetGo {} = |g| { let pl = g.player; g.pl[pl].launch_held = None; };
    /// whatever is up is backed out of
    11 Escape {} = |g| { let _ = game::game::game_escape(g); };
    // (12 to 16, 18 to 20 and 36 said what a host sets before a game is made: that is one record now, `kept::Kept`, and no number is used again for another)
    /// one knob, by its name (knobs.rs); a name no knob has is nothing
    17 Knob { name: &str, value: f64 } = |g, name, value| { let _ = super::knobs::set(g, name, value); };
    /// nobody plays, and the floor is looked at: the player stands aside, out of everybody's senses, and what is done is none of the mission's, the log's or the alarm's; 0 puts a fresh
    /// encounter back
    21 Aside { on: u32, x: f64, z: f64 } = |g, on, x, z| {
        (g.quiet_utility, g.touring, g.player_invisible) = (on != 0, on != 0, on != 0);
        if on == 0 { game::game::game_restart(g, None); }
        else { game::game::game_clear_aftermath(g); game::game::game_teleport_player(g, [x, 0.0, z]); }
    };
    /// what each man is thinking is said over him (and the meters' raw numbers with it)
    22 Labels { on: u32 } = |g, on| g.show_debug = on != 0;
    /// the hour over the level, for a visit (a word that is no hour: the level as it was built)
    23 Hour { id: &str } = |g, id| { let _ = game::hours::apply_hour(g, game::hours::hour_id_of(Some(id))); };
    /// how much of the sky the picked weather covers, and the share of the hour's own sun that comes through whatever the cover
    24 Cover { share: f64 } = |g, share| game::hours::turns::set_cover(g, share);
    25 Sun { share: f64 } = |g, share| game::hours::set_sun_strength(g, share, None);
    /// the level's moon: its strength, its height and bearing in degrees (a number that is none leaves it)
    26 Moon { k: f64, elev: f64, bearing: f64 } = |g, k, elev, bearing| game::hours::turns::set_moon(g, k, elev, bearing);
    /// the kit picture: the team's cards (as Kit's text) and how many come (under 0: as it stands), staged again
    30 KitStageCards { kept: &str, size: i32 } = |g, kept, size| { super::kept::read(g, kept); game::kitstage::cards(g, size); };
    /// whose card is open (0 the player) and the row under its highlight
    31 KitStageOpen { who: u32, field: &str } = |g, who, field| game::kitstage::open(g, who, field);
    /// where the picture's men should sit in the view: its shape, then their middle, their share of its height and width and a man's, the door's line and the edges it is hung between, and the column's top and
    /// foot (a number that is none: not said)
    32 KitStageFrame { aspect: f64, cx: f64, cy: f64, h: f64, w: f64, man: f64, door: f64, left: f64, right: f64, keep: f64, top: f64, bottom: f64 } = |g, aspect, cx, cy, h, w, man, door, left, right, keep, top, bottom| {
        game::kitstage::frame(g, aspect, game::kitstage::frame_ask(cx, cy, h, w, man, [door, left, right, keep], [top, bottom]));
    };
    /// the kit picture: the page has laid its copy over the picture, as the view's `veil` asked
    33 KitStageUnder {} = |g| game::kitstage::under(g);
    /// the kit picture: the page has shown its first picture: the acts may begin
    34 KitStageShown {} = |g| game::kitstage::shown(g);
    /// one of the panel's actions, by its name (actions.rs), with what its row picked
    35 Act { name: &str, value: f64, word: &str } = |g, name, value, word| super::actions::act(g, name, value, word);
}
