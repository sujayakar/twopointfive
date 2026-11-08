// WHAT THE PAGE READS OF THE GAME AND IS NOT SENT. Every field of the wire's table that is not on the page's list is given a read trap on a running page (an
// accessor round the value), the page is played through a walk, and a field the page's own code reads is named with the reader. Such a field holds what
// the start left, whatever the Rust game has done with it since: the fog of war was off on every page for that (its frame's record was on no list).
//   node tools/qa/readtrap-page.cjs [address]     env: PORT (the dev server's, default 5199) · PLAYWRIGHT_MODULE, CHROMIUM · READTRAP_BREAK=<field or type> takes it off the list for the walk: it must be named (fogofwar::FogPicture is what was once missing)
// A row is a FAULT unless one of three holds: the page writes the field itself (its own work: a scratch, a camera's), tools/wire/fields.json `unsent` says why the
// field or its whole type isn't sent (`wire.ts unsent`), or it goes at a start only and is today what it was then: at the walk's end the whole state comes
// over once, and a field whose value that moved is STALE. NOT SEEN: a module's own plain variable, a read inside a value that goes whole, a side table.
// Exit 0 pass · 1 a fault. On the software adapter: nothing is drawn (the walk steps without rendering but for its last frames).
const { chromium } = require(process.env.PLAYWRIGHT_MODULE || 'playwright'), fs = require('fs'), path = require('path');
const PORT = process.env.PORT || 5199, Q = process.argv[2] || 'levelFile=pass.json&roomclear&squad4', BREAK = process.env.READTRAP_BREAK || '', SAY = process.argv.includes('--say');
const LIST = JSON.parse(fs.readFileSync(path.join(__dirname, '../wire/fields.json'), 'utf8')), UNSENT = LIST.unsent ?? {}, START = LIST.atStartOnly, why = q => UNSENT[q] ?? UNSENT[q.slice(0, q.lastIndexOf('.'))];
const FLAGS = ['--enable-unsafe-webgpu', '--use-gl=angle', '--use-angle=swiftshader', '--enable-features=Vulkan', '--use-vulkan=swiftshader', '--ignore-gpu-blocklist', '--disable-gpu-watchdog', '--no-sandbox'];
(async () => { const b = await chromium.launch({ headless: true, ...(process.env.CHROMIUM ? { executablePath: process.env.CHROMIUM } : {}), args: FLAGS }), p = await b.newPage({ viewport: { width: 640, height: 360 } });
  p.on('pageerror', e => console.log('PAGEERROR', String(e).slice(0, 300)));
  await p.goto(`http://localhost:${PORT}/?${Q}&paused&keeppage=0&quality=low&mute`, { timeout: 180000 });
  await p.waitForFunction(() => window.__ready !== undefined, null, { timeout: 600000 }); await p.evaluate(() => window.__ready);
  const R = await p.evaluate(async ({ BREAK }) => { const g = window.__game, BR = window.__rust.bridge(), { SCHEMA, parse, compiledExpr, helpersOf, bookOf } = BR, sent = new Set(BR.lists.step); if (BREAK) for (const q of [...sent]) if (q === BREAK || q.startsWith(BREAK)) sent.delete(q);
    const OURS = /^(host_api|input|knobs)::/, BRIDGE = /applyGame|hostHalf|exportGame|rootWalk|leafBytes|scanRebase|rustToolsWrote|helpersOf|installAsks|knobsToRust|RunTapeRec|JSON\.stringify|\b(leaf|pokes|readField|readStruct|watched\d*)$/;
    const TRAPPED = new Set(), READ = new Map(), WROTE = new Map(), STALE = new Set(), AT = new WeakMap(); let quiet = 0, whole = false, trapped = 0;
    const key = f => f.ts ?? f.expr?.match(/^o\.([A-Za-z_$][\w$]*)$/)?.[1] ?? null;
    const frames = () => (new Error().stack ?? '').split('\n').slice(3, 12).map(l => (l.match(/at (?:Object\.|new )?([\w.$]+)/) ?? [])[1] ?? '?');
    const note = (M, q) => { let e = M.get(q); if (!e) M.set(q, e = { n: 0, by: new Map() }); if (e.n++ > 600 && e.by.size) return; quiet++; const st = frames(); quiet--; if (st.some(x => BRIDGE.test(x))) { e.n--; return; } const at = st.slice(0, 3).join(' < '); e.by.set(at, (e.by.get(at) ?? 0) + 1); };
    const trap = (o, k, q) => { let T = AT.get(o); if (T?.has(k)) return; const d = Object.getOwnPropertyDescriptor(o, k); if (!d || !d.configurable || !('value' in d) || !d.writable) return; if (!T) AT.set(o, T = new Set()); T.add(k); let v = d.value; trapped++; TRAPPED.add(q);
      Object.defineProperty(o, k, { configurable: true, enumerable: d.enumerable, get() { if (!quiet) note(READ, q); return v; }, set(x) { if (whole) { if (!Object.is(v, x) && (typeof x !== 'object' || x === null || JSON.stringify(x)?.slice(0, 4000) !== JSON.stringify(v)?.slice(0, 4000))) STALE.add(q); } else if (!quiet) note(WROTE, q); v = x; } }); };
    // (one object may be two records: the fog's picture and the team's pass are both `fogOfWar`)
    const visit = (path, o, B, seen) => { if (!o || typeof o !== 'object') return; const was = seen.get(o); if (was?.has(path)) return; if (was) was.add(path); else seen.set(o, new Set([path])); const T = SCHEMA.types[path]; if (T?.k !== 'struct') return;
      for (const f of T.fields) { const q = `${T.mod}::${T.name}.${f.rust}`; if (f.echo || OURS.test(q)) continue; const N = parse(f.ty), k = key(f), st = N.k === 'type' && SCHEMA.types[N.path]?.k === 'struct';
        if (st) { let v; try { v = k ? o[k] : f.expr === 'o' ? o : f.expr ? compiledExpr(f.expr)(o, g, B) : undefined; } catch { v = undefined; } visit(N.path, v, B, seen); continue; }
        if (k && !sent.has(q)) trap(o, k, q); } };
    const take = () => { quiet++; try { const B = helpersOf(g, BR.knobs), bk = bookOf(g), seen = new Map(); visit(SCHEMA.root, g, B, seen); for (const t of SCHEMA.tables) { const N = parse(t.ty); if (N.k === 'type') for (const r of bk.listOf(t.id)) visit(N.path, r, B, seen); } } finally { quiet--; } };
    const run = async (n, keys = [], draw = 'none') => { for (let i = 0; i < n; i += 30) { take(); await window.__runFrames(Math.min(30, n - i), 1 / 60, keys, draw); for (const k of keys) window.__input.keys.delete(k); } };
    const tap = async k => { window.__input.pressed.add(k); await run(4); };
    g.godMode = true; window.__testVisible = true; await window.__runFrames(4, 1 / 60, [], 'none'); await run(60); READ.clear(); WROTE.clear();
    const said = []; const doing = async (what, fn) => { try { await fn(); } catch (e) { said.push(`${what}: ${String(e).slice(0, 160)}`); } };
    await doing('walk', async () => { for (const k of ['KeyW', 'KeyD', 'KeyS', 'KeyA']) await run(90, [k]); });
    await doing('keys', async () => { for (const k of ['KeyE', 'KeyQ', 'Digit1', 'Digit2', 'Digit3', 'KeyR', 'KeyN', 'KeyC', 'KeyV', 'KeyG', 'KeyF', 'Tab', 'Tab', 'KeyX', 'KeyZ', 'Space', 'KeyB', 'KeyL', 'KeyH', 'KeyM', 'Backslash', 'Backquote', 'Backquote', 'Backslash', 'Escape', 'Escape']) { await tap(k); await run(12); } });
    await doing('rings', async () => { for (const k of ['KeyQ', 'KeyE']) { await run(40, [k], true); await run(10); } });
    await doing('fire', async () => { window.__input.buttons = 1; await run(60); window.__input.buttons = 0; window.__input.buttons = 2; await run(40); window.__input.buttons = 0; });
    await doing('breach', async () => { const C = g.wallCharge; if (!C) return; window.__rust.call('WallCharge.setCarried', C, [2]); await run(4); });
    await doing('fight', async () => { for (const h of g.guards.filter(x => x.team !== 'operative').slice(0, 3)) { window.__rust.call('Game.teleportPlayer', g, [[h.char.pos[0] + 3, 0, h.char.pos[2]]]); await run(60); window.__input.buttons = 1; await run(30); window.__input.buttons = 0; await run(60); } });
    await doing('draw', async () => { await run(20, [], true); });
    const before = new Set(READ.keys()); quiet++; whole = true; try { window.__rust.whole(); } catch (e) { said.push(`whole: ${String(e).slice(0, 160)}`); } whole = false; quiet--;
    const rows = []; for (const q of before) { const e = READ.get(q), w = WROTE.get(q); if (!e.by.size) continue; rows.push({ q, n: e.n, by: [...e.by].sort((a, b) => b[1] - a[1]).slice(0, 4).map(x => x[0]), wrote: w ? [...w.by.keys()].slice(0, 2) : [], stale: STALE.has(q) }); }
    return { rows, trapped, names: TRAPPED.size, said, stale: STALE.size, ticks: g.time }; }, { BREAK });
  for (const s of R.said) console.log(`note ${s}`);
  let bad = 0; const rows = R.rows.sort((a, b) => a.q < b.q ? -1 : 1);
  for (const r of rows) { const w = why(r.q), own = r.wrote.length > 0, start = START.includes(r.q) && !r.stale, fault = !w && !own && !start; if (fault) bad++; if (fault || SAY) console.log(`${fault ? 'FAULT' : w ? 'said ' : own ? 'own  ' : 'start'} ${r.q}${r.stale ? ' STALE' : ''}\t${r.n}\t${r.by.join(' | ')}${own ? `\twritten by ${r.wrote.join(' | ')}` : ''}${w ? `\t${w}` : ''}`); }
  console.log(`=== readtrap-page: ${R.names} fields trapped on ${R.trapped} objects, ${rows.length} read and not sent, ${bad} with no reason${bad ? '' : ', all pass'} (${R.stale} moved by the whole state at the end) ===`);
  await b.close(); process.exit(bad || !R.trapped ? 1 : 0); })().catch(e => { console.error(String(e).slice(0, 1500)); process.exit(1); });
