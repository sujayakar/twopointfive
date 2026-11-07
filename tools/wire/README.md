# The wire's table

The Rust game owns its types: each module's `types.rs`, `knobs.rs` and `ids.rs`, and the bodies, are source you edit by hand. A state only
Rust uses needs nothing here. This folder is only about what the page reads of the game.

- `tools/blitz/schema.json` says how each carried type is written, `fields.json` says which fields go every tick, which go to the page and
  which go at a start only, and `out-events.json` lists the events the game sends out.
- `wire.ts add` puts a field on the wire, and refuses a name nothing the page runs reads. `add-struct` adds a whole struct. `drop` takes
  fields off, and `drop-type` takes a type off once no field names it. An enum arm needs nothing: an arm the table doesn't know goes out as the tag 0xffff and the page keeps what it had.
- `wire.ts gen` writes these files and no others: `rust/game/src/puppet.rs`, `rust/game/src/state_words.rs`, `tools/blitz/puppet-apply.ts`,
  `tools/blitz/puppet-lists.json`, `tools/blitz/out-dispatch.ts`, the `layout` in `schema.json` and the one `LAYOUT` line at the end of
  `rust/game/src/io.rs`.
- **It never writes a types file.** `gen` stops if its list ever holds another file, or if `puppet.rs` or `state_words.rs` declares a struct or
  an enum. `gen --check` runs the same checks and exits 1 where a file would change.
- `cost.ts` says what a tick's export and apply cost now, on the page's list.

Both halves carry the stamp (`LAYOUT`), and a host refuses a module of another stamp. Change the table with the tool, not by hand, and the
stamp follows.
