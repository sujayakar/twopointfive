# The game's module

`rust/game_face` is the game as one WebAssembly module with one memory: what a page, the game server and the tools load. It links the rules
(`rust/game_rules`), what is drawn (`rust/game_draw`), a start and what a host does round a step (`rust/game_start`) and the world's
numbers (`rust/sim`). It imports nothing. A host hands it a start and what each seat's hands did, and reads back what is drawn and told; a game is a
number.

**The calls are declared once**, in `rust/game_start/src/host/calls.rs`: a name, how it holds a game (`reads`, `does`, or `free` of any), its arguments by type
and what comes back. `rust/game_face/src/` has a function of each name and makes the exports from the list; the program `half` writes the page's caller
(`src/draw/face.ts`, class `Face`) and the readers of what is told (`src/view/read.ts`). Every export has one shape: the game's number and one run of bytes, the
arguments in their order (`host/field.rs`, `src/draw/writer.ts`); back comes a number, or the bytes of a type that crosses (`src/draw/wire.ts`). A wrong
name, argument or answer does not compile on either side, and a module of another list is refused as it loads (the stamp).

## Build

```sh
bun run tools/blitz/face-build.ts                              # builds if a source is newer, and says where the file is
cd rust/game_face
cargo build --target wasm32-unknown-unknown --profile play     # optimized, no link-time optimization: what the dev server builds
cargo build --target wasm32-unknown-unknown --release          # the fat link: what `bun run build` ships
```

The compiler and the target are `rust/rust-toolchain.toml`'s. The head of `tools/blitz/face-build.ts` says which file is served when: a newer build of
either profile, or the checked-in module (under `rust/game_face/`, `--prebuilt` makes it) where its hash is this checkout's sources' hash. The module aborts
on a panic, so a host catches a trap and reads the words (`face_panic_ptr`); the game of that call is gone, the others and every game kept stand. The
checked-in module keeps its functions' names (1.3 of its 19 MB), so that a fault from a player's machine says them.
