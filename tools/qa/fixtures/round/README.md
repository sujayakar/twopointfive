# A level's count of round things, as a file of its own

`<name>.txt` here is what `tools/qa/round-check.ts` holds `levels/<name>.json` to on its first line: how many round things the round rule finds in it.
It wins over the level's row in the check's `KNOWN` table. The check's head says why and what is red.

Write one in the commit that gives a level a round thing, or takes one away:

    # <what changed>
    <as the file has it> <with its lived-in rooms stood in>

Two whole numbers on one line with a space between, for example `0 18`. A line that starts with `#` says what changed.
`TPS_LIVED=1 bun run tools/qa/round-check.ts` reads the second number, the plain run the first, and each says what it found when the two differ.
A look twin (`<name>-gwot.json`) has its own file or none.
