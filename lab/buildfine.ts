// the page of the fine rooftop, its shapes baked into it: bun run buildfine.ts → dist/fine-page.html (opens from disk), dist/fine.html (the same without the document's shell), dist/shapes.fine.gz
import { mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import { gzipSync } from 'node:zlib';
import { STUDIES } from './src/lab';
import { makeFine, makeShape } from './src/fine/make';
import { bake } from './src/fine/file';
const t = performance.now(), f = makeFine(STUDIES.cars!()), raw = bake(f.shapes.map(makeShape)), z = gzipSync(raw, { level: 9 }); console.log(f.shapes.length, 'shapes made in', ((performance.now() - t) / 1000).toFixed(1), 's', (raw.length / 1e6).toFixed(1), 'MB,', (z.length / 1e6).toFixed(1), 'squeezed');
const r = await Bun.build({ entrypoints: ['src/finepage.ts'], target: 'browser', minify: true, loader: { '.wgsl': 'text' } }); if (!r.success) { console.error(r.logs.join('\n')); process.exit(1); }
const js = (await r.outputs[0]!.text()).replaceAll('</script', '<\\/script'), html = readFileSync('fine.src.html', 'utf8').replace('<script type="text/plain" id="shapes"></script>', () => `<script type="text/plain" id="shapes">${Buffer.from(z).toString('base64')}</script>`).replace('<script type="module" src="./finepage.js"></script>', () => `<script type="module">${js}</script>`);
mkdirSync('dist', { recursive: true }); writeFileSync('dist/shapes.fine.gz', z); writeFileSync('dist/fine-page.html', html); writeFileSync('dist/fine.html', html.replace(/<!doctype html>\s*<html[^>]*><head>/, '').replace('</head><body>', '').replace('</body></html>', '').replace(/<meta charset[^>]*><meta name="viewport"[^>]*>/, '')); console.log('dist/fine.html', (html.length / 1e6).toFixed(1), 'MB');
