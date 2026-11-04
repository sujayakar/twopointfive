# A level's base, as a file of its own

`<name>.txt` here is the base `tools/qa/lived-check.ts` holds `levels/<name>.json` to: the git blob id of that level file as meant.
It wins over the level's row in the check's `LEVEL_BASE` table. The check's head says why and what is red.

Write one only when a level's own part (everything but its lived things) was changed on purpose, in the commit that carries the change:

    git hash-object -w levels/<name>.json > tools/qa/fixtures/level-base/<name>.txt

A line that starts with `#` says what changed. A look twin (`<name>-gwot.json`) has its own file or none.
