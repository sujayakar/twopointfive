# The wire's table

The Rust game owns its types: each module's `types.rs`, `knobs.rs` and `ids.rs`, and the bodies, are source you edit by hand. A state only
Rust uses needs nothing here. This folder is only about what the page reads of the game.

- `tools/blitz/schema.json` says how each carried type is written, `fields.json` says which fields go every tick, which go to the page and
  which go at a start only, and `out-events.json` lists the events the game sends out.
- `wire.ts add` puts a field on the wire, and refuses a name nothing the page runs reads. `--serial` makes a row's own serial, so a new serial is a new object on the puppet; `--holds-serial` a field that names another object by its serial; `--serial-keys` a map or set keyed by serials. `add-struct` adds a whole struct (`--page` lists its fields for the page and the tools, `--tick` for the tools only, and a field's type may hold commas), `add-enum` a plain enum, `add-event` a call the Rust game hands to the host after a step (`drop-event` takes one off). `serial` says a field already on the wire names an object by its serial. `list` brings a field already on the wire back every tick, to the tools and with `--page` to the page too. `drop` takes
  fields off, and `drop-type` takes a type off once no field names it. An enum arm needs nothing: an arm the table doesn't know goes out as the tag 0xffff and the page keeps what it had.
- `wire.ts gen` writes these files and no others: `rust/game/src/puppet.rs`, `rust/game/src/state_words.rs`, `rust/game/src/id_tables.rs`,
  `tools/blitz/puppet-apply.ts`, `src/view/read.ts` (from the `view::` types; an ID in one is refused),
  `tools/blitz/puppet-lists.json`, `tools/blitz/out-dispatch.ts`, the `layout` in `schema.json` and the one `LAYOUT` line at the end of
  `rust/game/src/io.rs`.
- **It never writes a types file.** `gen` stops if its list ever holds another file, or if a file it makes in whole declares a struct or
  an enum. `gen --check` runs the same checks and exits 1 where a file would change.
- `cost.ts` says what a tick's export and apply cost now, on the page's list.

Both halves carry the stamp (`LAYOUT`), and a host refuses a module of another stamp. Change the table with the tool, not by hand, and the
stamp follows.

- `back-check.ts`: what the Rust game writes and a tool reads comes back on a list. `class-defaults.ts` writes `tools/blitz/class-defaults.json`, what a row the apply makes holds beside what it is sent, from the things a start builds (`--check`: red where the file would change, or where the class table lacks a class).
