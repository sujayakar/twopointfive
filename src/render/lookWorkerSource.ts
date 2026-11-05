// THE LOOK'S MAP WORKER AS TEXT (render/lookWorkers.ts) — a function that runs WHEN THE PAGE IS BUNDLED, not in the page: render/lookWorkerText.ts imports it
// `with { type: 'macro' }`, so the bundler calls it and writes what it returns into the bundle as a string. What it returns is render/lookWorkerMain.ts
// bundled by itself (the generator, the game's own maths, the few lines that answer a message). So the workers run the very files the page runs, in every
// build there is (the dev server, dist, the one-file page, a tool under bun), with no second entry point, no file beside the page and nothing generated to
// keep in step. (A macro may not call the bundler in its own process: it runs this bun's `build` as a command, which is the way bun gives.)
declare const Bun: { spawnSync(cmd: string[], o?: object): { success: boolean; stdout: { toString(): string }; stderr: { toString(): string } } };
declare const process: { execPath: string };
export function lookWorkerSource(): string {
  const entry = new URL('./lookWorkerMain.ts', import.meta.url).pathname;
  const res = Bun.spawnSync([process.execPath, 'build', entry, '--format=esm', '--target=browser'], { stdout: 'pipe', stderr: 'pipe' });
  const text = res.stdout.toString();
  if (!res.success || !/\bonmessage\b/.test(text)) throw new Error(`the look's map worker did not bundle (render/lookWorkerSource.ts): ${res.stderr.toString().slice(0, 600)}`);
  return text;
}
