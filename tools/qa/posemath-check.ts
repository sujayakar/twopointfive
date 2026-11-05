// A POSE'S BULK ARITHMETIC IN SINGLES (src/anim/skeleton.ts POSEMATH.singles) — the switch's own check. With it on, blendPose and the step sampleClip takes
// between two keys are worked in singles, a rotation's length a plain root, in the TypeScript (Math.fround at every step) and in the Rust module (singles to
// begin with, four nodes a step: rust/sim/src/single.rs). Off = the build as it was.
//   bun run tools/qa/posemath-check.ts                 this stand (frozen by default; TPS_PROFILE=shipped = the page's). RUN IT ON BOTH.
//   POSEMATH_BREAK=1 bun run tools/qa/posemath-check.ts   the red path, must exit 1: the module is asked every blend and sample WITHOUT the rule's bit while the
//                                                      TypeScript twin works in singles — the two sides of one rule apart
// lines
//   stand    the switch stands where this stand has it (frozen off, shipped on), its word raises or drops it and it alone, FROZEN_HOLDS has its row, the
//            stands' list of words has the word, the knob registry and the journal's list have the table
//   old      a recipe with no such table (a tape from before the rule) puts it OFF before a game is built; one that carries it puts it as carried
//   twin     tools/qa/pose-ident.ts with the rule on: every sample and every blend the same bits three ways (the TypeScript twin, the module's lanes, and the
//            plain definition kept in that tool), and with the rule off, as ever
//   layers   tools/qa/layer-ident.ts with the rule on: the module's layers (which read the tick's knob) and the TypeScript's (which read the switch) leave the
//            same pose
//   game     (the shipped stand, where the rows answer) a mission with the rule on, played with the poses' and the walk's rows up and with every row off: every pose of every man after
//            every tick folds to one word (tools/qa/pose-fold.ts) — the twin and the module are one game
//   rule     …and with the rule off the word is another: the switch reaches the game
//   late     a game stood with the rule on and put off before its first tick (pose-fold's POSEMATH_LATE=0) is the game stood with it off, and one stood
//            with it off and put on (POSEMATH_LATE=1) is the game stood with it on: nothing kept from the build stands on the other arithmetic. That is
//            what lets a recording's rule be written by journal.ts applyKnobs alone, after the game is stood (tools/qa/replay.ts says no word for it)
// exit 0 = every line · 1 = a line failed. Nothing here has been seen on a GPU: it is arithmetic, and the picture's skinning reads the same poses.
import { spawnSync } from 'node:child_process';
import { resolve } from 'node:path';
const ROOT = resolve(import.meta.dir, '../..'), BREAK = process.env.POSEMATH_BREAK === '1', STAND = process.env.TPS_PROFILE === 'shipped' ? 'shipped' : 'frozen';
const WORD = 'TPS_POSESINGLES', RUN = 'hostage-rescue:1';
let fails = 0; const line = (ok: boolean, name: string, detail: string) => { if (!ok) fails++; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${name}\n    ${detail}`); };
const child = (cmd: string[], env: Record<string, string | undefined>): { out: string; status: number | null } => {
  const e: Record<string, string | undefined> = { ...process.env, POSEMATH_BREAK: BREAK ? '1' : undefined, ...env }; for (const k of Object.keys(e)) if (e[k] === undefined) delete e[k];
  const r = spawnSync(cmd[0], cmd.slice(1), { cwd: ROOT, env: e as Record<string, string>, encoding: 'utf8', maxBuffer: 1 << 26 }); return { out: (r.stdout ?? '') + (r.stderr ?? ''), status: r.status };
};
const last = (s: string) => s.trim().split('\n').pop()?.slice(0, 220) ?? '';

// ---- in a child: what a stand leaves the switch at
if (process.argv[2] === '--stood') {
  const { standUp } = await import('./headless.ts'); await standUp({ seed: 1 });
  const SK: any = await import(`${ROOT}/src/anim/skeleton.ts`); console.log('POSEMATH-STOOD ' + JSON.stringify({ singles: SK.POSEMATH.singles })); process.exit(0);
}
// ---- in a child: a recipe's tables laid, as a replay's or a networked game's page lays them before its game is built
if (process.argv[2] === '--recipe') {
  await import('./headless.ts');
  const SK: any = await import(`${ROOT}/src/anim/skeleton.ts`), { RecipeStand }: any = await import(`${ROOT}/src/replay/recipestand.ts`);
  const got: boolean[] = [];
  for (const [was, tables] of [[true, {}], [false, { POSEMATH: { singles: true } }], [true, { POSEMATH: { singles: false } }]] as [boolean, object][]) {
    SK.POSEMATH.singles = was; new RecipeStand({ search: '?roomclear&squad4', seed: 7, tables } as never).tables(); got.push(SK.POSEMATH.singles); }
  console.log('POSEMATH-RECIPE ' + JSON.stringify(got)); process.exit(0);
}
const marked = (out: string, mark: string): any => { const l = out.split('\n').find(x => x.startsWith(mark)); return l ? JSON.parse(l.slice(mark.length)) : null; };
console.log(`a pose's arithmetic in singles · the ${STAND} stand${BREAK ? ' · POSEMATH_BREAK=1 (the red path)' : ''}`);

// ---- stand
{
  const stood = (env: Record<string, string | undefined>) => marked(child(['bun', 'run', 'tools/qa/posemath-check.ts', '--stood'], { [WORD]: undefined, ...env }).out, 'POSEMATH-STOOD ')?.singles;
  const want = STAND === 'shipped', bare = stood({}), up = stood({ [WORD]: '1' }), down = stood({ [WORD]: '0' });
  const P: any = await import(`${ROOT}/src/host/profile.ts`), E: any = await import(`${ROOT}/src/host/standEnv.ts`), K: any = await import(`${ROOT}/src/game/knobs.ts`), SK: any = await import(`${ROOT}/src/anim/skeleton.ts`);
  const books = P.FROZEN_HOLDS['POSEMATH.singles'] === WORD + '=1' && (E.STAND_ENV as string[]).includes(WORD) && K.knobRegistry().POSEMATH === SK.POSEMATH && (K.JOURNAL_KNOBS as readonly string[]).includes('POSEMATH');
  line(bare === want && up === true && down === false && books, `stand: the ${STAND} stand has the rule ${want ? 'on' : 'off'}, its word raises and drops it, and the lists hold it`,
    `as stood ${bare} · ${WORD}=1 ${up} · ${WORD}=0 ${down} · FROZEN_HOLDS row, the stands' word, the registry's table and the journal's name: ${books}`);
}
// ---- old
{
  const got = marked(child(['bun', 'run', 'tools/qa/posemath-check.ts', '--recipe'], {}).out, 'POSEMATH-RECIPE ');
  line(!!got && got[0] === false && got[1] === true && got[2] === false, 'old: a recipe from before the rule puts it off before a game is built, and one that carries it puts it as carried',
    `no table over a switch that was on → ${got?.[0]} · singles:true over off → ${got?.[1]} · singles:false over on → ${got?.[2]}`);
}
// ---- twin
{
  const on = child(['bun', 'run', 'tools/qa/pose-ident.ts'], { [WORD]: '1', RUST_BREAK: undefined }), off = child(['bun', 'run', 'tools/qa/pose-ident.ts'], { [WORD]: '0', RUST_BREAK: undefined, POSEMATH_BREAK: undefined });
  const a = marked(on.out, 'POSE-IDENT '), b = marked(off.out, 'POSE-IDENT ');
  line(!!a && !!b && a.differ === 0 && b.differ === 0 && a.asked > 100000 && on.status === 0 && off.status === 0, 'twin: with the rule on every sample and blend is the same bits from the TypeScript twin, the module and the plain definition; with it off, as ever',
    `on: ${a ? `${a.asked} questions, ${a.differ} differ (sample ${a.by.sample}, lines ${a.by.lines}, blend ${a.by.blend})` : last(on.out)} · off: ${b ? `${b.asked} questions, ${b.differ} differ` : last(off.out)}`);
}
// ---- layers (asked on the page's stand: the layers' check plays one scripted mission, and under the frozen stand's rules that mission asks too few of
//      the layers for the check to call itself fair — its own verdict there is "asked too little", with or without this rule and whatever answers)
if (STAND === 'shipped') {
  const r = child(['bun', 'run', 'tools/qa/layer-ident.ts'], { [WORD]: '1', RUST_BREAK: undefined, TPS_PROFILE: 'shipped' });
  line(r.status === 0 && /layer-ident: PASS/.test(r.out), 'layers: with the rule on the module\'s layers and the TypeScript\'s leave the same pose', last(r.out));
} else console.log('- (layers: not asked on the frozen stand, where the layers\' check calls its own mission unfair: too few layers asked)');
// ---- game, rule
{
  const fold = (env: Record<string, string | undefined>) => { const r = child(['bun', 'run', 'tools/qa/pose-fold.ts', RUN], { TPS_PROFILE: STAND, QA_HOLD: 'poses=0,walk=0', ...env }); return /word ([0-9a-f]{1,8})\b/.exec(r.out)?.[1] ?? `none (${last(r.out)})`; };
  const word = (w: string) => /^[0-9a-f]{1,8}$/.test(w);
  const on = fold({ [WORD]: '1' }), off = fold({ [WORD]: '0' });
  { const onRows = fold({ [WORD]: '1', QA_HOLD: undefined });
    line(word(on) && on === onRows, `game: ${RUN} with the rule on folds every pose of every man to one word with the poses' and the walk's rows up and with them held down`, `rows held down ${on} · rows up ${onRows}`); }
  line(word(off) && word(on) && off !== on, 'rule: with the rule off the word is another: the switch reaches the game', `rule on ${on} · rule off ${off}`);
  // (the rule written on a game that is already standing: the animator's kept tables are measured in doubles under either rule — skeleton.ts inDoubles — so
  //  a game built with the rule on and then put off before its first tick is the game built with it off)
  const late = fold({ [WORD]: '1', POSEMATH_LATE: '0' }), lateOn = fold({ [WORD]: '0', POSEMATH_LATE: '1' });
  line(word(late) && late === off && word(lateOn) && lateOn === on, 'late: the rule written on a standing game, before its first tick, is the game stood with it so, both ways',
    `stood on, then put off ${late} · stood off ${off} · stood off, then put on ${lateOn} · stood on ${on}`);
}
console.log(fails ? `\n=== posemath-check: ${fails} FAILED ===` : '\n=== posemath-check: ALL PASS ===');
process.exit(fails ? 1 : 0);
