// The 'enemysquads' family (game/enemysquads.ts — defender SQUADS on a level whose room clear structure declares them), run through probes.ts:
//   bun run tools/qa/probes.ts enemysquads | enemysquads-stage | enemysquads-noise | enemysquads-radiocheck
// PASS / FAIL each, all headless (the Game of headless.ts on levels/compound.json, started exactly as the page's scenario row starts it). No order is given
// to the team in any of them: what is under test is what each squad DOES with what it hears — how a whole clear goes is the scripted tally's
// (tools/qa/playthrough.ts --scn compound-four, whose RunOut carries every squad's timeline).
//   enemysquads-stage    the file's four squads stand as dealt (Yard 5 in the garage + shed, Gate 1 in the hut, West 2 in the sala + kitchen, Keep 3 in the office + bunk + back), sixteen characters
//   enemysquads-noise    surprise lost on open floor (one loud report where the team starts, nobody in contact): every squad says ONE thing on its net and
//                        gets on its doors, nobody is sent at the noise, and the first contact call is answered at once — not all of
//                        them out, not all of them at home when a way round exists
// SQUADS.on and its later rules are settled: the probes ask the page's side alone and write none of them. Until 2026-11-07 stage also asked 'SQUADS.on
// off: one pool' and 'a level with no squads block (records) stages none' (the build before squaddeal.ts deals such a level into squads: out by
// ruling), noise the first cut with SQUADS.openPair on (settled off), and contact the same staging with SQUADS.on off; contact also held rounds
// through thin walls off (WALLPEN.on, settled on): the mess is now staged with them, as the page plays it.
//   enemysquads-radiocheck  unseen drops are found by radio check or body (SQUADS.radioCheck and the rules that followed it, all settled): a man dropped
//                        silently where no mate can make the body out — nothing until he is asked for by name on the net, men WALK to
//                        his post, one of them has the body at his feet, THEN contact; a body in a mate's open view is contact by his eyes; a man taken
//                        from his post (held) is asked for, looked for, 'no sign', the men back at their posts and no call on the net; a loud drop is contact at once.
//                        (The rest of the radio check's sections, thirty floors of the game at some twenty seconds of work a floor, were cut to keep this under a minute.)
// What is said on the net is read off the log's rows.
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';
import { standUp, ROOT } from './headless.ts';
const { parseLevelData, validateLevelData } = await import(`${ROOT}/src/scene/levelData.ts`);
const { loadLevel } = await import(`${ROOT}/src/scene/levelLoad.ts`);
const { startFromQuery } = await import(`${ROOT}/src/game/scenarioRun.ts`);
const { SQUADS, squadsReport, squadsLive, squadCalls, squadMen } = await import(`${ROOT}/src/game/enemysquads.ts`);
/** a man's name as his own squad says it on the net: 'Tango 8' is his number, a man the level named keeps his name */
const NUMBER = ['Zero', 'One', 'Two', 'Three', 'Four', 'Five', 'Six', 'Seven', 'Eight', 'Nine', 'Ten', 'Eleven', 'Twelve', 'Thirteen', 'Fourteen', 'Fifteen', 'Sixteen'];
const mate = (m: any): string => { const k = /^Tango (\d+)$/.exec(m.callsign); return k ? NUMBER[+k[1]] ?? k[1] : m.callsign; };
const { hasHim, OWN } = await import(`${ROOT}/src/game/owner.ts`);
const { killGuard } = await import(`${ROOT}/src/game/combat.ts`);
const { BODY_LIGHT } = await import(`${ROOT}/src/game/guards.ts`);
const { PLAYER_ID } = await import(`${ROOT}/src/game/consts.ts`);
const { digestGame, hashDigest } = await import(`${ROOT}/src/game/digest.ts`);

const DT = 1 / 60;
function seeded(a: number) { return () => { a |= 0; a = (a + 0x6D2B79F5) | 0; let t = Math.imul(a ^ (a >>> 15), 1 | a); t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t; return ((t ^ (t >>> 14)) >>> 0) / 4294967296; }; }
type Verdict = (ok: boolean, label: string, detail: string) => void;
function family(title: string): { verdict: Verdict; done: () => boolean } {
  console.log(`\n=== ${title} ===`); let all = true;
  return { verdict: (ok, label, detail) => { all &&= ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${label}\n    ${detail}`); }, done: () => { console.log(`=== ${title.split(':')[0]}: ${all ? 'ALL PASS' : 'FAILURES above'} ===`); return all; } };
}
const CHOICES = ['barricade', 'flank', 'help'];
async function rig(file: string, seed: number) {
  Math.random = seeded(seed);
  const log = console.log, err = console.error, warn = console.warn;
  let W: any, why: string | null = null, data: any;
  try {
    data = parseLevelData(readFileSync(resolve(ROOT, 'levels', file), 'utf8'));
    console.log = console.error = console.warn = () => { };
    W = await standUp({ uploadEvery: 4, level: loadLevel(data) }); why = (await startFromQuery(W.game, new URLSearchParams(`?levelFile=${file}&roomclear=${seed}&squad4`))).why ?? null;
  } finally { console.log = log; console.error = err; console.warn = warn; }
  const g = W.game; g.godMode = true;
  // what is said on the net, by the log's own rows ('Tango 3 (radio): …'): a bubble or not, every such line has one
  const said: string[] = []; { const msg = g.msg.bind(g); g.msg = (t: string) => { const k = /\(radio\): (.*)$/.exec(t); if (k) said.push(k[1]); return msg(t); }; }
  return { W, g, data, why, said, step(secs: number) { for (let i = 0; i < secs * 60; i++) { g.player.visibility = 0.05; W.step(DT); } }, done() { /* (nothing of the stand's was written) */ },
    choices(name: string) { return (squadsReport(g).find((q: any) => q.name === name)?.events ?? []).filter((e: any) => CHOICES.includes(e.what)); } };
}
// where 'the team is in the mess' is staged: West's own rally point as the level file gives it. The block's rooms turn a corner since its rebuild, and the
// spot these probes had picked by eye off the old plan is behind a wall on the new one; the rally is where the file's author gathers the squad, on open floor
// of the mess in its men's view on the block as it stands (nothing in the format promises that: the lines that stand Sam here say where he ended up, and
// 'West is in contact' is red when nobody of it sees him)
const messSpot = (R: any): number[] => { const r = R.data.scenarios.roomClear.squads.find((q: any) => q.id === 'west')?.rally; if (!r) throw new Error('enemysquads probes: compound.json gives West no rally point to stand Sam on'); return [r[0], 0, r[2]]; };
const stood = (g: any) => `Sam at ${g.player.char.pos[0].toFixed(1)}, ${g.player.char.pos[2].toFixed(1)}${inRoom(g, 'mess', g.player.char.pos) ? ', in the mess' : ', NOT in the mess'}`;
const inRoom = (g: any, id: string, p: number[]) => { if (id === 'mess') return inRoom(g, 'sala', p) || inRoom(g, 'kitchen', p); if (id === 'keep') return ['office', 'bunk', 'back'].some(x => inRoom(g, x, p)); const b = g.roomClear.structure.rooms.find((r: any) => r.id === id).bounds; return p[0] >= b.x0 && p[0] <= b.x1 && p[2] >= b.z0 && p[2] <= b.z1; };

export async function enemysquadsStage(): Promise<boolean> {
  const F = family('enemysquads-stage: the squads of the file stand as dealt');
  const R = await rig('compound.json', 1);
  try {
    const g = R.g, rep = squadsReport(g);
    F.verdict(validateLevelData(R.data).length === 0 && !R.why, 'levels/compound.json validates and the scenario starts', `validate ${validateLevelData(R.data).length} errors · start ${R.why ?? 'ok'}`);
    F.verdict(rep.map((q: any) => `${q.name} ${q.men}`).join(' · ') === 'Yard 5 · Gate 1 · West 2 · Keep 3' && squadsLive(g), 'four squads take the eleven defenders by the rooms they were dealt into', rep.map((q: any) => `${q.name} ${q.men} ${q.state}`).join(' · '));
    F.verdict(1 + g.guards.length === 16 && g.nightBreach.hostiles.length === 11, 'sixteen characters on the floor: Sam, the four, eleven defenders', `${1 + g.guards.length} characters · ${g.nightBreach.hostiles.length} defenders`);
    const H = g.nightBreach.hostiles;
    F.verdict(H.every((h: any) => hasHim(h, OWN.squad)) && H.filter((h: any) => h.staysInRooms).length === 4, 'every defender is a squad\'s man; Keep\'s three and the gate\'s one stay in their rooms', `owned ${H.filter((h: any) => hasHim(h, OWN.squad)).length}/11 · stay in ${H.filter((h: any) => h.staysInRooms).length}`);
    R.step(6);
    F.verdict(squadsReport(g).every((q: any) => q.state === 'hold' && !q.events.length) && !R.said.length, 'with nothing heard and nobody met, every squad holds and the net is silent', squadsReport(g).map((q: any) => `${q.name} ${q.state}`).join(' · ') + ` · ${R.said.length} radio lines`);
  } finally { R.done(); }
  return F.done();
}

export async function enemysquadsNoise(): Promise<boolean> {
  const F = family('enemysquads-noise: surprise lost on open floor — nobody walks out at a noise, every squad stands to on its doors, and the first call is answered at once');
  for (const seed of [1, 2, 3]) {
    const R = await rig('compound.json', seed);
    try {
      const g = R.g; R.step(1);
      const at = [g.player.char.pos[0], 0, g.player.char.pos[2]];
      g.events.push({ kind: 'shot', pos: at, time: g.time, loud: true, level: 1, who: PLAYER_ID });
      R.step(6.5);
      const rep = squadsReport(g), ch = rep.map((q: any) => ({ name: q.name, c: R.choices(q.name) }));
      const words = ch.map((x: any) => `${x.name}: ${x.c.map((e: any) => `${e.what}@${e.t}`).join(',') || '-'}`).join(' · ');
      F.verdict(ch.every((x: any) => x.c.length === 1), `seed ${seed}: every squad has said one thing`, words);
      F.verdict(ch.every((x: any) => x.c[0]?.what === 'barricade') && rep.every((q: any) => q.out === 0), `seed ${seed}: nobody is sent at a noise nobody can place — every squad is on its doors`, `${ch.filter((x: any) => x.c[0]?.what === 'barricade').length} of ${ch.length} barricade · ${rep.reduce((n: number, q: any) => n + q.out, 0)} men out`);
      const charlie = g.roomClear.rooms.filter((r: any) => ['office', 'bunk', 'back'].includes(r.def.id)).flatMap((r: any) => r.defenders);
      F.verdict(charlie.every((h: any) => inRoom(g, 'keep', h.char.pos)), `seed ${seed}: Keep (mayLeave false) stays in its rooms`, charlie.map((h: any) => `${h.callsign} (${h.char.pos[0].toFixed(1)}, ${h.char.pos[2].toFixed(1)})`).join(' '));
      F.verdict(ch.every((x: any) => R.said.some(t => t.startsWith(`${x.name}:`))), `seed ${seed}: each word was said on the net under the squad's name`, R.said.join(' | ').slice(0, 300));
      // then the team is on the mess (a loud round inside it, Sam stood there in plain light): the squads that stood to have their answer out after the
      // net's lag and a short beat, not a thinking beat — men sent, or 'stays on its doors' when too few of it are free of their door work to spare a pair
      const mess = messSpot(R); g.teleportPlayer(mess); g.events.push({ kind: 'shot', pos: mess, time: g.time, loud: true, level: 1, who: PLAYER_ID });
      for (let i = 0; i < 4 * 60; i++) { g.player.visibility = 1; R.W.step(DT); }
      const call = squadsReport(g).find((q: any) => q.name === 'West').events.find((e: any) => e.what === 'contact');
      const word = call ? g.fight.log.find((l: any) => l.t >= call.t && /^\[squads\] Yard (flank|help|barricade|stays on its doors)/.test(l.what)) : null;
      const lag = call && word ? word.t - call.t : NaN, cap = SQUADS.netLag + SQUADS.readyBeat + 0.35;
      F.verdict(!!call && !!word && lag <= cap, `seed ${seed}: West calls contact and Yard, stood to, has its answer inside ${cap.toFixed(2)} s (a squad taken by surprise thinks ${SQUADS.think[0]} to ${SQUADS.think[1]} s on top of the lag)`, call ? `West contact @${call.t} · ${word ? `${word.what.slice(9)} @${word.t.toFixed(1)} (${lag.toFixed(2)} s)` : 'Yard said nothing'}` : 'no contact call');
    } finally { R.done(); }
  }
  return F.done();
}

export async function enemysquadsRadiocheck(): Promise<boolean> {
  const F = family('enemysquads-radiocheck: a drop nobody saw or heard is found out by the squad\'s radio check or by a mate\'s eyes on the body');
  const wasLit = { lit: BODY_LIGHT.lit, litNear: BODY_LIGHT.litNear };
  const dark =(on: boolean) => { BODY_LIGHT.lit = on ? 99 : wasLit.lit; BODY_LIGHT.litNear = on ? 99 : wasLit.litNear; };   // (a floor too dark to make a body out past the near rule's 2 m)
  const flatD = (a: number[], b: number[]) => Math.hypot(a[0] - b[0], a[2] - b[2]);
  const bravo = (g: any) => { const rs = g.roomClear.rooms.filter((r: any) => r.def.id === 'garage' || r.def.id === 'shed'); return rs.flatMap((r: any) => r.defenders.filter((h: any) => h.state !== 'dead')); };
  // the Yard man standing farthest from his nearest mate: the drop a mate is least likely to have at his feet
  const loneliest = (g: any) => { const B = bravo(g); let best = B[0], bd = -1; for (const m of B) { const d = Math.min(...B.filter((x: any) => x !== m).map((x: any) => flatD(x.char.pos, m.char.pos))); if (d > bd) { bd = d; best = m; } } return { m: best, d: bd }; };
  const ev = (g: any, n: string) => squadsReport(g).find((q: any) => q.name === n).events as any[];
  const first = (E: any[], what: string) => E.find(e => e.what === what)?.t ?? NaN;
  const until = (R: any, secs: number, stop: () => boolean) => { for (let i = 0; i < secs * 60 && !stop(); i++) { R.g.player.visibility = 0.05; R.W.step(DT); } };
  try {
    // ---- 2 · on, dark: nothing at the drop; asked for by name; men walk to his post; the body at a man's feet; contact
    { dark(true);
      const R = await rig('compound.json', 1);
      try { const g = R.g; R.step(1); const { m, d } = loneliest(g), t0 = g.time, at = [m.char.pos[0], 0, m.char.pos[2]]; killGuard(g, m, [1, 0, 0], true); R.step(0.1);
        F.verdict(!ev(g, 'Yard').some(e => e.what === 'contact') && !R.said.some(t => /contact/.test(t)), 'on: the same drop — no contact at the drop, nothing said', `${m.callsign} down @${t0.toFixed(2)}, his nearest mate ${d.toFixed(1)} m off · Yard ${squadsReport(g).find((q: any) => q.name === 'Yard').state} · ${R.said.length} radio lines`);
        let walked = 0, n = 0; until(R, 90, () => { for (const x of bravo(g)) if (x.script && x.speed > 0.4) { walked = Math.max(walked, x.speed); n++; } return ev(g, 'Yard').some(e => e.what === 'contact'); });
        const E = ev(g, 'Yard'), asks = first(E, 'asks'), looks = first(E, 'looks'), found = first(E, 'found'), c = first(E, 'contact');
        const iAsk = R.said.findIndex(t => new RegExp(`^Yard: ${mate(m)}\\? … ${mate(m)}, sound off`).test(t)), iGo = R.said.findIndex(t => new RegExp(`^Yard: nothing from ${mate(m)} — (one|two) moving to his post!`).test(t)), iC = R.said.findIndex(t => /^Yard: contact/.test(t));
        F.verdict(asks > t0 && looks > asks && found >= looks && c >= found, 'he is asked for, then looked for, then found, then it is contact — in that order', `down @${t0.toFixed(1)} · asks @${asks} · looks @${looks} · found @${found} · contact @${c} (${(c - t0).toFixed(1)} s after the drop) · ${E.map(e => `${e.what}@${e.t}`).join(',')} · ${E.find(e => e.what === 'contact')?.why ?? ''}`);
        F.verdict(iAsk >= 0 && iGo > iAsk && iC > iGo, 'the cue is on the radio in the squad\'s name: asked for by name, "moving to his post", and only then the contact call', R.said.filter(t => /^Yard/.test(t)).join(' | '));
        F.verdict(walked > 0.4 && walked <= g.tune.guardInvestigate + 0.3, 'the men who go to look WALK (the pace of a man going to look, their boots filed)', `fastest looker ${walked.toFixed(2)} m/s over ${n} frames · walk ${g.tune.guardInvestigate} · the chase ${g.tune.guardChase}`);
        const others = ['West', 'Keep'].map(q => ({ q, e: ev(g, q).filter(e => e.t < c && e.what !== 'check') }));
        F.verdict(others.every(o => !o.e.length) && !R.said.slice(0, iC).some(t => /^(West|Keep): (?!radio check)/.test(t)), 'no other squad moves or speaks before the body is found (their own radio checks aside)', others.map(o => `${o.q}: ${o.e.map(e => e.what).join(',') || '-'}`).join(' · '));
        const b = m.char.bones.hips ?? m.char.pos; F.verdict(flatD(b, at) < 3, '(the body lay where he stood)', `${flatD(b, at).toFixed(1)} m from the spot`);
      } finally { R.done(); } }
    // ---- 3 · on, lit: a body in a mate's open view is contact by his eyes, no check needed
    { dark(false); BODY_LIGHT.lit = 0; BODY_LIGHT.litNear = 0;   // (a floor light enough everywhere: what is under test is eyes, cone and line)
      const R = await rig('compound.json', 1);
      try { const g = R.g; R.step(1); const B = bravo(g);
        // the man a mate is looking at: nearest to the line of a mate's chest
        let m = B[0], best = -2; for (const v of B) for (const x of B) { if (x === v) continue; const to = [v.char.pos[0] - x.char.pos[0], v.char.pos[2] - x.char.pos[2]], l = Math.hypot(to[0], to[1]) || 1, c = (to[0] * x.char.chestDir[0] + to[1] * x.char.chestDir[2]) / l; if (l < 10 && c > best && !g.col.segmentBlocked([x.char.pos[0], 1.6, x.char.pos[2]], [v.char.pos[0], 0.9, v.char.pos[2]])) { best = c; m = v; } }
        const t0 = g.time; killGuard(g, m, [1, 0, 0], true); until(R, 6, () => ev(g, 'Yard').some(e => e.what === 'contact'));
        const E = ev(g, 'Yard'), found = first(E, 'found'), c = first(E, 'contact');
        F.verdict(best > 0.5 && c - t0 < 3 && found <= c && !E.some(e => e.what === 'asks' || e.what === 'looks'), 'lit: a man dropped in a mate\'s view — his mate has the body in sight, contact, nobody had to ask', `${m.callsign} (cos ${best.toFixed(2)} off a mate's chest) down @${t0.toFixed(2)} · found @${found} · contact @${c} · ${E.map(e => e.what).join(',')}`);
      } finally { R.done(); } }
    // ---- 4 · on: a man taken from his post — asked for, looked for, no sign: back on their doors, nothing on the net
    { dark(true);
      const R = await rig('compound.json', 1);
      try { const g = R.g; R.step(1); const { m } = loneliest(g), t0 = g.time, home = bravo(g).length;
        const away = () => { m.held = true; m.char.pos[0] = -60; m.char.pos[2] = -60; }; away();   // (in somebody's arm, carried clean off the floor: nobody's eyes on either of them)
        until(R, 120, () => { away(); return ev(g, 'Yard').some(e => e.what === 'no sign'); });
        const E = ev(g, 'Yard'), asks = first(E, 'asks'), looks = first(E, 'looks'), none = first(E, 'no sign');
        F.verdict(asks > t0 && looks > asks && none > looks && !E.some(e => e.what === 'contact'), 'a held man is asked for, looked for, and not found: no contact', `asks @${asks} · looks @${looks} · no sign @${none} · ${E.map(e => e.what).join(',')}`);
        F.verdict(R.said.some(t => new RegExp(`^Yard: no sign of ${mate(m)} — back on our doors!`).test(t)), 'they say so on the radio', R.said.filter(t => /^Yard/.test(t)).join(' | '));
        until(R, 15, () => { away(); return false; });
        const rep = squadsReport(g), back = bravo(g).filter((x: any) => x !== m && (inRoom(g, 'garage', x.char.pos) || inRoom(g, 'shed', x.char.pos))).length;
        const men = bravo(g).filter((x: any) => x !== m);
        F.verdict(men.every((x: any) => x.state !== 'alert' && x.state !== 'search' && x.awareness < 0.5 && !x.script), 'and nobody is roused by it: they saw no one (wary at most, back on their posts)', men.map((x: any) => `${x.callsign} ${x.state} ${x.awareness.toFixed(2)}`).join(' · '));
        F.verdict(back === home - 1 && rep.find((q: any) => q.name === 'Yard').out === 0, 'the men who went are back in the squad\'s rooms', `${back} of ${home - 1} in the workshop and store · ${rep.find((q: any) => q.name === 'Yard').out} out`);
        F.verdict(['West', 'Keep'].every(q => !ev(g, q).some(e => e.what !== 'check')) && !ev(g, 'Yard').some(e => (e.what === 'asks' && e.t > none) || e.what === 'contact') && squadsReport(g).every((q: any) => q.state === 'hold'), 'no other squad is told, nobody is in contact, and he is not asked for again', ['West', 'Keep'].map(q => `${q}: ${ev(g, q).map(e => e.what).join(',') || '-'}`).join(' · '));
      } finally { R.done(); } }
    // ---- 5 · on: a loud drop is contact at once (a drop a mate heard keeps the reaction it had)
    { dark(true);
      const R = await rig('compound.json', 1);
      try { const g = R.g; R.step(1); const { m } = loneliest(g), t0 = g.time;
        g.events.push({ kind: 'shot', pos: [m.char.pos[0], 0, m.char.pos[2]], time: g.time, loud: true, level: 1, who: PLAYER_ID }); killGuard(g, m, [1, 0, 0], false); R.step(0.5);
        const c = first(ev(g, 'Yard'), 'contact');
        F.verdict(c - t0 <= 0.2, 'a man shot down LOUD in its rooms: contact at once', `down @${t0.toFixed(2)} · Yard contact @${c}`);
      } finally { R.done(); } }
  } finally { SQUADS.callCutAsk = [1, 2]; BODY_LIGHT.lit = wasLit.lit; BODY_LIGHT.litNear = wasLit.litNear; }
  return F.done();
}
