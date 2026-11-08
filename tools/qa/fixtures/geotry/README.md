# A level's look-page shapes, as a file of its own

`<name>.txt` here is what `tools/qa/geotry-check.ts` holds `levels/<name>.json` to, as the look page opens it (its look twin where it has one, lived in
and dressed): how many round things the geometry trial makes of the kit's octagons, and how many yard walls it takes a bite out of.

A level with NO file must give none of either. Write, change or remove a level's file in the commit that gives it a round thing or a yard wall, or takes
one away:

    # <what changed>
    <round things> <bites>

Two whole numbers on one line with a space between, for example `19 3`. A line that starts with `#` says what changed. The check names the level and
what it found when the two differ, and is red on a file that names no level.
