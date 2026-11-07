// tools/qa/gunhand-throw-check.ts — a man with a long gun in his hands throws with the hand that is not on its grip
// (src/game/gunhand.ts GUNHAND.on + GUNHAND.throwFree; character.ts throwItem / throwLayer / throwOff; frag.ts fragThrowTick, fragInFist; rigProps.ts freeCanBoxes;
//  combat.ts guardKit).
//
// The fault: the throw is one move for everybody and its throwing arm is the right one. A defender with a carbine threw overhand with the carbine still drawn off
// the throwing fist — the gun went up behind his head and over the top with the grenade.
//
// THE BENCH (no game: a defender's figure with the carbine, driven the way the controllers drive the animator; IN HAND's long hold as the page has it):
//   (3) THE GUN STAYS IN THE TRIGGER HAND, LOWERED: through every frame of every throw the carbine's grip is within 1.5 cm of that hand's palm and that
//       wrist stays below its shoulder;
//   (4) THE OTHER HAND THROWS: the left wrist goes up over its shoulder in the overhand and is out ahead of him at the release in every one;
//   (5) THE CAN IS IN THAT HAND: until the release the throw is the free hand's by the animator's own word (throwOff: what the drawer asks before it
//       draws one) and the can's box (rigProps freeCanBoxes) is in the left fist.
// THE GAME (frag-theirs' corridor: a defender dealt the carbine, alert on two of the team bunched in his view):
//   (8) he throws with the carbine in his hands, and the grenade leaves him FRAG.defWind after his shout;
//   (9) it starts within a hand's breadth of the hand that threw it, the grenade is drawn in that fist until it leaves, and the trigger hand's wrist
//       stays below its shoulder through the throw.
// THE RULE IS SETTLED (GUNHAND.on and .throwFree, the in-hand pass and the throw by distance are constants of the game), so since 2026-11-07 nothing
// here writes them and no figure is stepped with the rule off. Gone, with their numbers: (1) the two stands' row and word, (2) the fault with the rule
// off, (6) the same throw with the rule off and on, (7) the key off bit for bit as the switch off, the halves of (4), (5), (8) and (9) that held the
// throw against the rule-off one, (10) a recording from before the switch and the page's off flag. The five scripts that held the throw by distance off
// to get the overhand no longer do (the write was refused: they ran as the page throws already).
//
//   bun run tools/qa/gunhand-throw-check.ts [--verbose]
//   GUNHAND_BREAK=1 …                                       (the staging broken: the bench man has no long gun in his hands, so the throw is his right
//                                                            hand's: (3), (4) and (5) must fail)
// Headless: poses and boxes are measured, nothing here is a picture. Never seen on a GPU or in play.
import './stand.ts';   // (the stand's word, written down before this file reads it)
import { resolve } from 'node:path';
const ROOT = resolve(import.meta.dir, '../..');

type V3 = [number, number, number];
const sub = (a: V3, b: V3): V3 => [a[0] - b[0], a[1] - b[1], a[2] - b[2]], len = (a: V3) => Math.hypot(a[0], a[1], a[2]);
/** a joint as the picture has it, in the world (the look-only pass may have moved a hand off bones.*) */
const drawn = (c: any, node: number): V3 => { const w = c.world, p = c.skel.nodePos(node); return [w[0] * p[0] + w[4] * p[1] + w[8] * p[2] + w[12], w[1] * p[0] + w[5] * p[1] + w[9] * p[2] + w[13], w[2] * p[0] + w[6] * p[1] + w[10] * p[2] + w[14]]; };
/** the hollow of a fist: 9 cm along the fingers from the wrist, 3 cm out of the palm */
const hollow = (wrist: V3, hf: { fingers: V3; back: V3 }): V3 => [0, 1, 2].map(i => wrist[i] + hf.fingers[i] * 0.09 - hf.back[i] * 0.03) as V3;
const FRAG_BODY = [0.13, 0.17, 0.08];   // (rigProps heldCanBoxes: the grenade's body)

export async function gunhandThrowCheck(): Promise<boolean> {
  const H: any = await import('./headless.ts');
  const CH: any = await import(`${ROOT}/src/game/character.ts`);
  const RP: any = await import(`${ROOT}/src/game/rigProps.ts`);
  const CB: any = await import(`${ROOT}/src/game/combat.ts`);
  const FR: any = await import(`${ROOT}/src/game/frag.ts`);
  const KN: any = await import(`${ROOT}/src/game/knobs.ts`);
  const VERBOSE = process.argv.includes('--verbose'), BREAK = process.env.GUNHAND_BREAK === '1';
  let all = true; const f1 = (x: number) => x.toFixed(1), f2 = (x: number) => x.toFixed(2), cm = (x: number) => f1(x * 100);
  const line = (ok: boolean, s: string, detail = '') => { all &&= ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${s}${detail ? `\n    ${detail}` : ''}`); };
  const hushed = async <T>(fn: () => Promise<T> | T): Promise<T> => { const log = console.log, err = console.error, warn = console.warn; console.log = console.error = console.warn = () => { }; try { return await fn(); } finally { console.log = log; console.error = err; console.warn = warn; } };
  console.log(`\n=== gunhand-throw: a man with a long gun in his hands throws with the hand that is not on its grip${BREAK ? ' (GUNHAND_BREAK: no long gun in the bench man\'s hands)' : ''} ===`);
  const knobs0 = KN.knobsCopy();
  try {
    // ------------------------------------------------------------ the bench
    const ch: any = await H.loadCastCharacter(); const DT = 1 / 60, REL = CH.THROW_RELEASE as number;
    const man = (): any => { const c = new CH.Character(ch, { id: 24, tint: [0.1, 0.1, 0.1], tint2: [0.05, 0.05, 0.05], pos: [0, 0, 0], yaw: 0 }), a = c.anim;
      a.phase = 0; a.trained = 0.12; a.upper = 'none'; a.stance = 'none'; a.longGun = !BREAK; a.longSteady = 0; a.longGrip = RP.carbineGrip(); c.muzzleAhead = RP.carbineMuzzleAhead(); return c; };
    const step = (c: any) => { c.inHand = RP.longHold('carbine'); if (c.vel) c.pos = [c.pos[0] + c.vel[0] * DT, c.pos[1], c.pos[2] + c.vel[2] * DT]; c.update(DT); };
    interface Row { t: number; held: boolean; free: boolean; read: string | null; gap: number; wrR: number; gripUp: number; wrL: number; aheadL: number; can: number }
    const read = (c: any): Row => { const n = c.anim.nodes, a = c.anim, meta: any[] = [], bx: any[] = [];
      RP.rigBoxes(c, { goggles: false, nv: false, slot: 1, lightOn: false, isGuard: true, holster: false, weapon: 'carbine' }, bx, RP.RIG, meta);
      const gm = meta.find(x => /\.grip$/.test(x.path)), grip: V3 = gm ? bx[gm.boxStart].c : [0, 0, 0];
      const wR = drawn(c, n.handR), sR = drawn(c, n.upperArmR), wL = drawn(c, n.handL), sL = drawn(c, n.upperArmL), f = c.forward();
      const cans: any[] = [], cm2: any[] = []; RP.freeCanBoxes(c, 'frag', cans, cm2); const body = cans[cm2.find(x => x.path === 'can.body')?.boxStart ?? 0];
      return { t: a.throwTime, held: a.throwing && a.throwTime < REL, free: a.throwOff, read: a.throwRead, gap: len(sub(grip, hollow(wR, c.handFrameR()))), wrR: wR[1] - sR[1], gripUp: grip[1] - sR[1], wrL: wL[1] - sL[1],
        aheadL: (wL[0] - sL[0]) * f[0] + (wL[2] - sL[2]) * f[2], can: body ? len(sub(body.c, hollow(wL, c.handFrameL()))) : 9 }; };
    interface Script { name: string; over: boolean; pre: (c: any) => void; dist?: number; /** thrown at a distance given: the swing is the distance's, not asked for the overhand */ by?: boolean }
    const aim = (c: any) => { c.anim.upper = 'aim'; }, low = (c: any) => { c.anim.stance = 'lowReady'; };
    const SCRIPTS: Script[] = [
      { name: 'aimed', over: true, pre: aim }, { name: 'low ready', over: true, pre: low }, { name: 'calm carry', over: true, pre: () => { } },
      { name: 'crouched', over: true, pre: c => { aim(c); c.anim.crouchTarget = 1; } }, { name: 'walking', over: true, pre: c => { aim(c); c.anim.speed = 1.4; c.vel = [0, 0, 1.4]; } },
      { name: 'a short one', over: true, pre: aim, dist: 5, by: true }, { name: 'a toss', over: false, pre: aim, dist: 1.5, by: true },
    ];
    /** one script on one figure */
    const play = (S: Script): { on: Row[] } => {
      const c = man(), rows: Row[] = [];
      S.pre(c);
      for (let i = 0; i < 60; i++) step(c);
      c.anim.throwItem(S.dist);
      for (let i = 0; i < 90 && (i < 2 || c.anim.inThrow); i++) { step(c); if (c.anim.inThrow) rows.push(read(c)); }
      return { on: rows };
    };
    const runs = SCRIPTS.map(S => ({ S, ...play(S) }));
    const mx = (rs: Row[], f: (r: Row) => number) => rs.reduce((a, r) => Math.max(a, f(r)), -9), mn = (rs: Row[], f: (r: Row) => number) => rs.reduce((a, r) => Math.min(a, f(r)), 9);
    const atRel = (rs: Row[]) => rs.reduce((b, r) => Math.abs(r.t - REL) < Math.abs(b.t - REL) ? r : b, rs[0]);
    // (3) the gun stays in the trigger hand, lowered
    { const say = runs.map(r => `${r.S.name}: grip to the palm's hollow ${cm(mx(r.on, x => x.gap))} cm at worst · the trigger wrist ${cm(-mx(r.on, x => x.wrR))} cm below its shoulder at its highest · ${r.on.length} frames`);
      line(runs.every(r => r.on.length > 30 && mx(r.on, x => x.gap) <= 0.015 && mx(r.on, x => x.wrR) < 0), '(3) through every frame of every throw the carbine\'s grip is within 1.5 cm of the trigger hand\'s palm and that wrist stays below its shoulder', say.join('\n    ')); }
    // (4) the other hand throws
    { const say = runs.map(r => `${r.S.name} (${atRel(r.on).read}): the left wrist ${cm(mx(r.on, x => x.wrL))} cm over its shoulder at its highest, ${cm(atRel(r.on).aheadL)} cm ahead of it at the release`);
      line(runs.every(r => atRel(r.on).aheadL > 0.3 && (!r.S.over || r.S.by || mx(r.on, x => x.wrL) > 0.1)), '(4) the other hand throws: the left wrist goes up over its shoulder in the overhand, and is out ahead of him at the release in every one', say.join('\n    ')); }
    // (5) the can is in that hand
    { const held = (rs: Row[]) => rs.filter(r => r.held), say = runs.map(r => `${r.S.name}: ${cm(mx(held(r.on), x => x.can))} cm from the left fist's hollow at worst over ${held(r.on).length} frames`);
      line(runs.every(r => held(r.on).length >= 12 && held(r.on).every(x => x.free) && mx(held(r.on), x => x.can) <= 0.06), '(5) until the release the throw is the free hand\'s by the animator\'s own word (what the drawer asks) and the can\'s box is in the left fist', say.join('\n    ')); }
    if (VERBOSE) for (const r of runs) console.log(`    [${r.S.name}] t / wrR / wrL / gap / can\n` + r.on.filter((_x, i) => i % 4 === 0).map(x => `      ${f2(x.t)}  ${cm(x.wrR)}  ${cm(x.wrL)}  ${cm(x.gap)}  ${cm(x.can)}`).join('\n'));

    // ------------------------------------------------------------ the game
    const FP: any = await import('./frag-probes.ts');
    const game = async (seed: number) => {
      const S: any = await hushed(() => FP.theirsCorridor(seed, true)); const g = S.g, C = S.C, R = S.R;
      try {
        let shoutT = NaN; { const say = g.say.bind(g); g.say = (gd: any, text: string, radio?: boolean) => { if (gd === C && isNaN(shoutT) && /out|hole/i.test(text)) shoutT = g.time; return say(gd, text, radio); }; }
        let out: { t: number; from: V3; hl: V3; hr: V3; who: number } | null = null;
        { const sp = g.items.spawn.bind(g.items); g.items.spawn = (kind: string, from: V3, v0: V3, o: any) => { if (kind === 'frag' && !out) out = { t: g.time, from: [...from] as V3, hl: [...C.char.bones.handL] as V3, hr: [...C.char.bones.handR] as V3, who: o?.who }; return sp(kind, from, v0, o); }; }
        let frames = 0, fist = 0, canWorst = 0, wrR = -9, wrL = -9, long = true; const n0 = FR.FRAG_LOG.length;
        await hushed(() => { for (let i = 0; i < 14 * 60 && FR.FRAG_LOG.length === n0; i++) { R.step();
          const c = C.char, a = c.anim; if (!a.inThrow) continue; frames++; long &&= a.longGun;
          const b = c.bones; wrR = Math.max(wrR, b.handR[1] - b.upperArmR[1]); wrL = Math.max(wrL, b.handL[1] - b.upperArmL[1]);
          if (!FR.fragInFist(g, C)) continue;
          const bx: any[] = []; CB.handProps(g, bx); const can = bx.find(x => (x.owner & 0xff) === (c.id & 0xff) && FRAG_BODY.every((v, k) => Math.abs(x.albedo[k] - v) < 1e-6));
          if (can) { fist++; canWorst = Math.max(canWorst, len(sub(can.c, hollow(drawn(c, a.nodes.handL), c.handFrameL())))); } } });
        const rep = FR.FRAG_LOG[n0] ?? null, o = out as { t: number; from: V3; hl: V3; hr: V3; who: number } | null;
        return { who: C.callsign as string, long, lead: o ? o.t - shoutT : NaN, out: o, at: rep ? [...rep.at] as V3 : null, frames, fist, canWorst, wrR, wrL };
      } finally { S.restore(); }
    };
    type Run = Awaited<ReturnType<typeof game>>; let b: Run | null = null, seed = 0, tried = 0;
    for (const s of [3, 5, 8, 11]) { seed = s; tried++; b = await game(s); if (b.out && b.at) break; }
    if (!b || !b.out || !b.at) line(false, '(8) staging: a defender with the carbine throws', `no throw · seeds tried ${tried}`);
    else {
      line(b.long && Math.abs(b.lead - FR.FRAG.defWind) < 0.05,
        '(8) in the game: he throws with the carbine in his hands, and the grenade leaves him the wind-up after his shout', `seed ${seed}: ${b.who}, the carbine in his hands ${b.long} · ${f2(b.lead)} s after the shout (${FR.FRAG.defWind} asked) · the swing ${b.frames} frames`);
      line(len(sub(b.out.from, b.out.hl)) <= 0.1 && b.fist >= 10 && b.canWorst <= 0.06 && b.wrR < 0 && b.wrL > 0.1,
        '(9) it starts within a hand\'s breadth of the hand that threw it, the grenade is drawn in that fist until it leaves, and the trigger hand\'s wrist stays below its shoulder',
        `starts ${cm(len(sub(b.out.from, b.out.hl)))} cm from the left hand (${cm(len(sub(b.out.from, b.out.hr)))} from the right) · in the left fist ${b.fist} frames, ${cm(b.canWorst)} cm from its hollow at worst · the trigger wrist ${cm(-b.wrR)} cm below its shoulder at its highest, the left ${cm(b.wrL)} cm over its own`);
    }
  } finally { KN.knobsBack(knobs0); }
  console.log(`=== gunhand-throw: ${all ? 'ALL PASS' : 'FAILURES above'} ===`);
  return all;
}

if (import.meta.main) process.exit((await gunhandThrowCheck()) ? 0 : 1);
