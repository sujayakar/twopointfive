// A STUN CANISTER IS NOT THROWN AT A DOOR THE SQUAD PULLS OPEN — src/game/doortools.ts DOORTOOLS.bangWay (settled). The maintainer, 10-11, on the outside clear: "the
// squad does waste a lot of bangs". With doors that open one way, a 'bang and clear' given from a door's PULL face swings the leaf toward the stack, and the can,
// thrown a beat after the leaf starts to move, met it and went off on the squad's own side: the room was not flashed and the can was spent.
//   bun run tools/qa/bangway-check.ts [--seeds 1-3]        exit 0 = holds, 1 = it does not
// bang-tally.ts's own staged entry (a hot room, the four a few steps outside, 'bang and clear' from the menu's order) on the depot:
//   a held room's door from its pull face: no can goes off on the squad's side, and a can does go off in every run
// THE DOOR IS PICKED BY WHAT IT IS, not by a name (pick(), below): the depot's floor was rebuilt once and its side doors renamed, and a name that is
// gone made every run a skipped run. A door fits when it is an entry of a room a defender holds on EVERY seed asked and the squad's side of it (the entry's
// outside) is its pull face on every one of them (Doors.pullFace: the swing is dealt by the level's geometry). Of the doors that fit, the ids of PREFER come
// first in their order, so a floor that still has the door the lines were written on runs that door; then the fullest room. No such door = the lines FAIL
// with the doors the floor does have.
// THE RULE AND THE ONE-WAY DOORS ARE SETTLED (2026-11-07): nothing here writes the rule. Out: 'rule off: the can meets the leaf and goes off outside', the
// line of a pushed door played 'the same with the rule on and off, run for run', and the note for a stand whose leaves swing both ways.
// BANGWAY_BREAK=1, the red path: the entry is given as 'open and clear', so no can is thrown and the second line must fail (it was: the rule off).
import { ROOT } from './headless.ts';
import { run, stage, type Spec } from './bang-tally.ts';

const args = process.argv.slice(2), flag = (n: string) => { const i = args.indexOf(n); return i >= 0 ? args[i + 1] : undefined; };
const BREAK = process.env.BANGWAY_BREAK === '1';
const seedsOf = (s: string): number[] => { const m = s.match(/^(\d+)-(\d+)$/); return m ? Array.from({ length: +m[2] - +m[1] + 1 }, (_, i) => +m[1] + i) : s.split(',').map(Number); };
const SEEDS = seedsOf(flag('--seeds') ?? '1-3');
const SCN = 'depot-four';
type At = { door: string; room: string };
/** the door the lines were written on, per floor — only an order among the doors that FIT (pick()); an id that is gone or does not fit is passed over */
const PREFER = ['ab',     // the corridor depot: the door between the crew room and the comms room, as the crew room's entry — it opens toward the stack on the comms side
  'b_n'];                 // the motor-pool depot: the signals hut's one door, which opens out of the hut toward the stack (a held-room door that keeps its id)
/** every entry of every held room, with whether the squad's side of it is its pull face — kept only where room and face are the same on every seed asked */
async function doors(): Promise<(At & { pulled: boolean; men: number })[]> {
  const seen = new Map<string, At & { pulled: boolean; men: number; n: number; mixed: boolean }>();
  const log = console.log, err = console.error, warn = console.warn;
  for (const seed of SEEDS) {
    console.log = console.error = console.warn = () => { };
    let S: Awaited<ReturnType<typeof stage>>; try { S = await stage(SCN, seed); } finally { console.log = log; console.error = err; console.warn = warn; }
    const rooms = new Map<any, number>(); for (const d of S.defs) rooms.set(d.room, (rooms.get(d.room) ?? 0) + 1);
    for (const [room, men] of rooms) for (const e of room.entries) {
      const pulled = !!S.g.doors.pullFace(e.door, [e.at[0] + e.out[0] * 0.8, 0, e.at[2] + e.out[1] * 0.8]);   // (a stride outside the entry: where the stack stands)
      const k = `${e.door.def.name}|${S.roomKey(room)}`, was = seen.get(k);
      if (was) { was.n++; was.men = Math.min(was.men, men); if (was.pulled !== pulled) was.mixed = true; } else seen.set(k, { door: e.door.def.name, room: S.roomKey(room), pulled, men, n: 1, mixed: false });
    }
  }
  return [...seen.values()].filter(x => x.n === SEEDS.length && !x.mixed);
}
function pick(all: Awaited<ReturnType<typeof doors>>): At | null {
  const rank = (x: At) => { const i = PREFER.indexOf(x.door); return i < 0 ? PREFER.length : i; };
  const fit = all.filter(x => x.pulled).sort((a, b) => rank(a) - rank(b) || b.men - a.men || a.door.localeCompare(b.door) || a.room.localeCompare(b.room));
  return fit.length ? { door: fit[0].door, room: fit[0].room } : null;
}

async function arm(at: { door: string; room: string }) {
  const out = []; for (const seed of SEEDS) out.push(await run({ scn: SCN, ...at, arm: BREAK ? 'open' : 'bang', seed } as Spec, false)); return out;
}
const show = (rs: Awaited<ReturnType<typeof arm>>) => rs.map(r => r.note && !r.of ? `seed ${r.seed}: ${r.note}` : `seed ${r.seed}: ${r.bangs} can${r.bangs === 1 ? '' : 's'}, ${r.outside} outside, lost ${r.lost}/${r.team}, room ${r.down}/${r.of}`).join(' · ');

export async function bangwayCheck(): Promise<boolean> {
  let fails = 0; const line = (ok: boolean, what: string) => { console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${what}`); if (!ok) fails++; };
  console.log(`=== bangway-check · ${SCN} · seeds ${SEEDS.join(',')}${BREAK ? ' · BANGWAY_BREAK: open and clear given, no can' : ''} ===`);
  const all = await doors(), PULLED = pick(all);
  const have = all.map(x => `${x.door} (${x.pulled ? 'pulled' : 'pushed'}, room ${x.room})`).join(', ') || 'none';
  console.log(`  held-room doors on this floor: ${have}`);
  if (!PULLED) line(false, `a held room has a door the squad pulls open on every seed (the lines have nothing to run on — this floor's: ${have})`);
  else {
    const onA = await arm(PULLED);
    console.log(`  pulled door ${PULLED.door}: ${show(onA)}`);
    const ran = (rs: typeof onA) => rs.filter(r => r.of > 0);
    line(ran(onA).length === SEEDS.length && ran(onA).every(r => r.outside === 0), `no can goes off on the squad's side of a door they pull open (${ran(onA).reduce((a, r) => a + r.outside, 0)} outside over ${ran(onA).length} runs)`);
    line(ran(onA).length === SEEDS.length && ran(onA).every(r => r.bangs >= 1), `a can still goes off on every entry (${ran(onA).filter(r => r.bangs >= 1).length} of ${ran(onA).length}): it waits for the leaf, it is not given up`);
  }
  console.log(fails ? `=== bangway-check: ${fails} FAILED ===` : '=== bangway-check: ALL PASS ===');
  return fails === 0;
}
if (import.meta.main) process.exit((await bangwayCheck()) ? 0 : 1);
