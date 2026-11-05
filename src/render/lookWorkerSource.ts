// THE LOOK'S MAP GENERATOR AS TEXT, FOR ITS WORKERS (render/lookWorkers.ts) — a function that runs WHEN THE PAGE IS BUNDLED, not in the page: it is imported
// `with { type: 'macro' }`, so the bundler calls it and writes what it returns into the bundle as a string. What it returns is render/lookTex.ts as JavaScript
// (the file has no imports: it is the whole generator). So the workers run the very file the page runs, in every build there is (the dev server, dist, the
// one-file page, a tool under bun), with no second entry point, no file beside the page and nothing generated to keep in step.
// IF lookTex.ts EVER IMPORTS A MODULE its text alone will not run: a worker then fails to start, the page makes its maps on its own thread as it did, and
// tools/qa/lookworkers-check.ts is red.
declare const Bun: { file(u: URL): { text(): Promise<string> }; Transpiler: new (o: { loader: string; target: string }) => { transformSync(s: string): string } };
export async function lookWorkerSource(): Promise<string> {
  const ts = await Bun.file(new URL('./lookTex.ts', import.meta.url)).text();
  return new Bun.Transpiler({ loader: 'ts', target: 'browser' }).transformSync(ts);
}
