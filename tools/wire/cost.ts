// WHAT A TICK OF THE WIRE COSTS NOW: the page's list, on one mission, measured on the module that is built.   TPS_GAME_LIST=page TPS_GAME=rust bun run tools/wire/cost.ts [address word] [ticks]
process.env.TPS_GAME_LIST ??= 'page';
const { standUp } = await import('../qa/headless.ts');
const q = process.argv[2], N = +(process.argv[3] ?? 600), W: any = await standUp(q ? { search: `?${q}` } as any : {}), H: any = W.game.rustHost;
if (!H) { console.error('cost: no Rust host stands'); process.exit(1); }
for (let t = 0; t < 60; t++) W.step(1 / 60);
const T0 = { ...H.T }, t0 = performance.now(); for (let t = 0; t < N; t++) W.step(1 / 60); const all = (performance.now() - t0) / N, d = (k: string) => (H.T[k] - T0[k]) / N;
console.log(`a tick ${all.toFixed(2)} ms: the Rust step ${d('step').toFixed(2)}, export ${d('exportStep').toFixed(2)}, apply ${d('applyStep').toFixed(2)}, out events ${d('out').toFixed(2)}, the walk ${d('scan').toFixed(2)} (${N} ticks)`);
process.exit(0);
