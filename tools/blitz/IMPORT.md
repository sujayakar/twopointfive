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

**Fields that don't come back on purpose** (`INVERSE` in import.ts, `skipField`): `door_owns`, `who`, `pane_rule`, `pane_rows`, `glass_walkable`, `fire_pass`, `blast_listeners`, `listeners`, `RateCap.n`, `ClipSide.at`, `SkeletonInstance.rig`, `level_names`: either worked out from rows or set up once. Knob tables are read past.

**Not exercised:**
- A new man. No mission here made a Character after M, so his hook (rig, parts, block row) doesn't exist; `Item` is the only block row with one. `hooks.made` is where it goes.
- `hooks.gone`: nothing calls it, because lists are values and rows never leave a table.
- A second seat.
- A mid-script state: the export refuses a running beat, and I didn't test the importer against one.
- Bytes from Rust itself. All of these are TypeScript exports, though Rust's are byte-identical on the missions the bridge check has run.
- It has only run headless, never on a page.
