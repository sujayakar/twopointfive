# Each level's car lamps

`<name>.txt` here is what `tools/qa/carlamps-check.ts` holds `levels/<name>.json` to on its first line: the head and tail lamps the lamp rule finds on its
parked vehicles, and how many of them glow. These files ARE the check's rows: there is no table in the check. A level with no file holds no lamp.

    # <what the vehicles are, or what changed>
    <head lamps> <tail lamps> <of them glowing>

Three whole numbers on one line with a space between, for example `4 4 0`. A line that starts with `#` is a note.
A level whose vehicle stands in a lived-in room has two such lines, because a stand may leave that room out: first the count without it, then with it
(`0 0 0` then `2 2 0` for a level whose only vehicle is a lived thing).
A look twin (`<name>-gwot.json`) has its own file.

Write or change a level's file in the commit that gives the level a vehicle, takes one away or moves a lamp. The check is red on line 1 for a wrong
count, a file that is not counts, and a file for a level that is not under `levels/`.
