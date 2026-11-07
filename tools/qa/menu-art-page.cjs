// THE PLAY PAGE'S PICTURES ON THE REAL PAGE (menuStills.ts, planArt.ts): headless Chromium on the software WebGPU adapter (tools/qa/page-look.cjs's recipe).
//   PORT=5199 node tools/qa/menu-art-page.cjs [outDir]      (a dev server on that port: PORT=5199 bun run tools/serve.ts; ~4 min)
//   cards    the Mission step asks for the key art (assets/menu/*.jpg) and every card of the open group has a picture on it
//   onefile  the same page marked as the one-file build is (meta tps-one-page): nothing under assets/menu is asked for, every card still has a
//            picture (the menu's own drawn plan), and it is not the key art
// exit 0 = all of it, with no page error · 1 = otherwise · 2 = the script died. ONLY=cards|onefile = that part alone. Stills go to outDir when given.
const { chromium } = require('/opt/node-tools/node_modules/playwright');
const fs = require('fs'), path = require('path'), crypto = require('crypto');
const PORT = process.env.PORT || 5199, QUALITY = process.env.QUALITY || 'verylow', OUT = process.argv[2] || '', ONLY = process.env.ONLY || '';
if (ONLY && ONLY !== 'cards' && ONLY !== 'onefile') { console.log(`no part '${ONLY}': ONLY=cards|onefile`); process.exit(2); }
const FLAGS = ['--enable-unsafe-webgpu', '--use-gl=angle', '--use-angle=swiftshader', '--enable-features=Vulkan', '--use-vulkan=swiftshader', '--ignore-gpu-blocklist', '--disable-gpu-watchdog'];
const md5 = s => crypto.createHash('md5').update(s).digest('hex').slice(0, 10);
if (OUT) fs.mkdirSync(OUT, { recursive: true });
let bad = 0;
const check = (ok, what) => { console.log(`  ${ok ? 'PASS' : 'FAIL'}  ${what}`); if (!ok) bad++; };

(async () => {
  const browser = await chromium.launch({ headless: true, args: FLAGS });
  // a page on the Mission step; `one` = marked as the one-file build before any script of the page runs
  const open = async (W, H, one) => {
    const ctx = await browser.newContext({ viewport: { width: W, height: H }, deviceScaleFactor: 1 });
    await ctx.addInitScript(one => {
      addEventListener('DOMContentLoaded', () => { const s = document.createElement('style'); s.textContent = '#fatal { display: none !important; }'; document.head.appendChild(s); });
      if (!one) return;
      const mark = () => { if (!document.head) return false; const m = document.createElement('meta'); m.name = 'tps-one-page'; m.content = '1'; document.head.appendChild(m); return true; };
      if (!mark()) new MutationObserver((_, o) => { if (mark()) o.disconnect(); }).observe(document, { childList: true, subtree: true });
    }, one);
    const page = await ctx.newPage(), asked = [];
    page.on('pageerror', e => { console.log('  PAGE ERROR', String(e).slice(0, 300)); bad++; });
    page.on('request', r => { if (r.url().includes('/assets/menu/')) asked.push(r.url().split('/').pop()); });
    await page.goto(`http://localhost:${PORT}/?paused&quality=${QUALITY}&mute`, { waitUntil: 'domcontentloaded' });
    await page.waitForSelector('#menu .actions button', { timeout: 240000 }); await page.waitForTimeout(700);
    await page.click('#menu .actions button.primary'); await page.waitForSelector('#missions.one.on'); await page.waitForTimeout(3000);
    return { ctx, page, asked };
  };
  const shot = async (page, n) => { if (OUT) await page.screenshot({ path: path.join(OUT, `menu-art-${n}.png`), timeout: 240000 }); };
  // the cards drawn in the open group: [mission id, a hash of its picture, how many different colours are on it]
  const cards = page => page.evaluate(() => [...document.querySelectorAll('#missions .mgrid .mrow:not(.shut)')].map(r => {
    const c = r.querySelector('canvas'); if (!c || !c.width) return [r.getAttribute('data-mission'), '', 0];
    const d = c.getContext('2d').getImageData(0, 0, c.width, c.height).data, seen = new Set();
    for (let i = 0; i < d.length; i += 4 * 37) seen.add((d[i] << 16) | (d[i + 1] << 8) | d[i + 2]);
    return [r.getAttribute('data-mission'), c.toDataURL(), seen.size];
  }));

  let art = null;
  if (!ONLY || ONLY === 'cards' || ONLY === 'onefile') {
    console.log('cards · 1270x790');
    const { ctx, page, asked } = await open(1270, 790, false);
    art = await cards(page); await shot(page, 'cards');
    if (!ONLY || ONLY === 'cards') {
      check(asked.includes('stills.json') && asked.some(f => f.endsWith('.jpg')), `the key art is asked for (${asked.length} files under assets/menu)`);
      check(art.length > 0 && art.every(c => c[2] > 40), `every card of the open group has a picture (${art.length} cards, fewest colours ${Math.min(...art.map(c => c[2]))})`);
    }
    await ctx.close();
  }
  if (!ONLY || ONLY === 'onefile') {
    console.log('onefile · 1270x790');
    const { ctx, page, asked } = await open(1270, 790, true);
    const plan = await cards(page); await shot(page, 'onefile');
    check(await page.evaluate(() => !!document.querySelector('meta[name="tps-one-page"]')), 'the page is marked as the one-file build is');
    check(asked.length === 0, `nothing under assets/menu is asked for (${asked.length})`);
    check(plan.length > 0 && plan.every(c => c[2] > 4), `every card still has a picture (${plan.length} cards, fewest colours ${plan.length ? Math.min(...plan.map(c => c[2])) : 0})`);
    check(!!art && plan.length === art.length && plan.every((c, i) => c[0] === art[i][0] && md5(c[1]) !== md5(art[i][1])), 'the same cards, and none of them is the key art (the drawn plan)');
    await ctx.close();
  }
  await browser.close();
  console.log(bad ? `=== menu-art-page: FAIL (${bad}) ===` : '=== menu-art-page: ALL PASS ===');
  process.exit(bad ? 1 : 0);
})().catch(e => { console.log('DIED', e.message); process.exit(2); });
