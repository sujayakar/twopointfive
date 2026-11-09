// the page of meshes: bun run buildyard.ts → yard.html (opens from disk), dist/yard.html (the same without the document's shell)
import { mkdirSync, readFileSync, writeFileSync } from 'node:fs';
const r = await Bun.build({ entrypoints: ['src/meshpage.ts'], target: 'browser', minify: true, loader: { '.wgsl': 'text' } });
if (!r.success) { console.error(r.logs.join('\n')); process.exit(1); }
const js = (await r.outputs[0]!.text()).replaceAll('</script', '<\\/script'), html = readFileSync('yard.src.html', 'utf8').replace('<script type="module" src="./meshpage.js"></script>', () => `<script type="module">${js}</script>`);
mkdirSync('dist', { recursive: true }); writeFileSync('yard.html', html); writeFileSync('dist/yard.html', html.replace(/<!doctype html>\s*<html[^>]*><head>/, '').replace('</head><body>', '').replace('</body></html>', '').replace(/<meta charset[^>]*><meta name="viewport"[^>]*>/, '')); console.log('yard.html', (html.length / 1024).toFixed(0), 'KB');
