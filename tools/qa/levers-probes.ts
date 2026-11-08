// The 'levers' family (THE DIFFICULTY LEVERS — game/difficulty.ts LEVERS / setLever, ui/levers.ts on the mission sheet and the loadout screen), run through probes.ts:
//   bun run tools/qa/probes.ts levers | levers-rules | levers-run | levers-address          (or this file: bun run tools/qa/levers-probes.ts [name …])
// PASS / FAIL each, all headless. Nothing here looks at a picture.
//   levers-rules    the module alone: each lever moved alone writes its own rows and no other's; hard's rungs set
//                   lever by lever over standard read as hard; the level's switch puts every lever back; a moved lever is kept on this machine, read back at the
//                   next page load, left out of a Training lesson, and a ?fog= / ?enemyai= / ?enemykit= on the URL stands over it for the visit; links carry a moved lever
//                   only where the machine could not keep it, and with none moved are the links they were; every room clear's row says whether squads hold its
//                   level, as its level file has it (ScenarioEntry.squads → difficulty.ts squadsOf), and is worded so
//   levers-run      the fog lever's top rung is in force in a Game built after it (over what the machine keeps for the fog's own option)
//   levers-address  the harness stood from an address (standUp({ address })) holds the difficulty that address names before the game is made, and one that names none leaves
//                   the tables as they stood. LEVERS_ADDRESS_BREAK=1 leaves the words off: must fail
// THESE LINES WERE WRITTEN FOR THREE LEVERS and ran, until 2026-11-07, with the fourth lever and the squad fog's off rung held down (told.ts's two switches,
// by told-check.ts threeLevers). Both are settled up and the hold was refused, so it is out: a line here that counts three levers, or the fog's three rungs,
// fails by what the game is today.
import { standUp, ROOT } from './headless.ts';

const D = await import(`${ROOT}/src/game/difficulty.ts`);
const { PRESSURE } = await import(`${ROOT}/src/game/pressure.ts`);
const { DIGIN } = await import(`${ROOT}/src/game/digin.ts`);
const { FOG } = await import(`${ROOT}/src/game/fogofwar.ts`);

type Verdict = (ok: boolean, label: string, detail: string) => void;
function family(title: string): { verdict: Verdict; done: () => boolean } {
  console.log(`\n=== ${title} ===`); let all = true;
  return { verdict: (ok, label, detail) => { all &&= ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${label}\n    ${detail}`); }, done: () => { console.log(`=== ${title.split(':')[0]}: ${all ? 'ALL PASS' : 'FAILURES above'} ===`); return all; } };
}
const quiet = async <T>(fn: () => Promise<T> | T): Promise<T> => { const log = console.log, err = console.error, warn = console.warn; console.log = console.error = console.warn = () => {}; try { return await fn(); } finally { console.log = log; console.error = err; console.warn = warn; } };
const J = (o: unknown) => JSON.stringify(o);
const tables = () => J({ P: PRESSURE, D: DIGIN });
const fog = () => `${FOG.cone ? 'cone' : 'round'}${FOG.own ? '+own' : ''}`;
const shown = () => { const S = D.leversShown(); return `${S.fog}/${S.ai}/${S.kit}`; };
/** a store for the length of `fn` (null: a machine that can keep nothing), the page's own put back after */
async function withStore<T>(init: Record<string, string> | null, fn: (m: Map<string, string>) => Promise<T> | T): Promise<T> {
  const G = globalThis as any, had = 'localStorage' in G, prev = G.localStorage, m = new Map(Object.entries(init ?? {}));
  if (init === null) delete G.localStorage; else G.localStorage = { getItem: (k: string) => m.get(k) ?? null, setItem: (k: string, v: string) => { m.set(k, String(v)); }, removeItem: (k: string) => { m.delete(k); } };
  try { return await fn(m); } finally { if (had) G.localStorage = prev; else delete G.localStorage; }
}
/** the module as a fresh page finds it: standard, no lever moved, the structure a plain one, the fog's own option off */
function fresh() { D.difficultyHere(''); D.setConeVisit(null); D.setDifficulty('standard', false); FOG.own = false; }

// ───────────────────────────────────────────────────────────── rules ─────────────────────────────────────────────────────────────
export async function leversRules(): Promise<boolean> {
  const F = family('levers-rules: the difficulty levers as rules — a level puts every lever on its rung, a lever moved alone writes its own rows and no other\'s');
  const ownWas = FOG.own;
  try {
    fresh();
    // ---- the table
    { const L: any[] = D.LEVERS, keys = L.map(l => l.key).join(','), params = L.map(l => l.param);
      F.verdict(keys === 'fog,ai,kit' && L[0].rungs.length === 3 && L.every(l => l.rungs.length >= 2 && l.rungs.length <= 5 && l.rungs.every((r: any) => r.label.length <= 12 && r.words.length >= 20)) && new Set(params).size === 3 && !params.some(p => ['hard', 'difficulty', 'pressure', 'digin', 'fogcone', 'fogown'].includes(p)),
        'three levers — fog of war (three rungs), enemy AI, enemy kit — each rung with a short label and a line of words, each lever a URL key of its own', L.map(l => `${l.label} [?${l.param}=]: ${l.rungs.map((r: any) => r.id).join(' · ')}`).join('\n    ')); }
    // ---- each lever alone, over standard
    { const std = tables(), stdFog = fog(); D.setDifficulty('hard', false); const hard = tables(); D.setDifficulty('standard', false);
      const hardP = JSON.parse(hard).P, hardNoKit = J({ P: { ...hardP, stuns: 0 }, D: JSON.parse(hard).D });
      D.setLever('fog', 'cones', false); const a = { t: tables(), f: fog(), now: D.difficultyNow(), mix: D.leversMixed() };
      D.setLever('fog', 'own', false); const b = { t: tables(), f: fog() };
      D.setLever('fog', 'team', false); const c = { t: tables(), f: fog(), now: D.difficultyNow() };
      D.setLever('fog', null, false); const d = { t: tables(), f: fog(), now: D.difficultyNow(), set: J(D.leversSet()) };
      F.verdict(a.t === std && a.f === 'cone' && a.now === 'custom' && a.mix && b.t === std && b.f === 'cone+own' && c.t === std && c.f === 'round' && c.now === 'standard' && d.t === std && d.f === stdFog && d.now === 'standard' && d.set === '{}',
        'the fog lever alone, over standard: cones, then your own eyes, then back — the defenders\' rows never move; the level\'s own rung set by hand still reads as the level; let go, it is the level\'s again',
        `cones → fog ${a.f}, reads ${a.now} · own → ${b.f} · team → ${c.f}, reads ${c.now} · let go → ${d.f}, moved ${d.set}`);
      D.setLever('ai', 'hard', false); const e = { t: tables(), f: fog(), now: D.difficultyNow(), s: shown() }; D.setLever('ai', null, false);
      F.verdict(e.t === hardNoKit && e.f === stdFog && e.now === 'custom' && tables() === std, 'the AI lever alone: every row hard\'s defenders stand on (the press, the dig-in join, the drilled room) and nothing of the fog or the kit; let go, standard\'s rows to the byte',
        `${e.s} · fog ${e.f} · press ${JSON.parse(e.t).P.on} · dig in ${JSON.parse(e.t).D.on} · stuns ${JSON.parse(e.t).P.stuns}`);
      D.setLever('kit', 'stuns', false); const k = { t: tables(), f: fog(), now: D.difficultyNow(), s: shown() }; D.setLever('kit', null, false);
      const stdP = JSON.parse(std).P;
      F.verdict(k.t === J({ P: { ...stdP, stuns: D.LEVER_STUNS }, D: JSON.parse(std).D }) && k.f === stdFog && k.now === 'custom' && D.LEVER_STUNS === 1 && tables() === std, 'the kit lever alone: a stun charge each and no other row — the press stays off',
        `${k.s} · press ${JSON.parse(k.t).P.on} · stuns ${JSON.parse(k.t).P.stuns}`);
      // ---- hard, built lever by lever, is hard; hard with one lever let down is a mix with the others in force
      D.setLever('fog', 'cones', false); D.setLever('ai', 'hard', false); const built = { t: tables(), f: fog(), now: D.difficultyNow(), mix: D.leversMixed() };
      D.setDifficulty('hard', false); const h = { t: tables(), f: fog(), set: J(D.leversSet()) };
      D.setLever('fog', 'team', false); const soft = { t: tables(), f: fog(), now: D.difficultyNow(), mix: D.leversMixed() };
      D.setDifficulty('standard', false);
      F.verdict(built.t === hard && built.f === 'cone' && built.now === 'hard' && !built.mix && h.t === hard && h.f === 'cone' && h.set === '{}' && soft.t === hard && soft.f === 'round' && soft.now === 'custom' && soft.mix && tables() === std && fog() === stdFog,
        'hard\'s rungs set lever by lever over standard ARE hard (rows to the byte, and it reads hard); the level\'s switch puts every lever back; hard with the fog let down is hard\'s defenders seen all round',
        `built: reads ${built.now} · hard, fog → team: reads ${soft.now}, fog ${soft.f}, press ${JSON.parse(soft.t).P.on}`); }
    // ---- the fog's own option (FOG.own) is the lever's only while the lever is moved
    { FOG.own = true; D.setDifficulty('hard', false); const kept = fog();
      D.setLever('fog', 'cones', false); const over = fog(); D.setLever('fog', null, false); const back = fog(); D.setDifficulty('standard', false); const std = fog(); FOG.own = false;
      F.verdict(kept === 'cone+own' && over === 'cone' && back === 'cone+own' && std === 'round+own', 'a machine that keeps the fog\'s own-eyes option: the level leaves it alone, the fog lever stands over it while moved, and it is back when the lever lets go', `hard ${kept} · lever on cones ${over} · let go ${back} · standard ${std}`); }
    // ---- the structure: a level's own kit follows it where the kit lever was left alone
    { D.setDifficulty('hard', false); D.setLever('fog', 'own', false); D.difficultyHere('the block'); const a = `${shown()} stuns ${PRESSURE.stuns}`, aOk = PRESSURE.stuns === 1 && FOG.own;
      D.setLever('kit', 'none', false); const b = `${shown()} stuns ${PRESSURE.stuns}`, bOk = PRESSURE.stuns === 0 && PRESSURE.on;
      D.difficultyHere(''); D.setLever('kit', null, false); const c = `${shown()} stuns ${PRESSURE.stuns}`, cOk = PRESSURE.stuns === 0;
      D.setDifficulty('standard', false); D.setLever('kit', 'stuns', false); D.difficultyHere('the block'); const d = `${shown()} stuns ${PRESSURE.stuns}`, dOk = PRESSURE.stuns === 1 && !PRESSURE.on;
      fresh();
      F.verdict(aOk && bOk && cOk && dOk, 'hard with a lever moved, started on The block: its stun charges come with the level; the kit lever on guns takes them off there and nowhere else changes; standard with the kit lever on stuns carries them onto any level',
        `hard + fog own on The block: ${a} · kit → guns: ${b} · off The block, kit let go: ${c} · standard + stuns on The block: ${d}`); }
    // ---- kept on this machine; the URL over it for a visit; not on a Training lesson
    await withStore({}, m => {
      D.setLever('fog', 'own'); D.setLever('kit', 'stuns'); const stored = m.get('twopointsix.levers') ?? '', name = m.get('twopointsix.difficulty') ?? '(none)';
      fresh(); D.initDifficulty(new URLSearchParams('')); const a = shown();
      fresh(); D.initDifficulty(new URLSearchParams('difficulty=hard')); const b = `${shown()} base ${D.leversBase()}`;
      fresh(); D.initDifficulty(new URLSearchParams('fog=team&enemyai=hard&enemykit=junk')); const c = shown();
      fresh(); D.initDifficulty(new URLSearchParams(''), true); const p = `${shown()} moved ${J(D.leversSet())}`;
      fresh(); D.initDifficulty(new URLSearchParams('')); D.setDifficulty('hard'); const cleared = m.get('twopointsix.levers') ?? '(none)', nameH = m.get('twopointsix.difficulty');
      D.setLever('ai', 'standard'); const overHard = `${m.get('twopointsix.levers')} over ${m.get('twopointsix.difficulty')}`;
      fresh(); D.initDifficulty(new URLSearchParams('')); const d = `${shown()} base ${D.leversBase()}`;
      D.setDifficulty('standard'); const end = `${m.get('twopointsix.levers') ?? '(none)'} · ${m.get('twopointsix.difficulty') ?? '(none)'}`;
      m.set('twopointsix.levers', '{"fog":"sideways","ai":7,"kit":"stuns","extra":1}'); fresh(); D.initDifficulty(new URLSearchParams('')); const junk = `${shown()} moved ${J(D.leversSet())}`;
      fresh();
      F.verdict(stored === '{"fog":"own","kit":"stuns"}' && name === '(none)' && a === 'own/standard/stuns' && b === 'own/hard/stuns base hard' && c === 'team/hard/stuns' && p === 'team/standard/none moved {}'
        && cleared === '(none)' && nameH === 'hard' && overHard === '{"ai":"standard"} over hard' && d === 'cones/standard/none base hard' && end === '(none) · (none)' && junk === 'team/standard/stuns moved {"kit":"stuns"}',
        'a moved lever is kept on this machine and stands over whichever level the next page starts at; the URL\'s own lever stands over the kept one for the visit; a Training lesson reads none; the level\'s switch drops them; a rung the build does not have is not read',
        `stored ${stored} · next load ${a} · on a ?difficulty=hard link ${b} · ?fog=team&enemyai=hard ${c} · a lesson ${p}\n    Hard pressed: levers ${cleared}, name ${nameH} · AI let down over it: ${overHard} → next load ${d} · Standard pressed: ${end} · a store with junk in it: ${junk}`);
    });
    // ---- links
    { const plain = '?roomclear&squad4&mute';
      const none = { std: D.difficultyHref(plain), stdL: D.leversHref(plain) }; D.setDifficulty('hard', false); const noneH = { h: D.difficultyHref(plain), l: D.leversHref(`${plain}&difficulty=hard`) }; D.setDifficulty('standard', false);
      const kept = await withStore({}, () => { D.setLever('fog', 'own'); const r = { d: D.difficultyHref(plain), over: D.difficultyHref(`${plain}&difficulty=hard&fog=team`), l: D.leversHref(`${plain}&difficulty=hard&fog=team&enemykit=stuns`) }; D.setDifficulty('standard'); return r; });
      const bare = await withStore(null, () => { D.setLever('fog', 'own'); D.setLever('ai', 'hard'); const r = { d: D.difficultyHref(plain), l: D.leversHref(`${plain}&difficulty=hard&fog=team`) }; D.setDifficulty('standard'); return r; });
      fresh();
      F.verdict(none.std === plain && none.stdL === plain && noneH.h === `${plain}&difficulty=hard` && noneH.l === `${plain}&difficulty=hard`, 'no lever moved: the links are the links they were (standard on a link that names no level is left alone; hard rides on it)', `${none.std} · hard ${noneH.h}`);
      F.verdict(kept.d === `${plain}&difficulty=standard` && kept.over === `${plain}&difficulty=standard` && kept.l === `${plain}&difficulty=hard`, 'a moved lever the machine KEPT: the link names the level the levers stand over and carries no lever (the page it loads reads them back); a link that names its own level keeps it, and loses whatever levers it carried',
        `${kept.d} · a row's own hard link → ${kept.l}`);
      F.verdict(bare.d === `${plain}&difficulty=standard&fog=own&enemyai=hard` && bare.l === `${plain}&difficulty=hard&fog=own&enemyai=hard`, 'where nothing can be kept the link carries the moved levers itself, in place of whatever it carried', `${bare.d} · ${bare.l}`); }
    // ---- other roads: the kept own-eyes option, a lever the visitor arrived with, rows moved by hand, a lesson's page, a level's own kit
    { const plain = '?roomclear&squad4&mute', flip = () => { PRESSURE.second = !PRESSURE.second; };   // (a row moved by hand: one a level's own start writes — the table's first, `press`, is settled and takes no write)
      // the Game's constructor lays the fog lever over FOG.own (leversFog): own eyes held by the lever goes when the lever lets go
      fresh(); D.setLever('fog', 'own', false); D.leversFog(); const heldOn = FOG.own; D.setDifficulty('standard', false); const a = `${heldOn ? 'own' : 'NOT own'} → ${FOG.own ? 'STILL own' : 'off'}`, aOk = heldOn && !FOG.own;
      F.verdict(aOk, 'a new Game lays the fog lever over FOG.own: own eyes held by the lever goes when the lever lets go', `lever own eyes: ${a}`);
      // a page that arrived with ?fog=own, then Standard pressed: the link loses it
      const arrived = await withStore({}, () => { fresh(); D.initDifficulty(new URLSearchParams('fog=own')); const on = D.difficultyHref(`${plain}&fog=own`); D.setDifficulty('standard'); const off = D.difficultyHref(`${plain}&fog=own`), offL = D.leversHref(`${plain}&difficulty=hard&fog=own`); return { on, off, offL }; });
      F.verdict(arrived.on === `${plain}&difficulty=standard&fog=own` && arrived.off === plain && arrived.offL === `${plain}&difficulty=hard`, 'a lever the visitor ARRIVED with (?fog=own) rides the links while it stands, and is off them once the level\'s switch has let it go', `${arrived.on} · Standard pressed: ${arrived.off}`);
      // levers pressed in another order than the table's are still KEPT ones (the link carries none)
      const order = await withStore({}, () => { fresh(); D.setLever('kit', 'stuns'); D.setLever('fog', 'own'); const h = D.difficultyHref(plain); D.setDifficulty('standard'); return h; });
      F.verdict(order === `${plain}&difficulty=standard`, 'levers moved kit first, fog second, are kept ones like any: the link carries no lever', order);
      // the tables moved by another road since the level was set: the level is read back off them, and the fog lever writes no defenders' row
      fresh(); D.setDifficulty('hard', false); PRESSURE.on = false; DIGIN.on = false; const read = D.difficultyNow(); D.setLever('fog', 'cones', false); const c = `reads ${read} · fog lever moved: press ${PRESSURE.on ? 'BACK ON' : 'off'}, level under the levers ${D.leversBase()}`, cOk = read === 'standard' && !PRESSURE.on && !DIGIN.on && D.leversBase() === 'standard';
      fresh(); D.setDifficulty('hard', false); flip(); const t = tables(); D.setLever('fog', 'own', false); const d = `rows ${tables() === t ? 'as they stood' : 'REWRITTEN'} · fog ${fog()}`, dOk = tables() === t && fog() === 'cone+own' && D.difficultyNow() === 'custom';
      const cust = await withStore({}, m => { fresh(); D.setLever('ai', 'hard'); D.setLever('fog', 'own'); flip(); D.difficultyCustom(); const kept = m.get('twopointsix.levers') ?? '(none)', name = m.get('twopointsix.difficulty') ?? '(none)';
        fresh(); D.initDifficulty(new URLSearchParams('')); return { kept, name, fogOnly: fog() === 'cone+own' }; });
      fresh();
      F.verdict(cOk && dOk && cust.kept === '{"fog":"own"}' && cust.name === '(none)' && cust.fogOnly, 'tables moved by another road (both switches put off by hand under hard; a row moved under them): the level under the levers is read back off the tables, the fog lever writes the fog\'s rows alone, and rows moved by hand drop the kept AI and kit levers (the fog lever is its own)',
        `hard, both switches off by hand: ${c}\n    hard, a row moved by hand, fog lever → own eyes: ${d} · AI + fog levers kept, then a row moved by hand: kept ${cust.kept}, name ${cust.name}`);
      // a lesson's page keeps the kept levers off its tables; a lever moved there is moved AMONG them
      const lesson = await withStore({ 'twopointsix.levers': '{"fog":"own","kit":"stuns"}' }, m => { fresh(); D.initDifficulty(new URLSearchParams(''), true); const on = shown(); D.setLever('ai', 'hard'); const r = { on, after: shown(), kept: m.get('twopointsix.levers') }; D.initDifficulty(new URLSearchParams('')); fresh(); return r; });
      F.verdict(lesson.on === 'team/standard/none' && lesson.after === 'own/hard/stuns' && lesson.kept === '{"fog":"own","kit":"stuns","ai":"hard"}', 'on a Training lesson\'s page the kept levers are off the tables, and a lever moved there joins them: none is dropped from what the machine keeps', `the lesson: ${lesson.on} · AI moved there: ${lesson.after}, kept ${lesson.kept}`);
      // the kit a LEVEL carries on a mission's own structure, on a page that stands elsewhere; a press on the level's own rung there pins nothing
      fresh(); D.setDifficulty('hard', false); const at = `${D.leversShown('the block').kit} on The block, ${D.leversShown('').kit} elsewhere`;
      D.setLever('kit', 'none', false, 'the block'); const pinned = J(D.leversSet()); D.setLever('kit', 'stuns', false, 'the block'); const back = J(D.leversSet()); D.setLever('kit', 'none', false, ''); D.setLever('fog', 'cones', false, ''); D.setLever('ai', 'hard', false, ''); const own = J(D.leversSet());
      const fs = await import('node:fs'), blockFile = fs.readFileSync(new URL('../../levels/block.json', import.meta.url), 'utf8'), named = Object.keys(D.HARD_AT).every(k => k === D.structureOfLevel('block.json') && new RegExp(`"name":\\s*"${k}"`).test(blockFile));
      fresh();
      F.verdict(at === 'stuns on The block, none elsewhere' && pinned === '{"kit":"none"}' && back === '{}' && own === '{}' && named && D.structureOfLevel('depot.json') === '', 'the mission sheet shows the kit the level carries on the mission\'s OWN structure (hard: a charge each on The block); a lever put on another rung there is pinned, and one put on the level\'s own rung is let go of',
        `hard: ${at} · guns on The block: moved ${pinned} · stuns again: moved ${back} · every level file HARD_AT names is in the table: ${named}`); }
    // ---- a squad map is worded as one because its ROW says so, and every row says what its level file holds
    { const { ALL_MISSIONS } = await import(`${ROOT}/src/scene/scenarioList.ts`), fs = await import('node:fs');
      const rows = (ALL_MISSIONS as any[]).filter(e => e.kind === 'roomclear' && e.level);   // (a Training lesson too: its page reads no lever, but a kept Hard stands on it)
      const held = (file: string) => { try { return (JSON.parse(fs.readFileSync(new URL(`../../levels/${file}`, import.meta.url), 'utf8')).scenarios?.roomClear?.squads ?? []).length > 0; } catch { return null; } };
      const wrong = rows.filter(e => held(e.level) !== !!D.squadsOf(e)).map(e => `${e.id}: ${e.level} ${held(e.level) === null ? 'does not read' : held(e.level) ? 'names squads, the row does not (add squads: true to it)' : 'names no squads, the row says squads'}`);
      const pool = D.difficultyWords('hard', D.squadsOf({ level: 'new-map.json' })), flagged = D.difficultyWords('hard', D.squadsOf({ level: 'new-map.json', squads: true })), own = D.difficultyWords('hard', D.squadsOf((ALL_MISSIONS as any[]).find(e => e.id === 'compound-four')));
      const none = D.squadsOf(null) === false && D.squadsOf({}) === false && D.squadsOf({ squads: true }) === true, std = D.difficultyWords('standard', D.squadsOf({ level: 'new-map.json', squads: true }));
      F.verdict(wrong.length === 0 && rows.some(e => D.squadsOf(e)) && /come at you together/.test(pool) && /^hard: the squads are a drilled garrison/.test(flagged) && !/come at you together/.test(flagged) && /call it gone on the radio/.test(own) && none && std === D.difficultyWords('standard'),
        'every room clear on the list says on its own row whether squads hold its level, as the level file has it; a row that says so words Hard as the squads\' (a map with no sentence of its own: a drilled garrison), one that does not as the one pool',
        `${rows.length} rows, ${rows.filter(e => D.squadsOf(e)).length} held by squads${wrong.length ? `\n    ${wrong.join('\n    ')}` : ''}\n    a new map, flagged: ${flagged.slice(0, 60)}…`); }
  } finally { fresh(); FOG.own = ownWas; }
  return F.done();
}

// ───────────────────────────────────────────────────────────── screens ─────────────────────────────────────────────────────────────
// ───────────────────────────────────────────────────────────── run ─────────────────────────────────────────────────────────────
export async function leversRun(): Promise<boolean> {
  const F = family('levers-run: the fog lever in a Game built after it');
  const ownWas = FOG.own;
  try {
    fresh();
    { D.setLever('fog', 'own', false); const W = await quiet(() => standUp({ uploadEvery: 4 })); const a = fog(); void W;
      D.setLever('fog', 'cones', false); await quiet(() => standUp({ uploadEvery: 4 })); const b = fog();
      D.setLever('fog', null, false); await quiet(() => standUp({ uploadEvery: 4 })); const c = fog(); fresh();
      F.verdict(a === 'cone+own' && b === 'cone' && c === 'round', 'a Game built with the fog lever on your own eyes has the player\'s own cone in force (over the machine\'s kept feel rows, read in its constructor); on cones, the team\'s; let go, as the level', `own → ${a} · cones → ${b} · let go → ${c}`); }
  } finally { fresh(); FOG.own = ownWas; }
  return F.done();
}

/** A MISSION'S ADDRESS NAMES THE DIFFICULTY, AND THE HARNESS'S ADDRESSED STAND READS IT WHERE A PAGE DOES: before the game is made, and so before the mission is staged. No mission is
 *  staged here: what is held is the moment. `LEVERS_ADDRESS_BREAK=1`: the words are left off every address, as the stand was before it read them. Must fail. */
export async function leversAddress(): Promise<boolean> {
  const F = family('levers-address: the harness stood from an address holds the difficulty that address names, before the game is made');
  const { knobsCopy, knobsBack } = await import(`${ROOT}/src/game/knobs.ts`), knobs0 = knobsCopy(), ownWas = FOG.own, BREAK = process.env.LEVERS_ADDRESS_BREAK === '1';
  const stand = (place: string, words: string, opts: Record<string, unknown> = {}) => quiet(() => standUp({ seed: 1, address: `?${place}${BREAK ? '' : words}`, ...opts }));
  const read = () => J({ now: D.difficultyNow(), pressure: PRESSURE.on, digin: DIGIN.on, cone: FOG.cone });
  const SQUAD = 'levelFile=kit-yard.json&roomclear&squad4', LESSON = 'levelFile=tut-kit.json&roomclear&squad4';
  try {
    fresh();
    { await stand(SQUAD, '&hard'); const ok = D.difficultyNow() === 'hard' && PRESSURE.on && DIGIN.on && FOG.cone === D.HARD.cone;
      F.verdict(ok, '&hard: the tables stand on hard as the stand-up hands the game back, with no mission staged', read()); }
    fresh();
    { const was = tables(); await stand(SQUAD, ''); F.verdict(tables() === was && D.difficultyNow() === 'standard', 'no word on the address, standard before it: the tables are not touched', read()); }
    fresh(); D.setDifficulty('hard', false);
    { const was = tables(); await stand(SQUAD, ''); F.verdict(tables() === was && D.difficultyNow() === 'hard', 'no word on the address, a tool\'s own hard before it: the tables stand as the tool left them', read()); }
    for (const w of ['&hard=0', '&difficulty=standard']) { fresh(); D.setDifficulty('hard', false); await stand(SQUAD, w);
      F.verdict(D.difficultyNow() === 'standard' && !PRESSURE.on && !DIGIN.on, `${w}: an address that names standard puts it there, over a hard before it`, read()); }
    fresh();
    { await stand(SQUAD, '&enemyai=hard'); F.verdict(PRESSURE.on && DIGIN.on && !FOG.cone, '&enemyai=hard alone moves the defenders and leaves the fog', read()); }
    fresh();
    { await stand(SQUAD, '&hard&pressure=0'); F.verdict(!PRESSURE.on && DIGIN.on, '&hard&pressure=0: the older single switch stands over the level', read()); }
    fresh();
    { const was = tables(); await stand(LESSON, '&enemyai=hard'); F.verdict(tables() === was, 'a Training lesson\'s address reads no lever, as its page does not', read()); }
    fresh();
    { await stand(LESSON, '&hard'); F.verdict(D.difficultyNow() === 'hard', '…and it does read a level', read()); }
    fresh();
    { let on: boolean | null = null; await stand(SQUAD, '&hard', { tables: () => { PRESSURE.on = false; on = DIGIN.on; } });
      F.verdict(on === true && !PRESSURE.on, '`tables`, a recipe\'s, are laid over what the address wrote and before the game is made', read()); }
  } finally { fresh(); knobsBack(knobs0); FOG.own = ownWas; }
  return F.done();
}

if (import.meta.main) {
  const want = process.argv.slice(2).filter(a => !a.startsWith('--')), all: [string, () => Promise<boolean>][] = [['levers-rules', leversRules], ['levers-run', leversRun], ['levers-address', leversAddress]];
  let ok = true; for (const [n, fn] of all) if (!want.length || want.includes(n)) ok = (await fn()) && ok;
  process.exit(ok ? 0 : 1);
}
