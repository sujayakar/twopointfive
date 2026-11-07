// A MISSION'S OWN CARD FOR YOU IS THE WHOLE KIT (src/game/kit.ts ISSUE.whole).
//   bun run tools/qa/probes.ts issuewhole
//   ISSUEWHOLE_BREAK=1 …: the rule's broken path in kit.ts (the build before: the mission's hand-overs land over the card) — MUST FAIL.
// What it holds: a mission of the quiet kind whose level names your card stages that card as everything he carries — no sponge round, no camera, no
// wand, the launcher on the first load he does carry; a card that names sponge rounds (`rings`) carries that many; a block that names no card for you
// (the long hall's, empty), a mission with no block and a lesson's page with none are not this rule's. The tutorial's solo mission is stood up as its
// page stands it.
// SINCE 2026-11-07 the rule is settled on and the check writes neither it nor the switches it once raised or held off beside it (the card as a tree, the
// two gun rows, the cameras, the wand, the deal, the stealth card): the stand's line, every "off: the build before" half, "the same with the rule on and
// off" (with the tree off, with the stealth card off, on a lesson's page) and the recording from before the key are out. The lines that stood the
// stealth card OFF to read a hand-over now read what the page stages there.
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';
import { ROOT, standUp } from './headless.ts';
import { rig } from './doortools-probes.ts';

const K = await import(`${ROOT}/src/game/kit.ts`);
const { KITTREE } = await import(`${ROOT}/src/game/kittree.ts`);
const { stickyVisitSet } = await import(`${ROOT}/src/game/stickycam.ts`);
const { parseLevelData, validateLevelData } = await import(`${ROOT}/src/scene/levelData.ts`), { loadLevel } = await import(`${ROOT}/src/scene/levelLoad.ts`);
const { startFromQuery } = await import(`${ROOT}/src/game/scenarioRun.ts`);

const J = (x: unknown) => JSON.stringify(x);
const BROKEN = process.env.ISSUEWHOLE_BREAK === '1', FILE = 'tut-quiet.json';
const hushed = async <T>(fn: () => Promise<T> | T): Promise<T> => { const log = console.log, err = console.error, warn = console.warn; console.log = console.error = console.warn = () => { }; try { return await fn(); } finally { console.log = log; console.error = err; console.warn = warn; } };
const TEXT = readFileSync(resolve(ROOT, 'levels', FILE), 'utf8'), text = () => TEXT;
const held = (g: any) => { const pl = g.player, rf = pl.rifle; return { rings: rf.rings, cams: rf.attachments.includes('cam') ? rf.cams : 0, wand: !!pl.wand, smoke: pl.canisters, stun: pl.flashbangs, frags: pl.frags, shells: rf.shells, load: rf.attachment as string }; };

export async function issueWhole(): Promise<boolean> {
  console.log(`\n=== issuewhole: a mission's own card for you is the whole kit${BROKEN ? ' [ISSUEWHOLE_BREAK=1]' : ''} ===`);
  let all = true; const verdict = (ok: boolean, label: string, detail: string) => { all &&= ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${label}\n    ${detail}`); };
  await hushed(() => rig(17, { stage: false }));   // (a game stood up first, as every line below was written)
  const was = { big: K.BIGSLOT.on, on: K.KIT.on, slots: K.SLOTS.on, solo: K.KIT_SOLO.on };
  const raise = () => { K.kitMission(null); K.BIGSLOT.on = true; K.KIT.on = true; K.SLOTS.on = true; };
  try {
    // ── the file: the one level that names your card today
    { const block = JSON.parse(text()).scenarios?.issue?.members?.['0'] ?? null;
      verdict(!!block && (block.rings === undefined || (Number.isInteger(block.rings) && block.rings >= 0 && block.rings <= K.RINGS_MAX)) && (validateLevelData(JSON.parse(text())) as string[]).length === 0 && K.kitIssueWhole({ '0': block }) === !BROKEN && !K.kitIssueWhole({}) && !K.kitIssueWhole(null),
        'THE FILE: the solo tutorial\'s block names your card (it named its own sponge rounds, `rings`, while its lines taught the swap: none now); a block that names no card for you is not this rule\'s', `card ${J(block)}`); }

    { const errs = (c0: unknown, c1?: unknown) => { const d = JSON.parse(text()); d.scenarios.issue = { members: { '0': c0, ...(c1 ? { '1': c1 } : {}) } }; return (validateLevelData(d) as string[]).filter(e => /issue/.test(e)); };
      const takes = [0, 2, K.RINGS_MAX].every(n => errs({ weapon: 'rifle', rings: n }).length === 0) && errs({ rings: 1 }).length === 0;
      const refuses = [errs({ rings: K.RINGS_MAX + 1 }), errs({ rings: 1.5 }), errs({ rings: -1 }), errs({ weapon: 'pistol', rings: 1 }), errs({ weapon: 'shotgun', rings: 1 }), errs({}, { rings: 1 })].map(e => e.length);
      verdict(takes && refuses.every(n => n > 0) && K.RINGS_MAX === 5, 'THE VALIDATOR takes `rings` on your own card up to the launcher\'s five, and refuses more, a count that is no whole number, a card with no rifle, and a teammate\'s', `taken ${takes} · refused ${J(refuses)} · the card\'s most ${K.RINGS_MAX}`); }

    // ── the tutorial's solo mission as its page stands it
    const page = async () => { let r: any = null, why = '';
      try { K.kitIssuedPage(true); const W: any = await hushed(() => standUp({ level: loadLevel(parseLevelData(text())), seed: 1 })), g = W.game; raise(); K.kitLoad(null, '');
        const s = await hushed(() => startFromQuery(g, `?levelFile=${FILE}&quiet`)); why = s?.why ?? ''; K.kitSoloIf(g, false); for (let i = 0; i < 5; i++) W.step(1 / 60); r = held(g);
      } catch (x) { why = String((x as Error).message); } finally { K.kitIssuedPage(false); K.KIT_SOLO.on = false; }
      return { r, why }; };
    { const on = await page(), b = on.r, own: number = JSON.parse(text()).scenarios?.issue?.members?.['0']?.rings ?? 0;   // (the card's own sponge rounds: two while the mission taught the swap to the grenade, none since it carries the grenade alone)
      verdict(!on.why && !!b && b.rings === own && b.cams === 0 && !b.wand && b.load === (own ? 'ring' : 'he40') && `${b.smoke}/${b.stun}/${b.frags}/${b.shells}` === '2/2/2/3',
        'THE SOLO TUTORIAL ON ITS PAGE: he carries the block and nothing else: the sponge rounds it names and no more, the right button on them where it names any and on the grenades where it names none', `${J(b)}${on.why ? ` (${on.why})` : ''}`); }

    // ── the rule on a rig: a block that names them, a mission with no block, a lesson's page with none
    { K.kitLoad(null, ''); const R: any = await hushed(() => rig(17, { stage: false })); raise(); K.kitLoad(null, ''); const g = R.game;
      const others = g.guards.filter((gd: any) => !(gd.unit && gd.team === 'operative')), all0 = g.guards, sc = g.level.scenarios ?? (g.level.scenarios = {}), had = sc.issue;
      const run = (block: unknown, cams: number, lesson = false) => { K.kitShow(g); K.KIT_SOLO.on = true; g.guards = others; stickyVisitSet(g, cams); if (block) sc.issue = { members: { '0': block } }; else delete sc.issue;
        const k = g.player.rifle.attachments.indexOf('ring'); if (k >= 0) g.player.rifle.attIdx = k; K.kitIssuedPage(lesson); try { K.kitPlayer(g); return held(g); } finally { K.kitIssuedPage(false); } };
      const BLOCK = { weapon: 'rifle', smoke: 2, bangs: 2, frags: 2, shells: 3 };
      try {
        const table = J(K.KIT.members[0]);
        const on = run(BLOCK, 2), same = J(K.KIT.members[0]) === table;
        verdict(on.rings === 0 && on.cams === 0 && !on.wand && on.load === 'he40' && on.frags === 2 && on.shells === 3 && same,
          'A QUIET RUN ALONE UNDER A FOG, with a block: none of the mission\'s five sponge rounds, two cameras and wand land over it, the launcher is on the grenade, and the table\'s own card is not touched', `${J(on)} · the table ${same ? 'as it was' : 'CHANGED'}`);
        { const named = run({ ...BLOCK, rings: 2 }, 2), top = run({ ...BLOCK, rings: K.RINGS_MAX }, 2); K.kitShow(g); K.KIT_SOLO.on = true; g.guards = others; stickyVisitSet(g, 2); const FITS = { weapon: 'rifle', smoke: 0, bangs: 1, frags: 1, shells: 1, rings: 2 }; sc.issue = { members: { '0': FITS } }; { const k = g.player.rifle.attachments.indexOf('ring'); if (k >= 0) g.player.rifle.attIdx = k; } K.kitStage(g, []); const staged = held(g);
          verdict(named.rings === 2 && named.load === 'ring' && named.cams === 0 && !named.wand && named.shells === 3 && top.rings === K.RINGS_MAX && staged.rings === 2 && staged.load === 'ring' && staged.shells === 1,
            'A CARD THAT NAMES SPONGE ROUNDS (`rings`) carries that many and nothing else of the mission\'s: the launcher comes up on them with the grenades its next load; a card that fits its pouches keeps them through the squad\'s staging too (one that does not is trimmed there like any card)', `rings 2: ${J(named)} · rings ${K.RINGS_MAX}: ${top.rings} · a fitting card through kitStage: ${J(staged)}`); }
        g.guards = all0; const ops0 = { members: {} }; const hall = () => { K.kitShow(g); K.KIT_SOLO.on = true; g.guards = others; stickyVisitSet(g, 2); sc.issue = ops0; { const k = g.player.rifle.attachments.indexOf('ring'); if (k >= 0) g.player.rifle.attIdx = k; } K.kitPlayer(g); return held(g); };
        const hallOn = hall();
        verdict(hallOn.rings === KITTREE.lend && hallOn.cams === 2 && hallOn.wand, 'A BLOCK THAT NAMES NO CARD FOR YOU (the long hall\'s, empty: written for its room clear) leaves a night alone on that level its five sponge rounds, two cameras and the wand', J(hallOn));
        const plain = run(null, 2);
        verdict(plain.rings === 5 && plain.cams === 2 && plain.wand,
          'A MISSION WITH NO BLOCK is not this rule\'s: the stealth card as issued on a quiet run', `stealth card ${J(plain)}`);
        const lesOn = run(null, 2, true);
        verdict(lesOn.wand, 'A LESSON\'S PAGE WITH NO BLOCK: the mission\'s wand is handed over the plain issue', J(lesOn));
      } finally { K.kitShow(g); if (had) sc.issue = had; else delete sc.issue; g.guards = all0; stickyVisitSet(g, 0); K.KIT_SOLO.on = false; }
    }
  } finally { K.kitMission(null); K.BIGSLOT.on = was.big; K.KIT.on = was.on; K.SLOTS.on = was.slots; K.KIT_SOLO.on = was.solo; K.kitIssuedPage(false); K.kitLoad(null, ''); K.SLOTS.on = was.slots; K.KIT.on = was.on; }
  console.log(`=== issuewhole: ${all ? 'ALL PASS' : 'FAILURES above'} ===`);
  return all;
}
if (import.meta.main) issueWhole().then(ok => process.exit(ok ? 0 : 1));
