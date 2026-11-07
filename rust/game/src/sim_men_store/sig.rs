#![allow(unused_variables)]
//! Every function and method of `sim_men_store`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// INVENTED (MenStore.enter): a row for a new man, the lowest free one, its generation counted
pub fn man_enter(g: &mut crate::game::Game) -> u32 { let m = &mut ::sim::state::state(g.sim).men; for r in 0..::sim::men::ROWS { if m.gen[r] & 1 == 0 { m.gen[r] += 1; return r as u32; } } panic!("a game of more than {} men: levelfit.ts menHeld says a game cannot hold that many", ::sim::men::ROWS) }
/// INVENTED (MenStore.leave): his row is free again, and zeros
pub fn man_leave(g: &mut crate::game::Game, r: u32) { let m = &mut ::sim::state::state(g.sim).men; let r = r as usize; assert!(m.gen[r] & 1 == 1, "row {} of the men left twice", r); m.since[r] = 0.0; m.owner[r] = 0; m.rank[r] = 0; m.reason[r] = 0; m.gen[r] += 1; }
/// INVENTED (store.ts standMan): a Character's row, as he is made in a game
pub fn stand_man(g: &mut crate::game::Game, c: CharId) { let r = man_enter(g); let ch = &mut g.ch[c]; ch.man_row = r; ch.man_kept = None; }
/// INVENTED (store.ts manLeaves): HE HAS LEFT THE GAME: his row is free, and he reads his numbers as they stood. guards.ts:2940, :3522, civilians.ts:207, game.ts:1496 call it
pub fn man_leaves(g: &mut crate::game::Game, c: CharId) { if g.ch[c].man_row == 0xffffffff { return; } let kept = [man_owner(g, c) as f64, man_rank(g, c) as f64, man_reason(g, c) as f64, man_since(g, c)]; let r = g.ch[c].man_row; man_leave(g, r); let ch = &mut g.ch[c]; ch.man_row = 0xffffffff; ch.man_kept = Some(kept); }
fn man_word(g: &crate::game::Game, c: CharId, col: fn(&::sim::men::Men) -> &[u32; ::sim::men::ROWS], kept: usize) -> u32 { let ch = &g.ch[c]; if ch.man_row != 0xffffffff { col(&::sim::state::state(g.sim).men)[ch.man_row as usize] } else { ch.man_kept.map_or(0, |k| k[kept] as u32) } }
/// INVENTED (cols.ts ManCols.owner): a bit for each rule with a hand on him. A guard's man is `g.gd[gd].ch`
pub fn man_owner(g: &crate::game::Game, c: CharId) -> u32 { man_word(g, c, |m| &m.owner, 0) }
/// INVENTED (cols.ts ManCols.rank): the place of that top bit in the ranks
pub fn man_rank(g: &crate::game::Game, c: CharId) -> u32 { man_word(g, c, |m| &m.rank, 1) }
/// INVENTED (cols.ts ManCols.reason): which of the top rule's reasons it gave
pub fn man_reason(g: &crate::game::Game, c: CharId) -> u32 { man_word(g, c, |m| &m.reason, 2) }
/// INVENTED (cols.ts ManCols.since): the game time the claim now on top came to the top
pub fn man_since(g: &crate::game::Game, c: CharId) -> f64 { let ch = &g.ch[c]; if ch.man_row != 0xffffffff { ::sim::state::state(g.sim).men.since[ch.man_row as usize] } else { ch.man_kept.map_or(0.0, |k| k[3]) } }
/// INVENTED: the writes of the same columns
pub fn man_set_owner(g: &mut crate::game::Game, c: CharId, v: u32) { let ch = &mut g.ch[c]; if ch.man_row != 0xffffffff { ::sim::state::state(g.sim).men.owner[ch.man_row as usize] = v; } else if let Some(k) = ch.man_kept.as_mut() { k[0] = v as f64; } }
pub fn man_set_rank(g: &mut crate::game::Game, c: CharId, v: u32) { let ch = &mut g.ch[c]; if ch.man_row != 0xffffffff { ::sim::state::state(g.sim).men.rank[ch.man_row as usize] = v; } else if let Some(k) = ch.man_kept.as_mut() { k[1] = v as f64; } }
pub fn man_set_reason(g: &mut crate::game::Game, c: CharId, v: u32) { let ch = &mut g.ch[c]; if ch.man_row != 0xffffffff { ::sim::state::state(g.sim).men.reason[ch.man_row as usize] = v; } else if let Some(k) = ch.man_kept.as_mut() { k[2] = v as f64; } }
pub fn man_set_since(g: &mut crate::game::Game, c: CharId, v: f64) { let ch = &mut g.ch[c]; if ch.man_row != 0xffffffff { ::sim::state::state(g.sim).men.since[ch.man_row as usize] = v; } else if let Some(k) = ch.man_kept.as_mut() { k[3] = v; } }

