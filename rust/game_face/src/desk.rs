//! The desk's two calls (game_start `host::desk`): what the cards before a game show and press, with no game started.
use game_start::host::desk::Desk;
use std::cell::RefCell;

thread_local! { static DESK: RefCell<Option<Box<Desk>>> = const { RefCell::new(None) }; }
fn desk<T>(f: impl FnOnce(&mut Desk) -> T) -> T { DESK.with(|d| f(d.borrow_mut().get_or_insert_with(|| Box::new(Desk::new())))) }

pub fn desk_machine(kept: &str, address: &str) -> Result<(), String> { desk(|d| d.machine(kept, address)) }
pub fn desk_ask(ask: &[u8]) -> Result<Vec<u8>, String> { desk(|d| d.ask(ask)) }
