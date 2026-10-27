// THE ONE DOOR A FILE LEAVES THE PAGE BY (src/host/savefile.ts), on a fake page. A kind that has given its download up (a take, a result, a journal) goes to
// whoever listens and to nobody else: no download is asked for, with a listener or without, and the door answers what became of the file ('sent', 'held',
// false) so its saver can say so; `?downloads=1` (SAVE_UP.on = false) is the door as it was. A kind still downloaded (a level, and a replay and a checkpoint
// until their buttons say otherwise) is asked for once under the file's name and the listener told after it; a download that cannot be asked for THROWS (a
// caller must not mark the file handed over) and nobody is told. A listener that throws, or whose promise is refused, loses nothing. A saver hears
// once, later, how a file that was taken ended. The door over the backend's real file intake, mapped as main.ts maps it. The words a saver says; the recorder's take and the others go through the door; and no file under src/ makes a download of its own (by `.download =` or createObjectURL).
//   bun run tools/qa/savefile-check.ts   (exit 0 = all pass; SAVEFILE_BREAK=1 = nobody listens: must fail)
import { readFileSync, readdirSync } from 'node:fs';
import { join } from 'node:path';
import { SAVED, SAVE_UP, saveFile, tellSaved, downloads, wanted, savedWords, savedEnd, type Saved, type SavedDone, type SavedFile, type Went } from '../../src/host/savefile';
import { FileSync, FILESYNC } from '../../src/cloud/files';
import { wire } from '../../src/cloud/wire';
const is = (w: Went, went: Saved, local: boolean) => w.went === went && w.local === local;
import { PerfRecorder } from '../../src/ui/perfrec';
import { downloadReplay } from '../../src/replay/format';
import { download as editorDownload } from '../../src/editor/persist';
const BREAK = process.env.SAVEFILE_BREAK === '1';
let red = 0; const check = (ok: boolean, w: string) => { console.log(`${ok ? '- PASS' : '- FAIL'} — ${w}`); if (!ok) red++; };
const G = globalThis as any;
G.fetch = () => Promise.reject(new Error('no server in this check'));   // (the recorder also POSTs a take to the shot server of a page on this machine: not from here)
let rejections = 0; process.on('unhandledRejection', () => { rejections++; });

// a page: anchors that count their clicks, blob addresses that count what was made and what was let go
const clicks: { name: string; href: string }[] = []; let made = 0, revoked = 0, failClick = false;
const page = () => { G.document = { createElement: () => { const a: any = { href: '', download: '', click() { if (failClick) throw new Error('the browser refused'); clicks.push({ name: a.download, href: a.href }); }, remove() {} }; return a; }, body: { appendChild() {} } }; };
const noPage = () => { delete G.document; };
G.URL.createObjectURL = () => `blob:fake/${++made}`; G.URL.revokeObjectURL = () => { revoked++; };
const told: SavedFile[] = []; const order: string[] = [];
const listen = (takes: Saved | boolean | undefined = true) => { SAVED.sink = BREAK ? null : f => { order.push(`told ${f.name} after ${clicks.length} clicks`); told.push(f); return takes; }; };

page();
{ // nobody listens
  SAVED.sink = null; const went = saveFile('perf', 'a.json', new Blob(['{}'])), went2 = saveFile('level', 'a2.json', new Blob(['{}']));
  check(is(went, false, false) && is(went2, false, true) && !wanted('perf') && wanted('level') && clicks.length === 1 && clicks[0].name === 'a2.json' && /^blob:fake\//.test(clicks[0].href), `nobody listening: a take goes nowhere and the door says so, with no download; a level is still downloaded, once, under its name (${clicks.map(c => c.name).join(', ')})`);
  check(!downloads('perf') && !downloads('bench') && !downloads('session') && downloads('level') && downloads('replay') && downloads('checkpoint') && SAVE_UP.on, `a take, a result and a journal have given their download up; a level, a replay and a checkpoint are still downloaded (${SAVE_UP.local.join(', ')})`);
}
{ // a listener
  listen(); const blob = new Blob(['{"x":1}'], { type: 'application/json' }), c = clicks.length;
  const went = saveFile('perf', 'b.json', blob, { meta: { commit: 'abc', of: undefined } });
  check(is(went, 'sent', false) && wanted('perf') && clicks.length === c && told.length === 1 && told[0].kind === 'perf' && told[0].name === 'b.json' && told[0].blob === blob && JSON.stringify(told[0].meta) === '{"commit":"abc"}', 'a listener is handed what was saved and no download is asked for: the kind, the name, the same bytes, the facts that were given');
  saveFile('level', 'c.json', new Blob(['x']), { meta: { commit: undefined } });
  check(told.length === 2 && !('meta' in told[1]) && clicks.length === c + 1 && order[1] === `told c.json after ${c + 1} clicks`, 'a level is downloaded and the listener told after it; a file with no facts carries none');
  const answers: [Saved | boolean | undefined, Saved][] = [['sent', 'sent'], [true, 'sent'], ['held', 'held'], [false, false], [undefined, false]];
  const got = answers.map(([a]) => { if (a === undefined) SAVED.sink = BREAK ? null : () => {}; else listen(a); return saveFile('bench', 'c2.json', new Blob(['x'])); });
  check(!BREAK && got.every((g, i) => is(g, answers[i][1], false)) && clicks.length === c + 1, `the door hands the listener's answer back: sent, held, or nothing (a listener that answers nothing has not said it is sending) (${got.map(g => g.went).join(', ')})`);
}
{ // a listener that throws, and one whose promise is refused
  SAVED.sink = () => { throw new Error('the backend is away'); }; const n = clicks.length;
  let threw = false, went: Went = { went: 'sent', local: true }; try { went = saveFile('session', 'd.json', new Blob(['x'])); } catch { threw = true; }
  if (!BREAK) SAVED.sink = (() => Promise.reject(new Error('the backend is away'))) as any; const went2 = saveFile('session', 'd2.json', new Blob(['x']));
  await new Promise(r => setTimeout(r, 20));
  check(!threw && is(went, false, false) && is(went2, false, false) && clicks.length === n && rejections === 0, 'a listener that throws or whose promise is refused: the caller hears no error, is told the file went nowhere, and no rejection is left unhandled');
}
{ // a download that cannot be asked for
  listen(); const n = told.length, r0 = revoked; failClick = true;
  let threw = false; try { saveFile('level', 'e.json', new Blob(['x'])); } catch { threw = true; } failClick = false;
  noPage(); let threw2 = false; try { saveFile('replay', 'e2.json', new Blob(['x'])); } catch { threw2 = true; }
  let threw3 = false, went: Went = { went: false, local: true }; try { went = saveFile('perf', 'e3.json', new Blob(['x'])); } catch { threw3 = true; } page();
  check(threw && threw2 && told.length === n + 1 && revoked === r0 + 1, 'a download that cannot be asked for throws (the browser refused; no page): nobody is told, and the address made for it is let go');
  check(!BREAK && !threw3 && is(went, 'sent', false), 'a file that is not downloaded needs no page: the listener has it');
}
{ // for the listener alone
  listen(); const n = told.length, c = clicks.length;
  noPage(); const went = tellSaved('bench', 'f.json', new Blob(['{}']), { commit: 'abc' }); page();
  check(went === 'sent' && told.length === n + 1 && told[n].kind === 'bench' && told[n].meta?.commit === 'abc' && clicks.length === c, 'a thing told to the listener alone asks for no download, page or no page, and the listener gets it');
}
{ // the door as it was (?downloads=1)
  listen('held'); const n = told.length, c = clicks.length; SAVE_UP.on = false;
  const went = saveFile('perf', 'g.json', new Blob(['x']), { holdMs: 10 }); SAVE_UP.on = true;
  check(is(went, 'held', true) && downloads('level') && clicks.length === c + 1 && clicks[c].name === 'g.json' && told.length === n + 1 && order[order.length - 1] === `told g.json after ${c + 1} clicks`, 'with downloads put back a take is downloaded and the listener told after it, as it was');
}
{ // how a file ended, heard later
  let end: SavedDone | undefined; const heard: boolean[] = [], hear = (ok: boolean) => { heard.push(ok); };
  SAVED.sink = BREAK ? null : (_f, done) => { end = done; return 'sent'; };
  const w = saveFile('perf', 'h.json', new Blob(['x']), { done: hear }); const before = heard.length; end?.(true); end?.(false);
  check(is(w, 'sent', false) && before === 0 && heard.join() === 'true', `a saver is told once, later, that its file arrived (${heard.join()})`);
  heard.length = 0; SAVED.sink = BREAK ? null : (_f, done) => { done?.(false); return 'held'; }; const early = tellSaved('bench', 'h2.json', new Blob(['x']), undefined, hear); const at0 = heard.length; await new Promise(r => setTimeout(r, 5));
  check(early === 'held' && at0 === 0 && heard.join() === 'false', 'a listener that lets a file go the moment it is handed over: the saver hears it after the answer, not before');
  heard.length = 0; SAVED.sink = (_f, done) => { done?.(true); setTimeout(() => done?.(true), 1); return false; }; tellSaved('bench', 'h3.json', new Blob(['x']), undefined, hear); await new Promise(r => setTimeout(r, 10));
  SAVED.sink = (_f, done) => { end = done; return 'sent'; }; tellSaved('bench', 'h4.json', new Blob(['x']), undefined, () => { throw new Error('the card is gone'); }); let threw = false; try { end?.(true); } catch { threw = true; }
  check(heard.length === 0 && !threw, 'a file that was not taken has no end to hear of, and a saver that throws on hearing troubles nobody');
}
{ // the door over the backend's own file intake, mapped as main.ts maps it (the line is read below)
  let on = true, may = false; const dead = wire('http://127.0.0.1:9', { fetch: (() => Promise.reject(new Error('no backend in this check'))) as any });
  const F = new FileSync(dead, { pid: 'p', sid: 'savefile-check', on: () => on, may: () => may });
  const save = (n: string, b: Blob, k: string, m?: Record<string, string>, d?: SavedDone): 'sent' | 'held' | 'off' | 'no' => { if (!on) return 'off'; const t = F.add(n, b, k, m, d); return t === 'now' ? 'sent' : t ?? 'no'; };   // (cloud.ts `save`)
  SAVED.sink = BREAK ? null : (f, done) => { const took = save(f.name, f.blob, f.kind, f.meta, done); return took === 'sent' || took === 'held' ? took : false; };
  const held = saveFile('perf', 'i.json', new Blob(['{}'])); may = true; const now = saveFile('bench', 'i2.json', new Blob(['{}']));
  const max = FILESYNC.max, raw = FILESYNC.rawMax; FILESYNC.max = FILESYNC.rawMax = 4; const big = saveFile('perf', 'i3.json', new Blob(['123456'])); FILESYNC.max = max; FILESYNC.rawMax = raw;
  on = false; const off = saveFile('perf', 'i4.json', new Blob(['{}'])); await new Promise(r => setTimeout(r, 20));
  check(is(held, 'held', false) && is(now, 'sent', false) && is(big, false, false) && is(off, false, false), `over the backend's intake: a file kept for the notice is held, one going up is being sent, one too large or with sharing off is not sent (${[held, now, big, off].map(w => w.went).join(', ')})`);
}
{ // what a saver says of it
  check(savedWords('The recording', 'sent') === 'The recording is being sent to the developer.' && savedWords('The recording', 'held') === 'The recording will be sent to the developer.' && savedWords('The recording', false) === "The recording wasn't sent."
    && savedWords('Your results', 'sent', true) === 'Your results are being sent to the developer.' && savedWords('Your results', false, true) === "Your results weren't sent."
    && savedEnd('The recording', true) === 'The recording was sent to the developer.' && savedEnd('The checkpoint', false) === "The checkpoint couldn't be sent." && savedWords('The recording', 'arrived') === 'The recording was sent to the developer.' && savedWords('Your results', 'arrived', true) === 'Your results were sent to the developer.', 'the words for where a file went: being sent, will be sent, not sent, and sent');
}
{ // the recorder's takes: a plain one, and a benchmark's
  listen(); told.length = 0; const clicks0 = clicks.length;
  const timer: any = { lastFrame: new Map(), results: new Map(), lastRaw: [] };
  const rec = new PerfRecorder({ timer, settings: {}, renderW: 320, renderH: 180 } as any, { guards: [], saves: { stats: { autosaves: 0, manual: 0, loads: 0 } } } as any, {} as any, { current: 'high', auto: false } as any, () => {});
  const dump = (events: any[], commit?: string): any => ({ version: 1, startedAt: '2026-01-02T03:04:05.000Z', ...(commit ? { commit } : {}), note: '', frames: [], events, settingsAtStart: {}, settingsAtEnd: {} });
  (rec as any).startedAt = '2026-01-02T03:04:05.000Z';
  rec.deliver(dump([{ t: 0, frame: 0, kind: 'note', text: 'x' }])); const first = rec.lastSent; const bytes = told[0] ? await told[0].blob.text() : '';
  let back: any = null; try { back = JSON.parse(bytes); } catch { /* */ }
  check(!!back && back.version === 1 && back.startedAt === '2026-01-02T03:04:05.000Z' && Array.isArray(back.frames) && back.events.length === 1 && back.events[0].text === 'x' && !rec.lastLocal, `what the listener is handed is the take itself, as JSON that reads back (${bytes.length} bytes)`);
  rec.deliver(dump([{ t: 0, frame: 0, kind: 'bench', text: '{"version":1}' }], 'c0ffee'));
  check(told.length === 2 && first === 'sent' && clicks.length === clicks0 && told[0].kind === 'perf' && told[0].name === 'perf-2026-01-02T03-04-05-000Z.json' && !('meta' in told[0]), `a recorder's take is filed as one with no download, and the recorder knows a listener is sending it (${told[0]?.kind} ${told[0]?.name})`);
  check(told[1]?.kind === 'perf' && JSON.stringify(told[1].meta) === '{"commit":"c0ffee","of":"bench"}' && told[1].blob.size > 0, 'a benchmark\'s take says what it is of, and its build, beside it');
  noPage(); let threw = false; try { rec.deliver(dump([])); } catch { threw = true; } const bare = rec.lastSent; page();
  listen('held'); rec.deliver(dump([])); const held = rec.lastSent; SAVED.sink = null; rec.deliver(dump([]));
  SAVE_UP.on = false; let local = false; try { rec.deliver(dump([])); local = rec.lastLocal; } finally { SAVE_UP.on = true; }
  check(!threw && bare === 'sent' && held === 'held' && rec.lastSent === false && local, 'the recorder with no page still reaches the listener; it knows a take that is held, one nobody took, and one the browser was handed');
}
{ // the real recorder hears how its take ended, and whoever asked is told
  const notes: string[] = [], ends: SavedDone[] = []; SAVED.sink = BREAK ? null : (_f, done) => { if (done) ends.push(done); return 'sent'; };
  const timer: any = { lastFrame: new Map(), results: new Map(), lastRaw: [] };
  const rec = new PerfRecorder({ timer, settings: {}, renderW: 320, renderH: 180 } as any, { guards: [], saves: { stats: { autosaves: 0, manual: 0, loads: 0 } } } as any, {} as any, { current: 'high', auto: false } as any, s => { notes.push(s); });
  const dump: any = { version: 1, startedAt: '2026-01-02T03:04:05.000Z', note: '', frames: [], events: [], settingsAtStart: {}, settingsAtEnd: {} };
  (rec as any).startedAt = dump.startedAt;
  rec.deliver(dump); const going = rec.lastSent; ends[0]?.(true);
  check(going === 'sent' && rec.lastSent === 'arrived' && notes.join(' | ') === 'The recording was sent to the developer.', `a take that arrives: the recorder knows, and a note says so (${notes.join(' | ')})`);
  notes.length = 0; const fates: unknown[] = []; rec.deliver(dump, f => { fates.push(f); }); rec.deliver(dump); ends[1]?.(false); ends[2]?.(false);
  check(fates.join() === 'false' && rec.lastSent === false && notes.join(' | ') === "The recording couldn't be sent.", `whoever asked to hear of one take's end hears of that take alone, in place of the note; an older take's end does not move the newest one's word, and a newest take let go is said (${fates.join()} · ${notes.join(' | ')})`);
  SAVED.sink = null;
}
{ // a saved replay, an editor's level
  listen(); told.length = 0;
  const name = await downloadReplay({ savedAt: '2026-01-02T03:04:05.000Z', seconds: 12, notes: [{ t: 1, text: ' ' }, { t: 2, text: 'here' }] } as any);
  editorDownload('my level', '{"boxes":[]}');
  check(told.length === 2 && told[0].kind === 'replay' && told[0].name === name && /^twopointsix-replay-.*-1notes\.json$/.test(name) && told[1].kind === 'level' && told[1].name === 'my_level.json', `a saved replay and an editor's level go out through the door (${told.map(f => `${f.kind} ${f.name}`).join(' · ')})`);
  noPage(); let threw = false, threw2 = false; try { await downloadReplay({ savedAt: '2026-01-02T03:04:05.000Z', seconds: 1, notes: [] } as any); } catch { threw = true; } try { editorDownload('x', '{}'); } catch { threw2 = true; } page();
  check(threw && threw2, 'with no page a replay\'s and a level\'s save fail out loud, as they did: no caller reports a file that never went out');
}
{ // no other door: nothing under src/ makes a download of its own
  const files: string[] = []; const walk = (d: string) => { for (const e of readdirSync(d, { withFileTypes: true })) { const p = join(d, e.name); if (e.isDirectory()) walk(p); else if (/\.ts$/.test(e.name)) files.push(p); } };
  const root = join(import.meta.dir, '../../src'); walk(root);
  const own = files.filter(f => !/host\/savefile\.ts$/.test(f) && /\.download\s*=|createObjectURL\(|setAttribute\(\s*'download'/.test(readFileSync(f, 'utf8'))).map(f => f.slice(root.length + 1));
  const through = ['ui/perfrec.ts', 'ui/sessionrec.ts', 'ui/bench.ts', 'replay/format.ts', 'ui/bindings.ts', 'editor/persist.ts'].filter(f => !/from '\.\.\/host\/savefile'/.test(readFileSync(join(root, f), 'utf8')));
  const sets = files.filter(f => /SAVED\.sink\s*=[^=]/.test(readFileSync(f, 'utf8'))).map(f => f.slice(root.length + 1)), main = readFileSync(join(root, 'main.ts'), 'utf8');
  check(sets.join() === 'main.ts' && /SAVED\.sink = cloud \? \(f, done\) => \{ const took = cloud\.save\(f\.name, f\.blob,[^\n]*f\.meta, done\); return took === 'sent' \|\| took === 'held' \? took : false; \} : null;/.test(main), `one place sets the listener: main.ts, to the backend's file intake, and hands its whole answer back (${sets.join(', ') || 'nowhere'})`);
  check(own.length === 0 && through.length === 0, `no file under src/ makes a download of its own, and the six spots save through the door${own.length ? ` (own: ${own.join(', ')})` : ''}${through.length ? ` (not through it: ${through.join(', ')})` : ''}`);
}
console.log(red ? `\n${red} FAILED` : '\nall pass');
process.exit(red ? 1 : 0);
