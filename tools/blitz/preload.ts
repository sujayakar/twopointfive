// THE BRIDGE'S PRELOAD, INERT UNLESS A WORD ASKS FOR A GAME THAT NEEDS IT (bunfig.toml `preload`; tools/blitz/PLAY.md). The bridge reads the simulation's own
// module variables through a window that tools/blitz/expose.ts opens as each source file loads, so that file has to be registered before any source file is
// read: a preload. This one is listed for every `bun` started in the repository and does nothing at all (it reads one small file of this folder and no source
// file) unless the harness's word asks for the Rust game or its TypeScript side: then it registers the window, in this process and, by the same list, in every
// child `bun` the tool starts in the repository's root.
import { asked } from './word.ts';
if (asked()) await import('./expose.ts');
