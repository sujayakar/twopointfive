# What a mission already owes the defenders' way round

`tools/qa/wayround-check.ts` asks, for every mission with defender squads, whether the lived rooms' furniture takes a way round away
(LOST) or makes a way longer that is within 2 m of its length limit (SQUEEZED). Its head says how.

`<mission id>.txt` here is what a mission is known to owe: lines that start with `#` say what and since when, then one line
`<lost> <squeezed>`, as the check counts them on seeds 1-3 (where the two stands differ: a line each, `shipped <lost> <squeezed>` and
`frozen <lost> <squeezed>`; a stand with no line owes nothing). The check is red when a mission has more of either than its file, and a
mission with no file may have none.

A file is a debt with a name on it. Write one only for a loss that was already in the build when it was found, never to pass a level
you are furnishing: move the furniture off the way instead. When the level is mended the check says the file can be lowered or
deleted.
