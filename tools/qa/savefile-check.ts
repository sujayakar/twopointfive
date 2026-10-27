// THE ONE DOOR A FILE LEAVES THE PAGE BY (src/host/savefile.ts), on a fake page: the download is asked for once under the file's name; whoever listens is told
// what went out, after it, with the file's kind and facts, and the door answers whether the listener took it; a listener that throws, or whose promise is
// refused, loses nothing; a download that cannot be asked for THROWS (a caller must not mark the file handed over) and nobody is told; a file for the
// listener alone asks for no download; a quiet one tells nobody; the recorder's take and the others go through the door; and no file under src/ makes a
// download of its own (by `.download =` or createObjectURL: the two ways this tree ever did).
//   bun run tools/qa/savefile-check.ts   (exit 0 = all pass; SAVEFILE_BREAK=1 = nobody listens: must fail)
import { readFileSync, readdirSync } from 'node:fs';
import { join } from 'node:path';
import { SAVED, saveFile, tellSaved, type SavedFile } from '../../src/host/savefile';
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
const listen = (takes: boolean | undefined = true) => { SAVED.sink = BREAK ? null : f => { order.push(`told ${f.name} after ${clicks.length} clicks`); told.push(f); return takes; }; };

page();
{ // nobody listens: the build as it was
  SAVED.sink = null; const took = saveFile('perf', 'a.json', new Blob(['{}']));
  check(took === false && clicks.length === 1 && clicks[0].name === 'a.json' && /^blob:fake\//.test(clicks[0].href), 'nobody listening: the download is asked for once, under the file\'s name, and the door says nobody took it');
}
{ // a listener
  listen(); const blob = new Blob(['{"x":1}'], { type: 'application/json' });
  const took = saveFile('perf', 'b.json', blob, { meta: { commit: 'abc', of: undefined } });
  check(took === true && clicks.length === 2 && told.length === 1 && told[0].kind === 'perf' && told[0].name === 'b.json' && told[0].blob === blob && JSON.stringify(told[0].meta) === '{"commit":"abc"}' && order[0] === 'told b.json after 2 clicks', 'a listener is told what went out, after the download was asked for: the kind, the name, the same bytes, the facts that were given');
  saveFile('level', 'c.json', new Blob(['x']), { meta: { commit: undefined } });
  check(told.length === 2 && !('meta' in told[1]), 'a file with no facts carries none');
  listen(false); const no = saveFile('perf', 'c2.json', new Blob(['x'])); listen(undefined); const quietYes = saveFile('perf', 'c3.json', new Blob(['x']));
  check(no === false && quietYes === true && told.length === 4, 'only a listener\'s plain no says the file is not taken: one that answers nothing is taken to be sending it');
}
{ // a listener that throws, and one whose promise is refused
  SAVED.sink = () => { throw new Error('the backend is away'); }; const n = clicks.length;
  let threw = false, took = true; try { took = saveFile('session', 'd.json', new Blob(['x'])); } catch { threw = true; }
  if (!BREAK) SAVED.sink = (() => Promise.reject(new Error('the backend is away'))) as any; const took2 = saveFile('session', 'd2.json', new Blob(['x']));
  await new Promise(r => setTimeout(r, 20));
  check(!threw && took === false && took2 === true && clicks.length === n + 2 && rejections === 0, 'a listener that throws or whose promise is refused loses nothing: both files went out, the caller hears no error and no rejection is left unhandled');
}
{ // a download that cannot be asked for
  listen(); const n = told.length, r0 = revoked; failClick = true;
  let threw = false; try { saveFile('dump', 'e.json', new Blob(['x'])); } catch { threw = true; } failClick = false;
  noPage(); let threw2 = false; try { saveFile('perf', 'e2.json', new Blob(['x'])); } catch { threw2 = true; } page();
  check(threw && threw2 && told.length === n && revoked === r0 + 1, 'a download that cannot be asked for throws (the browser refused; no page): nobody is told, and the address made for it is let go');
}
{ // for the listener alone, and a quiet one
  listen(); const n = told.length, c = clicks.length;
  noPage(); const took = tellSaved('bench', 'f.json', new Blob(['{}']), { commit: 'abc' }); page();
  check(took === true && told.length === n + 1 && told[n].kind === 'bench' && told[n].meta?.commit === 'abc' && clicks.length === c, 'a thing told to the listener alone asks for no download, page or no page, and the listener gets it');
  check(saveFile('dump', 'g.json', new Blob(['x']), { quiet: true, holdMs: 10 }) === false && told.length === n + 1 && clicks.length === c + 1, 'a quiet file goes out and the listener is not told');
}
{ // the recorder's takes: a plain one, and a benchmark's
  listen(); told.length = 0;
  const timer: any = { lastFrame: new Map(), results: new Map(), lastRaw: [] };
  const rec = new PerfRecorder({ timer, settings: {}, renderW: 320, renderH: 180 } as any, { guards: [], saves: { stats: { autosaves: 0, manual: 0, loads: 0 } } } as any, {} as any, { current: 'high', auto: false } as any, () => {});
  const dump = (events: any[], commit?: string): any => ({ version: 1, startedAt: '2026-01-02T03:04:05.000Z', ...(commit ? { commit } : {}), note: '', frames: [], events, settingsAtStart: {}, settingsAtEnd: {} });
  (rec as any).startedAt = '2026-01-02T03:04:05.000Z';
  rec.deliver(dump([{ t: 0, frame: 0, kind: 'note', text: 'x' }])); const first = rec.lastSent;
  rec.deliver(dump([{ t: 0, frame: 0, kind: 'bench', text: '{"version":1}' }], 'c0ffee'));
  check(told.length === 2 && first === true && told[0].kind === 'perf' && told[0].name === 'perf-2026-01-02T03-04-05-000Z.json' && !('meta' in told[0]), `a recorder's take is filed as one, and the recorder knows a listener took it (${told[0]?.kind} ${told[0]?.name})`);
  check(told[1]?.kind === 'perf' && JSON.stringify(told[1].meta) === '{"commit":"c0ffee","of":"bench"}' && told[1].blob.size > 0, 'a benchmark\'s take says what it is of, and its build, beside it');
  noPage(); let threw = false; try { rec.deliver(dump([])); } catch { threw = true; } page();
  check(!threw && rec.lastSent === false, 'the recorder with no page: no throw, as before, and nothing was taken');
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
  const through = ['ui/perfrec.ts', 'ui/sessionrec.ts', 'ui/bench.ts', 'replay/format.ts', 'replay/dumpui.ts', 'ui/bindings.ts', 'editor/persist.ts'].filter(f => !/from '\.\.\/host\/savefile'/.test(readFileSync(join(root, f), 'utf8')));
  check(own.length === 0 && through.length === 0, `no file under src/ makes a download of its own, and the seven spots save through the door${own.length ? ` (own: ${own.join(', ')})` : ''}${through.length ? ` (not through it: ${through.join(', ')})` : ''}`);
}
console.log(red ? `\n${red} FAILED` : '\nall pass');
process.exit(red ? 1 : 0);
