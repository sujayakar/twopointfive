// THE ONE DOOR A FILE LEAVES THE PAGE BY (src/host/savefile.ts), on a fake page. A kind that has given its download up (a take, a result, a journal, an exported checkpoint) goes to
// whoever listens and to nobody else: no download is asked for, with a listener or without, and the door answers what became of the file ('sent', 'held',
// false) so its saver can say so; `?downloads=1` (SAVE_UP.on = false) is the door as it was. The one kind still downloaded (the editor's level) is asked for once under the file's name and the listener told after it; a download that cannot be asked for THROWS (a
// caller must not mark the file handed over) and nobody is told. A listener that throws, or whose promise is refused, loses nothing. A saver hears
// once, later, how a file that was taken ended. The door over the backend's real file intake, mapped as main.ts maps it. The words a saver says; every kind on the door's own list is saved through it by some file; and no file under src/ makes a download of its own (by `.download =` or createObjectURL; an address whose every use in its module is a worker's is a script's and no file). A replay is no kind of this door's: its pages send one themselves, and nothing of them saves through here.
//   bun run tools/qa/savefile-check.ts   (exit 0 = all pass; SAVEFILE_BREAK=1 = nobody listens: must fail)
import { readFileSync, readdirSync } from 'node:fs';
import { join } from 'node:path';
import { SAVED, SAVED_KINDS, SAVE_UP, saveFile, tellSaved, downloads, wanted, savedWords, savedEnd, type Saved, type SavedDone, type SavedFile, type Went } from '../../src/host/savefile';
import { FileSync, FILESYNC } from '../../src/cloud/files';
import { wire } from '../../src/cloud/wire';
const is = (w: Went, went: Saved, local: boolean) => w.went === went && w.local === local;
import { PerfRecorder } from '../../src/ui/perfrec';
import { download as editorDownload } from '../../src/editor/persist';
import { replayBlob } from '../../src/replay/format';
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
  check(SAVED_KINDS.filter(k => downloads(k)).join() === 'level' && SAVE_UP.local.join() === 'level' && SAVE_UP.on, `of the door's kinds the editor's level is the one still downloaded; every other has given its download up (${SAVED_KINDS.map(k => `${k}${downloads(k) ? ': downloaded' : ''}`).join(', ')})`);
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
  noPage(); let threw2 = false; try { saveFile('level', 'e2.json', new Blob(['x'])); } catch { threw2 = true; }
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
{ // an editor's level, the one file still downloaded
  listen(); told.length = 0; const n = clicks.length;
  editorDownload('my level', '{"boxes":[]}');
  check(told.length === 1 && told[0].kind === 'level' && told[0].name === 'my_level.json' && clicks.length === n + 1, `an editor's level goes out through the door, downloaded once (${told.map(f => `${f.kind} ${f.name}`).join(' · ')})`);
  noPage(); let threw = false; try { editorDownload('x', '{}'); } catch { threw = true; } page();
  check(threw, 'with no page a level\'s save fails out loud, as it did: no caller reports a file that never went out');
}
{ // no other door: nothing under src/ makes a download of its own
  const files: string[] = []; const walk = (d: string) => { for (const e of readdirSync(d, { withFileTypes: true })) { const p = join(d, e.name); if (e.isDirectory()) walk(p); else if (/\.ts$/.test(e.name)) files.push(p); } };
  const root = join(import.meta.dir, '../../src'); walk(root);
  const texts = new Map(files.map(f => [f.slice(root.length + 1), readFileSync(f, 'utf8')] as const));
  // (WHAT AN ADDRESS IS USED FOR SAYS WHAT IT IS. An address made with createObjectURL is a script's, and no file, when EVERY use of it in the module that
  //  makes it is one of a worker's: it is made inside `new Worker(…)`, or kept under one name that is only ever handed to a worker, let go
  //  (revokeObjectURL), emptied, or asked whether it is there. Any other use of that name, or an address made any other way, is a file leaving the page.
  //  So the rule is a list of what a worker needs, closed, and not a list of the ways out of a page, which has no end; it names no file and reads no
  //  Blob (render/lookWorkers.ts is the one module it lets by today). A download's own spellings are red wherever they stand. WHAT IT CANNOT SEE: a
  //  download with no address of the page's making and none of those spellings (a `data:` link written as an object's `download` key))
  const DOWNLOAD = /\.download\s*=|setAttribute\(\s*['"`]download['"`]|\[\s*['"`]download['"`]\s*\]\s*=/;
  const scriptOnly = (text: string): boolean => {
    let rest = text.replace(/new\s+(?:Shared)?Worker\(\s*URL\.createObjectURL\(/g, 'new Worker((');
    for (const n of new Set([...text.matchAll(/([A-Za-z_$][\w$]*)\s*=\s*URL\.createObjectURL\(/g)].map(m => m[1]))) {
      const at = `(?:[\\w$]+\\.)*${n.replace(/\$/g, '\\$')}`, worker = new RegExp(`new\\s+(?:Shared)?Worker\\(\\s*${at}\\s*[,)]`, 'g');
      if (!worker.test(text)) return false;   // (kept, and never handed to a worker)
      for (const use of [worker, new RegExp(`${at}\\s*=\\s*URL\\.createObjectURL\\(`, 'g'), new RegExp(`revokeObjectURL\\(\\s*${at}\\s*\\)`, 'g'), new RegExp(`${at}\\s*=\\s*''`, 'g'), new RegExp(`\\bif\\s*\\(\\s*${at}\\s*\\)`, 'g')]) rest = rest.replace(use, ' ');
      if (new RegExp(`(?<![\\w$])${n.replace(/\$/g, '\\$')}(?![\\w$])`).test(rest)) return false;   // (the name is used some other way)
    }
    return !/createObjectURL\(/.test(rest);
  };
  const ownDownload = (text: string): boolean => DOWNLOAD.test(text) || (/createObjectURL\(/.test(text) && !scriptOnly(text));
  { const made = "const u = URL.createObjectURL(new Blob([text], { type: 'text/javascript' }));", worker = `${made} const w = new Worker(u);`;
    const tries: [string, string, boolean][] = [
      ['a worker handed its script by a kept name', "class P { private url = ''; open() { this.url = URL.createObjectURL(b); const w = new Worker(this.url, { name: 'x' }); } close() { if (this.url) { URL.revokeObjectURL(this.url); this.url = ''; } } }", false],
      ['a worker handed its script as it is made', "const w = new Worker(URL.createObjectURL(new Blob([t[0]], { type: \"text/javascript\" })));", false], ['a worker handed its script', worker, false],
      ['an address handed to nothing', made, true], ['an address made and handed on, beside a worker', `${worker} export const addressOf = (b: Blob) => URL.createObjectURL(b);`, true],
      ['a link beside a worker', `${worker} a.href = u;`, true], ['a window beside a worker', `${worker} open(u, '_blank');`, true], ['a frame beside a worker', `${worker} f.setAttribute('src', u);`, true],
      ['a frame written out beside a worker', `${worker} el.innerHTML = \`<iframe src="\${u}">\`;`, true], ['the address given away beside a worker', `${worker} export const where = () => u;`, true],
      ['a worker only in a note', `// new Worker(u)\n${made} window.open(u);`, true],
      ['a download', "a.download = name;", true], ['a download by its attribute', "a.setAttribute('download', name);", true], ['a download by its attribute, the other quotes', 'a.setAttribute("download", name);', true], ['a download by its key', "a['download'] = name;", true],
      ['a download beside a worker', `${worker} a.download = 'x.js';`, true]];
    const wrong = tries.filter(([, text, want]) => ownDownload(text) !== want).map(([what]) => what);
    check(wrong.length === 0, `an address is a script's, and no file, only where every use of it in its module is a worker's; every other address, and every download, is a file leaving the page${wrong.length ? ` (read wrongly: ${wrong.join(' · ')})` : ''}`); }
  const own = [...texts].filter(([f, text]) => !/host\/savefile\.ts$/.test(f) && ownDownload(text)).map(([f]) => f);
  // (a replay is not the door's: no file with replay in its path imports it, and the door has no kind to save one under. The two patterns are tried on
  // the lines they are for first, so that a pattern that can match nothing does not read as a pass)
  const DOOR = /from\s*['"][^'"]*host\/savefile['"]/;
  const replays = [...texts].filter(([f, text]) => /replay/i.test(f) && DOOR.test(text)).map(([f]) => f);
  check(DOOR.test("import { saveFile } from '../host/savefile';") && SAVED_KINDS.includes('level') && replays.length === 0 && !SAVED_KINDS.some(k => /replay/.test(k)) && !SAVE_UP.local.includes('replay' as any),
    `a replay does not leave the page by this door: no replay file imports it, and it has no such kind (${SAVED_KINDS.join(', ')})${replays.length ? ` (${replays.join(', ')})` : ''}`);
  // … and the file F7 sends is still the replay with its blank notes left out, named by the notes it keeps (replay/format.ts `replayBlob`)
  { const { name, blob } = replayBlob({ savedAt: '2026-01-02T03:04:05.000Z', seconds: 12, notes: [{ t: 1, text: ' ' }, { t: 2, text: 'here' }] } as any), back = JSON.parse(await blob.text());
    check(/^twopointsix-replay-.*-12s-1notes\.json$/.test(name) && back.notes.length === 1 && back.notes[0].text === 'here' && back.seconds === 12, `a replay's file leaves out a note with no words and is named by the notes it keeps (${name})`); }
  // EVERY KIND OF THE DOOR'S IS SAVED THROUGH IT, BY THE DOOR'S OWN LIST (host/savefile.ts SAVED_KINDS; this was five file names written by hand while
  // the door grew to eight kinds): each kind has a file that hands it to the door (saveFile or tellSaved), and no save's kind is one the lint cannot
  // read. A kind is read where it is said outright, or by a const the file declares once as a SavedKind and writes no other thing under; a save by
  // another name for the door, in a file that does not import it, or with any other first word is NOT READ, and red. Notes are left out (a save
  // spoken of in a note saves nothing). A kind nothing saves is a saver that left the door, or a kind to take off its list
  const code = (text: string): string => text.replace(/\/\*[\s\S]*?\*\//g, ' ').replace(/\/\/[^\n]*/g, ' ');
  const savedBy = (raw: string): { kinds: string[]; unread: number } => { const text = code(raw), kinds: string[] = []; let unread = /\b(?:saveFile|tellSaved)\s+as\s+[\w$]/.test(text) ? 1 : 0;
    for (const m of text.matchAll(/(?<![\w$.])(?:saveFile|tellSaved)\(\s*([^,)]*)/g)) { const a = m[1].trim(), said = /^'([^']+)'$/.exec(a);
      if (said) { kinds.push(said[1]); continue; }
      const held = /^[A-Za-z_$][\w$]*$/.test(a) ? [...text.matchAll(new RegExp(`\\bconst\\s+${a}\\s*:\\s*SavedKind\\s*=\\s*'([^']+)'`, 'g'))] : [];
      if (held.length === 1 && [...text.matchAll(new RegExp(`(?<![\\w$.])${a}\\s*[:=](?!=)`, 'g'))].length === 1) kinds.push(held[0][1]); else unread++; }
    return { kinds, unread }; };
  const saversOf = (list: readonly string[], of: ReadonlyMap<string, string>) => { const savers = new Map<string, string[]>(list.map(k => [k, []])), unread: string[] = [], strange: string[] = [];
    for (const [name, text] of of) { if (/host\/savefile\.ts$/.test(name)) continue; const r = savedBy(text), in_ = DOOR.test(text) || (/^host\//.test(name) && /from\s*['"]\.\/savefile['"]/.test(text)); if (r.unread || ((r.kinds.length > 0) && !in_)) unread.push(name);
      for (const k of r.kinds) { const l = savers.get(k); if (!l) strange.push(`${name}: ${k}`); else if (!l.includes(name)) l.push(name); } }
    return { savers, unread, strange, unsaved: list.filter(k => !savers.get(k)!.length) }; };
  { const door = "import { saveFile, tellSaved } from '../host/savefile';\n", t = saversOf(['perf', 'bench', 'net-log', 'ninth'], new Map([
      ['a.ts', `${door}const w = saveFile('perf', name, body);`], ['b.ts', `${door}// (it used to leave by saveFile('bench', n, b))`], ['c.ts', `${door}tellSaved('net-log', n, b); saveFile('odd', n, b);`],
      ['d.ts', `${door}saveFile(kindOf(x), n, b);`], ['e.ts', `${door}const kind: SavedKind = 'ninth'; export function put(kind: SavedKind) { return saveFile(kind, n, b); }`], ['f.ts', "saveFile('ninth', n, b);"],
      ['g.ts', "import { saveFile as put } from '../host/savefile'; put(k, n, b);"], ['h.ts', `${door}saveFile("ninth", n, b); saveFile((k as SavedKind), n, b);`], ['host/savefile.ts', "saveFile('bench', n, b);"]]));
    const k = savedBy(`${door}const KIND: SavedKind = 'modelerr'; saveFile(KIND, n, b);`);
    check(t.unsaved.join() === 'bench' && t.strange.join() === 'c.ts: odd' && t.unread.join() === 'd.ts,e.ts,f.ts,g.ts,h.ts' && t.savers.get('perf')!.join() === 'a.ts' && t.savers.get('net-log')!.join() === 'c.ts' && t.savers.get('ninth')!.join() === 'f.ts' && k.kinds.join() === 'modelerr' && k.unread === 0,
      `the walk on lines of its own: a kind only a note speaks of has nobody, a kind off the list is strange, and a save whose kind is not said outright or by one const is not read (nobody: ${t.unsaved.join()} · strange: ${t.strange.join()} · not read: ${t.unread.join()})`); }
  const { savers, unread, strange, unsaved } = saversOf(SAVED_KINDS, texts);
  const sets = files.filter(f => /SAVED\.sink\s*=[^=]/.test(readFileSync(f, 'utf8'))).map(f => f.slice(root.length + 1)), main = readFileSync(join(root, 'main.ts'), 'utf8');
  check(sets.join() === 'main.ts' && /SAVED\.sink = cloud \? \(f, done\) => \{ const took = cloud\.save\(f\.name, f\.blob,[^\n]*f\.meta, done\); return took === 'sent' \|\| took === 'held' \? took : false; \} : null;/.test(main), `one place sets the listener: main.ts, to the backend's file intake, and hands its whole answer back (${sets.join(', ') || 'nowhere'})`);
  // the debug panel's checkpoint export (ui/bindings.ts): its line is the door's answer in the shared words, with a second line for the file's end, and
  // the old "exported <name>" stands only where the browser was handed the file
  { const row = readFileSync(join(root, 'ui/bindings.ts'), 'utf8').split("text: 'export chosen'")[1]?.split("text: 'import…'")[0] ?? '';
    check(/saveFile\('checkpoint', name, [^\n]*\{ done: arrived => game\.msg\(savedEnd\('The checkpoint', arrived\)\) \}/.test(row) && /if \(w\.local\) game\.msg\(`exported /.test(row) && /game\.msg\(savedWords\('The checkpoint', w\.went\)\)/.test(row) && !/title: '[^']*\bdownload the\b/.test(row),
      'the checkpoint export says what became of its file in the shared words (sent, held, not sent, and its end), and says "exported" only of a file the browser was handed'); }
  check(own.length === 0, `no file under src/ makes a download of its own${own.length ? ` (own: ${own.join(', ')})` : ''}`);
  check(SAVED_KINDS.length > 1 && unsaved.length === 0 && unread.length === 0 && strange.length === 0, `every kind of the door's is saved through it, by the door's own list (${SAVED_KINDS.map(k => `${k}: ${savers.get(k)!.join(' + ') || 'NOBODY'}`).join(' · ')})${unread.length ? ` (a save whose kind is not read: ${unread.join(', ')})` : ''}${strange.length ? ` (a kind the door does not list: ${strange.join(', ')})` : ''}`);
}
console.log(red ? `\n${red} FAILED` : '\nall pass');
process.exit(red ? 1 : 0);
