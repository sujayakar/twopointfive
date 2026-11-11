//! A GAME: started, stepped, let go, held to another.
use super::{free_slot, moment, put, saves, Held, GAMES};
use game_start::handed::Handed;
use game_start::host::calls::GameNo;

pub fn stamp() -> u32 { game::crosses::stamp(&(game_draw::layout() + &game_start::host::crosses::text())) }
pub fn reserve(bytes: usize) { core::mem::drop(Vec::<u8>::with_capacity(bytes.max(1))); }

#[allow(clippy::too_many_arguments)]
pub fn start(level: &[u8], words: &str, mannequin: &[u8], mannequin_bin: &[u8], cast: &[u8], cast_look: &[u8], kept: &str, seed: u32, seats: u32, flags: u32) -> GameNo {
    let Some(slot) = free_slot() else { return 0 };
    let o = game_start::stand::StartOpts { seed, seats, slot, dress: flags & 1 != 0, ready: flags & 2 == 0, ..Default::default() };
    put(Held::new(game_start::handed::stand_handed(&Handed { level, words, mannequin, mannequin_bin, cast, cast_look, kept }, &o).game))
}
pub fn ready(x: &Held, lines: u32) -> bool { !game::scene_collision::light_ready(&x.g, lines as usize) }
pub fn drop(game: GameNo) {
    saves::shelf_off(game);
    moment::off(game);
    GAMES.with(|v| if let Some(x) = v.borrow_mut().get_mut((game as usize).wrapping_sub(1)) { *x = None; });
}
pub fn show_seat(x: &mut Held, seat: u32) { let s = x.g.seats[seat as usize]; game::seats::show_seat(&mut x.g, s); }

const SAVES: u32 = 8;
pub fn step(x: &mut Held, dt: f64, flags: u32) -> Result<(), String> {
    let Some(cmds) = x.cmds.take() else { return Err(String::from("nobody handed the tick a record (hand, record)")) };
    game::game::game_step_seats(&mut x.g, dt, &cmds);
    saves::dealt(&x.g);
    if flags & SAVES != 0 { saves::saves_turn(x, dt); } else { saves::restart_turn(x); }
    Ok(())
}
pub fn out_clear(x: &mut Held) { x.g.out.clear(); }

pub fn state_names() -> Vec<String> { game::state_words::state_names() }
pub fn state_words(x: &mut Held) -> Vec<u32> {
    let (mut o, mut words) = (game::bytes::W::new(), Vec::new());
    game::state_words::state_words(&mut x.g, &mut o, &mut words);
    words
}
pub fn die_n(x: &Held) -> f64 { x.g.dice.n }
pub fn part_names() -> Vec<String> { game_start::block::PART_NAMES.iter().map(|n| String::from(*n)).collect() }
pub fn part_hashes(x: &Held) -> Vec<u32> { game_start::block::part_hashes(x.g.sim) }
