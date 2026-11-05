// THE LOOK'S MAP WORKER AS TEXT (render/lookWorkers.ts) — a function that runs WHEN THE PAGE IS BUNDLED, not in the page: render/lookWorkerText.ts imports it
// `with { type: 'macro' }`, so the bundler calls it and writes what it returns into the bundle as a string. What it returns is render/lookWorkerMain.ts
// bundled by itself (the generator, the game's own maths, the few lines that answer a message) as ONE PLAIN SCRIPT, no module: a worker started from a
// blob as a module is refused on a page opened from disk, and the one-file page is opened from disk. So the workers run the very files the page runs, in
// every build there is (the dev server, dist, the one-file page, a tool under bun), with no second entry point, no file beside the page and nothing
// generated to keep in step. (A macro may not call the bundler in its own process: it runs this bun's `build` as a command, which is the way bun gives.)
// IT NEVER FAILS A BUILD: where the worker does not bundle it hands back no text, the page then makes its maps on its own thread as it always did
// (lookWorkers.ts says so once on the console), and tools/qa/lookworkers-check.ts is red.
declare const Bun: { spawnSync(cmd: string[], o?: object): { success: boolean; stdout: { toString(): string } } };
declare const process: { execPath: string };
export function lookWorkerSource(): string {
  try {
    const entry = (import.meta as unknown as { dir: string }).dir + '/lookWorkerMain.ts';   // (a path as the disk has it: a URL's would have a space as %20)
    const res = Bun.spawnSync([process.execPath, 'build', entry, '--format=iife', '--target=browser'], { stdout: 'pipe', stderr: 'pipe' });
    const text = res.stdout.toString();
    return res.success && /\bonmessage\b/.test(text) ? text : '';
  } catch { return ''; }
}
