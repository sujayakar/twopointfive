// THE SAVE DOOR'S LISTENER (host/savefile.ts), on whichever page has a backend: a file the page saves goes to the backend's file intake (cloud/files.ts: it returns at once, never throws, and
// sends only where this browser sends) and nowhere else, so its whole answer goes back to whoever saved the file: going up now, held until the notice has been read, or not going
// (the switch off, too large), and later whether it arrived (`done`). The file's short facts go beside it. A benchmark's take goes up under a kind of its own, so it
// can be told from a plain recording. (`off` and `no` both read as not sent: the saver's words are the same.)
import { SAVED } from '../host/savefile';
import type { Cloud } from './cloud';

export function doorTo(cloud: Cloud | null): void {
  SAVED.sink = cloud ? (f, done) => { const took = cloud.save(f.name, f.blob, f.kind === 'perf' && f.meta?.of === 'bench' ? 'benchperf' : f.kind, f.meta, done); return took === 'sent' || took === 'held' ? took : false; } : null;
}
