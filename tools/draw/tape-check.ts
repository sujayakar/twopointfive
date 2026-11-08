// A TAPE OF A RUN (src/tape/handtape.ts), with no page: missions played on the module as the page plays them, by a hands-off player and by a hand that does anything, with
// saves, a load, a restart, knobs turned, the panel's actions, an address's words and every question a page asks between two ticks, and each tape walked on another game that is asked nothing: the die, the parts and the
// whole state are what they were at every mark and at the end. The size is said.
//   bun run tools/draw/tape-check.ts [mission id…] [--secs 30]      exit 0 · 1 and each that parted, with the tick · 2 no such mission
//   TAPE_BREAK=1 …      one step of the tape is not walked: every walk must part
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';
import { gzipSync } from 'node:zlib';
import { faceHere } from '../blitz/face-build.ts';
import { characterFiles } from '../blitz/start-pack.ts';
import { DrawGame, type Start } from '../../src/host/drawgame';
import { HAND, sampleHand } from '../../src/draw/hand';
import { DrawPlay } from '../../src/host/drawplay';
import { HandTape, MARKS } from '../../src/tape/handtape';
import { FollowCamera } from '../../src/host/camera';
import { Section, View } from '../../src/draw/sections';

const ROOT = resolve(import.meta.dir, '../..'), BREAK = process.env.TAPE_BREAK === '1', args = process.argv.slice(2), at = args.indexOf('--secs'), SECS = at >= 0 ? Number(args[at + 1]) : 30;
const only = args.filter((a, i) => !a.startsWith('--') && (at < 0 || i !== at + 1));
const { ALL_MISSIONS, pickSearch } = await import(`${ROOT}/src/scene/scenarioList.ts`), { LOOK } = await import(`${ROOT}/src/scene/look.ts`);
LOOK.on = true;
const wasm = faceHere(); if (!wasm) { console.error('tape-check: no module of these sources (tools/blitz/face-build.ts)'); process.exit(1); }
const module = new WebAssembly.Module(new Uint8Array(readFileSync(wasm)));
const stand = (): DrawGame => new DrawGame({ exports: new WebAssembly.Instance(module, { env: { face_roll: () => { throw new Error('the game asked a die'); } } }).exports, bytes: 0, fetchMs: 0, compileMs: 0 });
const [mannequin, mannequinBin, cast, castLook] = characterFiles(ROOT);
const bytes = (f: string): Uint8Array => new Uint8Array(readFileSync(resolve(ROOT, 'levels', f)));

/** a die of the check's own */
const die = (s: number) => () => { s = (s + 0x6d2b79f5) | 0; let t = Math.imul(s ^ (s >>> 15), 1 | s); t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t; return ((t ^ (t >>> 14)) >>> 0) / 4294967296; };
const KEYS = ['KeyW', 'KeyA', 'KeyS', 'KeyD', 'ShiftLeft', 'KeyC'], DOWN = ['Space', 'KeyB', 'KeyF', 'KeyR', 'Digit1', 'Digit2', 'Digit3', 'Digit4', 'KeyV', 'F5', 'Escape', 'KeyG'];

let fails = 0;
{ const stray = only.filter(id => !(ALL_MISSIONS as { id: string }[]).some(m => m.id === id)); if (stray.length) { console.error(`tape-check: no mission ${stray.join(', ')}`); process.exit(2); } }
for (const m of ALL_MISSIONS as { id: string }[]) { if (only.length && !only.includes(m.id)) continue;
  for (const how of ['hands off', 'anything'] as const) {
    const words: string = pickSearch(m, ''), file = new URLSearchParams(words).get('levelFile') ?? 'office.json', seed = how === 'anything' ? 7 : 5, roll = die(seed + m.id.length);
    let at = 'the start';
    try {
      // ── played, with a page's questions between its ticks
      const g = stand(), D = g.drawn, cam = new FollowCamera(), input = { keys: new Set<string>(), pressed: new Set<string>(), buttons: 0, clicked: 0, wheel: 0, mouseX: 640, mouseY: 360 }, canvas = { clientWidth: 1280, clientHeight: 720, width: 1280, height: 720 };
      const T = g.tape = new HandTape(HAND.count), S: Start = { level: bytes(file), words: how === 'anything' ? `${words}&qtwice=0&firelean=0` : words, mannequin, mannequinBin, cast, castLook, seed, named: { level: file, build: 'this' } };
      if (how === 'anything') g.commands.difficulty(1);
      g.start(S);
      const follow = (): [number, number, number] => { const o = D.at[Section.view] + View.follow; return [D.f[o], D.f[o + 1], D.f[o + 2]]; };
      cam.aspect = 1280 / 720; cam.target = follow(); cam.rebuild();
      g.commands.god(1); if (how === 'hands off') g.handsOff({ seed, cap: SECS });
      g.tell();
      for (let i = 1; i <= SECS * 60; i++) { at = `tick ${i}`;
        if (how === 'anything') {
          if (roll() < 0.05) { input.keys.clear(); for (const k of KEYS) if (roll() < 0.3) input.keys.add(k); }
          if (roll() < 0.04) input.pressed.add(DOWN[Math.floor(roll() * DOWN.length)]);
          if (roll() < 0.1) { input.mouseX = roll() * 1280; input.mouseY = roll() * 720; }
          if (roll() < 0.03) { input.buttons = input.buttons ? 0 : 1 + Math.floor(roll() * 3); if (input.buttons & 1) input.clicked = 1; }
          if (i === 900) g.commands.load('manual.0'); if (i === 1500) { g.commands.restart(''); g.tell(); } if (i === 300) g.commands.unseen(1);
          if (i === 600) { g.turn('wet.prints', 3); g.commands.act('soak', 0, ''); g.commands.act('extra', 2, ''); g.commands.act('nerve', 0, 'brittle'); }
        } else if (i === 400) g.commands.save(); else if (i === 1200) g.commands.load('manual.0');
        g.step(1 / 60, sampleHand(g.numbersOf(), input, cam, canvas));
        input.pressed.clear(); input.clicked = 0; const o = D.at[Section.view] + View.aim; cam.step(1 / 60, follow(), [D.f[o], D.f[o + 1], D.f[o + 2]]);
        if (i % 3 === 0) { g.frame(0.5, cam.target); D.fresh(); g.frame(1, cam.target); } D.fresh();
        if (i % 7 === 0) { g.view(); g.cursor(input, cam, canvas); } if (i % 60 === 0) { g.verdict(); g.saves(); g.tally(); g.run(); } if (i % 600 === 0) g.debrief();
        if (i % MARKS.whole === 0) T.mark(g.mark(true)); }
      const done = g.tally(), end = g.mark(true); T.end('the check stopped');

      // ── walked, by a game that is asked nothing
      const b = T.whole(), w = stand(), P = new DrawPlay(w, b), feed = w.feed.bind(w); if (BREAK) w.feed = (row, seen) => { if (row.kind !== 'step' || row.tick !== 600) feed(row, seen); else w.ticks++; };
      P.start({ mannequin, mannequinBin, cast, castLook }); const t0 = performance.now();
      while (P.tick(false) && !P.parted) at = `the walk's tick ${P.at}`;
      const ms = performance.now() - t0, last = w.mark(true), bad = P.parted ? [`at tick ${P.parted.tick}: ${JSON.stringify(P.parted.mine)}, on the tape ${JSON.stringify(P.parted.said)}`] : (['die', 'parts', 'state'] as const).filter(k => last[k] !== end[k]).map(k => `the ${k} at the end`);
      const gz = gzipSync(b).length - gzipSync(S.level).length, wrong = bad.length || g.lost || w.lost || P.at !== SECS * 60 || (how === 'anything' && SECS > 25 && !P.tries.includes(1500) && !P.tries.includes(1499));
      if (wrong) fails++;
      console.log(`- ${wrong ? 'FAIL' : 'PASS'} — ${m.id}, ${how}: ${P.at} ticks walked in ${(ms / 1000).toFixed(1)} s, ${(gz / 1024 / SECS * 60).toFixed(0)} KB a minute packed, ${done.manual} saved by hand, ${done.autosaves} by itself, ${done.loads} loaded, ${P.tries.length} begun again${bad.length ? `\n    ${bad.slice(0, 3).join(' · ')}` : ''}`);
    } catch (e) { fails++; console.log(`- FAIL — ${m.id}, ${how}: stopped in ${at}\n    ${String((e as Error)?.message ?? e).slice(0, 400)}`); }
  } }
console.log(`=== tape: ${fails ? `${fails} FAILED` : 'ALL PASS'} ===`);
process.exit(fails ? 1 : 0);
