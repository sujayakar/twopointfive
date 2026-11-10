// One page: bun run build.ts → dist/index.html (opens from disk in Chrome) and dist/artifact.html (the same
// without the document's shell).
import { mkdirSync, readFileSync, writeFileSync } from 'node:fs';

const here = import.meta.dir;
const result = await Bun.build({
  entrypoints: [`${here}/src/main.ts`],
  target: 'browser',
  minify: process.argv.includes('--minify'),
  loader: { '.wgsl': 'text', '.css': 'text' },
});
if (!result.success) {
  console.error(result.logs.join('\n'));
  process.exit(1);
}
const script = (await result.outputs[0]!.text()).replaceAll('</script', '<\\/script');
const style = readFileSync(`${here}/src/page/style.css`, 'utf8');
const page = readFileSync(`${here}/index.html`, 'utf8')
  .replace('/*STYLE*/', () => style)
  .replace('/*SCRIPT*/', () => script);
mkdirSync(`${here}/dist`, { recursive: true });
writeFileSync(`${here}/dist/index.html`, page);
const inside = page
  .replace(/^[\s\S]*?(<title>)/, '$1')
  .replace('</head>\n<body>', '')
  .replace(/<\/body>\s*<\/html>\s*$/, '');
writeFileSync(`${here}/dist/artifact.html`, inside);
console.log(`dist/index.html ${(page.length / 1024).toFixed(0)} KB`);
