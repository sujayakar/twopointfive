// LIGHTER KIT MOVES FASTER AND HIDES BETTER — the check behind src/game/kitload.ts (KITLOAD.on: on in the page, ?nokitload; off on the frozen stand,
// TPS_KITLOAD=1).
//   bun run tools/qa/kitload-check.ts        (also `probes.ts kitload`; run it on BOTH stands: plain, and with TPS_PROFILE=shipped in front)
//   kitload-table   the scores as data: you as issued are the reference (×1 pace, notice and footfall); a bare card scores best and a full one worst;
//                   taking any one thing off a card never lowers a score; a fitting or a colour weighs nothing; with the pouches rule off nobody is weighed
//   kitload-you     the plain night with your card on you (kit.ts kitSolo), W held: a light card's top walk and sprint are quicker than the issued card's
//                   and a heavy card's slower; the first footfall is quieter light and louder heavy; a guard's eye builds its notice of you more slowly
//                   light and faster heavy, and the point-blank floor is the same for all three
//   kitload-save    a checkpoint holds the two scores
// KITLOAD_BREAK=1 = the floor ignores the scores with the switch on: kitload-you must fail (the table and the card
// hold either way: they read no floor).
// exit 0 = all hold · 1 = one does not
// THE HARNESS HAS NO GPU: a pace is a number, a footfall an event's level, a bar a width on a fake element. Nobody has seen or played any of this.
import { standUp, ROOT } from './headless.ts';

const K = await import(`${ROOT}/src/game/kit.ts`);
const L = await import(`${ROOT}/src/game/kitload.ts`);
const { KITLOAD } = L;
const Gd = await import(`${ROOT}/src/game/guards.ts`);
const { SCENARIOS } = await import(`${ROOT}/src/scene/scenarioList.ts`);
const { scenarioQuery } = await import(`${ROOT}/src/scene/scenarios.ts`);
const { startFromQuery, teamOf } = await import(`${ROOT}/src/game/scenarioRun.ts`);
const { digestGame, hashDigest } = await import(`${ROOT}/src/game/digest.ts`);
const { harnessProfile } = await import(`${ROOT}/src/host/profile.ts`);

const DT = 1 / 60, BROKEN = process.env.KITLOAD_BREAK === '1', STAND = harnessProfile() as string;
const f2 = (x: number) => (Number.isFinite(x) ? x.toFixed(2) : String(x)), f3 = (x: number) => (Number.isFinite(x) ? x.toFixed(3) : String(x));
type Verdict = (ok: boolean, label: string, detail: string) => void;
function family(title: string): { verdict: Verdict; done: () => boolean } {
  console.log(`\n=== ${title} ===`); let all = true;
  return { verdict: (ok, label, detail) => { all &&= ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${label}\n    ${detail}`); }, done: () => { console.log(`=== ${title.split(':')[0]}: ${all ? 'ALL PASS' : 'FAILURES above'} ===`); return all; } };
}
const hushed = async <T>(fn: () => Promise<T> | T): Promise<T> => { const log = console.log, err = console.error, warn = console.warn; console.log = console.error = console.warn = () => { }; try { return await fn(); } finally { console.log = log; console.error = err; console.warn = warn; } };
function seeded(a: number) { return () => { a |= 0; a = (a + 0x6D2B79F5) | 0; let t = Math.imul(a ^ (a >>> 15), 1 | a); t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t; return ((t ^ (t >>> 14)) >>> 0) / 4294967296; }; }
/** the kit table, the switch and the dice as they stood, put back after `fn` */
async function kept<T>(fn: () => Promise<T> | T): Promise<T> {
  const cards = K.KIT.members.map((m: any) => ({ ...m })), on = KITLOAD.on, slots = K.SLOTS.on, big = K.BIGSLOT.on, rnd = Math.random;
  try { rules(); return await fn(); } finally { K.BIGSLOT.on = big; K.SLOTS.on = slots; for (let i = 0; i < cards.length; i++) K.KIT.members[i] = cards[i]; KITLOAD.on = on; Math.random = rnd; }
}
/** the two kit rules the cards below are cut for, raised on either stand (the frozen stand holds both off as a game stands up: written again after each
 *  stand-up) — a card is weighed by the rows it HAS, and the hammer's row is the big slot's */
const rules = () => { K.SLOTS.on = true; K.BIGSLOT.on = true; };
/** three cards of yours: nothing but the pistol · as issued under the pouches rule · the long gun, every pouch a smoke can, and the hammer where the stand has a big slot */
const YOU = {
  light: { weapon: 'pistol', smoke: 0, bangs: 0, frags: 0, shells: 0, wand: false, cams: 0, charges: 0, walls: 0, big: 'none' },
  issued: { weapon: 'rifle', smoke: 1, bangs: 1, frags: 1, shells: 1, wand: false, cams: 0, charges: 0, walls: 0, big: 'none' },
  heavy: { weapon: 'rifle', smoke: 4, bangs: 0, frags: 0, shells: 0, wand: false, cams: 0, charges: 0, walls: 0, big: 'hammer' },
} as const;
const card = (i: number, over: Record<string, unknown>) => ({ ...K.kitDefault(i), ...over });

// ───────────────────────────────────────────────────────────── the table
export async function kitloadTable(): Promise<boolean> {
  const F = family('kitload-table: a card\'s two scores');
  await kept(() => {
    const R = L.loadRef(0), a = L.loadOf(0, card(0, YOU.light)), b = L.loadOf(0, card(0, YOU.issued)), c = L.loadOf(0, card(0, YOU.heavy));
    const worn = (x: any, r: any = R) => ({ ...x, refMob: r.mob, refCon: r.con });
    const say = (x: any) => `mobility ${f2(x.mob)} concealment ${f2(x.con)} (pace ×${f3(L.loadPaceK(worn(x)))}, seen ×${f3(L.loadSeenK(worn(x)))}, heard ×${f3(L.loadHeardK(worn(x)))})`;
    F.verdict(Math.abs(b.mob - R.mob) < 1e-9 && Math.abs(b.con - R.con) < 1e-9 && L.loadPaceK(worn(b)) === 1 && L.loadSeenK(worn(b)) === 1 && L.loadHeardK(worn(b)) === 1,
      'you as issued (the long gun, the pistol, one smoke, one stun, one frag, one 40 mm) are your own reference: ×1 on pace, notice and footfall', `issued: ${say(b)} · the reference: mobility ${f2(R.mob)} concealment ${f2(R.con)}`);
    { // (as the team WEARS the issue: kit.ts KITDEAL shares it out over three or four, and a pair carries it as the table writes it)
      let bad = ''; const one = (w: any) => !!w && L.loadPaceK(w) === 1 && L.loadSeenK(w) === 1 && L.loadHeardK(w) === 1;
      for (let i = 0; i < K.KIT_MEMBERS; i++) { if (!one(L.loadWorn(i, K.kitIssuedShown(i, 4), 4))) bad ||= `member ${i} of a four`; if (i <= 2 && !one(L.loadWorn(i, K.kitDefault(i), 2))) bad ||= `member ${i} of a pair`; }
      F.verdict(!bad, 'every man as issued is ×1 against his own issue, exactly — a four as it is dealt the issue, a pair as the table writes it', bad || Array.from({ length: K.KIT_MEMBERS }, (_x, i) => { const r = L.loadRef(i); return `${i}: ${f2(r.mob)} / ${f2(r.con)}`; }).join(' · ')); }
    F.verdict(a.mob > b.mob && b.mob > c.mob && a.con > b.con && b.con > c.con && L.loadPaceK(worn(a)) > 1 && L.loadPaceK(worn(c)) < 1 && L.loadSeenK(worn(a)) < 1 && L.loadSeenK(worn(c)) > 1 && L.loadHeardK(worn(a)) < 1 && L.loadHeardK(worn(c)) > 1,
      'the pistol alone scores best on both and the long gun with full pouches worst; lighter is quicker, noticed more slowly and quieter', `light: ${say(a)}\n    heavy (four smoke, the hammer): ${say(c)}`);
    // one thing off any card never lowers a score, on every member, from a card filled as far as its rows go
    let worst = '', n = 0;
    for (let i = 0; i < K.KIT_MEMBERS; i++) {
      const full: any = card(i, { weapon: 'rifle', smoke: 1, bangs: 1, frags: 1, charges: 1, shells: i ? 0 : 1, wand: true, cams: i ? 0 : 1, big: 'hammer' }), was = L.loadOf(i, full);
      for (const [f, v] of [['weapon', 'pistol'], ['smoke', 0], ['bangs', 0], ['frags', 0], ['charges', 0], ['shells', 0], ['wand', false], ['cams', 0], ['big', 'none']] as [string, unknown][]) {
        const less = L.loadOf(i, { ...full, [f]: v }); n++;
        if (less.mob < was.mob - 1e-12 || less.con < was.con - 1e-12) worst ||= `member ${i} without ${f}: ${f2(less.mob)} / ${f2(less.con)} under ${f2(was.mob)} / ${f2(was.con)}`;
      }
    }
    F.verdict(!worst, 'taking one thing off a card never lowers a score (every member, every thing)', worst || `${n} cases`);
    const base = L.loadOf(1, card(1, {})), dressed = L.loadOf(1, card(1, { light: false, nvg: false, suit: 'olive', trim: 'amber' }));
    F.verdict(base.mob === dressed.mob && base.con === dressed.con && L.loadShare(1, card(1, {}), 'suit') === null && L.loadShare(1, card(1, {}), 'light') === null,
      'a fitting or a colour weighs nothing', `Red One as issued ${f2(base.mob)} / ${f2(base.con)} · with no light, no goggles, olive and amber ${f2(dressed.mob)} / ${f2(dressed.con)}`);
    { // the guns: whatever rows a card has for them (one Weapon row, or a Primary and a Secondary — gunslots.ts), between them they weigh his guns once
      const GUNS = ['weapon', 'primary', 'secondary'], bad: string[] = [], said: string[] = [];
      for (let i = 0; i < K.KIT_MEMBERS; i++) for (const w of ['rifle', 'pistol']) {
        const m: any = card(i, { weapon: w }), fields = (K.kitRows(i) as any[]).map(r => r.field).filter(f => GUNS.includes(f)), guns = L.gunsOf(i, m) as string[];
        const rows = fields.reduce((a, f) => { const x = L.loadShare(i, m, f); return x ? [a[0] + x.mob, a[1] + x.con] : a; }, [0, 0]), list = guns.reduce((a, g) => { const x = L.gunLoad(g); return [a[0] + x.mob, a[1] + x.con]; }, [0, 0]);
        if (Math.abs(rows[0] - list[0]) > 1e-9 || Math.abs(rows[1] - list[1]) > 1e-9 || !guns.length) bad.push(`member ${i} ${w}: rows ${fields.join('+')} weigh ${f2(rows[0])} / ${f2(rows[1])}, his guns (${guns.join(', ')}) ${f2(list[0])} / ${f2(list[1])}`);
        if (i < 2) said.push(`${i ? 'Red One' : 'you'} ${w}: ${fields.join('+')} = ${guns.join(' + ')}`);
      }
      { // a gun weighs what its own row says (weapons.ts weightKg, bulk): the short gun under the carbine under the belt-fed gun, a pistol under them all
        const G = ['fiveseven', 'mp7', 'fn2000', 'mk12', 'mk46'].map(w => L.gunLoad(w)), up = G.every((x: any, k: number) => !k || (x.mob > G[k - 1].mob && x.con >= G[k - 1].con));
        F.verdict(up && G[0].mob > 0 && G[4].con > G[2].con + 0.2 && G[1].con < G[2].con - 0.1, 'a gun weighs what its own row says: a pistol, the short gun, the issue carbine, the long rifle, the belt-fed gun — each heavier on the legs and no easier to hide than the one before', ['pistol', 'MP7', 'carbine', 'Mk 12', 'Mk 46'].map((n, k) => `${n} ${f2(G[k].mob)} / ${f2(G[k].con)}`).join(' · ')); }
      F.verdict(!bad.length, 'a card\'s gun rows — one, or a primary and a secondary — weigh his guns once between them (every member, with and without a long gun)', bad.join(' · ') || said.join(' · '));
    }
    K.SLOTS.on = false; const free = [0, 1].map(i => L.loadWorn(i, K.kitDefault(i))); K.SLOTS.on = true;
    F.verdict(free.every((w: unknown) => w === null) && L.loadWorn(0, card(0, YOU.light)) !== null, 'the weight is the pouches rule\'s: with that rule off nobody is weighed (free counts, no table of weights cut for them)', `rule off: ${JSON.stringify(free)} · rule on: weighed`);
  });
  return F.done();
}

// ───────────────────────────────────────────────────────────── you
/** the plain night under a seed, the switch as `on` says (written after the stand-up: the frozen stand holds it off as a game stands), `you` on your card and
 *  the card put on you (kitSolo) */
async function night(seed: number, you: Record<string, unknown>, on: boolean) {
  Math.random = seeded(0x4c0ad + seed * 7919);
  for (let i = 0; i < K.KIT_MEMBERS; i++) K.KIT.members[i] = K.kitDefault(i);
  const W: any = await hushed(() => standUp({ seed })); const g = W.game; rules(); KITLOAD.on = on; g.godMode = true;
  K.KIT.members[0] = card(0, you); K.kitSolo(g);
  const step = (n: number) => { const log = console.log, err = console.error, warn = console.warn; console.log = console.error = console.warn = () => { }; try { for (let f = 0; f < n; f++) W.step(DT); } finally { console.log = log; console.error = err; console.warn = warn; } };
  return { W, g, step };
}
/** W held for `secs` from where the night puts him (Shift with it: a sprint): his top ground speed, how far he got, and his first footfall's level */
async function walk(you: Record<string, unknown>, o: { sprint?: boolean; secs?: number; on?: boolean } = {}) {
  const R = await night(3, you, o.on ?? true), g = R.g, pl = g.player, from = [...pl.char.pos]; g.aiEnabled = false;
  R.step(20); R.W.input.keys.add('KeyW'); if (o.sprint) R.W.input.keys.add('ShiftLeft');
  let top = 0, first = NaN; const seen = new Set<any>();
  for (let f = 0; f < Math.round((o.secs ?? 1.5) / DT); f++) {
    R.step(1); top = Math.max(top, Math.hypot(pl.char.vel[0], pl.char.vel[2]));
    for (const e of g.events) if (e.kind === 'step' && e.who === undefined && !seen.has(e)) { seen.add(e); if (!Number.isFinite(first)) first = e.level; }
  }
  R.W.input.keys.delete('KeyW'); R.W.input.keys.delete('ShiftLeft');
  return { top, first, went: Math.hypot(pl.char.pos[0] - from[0], pl.char.pos[2] - from[2]), digest: hashDigest(digestGame(g)) as string, load: pl.kitLoad as { mob: number; con: number } | null };
}
export async function kitloadYou(): Promise<boolean> {
  const F = family('kitload-you: your card is your pace, your footfall and the notice you build');
  await kept(async () => {
    const a = await walk(YOU.light), b = await walk(YOU.issued), c = await walk(YOU.heavy);
    const as = await walk(YOU.light, { sprint: true }), bs = await walk(YOU.issued, { sprint: true }), cs = await walk(YOU.heavy, { sprint: true });
    // the notice a guard's eye builds of him: one game, the card changed on him and put on again
    const R = await night(3, YOU.issued, true), g = R.g, gd = g.guards.find((x: any) => x.state !== 'dead'); g.aiEnabled = false; R.step(10);
    const gain = (you: Record<string, unknown>, d: number) => { K.KIT.members[0] = card(0, you); K.kitPlayer(g); g.player.visibility = 0.6; gd.smokeTrans = 1; return Gd.sightGain(g, gd, d) as number; };
    const far = [gain(YOU.light, 6), gain(YOU.issued, 6), gain(YOU.heavy, 6)], near = [gain(YOU.light, 1), gain(YOU.issued, 1), gain(YOU.heavy, 1)];
    F.verdict(near[0] === near[1] && near[1] === near[2] && near[0] > far[2], 'at his elbow (1 m) the point-blank floor is whole: the same whatever you carry', `light ${f3(near[0])} · issued ${f3(near[1])} · heavy ${f3(near[2])}`);
  });
  return F.done();
}

// ───────────────────────────────────────────────────────────── a checkpoint
export async function kitloadSave(): Promise<boolean> {
  const F = family('kitload-save: a checkpoint keeps the two scores');
  await kept(async () => {
    const R = await night(5, YOU.light, true), g = R.g; g.aiEnabled = false; R.step(30);
    const was = { ...g.player.kitLoad }, snap = R.W.snapshot('kitload', true), P = (snap as any)?.player ?? (snap as any)?.state?.player;
    g.player.kitLoad = { mob: 0.1, con: 0.1, refMob: 0.5, refCon: 0.5 }; R.W.restore(snap); const back = g.player.kitLoad;
    F.verdict(!!back && Math.abs(back.mob - was.mob) < 1e-9 && Math.abs(back.con - was.con) < 1e-9 && Math.abs(back.refMob - was.refMob) < 1e-9 && Math.abs(back.refCon - was.refCon) < 1e-9, 'saved with the switch on, changed, loaded: what his card weighs is the file\'s, his reference with it', `was ${f2(was.mob)} / ${f2(was.con)} against ${f2(was.refMob)} / ${f2(was.refCon)} · back ${back ? `${f2(back.mob)} / ${f2(back.con)} against ${f2(back.refMob)} / ${f2(back.refCon)}` : 'none'}${P ? ` · the file: ${JSON.stringify(P.kitLoad)}` : ''}`);
  });
  return F.done();
}

export async function kitloadCheck(only?: string): Promise<boolean> {
  const all: [string, () => Promise<boolean>][] = [['kitload-table', kitloadTable], ['kitload-you', kitloadYou], ['kitload-save', kitloadSave]];
  let ok = true; for (const [n, fn] of all) if (!only || only === n) ok = (await fn()) && ok;
  if (BROKEN) console.log('(KITLOAD_BREAK=1: kitload-you is expected to FAIL)');
  return ok;
}
if (import.meta.main) process.exit((await kitloadCheck(process.argv[2])) ? 0 : 1);
