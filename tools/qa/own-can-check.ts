// A BANG IS ITS OWN SIDE'S CAN TO EVERY TEAMMATE (doortools.ts DOORTOOLS.ownCan; throwables.ts Item.side; combat.ts detonateStun).
//   bun run tools/qa/own-can-check.ts [--scn barracks-four] [--seed 1]        exit 0 = every line PASS, 1 = a FAIL line
// Headless, on the door rules' stand. The door tools take an entry's own men — the thrower among them — off the guard list for the bang, so detonateStun,
// looking the thrower up on that list, used to take the can for a guard's: a teammate still on the list got a defender's count and fold from his own side's can.
//   rule    detonateStun asked directly, the thrower off the list as bang() leaves him: a teammate 4 m off with a line, one 1.5 m off, a defender 4 m off —
//           with the can unsigned (the fault) and signed 'operative' (the mend). The defender's count must not move
//           (and the wrong answer's other half: a man of team 'guard' — a quiet mission's staff — was taken for the can's own side and spared)
//   OWNCAN_BREAK=1 = the switch held off through the 'on' halves: the check must FAIL
export {};   // (a module: the top-level awaits below need it for the tools/qa typecheck)
for (const k of ['QA_DOORMANNERS', 'QA_DOORSTAY', 'QA_DOORBLAST', 'QA_FRAMEHOLDS', 'ONEWAY']) process.env[k] ??= '1';
const { ROOT } = await import('./headless.ts');
const BT: any = await import('./bang-tally.ts');
const { DOORTOOLS } = await import(`${ROOT}/src/game/doortools.ts`);
const { killGuard, detonateStun, FLASH } = await import(`${ROOT}/src/game/combat.ts`);

const arg = (k: string, d: string) => { const i = process.argv.indexOf(k); return i > 0 ? process.argv[i + 1] : d; };
const SCN = arg('--scn', 'barracks-four'), SEED = +arg('--seed', '1'), BREAK = process.env.OWNCAN_BREAK === '1';
const flat = (a: number[], b: number[]) => Math.hypot(a[0] - b[0], a[2] - b[2]);
const log = console.log, quiet = async <T>(fn: () => T | Promise<T>): Promise<T> => { const e = console.error, w = console.warn; console.log = console.error = console.warn = () => { }; try { return await fn(); } finally { console.log = log; console.error = e; console.warn = w; } };
let all = true; const verdict = (ok: boolean, label: string, detail: string) => { all &&= ok; log(`- ${ok ? 'PASS' : 'FAIL'} — ${label}\n    ${detail}`); };
const put = (gd: any, p: number[]) => { gd.char.pos[0] = p[0]; gd.char.pos[2] = p[2]; gd.char.vel = [0, 0, 0]; gd.path = []; gd.pathGoal = null; gd.speed = 0; };
const frame = (d: any, side: 1 | -1) => { const th = d.def.closedDir, u = [Math.cos(th), Math.sin(th)], n = [-u[1] * side, u[0] * side], c = d.frameCentre;
  return { at: (b: number, l: number): number[] => [c[0] + n[0] * b + u[0] * l, 0, c[2] + n[1] * b + u[1] * l], off: (p: number[]) => (p[0] - c[0]) * n[0] + (p[2] - c[2]) * n[1] }; };
const s1 = (x: number) => x.toFixed(1);

log(`=== own-can-check: ${SCN}, seed ${SEED} ===`);
// ---------------------------------------------------------------- rule
{ const res: string[] = []; let ok = true, staged = false;
  for (const signed of [false, true]) { const S: any = await quiet(() => BT.stage(SCN, SEED)), g = S.g, nav = g.col.nav;
    const ops: any[] = g.orders.squad().filter((x: any) => x.state !== 'dead'), foes: any[] = g.guards.filter((x: any) => x.state !== 'dead' && x.team === 'hostile'), foe = foes[0], staff = foes[1];
    if (ops.length < 3 || !foe || !staff) break;
    // open floor: a centre with three places round it, each with a line to the burst
    let at: number[] | null = null, spots: number[][] = [];
    for (const c of [g.player.char.pos, ...ops.map(x => x.char.pos)]) { if (at) break;
      for (let ox = -6; ox <= 6 && !at; ox += 1.5) for (let oz = -6; oz <= 6 && !at; oz += 1.5) { const o = [c[0] + ox, 0, c[2] + oz]; if (nav.isBlocked(o[0], o[2])) continue;
        const ring = (r: number) => { const out: number[][] = []; for (let k = 0; k < 16; k++) { const a = k * Math.PI / 8, p = [o[0] + Math.sin(a) * r, 0, o[2] + Math.cos(a) * r];
          if (!nav.isBlocked(p[0], p[2]) && !g.col.segmentBlocked([o[0], 0.55, o[2]], [p[0], 1.6, p[2]])) out.push(p); } return out; };
        const far = ring(4), near = ring(1.5); const a = far[0], b = far.find(p => flat(p, a) > 2.5), c2 = far.find(p => flat(p, a) > 2.5 && !!b && flat(p, b) > 2.5);
        if (a && b && c2 && near.length) { at = o; spots = [a, b, near[0], c2]; } } }
    if (!at) break; staged = true;
    const [thrower, mate, close] = ops; const away = (i: number) => [at![0] + 40 + i * 2, 0, at![2] + 40];
    for (const x of g.guards) if (x !== mate && x !== close && x !== foe && x !== staff) put(x, away(g.guards.indexOf(x)));
    await quiet(() => g.teleportPlayer(away(30) as any));
    const pin = () => { put(mate, spots[0]); put(foe, spots[1]); put(close, spots[2]); put(staff, spots[3]); }; pin(); await quiet(() => { for (let i = 0; i < 3; i++) { S.step(1); pin(); } });
    for (const x of [mate, foe, close, staff]) { x.dazzledUntil = 0; x.staggerUntil = 0; } staff.team = 'guard';   // (a staff man: what a quiet mission's partner is up against)
    const it = g.items.spawn('stun', [at[0], 0.25, at[2]], [0, 0, 0], { fuse: 99, heading: 0 }); it.who = thrower.char.id; it.fired = true; if (signed && !BREAK) it.side = 'operative';
    const list = g.guards; g.guards = list.filter((x: any) => x !== thrower); try { await quiet(() => detonateStun(g, it)); } finally { g.guards = list; }
    const secs = (x: any) => Math.max(0, x.dazzledUntil - g.time), fold = (x: any) => Math.max(0, x.staggerUntil - g.time);
    res.push(`${signed ? "signed 'operative'" : 'unsigned'}: teammate at 4 m blind ${s1(secs(mate))} s, folded ${s1(fold(mate))} s · teammate at 1.5 m blind ${s1(secs(close))} s, folded ${s1(fold(close))} s · defender at 4 m blind ${s1(secs(foe))} s, folded ${s1(fold(foe))} s · a man of team 'guard' at 4 m blind ${s1(secs(staff))} s, folded ${s1(fold(staff))} s`);
    const foeFull = secs(foe) >= FLASH.secs[1] - 0.01 && fold(foe) >= FLASH.stagger[1] - 0.01;
    ok &&= foeFull && (signed ? secs(mate) === 0 && fold(mate) === 0 && secs(close) > 0 && secs(close) <= FLASH.ownSecs + 0.01 && fold(close) === 0 && secs(staff) >= FLASH.secs[1] - 0.01 && fold(staff) > 0
      : secs(mate) >= FLASH.secs[1] - 0.01 && fold(mate) > 0 && secs(staff) === 0 && fold(staff) === 0); }
  verdict(staged && res.length === 2 && ok, 'the rule: an unsigned can with its thrower off the list is a guard\'s to his mates; signed, it is their own side\'s', staged ? res.join('\n    ') : 'could not stage both halves: four men, two defenders and open floor with four places round a burst (a pair scenario has too few men: use a squad of four)');
}
DOORTOOLS.ownCan = true;
log(`=== own-can-check: ${all ? 'ALL PASS' : 'FAILURES above'} ===`);
process.exit(all ? 0 : 1);
