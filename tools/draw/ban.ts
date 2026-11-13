// THE PAGE DRAWS A GAME IT DOES NOT HOLD. What a rule of play written in src/ would have to do and drawing need not, each held here (`bun run build` runs this first, tools/qa/checks.ts too):
//   1 CHANGE THE GAME. A game is changed by a call that acts (a start, a hand, a command, a step, a moment put back), and only the files of ACTORS make one, by any road: the module's call itself, a command, or a member
//     of a holder that makes one. Each says what for. What an actor does with what it is told, and who presses what it
//     hands a card to press (a restart, a save), no check sees: that is what their list is for.
//   2 WRITE ITS MEMORY. The rows are read through numbers no file may write (ArrayLike), and nothing but the wire and the rows' reader names the module's memory.
//   3 STAND SOMEWHERE. Every folder of src/, and every file beside them, is said in HERE: a new one is red until it says what it is.
//   4 MAKE A LEVEL. No file of src/ names a file of tools/, not even as a type: the game read its own level file.
// What no check sees is a drawing that works something out for itself and shows it (who is hidden, what a sound hits): that is asked of the game, and read.
//   bun run tools/draw/ban.ts                 exit 0 · 1 and each line that breaks it
//   BAN_BREAK=1 bun run tools/draw/ban.ts     the red path: one of each planted, in memory: exit 1 with every one found (2: no TypeScript here to see with, 3: it missed one)
// No TypeScript on this machine (a hosted build; vendor/typescript or TS_LIB=<path to typescript.js>): 3, and 4 by what the bundle runs; and it says so.
import { existsSync, readdirSync, readFileSync, statSync } from 'node:fs';
import { dirname, join, resolve } from 'node:path';

const ROOT = resolve(import.meta.dir, '../..'), BREAK = process.env.BAN_BREAK === '1';
/** what stands in src/ */
const HERE: Readonly<Record<string, string>> = {
  anim: 'the meshes as the GPU has them', audio: 'the mixer, the voices and the score', cloud: 'the hosted page\'s side of its backend', draw: 'the page\'s half of what crosses: written files, the wire, each section read',
  gpu: 'the device, pipelines, timers, the shaders\' text', host: 'the page: its boot, its frame, the game it holds a number of, input, camera, settings', math: 'vectors and exact arithmetic for drawing', music: 'the score\'s listening page',
  net: 'a networked game: records over a wire, the server\'s line, a client', picture: 'the picture\'s own air, rain, snow and fire', render: 'the passes and their shaders', scene: 'the box world as it is drawn, the look',
  smoke: 'the picture\'s smoke', sounds: 'the sounds\' listening page', tape: 'kept runs and the replay page', ui: 'menus, the HUD\'s drawing, the debugger', view: 'what a HUD is told, read',
  'engine.ts': 'the renderer', 'settings.ts': 'the renderer\'s settings',
};
/** the files that make a game do anything, and what for */
const ACTORS: Readonly<Record<string, string>> = {
  'src/host/drawgame.ts': 'holds the number of a game: its start, its hand, its commands, its steps, its moments', 'src/host/drawpage.ts': 'the page: a tick as time goes by, the player\'s hand, a save, a load, a restart, the floor looked at behind the title',
  'src/host/drawplay.ts': 'a tape walked', 'src/host/drawnet.ts': 'a networked game walked by what the server ran', 'src/net/modgame.ts': 'a networked game stepped on its seats\' records', 'src/host/debugger.ts': 'what a person turns or presses on the panel',
  'src/host/settings.ts': 'what a person sets on Options', 'src/host/machine.ts': 'the switches this machine keeps', 'src/host/kitpage.ts': 'the kit screen\'s picture, staged as its cards say', 'src/host/drawbench.ts': 'the benchmark\'s run: nobody\'s hands on it',
};
/** who may call the module itself (every call; the rest ask a holder) */
const CALLERS = ['src/host/drawgame.ts', 'src/net/modgame.ts', 'src/host/drawnet.ts', 'src/draw/face.ts', 'src/draw/wire.ts', 'src/draw/drawn.ts', 'src/draw/ask.ts', 'src/view/desk.ts'];
/** of the module's calls, those that change a game; the rest are asked */
const CHANGE = ['start', 'drop', 'ready', 'showSeat', 'step', 'outClear', 'hand', 'commands', 'handsOff', 'handsOn', 'networked', 'record', 'recover', 'momentBack', 'debugPanic'];
const ASKED = ['stamp', 'reserve', 'handsSaid', 'handsTrace', 'ahead', 'draw', 'drawOver', 'frame', 'drawLook', 'drawLead', 'drawAs', 'drawHold', 'drawCut', 'traffic', 'tickView', 'cursorView', 'debriefView', 'endView', 'runView', 'verdict', 'floor', 'map', 'tally', 'knobs', 'shown',
  'shownNow', 'actions', 'sight', 'solid', 'ray', 'hear', 'way', 'light', 'saves', 'savesTold', 'recoverWhat', 'moment', 'momentDrop', 'deskMachine', 'deskAsk', 'stateNames', 'stateWords', 'dieN', 'tickWord'];
/** the holders' classes: a member of one acts where it makes a call that does. Their own numbers of a hand are the hand */
const HOLDERS = ['DrawGame', 'NetGame', 'DrawPlay', 'DrawNet'], HANDS = ['DrawGame.numbersOf', 'DrawGame.commands'];
const MEMORY = ['src/draw/wire.ts', 'src/draw/drawn.ts'];
/** who reaches a game through a hook a tool uses (__draw, __runFrames, __pauseLoop, __timeScale), and what for */
const HOOKED: Readonly<Record<string, string>> = { 'src/host/drawpage.ts': 'hangs them', 'src/ui/pricerun.ts': 'the pricing run holds the loop and its pace as a script would' };

const TS_LIB = process.env.TS_LIB ?? join(ROOT, 'vendor/typescript/lib/typescript.js'), ts: any = existsSync(TS_LIB) ? ((m: any) => m.default ?? m)(await import(TS_LIB)) : null;
const rel = (p: string): string => p.startsWith(ROOT) ? p.slice(ROOT.length + 1) : p, bad: string[] = [];
const all = (d: string): string[] => readdirSync(join(ROOT, d)).flatMap(n => statSync(join(ROOT, d, n)).isDirectory() ? all(`${d}/${n}`) : [`${d}/${n}`]);
/** what the red path plants: a file of its own for each road */
const PLANT: Readonly<Record<string, string>> = !BREAK ? {} : {
  'src/rules/hunger.ts': 'export const x = 1;\n',
  'src/ui/planted1.ts': "import type { DrawGame } from '../host/drawgame';\nexport const heal = (g: DrawGame): void => { if (g.time > 9) g.commands.god(1); };\n",
  'src/audio/planted2.ts': "import type { DrawGame } from '../host/drawgame';\nexport const turn = (g: DrawGame): void => { g.turn('tune.guard_chase', 9); };\n",
  'src/render/planted3.ts': "import type { Drawn } from '../draw/drawn';\nexport const hide = (D: Drawn): void => { const f = D.f as Float32Array; f[3] = 0; };\n",
  'src/gpu/planted8.ts': "import type { Drawn } from '../draw/drawn';\nexport const brighter = (D: Drawn): void => { const { f } = D; f[7] += 1; };\n",
  'src/picture/planted4.ts': "import type { Face, GameNo } from '../draw/face';\nexport const faster = (F: Face, h: GameNo): void => { F.step(h, 1 / 60, 0); };\n",
  'src/scene/planted5.ts': "import type { Face } from '../draw/face';\nexport const raw = (F: Face): Float32Array => new Float32Array(F.memory.buffer);\n",
  'src/view/planted6.ts': "import type { Level } from '../../tools/levels/lib/level';\nexport type { LevelData } from '../../tools/levels/lib/levelData';\nexport type P = import('../../tools/levels/lib/levelLoad').X;\nvoid import('../../tools/levels/floors');\nexport type L = Level;\n",
  'src/smoke/planted7.ts': "export const through = (): void => { (window as unknown as { __draw: { game: { commands: { god(n: number): void } } } }).__draw.game.commands.god(1); };\n",
};
const files = [...all('src').filter(f => f.endsWith('.ts')), ...Object.keys(PLANT)], text = (f: string): string => PLANT[f] ?? readFileSync(join(ROOT, f), 'utf8');

// ---- 3 what stands where
{ const here = new Set([...readdirSync(join(ROOT, 'src')), ...Object.keys(PLANT).map(f => f.split('/')[1])]);
  for (const n of here) if (!(n in HERE)) bad.push(`src/${n}: nothing says what it is (HERE, in this file)`);
  for (const n of Object.keys(HERE)) if (!here.has(n)) bad.push(`src/${n}: said, and not there`); }

// ---- 4 no file of a level's making
const scan = new Bun.Transpiler({ loader: 'ts' });
for (const f of files) { const named: string[] = [];
  if (!ts) named.push(...scan.scanImports(text(f)).map(i => i.path));
  else { const walk = (n: any): void => {
    if ((ts.isImportDeclaration(n) || ts.isExportDeclaration(n)) && n.moduleSpecifier) named.push(n.moduleSpecifier.text);
    else if (ts.isImportEqualsDeclaration(n) && ts.isExternalModuleReference(n.moduleReference)) named.push(n.moduleReference.expression.text);
    else if (ts.isImportTypeNode(n) && ts.isLiteralTypeNode(n.argument)) named.push(n.argument.literal.text);
    else if (ts.isCallExpression(n) && n.expression.kind === ts.SyntaxKind.ImportKeyword && ts.isStringLiteralLike(n.arguments[0])) named.push(n.arguments[0].text);
    ts.forEachChild(n, walk); }; walk(ts.createSourceFile(f, text(f), ts.ScriptTarget.Latest, true)); }
  for (const p of named) if (p.startsWith('.') && rel(resolve(ROOT, dirname(f), p)).startsWith('tools/')) bad.push(`${f}: names ${rel(resolve(ROOT, dirname(f), p))}`); }

// ---- 1 and 2, by the compiler's own types
let acts = 0;
if (ts) {
  const cfg = ts.parseJsonConfigFileContent(ts.readConfigFile(join(ROOT, 'tsconfig.json'), ts.sys.readFile).config, ts.sys, ROOT), host = ts.createCompilerHost(cfg.options), base = { file: host.fileExists.bind(host), read: host.readFile.bind(host), source: host.getSourceFile.bind(host) };
  host.fileExists = (p: string) => rel(p) in PLANT || base.file(p); host.readFile = (p: string) => PLANT[rel(p)] ?? base.read(p);
  host.getSourceFile = (p: string, v: any, ...r: any[]) => rel(p) in PLANT ? ts.createSourceFile(p, PLANT[rel(p)], v, true) : base.source(p, v, ...r);
  const P = ts.createProgram([...cfg.fileNames, ...Object.keys(PLANT).map(f => join(ROOT, f))], cfg.options, host), C = P.getTypeChecker();
  /** the class and the member a name is of, as `Class.member` */
  const member = (n: any): string => { const d = C.getSymbolAtLocation(n)?.declarations?.[0]; return d?.parent && ts.isClassLike(d.parent) && d.parent.name ? `${d.parent.name.text}.${n.text}` : ''; };
  const sources = P.getSourceFiles().filter((s: any) => rel(s.fileName).startsWith('src/')), names = (sf: any, f: (n: any, m: string) => void): void => { const walk = (n: any): void => { if (ts.isIdentifier(n) && (ts.isPropertyAccessExpression(n.parent) && n.parent.name === n || ts.isBindingElement(n.parent))) { const m = member(n); if (m) f(n, m); } ts.forEachChild(n, walk); }; walk(sf); };
  // every call is said to change a game or to ask of one
  { const face = P.getSourceFile(join(ROOT, 'src/draw/face.ts')), cls = face.statements.find((s: any) => ts.isClassDeclaration(s) && s.name?.text === 'Face'), has = cls.members.filter((m: any) => ts.isMethodDeclaration(m)).map((m: any) => m.name.text);
    for (const m of has) if (!CHANGE.includes(m) && !ASKED.includes(m)) bad.push(`Face.${m}: a call that is neither in CHANGE nor in ASKED`);
    for (const m of [...CHANGE, ...ASKED]) if (!has.includes(m)) bad.push(`Face.${m}: said, and no call`); }
  // a holder's member acts where it makes a call that does, or asks one that acts
  const acting = new Set([...CHANGE.map(m => `Face.${m}`), ...HANDS]), bodies = new Map<string, Set<string>>();
  for (const sf of sources) { const walk = (n: any): void => { if (ts.isClassLike(n) && HOLDERS.includes(n.name?.text ?? '')) for (const m of n.members) { const k = `${n.name.text}.${m.name?.text ?? 'constructor'}`, of = bodies.get(k) ?? new Set<string>(); const inner = (x: any): void => { if (ts.isIdentifier(x) && ts.isPropertyAccessExpression(x.parent) && x.parent.name === x) { const t = member(x); if (t) of.add(t); } ts.forEachChild(x, inner); }; inner(m); bodies.set(k, of); } ts.forEachChild(n, walk); }; walk(sf); }
  const does = (m: string): boolean => acting.has(m) || m.startsWith('Commands.') && m !== 'Commands.length' && m !== 'Commands.take';
  for (let more = true; more;) { more = false; for (const [k, of] of bodies) if (!acting.has(k) && [...of].some(does)) { acting.add(k); more = true; } }
  acts = acting.size;
  const done = new Set<string>();
  for (const sf of sources) { const f = rel(sf.fileName); const line = (n: any): number => sf.getLineAndCharacterOfPosition(n.getStart()).line + 1;
    names(sf, (n, m) => {
      if (m.startsWith('Face.') && !CALLERS.includes(f)) bad.push(`${f}:${line(n)}: calls the module itself (${m})`);
      else if (does(m) && !f.startsWith('src/draw/')) { done.add(f); if (!(f in ACTORS)) bad.push(`${f}:${line(n)}: makes the game do something (${m}), and is no actor`); }
      if (m === 'Wire.memory' && !MEMORY.includes(f)) bad.push(`${f}:${line(n)}: names the module's memory`); });
    const walk = (n: any): void => {
      // (the rows' numbers made something that can be written)
      if (f !== 'src/draw/drawn.ts' && (ts.isAsExpression(n) || ts.isTypeAssertionExpression?.(n))) { let x = n.expression; while (ts.isAsExpression(x) || ts.isParenthesizedExpression(x)) x = x.expression; if (ts.isPropertyAccessExpression(x) && /^Drawn\.[fu]$/.test(member(x.name))) bad.push(`${f}:${line(n)}: makes the module's numbers something it can write`); }
      const left = ts.isBinaryExpression(n) && n.operatorToken.kind >= ts.SyntaxKind.FirstAssignment && n.operatorToken.kind <= ts.SyntaxKind.LastAssignment ? n.left : ts.isPrefixUnaryExpression(n) || ts.isPostfixUnaryExpression(n) ? [ts.SyntaxKind.PlusPlusToken, ts.SyntaxKind.MinusMinusToken].includes(n.operator) ? n.operand : undefined : undefined;
      if (left && ts.isElementAccessExpression(left) && C.getTypeAtLocation(left.expression).symbol?.name === 'ArrayLike') bad.push(`${f}:${line(n)}: writes numbers that are read and not written`);
      if (ts.isIdentifier(n) && /^__(draw|runFrames|pauseLoop|timeScale)$/.test(n.text) && !(f in HOOKED)) bad.push(`${f}:${line(n)}: reaches the game through a tool's hook (${n.text})`);
      ts.forEachChild(n, walk); }; walk(sf); }
  for (const f of Object.keys(ACTORS)) if (!done.has(f)) bad.push(`${f}: said to act, and does nothing`);
} else { console.log(`draw ban: no TypeScript at ${TS_LIB}: what stands where, and what the bundle runs of tools/`); if (BREAK) process.exit(2); }

for (const b of [...new Set(bad)]) console.log(`draw ban: ${b}`);
console.log(bad.length ? `draw ban: ${bad.length} FAULTS` : `draw ban: ${files.length} files of src/, ${ts ? `${acts} ways to make a game do something and ${Object.keys(ACTORS).length} files that do` : 'each folder said'}, none of a level's making`);
if (BREAK) { const missed = Object.keys(PLANT).filter(f => !bad.some(b => b.startsWith(`${f}:`)) && !(f === 'src/rules/hunger.ts' && bad.some(b => b.startsWith('src/rules:')))), four = bad.filter(b => b.startsWith('src/view/planted6.ts')).length;
  console.log(missed.length || four < 4 ? `draw ban: the red path planted ${Object.keys(PLANT).length} files and these were not seen: ${missed.join(' ')} (${four} of 4 names)` : `draw ban: every planted file found (${Object.keys(PLANT).length})`); if (missed.length || four < 4) process.exit(3); }
process.exit(bad.length ? 1 : 0);
