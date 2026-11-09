# The game's module

`rust/game_face` is the game as one WebAssembly module with one memory: what a page, the game server and the tools load. It links the rules (`rust/game_rules`), what is drawn (`rust/game_draw`), what a host
does round a step (`rust/game_run`), a start (`rust/game_start`) and the world's numbers (`rust/sim`). It imports nothing. A host hands it a start, what each seat's hands did, and reads back what is drawn and
told; a game is a number.

Each file of `rust/game_face/src` says its own doors at its head: `lib.rs` (the memory, a panic's words, a game's life and its step, the words two machines hold each other to), `hand.rs` (what a seat's hands
did, and the commands), `net.rs` (every seat's record of a tick), `draw.rs` (the sections a page draws), `view.rs` (what a HUD and the panel are told), `ask.rs` (the world's questions), `saves.rs` and
`moment.rs` (a game kept), `desk.rs` (the cards before a game), `map.rs`, `end.rs`, `checks.rs` (doors for tools).

## Build

```sh
bun run tools/blitz/face-build.ts                              # builds if a source is newer, and says where the file is
cd rust/game_face
cargo build --target wasm32-unknown-unknown --profile play     # optimised, no link-time optimisation: what the dev server serves
cargo build --target wasm32-unknown-unknown --release          # the fat link: what a static build ships
```

The compiler and the target are `rust/rust-toolchain.toml`'s. On four cores `play` is three and a half minutes from nothing. The module aborts on a panic, so a host catches a trap and reads the words
(`face_panic_ptr`); the game of that call is gone, the others and every game kept stand.
