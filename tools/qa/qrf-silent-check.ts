// GOING LOUD BRINGS THE RESPONSE — the check behind qrf.ts `silentNet` (`tune.qrfSilent`).
// The fault it holds shut (the missions goal's S2): the response team wanted a locked-down floor and only a living man of the shift locks it down, so a player
// who shot all three met nobody — control silent with gunfire in its ears, a minute in an empty building (the team came in 2 of 24 loud runs).
//   bun run tools/qa/qrf-silent-check.ts [--off]
// What it does, STAGED, on one game stood as the page ships it (the rule's edges, each from a fresh encounter; the played loud runs took two CPU minutes a seed):
//   3. QUIET WORK NEVER STARTS IT: all three put out cold with no gunfire heard — thirty seconds on, no radio check and the team idle
//   4. OLD GUNFIRE DOES NOT COUNT: the same with gunfire last heard 200 s before (the trigger's memory is 180) — nothing
//   5. A MAN BACK ON THE NET CANCELS IT: gunfire heard, two out cold, the third in Sam's arm — the radio check comes; let go of before the fourteen seconds,
//      nothing is sent for twenty more; out cold himself, the check comes again and the team is requested fourteen seconds after that
//   --off  the same with the rule off (TPS_QRFSILENT=0): the fault as it was — expected to FAIL (exit 1), the proof the check can
// exit 0 = all hold · 1 = one does not · 2 = bad flags
import { resolve } from 'node:path';

const argv = process.argv.slice(2), flag = (n: string) => { const i = argv.indexOf(n); return i >= 0 ? argv[i + 1] : undefined; };
for (const a of argv) if (a.startsWith('--') && !['--off', '--staged'].includes(a)) { console.error(`unknown flag ${a} (--off)`); process.exit(2); }
const off = argv.includes('--off');

if (argv.includes('--staged')) {   // a child: the staged edges as one JSON line (its own process: the switch is read as the game stands up)
  const { standUp, ROOT } = await import('./headless.ts');
  const { killGuard } = await import(`${ROOT}/src/game/combat.ts`), { startGrab } = await import(`${ROOT}/src/game/player.ts`);
  const log = console.log; console.log = console.error = console.warn = () => { };
  const W: any = await standUp({ uploadEvery: 4 }); const g = W.game; g.godMode = true;
  const run = (secs: number) => { for (let i = 0; i < Math.round(secs * 60); i++) W.step(1 / 60); };
  const what = () => (g.qrf.log as { t: number; what: string }[]).map(l => l.what);
  const out: { name: string; ok: boolean; words: string }[] = [];
  const cold = (gd: any) => killGuard(g, gd, [1, 0, 0], true, 'struck');
  { run(0.5); for (const gd of [...g.guards]) cold(gd); run(30);
    out.push({ name: 'quiet work never starts it', ok: g.qrf.state === 'idle' && !what().includes('radio-check') && g.guards.every((x: any) => x.state === 'dead'), words: `three out cold, no gunfire heard, 30 s on: team ${g.qrf.state}, log [${what().join(', ')}]` }); }
  { g.restartEncounter(); run(0.5); g.qrf.loudT = g.time - 200; for (const gd of [...g.guards]) cold(gd); run(30);
    out.push({ name: 'old gunfire does not count', ok: g.qrf.state === 'idle' && !what().includes('radio-check'), words: `gunfire last heard 200 s before the last man fell, 30 s on: team ${g.qrf.state}, log [${what().join(', ')}]` }); }
  { g.restartEncounter(); run(0.5); const [a, b, c] = g.guards as any[]; g.qrf.loudT = g.time; cold(a); cold(b);
    const yaw = c.char.bodyYaw; g.teleportPlayer([c.char.pos[0] - Math.sin(yaw) * 0.7, 0, c.char.pos[2] - Math.cos(yaw) * 0.7]); g.player.char.bodyYaw = g.player.char.aimYaw = yaw; g.player.holstered = true;
    startGrab(g, c); const grabbed = !!c.held; run(9); const checked = what().filter(x => x === 'radio-check').length, stillIdle = g.qrf.state === 'idle';
    c.held = null; run(20); const afterLetGo = g.qrf.state as string, alive = c.state !== 'dead';   // (let go of at nine seconds: he is back on the net)
    const t0 = g.time; if (alive) cold(c); let askedAt = -1; for (let i = 0; i < 60 * 20 && askedAt < 0; i++) { W.step(1 / 60); if (g.qrf.state !== 'idle') askedAt = g.time - t0; }
    const checks = what().filter(x => x === 'radio-check').length;
    out.push({ name: 'a man back on the net cancels it', ok: grabbed && checked === 1 && stillIdle && afterLetGo === 'idle' && alive && checks === 2 && askedAt > 13.5 && askedAt < 14.5 && g.qrf.reason === 'silent',
      words: `the third man ${grabbed ? 'in his arm' : 'NOT grabbed'}: ${checked} radio check by 9 s, team ${stillIdle ? 'idle' : 'CALLED'}; let go of, 20 s on: team ${afterLetGo}; then out cold: ${checks} radio checks in all, the team ${askedAt < 0 ? 'NEVER requested' : `requested ${askedAt.toFixed(1)} s after (reason ${g.qrf.reason})`}` }); }
  console.log = log; console.log(JSON.stringify(out)); process.exit(0);
}
let failed = false;
{ const pr = Bun.spawn(['bun', 'run', import.meta.path, '--staged'], { stdout: 'pipe', stderr: 'ignore', env: { ...process.env, TPS_QRFSILENT: off ? '0' : '1' } });
  const text = await new Response(pr.stdout).text(); await pr.exited;
  let rows: { name: string; ok: boolean; words: string }[] | null = null; try { rows = JSON.parse(text.trim().split('\n').pop()!); } catch { /* */ }
  if (!rows || rows.length !== 3) { failed = true; console.log('- FAIL — the staged edges did not run'); }
  else for (const x of rows) { if (!x.ok) failed = true; console.log(`- ${x.ok ? 'PASS' : 'FAIL'} — staged, ${x.name}: ${x.words}`); } }
console.log(failed ? `=== qrf-silent-check: FAILED${off ? ' (the rule is off: expected)' : ''} ===` : '=== qrf-silent-check: ALL PASS ===');
process.exit(failed ? 1 : 0);
