//! THE CALLS OF THE MODULE, declared once: what a page, a server or a tool asks of the games in it. The exports are made from the list in the crate `game_face` (which has a function of each name), and the
//! page's caller of each is written from the list's table by the program `half` (src/draw/face.ts).
//!
//!   /// what is said of it
//!   HOW name(argument: type, …) -> a number;      or  => what is told;    or neither
//!
//! HOW it holds a game: `reads` one, shared; `does` something to one; or is `free` of any, so it may be asked while a game is lost to a panic (a game it is handed by its number).
//! Every call crosses the same way: the game's number and one run of bytes, the arguments in their order (`field`); back comes a number, or what is told at face_out (its bytes, `game::crosses::Answer`).

/// a game's number in the module (0: none), and a moment's
pub type GameNo = u32;
pub type MomentNo = u32;

#[macro_export]
macro_rules! calls { ($made:ident) => { $made! {
    /// the stamp of everything that crosses: a page whose half is of another is refused
    free stamp() -> u32;
    /// room asked for and given back, so that no step has to grow the memory to find it
    free reserve(bytes: usize);

    /// A GAME FROM ITS START (game_start `handed`): the level file's text, the address's words (empty: none), the mannequin's gltf and its buffer, the cast's gltf and the look's (empty: none), what the machine keeps (empty:
    /// nothing). No state crosses. Flags: 1 the rows that dress the picture are up, 2 the host makes what `ready` makes. 0: there is no room for another
    free start(level: &[u8], words: &str, mannequin: &[u8], mannequin_bin: &[u8], cast: &[u8], cast_look: &[u8], kept: &str, seed: u32, seats: u32, flags: u32) -> GameNo;
    /// what the first step wants and such a start leaves, so many lines of it (the light under the roofs): there is more. A step that finds it wanting makes the rest
    reads ready(lines: u32) -> bool;
    /// the game and its place in the module are free again
    free drop(game: GameNo);
    /// which seat this machine shows, by its place in the game's list
    does show_seat(seat: u32);
    /// ONE TICK of every seat on what was handed. Flags: 8 the save controller's turn
    does step(dt: f64, flags: u32) => Result<(), String>;
    /// the host has taken (or does not want) what the tick told it: the list is a step's output and no part of the state, so nothing else empties it
    does out_clear();

    /// WHAT A PLAYER DID, for the next step: his hand's numbers and his keys' codes, those held and, after a line feed, those that went down. A seat nobody hands is as it last was, with nothing pressed
    does hand(seat: u32, numbers: Vec<f64>, keys: &str);
    /// WHAT A PLAYER OR A PANEL DID THAT IS NOT INPUT, before a step: the typed list (`command`), done in its order
    does commands(list: &[u8]) => Result<(), String>;
    /// nobody's hands are on a seat: the hands it is handed from here are his own (game_start `hands_off`): the seed, the seconds he plays, does he throw the breaker first, the doors by name he takes first
    does hands_off(seat: u32, seed: u32, cap: f64, breaker: bool, route: Vec<&str>);
    /// the seat is the host's again
    does hands_on();
    /// how he stands (nobody is hands off: none)
    reads hands_said() => Option<HandsSaid>;
    /// his timeline, a line an entry
    reads hands_trace() => String;

    /// once, as a networked game is stood: every seat is another machine's, so a record's view is worked out again from its pose on every machine alike
    does networked();
    /// A SEAT'S RECORD FOR THE NEXT STEP, as it travelled: its numbers, and its keys as a hand's. It is no hand: the host's own camera is not moved by it. Of a seat that has both, what was handed last is stepped
    does record(seat: u32, numbers: Vec<f64>, keys: &str) => Result<(), String>;
    /// WHERE THE SEAT'S MAN WOULD BE after records the game has not had: their numbers one after another, their keys a line feed between two. None: where the game has him. Nothing a tick reads is written
    does ahead(seat: u32, numbers: Vec<f64>, keys: &str) => Result<AheadView, String>;

    /// WHAT THE LAST TICK LEFT TO BE DRAWN, the sections of `want` made again from the game as it stands, once a tick: the address of a table of two words a section (its address and its rows). They lie there until the next call
    does draw(want: u32) -> usize;
    /// … of a tick that is not the first a host steps before it draws a frame: what moves is blended over them all
    does draw_over(want: u32) -> usize;
    /// A FRAME IS DRAWN, `between` 0 and 1 of the way from the tick before to the last, looking at `focus`: the rows that move stand there, and the table again
    does frame(between: f64, focus: [f64; 3]) -> usize;
    /// what the page says of its look, once and when it changes
    does draw_look(numbers: Vec<f64>);
    /// once a tick, of a networked host: its own man's place ahead of the tick: where the tick has him (his feet, the way he aims) and where he is drawn; 0 = nobody is ahead
    does draw_lead(man: u32, numbers: Vec<f64>);
    /// a viewer's own picture of a run: with the fog off it, and through a tube or not (none: as the run had it). From the first call the game keeps what such a picture is made from, tick by tick
    does draw_as(bared: bool, tube: Option<bool>);
    /// a tick of the host's in which the game was not stepped: what moves stands where the last tick left it
    does draw_hold();
    /// the picture goes on from where the game stands now, and blends from nothing before it (ticks were stepped that were not drawn)
    does draw_cut();
    /// the floors' traffic map (game_draw `traffic`), worked out as it is asked: once, as a level opens
    reads traffic() => Vec<u8>;
    /// a name that crosses by its number (a sound's)
    reads draw_name(number: u32) => String;

    /// what a HUD is told of the game as its last tick left it, about the seat this machine shows
    reads tick_view() => TickView;
    /// what the cursor is on, about a seat (none: the one this machine shows): the cursor (x y), the picture it is in (w h), the camera's view and projection and their inverse
    reads cursor_view(seat: Option<u32>, numbers: Vec<f64>) => CursorView;
    /// what the end card shows of the run, asked as it opens
    reads debrief_view() => DebriefView;
    /// … and beside that: how it ended with every man of it, the course's par, its marks and the floor its map is of
    reads end_view() => EndView;
    /// what a machine keeps of the run, asked as its try begins and as it ends
    reads run_view() => RunView;
    /// THE MISSION'S VERDICT (none: it is being played). The game's own tick latches it: the first end of a run stays
    reads verdict() => Option<OutcomeView>;
    /// the floor, asked once a stand, and who and what stands on it now
    reads floor() => FloorView;
    reads map() => MapView;
    /// what a measuring take notes
    reads tally() => Tally;
    /// every knob a person may turn
    reads knobs() => Vec<KnobRow>;
    /// the panel's rows of the game's numbers, and what each stands at now as its row shows it, in their order
    free shown() => Vec<ShownRow>;
    reads shown_now() => Vec<f64>;
    /// the panel's actions
    reads actions() => Vec<ActionRow>;

    /// is a line of sight from a to b stopped (glass is passed)
    reads sight(a: [f64; 3], b: [f64; 3]) -> bool;
    /// … by anything solid, whole glass with it
    reads solid(a: [f64; 3], b: [f64; 3]) -> bool;
    /// the nearest thing along a ray (d a unit vector), within `max`
    reads ray(o: [f64; 3], d: [f64; 3], max: f64) => Option<Met>;
    /// how a sound at `from` reaches a listener at `to`. A way round is searched for once a tick between the same two places of the floor; the game remembers none of it
    reads hear(from: [f64; 3], to: [f64; 3]) => SoundPath;
    /// the way on the floor from a to b, its corners (none: the floor has none): a search, kept by nobody here
    reads way(a: [f64; 3], b: [f64; 3], most: u32) => Option<Vec<[f64; 3]>>;
    /// the light that stands on a point, leaving out the carried lamps of `owners`; `steady`: what the place lights, with nothing that passes; `leave`: lamps of the level's own left out, by what feeds them
    does light(p: [f64; 3], owners: Vec<f64>, steady: bool, leave: f64) -> f64;

    /// this run's saves as a card lays them out
    reads saves() => SavesView;
    /// what the save controller tells the player
    does saves_told() => SavesTold;
    /// what a game lost to a panic would stand again from: a save's label (empty: its run's first step), none: nothing of it is kept
    free recover_what(game: GameNo) => Option<String>;
    /// it stands again, under a new number, from its best save, or from its run's first step
    free recover(game: GameNo, newest: bool) => Option<Recovered>;
    /// the moment the game stands at, between two ticks, whole; 0: it cannot be held (a hands-off player has the seat: what he is about is no part of the game)
    does moment() -> MomentNo;
    /// the game stands at a moment of its own again, also where it was lost to a panic since
    free moment_back(game: GameNo, moment: MomentNo) -> bool;
    free moment_drop(moment: MomentNo);

    /// the desk (`desk`), laid what this machine keeps and what the address says, as a start is laid
    free desk_machine(kept: &str, address: &str) => Result<(), String>;
    /// one ask of it, and the bytes of its answer
    free desk_ask(ask: &[u8]) => Result<Vec<u8>, String>;

    /// two games held to each other: the state's parts by name, and a hash of each
    free state_names() => Vec<String>;
    does state_words() => Vec<u32>;
    /// the die's count of draws
    reads die_n() -> f64;
    /// the parts of rust/sim's block by name, and a hash of each
    free part_names() => Vec<String>;
    reads part_hashes() => Vec<u32>;

    /// A PLANTED PANIC, for the check of the road back from one: inside the game, as a rule of the tick would
    does debug_panic();
} } }

use alloc::string::String;
use alloc::vec::Vec;
use game::crosses::{Answer, Ty, Types};
use game::view::{ActionRow, AheadView, CursorView, DebriefView, EndView, FloorView, HandsSaid, KnobRow, MapView, Met, OutcomeView, Recovered, RunView, SavesTold, SavesView, ShownRow, SoundPath, Tally, TickView};

/// what a call answers with
pub enum Back {
    Nothing,
    /// a number, by Rust's name of it
    Number(&'static str),
    /// what is told; may it refuse
    Told(fn(&mut Types) -> Ty, bool),
}
pub struct Call {
    pub name: &'static str,
    /// it holds a game
    pub held: bool,
    /// each by its name and Rust's name of its type
    pub args: &'static [(&'static str, &'static str)],
    pub back: Back,
}

macro_rules! table {
    (@held free) => { false };
    (@held reads) => { true };
    (@held does) => { true };
    (@back) => { Back::Nothing };
    (@back -> $n:ty) => { Back::Number(stringify!($n)) };
    (@back => $t:ty) => { Back::Told(<$t as Answer>::ty, <$t as Answer>::REFUSES) };
    ($($(#[doc = $d:expr])* $how:ident $name:ident($($a:ident: $t:ty),*) $(-> $n:ty)? $(=> $told:ty)?;)*) => {
        pub const CALLS: &[Call] = &[$(Call { name: stringify!($name), held: table!(@held $how), args: &[$((stringify!($a), stringify!($t))),*], back: table!(@back $(-> $n)? $(=> $told)?) }),*];
        /// what is said of each (no module carries it)
        pub const DOCS: &[&[&str]] = &[$(&[$($d),*]),*];
    };
}
calls!(table);

/// the calls as text, a line each
pub fn text() -> String {
    use core::fmt::Write;
    let mut s = String::new();
    for c in CALLS {
        let _ = write!(s, "{}{}", c.name, if c.held { " of a game" } else { "" });
        for (a, t) in c.args { let _ = write!(s, " {a}:{t}"); }
        match c.back {
            Back::Nothing => {}
            Back::Number(n) => { let _ = write!(s, " -> {n}"); }
            Back::Told(ty, refuses) => { let _ = write!(s, " => {}{}", ty(&mut Types::default()), if refuses { " or why not" } else { "" }); }
        }
        s.push('\n');
    }
    s
}
