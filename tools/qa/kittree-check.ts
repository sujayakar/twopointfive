// THE GEAR SCREEN AS A TREE (src/game/kittree.ts KITTREE: a settled rule, so nothing here asks the card of the build before it or writes the switch.
// Until 2026-11-07 a probe `off` did, and lines of `rows` and `mate40`. `KITTREE_BREAK=1`, read by kittree.ts itself, is the card asked with the tree never on: `rows` must fail)
//   rows    one list of rows for every man: each gun's own suppressor, the light and what the launcher fires hang UNDER a gun (KitRow.under), your card has
//           the suppressor, light and goggles rows a teammate's has, and a row the card does not have as it stands (no tube on this gun) is not there to step
//   guns    a suppressor is per gun: off your primary the same gun deploys with a bare muzzle, off a teammate's sidearm he is staged with the bare pistol
//           behind a suppressed carbine; a gun made with one, or that none fits, says so and is refused; the four bars set every gun apart
//   sponge  sponge rounds are a counted row: none as issued and none on a loud deploy, the launcher's five handed over the card on a quiet or solo run,
//           a pouch each when you pick them; no light on the card is a dead L, no goggles a dead N
// Nothing here looks at a picture (the page: kittree-page.cjs).
import { ROOT } from './headless.ts';
import { asProbes, exitCode, wanted } from './holds.ts';   // (run alone, each part runs as the probe the runner calls it by: one stand both ways)
import { rig } from './doortools-probes.ts';

const { KITTREE } = await import(`${ROOT}/src/game/kittree.ts`);
const { GUNSLOTS, gunPicks, pickRow } = await import(`${ROOT}/src/game/gunslots.ts`);
const K = await import(`${ROOT}/src/game/kit.ts`);
const { WEAPONS } = await import(`${ROOT}/src/game/weapons.ts`);
const { spawnUnit } = await import(`${ROOT}/src/game/guards.ts`);
const { GUN_STATS, gunStats } = await import(`${ROOT}/src/ui/gunStats.ts`);

type Verdict = (ok: boolean, label: string, detail: string) => void;
function family(title: string): { verdict: Verdict; done: () => boolean } {
  console.log(`\n=== ${title} ===`); let all = true;
  return { verdict: (ok, label, detail) => { all &&= ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${label}\n    ${detail}`); }, done: () => { console.log(`=== ${title.split(':')[0]}: ${all ? 'ALL PASS' : 'FAILURES above'} ===`); return all; } };
}
const hushed = async <T>(fn: () => Promise<T> | T): Promise<T> => { const log = console.log, err = console.error, warn = console.warn; console.log = console.error = console.warn = () => { }; try { return await fn(); } finally { console.log = log; console.error = err; console.warn = warn; } };
const J = (o: unknown) => JSON.stringify(o), f2 = (x: number) => (Number.isFinite(x) ? x.toFixed(2) : String(x));
/** the card as the page has it: the kit on, one big slot a card (both are a run's own to write; the settled rows are not written) */
const raise = () => { K.BIGSLOT.on = true; K.KIT.on = true; };
/** every table and card put back as found */
async function kept<T>(fn: () => Promise<T>): Promise<T> {
  const big = K.BIGSLOT.on, kit = K.KIT.on, cards = JSON.stringify(K.KIT.members);
  try { return await fn(); } finally { K.BIGSLOT.on = big; K.KIT.on = kit; K.KIT.members.splice(0, K.KIT.members.length, ...JSON.parse(cards)); }
}
const issue = () => { for (let i = 0; i < K.KIT.members.length; i++) K.KIT.members[i] = K.kitDefault(i); };
const rows = (i: number) => K.kitRows(i) as any[];
const row = (i: number, f: string) => rows(i).find(r => r.field === f);
/** the rows a card has as it stands, each with the gun it hangs under */
const shape = (i: number) => { const m = K.KIT.members[i]; return rows(i).filter(r => K.rowHas(r, m)).map(r => `${r.field}${r.under !== undefined ? `<${K.rowUnder(r, m)}` : ''}`); };
const LAUNCHER = ['rings', 'shells', 'cams'];

// ───────────────────────────────────────────────────────────── rows
export async function kittreeRows(): Promise<boolean> {
  const F = family('kittree-rows: one list of rows for every man, what rides on a gun under it');
  await kept(async () => {
    raise(); issue(); const you = shape(0), mate = shape(1);
    const under = (s: string[], f: string) => s.find(x => x.startsWith(`${f}<`))?.split('<')[1] ?? '';
    F.verdict(under(you, 'suppressed') === 'primary' && under(you, 'sideCan') === 'secondary' && under(mate, 'suppressed') === 'primary' && under(mate, 'sideCan') === 'secondary',
      'each gun has its own suppressor row under it, on your card and on a teammate\'s', `you: ${you.join(' ')}\n    Red One: ${mate.join(' ')}`);
    F.verdict(under(you, 'light') === 'primary' && under(mate, 'light') === 'primary' && you.includes('nvg') && mate.includes('nvg'),
      'your card has the weapon light (under the gun in your hands) and the goggles, as a teammate\'s has', `you: ${you.join(' ')}`);
    // the same rows, in the same order, but for what only your launcher fires and what a card had of its own before the switch
    const strip = (s: string[]) => s.map(x => x.split('<')[0]).filter(f => !LAUNCHER.includes(f));
    F.verdict(J(strip(you)) === J(strip(mate)), 'your card and a teammate\'s are the same rows in the same order (the launcher\'s loads aside)',
      `you: ${strip(you).join(' ')}\n    Red One: ${strip(mate).join(' ')}\n    set aside: ${LAUNCHER.join(' ')}`);
    F.verdict(LAUNCHER.filter(f => you.some(x => x.startsWith(`${f}<`))).every(f => under(you, f) === 'primary') && under(you, 'rings') === 'primary', 'what the launcher fires hangs under the primary, sponge rounds among it', you.filter(x => LAUNCHER.includes(x.split('<')[0])).join(' ') || 'none');
    // a primary with no tube: the launcher's rows are not on the card, and cannot be stepped from under it
    const longs = gunPicks('primary') as string[], noTube = longs.find(id => id !== 'none' && WEAPONS[pickRow(id, true)]?.launcher === false);
    if (noTube && !row(0, 'rings')) F.verdict(false, 'a Sponge rounds row to step', 'the card has none');
    else if (noTube) { K.kitSet(0, 'primary', noTube); const s = shape(0), t = K.kitTry(0, 'rings', 1), set = K.kitSet(0, 'rings', 2);
      F.verdict(!s.some(x => LAUNCHER.includes(x.split('<')[0])) && !t.ok && !set && K.KIT.members[0].rings === 0, `a primary with no launcher (${noTube}): its loads are off the card and a step at one is refused`, `${s.join(' ')} · step: ${t.ok ? 'TAKEN' : t.why} · set: ${set}`); }
    else F.verdict(false, 'a primary with no launcher is on offer', longs.join(' '));
    // no long gun: the light hangs under the sidearm, the primary's suppressor is gone
    issue(); K.kitSet(1, 'primary', 'none'); const alone = shape(1);
    F.verdict(under(alone, 'light') === 'secondary' && !alone.some(x => x.startsWith('suppressed<')), 'the pistol alone: the light is under the sidearm and no row is left for a gun not brought', alone.join(' '));
    // a suppressor's flag holds only while its gun has the choice: off, then the gun left at home, is on again; a paste of a gun that takes none takes nobody's off
    issue(); K.kitSet(0, 'suppressed', false); const off = K.KIT.members[0].suppressed; K.kitSet(0, 'primary', 'none'); const home = K.KIT.members[0].suppressed; K.kitSet(0, 'primary', 'mk18'); const back = K.kitSlotCan(K.KIT.members[0], 'primary');
    const none = (gunPicks('primary') as string[]).find(id => id !== 'none' && !pickRow(id, true) || (id !== 'none' && WEAPONS[pickRow(id, true)]?.suppressed === false));
    issue(); if (none) { K.kitSet(1, 'primary', none); const p = K.kitPaste(2, K.KIT.members[1]); K.kitSet(1, 'primary', 'mk18'); K.kitSet(2, 'primary', 'mk18');
      F.verdict(off === false && home === true && back === true && p.ok && K.kitSlotCan(K.KIT.members[1], 'primary') && K.kitSlotCan(K.KIT.members[2], 'primary'), `a flag holds only while its gun has the choice: yours off, the gun left home and brought again, is on; Red One's ${none} pasted onto Red Two, both back on the Mk 18, both suppressed`,
        `yours: ${off} → home ${home} → back ${back} · Red One ${K.KIT.members[1].suppressed}, Red Two ${K.KIT.members[2].suppressed}`); }
    else F.verdict(false, 'a primary no suppressor fits is on offer', (gunPicks('primary') as string[]).join(' '));
    // the line under a man's name says which gun is bare, by the names his tiles read
    issue(); K.kitSet(1, 'suppressed', false); const w1 = K.kitWords(1); K.kitSet(1, 'suppressed', true); K.kitSet(1, 'sideCan', false); const w2 = K.kitWords(1); K.kitSet(0, 'suppressed', false); const w0 = K.kitWords(0); issue(); const wi = [K.kitWords(0), K.kitWords(1)];
    const bareIn = (w: string) => w.split(' · ')[0].split(' + ').map(g => / bare$/.test(g)).join(',');   // (which of the two guns the line calls bare, whatever names this stand gives them)
    F.verdict(bareIn(w1) === 'true,false' && bareIn(w2) === 'false,true' && bareIn(w0) === 'true,false' && wi.every(w => bareIn(w) === 'false,false'), 'the line under a name follows a suppressor taken off: that gun reads bare, the other does not, and as issued neither does — for you as for a teammate',
      `Red One, primary off: '${w1}' · sidearm off: '${w2}' · you, primary off: '${w0}' · as issued: '${wi[1]}'`);
    // the row of kits for a man alone: the kits that differ on HIS card, put on his card and nobody else's
    const slotsWas = K.SLOTS.on; K.SLOTS.on = K.SLOTS.ok;   // (the named kits are the pouches' table's: on a stand that holds the pouches off the row has the one)
    issue(); K.kitSet(1, 'bangs', 0); const mateWas = J(K.KIT.members[1]), all = K.kitLoadouts().map((l: any) => l.id), mine = K.kitLoadouts(0).map((l: any) => l.id), other = mine.find((id: string) => id !== 'issued');
    const why = other ? K.kitLoadout(other, 0) : 'no kit of his own', now = K.kitLoadoutOf(0);
    F.verdict(mine[0] === 'issued' && mine.length >= 2 && mine.length <= all.length && why === null && now === other && J(K.KIT.members[1]) === mateWas, 'alone, the row lists the kits that put a different card on you, and a press writes your card only: a teammate\'s own card is as he left it',
      `all: ${all.join(' ')} · for you: ${mine.join(' ')} · put on '${other}': ${why ?? 'taken'}, the row reads '${now}' · Red One untouched: ${J(K.KIT.members[1]) === mateWas}`);
    K.SLOTS.on = slotsWas; issue();
    // a card in the store from before the tree: yours is as issued in the fields it never had; a teammate's one old Suppressor row is both his guns'
    const d0 = K.kitDefault(0) as any, old: any = { ...d0, suppressed: false, light: false, nvg: false }; delete old.sideCan; delete old.rings; const old0 = K.kitSanitize(0, old);
    const oldMate: any = { ...K.kitDefault(1), suppressed: false }; delete oldMate.sideCan; delete oldMate.rings; const old1 = K.kitSanitize(1, oldMate);
    F.verdict(old0.suppressed === true && old0.sideCan === true && old0.light === true && old0.nvg === true && old1.suppressed === false && old1.sideCan === false,
      'the store: your card from before the tree has its suppressors, light and goggles as issued; a teammate\'s one old Suppressor row is both his guns\'',
      `yours from before: ${J({ suppressed: old0.suppressed, sideCan: old0.sideCan, light: old0.light, nvg: old0.nvg })} · Red One's: ${J({ suppressed: old1.suppressed, sideCan: old1.sideCan })}`);
  });
  return F.done();
}

// ───────────────────────────────────────────────────────────── guns
export async function kittreeGuns(): Promise<boolean> {
  const F = family('kittree-guns: a suppressor is its gun\'s, and every gun reads apart');
  await kept(async () => {
    const R: any = await hushed(() => rig(17, { stage: false })); raise(); issue(); const g = R.game, at = g.player.char.pos;
    // yours: off the primary only, then off the sidearm only
    K.kitPlayer(g); 
    K.kitSet(0, 'suppressed', false); K.kitPlayer(g); 
    issue(); K.kitSet(0, 'sideCan', false); K.kitPlayer(g); 
    const say = (d: any) => `${d.id}${d.suppressed ? ' (suppressed' : ' (bare'}, ${d.report.voice} ${f2(d.report.level)})`;
    // a teammate: a suppressed carbine with a bare sidearm behind it, and the other way about
    const ops = [spawnUnit(g, { team: 'operative', pos: [at[0] + 1, 0, at[2] + 1], yaw: 0, name: 'Op', light: false })];
    issue(); K.kitSet(1, 'sideCan', false); K.kitStage(g, ops); const m1 = { hand: (ops[0] as any).weapon.def, back: WEAPONS[(ops[0] as any).second?.id] };
    issue(); K.kitSet(1, 'suppressed', false); K.kitStage(g, ops); const m2 = { hand: (ops[0] as any).weapon.def, back: WEAPONS[(ops[0] as any).second?.id] };
    F.verdict(!!m1.back && !!m2.back && m1.hand.suppressed && !m1.back.suppressed && !m2.hand.suppressed && m2.back.suppressed,
      'a teammate is staged with each gun as its own row says: a suppressed carbine over a bare sidearm, a bare carbine over a suppressed one', `sidearm's off: ${say(m1.hand)} + ${m1.back ? say(m1.back) : 'none'}\n    primary's off: ${say(m2.hand)} + ${m2.back ? say(m2.back) : 'none'}`);
    // a gun that has no choice says which way it is made, and its row cannot be moved
    issue(); const fixed: string[] = [], loose: string[] = [];
    for (const slot of ['primary', 'secondary'] as const) for (const id of gunPicks(slot) as string[]) { if (id === 'none') continue; issue(); K.kitSet(1, slot, id);
      const f = slot === 'primary' ? 'suppressed' : 'sideCan', r = row(1, f), m = K.KIT.members[1], word = (K.kitPeek(1, f, 1).why ?? '') as string, can = !!pickRow(id, true) && !!pickRow(id, false) && pickRow(id, true) !== pickRow(id, false);
      const was = K.rowGet(r, m), moved = K.kitSet(1, f, !was) && K.rowGet(r, K.KIT.members[1]) !== was;
      (can === !word && moved === can ? fixed : loose).push(`${id}: ${word || 'on / off'}${moved ? '' : ' (held)'}`); }
    F.verdict(!loose.length && fixed.some(x => x.includes('held')) && fixed.some(x => !x.includes('held')), 'every gun: its suppressor row moves only where the gun can be had both ways, and says why where it cannot', loose.length ? `WRONG ${loose.join(' · ')}` : fixed.join(' · '));
    // the four bars: no two guns of a slot read alike, a bare muzzle is less quiet
    issue(); const bars: Record<string, Record<string, number>> = {};
    for (const slot of ['primary', 'secondary'] as const) for (const id of gunPicks(slot) as string[]) { if (id === 'none') continue; issue(); K.kitSet(1, slot, id); const d = K.kitSlotDef(1, K.KIT.members[1], slot); if (d) bars[`${slot}:${id}`] = gunStats(d); }
    const ids = Object.keys(bars), near: string[] = [], range = (v: Record<string, number>) => Object.values(v).every(x => x >= 0 && x <= 1);
    for (const p of ids) for (const q of ids) if (p < q && p.split(':')[0] === q.split(':')[0] && !(GUN_STATS as any[]).some(S => Math.abs(bars[p][S.id] - bars[q][S.id]) >= 0.1)) near.push(`${p} = ${q}`);
    F.verdict(ids.length >= 6 && !near.length && ids.every(k => range(bars[k])), 'every gun a row offers has four bars inside the track, and no two guns of a slot are within a tenth on all four', near.length ? near.join(' · ') : ids.map(k => `${k.split(':')[1]} ${(GUN_STATS as any[]).map(S => f2(bars[k][S.id])).join('/')}`).join(' · '));
    issue(); const d1 = K.kitSlotDef(0, K.KIT.members[0], 'primary'); K.kitSet(0, 'suppressed', false); const d0 = K.kitSlotDef(0, K.KIT.members[0], 'primary');
    F.verdict(!!d1 && !!d0 && gunStats(d0).quiet < gunStats(d1).quiet && gunStats(d0).reach === gunStats(d1).reach, 'the suppressor off your primary: its quiet bar drops and nothing else moves', d1 && d0 ? `quiet ${f2(gunStats(d1).quiet)} → ${f2(gunStats(d0).quiet)}` : 'no row');
  });
  return F.done();
}

// ───────────────────────────────────────────────────────────── sponge
export async function kittreeSponge(): Promise<boolean> {
  const F = family('kittree-sponge: sponge rounds are a row, and only a quiet run is handed them');
  await kept(async () => {
    const R: any = await hushed(() => rig(17, { stage: false })); raise(); issue(); const g = R.game, pl = () => g.player;
    const quiet = (on: boolean) => { g.soloCard = on; if (on) g.guards = g.guards.filter((gd: any) => !(gd.unit && gd.team === 'operative')); };
    const r = row(0, 'rings'), issued = K.kitDefault(0).rings;
    F.verdict(!!r && issued === 0 && [0, 1, 2, 3, 4].every(i => K.kitDefault(i).rings === 0), 'a Sponge rounds row is on your card, and nobody is issued one', `row: ${r ? `${r.label} ${J(r.values)}` : 'none'} · issued ${[0, 1, 2, 3, 4].map(i => K.kitDefault(i).rings).join(' ')}`);
    quiet(false); K.kitPlayer(g); const loud = { rings: pl().rifle.rings, load: pl().rifle.attachment, tube: pl().rifle.attachments.includes('ring') };
    quiet(true); K.kitPlayer(g); const still = { rings: pl().rifle.rings, load: pl().rifle.attachment };
    F.verdict(loud.tube && loud.rings === 0 && loud.load !== 'ring' && still.rings === KITTREE.lend, 'deployed as issued: no sponge round on a loud mission (the right button is on the tube\'s first load you do carry), the launcher\'s five on a quiet or solo one',
      `loud: ${loud.rings} rounds, the tube on '${loud.load}' · quiet: ${still.rings} rounds, the tube on '${still.load}'`);
    // picked: counted against the pouches like any other thing, and what a loud deploy carries
    issue(); quiet(false); const p0 = K.kitCost(0).used; K.kitSet(0, 'smoke', 0); K.kitSet(0, 'frags', 0); const took = K.kitSet(0, 'rings', 2), p2 = K.kitCost(0); K.kitPlayer(g);
    quiet(true); K.kitPlayer(g); F.verdict(pl().rifle.rings === KITTREE.lend, '… and on a quiet run the mission\'s five stand over the two', `${pl().rifle.rings} rounds`);
    // your light and goggles rows are what you deploy with
    // (asked of the key: a press of L on the game itself; which guns' lamps it moved)
    const pressL = () => { const lamps = () => `${!!pl().rifle.lightOn}${!!pl().pistol.lightOn}`, was = lamps(); R.W.input.pressed.add('KeyL'); R.W.step(1 / 60); return lamps() !== was; };
    issue(); quiet(false); K.kitPlayer(g); const on = { l: pressL() }; pressL();   // (put out again: the card taken off puts a lit lamp out by itself)
    K.kitSet(0, 'light', false); K.kitSet(0, 'nvg', false); K.kitPlayer(g); const off = { l: pressL() };
    F.verdict(on.l && !off.l, 'your card without the light: L has nothing to switch (with it, as before)', `fitted: light ${on.l} · taken off: light ${off.l}`);
    issue(); K.kitPlayer(g);
  });
  return F.done();
}

// ───────────────────────────────────────────────────────────── a teammate's 40 mm
/** "should be symmetric between self and teammates (e.g. 40mm should work for squad)": a teammate's card has the 40 mm row where his round can be fired (the
 *  tree, and the order in the build: squad40.ts — X on the floor, G, 3), and the round his card carries is the round he is staged with */
export async function kittreeMate40(): Promise<boolean> {
  const F = family('kittree-mate40: a teammate\'s card has the 40 mm row where his round can be fired, and brings the round');
  const S40 = await import(`${ROOT}/src/game/squad40.ts`), slotsWas = K.SLOTS.on;
  const maxOf = () => K.SLOTS.table.items.shells?.max as number[] | undefined, max0 = maxOf()?.slice();
  const up = () => { raise(); K.SLOTS.on = K.SLOTS.ok; };
  await kept(async () => { try {
    up(); issue(); const asWritten = max0 ? max0.slice(1) : [];
    // the table: how many a teammate may carry, as the file writes it
    F.verdict(asWritten.length > 0 && asWritten.every(n => n === 1), 'the pouches table lets every teammate carry one 40 mm grenade (slots.json shell40)', `a teammate's most, by man: ${asWritten.join(' ') || 'no such item'}`);
    { const m = maxOf(); if (m) for (let i = 1; i < m.length; i++) m[i] = Math.max(m[i], 1); }   // (the lines below hold the card's own rules whatever the file says: the one number squad40-check.ts raises too)
    const r1 = row(1, 'shells'), mate = shape(1), you = shape(0);
    F.verdict(!!r1 && r1.values.join() === '0,1' && mate.includes('shells<primary') && you.includes('shells<primary') && K.KIT.members[1].shells === 0, 'Red One\'s card has a 40 mm row under his primary, where yours is: none or one, none as issued',
      `his row: ${r1 ? r1.values.join(' ') : 'none'} · his card: ${mate.join(' ')} · as issued ${K.KIT.members[1].shells}`);
    // one taken: a pouch, and his line says it (a stun canister left behind for the room in his pouches)
    K.kitSet(1, 'bangs', 1); const c0 = K.kitCost(1).used, t = K.kitTry(1, 'shells', 1), c1 = K.kitCost(1).used, w = K.kitWords(1), top = (row(1, 'shells')?.values ?? []).slice(-1)[0];
    F.verdict(t.ok && K.KIT.members[1].shells === 1 && c1 === c0 + 1 && /40\u00a0mm/.test(w) && top === 1, 'one taken costs him a pouch and the line under his name says it; one is the most his row offers',
      `pouches ${c0} → ${c1} · '${w}' · the row's most: ${top}`);
    // a primary with no tube: the row leaves the card and the round with it
    const noTube = (gunPicks('primary') as string[]).find(id => id !== 'none' && WEAPONS[pickRow(id, true) ?? pickRow(id, false)]?.launcher === false);
    if (noTube) K.kitSet(1, 'primary', noTube); const bare = shape(1), left = K.KIT.members[1].shells; K.kitSet(1, 'primary', 'mk18');
    F.verdict(!!noTube && !bare.some(x => x.startsWith('shells')) && left === 0, 'a primary with nothing under its barrel: the row leaves his card and the round with it', `${noTube}: ${bare.join(' ')} · rounds ${left}`);
    // staged: the round on his card is his to fire, and nobody else has one
    const R: any = await hushed(() => rig(17)); up(); { const m = maxOf(); if (m) for (let i = 1; i < m.length; i++) m[i] = Math.max(m[i], 1); } issue(); K.kitSet(1, 'bangs', 1); K.kitSet(1, 'shells', 1);
    const g = R.game, team: any[] = g.nightBreach.ops; K.kitStage(g, team); const got = team.map(gd => S40.squad40Of(g, gd) as number);
    F.verdict(team.length >= 1 && got[0] === 1 && got.slice(1).every(n => n === 0), 'staged: the man whose card carries the round has it to fire on your order, and nobody else does',
      `rounds by man: ${got.join(' ')}`);
  } finally { const m = maxOf(); if (m && max0) m.splice(0, m.length, ...max0); K.SLOTS.on = slotsWas; } });
  return F.done();
}

if (import.meta.main) {
  process.exit(exitCode(await asProbes({ 'kittree-rows': kittreeRows, 'kittree-guns': kittreeGuns, 'kittree-sponge': kittreeSponge, 'kittree-mate40': kittreeMate40 }, wanted())));
}
