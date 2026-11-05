# A level's places for a wall charge, held

`<name>.txt` here is how many places `tools/qa/wallfurn-check.ts` finds on `levels/<name>.json` where a wall charge can be set: the `(N places)`
on the level's first line of that check, on the frozen stand and then on the shipped stand (`TPS_PROFILE=shipped`, the level with its lived-in
rooms in). The check is red when the level has another number than its file holds on the stand it is run on.

Furniture takes these places: a thing against a wall leaves a charge nowhere to be set from, or no floor behind the wall. Dressing a level must not
take one by accident. When a level's count changes, either put it back, or write the new count in the same commit as the change and, for a loss,
say in the file which places went and why.

    # <for a loss kept on purpose: which places, and why>
    <places on the frozen stand> <places on the shipped stand>

Two whole numbers on one line, a space between. A line that starts with `#` is a note. Nothing about a level's places is written in the check's
source.

- Every level a mission on the list plays on has a file, the tutorials and the quiet missions among them: a wall there can be breached by a
  player who chooses to. A run of the check that names no level asks them all, and every level with a file. A level with no file is red, and the
  line gives the count on the stand of the run.
- More places than the file holds is red too, and the line says what to write: slack in a file is room for a later loss to pass.
- A file that is not two counts, and a file for no level under `levels/`, are red.
- A look twin (`<name>-gwot.json`) has no file. It has its level's boxes and is asked against its level's file: one fact is written once.
- It is a count, not the set of places: a place lost and another gained in the same change is not seen.
- `WALLPLACES_BREAK=1` reads the sweep as one place short on every level: every line must fail, on both stands.
