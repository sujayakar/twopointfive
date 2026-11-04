// ONE MAN DRAWN AHEAD, AND PUT BACK WHATEVER THROWS (src/host/ahead.ts AheadDraw.around: the picture's half of a networked page's look ahead).
//   bun run tools/qa/aheaddraw-check.ts                    every line below; exit 0 = all pass, 1 = one failed. Run it on BOTH stands (TPS_PROFILE=shipped in front).
//   AHEADDRAW_BREAK=1 bun run tools/qa/aheaddraw-check.ts  the red path: the pair as it stood before `around` (the pose outside the try). 'a pose that throws'
//                                                          must FAIL, and 'the draw after it' with it (it is put back over what was left hung); no other line
// WHY: for the length of one draw the game's own boxes, lights and shadow proxies of one man are the MOVED ones. A networked page's next tick must find the
// tick's own again, or its game is no longer the server's. The draw and the pose both run code that can throw (a renderer, a palette write), and the pair has to
// hold through either. The page check (mpclient-page.cjs) sees real drawn frames; nothing there can make one throw.
// WHAT IS HELD (headless: a whole Game on the harness's stand-in engine, its own men's palette; no GPU, nothing drawn):
//   moved               inside the draw the boxes he wears, the light he carries and his shadow proxies are moved by the look ahead's move, and nobody else's
//   put back            after it every box, light and proxy is the tick's again: the same arrays by reference, the proxies value for value
//   a draw that throws  the error comes out of `around`, and everything is put back
//   a pose that throws  the palette write throws after his boxes and lights were hung moved: the error comes out, the draw never runs, everything is put back
import { standUp, ROOT } from './headless.ts';

const BREAK = process.env.AHEADDRAW_BREAK === '1';
const { AheadDraw } = await import(`${ROOT}/src/host/ahead.ts`);
const { MKEY } = await import(`${ROOT}/src/scene/boxes.ts`);
/** a box he wears or carries, as ahead.ts says it: the rig's own key with his id in it (a prop's key can hold the same byte, and is nobody's) */
const worn = (b: any, id8: number): boolean => b.mkey !== undefined && b.mkey >>> 24 === MKEY.rig >>> 24 && ((b.mkey >>> 8) & 0xff) === id8;
let failed = 0;
const line = (ok: boolean, text: string): void => { if (!ok) failed++; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${text}`); };

const W: any = await standUp({ seed: 5 });
for (let i = 0; i < 30; i++) W.step(1 / 60);
const E = W.engine, world = E.world, lights = E.lights, skin = E.skin;
const id = world.capsuleU32[7] & 0xff;   // (the first man's: the player)
let boom = false, writes = 0;
{ const real = skin.drawJointsOf.bind(skin); skin.drawJointsOf = (i: number, d: Float32Array): void => { writes++; if (boom) throw new Error('the palette write threw'); real(i, d); }; }

interface Kept { box: { b: any; c: any; rot: any; dc: any; yaw: number; at: number[] }[]; light: { l: any; pos: any; dir: any; at: number[] }[]; caps: Float32Array; info: Float32Array }
const keep = (): Kept => ({
  box: world.dynamics.map((b: any) => ({ b, c: b.c, rot: b.rot, dc: b.dc, yaw: b.yaw, at: [b.c[0], b.c[1], b.c[2]] })),
  light: [...lights.lights, ...lights.ir].map((l: any) => ({ l, pos: l.pos, dir: l.dir, at: [l.pos[0], l.pos[1], l.pos[2]] })),
  caps: world.capsuleData.slice(), info: world.infoF32.slice() });
/** how many things are not as kept (0 = the tick's own, all of it) */
const changed = (K: Kept): { boxes: number; lights: number; caps: number } => {
  let boxes = 0, ls = 0, caps = 0;
  for (const k of K.box) if (k.b.c !== k.c || k.b.rot !== k.rot || k.b.dc !== k.dc || k.b.yaw !== k.yaw || k.at.some((v, i) => k.c[i] !== v)) boxes++;
  for (const k of K.light) if (k.l.pos !== k.pos || k.l.dir !== k.dir || k.at.some((v, i) => k.pos[i] !== v)) ls++;
  const D = world.capsuleData; for (let i = 0; i < K.caps.length; i++) if (D[i] !== K.caps[i]) caps++;
  const I = world.infoF32; for (let i = 0; i < K.info.length; i++) if (I[i] !== K.info[i]) caps++;
  return { boxes, lights: ls, caps };
};
const none = (c: { boxes: number; lights: number; caps: number }): boolean => c.boxes + c.lights + c.caps === 0;
const say = (c: { boxes: number; lights: number; caps: number }): string => none(c) ? 'every box, light and proxy is the tick\'s own again' : `${c.boxes} boxes, ${c.lights} lights and ${c.caps} proxy words are STILL THE MOVED ONES`;

const ad: any = new AheadDraw({ world, lights, skin, outline: null });
// (the red path: the pair as it stood — the pose outside the try, so a pose that throws is never taken down)
if (BREAK) ad.around = (a: number, blended: Float32Array | null, draw: () => void): void => { ad.pose(a, blended); try { draw(); } finally { ad.unpose(); } };
const me = { pos: [world.infoF32[24], 0, world.infoF32[26]], yaw: 0 }, MOVE = 0.4;
const there = { pos: [me.pos[0] + MOVE, 0, me.pos[2]], yaw: 0.3 };
ad.capture(id, me, there);

const K = keep();
// ---- moved, and put back
{ let inside = { boxes: 0, lights: 0, caps: 0 }, others = 0;
  ad.around(1, null, () => { inside = changed(K);
    for (const k of K.box) if (k.b.c !== k.c && !worn(k.b, id)) others++; });
  const L = ad.last, his = world.dynamics.filter((b: any) => worn(b, id)).length;
  line(L.on && L.boxes > 0 && L.caps === 1 && inside.boxes === L.boxes && inside.boxes === his && inside.lights === L.lights && inside.caps > 0 && others === 0 && Math.abs(L.move - MOVE) < 1e-9,
    `moved: inside the draw man ${id} stood ${L.move.toFixed(2)} m on and ${L.turn.toFixed(2)} rad round: ${inside.boxes} of the ${his} boxes he wears (the look ahead says ${L.boxes}), ${inside.lights} carried light${inside.lights === 1 ? '' : 's'} (${L.lights}), ${L.men} palette instance${L.men === 1 ? '' : 's'} and ${L.caps} set of shadow proxies were the moved ones, and ${others} of anybody else's boxes`);
  const after = changed(K);
  line(none(after), `put back: after the draw ${say(after)}`);
}
// ---- a draw that throws
{ let out = '';
  try { ad.around(1, null, () => { throw new Error('the render threw'); }); } catch (e) { out = String((e as Error).message); }
  const after = changed(K);
  line(out === 'the render threw' && none(after), `a draw that throws: the error ${out ? `came out ("${out}")` : 'DID NOT COME OUT'}, and ${say(after)}`);
}
// ---- a pose that throws (half way: his boxes and lights are hung before the palette is written)
{ let out = '', drew = false; boom = true; const w0 = writes;
  try { ad.around(1, null, () => { drew = true; }); } catch (e) { out = String((e as Error).message); }
  boom = false;
  const after = changed(K), hung = ad.last.boxes;
  line(out === 'the palette write threw' && !drew && writes > w0 && hung > 0 && none(after),
    `a pose that throws: the palette write threw with ${hung} of his boxes and ${ad.last.lights} light${ad.last.lights === 1 ? '' : 's'} already hung moved; the error ${out ? `came out ("${out}")` : 'DID NOT COME OUT'}, the draw ${drew ? 'RAN' : 'never ran'}, and ${say(after)}`);
  // (…and the next draw is a whole one again, put back as the first was)
  let again = 0; ad.around(1, null, () => { again = changed(K).boxes; });
  const last = changed(K);
  line(again > 0 && none(last), `the draw after it: ${again} boxes moved inside it, and ${say(last)}`);
}

console.log(failed ? `aheaddraw-check: ${failed} line${failed === 1 ? '' : 's'} FAILED${BREAK ? ' (AHEADDRAW_BREAK=1: \'a pose that throws\' must, and the draw after it with it)' : ''}` : '=== aheaddraw-check: ALL PASS ===');
process.exit(failed ? 1 : 0);
