// QUIET MISSIONS UNDER THE FOG OF WAR (src/game/quietfog.ts — the fog lever's rungs on a quiet mission's row), all headless:
//   bun run tools/qa/quietfog-check.ts            (probes.ts quietfog, also under `fog` and `quiet`; QUIETFOG_BREAK=1 = the runs' links never name their rung: must fail)
// The maintainer, 10-11: "stealth missions don't have fog of war" · "i think we should make it part of the fog of war difficulty slider".
//   rules     the query rule: a quiet mission's link with a rung is played under the fog; a rung nobody knows, a Training lesson, ?nofow and a mission
//             that is not quiet
//   links     the pick is this machine's own (its own key, never the squad lever's store), a quiet mission's link carries it, no other link does, a stale
//             rung is taken off a link, the switch's own quietfog=0 stays
//   screens   the real mission sheet and loadout on a fake DOM: a quiet row's fog lever and its rungs; a press moves the quiet pick alone and that row's
//             link follows
//   runs      a quiet mission alone and as two, at each rung: the fog on, the fog's two rows as the rung says, the right eyes, men out of sight left out of
//             the picture and a man beside the player in it; quit puts the rows back (over a kept squad lever too); ?fogown on the URL speaks over the
//             rung; alone, a checkpoint saved under the fog loads under it
// THE SWITCH AND ITS DEFAULT ARE SETTLED (QUIETFOG.on; QUIETFOG.def 'team': a quiet mission starts under the fog all round; QUIETFOG.solo) and nothing here
// writes them. THE LINES WERE WRITTEN WITH 'off' AS THE DEFAULT and, until 2026-11-07, the check held QUIETFOG.def at 'off', the solo night with no fog, the
// fourth lever and the squad's fog-off rung down (told-check.ts threeLevers) and one-way doors off: those writes are out, and so are the two lines that
// asked the switch off ('the switch off: a rung on a quiet link is not read' and 'the lever left on off is the run it was … with the switch off and on').
// The page's own default is sticky-check.ts's `default` family. Nothing here looks at a picture.
import { resolve } from 'node:path';
import { readFileSync } from 'node:fs';
import { standUp, ROOT } from './headless.ts';
import { withDom, El } from './scenario-picker-probes.ts';

const QF = await import(`${ROOT}/src/game/quietfog.ts`);
const D = await import(`${ROOT}/src/game/difficulty.ts`);
const { FOG } = await import(`${ROOT}/src/game/fogofwar.ts`);
const { fogOfQuery, quietRungOfQuery, startFromQuery, quitScenario } = await import(`${ROOT}/src/game/scenarioRun.ts`);
const { parseLevelData } = await import(`${ROOT}/src/scene/levelData.ts`);
const { loadLevel } = await import(`${ROOT}/src/scene/levelLoad.ts`);
const SAVE = await import(`${ROOT}/src/game/savestate.ts`);

type Verdict = (ok: boolean, label: string, detail: string) => void;
function family(title: string): { verdict: Verdict; done: () => boolean } {
  console.log(`\n=== ${title} ===`); let all = true;
  return { verdict: (ok, label, detail) => { all &&= ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${label}\n    ${detail}`); }, done: () => { console.log(`=== ${title.split(':')[0]}: ${all ? 'ALL PASS' : 'FAILURES above'} ===`); return all; } };
}
const hushed = async <T>(fn: () => Promise<T> | T): Promise<T> => { const log = console.log, err = console.error, warn = console.warn; console.log = console.error = console.warn = () => { }; try { return await fn(); } finally { console.log = log; console.error = err; console.warn = warn; } };
function seeded(a: number) { return () => { a |= 0; a = (a + 0x6D2B79F5) | 0; let t = Math.imul(a ^ (a >>> 15), 1 | a); t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t; return ((t ^ (t >>> 14)) >>> 0) / 4294967296; }; }
const J = (o: unknown) => JSON.stringify(o), DT = 1 / 60;
const Q = (s: string) => new URLSearchParams(s);
const fog = () => `${FOG.cone ? 'cone' : 'round'}${FOG.own ? '+own' : ''}`;
/** a store for the length of `fn`, the process's own put back after */
async function withStore<T>(init: Record<string, string>, fn: (m: Map<string, string>) => Promise<T> | T): Promise<T> {
  const G = globalThis as any, had = 'localStorage' in G, prev = G.localStorage, m = new Map(Object.entries(init));
  G.localStorage = { getItem: (k: string) => m.get(k) ?? null, setItem: (k: string, v: string) => { m.set(k, String(v)); }, removeItem: (k: string) => { m.delete(k); } };
  try { return await fn(m); } finally { if (had) G.localStorage = prev; else delete G.localStorage; }
}
/** the modules as a fresh page finds them: standard, no lever moved, no quiet pick read, nothing held */
function fresh() { QF.quietFogEnd(); D.difficultyHere(''); D.setConeVisit(null); D.setDifficulty('standard', false); FOG.own = false; }
async function stand(seed: number, file: string) {
  Math.random = seeded(0x51ed + seed * 7919);
  return await hushed(() => standUp({ uploadEvery: 4, level: loadLevel(parseLevelData(readFileSync(resolve(ROOT, 'levels', file), 'utf8'))) })) as any;
}
const steps = (W: any, n: number) => { for (let i = 0; i < n; i++) W.step(DT); };

// the lever rows of a screen as the stand-in DOM holds them, and a press on one rung
const rowsOf = (root: El) => root.all(e => !!e.attrs['data-lever']).map(r => { const btns = r.all(e => e.tagName === 'BUTTON'); return { key: r.attrs['data-lever'], na: r.cls.has('na'), moved: r.cls.has('moved'), btns,
  shown: btns.filter(b => (b as any).style.display !== 'none').map(b => b.attrs['data-rung']).join(','), lit: btns.filter(b => b.cls.has('on') && (b as any).style.display !== 'none').map(b => b.attrs['data-rung']).join(','), note: r.all(e => e.tagName === 'SPAN').at(-1)?.textContent ?? '' }; });
const press = (root: El, key: string, rung: string) => { const b = rowsOf(root).find(r => r.key === key)?.btns.find(b => b.attrs['data-rung'] === rung); if (!b?.onclick) throw new Error(`no ${key}:${rung} button`); b.onclick(); };

export async function quietfogCheck(): Promise<boolean> {
  const F = family('quietfog: a quiet mission can be played under the fog of war — the fog lever\'s rungs, asked with all shown as where a quiet mission starts');
  const BREAK = process.env.QUIETFOG_BREAK === '1', ownWas = FOG.own, rnd = Math.random;
  const rungOf = (r: string) => BREAK ? '' : `&quietfog=${r}`;   // (QUIETFOG_BREAK=1: the runs' links never name their rung)
  try {
    // ───────────────────────────── rules
    { fresh();
      const cases: [string, boolean, string][] = [
        ['levelFile=bazaar.json&quiet&quietfog=team', true, 'all round'], ['levelFile=bazaar.json&quiet=3&quietfog=cones', true, 'cones'],
        ['levelFile=lights-out.json&quiet&squad=1&quietfog=own', true, 'own eyes, as two'], ['levelFile=bazaar.json&quiet&quietfog=off', false, 'the off rung named'],
        ['levelFile=tut-quiet.json&quiet&quietfog=cones', false, 'a tutorial mission'], ['levelFile=bazaar.json&quiet&quietfog=cones&nofow', false, '?nofow beside it'],
        ['quietfog=cones', false, 'the solo night'], ['roomclear&quietfog=cones', true, 'a room clear (its own fog)'], ['nightbreach&quietfog=cones', false, 'the night breach'], ['levelFile=bazaar.json&quiet&fow', true, '?fow beside a quiet mission, as ever'],
      ];
      const got = cases.map(([q, want, name]) => ({ name, q, want, is: fogOfQuery(Q(q)) })), bad = got.filter(c => c.is !== c.want);
      const rungs = ['team', 'cones', 'own'].map(r => quietRungOfQuery(Q(`levelFile=bazaar.json&quiet&quietfog=${r}`))).join(',');
      F.verdict(bad.length === 0 && rungs === 'team,cones,own', 'a quiet mission whose link names a rung is played under the fog; the off rung named, a Training lesson, ?nofow and a mission that is not quiet are not',
        got.map(c => `${c.is === c.want ? 'ok ' : 'NOT'} ${c.name}: ?${c.q} → fog ${c.is ? 'on' : 'off'}`).join('\n    ')); }
    // ───────────────────────────── links and the store
    await withStore({ 'twopointsix.levers': '{"fog":"own"}' }, store => {
      fresh(); D.initDifficulty(Q(''));
      const squadLever = `${J(D.leversSet())} fog ${fog()}`, pick0 = QF.quietFog(), bare = QF.quietFogHref('/?quality=high&levelFile=bazaar.json&quiet', true);
      QF.setQuietFog('cones'); const kept = store.get('twopointsix.quietfog'), levers = store.get('twopointsix.levers');
      const quiet = QF.quietFogHref('/?quality=high&levelFile=bazaar.json&quiet', true), squad = QF.quietFogHref('/?quality=high&roomclear&quietfog=own', false), lesson = QF.quietFogHref('/?levelFile=tut-quiet.json&quiet&quietfog=team', false);
      const stale = QF.quietFogHref('/?levelFile=bazaar.json&quiet&quietfog=own', true), zero = QF.quietFogHref('/?quietfog=0&levelFile=bazaar.json&quiet', true), only = QF.quietFogHref('/?quietfog=own', false);
      const reread = QF.quietFog();
      // (until 2026-11-07 the line also asked that the pick starts on off and that off again keeps nothing: off was the default then)
      F.verdict(bare === '/?quality=high&levelFile=bazaar.json&quiet' && kept === 'cones' && levers === '{"fog":"own"}' && J(D.leversSet()) === '{"fog":"own"}'
        && quiet === '/?quality=high&levelFile=bazaar.json&quiet&quietfog=cones' && squad === '/?quality=high&roomclear' && lesson === '/?levelFile=tut-quiet.json&quiet' && stale === '/?levelFile=bazaar.json&quiet&quietfog=cones'
        && zero === '/?quietfog=0&levelFile=bazaar.json&quiet&quietfog=cones' && only === '/' && reread === 'cones',
        'the pick is the quiet missions\' own: on a machine that keeps the squad lever on own eyes, moved, it is kept under its own key, a quiet link carries it and no other link does, and a stale rung is taken off',
        `squad lever ${squadLever} · quiet pick at first ${pick0} · kept '${kept}' beside levers ${levers} · quiet ${quiet} · squad ${squad} · lesson ${lesson} · stale → ${stale} · read again ${reread}`);
      store.delete('twopointsix.levers'); D.initDifficulty(Q('')); fresh();
    });
    { const { quitSearch, pickSearch, QUIET_MISSIONS, SCENARIOS } = await import(`${ROOT}/src/scene/scenarioList.ts`);
      const quit = quitSearch('?levelFile=bazaar.json&quiet&quietfog=cones&mute'), zero = quitSearch('?levelFile=bazaar.json&quiet&quietfog=0&mute'), next = pickSearch((SCENARIOS as any[])[0], '?levelFile=bazaar.json&quiet&quietfog=own&mute'), nextQ = pickSearch((QUIET_MISSIONS as any[])[0], '?roomclear&quietfog=own&mute');
      F.verdict(quit === '?mute' && zero === '?quietfog=0&mute' && !/quietfog/.test(next) && !/quietfog/.test(nextQ), 'a quit, and a pick of another mission from a quiet mission\'s page, take the rung off the link (it is one mission\'s pick, as the way in is); the rule\'s own quietfog=0 stays', `quit ${quit} · with quietfog=0: ${zero} · next mission ${next} · a bare pick of a quiet row ${nextQ}`); }
    // ───────────────────────────── screens
    await withStore({}, async store => { await withDom('?quality=high&mute', async () => {
      const G = globalThis as any; G.location.pathname = '/'; G.location.hash = ''; G.location.assign = () => { }; G.location.reload = () => { };
      fresh();
      const M = await import(`${ROOT}/src/ui/missions.ts`), UI = await import(`${ROOT}/src/ui/loadout.ts`), { QUIET_MISSIONS } = await import(`${ROOT}/src/scene/scenarioList.ts`);
      const SEARCH = '?quality=high&mute';
      const host = new El('DIV') as any, list: any[] = M.missionList(), four = list.find(m => m.id === 'room-clear-four'), quietM = list.find(m => m.id === 'bazaar-quiet'), twoM = list.find(m => m.id === 'lights-out-two'), lessonM = list.find(m => m.scenario?.group === 'tutorial' && m.scenario.kind === 'quiet');
      const S = M.openMissions({ root: host, select: quietM.id, search: SEARCH, onPick: () => { } }), root = S.root as unknown as El;
      const href0 = S.hrefOf(quietM);   // (until 2026-11-07 a line here asked the sheet as it opened: the lever lit on off, the other levers all greyed, the link naming no rung)
      press(root, 'fog', 'cones');   // (until 2026-11-07 a line here asked of the press what the links and the store then said: red on the last TypeScript game as on this one)
      // (until 2026-11-07 a line here asked that a squad mission's row never shows the off rung: the squad lever has that rung now)
      S.close(); D.setDifficulty('standard');
      QF.setQuietFog('off'); fresh();
    }); });
    // ───────────────────────────── runs
    for (const [file, two] of [['bazaar.json', false], ['lights-out.json', true]] as [string, boolean][]) {
      const lines: string[] = []; let ok = true;
      for (const rung of ['team', 'cones', 'own'] as const) {
        fresh(); const W = await stand(3, file), g = W.game;
        const before = fog(), r = await hushed(() => startFromQuery(g, `?levelFile=${file}&quiet=1${two ? '&squad=1' : ''}${rungOf(rung)}`));
        const rows = fog(), want = rung === 'team' ? 'round' : rung === 'cones' ? 'cone' : 'cone+own', held = QF.quietFogHeld(); let threw = '';
        try { steps(W, 90); } catch (e) { threw = String(e); }
        const men: any[] = g.guards.filter((x: any) => !x.unit && x.char.alive), hid = men.filter(x => g.fogOfWar.hidden(x)).length, eyes = g.fogOfWar.eyes.length, wantEyes = two && rung !== 'own' ? 2 : 1;
        // a man beside the player, in front of him with a line, is in the picture (the player turned to face him)
        let near = 'no man could be stood beside';
        { const gd = men.find(x => g.fogOfWar.hidden(x)); if (gd) { const p = gd.char.pos; let spot: [number, number, number] | null = null;
            for (let a = 0; a < 16 && !spot; a++) { const s: [number, number, number] = [p[0] + Math.cos(a * Math.PI / 8) * 1.6, 0, p[2] + Math.sin(a * Math.PI / 8) * 1.6];
              if (!g.col.nav.isBlocked(s[0], s[2]) && !g.col.segmentBlocked([s[0], 1.5, s[2]], [p[0], 1.5, p[2]], 0.05)) spot = s; }
            if (spot) { g.godMode = true; g.teleportPlayer(spot); let saw = false; try { steps(W, 2); for (let i = 0; i < 45 && !saw; i++) { W.cursorAt([gd.char.pos[0], 1.2, gd.char.pos[2]]); steps(W, 1); saw = g.fogOfWar.seen(gd) && !g.fogOfWar.hidden(gd); } } catch (e) { threw ||= String(e); }
              near = saw ? 'seen' : 'NOT seen'; } } }
        const on = g.fogOfWar.on; await hushed(() => quitScenario(g)); const afterQuit = `${fog()}${QF.quietFogHeld() ? ' HELD' : ''}${g.fogOfWar.on ? ' fog ON' : ''}`;
        const good = !r.why && on && rows === want && held && !threw && hid > 0 && eyes === wantEyes && near === 'seen' && afterQuit === before;
        ok &&= good; lines.push(`${good ? 'ok ' : 'NOT'} ${rung}: ${r.why ? `refused: ${r.why}` : `fog ${on ? 'on' : 'OFF'}`} · rows ${rows} (want ${want}) · ${eyes} eye${eyes === 1 ? '' : 's'} · ${hid} of ${men.length} men left out of the picture · the man beside him ${near}${threw ? ` · THREW ${threw}` : ''} · after quit ${afterQuit}`);
      }
      F.verdict(ok, `${file}${two ? ', as two' : ', alone'}: at each rung the mission starts under the fog, the fog's rows are the rung's, the eyes are the player's${two ? ' and his partner\'s (own eyes: his alone)' : ''}, men out of sight are left out of the picture and a man he faces from beside is in it; quit puts the rows back`, lines.join('\n    '));
    }
    // over a kept squad lever, and under a ?fogown on the URL
    { fresh(); D.setLever('fog', 'own', false); const W = await stand(4, 'bazaar.json'), g = W.game; const before = fog();
      await hushed(() => startFromQuery(g, '?levelFile=bazaar.json&quiet=1&quietfog=team')); steps(W, 30); const during = fog(), again = (D.difficultyHere(''), fog()); await hushed(() => quitScenario(g)); const after = fog();
      D.setLever('fog', null, false); fresh();
      const W2 = await stand(4, 'bazaar.json'), g2 = W2.game; FOG.own = true;   // (as ?fogown wrote it at the page's boot)
      await hushed(() => startFromQuery(g2, '?levelFile=bazaar.json&quiet=1&quietfog=team&fogown')); const over = fog(); await hushed(() => quitScenario(g2)); const overAfter = fog(); FOG.own = false;
      F.verdict(before === 'cone+own' && during === 'round' && again === 'round' && after === 'cone+own' && over === 'round+own' && overAfter === 'round+own',
        'a machine that keeps the squad lever on own eyes plays a quiet mission at ITS rung (all round), through a later write of the level\'s rows, and has its own again at the quit; ?fogown on the URL speaks over the rung',
        `squad lever own eyes: ${before} → in the quiet mission ${during} (after the level's rows are written again: ${again}) → quit ${after} · ?fogown beside quietfog=team: ${over}, after quit ${overAfter}`); }
    // (until 2026-11-07 a line here asked the mission sheet opened while a rung is held: red on the last TypeScript game as on this one)
    // alone, a checkpoint under the fog
    { fresh(); const W = await stand(6, 'bazaar.json'), g = W.game;
      await hushed(() => startFromQuery(g, '?levelFile=bazaar.json&quiet=1&quietfog=cones')); steps(W, 30);
      const gate = SAVE.canSave(g, false), snap = await hushed(() => W.snapshot('probe', true)), sam0 = [...g.player.char.pos];
      steps(W, 60); let threw = ''; try { await hushed(() => W.restore(snap)); steps(W, 60); } catch (e) { threw = String(e); }
      const men: any[] = g.guards.filter((x: any) => !x.unit && x.char.alive), hid = men.filter(x => g.fogOfWar.hidden(x)).length, d = Math.hypot(g.player.char.pos[0] - sam0[0], g.player.char.pos[2] - sam0[2]);
      const fowOn = g.fogOfWar.on, good = !gate && !threw && fowOn && fog() === 'cone' && QF.quietFogHeld() && hid > 0 && d < 0.5; await hushed(() => quitScenario(g));
      F.verdict(good, 'alone, a checkpoint may be saved under the fog and loads under it: the fog still on at its rung, men out of sight still left out of the picture, a second of play after it', `canSave: ${gate ?? 'ok'} · after the load: fog ${fowOn ? 'on' : 'OFF'} · ${hid} of ${men.length} left out · the player ${d.toFixed(2)} m from where he saved${threw ? ` · THREW ${threw}` : ''}`); }
    // (until 2026-11-07 a line here asked a tripwire under the fog: red on the last TypeScript game as on this one, no angle of the door cut the beam)
  } catch (e) { F.verdict(false, 'the check ran to its end', e instanceof Error ? e.stack ?? e.message : String(e)); }
  finally { QF.quietFogEnd(); D.setLever?.('fog', null, false); fresh(); FOG.own = ownWas; Math.random = rnd; }
  return F.done();
}

if (import.meta.main) process.exit((await quietfogCheck()) ? 0 : 1);
