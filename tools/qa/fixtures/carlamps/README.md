# A level's car lamps, as a file of its own

`<name>.txt` here is what `tools/qa/carlamps-check.ts` holds `levels/<name>.json` to on its first line: the head and tail lamps the lamp rule finds on its
parked vehicles, and how many of them glow. It wins over the level's row in the check's `KNOWN` table. The check's last comment says why and what is red.

Write one in the commit that gives a level a vehicle, takes one away or moves a lamp:

    # <what changed>
    <head lamps> <tail lamps> <of them glowing>

Three whole numbers on one line with a space between, for example `4 4 0`. A line that starts with `#` says what changed.
A level whose vehicle stands in a lived-in room has two such lines, because a stand may leave that room out: first the count without it, then with it.
A file is a row of the table and is read as one: what the table does with a row that allows no lamp at all, it does with a file that does.
A look twin (`<name>-gwot.json`) has its own file or none.
