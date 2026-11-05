// the look's map worker, as the text it is started from: written in here when the page is bundled (render/lookWorkerSource.ts, a macro). A module of its own,
// asked for only when a worker is about to be started (render/lookWorkers.ts): a tool under bun that starts none never bundles it
import { lookWorkerSource } from './lookWorkerSource' with { type: 'macro' };
export const LOOK_WORKER_TEXT = lookWorkerSource() as unknown as string;
