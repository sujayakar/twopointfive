// ONE BROWSER START: what the page scripts share. Headless Chromium through Playwright, by default on the software WebGPU adapter, a window on the page of the dev server at PORT,
// its errors kept, PASS / FAIL lines counted, and the last line and the exit code. tools/qa/pages.ts runs the scripts and hands them PORT, PLAYWRIGHT_MODULE, CHROMIUM and CHROME.
//   const P = page({ name: 'kitpaste-page', gpu: false }), { check, note } = P;
//   const W = await P.open({ size: [1440, 900], init: [[fn, arg]], errors: { skip: /re/ } });   // W: { browser, context, page, errors, go(query, options), ready(ms), step(n, keys, draw) }
//   check(ok, 'words', detail); await P.end();                                                  // closes the browsers, says the last line, exits 1 on a FAIL and 0 without one
// A window's page keeps Playwright's default timeout unless `timeout` asks. Playwright's clicks hang on the software adapter: a script clicks by script.
const fs = require('node:fs');

/** the flags that put the page on the software WebGPU adapter. (Playwright turns the browser's sandbox off by itself.) */
const SOFTWARE = ['--enable-unsafe-webgpu', '--use-gl=angle', '--use-angle=swiftshader', '--enable-features=Vulkan', '--use-vulkan=swiftshader', '--ignore-gpu-blocklist', '--disable-gpu-watchdog'];
/** no adapter at all: the start page is plain DOM and live from the first tick */
const NO_GPU = ['--disable-gpu'];
const PORT = process.env.PORT || 5199;
/** the software adapter's own lines about the smoke kernels, which it cannot make */
const NOISE = /""?smoke[A-Za-z0-9_]*""?|\[pipeline\] smoke_|number of storage textures \(5\) in the Compute stage/;
/** a console line from the browser's own ask for a favicon, or from the old Vite address */
const NOT_THE_PAGE = /\/favicon\.ico$|^http:\/\/localhost:5174\//;
const T0 = Date.now();
/** a line with the seconds since the script started */
const note = (...a) => console.log(`[${((Date.now() - T0) / 1000).toFixed(0)}s]`, ...a);

function playwright() {
  for (const at of [process.env.PLAYWRIGHT_MODULE, 'playwright', '/opt/node-tools/node_modules/playwright']) { if (!at) continue; try { return require(at); } catch { /* the next */ } }
  throw new Error('no Playwright: set PLAYWRIGHT_MODULE or NODE_PATH');
}
const browserFile = () => process.env.CHROMIUM || process.env.CHROME || (fs.existsSync('/usr/bin/chromium') ? '/usr/bin/chromium' : '');
/** where the browser comes from, as launch options: a file on this machine, or the Chromium Playwright installed */
const browserAt = () => { const file = browserFile(); return file ? { executablePath: file } : { channel: 'chromium' }; };
const launch = ({ flags = SOFTWARE, ...more } = {}) => playwright().chromium.launch({ headless: true, ...browserAt(), args: flags, ...more });

/** a page's errors, kept in `into`. skip: a console line that does not count (a pattern or a list of them) · skipAt: nor one from an address like this · thrown: a thrown error that does not count · console: false = only what is thrown · also: a console line of any type that counts · warnings: a warning counts · as: a word in front · say: each is noted as it comes */
function watch(page, { into = [], skip, skipAt, thrown, console: heard = true, also, warnings = false, as = '', say = false } = {}) {
  const keep = (text, what) => { into.push(as + text); if (say) note(what, text.slice(0, 200)); };
  page.on('pageerror', x => { const s = String(x && x.stack || x); if (!thrown || !thrown.test(s)) keep(s.slice(0, 400), 'PAGE ERROR'); });
  if (heard) page.on('console', m => {
    const t = m.text(), type = m.type(), at = m.location().url || ''; if (!(type === 'error' || (warnings && type === 'warning') || (also && also.test(t))) || [].concat(skip ?? []).some(r => r.test(t)) || (skipAt && skipAt.test(at))) return;
    keep(/Failed to load resource/.test(t) ? `${t} ${at}` : t.slice(0, 300), 'console error');   // (the address that failed is named)
  });
  return into;
}

/** in a context's every page, before its scripts: the red card stays out of the way of a page that has no adapter */
const FATAL_OFF = () => addEventListener('DOMContentLoaded', () => { const s = document.createElement('style'); s.textContent = '#fatal { display: none !important; }'; document.head.appendChild(s); });

/** a page under the software adapter can be minutes shutting: the run does not wait on it past half a minute */
const late = p => Promise.race([p.catch(() => { }), new Promise(r => setTimeout(r, 30000))]);

/** the script's run. name: for the last line · detail: how a check's third word is written ('line' under it, 'json' behind it, 'words' behind it in brackets, 'fail' behind it on a FAIL alone) · flags: the browser's · gpu: false = no adapter · port: another server's */
function page({ name = 'page', detail = 'line', gpu = true, flags = gpu ? SOFTWARE : NO_GPU, port = PORT } = {}) {
  const browsers = []; let first = null;
  const P = {
    fails: 0, lines: [], note, port,
    check(ok, words, got) {
      P.lines.push({ ok: !!ok, label: words, detail: got }); if (!ok) P.fails++;
      const tail = detail === 'line' ? (got === undefined || got === '' ? '' : `\n    ${got}`) : detail === 'json' ? (got === undefined ? '' : `   (${JSON.stringify(got)})`)
        : detail === 'words' ? (got ? ` (${got})` : '') : ok || got === undefined ? '' : `   (got ${JSON.stringify(got)})`;
      console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${words}${tail}`);
    },
    url: (query = '', at = port) => `http://localhost:${at}/?${query}`,
    /** a browser of its own; `end` closes what is still open */
    async launch(options = {}) { const b = await launch({ flags, ...options }); browsers.push(b); return b; },
    /** a context. browser: the run's first if not named · size · dpr · options: the rest of newContext */
    async context({ browser, size = [1280, 720], dpr = 1, options = {} } = {}) { const b = browser || (first ??= await P.launch()); return b.newContext({ viewport: { width: size[0], height: size[1] }, deviceScaleFactor: dpr, ...options }); },
    /** a window: a page in `context`, or in a new context on what `P.context` takes. init: functions or [function, argument], run first in every page of the context · fatalOff · errors: what `watch` takes · timeout: the page's default · port */
    async open({ context, init = [], fatalOff = false, errors = {}, timeout, port: at = port, ...where } = {}) {
      context ??= await P.context(where);
      if (fatalOff) await context.addInitScript(FATAL_OFF);
      for (const s of init) await (Array.isArray(s) ? context.addInitScript(s[0], s[1]) : context.addInitScript(s));
      const w = await context.newPage(); if (timeout !== undefined) w.setDefaultTimeout(timeout);
      const seen = watch(w, errors);
      return { browser: context.browser(), context, page: w, errors: seen,
        go: (query, how) => w.goto(P.url(query, at), how),
        ready: async (ms = 1500000) => { await w.waitForFunction(() => !!window.__ready, null, { timeout: ms }); await w.evaluate(async () => { await window.__ready; }); },
        step: (n, keys = [], draw = 'last') => w.evaluate(([n, keys, draw]) => window.__draw.run(n, keys, draw), [n, keys, draw]) };
    },
    /** every browser closed */
    async close() { await late(Promise.all(browsers.map(b => b.close().catch(() => { })))); browsers.length = 0; first = null; },
    /** the last line (no word: `=== name: ALL PASS ===` or `=== name: N FAILED ===`; '' none), and out with 1 on a FAIL and 0 without one, or with `code` */
    async end(line, code) {
      await P.close();
      const said = line !== undefined ? line : P.fails ? `=== ${name}: ${P.fails} FAILED ===` : `=== ${name}: ALL PASS ===`;
      if (said) console.log(said);
      process.exit(code ?? (P.fails ? 1 : 0));
    },
  };
  P.check = P.check.bind(P);
  return P;
}

module.exports = { page, launch, playwright, browserAt, browserFile, note, late, SOFTWARE, NO_GPU, NOISE, NOT_THE_PAGE, PORT };
