// A NAMED COAT'S FINISH GOES WHEREVER ITS WORD GOES (src/scene/stuff.ts COATFINISH, coatRough; read by src/scene/boxes.ts matWord) — a rule of the picture only.
// A box that says a named coat (a vehicle's body: 'carpaint') is handed to the GPU no rougher than that coat, whatever row painted it and whatever number a
// kit, a skin or a hand wrote beside the word; with the switch down it is handed the finish its file gives it. No box is changed: Box.rough is the file's,
// always. Roughness only: no box's `metal` moves.
//   bun run tools/qa/coatfinish-check.ts        (probes.ts coatfinish; COATFINISH_BREAK=1 must FAIL — the rule is left down where the check raises it)
// Both stands: the check gives the stand its word (TPS_COATFINISH) itself. Headless: nothing here is a picture. What is asserted is the one float a box's
// finish is sent as (matWord; tools/qa/materials-probes.ts and lookmaps-check.ts hold that the world sends that float and the shader reads it back). What a
// clear coat shows of what stands round it is the reflection pass's, and nobody has seen that on a GPU through this check.
import { readFileSync, readdirSync } from 'node:fs';
import { standUp, ROOT } from './headless.ts';
const { parseLevelData } = await import(`${ROOT}/src/scene/levelData.ts`);
const { loadLevel } = await import(`${ROOT}/src/scene/levelLoad.ts`);
const { COATFINISH, NAMED_COATS, namedCoat } = await import(`${ROOT}/src/scene/stuff.ts`);
const { makeBox, matWord, wordOf, ROUGH_DEFAULT } = await import(`${ROOT}/src/scene/boxes.ts`);
const { digestGame, hashDigest } = await import(`${ROOT}/src/game/digest.ts`);
const { fireWeapon } = await import(`${ROOT}/src/game/combat.ts`);

const DT = 1 / 60, BREAK = process.env.COATFINISH_BREAK === '1', FILE = 'gwot-yard.json', TWIN = 'depot-gwot.json';
function seeded(seed: number) { let a = seed >>> 0; return () => { a |= 0; a = a + 0x6D2B79F5 | 0; let t = Math.imul(a ^ a >>> 15, 1 | a); t = t + Math.imul(t ^ t >>> 7, 61 | t) ^ t; return ((t ^ t >>> 14) >>> 0) / 4294967296; }; }
async function hushed<T>(f: () => T | Promise<T>): Promise<T> { const log = console.log, warn = console.warn; console.log = console.warn = () => { }; try { return await f(); } finally { console.log = log; console.warn = warn; } }
const levelOf = (file: string) => loadLevel(parseLevelData(readFileSync(`${ROOT}/levels/${file}`, 'utf8')));
let stood: boolean | null = null;   // (the switch as a stand with no word of the check's left it, read once)
async function stand(on: boolean, file = FILE): Promise<any> {
  Math.random = seeded(41);
  const word = process.env.TPS_COATFINISH;
  try {
    if (stood === null) { await hushed(() => standUp({ level: levelOf(FILE), seed: 1 })); stood = !!COATFINISH.on; }
    process.env.TPS_COATFINISH = on && !BREAK ? '1' : '0';
    return await hushed(() => standUp({ level: levelOf(file), seed: 1 }));
  } finally { if (word === undefined) delete process.env.TPS_COATFINISH; else process.env.TPS_COATFINISH = word; }
}
const up = (v: boolean) => { COATFINISH.on = v && !BREAK; };
/** the roughness a box is sent with (matWord's float for its roughness and its word alone: the flags ride above it), and the one its own number alone would send */
const sent = (b: any): number => matWord({ rough: b.rough, stuff: b.stuff } as any), own = (b: any): number => Math.min(1, Math.max(0, b.rough ?? ROUGH_DEFAULT));
/** a list of boxes as the rule sends them now: how many say a named coat, how many of those go out at the coat's (or smoother, where their own is), how many
 *  boxes that say none go out other than their own */
function tally(boxes: any[]) {
  let said = 0, coated = 0, asOwn = 0, others = 0, pieces = 0; const owns = new Set<string>();
  for (const b of boxes) { const c = namedCoat(b.stuff), s = sent(b), o = own(b);
    if (!c) { if (s !== o) others++; continue; }
    said++; if (b.piece === true) pieces++; owns.add(String(b.rough ?? 'none')); if (s === Math.min(c.rough, o)) coated++; if (s === o) asOwn++; }
  return { said, coated, asOwn, others, pieces, owns: [...owns].sort().join(', ') };
}

export async function coatfinishCheck(): Promise<boolean> {
  let all = true;
  const line = (ok: boolean, s: string, detail = '') => { all &&= ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${s}${detail ? `\n    ${detail}` : ''}`); };
  console.log(`\n=== coatfinish: a named coat's finish goes wherever its word goes${BREAK ? ' (COATFINISH_BREAK: the rule left down)' : ''} ===`);
  const was = { ...COATFINISH }, rnd = Math.random, cr = (NAMED_COATS as any).carpaint.rough as number;
  try {
    const W = await stand(true), asStood = !!COATFINISH.on;
    { const shipped = (process.env.TPS_PROFILE ?? '') === 'shipped', said = process.env.TPS_COATFINISH;
      line(stood === (said !== undefined ? said === '1' : shipped) && asStood === !BREAK, '0 the stand: the rule is down on the frozen stand and up on the shipped one, or as its word says where the run gives one; and a stand given the word has it up',
        `stood up ${stood} · ${shipped ? 'shipped' : 'frozen'}${said !== undefined ? ` with TPS_COATFINISH=${said}` : ''} · given TPS_COATFINISH=1: ${asStood}`); }

    // (1) the yard as stood (its shapes' pieces with it where the stand has shapes): every box that says car paint goes out at the coat's roughness with the
    //     rule up and at its own with the rule down; a box that says no coat goes out at its own either way; and no box's own number or metal is touched
    { const st = W.level.boxes as any[], filed = levelOf(FILE).boxes as any[], n = Math.min(st.length, filed.length);
      up(true); const U = tally(st); up(false); const D = tally(st); up(true);
      let touched = 0; for (let i = 0; i < n; i++) if (st[i].piece !== true && (st[i].rough !== filed[i].rough || !!st[i].metal !== !!filed[i].metal)) touched++;
      line(U.said > 0 && U.coated === U.said && U.asOwn === 0 && U.others === 0 && D.said === U.said && D.asOwn === D.said && D.coated === 0 && D.others === 0 && touched === 0,
        `1 the yard stood up: with the rule up every box that says car paint, and each piece of its shape, is sent at the coat's roughness (${cr}); with it down each is sent at its own; a box that says no coat is sent at its own either way; no box's own number or metal differs from its file's`,
        `up: ${U.coated} of ${U.said} at the coat's (${U.pieces} of them pieces; their own numbers ${U.owns}), ${U.others} others moved · down: ${D.asOwn} of ${D.said} at their own, ${D.others} others moved · boxes whose own finish differs from the file's: ${touched}`); }

    // (2) the rule's cases by hand: a look's twin whose skin wrote a number beside the word; a body smoother than the coat (wet with rain) stays smoother; a
    //     row that is bare metal stays metal under the coat's roughness; a piece broken off a car, which copies the word and no finish; a word that is no coat
    { const twin = (levelOf(TWIN).boxes as any[]).filter((b: any) => namedCoat(b.stuff)), tOwn = [...new Set(twin.map((b: any) => String(b.rough ?? 'none')))].join(', ');
      const bx = (o: any) => makeBox({ c: [0, 0, 0], h: [1, 1, 1], ...o }) as any, body = bx({ stuff: 'carpaint', rough: 0.35 }), wet = bx({ stuff: 'carpaint', rough: 0.08 }), bare = bx({ stuff: 'carpaint', rough: 0.38, metal: true }), piece = bx({ ...wordOf(body) }), plain = bx({ stuff: 'paint', rough: 0.35 }), odd = bx({ stuff: 'constructor', rough: 0.35 });
      up(true); const tUp = twin.every((b: any) => sent(b) === cr), a = [sent(body), sent(wet), matWord(bare), sent(piece), sent(plain), sent(odd)];
      up(false); const tDown = twin.every((b: any) => sent(b) === own(b)), d = [sent(body), sent(wet), matWord(bare), sent(piece), sent(plain), sent(odd)]; up(true);
      line(twin.length > 0 && twin.some((b: any) => (b.rough ?? 1) > cr) && tUp && tDown && a.join() === [cr, 0.08, cr + 2, cr, 0.35, 0.35].join() && d.join() === [0.35, 0.08, 0.38 + 2, 1, 0.35, 0.35].join() && piece.rough === undefined && body.rough === 0.35,
        '2 the cases: a look\'s twin whose skin wrote a number beside the word is sent at the coat\'s, and at that number with the rule down; a body smoother than the coat stays smoother; a bare-metal row keeps its metal under the coat\'s roughness; a piece broken off a car, which copies the word and no finish, is sent at the coat\'s; a plain paint and a word that is no stuff are sent at their own; and no box\'s own number is written',
        `${TWIN}: ${twin.length} boxes, own ${tOwn}: up ${tUp}, down ${tDown} · body, wet body, metal row (the whole word), piece, plain paint, no word — up ${a.join(' ')} · down ${d.join(' ')}`); }

    // (3) every level file: each box that says a named coat is sent no rougher than the coat, and no other box of any level is sent other than its own
    { let files = 0, said = 0, off = 0, other = 0; up(true);
      for (const f of readdirSync(`${ROOT}/levels`).filter(f => f.endsWith('.json') && !f.includes('.lint')).sort()) { const T = tally(levelOf(f).boxes as any[]); if (T.said) files++; said += T.said; off += T.said - T.coated; other += T.others; }
      line(said > 400 && off === 0 && other === 0, '3 every level file through the rule: each box that says a named coat is sent at the coat\'s roughness (or its own, where that is smoother), and no other box of any level is sent other than its own', `${said} boxes in ${files} files · not the coat's ${off} · others moved ${other}`); }

    // (4) a rule of the picture: the same seeded rounds toward the yard's pickup with the rule up and down is the same run
    { const run = async (on: boolean) => { const V: any = await stand(on), g = V.game; const body = (V.level.boxes as any[]).find((b: any) => namedCoat(b.stuff) && b.h[0] > 1);
        await hushed(() => { for (let k = 0; k < 4; k++) fireWeapon(g, g.player.char, [body.c[0], body.c[1], body.c[2]], true); for (let s = 0; s < 150; s++) V.step(DT); });
        return { h: hashDigest(digestGame(g, true)) as string, marks: g.decals.list.length as number, sent: sent(body), own: body.rough as number }; };
      const a = await run(true), b = await run(false);
      line(a.h === b.h && a.marks === b.marks && a.sent === cr && b.sent === b.own && a.own === b.own && b.own !== cr, '4 a rule of the picture: four seeded rounds fired toward the yard\'s pickup and 2.5 s, with the rule up and with it down — the same run by digest and as many marks; the body is sent at the coat\'s in one and at its own in the other, and its own number is the same in both',
        `digest ${a.h} / ${b.h} · marks ${a.marks} / ${b.marks} · the body sent at ${a.sent} / ${b.sent} · its own ${a.own} / ${b.own}`); }

    // (5) the stands' list, the registry, the journal and a recording from before the table
    { const J: any = await import(`${ROOT}/src/game/journal.ts`), PR: any = await import(`${ROOT}/src/host/profile.ts`), SE: any = await import(`${ROOT}/src/host/standEnv.ts`), KN: any = await import(`${ROOT}/src/game/knobs.ts`);
      const snap = () => JSON.parse(JSON.stringify(J.knobsSnapshot(W.game)));
      COATFINISH.on = true; const old = snap(); delete old.tables.COATFINISH; J.applyKnobs(W.game, old); const before = COATFINISH.on;
      COATFINISH.on = true; const rec = snap(); COATFINISH.on = false; J.applyKnobs(W.game, rec); const kept = COATFINISH.on;
      const row = PR.FROZEN_HOLDS['COATFINISH.on'] as string | undefined;
      line(before === false && kept === true && !!row && row.startsWith('TPS_COATFINISH=1') && SE.STAND_ENV.includes('TPS_COATFINISH') && KN.JOURNAL_KNOBS.includes('COATFINISH') && KN.knobRegistry().COATFINISH === COATFINISH
        && Object.values(NAMED_COATS as Record<string, any>).every(c => typeof c.rough === 'number' && c.rough > 0 && c.rough < 1 && c.under.length > 0),
        '5 the rows: a recording from before the table plays with the rule down and one made with it up keeps it; the stands\' list has the row and the word, the journal the table, the registry\'s table is the one the finish is read by, and every named coat says how smooth it is',
        `applyKnobs without the table: ${before} · recorded up: ${kept} · FROZEN_HOLDS ${row ?? 'no row'} · STAND_ENV ${SE.STAND_ENV.includes('TPS_COATFINISH')} · journal ${KN.JOURNAL_KNOBS.includes('COATFINISH')}`); }
  } finally { Object.assign(COATFINISH, was); Math.random = rnd; }
  console.log(`=== coatfinish: ${all ? 'ALL PASS' : 'FAILURES above'} ===`);
  return all;
}
if (import.meta.main) process.exit((await coatfinishCheck()) ? 0 : 1);
