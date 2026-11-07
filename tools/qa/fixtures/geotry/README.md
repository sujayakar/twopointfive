# A level's look-page shapes, as a file of its own

`<name>.txt` here is what `tools/qa/geotry-check.ts` holds `levels/<name>.json` to, as the look page opens it (its look twin where it has one, lived in
and dressed): how many yard walls the geometry trial takes a bite out of.

A level with NO file must give none. Write, change or remove a level's file in the commit that gives it a yard wall, or takes
one away:

    # <what changed>
    <bites>

One whole number on one line, for example `3`. A line that starts with `#` says what changed. The check names the level and
what it found when the two differ, and is red on a file that names no level.
