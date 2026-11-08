// one page: bun run build.ts → dist/index.html and rooftop.html (open from disk), dist/artifact.html (the same without the document's shell)
import { mkdirSync, readFileSync, writeFileSync } from 'node:fs';
const r = await Bun.build({ entrypoints: ['src/main.ts'], target: 'browser', minify: true, loader: { '.wgsl': 'text' } });
if (!r.success) { console.error(r.logs.join('\n')); process.exit(1); }
const js = (await r.outputs[0]!.text()).replaceAll('</script', '<\\/script'), html = readFileSync('index.html', 'utf8').replace('<script type="module" src="./main.js"></script>', () => `<script type="module">${js}</script>`);
mkdirSync('dist', { recursive: true }); writeFileSync('dist/index.html', html); writeFileSync('rooftop.html', html); writeFileSync('dist/artifact.html', html.replace(/<!doctype html>\s*<html[^>]*><head>/, '').replace('</head><body>', '').replace('</body></html>', '').replace(/<meta charset[^>]*><meta name="viewport"[^>]*>/, '')); console.log('dist/index.html', (html.length / 1024).toFixed(0), 'KB');
