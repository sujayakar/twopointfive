// Every case of src/game/use.ts as the page plays it: its answer, the dice it drew and the calls it made, as JSON on one line (retire-check.ts reads it
// before the fold and after it).
import { pageSwitches } from '../src/host/profile';
import { applyKnobs } from '../src/game/journal';
import { bind } from '../src/ui/bindings';
import { CASES } from '../src/game/use';
import { LOG, reset } from '../src/game/log';
pageSwitches(new URLSearchParams('')); bind(); void applyKnobs;
const out: Record<string, unknown> = {};
for (const [name, fn] of Object.entries(CASES)) { reset(); let v: unknown; try { v = fn(); } catch (e) { v = `THREW ${e instanceof Error ? e.message : String(e)}`; } out[name] = { v, draws: LOG.draws, calls: LOG.calls, sum: LOG.sum }; }
console.log(JSON.stringify(out));
