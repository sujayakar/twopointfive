// THE BRIDGE'S PRELOAD, INERT UNLESS A WORD ASKS FOR A GAME THAT NEEDS IT (bunfig.toml `preload`; tools/blitz/PLAY.md). The bridge reads the simulation's own
// module variables through a window that tools/blitz/expose.ts opens as each source file loads, so that file has to be registered before any source file is
// read: a preload. This one is listed for every `bun` started in the repository and does nothing at all (it reads one small file of this folder and no source
// file) unless the harness's word asks for the Rust game or its TypeScript side: then it registers the window, in this process and, by the same list, in every
// child `bun` the tool starts in the repository's root.
import { resolve } from 'node:path';
import { asked, top } from './word.ts';
if (asked()) { top(); await import('./expose.ts'); }
// A PRELOAD ASKED FOR ON THE COMMAND LINE STILL RUNS. bun runs the configuration's preload list IN PLACE OF the command line's (`--preload`, also through
// BUN_OPTIONS), not beside it: with this file listed, `bun run --preload ./tools/blitz/expose.ts <tool>` (the compare loop's and the bridge's own tools) would
// open no window and the walk would find no module variable. So the ones the command named are loaded here, in their order (a module loaded twice is one module).
{ const A = process.execArgv, named: string[] = [];
  for (let i = 0; i < A.length; i++) { const m = /^(?:--preload|--require|--import|-r)(?:=(.*))?$/.exec(A[i]); if (!m) continue; const p = m[1] ?? A[++i]; if (p) named.push(p); }
  for (const p of named) await import(resolve(process.cwd(), p)); }
