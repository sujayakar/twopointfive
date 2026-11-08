// A THING THE APPLY MAKES SAYS NOTHING AS ITS TYPE SAYS IT: after an apply every Option field of a made row or record that its TypeScript type has as `X | null` is a value or
// null, never not there (tools/blitz/class-nulls.json, by tools/wire/nulls.ts). A run's verdict is such a record: its `kind` stood as not there on the solo night, `kind !== null`
// read a squad's mission, and the end card lost its half.
//   TPS_GAME=rust bun run tools/blitz/made-null-check.ts     exit 0 every made thing whole · 1 a field not there · 3 too little was made to say
// MADE_NULL_BREAK=1 takes the mend out, and the check must fail.
if (process.env.MADE_NULL_BREAK) (globalThis as any).__madeNullBreak = true;
const { standUp } = await import('../qa/headless.ts');
const { startFromQuery } = await import('../../src/game/scenarioRun.ts');
const { MADE } = await import('./import.ts');
const NULLS: Record<string, string[]> = (await import('./class-nulls.json')).default as any;
const made = new Map<string, Set<any>>(), lost = new Map<string, number>(); let n = 0, ends = 0;
MADE.told = (o, T) => { const t = `${T.mod}::${T.name}`; if (!NULLS[t]) return; const S = made.get(t) ?? made.set(t, new Set()).get(t)!; if (S.size < 4000) S.add(o); n++; };
/** every made thing as it now stands: what its class answers by an accessor is no field */
const hold = (where: string): void => { for (const [t, S] of made) for (const o of S) for (const k of NULLS[t]) if (!(k in Object.getPrototypeOf(o)) && o[k] === undefined) { const q = `${t}.${k}`; if (!lost.has(q)) console.log(`FAIL: ${q} is not there on a made one (${where})`); lost.set(q, (lost.get(q) ?? 0) + 1); } };
const ended = (): any[] => [...made.get('verdict::RunVerdict') ?? []];
for (const q of ['solo', 'roomclear&squad4', 'rescue', 'nightbreach']) { const W: any = await standUp({ search: `?${q}` } as any), g = W.game;
  if (q !== 'solo') startFromQuery(g, `?${q}`);
  W.step(1 / 60); g.godMode = false; hold(`${q}, as it stands`);
  const run = (secs: number) => { for (let i = 0; i < secs * 60; i++) W.step(1 / 60); };
  run(2); for (const h of g.guards.filter((x: any) => x.team !== 'operative').slice(0, 3)) { g.dropThrowable('frag', [h.char.pos[0], 0.3, h.char.pos[2]]); run(3); hold(`${q}, a frag`); }
  const before = ended().length; for (let i = 0; i < 4 && ended().length === before; i++) { const p = g.player.char.pos; for (let k = 0; k < 3; k++) g.dropThrowable('frag', [p[0], 0.3, p[2]]); run(8); }
  const V = ended().pop(); if (ended().length > before) { ends++; console.log(`${q}: ended, kind ${V.kind}, approach ${V.approach}, hostages ${V.hostages && 'some'}`); } else console.log(`${q}: no end (down ${g.player.down})`);
  hold(`${q}, its end`); }
const kinds = made.size; console.log(`=== made-null-check: ${n} made of ${kinds} types that can be null, ${ends} runs ended: ${lost.size ? `${lost.size} fields not there` : 'every one whole'} ===`);
process.exit(lost.size ? 1 : kinds < 10 || !ends ? 3 : 0);
