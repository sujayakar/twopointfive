// A TAPE OF A RUN (src/tape/handtape.ts), with no page: missions played on the module as draw.html plays them, by a hands-off player and by a hand that does anything, with
// saves, a load, a restart, knobs turned and every question a page asks between two ticks, and each tape walked on another game that is asked nothing: the die, the parts and the
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
import { HandTape, HandWalk, MARKS, hashOf, type TapeMark } from '../../src/tape/handtape';
import { FollowCamera } from '../../src/game/camera';
import { Section, View } from '../../src/draw/sections';

const ROOT = resolve(import.meta.dir, '../..'), BREAK = process.env.TAPE_BREAK === '1', args = process.argv.slice(2), at = args.indexOf('--secs'), SECS = at >= 0 ? Number(args[at + 1]) : 30;
const only = args.filter((a, i) => !a.startsWith('--') && (at < 0 || i !== at + 1));
const { ALL_MISSIONS, pickSearch } = await import(`${ROOT}/src/scene/scenarioList.ts`), { lookLevelFile, LOOK } = await import(`${ROOT}/src/scene/look.ts`);
LOOK.on = true;
const wasm = faceHere(); if (!wasm) { console.error('tape-check: no module of these sources (tools/blitz/face-build.ts)'); process.exit(1); }
const module = new WebAssembly.Module(new Uint8Array(readFileSync(wasm)));
const stand = (): DrawGame => new DrawGame({ exports: new WebAssembly.Instance(module, { env: { face_roll: () => { throw new Error('the game asked a die'); } } }).exports, bytes: 0, fetchMs: 0, compileMs: 0 });
const [mannequin, mannequinBin, cast, castLook] = characterFiles(ROOT), enc = new TextEncoder();
const bytes = (f: string): Uint8Array => { try { return new Uint8Array(readFileSync(resolve(ROOT, 'levels', lookLevelFile(f)))); } catch { return new Uint8Array(readFileSync(resolve(ROOT, 'levels', f))); } };

const out = (g: DrawGame): Uint8Array => { const p = g.X.face_out_ptr() >>> 0, n = g.X.face_out_len() >>> 0; return new Uint8Array(g.memory.buffer.slice(p, p + n)); };
const parts = (g: DrawGame): number => hashOf(new Uint8Array(g.memory.buffer, g.X.face_part_hashes(g.X.face_slot(g.h)) >>> 0, g.X.face_parts() * 4));
const state = (g: DrawGame): number => { g.X.face_state_words(g.h); const h = hashOf(out(g)); g.X.face_out_clear(g.h); return h; };
const mark = (g: DrawGame, tick: number): TapeMark => ({ tick, die: g.X.face_die_n(g.h), parts: parts(g), state: tick % MARKS.whole === 0 ? state(g) : null });

/** a die of the check's own */
const die = (s: number) => () => { s = (s + 0x6d2b79f5) | 0; let t = Math.imul(s ^ (s >>> 15), 1 | s); t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t; return ((t ^ (t >>> 14)) >>> 0) / 4294967296; };
const KEYS = ['KeyW', 'KeyA', 'KeyS', 'KeyD', 'ShiftLeft', 'KeyC'], DOWN = ['Space', 'KeyB', 'KeyF', 'KeyR', 'Digit1', 'Digit2', 'Digit3', 'Digit4', 'KeyV', 'F5', 'Escape', 'KeyG'];

let fails = 0;
{ const stray = only.filter(id => !(ALL_MISSIONS as { id: string }[]).some(m => m.id === id)); if (stray.length) { console.error(`tape-check: no mission ${stray.join(', ')}`); process.exit(2); } }
for (const m of ALL_MISSIONS as { id: string }[]) { if (only.length && !only.includes(m.id)) continue;
  for (const how of ['hands off', 'anything'] as const) {
    const words: string = pickSearch(m, ''), file = new URLSearchParams(words).get('levelFile') ?? 'office.json', seed = how === 'anything' ? 7 : 5, roll = die(seed + m.id.length);
    let at = 'the start', end: TapeMark;
    try {
      // ── played, with a page's questions between its ticks
      const g = stand(), D = g.drawn, cam = new FollowCamera(), input = { keys: new Set<string>(), pressed: new Set<string>(), buttons: 0, clicked: 0, wheel: 0, mouseX: 640, mouseY: 360 }, canvas = { clientWidth: 1280, clientHeight: 720, width: 1280, height: 720 };
      const take = g.commands.take.bind(g.commands); let said: Uint8Array = new Uint8Array(0); g.commands.take = () => said = take().slice();   // (what a step took: the recorder's place is in DrawGame)
      const T = new HandTape(HAND.count), S: Start = { level: bytes(file), words, mannequin, mannequinBin, cast, castLook, seed };
      if (how === 'anything') g.commands.difficulty(1);
      g.start(S);
      T.start({ build: 'this', levelName: file, level: S.level, words, parts: [['mannequin', mannequin], ['mannequinBin', mannequinBin], ['cast', cast], ['castLook', castLook]].map(([name, b]) => ({ name: name as string, length: (b as Uint8Array).length, hash: hashOf(b as Uint8Array) })), seed, seats: 1, flags: 1, commands: said });
      const tell = (): void => { said = new Uint8Array(0); g.tell(); if (said.length) T.order(said); };
      const follow = (): [number, number, number] => { const o = D.at[Section.view] + View.follow; return [D.f[o], D.f[o + 1], D.f[o + 2]]; };
      cam.aspect = 1280 / 720; cam.target = follow(); cam.rebuild();
      g.commands.god(1); if (how === 'hands off') { const plan = { seed, cap: SECS }; g.handsOff(plan); T.hands(enc.encode([plan.seed, plan.cap, 0].join('\n'))); }
      tell();
      for (let i = 1; i <= SECS * 60; i++) { at = `tick ${i}`;
        if (how === 'anything') {
          if (roll() < 0.05) { input.keys.clear(); for (const k of KEYS) if (roll() < 0.3) input.keys.add(k); }
          if (roll() < 0.04) input.pressed.add(DOWN[Math.floor(roll() * DOWN.length)]);
          if (roll() < 0.1) { input.mouseX = roll() * 1280; input.mouseY = roll() * 720; }
          if (roll() < 0.03) { input.buttons = input.buttons ? 0 : 1 + Math.floor(roll() * 3); if (input.buttons & 1) input.clicked = 1; }
          if (i === 900) g.commands.load('manual.0'); if (i === 1500) { g.commands.restart(''); tell(); } if (i === 300) g.commands.unseen(1);
        } else if (i === 400) g.commands.save(); else if (i === 1200) g.commands.load('manual.0');
        said = new Uint8Array(0); const keys = sampleHand(g.numbersOf(), input, cam, canvas), hand = g.numbersOf().slice();
        g.step(1 / 60, keys); if (said.length) T.order(said); T.step(1 / 60, hand, keys);
        input.pressed.clear(); input.clicked = 0; const o = D.at[Section.view] + View.aim; cam.step(1 / 60, follow(), [D.f[o], D.f[o + 1], D.f[o + 2]]);
        if (i % 3 === 0) { g.frame(0.5, cam.target); D.fresh(); g.frame(1, cam.target); } D.fresh();
        if (i % 7 === 0) { g.view(); g.cursor(input, cam, canvas); } if (i % 60 === 0) { g.verdict(); g.saves(); g.tally(); g.run(); } if (i % 600 === 0) g.debrief();
        if (i % MARKS.every === 0) T.mark(mark(g, i)); }
      const done = g.tally();
      end = { ...mark(g, SECS * 60), state: state(g) }; T.end('the check stopped');

      // ── walked, by a game that is asked nothing
      const b = T.whole(), W = new HandWalk(b, HAND.count), w = stand(), bad: string[] = [];
      w.commands.take = () => W.start.commands;
      w.start({ level: W.start.level, words: W.start.words, mannequin, mannequinBin, cast, castLook, seed: W.start.seed, seats: W.start.seats });
      const X = w.X as Record<string, (...a: number[]) => number>, handed = (v: Uint8Array, f: (p: number, n: number) => void): void => { const p = X.face_alloc(v.length) >>> 0; new Uint8Array(w.memory.buffer, p, v.length).set(v); f(p, v.length); X.face_free(p, v.length); };
      let r; const t0 = performance.now();
      while ((r = W.next()) && !bad.length) { at = `the walk's tick ${W.ticks}`;
        if (r.kind === 'step') { if (BREAK && r.tick === 600) continue; w.numbersOf().set(r.hand); w.step(r.dt, r.keys); }
        else if (r.kind === 'order') handed(r.bytes, (p, n) => X.face_commands(w.h, p, n)); else if (r.kind === 'hands') handed(r.plan, (p, n) => X.face_hands_off(w.h, 0, p, n));
        else if (r.kind === 'mark') { const now = mark(w, r.mark.tick); for (const k of ['die', 'parts', 'state'] as const) if (now[k] !== r.mark[k]) bad.push(`the ${k} at tick ${r.mark.tick}: ${now[k]}, on the tape ${r.mark[k]}`); } }
      const ms = performance.now() - t0, last: TapeMark = { ...mark(w, SECS * 60), state: state(w) }; for (const k of ['die', 'parts', 'state'] as const) if (!bad.length && last[k] !== end[k]) bad.push(`the ${k} at the end`);
      const gz = gzipSync(b).length - gzipSync(W.start.level).length, lost = g.lost || w.lost;
      if (bad.length || lost || W.ticks !== SECS * 60) fails++;
      console.log(`- ${bad.length || lost || W.ticks !== SECS * 60 ? 'FAIL' : 'PASS'} — ${m.id}, ${how}: ${W.ticks} ticks walked in ${(ms / 1000).toFixed(1)} s, ${(gz / 1024 / SECS * 60).toFixed(0)} KB a minute packed, ${done.manual} saved by hand, ${done.autosaves} by itself, ${done.loads} loaded${bad.length ? `\n    ${bad.slice(0, 3).join(' · ')}` : ''}`);
    } catch (e) { fails++; console.log(`- FAIL — ${m.id}, ${how}: stopped in ${at}\n    ${String((e as Error)?.message ?? e).slice(0, 400)}`); }
  } }
console.log(`=== tape: ${fails ? `${fails} FAILED` : 'ALL PASS'} ===`);
process.exit(fails ? 1 : 0);
