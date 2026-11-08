// THE BIG SLOT (src/game/kit.ts BIGSLOT — every card has ONE big slot beside its pouches: the wall charge, the breaching shotgun, the dynamic
// hammer; src/game/bigkit.ts — the last two on a locked door: yours a row in the boot's place, a teammate's on a door order), headless:
//   bun run tools/qa/bigslot-check.ts [card|off|you|mate|shut|moves|weight|wiring]      (BIGKIT_BREAK=1 = the tools do nothing to a lock: every 'the lock is gone' line must fail)
// The maintainer, 10-14: "you have a single big slot on the bottom that can fit larger kit items and then smaller pouches for other stuff" · "i do think we
// should change the breaching shotgun to not take up a primary weapon slot" · "and then we should also add a dynamic hammer".
//   card    the switch on: one 'big' row a card, the weapon row a rifle or a pistol, no wall charge row; no step of the row puts two big
//           things on a man; a stored shotgun-as-gun card is read as the issued gun with the shotgun in the big slot; a stored card with two big things comes
//           down to one (the wall charge before a hammer) and says so; a hand that still sets 'walls' or the shotgun as a gun is
//           answered through the big row; the issued kit and every squad kit of the table are legal; a big thing fills the slot and no pouch; a table
//           from before the big slot marks the three big things by default. The switch off: a teammate's rows are the build before's, the weapon row
//           offers the shotgun, and a stored `big` is kept, inert
//   off     the switch off: a hammer forced onto the Player gets no row at a locked door, a staged man whose card says hammer carries none, and the
//           room clear's manager door under open and clear is the same run by digest with the cards' big slots filled as with them empty
//   you     the plain night's locked storage door, the tool put on you off your card. The shotgun and the hammer each, from the face the leaf opens away
//           from and from the face it opens toward (where the frame holds a boot): the row stands in the boot's place, and inside three seconds the lock is
//           gone — the shotgun's as gunfire, the hammer's with no shot: ONE blow where a boot would do, BIGSLOT.swings where the frame holds one, and
//           through that beat the HUD never says 'the frame took the boot' (a bare boot there does); from the face it opens toward the leaf stays home
//           and a hand opens it. A wedged door gives to the hammer and offers the shotgun nothing; a steel door offers neither
//   mate    the room clear's manager door under open and clear. A man whose card carries the shotgun keeps his rifle, brings the M870 up for the lock and
//           has his own gun back the moment it is off — and at once when he is given another order with it up. A man whose card
//           carries the hammer takes the lock in one blow and no shot. And a door locked on the face it opens toward, wedged (no pick opens it):
//           refused as ever without a tool, taken by the hammer in BIGSLOT.swings blows and pulled by hand — but refused to a SILENT squad, hammer or
//           no; the shotgun takes a keyed lock from that face the same way
//   shut    BIGSLOT.shut + BIGSLOT.beside (the maintainer after playing, 10-15: "i don't see how to use breaching shotgun or dynamic hammer" · "these
//           things should work for unlocked doors too" · "helps also not be directly in front of the door when opening" · "hammer should also help with
//           barricaded doors"). YOU at a door that is NOT locked: its own rows stand and the tool's row is listed under them; performed, it is worked from
//           beside the latch jamb, the latch is broken and the door ENDS OPEN from either face with no second press; a wedged door gives to the hammer
//           from the face it opens toward too. A TEAMMATE: the squad's own row on a door's tools — given by its row —
//           takes an unlocked door with the tool from either face and ends (no second trip to the lock), takes a locked one without the halt at a door
//           found shut, is refused to a silent squad, and is not there when nobody carries one; a plain open and clear on an unlocked door still uses
//           the handle
//   moves   BIGSLOT.moves (the maintainer, 10-15 evening, with Door Kickers 2 footage: "using the hammer shouldn't require being directly in front of the
//           door?" · "they put away their gun, equip the hammer, wind up, and then hit the door in one motion"). YOU, from where a stack's second man
//           stands (outside the hand's reach): the row is there, he WALKS to a place beside the frame and is never in the opening; a man on the latch
//           jamb leaves him the place off that man's shoulder, or the hinge jamb when he came from that side and the leaf opens away; the animator is
//           told each blow a wind-up ahead and it lands when told; the shotgun's place is a hand from the lock. A TEAMMATE's hammer is off his back
//           as he is sent and back on it after; both tools from hard by the frame. OVER THE RIFLE: with his rifle slung
//           across his back neither tool's boxes lie inside the rifle's (box against box, on this stand's body and rifle, and in the game's own frame);
//           with the rifle in his hands the tool lies on his back with no clearance. Looks at no picture
//   wiring  the two stands' row and word, the knob registry, the journal's tables (a recording from before the table replays with the switch off), slots.json
// SINCE 2026-11-07 NOTHING HERE WRITES A SETTLED ROW. BIGSLOT.shut, beside, moves, snug and weight, the tool's place at a pack's side (PACKSIDE), the
// kit's own hand (KITANIM) and the doors' frame rule are constants of the game: the rig no longer writes them, and every line that asked the build
// before one of them is cut (both knobs off, the moves off, the weight off, the first-pass timing, a recording from before each key). BIGSLOT.on is
// still a variable of a run, so `off` and the card's switch-off lines stand. Where a verdict needs a state no level of the repo has (a lock keyed to
// the face a leaf opens toward, a wedge under a leaf) the check sets it by hand and the verdict's line says so.
// BIGKIT_BREAK and BIGWEIGHT_BREAK are bigkit.ts's own. Nothing here looks at
// a picture: neither tool has ever been seen on a GPU.
import { resolve } from 'node:path';
import { readFileSync } from 'node:fs';
import { standUp, ROOT } from './headless.ts';
import { asProbes, exitCode, wanted } from './holds.ts';   // (run alone, each part runs as the probe the runner calls it by: one stand both ways)

const K = await import(`${ROOT}/src/game/kit.ts`);
const BK = await import(`${ROOT}/src/game/bigkit.ts`);
const { BIG_STATS } = BK;
const { doorPt, alongOf, offOf } = await import(`${ROOT}/src/game/doortools.ts`);
const flat = (a: number[], b: number[]) => Math.hypot(a[0] - b[0], a[2] - b[2]);
const { killGuard, handProps } = await import(`${ROOT}/src/game/combat.ts`);
const { BIG_TAG, SC20K_TAG, RIG, rifleM4, sc20kBoxes, bigBoxes } = await import(`${ROOT}/src/game/rigProps.ts`);
const { KITANIM, kitSlingFront, kitInHand } = await import(`${ROOT}/src/game/kitanim.ts`);
const { Character, KIT: HANDKIT } = await import(`${ROOT}/src/game/character.ts`);   // (HANDKIT: the kit hand's own times — the gun swap a tool is weighed against)
/** the hollow of his right fist as the PICTURE has it (the drawn wrist: the look-only pass of inhand.ts may have moved it off bones.handR), and how far a point is from a drawn box (0 inside) */


/** the big slot's hammer as it is DRAWN on the man with this character id (rigProps.ts bigBoxes, by its tag): how many boxes, and the head's centre; `all` = every box the frame draws on anybody */
const drawnTool = (g: any, id: number) => { const out: any[] = []; handProps(g, out); const mine = out.filter(b => b.name === BIG_TAG && b.owner === (id & 0xff)), head = mine.find(b => b.h[0] > 0.05); return { n: mine.length, head: head ? [...head.c] as number[] : null, all: out.length }; };
const SIL = await import(`${ROOT}/src/game/silent.ts`);
const { harnessProfile, FROZEN_HOLDS } = await import(`${ROOT}/src/host/profile.ts`);

type Verdict = (ok: boolean, label: string, detail: string) => void;
function family(title: string): { verdict: Verdict; done: () => boolean } {
  console.log(`\n=== ${title} ===`); let all = true;
  return { verdict: (ok, label, detail) => { all &&= ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${label}\n    ${detail}`); }, done: () => { console.log(`=== ${title.split(':')[0]}: ${all ? 'ALL PASS' : 'FAILURES above'} ===`); return all; } };
}
const hushed = async <T>(fn: () => Promise<T> | T): Promise<T> => { const log = console.log, err = console.error, warn = console.warn; console.log = console.error = console.warn = () => { }; try { return await fn(); } finally { console.log = log; console.error = err; console.warn = warn; } };
function seeded(a: number) { return () => { a |= 0; a = (a + 0x6D2B79F5) | 0; let t = Math.imul(a ^ (a >>> 15), 1 | a); t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t; return ((t ^ (t >>> 14)) >>> 0) / 4294967296; }; }
const J = (o: unknown) => JSON.stringify(o), DT = 1 / 60, f2 = (x: number) => (Number.isFinite(x) ? x.toFixed(2) : String(x));
const STAND = harnessProfile() as string, BROKEN = process.env.BIGKIT_BREAK === '1', WEIGHT_BROKEN = process.env.BIGWEIGHT_BREAK === '1';
const memStore = (init: Record<string, string> = {}) => { const m = new Map(Object.entries(init)); return { m, getItem: (k: string) => m.get(k) ?? null, setItem: (k: string, v: string) => { m.set(k, String(v)); }, removeItem: (k: string) => { m.delete(k); } }; };
const fields = (i: number): string[] => K.kitRows(i).map((r: any) => r.field as string);
const rowOf = (i: number, f: string) => K.kitRows(i).find((r: any) => r.field === f);
/** how many big things a card holds, counted off its fields (never asked of the table under test) */
const bigThings = (m: any) => (m.walls | 0) + (m.big && m.big !== 'none' ? 1 : 0);
const stats = () => ({ ...BIG_STATS }) as Record<string, number>;
const since = (a: Record<string, number>) => Object.fromEntries(Object.entries(BIG_STATS as Record<string, number>).map(([k, v]) => [k, v - a[k]])) as Record<string, number>;
/** the kit table, its rules, the switch, the silent order's switch and the dice as they stood, put back after `fn` (the store the table writes to is left on nothing) */
async function kept<T>(fn: () => Promise<T> | T): Promise<T> {
  const cards = K.KIT.members.map((m: any) => ({ ...m })), on = K.KIT.on, slots = K.SLOTS.on, big = K.BIGSLOT.on, hush = SIL.SILENT.on, rnd = Math.random;
  try { return await fn(); } finally { K.kitLoad(null, ''); K.BIGSLOT.on = big; K.SLOTS.on = slots; K.KIT.on = on; for (let i = 0; i < cards.length; i++) K.KIT.members[i] = cards[i]; SIL.SILENT.on = hush; Math.random = rnd; }
}

// where Sam stands for the manager's door (his side of it is the side the squad works it from): the corridor, off the line of the opening
const SAM_MANAGER = [24.0, 0, 11.2];
/** the squad stood at Sam's feet, on free floor: the walk from the staging to the door is most of a door order's frames, and no line here is about it */
function closeUp(R: Rig, sam: number[]) { const nav = R.game.col.nav; R.ops.forEach((m: any, i: number) => { const a = i * 2.1 + 1, p = nav.nearestFreePoint(sam[0] + Math.cos(a) * 1.5, sam[2] + Math.sin(a) * 1.5) ?? sam; m.char.pos = [p[0], m.char.pos[1], p[2]]; m.char.vel = [0, 0, 0]; m.path = []; m.pathGoal = null; }); }
type Ev = { t: number; kind: string; weapon?: string; who?: number; loud: boolean };
/** One game under a seed, the switch as `on` says, `cards` written on the loadout
 *  cards BEFORE the staging — so the staging is what hands a man his tool — and the room clear staged (`stage: false` = the plain night, Sam alone).
 *  Sam in god mode and dark; every game event but footfalls kept, with its time. */
async function rig(seed: number, o: { cards?: Record<number, Record<string, unknown>>; stage?: boolean; on?: boolean } = {}) {
  Math.random = seeded(0x51b6 + seed * 7919);
  for (let i = 0; i < K.KIT_MEMBERS; i++) K.KIT.members[i] = K.kitDefault(i);   // (nothing of the game before this one: its cards)
  const W: any = await hushed(() => standUp()); const game = W.game; K.BIGSLOT.on = o.on ?? true;
  for (let i = 0; i < K.KIT_MEMBERS; i++) K.KIT.members[i] = { ...K.kitDefault(i), ...(o.cards?.[i] ?? {}) };
  game.godMode = true;
  const said: string[] = []; { const msg = game.msg.bind(game); game.msg = (t: string) => { said.push(String(t)); msg(t); }; }   // (every line the HUD is handed)
  const why = o.stage === false ? null : await hushed(() => game.roomClear.start(seed));
  const seen = new Set<any>();
  const R = { W, game, input: W.input, O: game.orders, D: game.doorTools, NB: game.nightBreach, why, said, ev: [] as Ev[], post: null as null | (() => void),
    get ops(): any[] { return game.nightBreach.ops; },
    step(n = 1) { for (let i = 0; i < n; i++) { game.player.visibility = 0.05; W.step(DT);
      for (const e of game.events) { if (seen.has(e)) continue; seen.add(e); if (e.kind !== 'step') R.ev.push({ t: e.time, kind: e.kind, weapon: e.weapon, who: e.who, loud: !!e.loud }); }
      R.post?.(); } },
    until(pred: () => boolean, secs: number) { for (let i = 0; i < secs * 60 && !pred(); i++) R.step(); return pred(); },
    door(name: string): any { const d = game.doors.list.find((x: any) => x.def.name === name); if (!d) throw new Error(`no door '${name}' in the level`); return d; },
    /** every beat of every door order so far, seconds after `t0` */
    beats(t0: number): string { return [...R.D.done, ...R.D.ops].flatMap((op: any) => op.beats).map((b: any) => `${f2(b.t - t0)} ${b.what}`).join(' | '); },
    evs(t0: number, kinds: string[]): string { return R.ev.filter(e => e.t >= t0 - 1e-6 && kinds.includes(e.kind)).map(e => `${f2(e.t - t0)} ${e.kind}${e.weapon ? `:${e.weapon}` : ''}`).join(', ') || 'none'; },
  };
  await hushed(() => R.step(30));
  return R;
}
type Rig = Awaited<ReturnType<typeof rig>>;
const rowsOf = (g: any): string[] => g.nearby.filter((x: any) => x.kind === 'door').map((x: any) => x.verb as string);
/** the face of d the leaf opens AWAY from (a boot there drives it) and the one it opens toward (where it rests on the frame's stop) */
const faceOf = (g: any, d: any, which: 'push' | 'pull'): 1 | -1 => { const pull = ([1, -1] as const).find(s => g.doors.pullFace(d, doorPt(d, d.def.width / 2, 0.75, s))) ?? 1; return (which === 'pull' ? pull : -pull) as 1 | -1; };
/** a leaf put home on its latch and locked against the man at `from`, by hand: `lockId` 'wedge:…' = a wedge under it (digin.ts), which no pick opens */
function lockAgainst(g: any, d: any, from: number[], lockId?: string) { d.angle = 0; d.vel = 0; d.latched = true; d.locked = true; d.lockBroken = false; d.keySide = g.doors.side(d, from); if (lockId) d.lockId = lockId; }

// ───────────────────────────────────────────────────────────── card
// (the sums the tool's times are made of, on the settled rules: the game's own went with the TypeScript tick, the numbers are the table's)
const bigEquip = (kind: 'hammer' | 'shotgun'): number => (kind === 'hammer' ? K.BIGSLOT.equipW : K.BIGSLOT.drawW);
export async function bigCard(): Promise<boolean> {
  const F = family('bigslot-card: one big slot a card — the shotgun off the weapon row, the hammer beside it, never two big things, never a pouch');
  await kept(async () => {
    try {
      await hushed(() => standUp());   // (the tables stood as the stand has them)
      
      K.BIGSLOT.on = true; K.kitLoad(null, '');
      const per = [0, 1, 2, 3, 4].map(i => { const f = fields(i), big = rowOf(i, 'big'), gun = rowOf(i, 'weapon'); return { f, big: (big?.values ?? []) as string[], gun: (gun?.values ?? []) as string[] }; });
      K.kitLoad(null, '?noslots'); const free = [0, 1].map(i => ({ f: fields(i), big: (rowOf(i, 'big')?.values ?? []) as string[], gun: (rowOf(i, 'weapon')?.values ?? []) as string[] }));

      // no step of the row leaves two big things on a man — walked twice round, both ways, on you and on a teammate
      K.kitLoad(null, '');
      let worst = 0, refused = 0; const walked: string[] = [];
      for (const i of [0, 1]) { const n = rowOf(i, 'big').values.length, seen = new Set<string>();
        for (const dir of [1, -1] as const) for (let k = 0; k < n * 2; k++) { const t = K.kitTry(i, 'big', dir); if (!t.ok) refused++; seen.add(String(t.value)); worst = Math.max(worst, bigThings(K.KIT.members[i]), K.kitCost(i).big?.used ?? 9); }
        walked.push(`${i}: ${[...seen].join(' → ')}`); if (seen.size !== n) worst = 9; K.kitSet(i, 'big', 'none'); }
      F.verdict(worst === 1 && refused === 0, 'stepped twice round, both ways, the row visits every one of its values, is never refused, and the card never holds two big things', `${walked.join(' · ')} · most big things on a card at any step ${worst} · refused ${refused}`);

      // a kit stored by the build before: the shotgun as a man's gun
      { const old = memStore({ [K.KIT_LS]: J({ v: 2, members: [0, 1, 2, 3, 4].map(i => (i <= 1 ? { ...K.kitDefault(i), weapon: 'shotgun' } : K.kitDefault(i))) }) });
        K.kitLoad(old, ''); const you = { ...K.KIT.members[0] }, mate = { ...K.KIT.members[1] }, legal = K.kitLegal(), words = K.kitWords(1);
        F.verdict(you.weapon === K.kitDefault(0).weapon && you.big === 'shotgun' && mate.weapon === K.kitDefault(1).weapon && mate.big === 'shotgun' && K.kitGun(mate) !== 'm870' && legal === null && /shotgun/.test(words),
          'a stored card whose GUN is the shotgun (the build before\'s; yours and a teammate\'s) is read as the issued gun with the shotgun in the big slot', `you: weapon ${you.weapon}, big ${you.big} · Red One: weapon ${mate.weapon} (${K.kitGun(mate)}), big ${mate.big} · legal ${legal} · "${words}"`); }
      // … and one over the big slot
      // (the wall charge is the last thing a man over his big slot gives up — the card shows it before the rest, kit.ts bigOf: beside a hammer it is the hammer that stays home)
      { const walls = rowOf(1, 'big').values.includes('wall');
        const st = memStore({ [K.KIT_LS]: J({ v: 2, members: [0, 1, 2, 3, 4].map(i => (i === 1 ? { ...K.kitDefault(1), walls: 1, big: 'hammer' } : K.kitDefault(i))) }) });
        K.kitLoad(st, ''); const m = { ...K.KIT.members[1] }, note = K.KIT_NOTE[1], c = K.kitCost(1);
        F.verdict(bigThings(m) === 1 && c.big?.used === 1 && K.kitLegal() === null && (walls ? m.walls === 1 && m.big === 'none' && K.bigOf(m) === 'wall' && /hammer/.test(note) && /left behind/.test(note) && /big slot/.test(note) && !/wall charge/.test(note) : m.walls === 0 && m.big === 'hammer' && note === ''),
          walls ? 'a stored teammate\'s card with a wall charge AND a hammer comes down to one big thing: it keeps the WALL CHARGE and leaves the hammer behind, and the card says so in words'
            : 'a stored teammate\'s card with a wall charge AND a hammer, in a build with no wall charge row: the wall charge is not read, the hammer is his one big thing, and nothing is said', `kept: walls ${m.walls}, big ${m.big} · big slot ${J(c.big)} · note: "${note}"`); }
      // … and two of the same big thing: one stays
      { const walls = rowOf(1, 'big').values.includes('wall');
        const st = memStore({ [K.KIT_LS]: J({ v: 2, members: [0, 1, 2, 3, 4].map(i => (i === 1 ? { ...K.kitDefault(1), walls: 2 } : K.kitDefault(i))) }) });
        K.kitLoad(st, ''); const m = { ...K.KIT.members[1] }, note = K.KIT_NOTE[1], c = K.kitCost(1);
        F.verdict(K.kitLegal() === null && (walls ? bigThings(m) === 1 && m.walls === 1 && c.big?.used === 1 && /1 wall charge left behind/.test(note) && /big slot/.test(note) : m.walls === 0),
          walls ? 'a stored teammate\'s card with TWO wall charges (the build before\'s most a man) keeps one, and the card says the other stayed behind' : 'two stored wall charges in a build with no wall charge row: neither is read', `kept: walls ${m.walls}, big ${m.big} · big slot ${J(c.big)} · note: "${note}"`); }
      // a hand that still names the rows of the build before (a tool, a tally) is answered through the big row
      { K.kitLoad(null, ''); const gun0 = K.kitDefault(1).weapon, walls = rowOf(1, 'big').values.includes('wall'), was = (i: number) => `${K.bigOf(K.KIT.members[i])}`;
        const b = K.kitSet(1, 'weapon', 'shotgun'), b1 = was(1), bGun = K.KIT.members[1].weapon, bMost = bigThings(K.KIT.members[1]);
        const c = walls ? K.kitSet(1, 'walls', 1) : true, c1 = walls ? was(1) : 'wall', cMost = bigThings(K.KIT.members[1]);
        const d = walls ? K.kitSet(1, 'walls', 0) : true, d1 = walls ? was(1) : 'none';
        const e = K.kitSet(1, 'weapon', 'shotgun') && K.kitSet(1, 'weapon', gun0), e1 = was(1), eGun = K.KIT.members[1].weapon;
        const y = K.kitSet(0, 'weapon', 'shotgun'), y1 = was(0), yGun = K.KIT.members[0].weapon; K.kitSet(0, 'big', 'none'); K.kitSet(1, 'big', 'none');
        F.verdict(b && b1 === 'shotgun' && bGun === gun0 && bMost === 1 && c && c1 === 'wall' && cMost === 1 && d && d1 === 'none' && e && e1 === 'shotgun' && eGun === gun0 && y && y1 === 'shotgun' && yGun === K.kitDefault(0).weapon && K.kitLegal() === null,
          `the switch on, kitSet by the build before's names — the weapon 'shotgun'${walls ? ", 'walls' 1" : ' (this build has no wall charge row: \'walls\' not asked)'} — is answered through the big row: one big thing at a time (the wall charge takes the shotgun's place), and the weapon stays the issued gun — set back to it by name, the shotgun stays where the big row has it`,
          `Red One: weapon shotgun → ${b} big ${b1}, weapon ${bGun} · walls 1 → ${c} big ${c1} · walls 0 → ${d} big ${d1} · weapon shotgun, then the issued gun → ${e} big ${e1}, weapon ${eGun} · you: weapon shotgun → ${y} big ${y1}, weapon ${yGun}`); }

      // the issued kit and the table's squad kits
      { K.kitLoad(null, ''); const issued = K.kitLegal(), list = K.kitLoadouts() as { id: string; name: string }[];
        const each = list.map(l => { const w = K.kitLoadout(l.id), legal = K.kitLegal(), most = Math.max(...K.KIT.members.map(bigThings)); return { id: l.id, w, legal, most }; });
        F.verdict(issued === null && list.length >= 2 && each.every(e => e.w === null && e.legal === null && e.most <= 1) && K.SLOTS.table.v === 10,
          'the issued kit and every squad kit the screen can put on are legal fills under the big slot (slots.json v10), no man of them with two big things', `issued: ${issued === null ? 'legal' : issued} · ${each.map(e => `${e.id}: ${e.w ?? 'put on'}, ${e.legal ?? 'legal'}, most big things ${e.most}`).join(' · ')} · table v${K.SLOTS.table.v}`); }

      // a big thing fills the slot and no pouch — yours on a card whose four pouches are full
      { K.kitLoad(null, ''); const lines: string[] = []; let ok = true;
        for (const i of [0, 1]) for (const v of ['shotgun', 'hammer']) { K.kitSet(i, 'big', 'none'); const c0 = K.kitCost(i), set = K.kitSet(i, 'big', v), c1 = K.kitCost(i), price = K.kitRowCost(i, rowOf(i, 'big'));
          ok &&= set && c0.big?.used === 0 && c0.big.total === 1 && c1.big?.used === 1 && c1.big.total === 1 && c1.used === c0.used && c1.total === c0.total && !price?.pouches && K.kitLegal() === null;
          lines.push(`${i} ${v}: set ${set} · pouches ${c0.used} of ${c0.total} → ${c1.used} of ${c1.total} · big ${J(c0.big)} → ${J(c1.big)}`); }
        const full = K.kitCost(0); K.kitSet(0, 'big', 'none'); K.kitSet(1, 'big', 'none');
        F.verdict(ok && full.used === full.total, 'the shotgun or the hammer on a card reads 1 of 1 in its big slot and leaves its pouches as they were — yours taken with all four pouches full', lines.join('\n    ')); }

      // a table from before the big slot (no tool marked big, no member given a slot): the three things the slot is for are big by default
      { const RAW = JSON.parse(readFileSync(resolve(ROOT, 'src/game/slots.json'), 'utf8')), strip = <T extends object>(o: T) => { const { big: _big, ...rest } = o as T & { big?: unknown }; return rest; };
        const old = { ...RAW, members: RAW.members.map(strip), tools: RAW.tools.map(strip) };
        K.kitLoad(null, ''); let why: string | null = 'not asked', marks = '', slotsOf = '', cost: { used: number; big: unknown } | null = null, back: string | null = 'not put back';
        try { why = K.slotsSet(old); const T = K.SLOTS.table; marks = ['walls', 'shotgun', 'hammer'].map(k => `${k} ${T.items[k] ? !!T.items[k].big : 'no such tool'}`).join(', '); slotsOf = J(T.big);
          K.kitSet(1, 'big', 'hammer'); const c = K.kitCost(1); cost = { used: c.used, big: c.big }; K.kitSet(1, 'big', 'none'); }
        finally { back = K.slotsSet(RAW); }
        const issued = K.kitCost(1, K.kitDefault(1)).used, T = K.SLOTS.table;
         }

      // the switch off: the build before's rows, and `big` inert
      { K.BIGSLOT.on = false; K.kitLoad(null, '');
        const f = fields(1), walls = f.includes('walls'), want = ['weapon', 'suppressed', 'light', 'nvg', 'bangs', 'smoke', 'charges', ...(walls ? ['walls'] : []), 'frags', ...(f.includes('wand') ? ['wand'] : []) /* (a teammate's optical wand: a row of his card while WAND.mate is on, no affair of the big slot's) */, 'suit', 'trim'];
        const guns = [0, 1].map(i => (rowOf(i, 'weapon').values as string[]).join('/')), cost = K.kitCost(1);
        F.verdict(f.join() === want.join() && guns.every(g => g === 'rifle/pistol/shotgun') && !fields(0).includes('big') && cost.big === null,
          'THE SWITCH OFF: a teammate\'s rows are the build before\'s to the row, the weapon row offers the shotgun (yours too), and no card has a big row or a big slot to count', `Red One: ${f.join(' ')} · weapon rows ${guns.join(' · ')} · you: ${fields(0).join(' ')} · kitCost(1).big ${J(cost.big)}`);
        const st = memStore({ [K.KIT_LS]: J({ v: 2, members: [0, 1, 2, 3, 4].map(i => (i === 1 ? { ...K.kitDefault(1), big: 'hammer' } : i === 2 ? { ...K.kitDefault(2), weapon: 'shotgun' } : K.kitDefault(i))) }) });
        K.kitLoad(st, ''); const off = { ...K.KIT.members[1] }, issued = K.kitCost(1, K.kitDefault(1)).used, used = K.kitCost(1).used, words = K.kitWords(1);
        const wrote = K.kitSet(1, 'light', !off.light), stored = JSON.parse(st.m.get(K.KIT_LS) ?? '{}').members?.[1]?.big;
        K.BIGSLOT.on = true; K.kitLoad(memStore(Object.fromEntries(st.m)), ''); 
         }
    } catch (e) { F.verdict(false, 'the check ran', String((e as Error)?.stack ?? e)); }
  });
  return F.done();
}

/** an entry order on d as the menu's row gives it — and, where the stand halts an entry at a door found shut (DOORTOOLS.testDoor, the page's: "your call"),
 *  the same row again, which is the order that takes the lock. Stepped until the lock is off, the order is refused or over, or `cap` seconds. */
function enter(R: Rig, d: any, cap = 45) {
  const give = (): string | null => R.O.give('openClear', R.O.targetAt(d.frameCentre)), last = () => R.D.done[R.D.done.length - 1];
  const t0 = R.game.time; let w = give(), again = false, tGive = t0;
  R.until(() => { if (!d.locked || typeof w === 'string') return true; if (R.D.ops.length || R.game.time - tGive < 0.5) return false;
    if (!again && last()?.ended === 'found shut') { again = true; w = give(); tGive = R.game.time; return false; } return true; }, cap);
  return { t0, w, again, gone: !d.locked && !!d.lockBroken, secs: d.locked ? NaN : R.game.time - t0, ended: R.D.ops.length ? '' : String(last()?.ended ?? '') };
}


// ───────────────────────────────────────────────────────────── you
/** the plain night, the tool on your card and put on you by the card (kitSolo), you at arm's length off the middle of `door`'s leaf on `face`, the row
 *  performed and three seconds given it. `lock`: a wedge put under the leaf by hand. */
async function you(seed: number, tool: 'shotgun' | 'hammer', face: 'push' | 'pull', o: { door?: string; lock?: string; open?: boolean } = {}) {
  const R = await rig(seed, { stage: false, cards: { 0: { big: tool } } }), g = R.game, pl = g.player, d = R.door(o.door ?? 'storage'); g.aiEnabled = false;
  const verb = tool === 'shotgun' ? 'breach' : 'hammer';
  // (a steel door: the face its lock is against, as the level keys it — a boot rings off either)
  const steel = !!d.def.steel, side = steel ? ([1, -1] as const).find(s => g.doors.side(d, doorPt(d, d.def.width / 2, 0.75, s)) === d.keySide) ?? 1 : faceOf(g, d, face), spot = doorPt(d, d.def.width / 2, 0.75, side);
  const keyed = !o.open && (!g.doors.lockedOut(d, spot, pl.keys) || !!o.lock); if (keyed) lockAgainst(g, d, spot, o.lock);
  if (o.open) { d.angle = 0; d.vel = 0; d.latched = true; d.locked = false; d.lockBroken = false; }   // (`open`: shut on its latch and NOT locked, by hand)
  g.teleportPlayer(spot); R.step(12); const bare = rowsOf(g);
  K.kitSolo(g); R.step(3); const carried = pl.bigKit as string, rows = rowsOf(g);
  const out = g.doors.lockedOut(d, spot, pl.keys) as boolean, held = g.doors.bootHolds(d, spot) as boolean, offered = rows.includes('breach') ? 'shotgun' : rows.includes('hammer') ? 'hammer' : null;
  const k = bare.indexOf('kick'), inPlace = rows.includes(verb) && !rows.includes('kick') && (k >= 0 ? rows.length === bare.length && rows.every((v, i) => (i === k ? v === verb : v === bare[i])) : rows.filter(v => v !== verb).join() === bare.join());
  const st = stats(), t0 = g.time, said0 = R.said.length, blows = BK.hammerBlows(g, d, spot) as number, picked = g.selectVerb('door', verb); if (picked) { R.input.pressed.add('Space'); R.step(); }
  const began = pl.kick?.tool?.kind === tool, need = (pl.kick?.tool?.need ?? NaN) as number, gunLead = (pl.kick?.tool?.lead ?? 0) as number;   // (BIGSLOT.weight: his own gun put away by hand first)
  // where he stands when the first blow lands or the first shell goes, as metres along the leaf from its hinge (0‥width = in the opening)
  let at: number[] | null = null; R.post = () => { if (!at && BIG_STATS.samBlows + BIG_STATS.samShells > st.samBlows + st.samShells) at = [...pl.char.pos]; };
  const given = () => (o.open ? !d.latched || !!d.lockBroken : !d.locked);
  R.until(given, 3); const secs = given() ? g.time - t0 : NaN; R.until(() => !pl.kick, 2); R.step(6); R.post = null;
  const p0 = doorPt(d, 0, 0, side), p1 = doorPt(d, 1, 0, side), along = at ? (at[0] - p0[0]) * (p1[0] - p0[0]) + (at[2] - p0[2]) * (p1[2] - p0[2]) : NaN, beside = along < -0.05 || along > d.def.width + 0.05;
  const home0 = d.isClosed() as boolean, angle0 = d.angle as number; if (!home0) R.until(() => Math.abs(d.angle) > 0.5, 2); const endsOpen = Math.abs(d.angle) > 0.5;
  const res = { R, g, d, keyed, steel, carried, bare, rows, out, held, offered, inPlace, picked, began, need, gunLead, blows, secs, gone: !d.locked && !!d.lockBroken, st: since(st), t0,
    shots: R.ev.filter(e => e.t >= t0 && (e.kind === 'shot' || e.kind === 'guardShot')), kicks: R.ev.filter(e => e.t >= t0 && e.kind === 'kick').length,
    thuds: R.ev.filter(e => e.t >= t0 && e.kind === 'door' && e.loud && e.t < t0 + (Number.isFinite(secs) ? secs : 3) - 1e-6).length, said: R.said.slice(said0),
    home: home0, angle: endsOpen ? d.angle as number : angle0, after: rowsOf(g), hand: '', opened: false, along, beside, endsOpen };
  if (face === 'pull' && !steel && home0 && !o.open) { const hand = res.after.find(v => v === 'pull' || v === 'push'); if (hand && g.selectVerb('door', hand)) { res.hand = hand; R.input.pressed.add('Space'); R.step(); R.until(() => Math.abs(d.angle) > 0.5, 2); res.opened = Math.abs(d.angle) > 0.5; } }
  return res;
}

// ───────────────────────────────────────────────────────────── a teammate
export async function bigMate(): Promise<boolean> {
  const F = family('bigslot-mate: a teammate\'s big slot on a door order — the shotgun up for the lock and slung after, the hammer\'s blows');
  await kept(async () => {
    try {
      const SEED = 8;   // (the seed tools/qa/shotgun-probes.ts takes this door on: the pair lives to the lock either way)
      // the shotgun: his rifle his own, the M870 up for the lock, his rifle back
      { const R = await rig(SEED, { cards: { 1: { big: 'shotgun' } } }), g = R.game, d = R.door('manager'), [gd, other] = R.ops; g.teleportPlayer(SAM_MANAGER); closeUp(R, SAM_MANAGER); R.step(5);
        const own = gd.weapon.def.id as string, big = K.kitBig(gd) as string, bigOther = K.kitBig(other) as string, locked = d.locked && d.latched, st = stats();
        let tOut = NaN, inHand = '';
        R.post = () => { if (isNaN(tOut) && gd.slung?.out) { tOut = g.time; inHand = gd.weapon.def.id; } };
        const e = enter(R, d), tGone = g.time; R.step(15); R.post = null;
        const now = gd.weapon.def.id as string, out = !!gd.slung?.out, by = st, s = since(by), shell = R.ev.filter(x => x.t >= e.t0 && x.kind === 'guardShot' && x.weapon === 'm870' && x.who === gd.char.id).length;
        F.verdict(R.why === null && locked && own !== 'm870' && gd.route.weapon !== 'm870' && big === 'shotgun' && bigOther === 'none' && g.player.bigKit === 'none',
          'A TEAMMATE, THE SHOTGUN: staged with it in his card\'s big slot he holds his own gun — the shotgun is kit (kitBig), and nobody else carries one', `${gd.callsign}: ${own}, big slot ${big} · ${other.callsign}: ${other.weapon.def.id}, big slot ${bigOther} · you: ${g.player.bigKit}`);
        F.verdict(e.w === null && s.drawn === 1 && inHand === 'm870' && tGone - tOut >= bigEquip('shotgun') - DT, `open and clear on the locked manager's door${e.again ? ' (the stand halts an entry at a door found shut: the row given again, as the lead\'s "your call" asks)' : ''}: he brings the M870 up off its sling, and no shell goes before it has been up its beat`, `give: ${e.w ?? 'taken'} · drawn ${s.drawn} · in his hands ${inHand} from ${f2(tOut - e.t0)} s · the lock went ${f2(tGone - tOut)} s later (the beat ${f2(bigEquip('shotgun'))})`);
        F.verdict(e.gone && shell >= 1 && R.D.stats.kicks === 0, 'the lock is gone, shot off — his shell, and no boot', `lock off ${!d.locked}, keep broken ${!!d.lockBroken} at ${f2(e.secs)} s · his m870 shells heard ${shell} · door tools: shot ${R.D.stats.shot}, kicks ${R.D.stats.kicks}${BROKEN ? ' · BIGKIT_BREAK=1' : ''} · ${R.beats(e.t0)}`);
        F.verdict(e.gone && now === own && !out && gd.route.weapon !== 'm870' && s.slung === 1, 'and a quarter of a second after it his own gun is in his hands again, the shotgun slung', `now ${now} (his ${own}), out ${out}, slung ${s.slung}`); }
      // another order given with the gun up
      { const R = await rig(SEED, { cards: { 1: { big: 'shotgun' } } }), g = R.game, d = R.door('manager'), gd = R.ops[0]; g.teleportPlayer(SAM_MANAGER); closeUp(R, SAM_MANAGER); R.step(5);
        const own = gd.weapon.def.id as string, target = () => R.O.targetAt(d.frameCentre), last = () => R.D.done[R.D.done.length - 1];
        /** the row given (again where the stand halts at a door found shut) and stepped until the gun is up or the lock is off */
        const up = () => { let w = R.O.give('openClear', target()), again = false, t = g.time;
          R.until(() => { if (gd.slung?.out || !d.locked || typeof w === 'string') return true; if (!R.D.ops.length && g.time - t > 0.5 && !again && last()?.ended === 'found shut') { again = true; w = R.O.give('openClear', target()); t = g.time; } return false; }, 45);
          return !!gd.slung?.out && gd.weapon.def.id === 'm870' && d.locked; };
        const up2 = up(); const w = R.O.give('fallIn', target()); R.step(2); const b = { gun: gd.weapon.def.id as string, out: !!gd.slung?.out, route: gd.route.weapon as string };
        R.step(60); const still = d.locked && gd.weapon.def.id === own;
        F.verdict(up2 && w === null && b.gun === own && !b.out && b.route !== 'm870' && still, 'the same door again, and ANOTHER ORDER given him with it up (fall in): two frames on, his own gun; the lock was never shot', `up ${up2} · fall in: ${w ?? 'taken'} · ${b.gun}, out ${b.out}, route ${b.route} · a second later: locked ${d.locked}, ${gd.weapon.def.id}`); }
      // the hammer
      
      // the face the leaf opens toward. The break room's north door from inside the room: shut and locked against the squad by hand — and the room's
      // defenders taken off the floor first (the stack would stand among them: the fight is the room fight's dice, not this check's)
      { const pull = async (tool: string, lockId?: string, silent = false) => {
          const R = await rig(SEED, { cards: tool === 'none' ? {} : { 1: { big: tool } } }), g = R.game, d = R.door('breakroom_n'), gd = R.ops[0];
          await hushed(() => { for (const h of R.NB.hostiles) if (h.state !== 'dead') killGuard(g, h, [1, 0, 0], true); R.step(60); });
          const sam = doorPt(d, d.def.width + 1.2, 1.4, faceOf(g, d, 'pull')); lockAgainst(g, d, sam, lockId); g.teleportPlayer(sam); closeUp(R, sam); R.step(5);
          const switched = silent && !SIL.SILENT.on; if (silent) { SIL.SILENT.on = true; SIL.setSilent(g, true); R.step(2); }   // (the squad's silent order: its switch is a run's own, raised by hand where it is not up)
          const quiet = SIL.squadSilent(g) as boolean, held = g.doors.bootHolds(d, sam) as boolean, st = stats(), e = enter(R, d); R.until(() => Math.abs(d.angle) > 0.5, 4); R.step(15);
          return { R, d, gd, switched, quiet, held, e, refused: typeof e.w === 'string' ? e.w : e.ended, s: since(st), open: Math.abs(d.angle) > 0.5, gun: gd.weapon.def.id as string, out: !!gd.slung?.out, shove: /shove|pull/.test(R.beats(e.t0)), kicks: R.D.stats.kicks as number, picks: R.D.stats.picks as number };
        };
        const none = await pull('none', 'wedge:breakroom_n'), ham = await pull('hammer', 'wedge:breakroom_n'), gun = await pull('shotgun');
        const hand = `set BY HAND: the leaf shut and locked against the squad from inside the room, the room's defenders taken off the floor`;
        F.verdict(none.held && none.d.locked && !none.open && (typeof none.e.w === 'string' ? /no boot/.test(none.e.w) : /no boot/.test(none.e.ended)) && Object.values(none.s).every(n => n === 0),
          'A DOOR LOCKED ON THE FACE IT OPENS TOWARD, wedged (no pick opens it), nobody carrying a tool: the entry is refused as it always was', `${hand} · the frame holds a boot ${none.held} · give: ${none.e.w ?? 'taken'} · ended: ${none.e.ended || '-'} · still locked ${none.d.locked}`);
        F.verdict(ham.held && ham.e.w === null && ham.e.gone && ham.s.mateBlows === K.BIGSLOT.swings && K.BIGSLOT.swings === 2 && /hammer: a blow/.test(ham.R.beats(ham.e.t0)) && ham.s.mateHammered === 1 && ham.kicks === 0,
          'the same door with a hammer in the stack is NOT refused: two blows (BIGSLOT.swings, where the frame holds a boot) and the lock is gone', `give: ${ham.e.w ?? 'taken'} · lock off ${!ham.d.locked}, broken ${!!ham.d.lockBroken} at ${f2(ham.e.secs)} s · blows ${ham.s.mateBlows}, hammered ${ham.s.mateHammered}${BROKEN ? ' · BIGKIT_BREAK=1' : ''} · ${ham.R.beats(ham.e.t0)}`);
        F.verdict(ham.e.gone && ham.open, '… and the leaf, left home in its frame by the blow, is then opened by hand and the entry goes on', `angle ${f2(ham.d.angle)} · open ${ham.open}`);
        F.verdict(gun.held && gun.e.w === null && gun.e.gone && gun.s.drawn === 1 && gun.s.slung === 1 && gun.picks === 0 && gun.open && gun.gun !== 'm870' && !gun.out,
          'a keyed lock on that face with the SHOTGUN in the stack: shot off from the face the leaf opens toward (no pick), the leaf pulled by hand, his own gun back', `give: ${gun.e.w ?? 'taken'} · lock off ${!gun.d.locked}, broken ${!!gun.d.lockBroken} at ${f2(gun.e.secs)} s · drawn ${gun.s.drawn}, slung ${gun.s.slung} · picks ${gun.picks} · open ${gun.open} · now ${gun.gun}${BROKEN ? ' · BIGKIT_BREAK=1' : ''} · ${gun.R.beats(gun.e.t0)}`); }
    } catch (e) { F.verdict(false, 'the check ran', String((e as Error)?.stack ?? e)); }
  });
  return F.done();
}

// ───────────────────────────────────────────────────────────── any shut door, from beside the frame
/** the room clear, `tool` on the first teammate's card, and the big slot's own row on a door: the manager's from the corridor (the face the leaf opens AWAY
 *  from) or the break room's north door from inside the room (the face it opens TOWARD — its defenders taken off the floor first, as bigMate does). The door
 *  NOT locked unless `lock` says so (by hand either way). `key`: the row given as the page gives it — the menu opened on the door, then H; else by its id
 *  (`order`: another row's). Stepped until the door stands open, the order is refused or over. */
async function mateShut(tool: 'none' | 'hammer' | 'shotgun', o: { face?: 'push' | 'pull'; lock?: boolean | string; order?: string; silent?: boolean; /** called after every frame from the order on (the moves' family) */ watch?: (R: Rig, gd: any, d: any) => void } = {}) {
  const R = await rig(8, { cards: tool === 'none' ? {} : { 1: { big: tool } } }), g = R.game, pullFace = o.face === 'pull', d = R.door(pullFace ? 'breakroom_n' : 'manager'), gd = R.ops[0];
  if (pullFace) await hushed(() => { for (const h of R.NB.hostiles) if (h.state !== 'dead') killGuard(g, h, [1, 0, 0], true); R.step(60); });
  const sam = pullFace ? doorPt(d, d.def.width + 1.2, 1.4, faceOf(g, d, 'pull')) : SAM_MANAGER;
  if (o.lock) lockAgainst(g, d, sam, typeof o.lock === 'string' ? o.lock : undefined); else { d.angle = 0; d.vel = 0; d.latched = true; d.locked = false; d.lockBroken = false; }
  g.teleportPlayer(sam); closeUp(R, sam); R.step(5);
  const switched = !!o.silent && !SIL.SILENT.on; if (o.silent) { SIL.SILENT.on = true; SIL.setSilent(g, true); R.step(2); }
  const st = stats(), t0 = g.time, target = () => R.O.targetAt(d.frameCentre), id = o.order ?? 'bigClear';
  let w: string | null = null;
  w = R.O.give(id, target());
  const big = R.D.ops.some((op: any) => op.big);
  if (o.watch) R.post = () => o.watch!(R, gd, d);
  R.until(() => typeof w === 'string' || Math.abs(d.angle) > 0.5 || (!R.D.ops.length && g.time - t0 > 0.5), 30); const secs = g.time - t0; R.step(o.watch ? 80 : 20); R.post = null;
  const beats = R.beats(t0), last = R.D.done[R.D.done.length - 1], trips = (beats.match(/(?:^|\| )[\d.]+ (?:hammer|gun):/g) ?? []).length;
  return { R, g, d, gd, switched, w, big, secs, beats, trips, s: since(st), open: Math.abs(d.angle) > 0.5, broken: !!d.lockBroken, locked: d.locked as boolean, ended: R.D.ops.length ? '' : String(last?.ended ?? ''),
    kicks: R.D.stats.kicks as number, gun: gd.weapon.def.id as string, out: !!gd.slung?.out, shell: R.ev.filter(x => x.t >= t0 && x.kind === 'guardShot' && x.weapon === 'm870').length, mateKick: R.ev.filter(x => x.t >= t0 && x.kind === 'kick' && x.who === gd.char.id).length };
}
export async function bigShut(): Promise<boolean> {
  const F = family('bigslot-shut: the tools open ANY shut door, from beside the frame, and the door ends open — yours on the door\'s rows, the squad\'s on H');
  await kept(async () => {
    try {
      const B = BROKEN ? ' · BIGKIT_BREAK=1' : '';
      // A TEAMMATE: the squad's own row
      { const r = await mateShut('hammer');
        F.verdict(r.w === null && r.big, 'A TEAMMATE, THE HAMMER: given on the (unlocked) manager\'s door, the order is taken', `give ${r.w} · a door order ${r.big}`);
        F.verdict(r.open && r.broken && r.s.mateBlows === 1 && r.s.mateHammered === 1 && r.kicks === 0 && r.trips === 1 && r.shell === 0 && r.mateKick >= 1, 'he takes the door with ONE blow from beside the frame — loud as the boot, no shot, no boot after it — and the entry goes in', `open ${r.open} after ${f2(r.secs)} s · latch broken ${r.broken} · blows ${r.s.mateBlows}, hammered ${r.s.mateHammered} · door tools' kicks ${r.kicks}${B} · ${r.beats}`); }
      
      { const r = await mateShut('hammer', { face: 'pull' });
        F.verdict(r.w === null && r.open && r.broken && r.s.mateBlows === 1 && r.s.mateHammered === 1 && r.trips === 1, 'THE FACE THE LEAF OPENS TOWARD, unlocked (the break room\'s north door from inside, set by hand): one blow, the leaf left home — and a hand pulls it; the tool is not sent to it a second time', `give: ${r.w ?? 'taken'} · open ${r.open} after ${f2(r.secs)} s · blows ${r.s.mateBlows} · trips to the door with the tool ${r.trips}${B} · ${r.beats}`); }
      
      
      { const none = await mateShut('none'), quiet = await mateShut('hammer', { silent: true });
        F.verdict(typeof none.w === 'string' && !none.big && none.s.mateBlows === 0, 'NOBODY CARRIES ONE: the order is refused', `give ${none.w}`);
        // (silent.ts: the row is one of LOUD_ROWS, as bang and clear and the charge are — picking it IS the squad going loud)
        F.verdict(quiet.w === null && !SIL.squadSilent(quiet.g) && quiet.s.mateHammered === 1 && quiet.open && (SIL.LOUD_ROWS as string[]).includes('bigClear'),
          'the squad told SILENT: the row is a LOUD tool (silent.ts LOUD_ROWS) — picking it is the squad going loud, and the hammer takes the door',
          `${quiet.switched ? 'the silent order\'s switch raised BY HAND · ' : ''}give: ${quiet.w ?? 'taken'} · silent after ${SIL.squadSilent(quiet.g)} · hammered ${quiet.s.mateHammered} · open ${quiet.open}${B}`);
        { const HF = await import(`${ROOT}/src/game/holdfire.ts`); F.verdict((HF.LIFT_ORDERS as string[]).includes('bigClear'), 'and it is an order that means fire (holdfire.ts LIFT_ORDERS), as open and clear is', `LIFT_ORDERS: ${(HF.LIFT_ORDERS as string[]).join(' ')}`); }
        // a wedge nobody has tried, only the shotgun to hand: the menu does not read the wedge (an entry halts at a door found shut)
         }
    } catch (e) { F.verdict(false, 'the check ran', String((e as Error)?.stack ?? e)); }
  });
  return F.done();
}

// ───────────────────────────────────────────────────────────── wiring
export async function bigWiring(): Promise<boolean> {
  const F = family('bigslot-wiring: the switch is a row of the two stands, a knob table and a table of the journal');
  await kept(async () => {
    try {
      const W: any = await hushed(() => standUp()), g = W.game, stood = K.BIGSLOT.on as boolean, env = process.env.TPS_BIGSLOT;
      const want = STAND === 'frozen' ? env === '1' : env === undefined ? true : env === '1';
      F.verdict(stood === want, `the ${STAND} stand stands a game up with the switch ${want ? 'ON' : 'OFF'}${env !== undefined ? ` (TPS_BIGSLOT=${env})` : STAND === 'frozen' ? ' (held off: TPS_BIGSLOT=1 raises it)' : ' (as the page opens: TPS_BIGSLOT=0 holds it off)'}`, `BIGSLOT.on after the stand-up: ${stood}`);
      const s = (await import(`${ROOT}/src/host/standEnv.ts`)).STAND_ENV as string[], KN = await import(`${ROOT}/src/game/knobs.ts`), R = KN.knobRegistry(), JN = (KN.JOURNAL_KNOBS as readonly string[]).includes('BIGSLOT');
      F.verdict(s.includes('TPS_BIGSLOT') && FROZEN_HOLDS['BIGSLOT.on'] === 'TPS_BIGSLOT=1' && R.BIGSLOT === K.BIGSLOT && JN, 'the switch is a row of the two stands\' list, has its word, is in the knob registry and among the tables a recording carries', `STAND_ENV ${s.includes('TPS_BIGSLOT')} · FROZEN_HOLDS ${FROZEN_HOLDS['BIGSLOT.on']} · registry ${R.BIGSLOT === K.BIGSLOT} · JOURNAL_KNOBS ${JN}`);
      const JR = await import(`${ROOT}/src/game/journal.ts`);
      K.BIGSLOT.on = true; const own = JR.knobsSnapshot(g), carried = (own.tables as any).BIGSLOT?.on; const old = JR.knobsSnapshot(g); delete (old.tables as any).BIGSLOT;
      await hushed(() => JR.applyKnobs(g, old)); const without = K.BIGSLOT.on as boolean;
      await hushed(() => JR.applyKnobs(g, own)); const withIt = K.BIGSLOT.on as boolean;
      F.verdict(carried === true && without === false && withIt === true, 'a recording made with the switch on carries it and replays with it on; one from before the table replays with it OFF', `the snapshot's table says ${carried} · applied with no BIGSLOT table: ${without} · applied with its own: ${withIt}`);
      const T = K.SLOTS.table, bigs = Object.entries(T.items as Record<string, { big?: boolean }>).filter(([, it]) => it.big).map(([k]) => k).sort();
      F.verdict(T.v === 10 && T.big.length === K.KIT_MEMBERS && T.big.every((n: number) => n === 1) && bigs.join() === ['hammer', 'shotgun', ...(T.items.walls ? ['walls'] : [])].join(), 'slots.json is v10: one big slot a man, and the shotgun and the hammer marked big (the wall charge with them where the build prices one) and nothing else', `v${T.v} · big slots ${J(T.big)} · big things: ${bigs.join(', ')}`);
    } catch (e) { F.verdict(false, 'the check ran', String((e as Error)?.stack ?? e)); }
  });
  return F.done();
}

// ───────────────────────────────────────────────────────────── moves
/** BIGSLOT.moves, YOU: the plain night's storage door, shut and NOT locked (by hand), the tool on your card; you stood `from` = [metres along the frame line
 *  from the hinge, metres off the wall] on `face`; `man`: a guard of the floor stood BY HAND on the latch jamb (where a stack's lead stands — the plain
 *  night has no teammate). The row taken if it is there, and every frame of the beat watched. */
export async function walkTo(seed: number, tool: 'shotgun' | 'hammer', face: 'push' | 'pull', from: [number, number], o: { man?: boolean; /** a second man of the file, stood by hand at [along, off] */ second?: [number, number]; /** not read since 2026-11-07 (the weight is a settled rule): here until tools/qa/swap-times.ts and bigmoves-strip.ts stop passing it */ noweight?: boolean; front?: boolean; /** what is in his hands as the row is taken (a digit pressed and its swap let finish) */ hand?: 'pistol' | 'rifle'; /** called after every frame from the row taken until his gun is up again (tools/qa/bigmoves-strip.ts draws them) */ each?: (g: any, sinceRow: number) => void; /** two frames given the kit's own hand before the gun is brought up (once: that hand raised by name) */ kit?: boolean } = {}) {
  const R = await rig(seed, { stage: false, cards: { 0: { big: tool } } }), g = R.game, pl = g.player, d = R.door('storage'), W = d.def.width as number; g.aiEnabled = false;
  const verb = tool === 'shotgun' ? 'breach' : 'hammer', side = faceOf(g, d, face);
  d.angle = 0; d.vel = 0; d.latched = true; d.locked = false; d.lockBroken = false;
  const spot = doorPt(d, from[0], from[1], side), floor = !g.col.nav.isBlocked(spot[0], spot[2]);
  const fc = (p: number[]) => ({ along: alongOf(d, p), off: offOf(d, p, side) as number });
  // (the stand-ins are guards of the floor, who shuffle and work a leaf they stand by — a teammate on his mark does neither: each is put back on his
  // mark every frame, and the leaf put home until the row is taken)
  const alive = g.guards.filter((m: any) => m.state !== 'dead'), man: any = o.man ? alive[0] : null, second: any = o.second ? alive[1] : null;
  const marks: [any, number[]][] = []; if (man) { const j = g.doors.kickSpotSide(d, doorPt(d, W / 2, 0.75, side)); marks.push([man, [j.pos[0], 0, j.pos[2]]]); } if (second) marks.push([second, doorPt(d, o.second![0], o.second![1], side)]);
  const pin = () => { for (const [m, p] of marks) { m.char.pos = [...p]; m.char.vel = [0, 0, 0]; } }; pin();
  for (const m of g.guards) if (m !== man && m !== second) m.char.pos = [m.char.pos[0] + 400, 0, m.char.pos[2] + 400];   // (every other man of the floor off it: nobody walks into the picture)
  // (a guard of the floor stood by a leaf works on it — a teammate on his mark does not: the leaf is put home again every frame until the row is taken)
  const home = () => { d.angle = 0; d.vel = 0; d.latched = true; pin(); }; if (man) R.post = home;
  const mine = (x: any) => x.kind === 'door' && x.door === d;
  g.teleportPlayer(spot); R.step(12); const bare = g.nearby.filter(mine).map((x: any) => x.verb as string);
  K.kitSolo(g); R.step(3);
  // (`hand`: what he holds as the row is taken — the digit pressed, the kit's own swap let finish; `front`: his rifle on its sling in front, as a loud mission has it)
  const sling0 = KITANIM.sling; if (o.front !== undefined) KITANIM.sling = o.front ? 1 : 2; if (o.kit) R.step(2);
  if (o.hand) { const up = o.hand === 'rifle'; R.input.pressed.add(up ? 'Digit2' : 'Digit1'); R.step(1); if (!(up ? pl.rifleUp : pl.pistolUp)) { R.input.pressed.add('KeyE'); R.step(1); } R.until(() => (up ? pl.rifleUp : pl.pistolUp) && !(pl.equipT > 0) && !pl.char.anim.kitBusy, 6); R.step(20); }
  const held = kitInHand(g) as string | null;
  const rows = g.nearby.filter(mine).map((x: any) => x.verb as string), reach = Math.min(flat(d.pos, spot), flat(d.frameCentre, spot));
  // the row taken: Space on it where it is the list's (a nearer door may have the list's one such row — then the same call the row makes, BY HAND)
  const st = stats(), t0 = g.time, listed = rows.includes(verb), picked = listed && g.selectVerb('door', verb) && g.hover?.door === d; R.post = null;
  // (… and THROUGH the beat, until a blow or a shell of his has landed: a guard of the floor is staff, and a leaf he stands by is brought round to him by his
  //  body — doors.ts update; a teammate's is not. Put home inside the frame, after the doors' own update, so his beat never finds it open under his hands)
  if (man) { const D: any = g.doors, up0 = D.update.bind(D), h0 = st.samBlows + st.samShells;
    D.update = (...a: any[]) => { const r = up0(...a); if (pl.kick && BIG_STATS.samBlows + BIG_STATS.samShells === h0 && !d.lockBroken) { d.angle = 0; d.vel = 0; d.latched = true; } return r; }; }
  if (picked) { R.input.pressed.add('Space'); R.step(); } else if (BK.samBigFar(g, d, pl.char.pos) || reach < 1.35) { BK.samBigStart(g, d); R.step(); }
  const began = pl.kick?.tool?.kind === tool && pl.kick.door === d, place = began ? fc(pl.kick.spot) : null, lead = (began ? pl.kick.tool.lead ?? 0 : 0) as number;
  if (process.env.DBG) console.log('DBG', seed, face, 'sam', J(fc(pl.char.pos)), 'man', man ? J(fc(man.char.pos)) : '-', 'rows', rows.join(','), 'listed', listed, 'picked', picked, 'began', began, 'place', J(place), 'spots', BK.bigSpots(g, d, side, tool).map((x: any) => J(fc(x.pos))).join(' '), 'closed', d.isClosed(), d.angle);
  // every frame of the beat: where he is, whether his legs walk, the longest step of one frame, what the animator is told, when the blows land
  let inOpening = 0, walked = 0, stride = 0, posed = 0, firstTold = NaN, told: number[] = [], landed: number[] = [], arrive = NaN, endTold = NaN, at: number[] | null = null, last = [...pl.char.pos], hits = st.samBlows + st.samShells;
  let turned = false; const via0 = began ? pl.kick.tool.moves?.via : undefined;
  let pullK = NaN, moved = false;
  // BIGSLOT.weight: the frames his own gun is on its way to its place (the beat's clock below zero) — the kit's hand at work in them, the gun still drawn
  // (never popped away), how far his right hand travels in his body's frame — then when the tool is all in his hands, and how long it takes to go back
  let carry = 0, carryBusy = 0, carryHid = 0, handWent = 0, toolIn = NaN, toolBack = NaN, hid0 = NaN, stepOut = 0, bareTail = 0, pops = 0, drawing = false; let hand0: number[] | null = null, handWas: number[] | null = null; const rec = { equip: NaN, stow: NaN };
  const inBody = () => { const c = pl.char, h = c.bones.handR ?? c.pos, y = c.bodyYaw, dx = h[0] - c.pos[0], dz = h[2] - c.pos[2]; return [dx * Math.cos(y) - dz * Math.sin(y), h[1] - c.pos[1], dx * Math.sin(y) + dz * Math.cos(y)]; };
  const dist3 = (p: number[], q: number[]) => Math.hypot(p[0] - q[0], p[1] - q[1], p[2] - q[2]), hand = () => [...(pl.char.bones.handR ?? pl.char.pos)] as number[];
  const drawn0 = drawnTool(g, pl.char.id), back0 = drawn0.head ? dist3(drawn0.head, hand()) : NaN; let headAt = NaN, inHand = NaN;
  R.post = () => { pin(); o.each?.(g, g.time - t0); if (via0 && pl.kick && !pl.kick.tool?.moves?.via) turned = true; const p = pl.char.pos, f = fc(p), T = pl.kick?.tool, P = pl.char.anim.bigTool; stride = Math.max(stride, flat(p, last)); last = [...p];
    if (pl.kick) { if (f.along > 0.05 && f.along < W - 0.05 && f.off < 1.3) inOpening++; if (pl.char.anim.speed > 0.2) walked++; }
    if (!moved && Math.abs(d.angle) > 0.02) { moved = true; pullK = pl.char.anim.bigK; }
    if (P) { posed++; if (P.blows.length && Number.isNaN(firstTold)) firstTold = P.blows[0] - P.t; told = [...P.blows]; if (P.end >= 0) endTold = P.end; }
    if (T?.moves?.arrived && Number.isNaN(arrive)) arrive = T.t;
    if (T && T.t < 0) { carry++; if (pl.char.anim.kitBusy) carryBusy++; if (pl.char.anim.hideHeldItem) carryHid++; const h = inBody(); hand0 ??= h; handWent = Math.max(handWent, dist3(h, hand0)); }
    if (T && Number.isNaN(hid0) && pl.char.anim.hideHeldItem) hid0 = T.t;
    if (T && Number.isNaN(toolIn)) { const h = inBody(); if (handWas) stepOut = Math.max(stepOut, dist3(h, handWas)); handWas = h; }   // (the longest step of his right hand in one frame, in his body's frame, from the row to the tool in his hands)
    if (T && Number.isNaN(toolIn) && pl.char.anim.bigK >= 0.98) toolIn = T.t;
    if (P) { rec.equip = P.equip; rec.stow = P.stow; }
    // (the way back: a frame of the tail with his gun free to fire — the tool still on its way, his hands not on the gun; and, after the blow, a frame his gun
    //  is shown in his hands with no errand of the kit's before the kit's hand draws it)
    if (!pl.kick && P && !(pl.equipT > 0)) bareTail++;
    if (landed.length) { if (pl.char.anim.kitBusy) drawing = true; else if (!drawing && !pl.char.anim.hideHeldItem && (pl.kick || P)) pops++; }
    if (P && P.end >= 0 && Number.isNaN(toolBack) && P.t > P.end && !(pl.char.anim.bigK > 0.02)) toolBack = P.t - P.end;
    const n = BIG_STATS.samBlows + BIG_STATS.samShells; if (n > hits) { hits = n; landed.push(T ? T.t : NaN); at ??= [...p];
      if (T?.moves && Number.isNaN(headAt)) { const D = drawnTool(g, pl.char.id); if (D.head) { headAt = dist3(D.head, T.moves.at); inHand = dist3(D.head, hand()); } } } };
  R.until(() => !d.latched || !!d.lockBroken, 6); const secs = g.time - t0; R.until(() => !pl.kick, 4);
  // his legs are his own: the tool is still on its way back (the pose kept, his gun locked) — and then it is on him again
  const freeAt = { posed: !!pl.char.anim.bigTool, lock: pl.equipT as number, end: pl.char.anim.bigTool?.end ?? NaN, t: pl.char.anim.bigTool?.t ?? NaN };
  // THE TAIL (bigkit.ts samBigTail): no tool's row is his inside it, a teleport drops it — read on a copy of the moment, the run itself goes on untouched —
  // and how long after the blow a round could leave his gun
  const B0 = pl.char.anim.bigTool, tail = { on: !!B0 && !pl.kick?.tool, left: B0 && B0.end >= 0 ? Math.max(0, B0.end + B0.stow - B0.t) : 0 };
  // (the door's end and where the men stand are read NOW, the stand-ins still on their marks: left alone for seconds a guard of the floor shuts a leaf and walks)
  R.until(() => Math.abs(d.angle) > 0.5, 2); const open = Math.abs(d.angle) > 0.5, manAt = man && at ? flat(man.char.pos, at) : NaN, secondAt = second && at ? flat(second.char.pos, at) : NaN;
  let gunAt = NaN; for (let i = 0; i < 200; i++) { R.step(1); if (Number.isNaN(gunAt) && !pl.char.anim.bigTool && pl.equipT <= 0 && !pl.char.anim.kitBusy && !pl.char.anim.hideHeldItem) { gunAt = g.time - t0 - lead - (landed[0] ?? NaN); if (i >= 40) break; } }   // (`landed` is on the beat's own clock, which starts `lead` below zero)
  R.post = null; KITANIM.sling = sling0; const drawn1 = drawnTool(g, pl.char.id), back1 = drawn1.head ? dist3(drawn1.head, hand()) : NaN;
  const stood = at ? fc(at) : null;
  return { R, g, d, W, floor, reach, bare, rows, held, lead, carry, carryBusy, carryHid, handWent, toolIn, toolBack, hid0, stepOut, rec, bareTail, pops, stowOf: (freeAt.posed ? freeAt.lock : NaN) as number, picked, listed, began, corner: via0 ? fc(via0) : null, turned, secondAt, place, stood, drawn0, drawn1, back0, back1, headAt, inHand, freeAt, man, manAt, inOpening, walked, stride, posed, firstTold, told, landed, arrive, endTold, secs,
    open, broken: !!d.lockBroken, gave: !!d.lockBroken && since(st).samHammered + since(st).samShot === 1, st: since(st), poseAfter: pl.char.anim.bigTool, pullK, gunAt, tail, lockAt: at ? flat(at, BK.bigSpots(g, d, side, tool)[0]?.at ?? d.pos) : NaN,
    shots: R.ev.filter(e => e.t >= t0 && (e.kind === 'shot' || e.kind === 'guardShot')).length, kicks: R.ev.filter(e => e.t >= t0 && e.kind === 'kick').length };
}
/** the gun swap the tools are weighed against (character.ts KIT, kitanim.ts): seconds from the digit to the other gun up, the kit's hand done — `front`: the
 *  rifle on its sling in front, as a loud mission wears it. */
export async function gunSwap(to: 'pistol' | 'rifle', front: boolean) {
  const R = await rig(74, { stage: false }), g = R.game, pl = g.player, an: any = pl.char.anim, sling0 = KITANIM.sling; g.aiEnabled = false;
  KITANIM.sling = front ? 1 : 2; R.step(3);
  const up = (w: 'pistol' | 'rifle') => { const is = () => (w === 'rifle' ? pl.rifleUp : pl.pistolUp) as boolean, t0 = g.time; R.input.pressed.add(w === 'rifle' ? 'Digit2' : 'Digit1'); R.step(1); if (!is()) { R.input.pressed.add('KeyE'); R.step(1); }
    const ok = R.until(() => is() && !(pl.equipT > 0) && !an.kitBusy, 6); return ok ? g.time - t0 : NaN; };
  up(to === 'rifle' ? 'pistol' : 'rifle'); R.step(30); const secs = up(to);
  KITANIM.sling = sling0; return secs;
}

/** the file's probes, by the names the runner calls them (the holds each is owed — the card's one gun row, the table's layout — are tools/qa/holds.ts's) */
const PROBES = { 'bigslot-card': bigCard, 'bigslot-mate': bigMate, 'bigslot-shut': bigShut, 'bigslot-wiring': bigWiring };
async function bigslotCheck(want: readonly string[]): Promise<boolean | null> {
  console.log(`bigslot-check on the ${STAND.toUpperCase()} stand${BROKEN ? ' — BIGKIT_BREAK=1: the tools do nothing to a lock, the lock lines below must FAIL' : ''}`);
  const ok = await asProbes(PROBES, want);
  console.log(`\nbigslot-check (${STAND} stand): ${ok === null ? 'NOTHING RAN' : ok ? 'ALL PASS' : 'FAILED'}`);
  return ok;
}
if (import.meta.main) process.exit(exitCode(await bigslotCheck(wanted())));
