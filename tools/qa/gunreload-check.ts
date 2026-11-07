// EACH GUN IS FED ITS OWN WAY (src/game/gunreload.ts, under gunlook.ts GUNLOOK; `GUNLOOK_BREAK=1` must fail)
//   you       the seven rows with a model, each reloaded in your hands from EMPTY and from PART FULL: begun empty the reload is the row's own length; what
//             is heard, in order, is the gun's own — a belt gun's cover, box, belt, cover and handle; a tube gun's shells, as many as went in (the M1014
//             begun empty: the first in the port and the bolt dropped on it); a magazine gun's three as ever — the belt gun's on the row's own stages;
//             what falls is the gun's own empty (nothing out of a tube gun)
//   hands     the Mk 46's cover is up between its two cover sounds and its box off the gun between box off and box on; a thing of the gun's own size is
//             in the support hand and never the old magazine with it; a tube gun's hand makes a trip a shell; the MP7's hand goes to the foot of the
//             grip, the belt gun's over the top then under
//   mate      a teammate with each of the four long guns a card can hand him, his own reload beat: the row's length, full after it, the gun's own sounds
//             on his clock (always as a gun run dry), his empty drawn as what it is, the cover up on him too; and the room fight's clock plays the same list
//   shells    A TUBE GUN BY THE SHELL (`GUNSHELLS_BREAK=1` must fail): the M1014 and the pump gun reloaded in your hands take a lead, so long a shell
//             and a tail — one, three and a whole tube; each shell is in the gun the frame it is heard; the end is the end the old reload came to; a
//             reload cancelled after three keeps three, and the shells he carries are three fewer; the trigger breaks the feeding off — a chambered
//             round fires that press, a pump gun with nothing chambered runs its pump first; loose shells are counted one by one; a teammate's beat is
//             as long as the shells that go in
// SINCE 2026-11-07 every family is asked once, with the game's own rule (the gun's look and the shells key are settled on; this file writes neither, nor
// the gun slots, the deal or the movement pass): the run with the switch off and every line that set the two side by side are gone (the same frame and
// rounds as with the switch off; "with the switch off every reload is a magazine change as it was"; the mate's one clack; the counting clock silent
// with the switch off; the key off under the switch; the page's flag and a recording from before the key). GONE BY A LATER SETTLED RULE: the three
// families above the shells key were the build before it, staged with the key off: "the last shell home where the magazine went in" (a tube gun's
// shells on the old magazine stages) asked that build and is deleted; the belt gun's half of that line stays, on the row's own stage numbers
// (reload.out / in_ / rack), which is where the off run's three sounds stood. `GUNLOOK_BREAK=1` is gunlook.ts's own. `GUNSHELLS_BREAK=1`: the R key
// is never pressed in the shells family, so nothing is fed and its lines of time, shells and books fail.
// Nothing here looks at a picture or hears a sound: the names that WOULD have played (headless.ts audioLog), the boxes that would have been drawn,
// where the wrists are.
import { ROOT } from './headless.ts';
import { rig } from './doortools-probes.ts';

const FNP: any = await import('./fn2000-probes.ts');
const GR: any = await import(`${ROOT}/src/game/gunreload.ts`);
const K: any = await import(`${ROOT}/src/game/kit.ts`);
const KA: any = await import(`${ROOT}/src/game/kitanim.ts`);
const KN: any = await import(`${ROOT}/src/game/knobs.ts`);
const { WEAPONS, Rifle, Pistol } = await import(`${ROOT}/src/game/weapons.ts`);
const { spawnUnit, startReloadBeat, tickReloadBeat } = await import(`${ROOT}/src/game/guards.ts`);

type Verdict = (ok: boolean, label: string, detail: string) => void;
function family(title: string): { verdict: Verdict; done: () => boolean } {
  console.log(`\n=== ${title} ===`); let all = true;
  return { verdict: (ok, label, detail) => { all &&= ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${label}\n    ${detail}`); }, done: () => { console.log(`=== ${title.split(':')[0]}: ${all ? 'ALL PASS' : 'FAILURES above'} ===`); return all; } };
}
const hushed = async <T>(fn: () => Promise<T> | T): Promise<T> => { const log = console.log, err = console.error, warn = console.warn; console.log = console.error = console.warn = () => { }; try { return await fn(); } finally { console.log = log; console.error = err; console.warn = warn; } };
const J = (o: unknown) => JSON.stringify(o), f2 = (x: number) => (Number.isFinite(x) ? x.toFixed(2) : String(x));
const DT = 1 / 60, AIM = [27.5, 1.2, 11.5];
const SHELLS_BREAK = process.env.GUNSHELLS_BREAK === '1';
/** a joint of the DRAWN skeleton, world */
const drawn = (c: any, node: number): number[] => { const w = c.world, p = c.skel.nodePos(node); return [w[0] * p[0] + w[4] * p[1] + w[8] * p[2] + w[12], w[1] * p[0] + w[5] * p[1] + w[9] * p[2] + w[13], w[2] * p[0] + w[6] * p[1] + w[10] * p[2] + w[14]]; };
/** the big slot a card's own long gun rides on (a run's own row, not a settled one) */
const raise = () => { K.BIGSLOT.on = true; };
/** every table and card put back as found */
async function kept<T>(fn: () => Promise<T>): Promise<T> {
  const knobs = KN.knobsCopy(), cards = JSON.stringify(K.KIT.members);
  try { return await fn(); } finally { KN.knobsBack(knobs); K.KIT.members.splice(0, K.KIT.members.length, ...JSON.parse(cards)); }
}

const LONG = ['mk12', 'mp7', 'mk46', 'm1014', 'm870'] as const, SIDE = ['hk45', 'g18'] as const, STATES = ['empty', 'part'] as const;
type Id = (typeof LONG)[number] | (typeof SIDE)[number];
/** what a reload makes heard (anything else in the log — a footstep, weather — is not the gun's) */
const FOLEY = new Set(['magOut', 'magIn', 'rack', 'beltCoverUp', 'beltBoxOff', 'beltBoxOn', 'beltLay', 'beltCoverDown', 'chargeLmg', 'shellIn', 'boltDrop']);
/** THE GUN'S OWN LIST, written out here (not asked of the module): what is heard, in order, of a reload that puts `n` rounds in */
function ownList(id: Id, empty: boolean, n: number): string[] {
  const shells = (k: number) => Array.from({ length: k }, () => 'shellIn');
  if (id === 'mk46') return ['beltCoverUp', 'beltBoxOff', 'beltBoxOn', 'beltLay', 'beltCoverDown', ...(empty ? ['chargeLmg'] : [])];
  if (id === 'm1014') return empty ? ['shellIn', 'boltDrop', ...shells(n - 1)] : shells(n);
  if (id === 'm870') return [...shells(n), ...(empty ? ['rack'] : [])];
  return ['magOut', 'magIn', ...(empty ? ['rack'] : [])];
}
/** what an EMPTY one leaving the gun is drawn as on the floor ('mag' item fill) — null: nothing falls */
const DROP: Record<Id, number | null> = { mk12: 6, mp7: 7, mk46: 5, m1014: null, m870: null, hk45: 0, g18: 4 };
/** the thing in the support hand: its half extents */
const HAND: Record<Id, number[]> = { mk12: [0.012, 0.055, 0.03], mp7: [0.008, 0.095, 0.013], mk46: [0.035, 0.045, 0.06], m1014: [0.01, 0.035, 0.01], m870: [0.01, 0.035, 0.01], hk45: [0.009, 0.055, 0.018], g18: [0.008, 0.11, 0.016] };

interface Frame { t: number; show: { noMag: boolean; cover: number } | null; boxes: any[]; oldHand: boolean; magL: unknown; z: number; y: number }
interface Run { began: boolean; frames: number; mag: number; chamber: number; spare: string; gained: number; foley: { name: string; t: number }[]; drops: number[]; fr: Frame[] }

/** one reload of `id` in your hands: the R key, every frame until the weapon says it is done */
function one(W: any, id: Id, st: 'empty' | 'part', long: boolean): Run {
  const g = W.game, pl = g.player, I = W.input, c = pl.char, an = c.anim, def = WEAPONS[id];
  const gun = long ? (pl.rifle = new Rifle(def)) : (pl.pistol = new Pistol(def));
  for (let f = 0; f < 40; f++) { W.cursorAt(AIM); W.step(DT); }
  if (st === 'empty') { gun.mag = 0; gun.chamber = 0; } else { gun.chamber = 1; gun.mag = def.pellets ? def.magCapacity - 3 : Math.floor(def.magCapacity / 2); }
  const before = gun.mag + gun.chamber, items0 = new Set(g.items.items), a: any[] = []; W.audioLog = a; const t0 = g.time;
  I.pressed.add('KeyR'); let frames = -1; const fr: Frame[] = [];
  for (let f = 0; f < 700 && frames < 0; f++) { W.cursorAt(AIM); W.step(DT);
    const bx: any[] = []; GR.reloadHandBoxes(c, id, bx);
    const kit = KA.kitShow(g, { pistolInHand: !long, rifle: long ? 'hand' : 'slung', held: null, knife: false });
    // (the frame the hand's stations are written in — character.ts reloadArm: z out of the trigger fist along its fingers, y the world's up squared to it)
    const nd = an.nodes, hl = drawn(c, nd.handL), hr = drawn(c, nd.handR), kn = drawn(c, nd.middle1R), rel = [hl[0] - hr[0], hl[1] - hr[1], hl[2] - hr[2]];
    let d = [kn[0] - hr[0], kn[1] - hr[1], kn[2] - hr[2]]; { const L = Math.hypot(d[0], d[1], d[2]) || 1; d = d.map(x => x / L); }
    let u = [-d[0] * d[1], 1 - d[1] * d[1], -d[2] * d[1]]; { const L = Math.hypot(u[0], u[1], u[2]) || 1; u = u.map(x => x / L); }
    fr.push({ t: g.time - t0, show: GR.reloadShow(c, id), boxes: bx, oldHand: an.reloadShow?.hand === true, magL: kit?.magL ?? null, z: rel[0] * d[0] + rel[1] * d[1] + rel[2] * d[2], y: rel[0] * u[0] + rel[1] * u[1] + rel[2] * u[2] });
    if (!gun.reloading) frames = f; }
  W.audioLog = null;
  return { began: frames > 0, frames, mag: gun.mag ?? -1, chamber: gun.chamber, spare: J(gun.spare), gained: (gun.mag ?? 0) + gun.chamber - before,
    foley: a.filter(e => FOLEY.has(e.name)).map(e => ({ name: e.name as string, t: e.t - t0 })), drops: g.items.items.filter((x: any) => !items0.has(x) && x.kind === 'mag').map((x: any) => x.fill), fr };
}
/** all fourteen, one game, in one order */
async function pass(): Promise<Record<string, Run>> {
  const W: any = await hushed(() => FNP.stage(2002)); raise();
  const out: Record<string, Run> = {}, g = W.game, I = W.input;
  for (const id of LONG) for (const st of STATES) out[`${id} ${st}`] = one(W, id, st, true);
  I.pressed.add('Digit1'); for (let f = 0; f < 160; f++) { W.cursorAt(AIM); W.step(DT); }
  if (!g.player.pistolUp) throw new Error('the pistol never came up on the 1 key');
  for (const id of SIDE) for (const st of STATES) out[`${id} ${st}`] = one(W, id, st, false);
  return out;
}
const names = (r: Run) => r.foley.map(e => e.name), at = (r: Run, n: string, k = 0) => r.foley.filter(e => e.name === n)[k]?.t ?? NaN;
const near = (a: number, b: number, tol = DT * 1.5) => Math.abs(a - b) <= tol;

function judge(F: { verdict: Verdict }, on: Record<string, Run>) {
  const keys = Object.keys(on), idOf = (k: string) => k.split(' ')[0] as Id, empty = (k: string) => k.endsWith('empty');
  // the row's own length
  { const bad = keys.filter(k => !on[k].began);
    const row = keys.filter(k => empty(k) && !(Math.abs(on[k].frames - WEAPONS[idOf(k)].reload.dur * 60) <= 2));
    F.verdict(bad.length === 0 && row.length === 0, `every one of the fourteen reloads begins and ends; begun empty it is the row's own length`,
      keys.filter(empty).map(k => `${idOf(k)} ${on[k].frames} f (the row ${f2(WEAPONS[idOf(k)].reload.dur)} s) → ${on[k].mag}+${on[k].chamber}`).join(' · ') + (bad.length ? `\n    NEVER BEGAN: ${bad.join(', ')}` : '') + (row.length ? `\n    NOT THE ROW'S LENGTH: ${row.join(', ')}` : '')); }
  // the gun's own list, in order
  { const bad = keys.filter(k => J(names(on[k])) !== J(ownList(idOf(k), empty(k), on[k].gained)));
    F.verdict(bad.length === 0, `what is heard, in order, is the gun's own — a belt gun's cover, box, belt, cover (and handle on an empty chamber); a tube gun's shells; a magazine gun's three`,
      ['mk46 empty', 'mk46 part', 'm1014 empty', 'm870 empty', 'mp7 empty'].map(k => `${k}: ${names(on[k]).join(' ')}`).join('\n    ') + (bad.length ? `\n    NOT ITS OWN: ${bad.map(k => `${k}: ${names(on[k]).join(' ')}`).join(' | ')}` : '')); }
  // a shell a round
  { const ks = keys.filter(k => idOf(k) === 'm1014' || idOf(k) === 'm870'), n = (k: string) => names(on[k]).filter(x => x === 'shellIn').length;
    const ok = ks.every(k => n(k) === on[k].gained && on[k].gained > 0) && n('m1014 empty') === 7 && n('m1014 part') === 3 && on['m1014 empty'].foley[1]?.name === 'boltDrop' && on['m1014 empty'].foley[1].t - on['m1014 empty'].foley[0].t < 0.2 && !names(on['m870 empty']).includes('boltDrop');
    F.verdict(ok, `a tube gun plays one shell for every shell that went in — seven into the empty M1014, the first of them followed at once by the bolt; three into one three short`, ks.map(k => `${k}: ${n(k)} shells heard, ${on[k].gained} in`).join(' · ')); }
  // on the weapon's own stages
  { const rows: string[] = []; let ok = true;
    for (const k of keys) { const id = idOf(k), b = on[k]; if (id !== 'mk46') continue;
      const S = WEAPONS[id].reload, o = S.out, i = S.in_, r = S.rack, ts = b.foley.map(e => e.t), inside = ts.every((t, j) => t > 0 && t < b.frames * DT && (j === 0 || t >= ts[j - 1]));
      const good = inside && near(at(b, 'beltBoxOff'), o) && near(at(b, 'beltLay'), i) && at(b, 'beltCoverUp') < o - 0.05 && at(b, 'beltBoxOn') > o + 0.3 && at(b, 'beltBoxOn') < i - 0.1 && at(b, 'beltCoverDown') > i + 0.05 && (!empty(k) || (near(at(b, 'chargeLmg'), r) && at(b, 'beltCoverDown') < r - 0.05));
      ok &&= good; rows.push(`${k}: ${b.foley.map(e => `${e.name} ${f2(e.t)}`).join(', ')}${good ? '' : '  ← NOT ON ITS STAGES'}`); }
    F.verdict(ok, `the belt gun's sounds stand ON the row's stages and inside its length: the box off at the row's magazine out, the belt laid at its magazine in, the handle at its rack`, rows.join('\n    ')); }
  // what falls
  { const bad = keys.filter(k => J(on[k].drops) !== J(empty(k) && DROP[idOf(k)] !== null ? [DROP[idOf(k)]] : []));
    F.verdict(bad.length === 0, `an EMPTY one leaving the gun falls as what it is — the belt box, the short twenty, the MP7's long forty, the Glock 18's long magazine — a part-full one is kept, and nothing falls out of a tube gun`, keys.filter(empty).map(k => `${idOf(k)}: ${on[k].drops.length ? `fill ${on[k].drops.join(',')}` : 'nothing'}`).join(' · ') + (bad.length ? `\n    WRONG: ${bad.map(k => `${k} ${J(on[k].drops)}`).join(' | ')}` : '')); }
}
function judgeHands(F: { verdict: Verdict }, on: Record<string, Run>) {
  const keys = Object.keys(on), idOf = (k: string) => k.split(' ')[0] as Id;
  // the cover and the box
  { const lines: string[] = []; let ok = true;
    for (const k of ['mk46 empty', 'mk46 part']) { const b = on[k], up = at(b, 'beltCoverUp'), dn = at(b, 'beltCoverDown'), o = at(b, 'beltBoxOff'), n = at(b, 'beltBoxOn');
      const cov = (f: Frame) => f.show?.cover ?? 0, mid = b.fr.filter(f => f.t > up + 0.25 && f.t < dn - 0.2), out = b.fr.filter(f => f.t < up - 0.02 || f.t > dn + 0.05);
      const offGun = b.fr.filter(f => f.show?.noMag), good = mid.length > 30 && mid.every(f => cov(f) > 0.99) && out.every(f => cov(f) === 0) && offGun.length > 10 && offGun.every(f => f.t >= o - DT && f.t <= n + DT) && b.fr.filter(f => f.t > o + 2 * DT && f.t < n - 2 * DT).every(f => f.show?.noMag);
      ok &&= good; lines.push(`${k}: cover up ${f2(up)} → down ${f2(dn)} s, fully up on ${mid.filter(f => cov(f) > 0.99).length} / ${mid.length} frames between, down on ${out.filter(f => cov(f) === 0).length} / ${out.length} outside · box off the gun ${f2(offGun[0]?.t ?? NaN)} → ${f2(offGun.at(-1)?.t ?? NaN)} s (sounds ${f2(o)} → ${f2(n)})`); }
    F.verdict(ok, 'the Mk 46\'s feed cover is up between its two cover sounds and down outside them, and its belt box is off the gun from box off to box on', lines.join('\n    ')); }
  // the thing in the hand: the gun's own, alone
  { const lines: string[] = []; let ok = true;
    for (const k of keys) { const id = idOf(k), b = on[k], held = b.fr.filter(f => f.boxes.length > 0), h = HAND[id];
      const size = held.every(f => f.boxes.length === 1 && [0, 1, 2].every(i => Math.abs(f.boxes[0].h[i] - h[i]) < 1e-6) && f.boxes[0].owner === 1), alone = b.fr.every(f => f.magL === null);
      const shown = b.fr.some(f => f.show !== null), want = shown ? held.length > 3 : held.length === 0;   // (no change shown on him — a pistol with the kit's hands held off — nothing carried either)
      const where = !b.fr.some(f => f.oldHand && f.boxes.length === 0 && (id === 'mk12' || id === 'mp7' || id === 'hk45' || id === 'g18'));
      const good = size && alone && want && where; ok &&= good; lines.push(`${k}: ${held.length} frames with its own box${shown ? '' : ' (no change shown)'} · the old magazine with it on ${b.fr.filter(f => f.magL !== null).length}${good ? '' : '  ← WRONG'}`); }
    F.verdict(ok, 'through every reload shown on him the support hand carries ONE box of the gun\'s own size, stamped his, and never the old magazine with it; a magazine gun has it on every frame the old one was there', lines.join('\n    ')); }
  // a trip a shell
  { const trips = (r: Run) => r.fr.filter((f, i) => f.boxes.length > 0 && (i === 0 || r.fr[i - 1].boxes.length === 0)).length, ks = keys.filter(k => idOf(k) === 'm1014' || idOf(k) === 'm870');
    F.verdict(ks.every(k => trips(on[k]) === on[k].gained), 'a tube gun\'s hand makes one trip with a shell in its fingers for every shell that goes in', ks.map(k => `${k}: ${trips(on[k])} trips, ${on[k].gained} in`).join(' · ')); }
  // where the hand goes
  { const zAt = (r: Run, t: number) => r.fr.reduce((b, f) => Math.abs(f.t - t) < Math.abs(b.t - t) ? f : b), mp = on['mp7 empty'], mk = on['mk12 empty'], bt = on['mk46 empty'];
    const fm = zAt(mp, at(mp, 'magOut')), zk = zAt(mk, at(mk, 'magOut')).z, yc = zAt(bt, at(bt, 'beltCoverUp') + 0.1).y, yb = zAt(bt, at(bt, 'beltBoxOff')).y;
     }
}

// ------------------------------------------------------------------ the shells key
/** the two rows' numbers, written out here (the guns' own call): seconds to the shells, a shell, the finish, and the bolt inside the finish */
const SHELL: Record<'m1014' | 'm870', { lead: number; each: number; tail: number; bolt: number; cap: number; rack: string }> = {
  m1014: { lead: 0.4, each: 0.36, tail: 0.7, bolt: 0.4, cap: 7, rack: 'boltDrop' }, m870: { lead: 0.4, each: 0.35, tail: 0.7, bolt: 0.4, cap: 6, rack: 'rack' } };
interface SRun { began: boolean; secs: number; mag: number; chamber: number; carried: number; spare: number[]; fired: number; canFire: boolean; shells: { t: number; rounds: number; before: number }[]; foley: { name: string; t: number }[]; trips: number; reloading: boolean }
/** one reload of a tube gun in your hands: the gun set to `tube` + `chamber` (and `spare`, the loose shells he carries), the R key (`key` false: no key — what
 *  the trigger does to a gun as it stands), every frame until the weapon says it is done or `act` ends it: 'cancel' = let go as a sprint lets it go, 'fire' =
 *  the trigger pressed on the next frame. `gun`: carry on with the gun of the run before. */
function shellRun(W: any, id: 'm1014' | 'm870', tube: number, chamber: number, o: { spare?: number[]; key?: boolean; gun?: any; pre?: (gun: any) => void; act?: (heard: number, t: number) => 'cancel' | 'fire' | null } = {}): SRun & { gun: any } {
  const g = W.game, pl = g.player, I = W.input, c = pl.char, def = WEAPONS[id];
  const gun = o.gun ?? new Rifle(def); pl.rifle = gun;
  if (!o.gun) { for (let f = 0; f < 40; f++) { W.cursorAt(AIM); W.step(DT); } gun.cancelReload(); c.anim.cancelReload(); gun.mag = tube; gun.chamber = chamber; if (o.spare) gun.spare = [...o.spare]; o.pre?.(gun); }
  const a: any[] = []; W.audioLog = a; const t0 = g.time, fired0 = gun.shotsFired, shells: SRun['shells'] = []; let seen = 0, secs = -1, began = false, trips = 0, held = false, press = o.key === false, was = (gun.mag ?? 0) + gun.chamber;
  if (o.key !== false && !SHELLS_BREAK) I.pressed.add('KeyR');
  for (let f = 0; f < 700 && secs < 0; f++) {
    W.cursorAt(AIM); if (press) { I.buttons |= 1; I.clicked |= 1; } W.step(DT); if (press) { I.buttons = 0; press = false; }
    began ||= gun.reloading;
    const bx: any[] = []; GR.reloadHandBoxes(c, id, bx); if (bx.length > 0 && !held) trips++; held = bx.length > 0;
    const sh = a.filter(e => e.name === 'shellIn'); while (seen < sh.length) { shells.push({ t: sh[seen].t - t0, rounds: (gun.mag ?? 0) + gun.chamber, before: was }); seen++; } was = (gun.mag ?? 0) + gun.chamber;
    const act = gun.reloading ? o.act?.(seen, g.time - t0) : null;
    if (act === 'cancel') { gun.cancelReload(); c.anim.cancelReload(); secs = g.time - t0; }
    else if (act === 'fire') press = true;
    else if (!gun.reloading && (began || f > 30)) secs = g.time - t0;
  }
  W.audioLog = null;
  return { gun, began, secs, mag: gun.mag ?? -1, chamber: gun.chamber, carried: gun.spare.reduce((x: number, y: number) => x + y, 0), spare: [...gun.spare].sort((x: number, y: number) => y - x), fired: gun.shotsFired - fired0, canFire: gun.canFire(), shells,
    foley: a.filter(e => FOLEY.has(e.name)).map(e => ({ name: e.name as string, t: e.t - t0 })), trips, reloading: gun.reloading };
}
export async function gunreloadShells(): Promise<boolean> {
  let all = true;
  await kept(async () => {
    const F = family('gunreload-shells: a tube gun is fed a shell at a time, in time and in the books');
    const IDS = ['m1014', 'm870'] as const, secsOf = (id: 'm1014' | 'm870', n: number) => SHELL[id].lead + n * SHELL[id].each + SHELL[id].tail, tol = 2.5 * DT;
    const W: any = await hushed(() => FNP.stage(2002)); raise();
    // the rows
    { const got = IDS.map(id => WEAPONS[id].reload), others = Object.values(WEAPONS as Record<string, any>).filter(d => d.reload.shell && d.id !== 'm1014' && d.id !== 'm870').map(d => d.id);
      const same = J([got[0].dur, got[0].out, got[0].in_, got[0].rack, got[1].dur, got[1].out, got[1].in_, got[1].rack]) === J([3.6, 0.4, 2.9, 3.3, 3.2, 0.4, 2.5, 2.9]);
      F.verdict(IDS.every((id, i) => J(got[i].shell) === J({ lead: SHELL[id].lead, each: SHELL[id].each, tail: SHELL[id].tail, bolt: SHELL[id].bolt }) && WEAPONS[id].magCapacity === SHELL[id].cap) && others.length === 0 && same,
        'the two tube guns\' rows carry the guns\' own numbers for a shell at a time, no other row does, and their old stages stand as they stood', IDS.map((id, i) => `${id}: ${J(got[i].shell)}`).join(' · ') + ` · other rows with a shell beat: ${others.join(', ') || 'none'} · old stages as they were: ${same}`); }
    // the time, and each shell in the gun as it is heard
    const runs: Record<string, SRun> = {};
    for (const id of IDS) for (let n = 1; n <= SHELL[id].cap; n++) runs[`${id} ${n}`] = n === SHELL[id].cap ? shellRun(W, id, 0, 0) : shellRun(W, id, SHELL[id].cap - n, 1);
    { const ks = Object.keys(runs), nOf = (k: string) => +k.split(' ')[1], idOf = (k: string) => k.split(' ')[0] as 'm1014' | 'm870';
      const bad = ks.filter(k => !(runs[k].began && Math.abs(runs[k].secs - secsOf(idOf(k), nOf(k))) <= tol));
      F.verdict(bad.length === 0, 'a reload takes the lead, so long a shell and the tail: every count from one shell to a whole tube from empty', ks.map(k => `${k}: ${f2(runs[k].secs)} s (wants ${f2(secsOf(idOf(k), nOf(k)))})`).join(' · ') + (bad.length ? `\n    NOT ITS TIME: ${bad.join(', ')}` : ''));
      const late = ks.filter(k => { const S = SHELL[idOf(k)], r = runs[k], n0 = nOf(k) === S.cap ? 0 : S.cap - nOf(k) + 1;
        return !(r.shells.length === nOf(k) && r.shells.every((e, i) => Math.abs(e.t - (S.lead + (i + 1) * S.each)) <= tol && e.rounds === n0 + i + 1 && e.before === n0 + i)); });
      F.verdict(late.length === 0, 'every shell is heard at its own moment and goes INTO the gun on that frame, not the one before: the rounds in it go up by one a shell', ks.map(k => `${k}: ${runs[k].shells.map(e => `${f2(e.t)}→${e.rounds}`).join(' ')}`).join('\n    ') + (late.length ? `\n    NOT SO: ${late.join(', ')}` : ''));
      const tr = ks.filter(k => runs[k].trips !== nOf(k));
      F.verdict(tr.length === 0, 'the hand makes one trip with a shell in its fingers for every shell, on the beat the weapon is on', ks.map(k => `${k}: ${runs[k].trips} trips`).join(' · '));
      const ends = ks.filter(k => { const S = SHELL[idOf(k)], r = runs[k], empty = nOf(k) === S.cap; return !(r.chamber === 1 && r.mag === (empty ? S.cap - 1 : S.cap) && r.carried === 3 * S.cap - nOf(k)); });
      const heard = IDS.every(id => { const r = runs[`${id} ${SHELL[id].cap}`], nm = r.foley.map(e => e.name); return J(nm) === J(ownList(id, true, SHELL[id].cap)); });
      F.verdict(ends.length === 0 && heard, 'it ends where the old reload ended: from empty a tube less one and one chambered (the M1014\'s first shell in the port with the bolt dropped on it, the pump gun\'s pump run at the end), topped up to full, the shells he carries fewer by what went in', ks.map(k => `${k}: ${runs[k].mag}+${runs[k].chamber}, carries ${runs[k].carried}`).join(' · ') + ` · heard from empty as the gun's own list: ${heard}`); }
    // a reload cut short keeps what was fed
    { const a = shellRun(W, 'm1014', 0, 0, { act: n => n >= 3 ? 'cancel' : null }), b = shellRun(W, 'm870', 0, 0, { act: n => n >= 3 ? 'cancel' : null });
      F.verdict(a.mag + a.chamber === 3 && a.carried === 18 && a.canFire && b.mag === 3 && b.chamber === 0 && b.carried === 15 && !b.canFire,
        'a reload let go after three shells keeps three, and he carries three fewer: the M1014 has one of them chambered and may fire; the pump gun has three in the tube and nothing chambered', `M1014: ${a.mag}+${a.chamber}, carries ${a.carried}, may fire ${a.canFire} · pump gun: ${b.mag}+${b.chamber}, carries ${b.carried}, may fire ${b.canFire}`);
      const b2 = shellRun(W, 'm870', 0, 0, { gun: b.gun, key: false }), b3 = shellRun(W, 'm870', 0, 0, { gun: b.gun, key: false });
      F.verdict(b2.began && Math.abs(b2.secs - SHELL.m870.bolt) <= tol && J(b2.foley.map(e => e.name)) === J(['rack']) && b2.fired === 0 && b2.mag === 2 && b2.chamber === 1 && b3.fired === 1,
        'the trigger on that pump gun runs the pump on what is in the tube — its own short beat, nothing fired — and the next press fires', `the first press: ${f2(b2.secs)} s (wants ${f2(SHELL.m870.bolt)}), heard ${b2.foley.map(e => e.name).join(' ') || 'nothing'}, fired ${b2.fired}, then ${b2.mag}+${b2.chamber} · the second press fired ${b3.fired}`);
      const inf = shellRun(W, 'm870', 3, 0, { key: false, pre: gun => { gun.infinite = true; } });
      F.verdict(inf.fired === 1 && !inf.began, 'with unlimited ammunition on, the press on a pump gun with nothing chambered fires as it always did: no pump run is put in its way', `fired ${inf.fired}, a reload began: ${inf.began}`);
      const a2 = shellRun(W, 'm1014', 0, 0, { gun: a.gun });
      F.verdict(a2.began && Math.abs(a2.secs - secsOf('m1014', 5)) <= tol && a2.mag === 7 && a2.chamber === 1 && a2.carried === 13, 'R on the M1014 again feeds only the room that is left: five shells\' time, full, and five fewer carried', `${f2(a2.secs)} s (wants ${f2(secsOf('m1014', 5))}), ${a2.mag}+${a2.chamber}, carries ${a2.carried}`); }
    // the trigger breaks it off
    { const r = shellRun(W, 'm1014', 4, 1, { act: n => n >= 1 ? 'fire' : null });
      F.verdict(r.fired === 1 && !r.reloading && r.shells.length === 1 && r.mag + r.chamber === 5 && r.secs < SHELL.m1014.lead + SHELL.m1014.each + 4 * DT,
        'the trigger pressed while he feeds a gun with a round chambered lets the reload go and fires that press: one shell in, one round out', `fired ${r.fired}, ${r.shells.length} shell in, ${r.mag}+${r.chamber} after, over at ${f2(r.secs)} s`); }
    // loose shells
    { const r = shellRun(W, 'm1014', 2, 1, { spare: [1, 1] }), none = shellRun(W, 'm1014', 0, 0, { gun: r.gun });
      F.verdict(r.began && r.shells.length === 2 && Math.abs(r.secs - secsOf('m1014', 2)) <= tol && r.mag === 4 && r.carried === 0 && !none.began,
        'with two loose shells left two go in, in two shells\' time, and R after that begins nothing', `${r.shells.length} in over ${f2(r.secs)} s (wants ${f2(secsOf('m1014', 2))}), ${r.mag}+${r.chamber}, carries ${r.carried} · R again began a reload: ${none.began}`); }
    // a teammate's beat, and a clock that only counts
    { const R: any = await hushed(() => rig(17, { stage: false })); raise(); const g = R.game; K.KIT.on = true; g.guards.length = 0;
      K.KIT.members[1] = K.kitDefault(1); K.kitSet(1, 'primary', 'm1014');
      const p = g.player.char.pos, ops = [spawnUnit(g, { team: 'operative', pos: [p[0] + 1, 0, p[2] + 1], yaw: 0, name: 'Op0', light: false })]; K.kitStage(g, ops); for (let f = 0; f < 30; f++) R.step();
      const gd: any = ops[0], w = gd.weapon, beat = (tube: number, chamber: number) => { w.rearm(); w.mag = tube; w.chamber = chamber; const a: any[] = []; R.W.audioLog = a; startReloadBeat(g, gd); const len = gd.reloadT; let mid = -1, ticks = 0;
        while (gd.reloadT >= 0 && ticks < 900) { tickReloadBeat(g, gd, DT); ticks++; R.step(); if (mid < 0 && a.filter(e => e.name === 'shellIn').length >= 3) mid = (w.mag ?? 0) + w.chamber; }
        R.W.audioLog = null; return { len, ticks, mid, full: (w.mag ?? 0) + w.chamber, shells: a.filter(e => e.name === 'shellIn').length }; };
      const dry = beat(0, 0), part = beat(4, 1);
      const w2 = new Rifle(WEAPONS.m870); w2.mag = 0; w2.chamber = 0; const row = GR.feedBegin(w2), heard: string[] = [], g2 = { audio: { play: (n: string) => { heard.push(n); } } };
      for (let t = 0; t < row.dur + 0.1; t += DT) GR.feedTick(g2, { pos: [0, 0, 0] }, w2.def, t, t + DT, 0.7, w2);
      // a beat dropped part way (his gun put away, his fight reset): what went in shoots, and the next beat is for the room that is left
      const w3 = new Rifle(WEAPONS.m870); w3.mag = 0; w3.chamber = 0; GR.feedBegin(w3); for (let t = 0; t < 1.5; t += DT) GR.feedTick({ audio: { play: () => {} } }, { pos: [0, 0, 0] }, w3.def, t, t + DT, 0.7, w3);
      const cut = { rounds: (w3.mag ?? 0) + w3.chamber, fires: w3.canFire() }, again = GR.feedBegin(w3).dur;
      F.verdict(cut.rounds === 3 && cut.fires && Math.abs(again - secsOf('m870', 4)) < 1e-6,
        'a beat dropped after three shells leaves him a gun that shoots with three in it; and his next beat is for the room left in the tube, four shells',
        `after the drop: ${cut.rounds} in it, may fire ${cut.fires} · the next beat ${f2(again)} s (wants ${f2(secsOf('m870', 4))})`);
      // every clock that waits a reload out asks the gun (read off the source: the room fight's and the covering fire's are not run here)
      const src = async (f: string) => await Bun.file(`${ROOT}/src/game/${f}`).text(), [rf, cf, gs, mo, gm] = await Promise.all(['roomfight.ts', 'coverfire.ts', 'guards.ts', 'morale.ts', 'game.ts'].map(src));
      const sites = { 'the room fight begins': /O\.reloadUntil = g\.time \+ feedBegin\(gd\.weapon\)\.dur/.test(rf), 'the room fight ticks': /gd\.weapon\.reloadRow\.dur/.test(rf) && !/weapon\.def\.reload\.dur/.test(rf), 'covering fire': /O\.reloadUntil = g\.time \+ feedBegin\(gd\.weapon\)\.dur/.test(cf) && !/weapon\.def\.reload\.dur/.test(cf),
        'a guard\'s beat': /gd\.reloadT = feedBegin\(gd\.weapon\)\.dur/.test(gs), 'the long burst run dry': /feedBegin\(gd\.weapon\)/.test(mo) && !/weapon\.def\.reload\.dur/.test(mo), 'the teammate\'s hands': /const R = gd\.weapon\.reloadRow/.test(gm) };
      F.verdict(Object.values(sites).every(Boolean), 'every clock that waits a man\'s reload out begins it by the gun as it stands and reads the beat off the gun (read in the source)', Object.entries(sites).map(([k, v]) => `${k}: ${v}`).join(' · '));
      F.verdict(w.def.id === 'm1014' && Math.abs(dry.len - secsOf('m1014', 7)) < 1e-6 && dry.shells === 7 && dry.mid === 3 && dry.full === 8 && Math.abs(part.len - secsOf('m1014', 3)) < 1e-6 && part.shells === 3 && part.full === 8
        && Math.abs(row.dur - secsOf('m870', 6)) < 1e-6 && J(heard) === J(ownList('m870', true, 6)) && w2.mag === 5 && w2.chamber === 1,
        'a teammate\'s beat is as long as the shells that go in — a dry M1014 the whole tube\'s time with three in it after the third is heard, one four shells up three shells\' time — and full after it; a clock that only counts (the room fight\'s) books and plays the pump gun\'s six and its pump',
        `dry: ${f2(dry.len)} s, ${dry.shells} shells, ${dry.mid} in it after the third, ${dry.full} after · four up: ${f2(part.len)} s, ${part.shells} shells, ${part.full} after · the counting clock: ${f2(row.dur)} s, ${heard.join(' ')}, ${w2.mag}+${w2.chamber} in it`); }
    all = F.done() && all;
  });
  return all;
}

export async function gunreloadCheck(): Promise<boolean> {
  let all = true;
  await kept(async () => {
    { const F = family('gunreload-you: each gun reloads its own way in your hands');
      const on = await pass();
      judge(F, on);
      const H = family('gunreload-hands: what the support hand does and carries');
      judgeHands(H, on);
      all = F.done() && all; all = H.done() && all; }
    { const F = family('gunreload-mate: a teammate\'s reload beat is his gun\'s own');
      const GUNS = ['mk12', 'mp7', 'mk46', 'm1014'] as const;
      const run = async () => {
        const R: any = await hushed(() => rig(17, { stage: false })); raise(); const g = R.game; K.KIT.on = true; g.guards.length = 0;
        for (let i = 0; i < 4; i++) { K.KIT.members[i + 1] = K.kitDefault(i + 1); K.kitSet(i + 1, 'primary', GUNS[i]); }
        const p = g.player.char.pos, ops = GUNS.map((_, k) => spawnUnit(g, { team: 'operative', pos: [p[0] + 1 + k, 0, p[2] + 1], yaw: 0, name: `Op${k}`, light: false }));
        K.kitStage(g, ops); for (let f = 0; f < 30; f++) R.step();
        return ops.map((gd: any) => { const w = gd.weapon, id = w.def.id as Id; w.mag = 0; w.chamber = 0; const a: any[] = []; R.W.audioLog = a; const items0 = new Set(g.items.items), t0 = g.time;
          startReloadBeat(g, gd); let ticks = 0, cover = 0, boxes = 0; const began = gd.reloadT > 0;
          while (gd.reloadT >= 0 && ticks < 900) { tickReloadBeat(g, gd, DT); ticks++; R.step(); const s = GR.reloadShow(gd.char, id), bx: any[] = []; GR.reloadHandBoxes(gd.char, id, bx); if (s) cover = Math.max(cover, s.cover); if (bx.length === 1 && bx[0].owner === (gd.char.id & 0xff)) boxes++; }
          R.W.audioLog = null;
          return { id, began, ticks, full: w.mag + w.chamber, cap: w.def.magCapacity, cover, boxes, foley: a.filter(e => FOLEY.has(e.name)).map(e => ({ name: e.name as string, t: e.t - t0 })), drops: g.items.items.filter((x: any) => !items0.has(x) && x.kind === 'mag').map((x: any) => x.fill) }; });
      };
      const on = await run(), nm = (r: any) => r.foley.map((e: any) => e.name);
      F.verdict(on.every((r: any) => J(nm(r)) === J(r.id === 'mk12' || r.id === 'mp7' ? ['magOut'] : ownList(r.id, true, r.cap))) && J(on.map((r: any) => r.drops)) === J([[6], [7], [5], []]),
        'the belt gun and the tube gun are heard as themselves on his clock (as guns run dry: the whole box, the whole tube, the first shell in the port), the magazine guns clack as ever, and what falls is the gun\'s own — nothing out of the tube gun', on.map((r: any) => `${r.id}: ${nm(r).join(' ')} · ${r.drops.length ? `fill ${r.drops.join(',')}` : 'nothing falls'}`).join('\n    '));
      F.verdict(on[2].cover > 0.99 && on.every((r: any) => r.boxes > 3), 'the hand\'s stations are dealt to him too: the Mk 46\'s cover comes up on him, and each of the four has a box of his gun\'s own in his support hand on some frames', on.map((r: any) => `${r.id}: cover ${f2(r.cover)}, ${r.boxes} frames with a box`).join(' · '));
      // the room fight's clock (roomfight.ts reloadUntil → gunreload.ts feedTick): the same list off a clock that only counts
      { const heard: string[] = [], g = { audio: { play: (n: string) => { heard.push(n); } } }, c = { pos: [0, 0, 0] }, def = WEAPONS.mk46;
        for (let t = 0; t < def.reload.dur + 0.1; t += DT) GR.feedTick(g, c, def, t, t + DT);
        const quiet: string[] = [], g2 = { audio: { play: (n: string) => { quiet.push(n); } } }; for (let t = 0; t < 3; t += DT) GR.feedTick(g2, c, WEAPONS.mk12, t, t + DT);
        F.verdict(J(heard) === J(ownList('mk46', true, 100)) && quiet.length === 0, 'a clock that only counts (the room fight\'s) plays the belt gun\'s six once each in order, and nothing for a magazine gun', `Mk 46: ${heard.join(' ')} · Mk 12: ${quiet.length}`); }
      all = F.done() && all; }
  });
  return all;
}

if (import.meta.main) { let ok = false; try { ok = await gunreloadCheck(); ok = await gunreloadShells() && ok; } catch (e) { console.log(`- FAIL — gunreload-check threw\n    ${(e as Error).stack ?? (e as Error).message}`); }
  console.log(`\n=== gunreload-check: ${ok ? 'ALL PASS' : 'FAILED'} ===`); process.exit(ok ? 0 : 1); }
