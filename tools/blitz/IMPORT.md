# The importer

`applyGame(g, blob, knobs, { only?, hooks? })` (import.ts) applies an exported state onto a live game in place. It is an interpreter over schema.json, like export.ts, so a new field needs nothing here.

**Check** (`import-check.ts <scn>:<seed>:<M>:<N>`, roles a then b, separate processes): A plays to frame M and is exported, then to N and is exported. B plays to M, is exported, takes A's blob at N, and is exported again. The two exports at M differ only in `PropSystemStats.ms`, a wall clock, which is now `schema.scratch`.

| mission | M to N | result | apply |
|---|---|---|---|
| room-clear-four:1 | 600 to 2400 | the bytes are A's | 347 ms |
| depot-four:1 | 1000 to 4500 | the bytes are A's | 264 ms |
| hostage-rescue:2 | 600 to 3000 | the bytes are A's | 270 ms |
| snatch-four:1 | 100 to 1500 | the bytes are A's | 366 ms |
| glasshouse-four:1 | 30 to 900 | the bytes are A's | 320 ms |
| pass.json:1 | 300 to 1400 | the bytes are A's | 379 ms |

Those times are for whole states with several processes running. `only` with the every-tick list (`--only puppet-fields.json`): room-clear-four 600 to 2400, 108 ms, 96,110 values on the list, 0 differ.

**How it handles things:**
- A row keeps its object by the export's book, so a page must have exported the game once before the first apply.
- New rows and values are made whole, with their class's prototype from `classes.ts` (generated).
- Arrays and structs are written in place. An object reached twice in one apply stands in two places: the same bytes keep the one object, other bytes give this holder its own. The TypeScript shares by reference what Rust holds as values (`CoverJob.to` is a `LiveOpening.at`).
- Module variables go through `rollbackVar`'s setter or a container in place. expose.ts now has setters for the `let`s no registry holds, tool only: on a page those report `no setter`.
- It raises `PUT_BACKS.n` so the box mirror knows it wasn't told.

**One finding:** `src/game` only loads from game.ts. Entering the cycle elsewhere throws at player.ts:259 (`AIM_HOOKS`), against the rule in CLAUDE.md. `classes.ts` imports game.ts first for that reason.

**Fields that don't come back on purpose** (`INVERSE` in import.ts, `skipField`): `door_owns`, `who`, `pane_rule`, `pane_rows`, `glass_walkable`, `fire_pass`, `blast_listeners`, `listeners`, `RateCap.n`, `SkeletonInstance.rig`, `level_names`: either worked out from rows or set up once. Knob tables are read past.

**Not exercised:**
- A new man. No mission here made a Character after M, so his hook (rig, parts, block row) doesn't exist; `Item` is the only block row with one. `hooks.made` is where it goes.
- `hooks.gone`: nothing calls it, because lists are values and rows never leave a table.
- A second seat.
- Bytes from Rust itself. All of these are TypeScript exports, though Rust's are byte-identical on the missions the bridge check has run.
- It has only run headless, never on a page.

## An echo

A field Rust owns and TypeScript can't hold (`schema` marks it `echo`; hand2.cjs `H.bridge.echo` is the list) isn't written onto the puppet. A whole apply keeps a copy of its bytes beside the object (so a blob that is a view of the module's memory is safe) and the puppet's export writes them back, so a re-export after an apply is Rust's bytes everywhere and such a field needs no class in a host's compare.

Today: `Game.door_owns`, `AnimGltfMod.nvg_bits` and `suit_bits`, `WallBreak.listeners`, `StaticCollision.mirror`, `Mods.scene_box_mirror`, `WallreactMod.wall_react`, `LiveBeat.it`, `LiveBeat.waiting`, and every `blank` field (`H.bridge.blank`: fields only Rust has, carried as a counted blob; the TypeScript's walk writes none, and an import keeps what stands).

A filtered apply doesn't keep echoes, so a filtered host hands nothing back whole. If TypeScript does write such a field on the puppet, the write is lost silently: that is what makes it Rust's.

## A reference by serial

A field holding an object's identity (`schema.serial`, a `u32` or `Option<u32>` with a TypeScript name) now takes its object after the whole walk, so a row later in the walk can be the one that names it, and it works where the puppet's field was null. `Lesson.run === g.mission` holds on a puppet by this.

## What a None is

The walk writes `Option<T>`, and in the TypeScript a None is `null` or `undefined` by the field's type: `none` in the schema says which, read off the checker where only one is admitted. A field that admits both goes by what stands there: `undefined` stays, anything else becomes `null`. `Option<Option<T>>` is exact: None is `undefined`, Some(None) is `null`.

An empty list, map or set read over `undefined` makes an empty container: the field's type says it holds one. Read over `null` it stays `null`.

A map that is a plain object in the TypeScript (`rec` in the schema: `ChronEvent.data`, `Character.bones`) is made as a plain object, not a `Map`, so `e.data?.method` reads what it should.

A seat's record (`Seat.own`, `keepNone`): Rust holds no record for the seat that stands in the game, whose fields are the game's own. The TypeScript keeps one for every seat and its `Game` reads through it (seats.ts), so None leaves the record standing and the game's fields are written through it.

## The counters

`g.stats.<module>` is on the WHOLE walk, written into the module's table (`*_STATS`, by `B.top`), each row by name: a table's rows by a word where Rust holds a fixed array go by `B.keysOf`, and a `JsMap` by `B.nonZero` (a word Rust never counted stays in the table at 0). They are not on the every-tick list. The glass's counters are the host's rows plus the wasm's tally (`stats_as_counted`), as `GB_STATS` is on the TypeScript's side.

## After an apply

`schema.afterApply` lines run after every apply. Today one: `cleared.builtFor` follows `roomOfCell`. A memo's key the bridge doesn't carry makes a question on the puppet rebuild the memo, and that build writes the game's own count (cleared.ts: `this.known = 0`).
