// WHAT HOLDS A FAMILY OF THE RUST MODULE THAT HAS NO TWIN (docs/agents/rust.md "What holds a family once its TypeScript is gone"). While a system stood
// in both languages its check ran every call both ways on the same input and compared the answers. With one language there is nothing to compare a call
// to, so a family's check holds its calls to THEIR OWN EARLIER ANSWERS, four ways, and this file is the four:
//   THE WORD      every call the tool watches is folded into a word under a name (a layer, a kind of question): the bits of each number as they stand
//                 (a single as a single, a double as its two halves, every not-a-number as one word, since a NaN's own bits are nobody's rule), two lanes
//                 of thirty-two bits, with the count of calls beside it.                                                   `Fold`
//   THE RED PATH  needs no recording: under the family's break word the tool plays the same run again, plain, in a child, and EVERY name's word must
//                 differ from its own. A name whose word stayed is a call the break did not reach.                          `--words` is the child's
//                 (It says less than two languages did: a name's word also moves when the break bent what came BEFORE it in the run, so a moved
//                 word is "the run under this name moved", not "this name's own arithmetic was bent". Only a word that stayed is certain.)
//   THE PIN       a fixture under tools/qa/fixtures, recorded ON THE FROZEN STAND ONLY, where a run's course moves by the frozen digests' ceremony
//                 (on the page's stand it moves with every rule that lands, and a recording there would be written again by reflex and hold nothing).
//                 `--record` writes it and says which tree it was taken on (and refuses to let go of a name the recording held, but with `--forget`). The first recording of a family is taken on the last tree where both
//                 languages stood and agreed on every call of that same run; that is all the pin ever knew of the TypeScript.
//   --against     `--against <root>`: the same run played by the tool of another checkout, every count and word the same. For a change that must not
//                 move an answer (the tree before, the tree after), on any stand, with no file.
// IT IS A LOWER BAR than two languages a call: a mistake both recordings share is held as firmly as the truth.
// A tool that uses it: makes a `Fold`, folds each watched call, and after its run hands `hold` its words (tools/qa/layer-ident.ts is the first; how a
// family takes it up, step by step: docs/agents/rust.md, the same bullet). Not game code.
import { resolve } from 'node:path';
import { readFileSync, writeFileSync, existsSync } from 'node:fs';
import { spawnSync } from 'node:child_process';

export type Words = Record<string, { asked: number; word: string }>;
interface Lane { asked: number; a: number; b: number }
const u1 = new Uint32Array(1), s1 = new Float32Array(u1.buffer), u2 = new Uint32Array(2), d1 = new Float64Array(u2.buffer);
/** the calls of a run, folded a name */
export class Fold {
  private seen: Record<string, Lane> = {};
  /** a call under `name` begins: its lane, to fold what the call left into */
  call(name: string): Lane { const e = this.seen[name] ?? (this.seen[name] = { asked: 0, a: 2166136261, b: 0x9e3779b9 }); e.asked++; return e; }
  word(e: Lane, w: number): void { e.a = Math.imul(e.a ^ w, 16777619) >>> 0; let b = Math.imul((e.b + w) | 0, 0x85ebca6b); b ^= b >>> 13; e.b = Math.imul(b, 0xc2b2ae35) >>> 0; }
  single(e: Lane, v: number): void { if (v !== v) this.word(e, 0x7fc00000); else { s1[0] = v; this.word(e, u1[0]); } }
  double(e: Lane, v: number): void { if (v !== v) { this.word(e, 0x7ff80000); this.word(e, 0); } else { d1[0] = v; this.word(e, u2[0]); this.word(e, u2[1]); } }
  singles(e: Lane, d: ArrayLike<number>): void { for (let i = 0; i < d.length; i++) this.single(e, d[i]); }
  doubles(e: Lane, d: ArrayLike<number>): void { for (let i = 0; i < d.length; i++) this.double(e, d[i]); }
  words(): Words { return Object.fromEntries(Object.keys(this.seen).sort().map(n => [n, { asked: this.seen[n].asked, word: this.seen[n].a.toString(16).padStart(8, '0') + this.seen[n].b.toString(16).padStart(8, '0') }])); }
}

const MARK = 'IDENT-WORDS ';
export interface Held {
  /** the names whose count or word is not the one they were held to (under the break word: the names that DID move are not in it — see `stayed`) */
  off: string[];
  /** under the break word: the names whose word did not move */
  stayed: string[];
  /** what the words were held to ('' = nothing: the page's stand, plain) */
  to: string;
  /** under `--record`, when the run may be recorded: WRITE THE RECORDING. The caller calls it last, and only when every verdict of its run passed (a
   *  verdict judged after `hold` returns counts too): a recording is the answers of a run that was right. */
  record?: () => void;
}
/** HOLD A RUN'S WORDS. `o.tool` = the tool's path from the checkout's root · `o.fixture` = its file's name under tools/qa/fixtures · `o.run` = what was
 *  played, as the tool's own arguments say it (`o.runArgs` play it again) · `o.breakWord` = the family's break word, set in this process or not ·
 *  `o.unfit` = why this run may not be recorded ('' = it may). It prints the child's line when asked for one (`--words`: the caller then exits),
 *  judges through `o.verdict`, and answers what moved; under `--record` it writes nothing itself and answers `record` (see Held). */
export function hold(o: { root: string; tool: string; fixture: string; run: string; runArgs: string[]; end: string; words: Words; breakWord: string; unfit: string;
  verdict: (ok: boolean, label: string, detail: string) => void; say: (line: string) => void }): Held {
  const args = process.argv.slice(2), flag = (n: string) => { const i = args.indexOf(n); return i >= 0 ? args[i + 1] : undefined; };
  const stand = String(process.env.TPS_PROFILE ?? 'frozen'), FIX = `${o.root}/tools/qa/fixtures/${o.fixture}`, BREAK = process.env[o.breakWord] === '1', words = o.words;
  o.say(MARK + JSON.stringify({ run: o.run, stand, end: o.end, words }));
  if (args.includes('--words')) return { off: [], stayed: [], to: '' };
  const child = (root: string, env: Record<string, string | undefined>): { words: Words | null; say: string } => {
    const e: Record<string, string | undefined> = { ...process.env, ...env }; for (const k of Object.keys(e)) if (e[k] === undefined) delete e[k];
    const c = spawnSync('bun', ['run', `${root}/${o.tool}`, '--words', ...o.runArgs], { cwd: root, env: e as Record<string, string>, encoding: 'utf8', maxBuffer: 1 << 26 });
    const l = String(c.stdout ?? '').split('\n').find((x: string) => x.startsWith(MARK));
    return l ? { words: JSON.parse(l.slice(MARK.length)).words, say: '' } : { words: null, say: `the child left no words (exit ${c.status}${c.error ? `, ${String(c.error.message)}` : ''}): ${String(c.stderr || c.stdout || '').trim().split('\n').slice(-2).join(' / ').slice(0, 300)}` };
  };
  const file: { what: string; stands: Record<string, { run: string; end: string; tree: string; words: Words }> } = existsSync(FIX) ? JSON.parse(readFileSync(FIX, 'utf8')) : { what: '', stands: {} };
  const against = flag('--against'), names = (h: Words) => [...new Set([...Object.keys(words), ...Object.keys(h)])].sort(), calls = Object.values(words).reduce((a, w) => a + w.asked, 0);
  const differ = (h: Words) => names(h).filter(n => words[n]?.asked !== h[n]?.asked || words[n]?.word !== h[n]?.word);
  const show = (h: Words, ns: string[]) => ns.slice(0, 12).map(n => `${n} ${words[n]?.asked ?? 0} calls ${words[n]?.word ?? 'none'}, held to ${h[n]?.asked ?? 0} calls ${h[n]?.word ?? 'none'}`).join(' · ') + (ns.length > 12 ? ' · …' : '');
  if (BREAK) {   // ---- the red path: every word must MOVE against the same run played plain
    const c = child(o.root, { [o.breakWord]: undefined }), to = 'the same run played plain by a child';
    // (no plain run to hold the words to: nothing is known to have moved, and a tool that reads `off` as "the break reached these" must not be told it did)
    if (!c.words) { o.verdict(false, `same: every count and word is that of ${to}`, `NOTHING COMPARED: ${c.say}`); return { off: [], stayed: Object.keys(words), to }; }
    const off = differ(c.words), stayed = names(c.words).filter(n => !off.includes(n));
    o.verdict(off.length === 0, `same: every count and word is that of ${to}`, `${off.length} of ${names(c.words).length} DIFFER${stayed.length ? ` · THE BREAK DID NOT REACH: ${stayed.join(', ')}` : ' (every one: the break reaches each)'}: ${show(c.words, off)}`);
    return { off, stayed, to };
  }
  if (args.includes('--against') && !against) { o.verdict(false, 'same: every count and word is that of another checkout\'s run', 'NOTHING COMPARED: --against wants the other checkout\'s root after it'); return { off: [], stayed: [], to: '' }; }
  if (against) {
    const c = child(resolve(against), {}), to = `the run as the checkout at ${resolve(against)} plays it`;
    if (!c.words) { o.verdict(false, `same: every count and word is that of ${to}`, `NOTHING COMPARED: ${c.say}`); return { off: [], stayed: [], to }; }
    const off = differ(c.words); o.verdict(off.length === 0, `same: every count and word is that of ${to}`, off.length ? `${off.length} of ${names(c.words).length} DIFFER: ${show(c.words, off)}` : `${names(c.words).length} names, ${calls} calls`);
    return { off, stayed: [], to };
  }
  if (args.includes('--record')) {
    // (a name the recording holds and this run does not reach is not let go of quietly: what a recording holds can shrink only by a word said for it)
    const gone = Object.keys(file.stands[stand]?.words ?? {}).filter(n => !words[n]);
    const why = stand !== 'frozen' ? 'a recording is the frozen stand\'s (TPS_PROFILE=frozen): on the page\'s stand a run\'s course moves with every rule that lands' : o.unfit
      || (gone.length && !args.includes('--forget') ? `the recording holds ${gone.join(', ')} and this run does not reach ${gone.length > 1 ? 'them' : 'it'} (--record --forget writes a recording without)` : '');
    if (why) { o.verdict(false, 'same: the run is recorded', `NOTHING WRITTEN: ${why}`); return { off: [], stayed: [], to: '' }; }
    const git = (...a: string[]) => String(spawnSync('git', ['-C', o.root, ...a], { encoding: 'utf8' }).stdout ?? '').trim();
    const tree = flag('--tree') ?? `${git('log', '-1', '--format=%h %s')}${git('status', '--porcelain', '--', 'src', 'rust') ? ', with changes not committed' : ''}`;
    return { off: [], stayed: [], to: '', record: () => {
      file.what = `${o.tool}: every name's count of calls and word over its run on the frozen stand (tools/qa/ident-pin.ts; --record writes it)`;
      file.stands[stand] = { run: o.run, end: o.end, tree, words };
      writeFileSync(FIX, JSON.stringify(file, null, 1) + '\n');
      o.say(`- RECORDED — same: ${Object.keys(words).length} names, ${calls} calls, written to tools/qa/fixtures/${o.fixture} for the ${stand} stand · taken on: ${tree}`);
    } };
  }
  const pin = file.stands[stand];
  if (pin && pin.run === o.run) {
    const off = differ(pin.words), to = `the recording (tools/qa/fixtures/${o.fixture}, the ${stand} stand; taken on: ${pin.tree})`;
    o.verdict(off.length === 0 && pin.end === o.end, `same: every count and word is that of ${to}`, off.length ? `${off.length} of ${names(pin.words).length} DIFFER: ${show(pin.words, off)}` : `${names(pin.words).length} names, ${calls} calls${pin.end === o.end ? '' : ` · THE RUN ENDS ${o.end}, the recording's ended ${pin.end}`}`);
    return { off, stayed: [], to };
  }
  // (nothing to hold the words to. On the frozen stand that is a recording missing, which is red: the pin is that stand's whole job. A recording of
  //  ANOTHER run, or the page's stand, is said and is not a failure: the caller asked for words nothing was ever held to)
  if (stand === 'frozen' && !pin) o.verdict(false, `same: every count and word is that of the recording (tools/qa/fixtures/${o.fixture})`, `NO RECORDING for the frozen stand: nothing was compared (--record writes one, on a tree whose answers are known)`);
  else o.say(`- NOT HELD — same: ${pin ? `the recording is of ${pin.run}, not of ${o.run}` : `the ${stand} stand has no recording`} and nothing else was asked for (--against <root>): the words are in the ${MARK.trim()} line`);
  return { off: [], stayed: [], to: '' };
}
