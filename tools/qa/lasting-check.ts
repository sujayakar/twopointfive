// WHAT SOUNDS FOR AS LONG AS IT GOES ON (src/audio/audio.ts lasting): the mixer over a stand-in graph. One looping voice a thing, held while its row is told and let go when it is not; its level by how it goes; silent
// while the game's clock stands; the nearest of them where there are more than it holds; and a jump heard as it stands.
//   bun run tools/qa/lasting-check.ts            exit 0 = every check passed, 1 = one failed · LASTING_BREAK=1 = a row gone is never told: must FAIL
import { AudioEngine, type LastingSound, type SoundAsk } from '../../src/audio/audio';
import { sfxContext, type AudioBackend, type BackendVoice } from '../../src/audio/backend';
import { HUMS } from '../../src/audio/loops';
import { readFileSync } from 'node:fs';

const BREAK = process.env.LASTING_BREAK === '1';
let red = 0; const check = (ok: boolean, what: string, got?: unknown) => { console.log(`${ok ? '- PASS' : '- FAIL'} — ${what}${got === undefined ? '' : ` (${JSON.stringify(got)})`}`); if (!ok) red++; };
interface V { buffer: string; loop: boolean; gain: number; rate: number; stopped: boolean; released: boolean; pos: number[] }
let now = 1, clock = 10; const voices: V[] = [], lengths = new Map<string, number>(), asks: SoundAsk[] = [];
const graph: AudioBackend = { sampleRate: 24000, now: () => now, sfxContext: sfxContext(24000), loadBuffer: (id, b) => { lengths.set(id, b.duration); }, duration: id => lengths.get(id) ?? 0,
  play: o => { const v: V = { buffer: o.buffer, loop: !!o.loop, gain: o.gain, rate: o.rate, stopped: false, released: false, pos: [...(o.pos ?? [])] }; voices.push(v); return v as unknown as BackendVoice; },
  setVoice: (v, p) => { const w = v as unknown as V; if (p.pos) w.pos = [...p.pos]; if (p.gain !== undefined) w.gain = p.gain; if (p.rate !== undefined) w.rate = p.rate; }, stop: v => { (v as unknown as V).stopped = true; }, release: v => { (v as unknown as V).released = true; },
  setListener() {}, setBus() {}, setReverb() {}, rainLevel() {} };
const ask = { hear: (p: number[]) => ({ apparent: p, muffle: 0, pathLen: 1, routed: false }), ray: () => null, together: (f: () => void) => f(), blocked: () => false };
const A = new AudioEngine(ask as never, () => graph); A.start(); A.setListener([0, 1.6, 0], [0, 0, -1]); A.tap = a => { asks.push(a); };
const can = (age: number, o: Partial<LastingSound> = {}): LastingSound => ({ name: 'smokeHiss', id: 7, pos: [2, 0.1, 0], gain: 1, hard: 1, age, left: 32 - age, reach: 0, ...o });
const chair = (hard: number, id = 1, x = 3): LastingSound => ({ name: 'chairRoll', id, pos: [x, 0, 0], gain: 0.5, hard, age: Infinity, left: Infinity, reach: 0 });
let last: readonly LastingSound[] = [];
const tick = (rows: readonly LastingSound[], secs = 1 / 60) => { now += secs; clock += secs; if (BREAK) rows = [...rows, ...last.filter(r => !rows.some(q => q.name === r.name && q.id === r.id))]; last = rows; A.lasting(rows, clock); A.update(secs, now); };
const held = () => voices.filter(v => !v.stopped && v.loop);

tick([can(0.1)]); tick([can(0.12)]);
check(held().length === 1 && held()[0].buffer === 'lasting:smokeHiss' && held()[0].gain > 0 && held()[0].gain < 0.3, 'a can that has begun to pour is one voice, coming up', held());
for (let i = 0; i < 30; i++) tick([can(0.5 + i / 60)]);
check(held().length === 1 && Math.abs(held()[0].gain - 0.5) < 1e-6, 'it is held at its level', held()[0]?.gain);
check(asks.filter(a => a.name === 'smokeHiss').length === 1 && asks[0].heard === 2 && Math.abs(asks[0].dur - 31.9) < 1e-6, 'the tap is told once, with what it has left', asks);
now += 0.5; A.update(0.5, now); check(held()[0].gain === 0, 'the clock stands, and it is silent');
tick([can(2)]); check(held()[0]?.gain > 0.4, 'the clock goes on, and it is heard');
A.hush(); check(held()[0]?.gain === 0 && !held()[0].stopped, 'a jump hushes it'); tick([can(12)]); check(held().length === 1 && held()[0].gain > 0.4, 'and the same can goes on in the same voice');
tick([can(31.5)]); check(held()[0]?.gain > 0 && held()[0].gain < 0.2, 'its last seconds die away', held()[0]?.gain);
tick([]); check(held().length === 0, 'a moment with no can has no hiss');
now += 2; tick([]); check(voices.filter(v => v.buffer === 'lasting:smokeHiss').every(v => v.released), 'and its voice is given back');
tick([can(20)]); tick([can(20.02)]); check(held().length === 1 && held()[0].gain > 0.4, 'a moment with a can half burnt is heard at once at its level');
tick([chair(1)]); tick([chair(1)]); const fast = { ...held().find(v => v.buffer === 'chairRoll')! }; tick([chair(0.2)]); const slow = { ...held().find(v => v.buffer === 'chairRoll')! };
check(fast.gain > slow.gain && fast.rate > slow.rate && slow.gain > 0, 'a chair is quieter and lower as it slows', { fast: fast.gain, slow: slow.gain });
tick([chair(0)]); check(held()[0].gain === 0, 'and at rest makes no sound');
tick([]); now += 2; tick([]);
const many = Array.from({ length: 12 }, (_, i) => chair(1, 100 + i, 2 + i)); tick(many); tick(many);
check(held().length === 8 && Math.max(...held().map(v => v.pos[0])) === 9, 'of twelve the nearest eight are held', held().map(v => v.pos[0]));
const near = [...many, chair(1, 200, 1)]; tick(near); now += 2; tick(near); tick(near);
check(held().length === 8 && held().some(v => v.pos[0] === 1) && !held().some(v => v.pos[0] === 9), 'a nearer one takes the farthest one\'s place', held().map(v => v.pos[0]));
A.enabled = false; tick([]); now += 2; asks.length = 0; tick([can(3, { id: 9 })]); tick([can(3.02, { id: 9 })]);
check(held().length === 0 && asks.length === 1 && asks[0].heard === 0, 'a mixer put off plays none of it and tells the tap once', asks);
A.enabled = true; tick([can(3.1, { id: 9 })]); tick([can(3.12, { id: 9 })]); check(held().length === 1, 'put on again, what goes on is heard');
check(asks.length === 2 && asks[1].heard === 2, 'and the tap is told it is heard now', asks.map(a => a.heard));
// another moment of the game, with no time gone by in it
clock += 100; A.lasting([can(20, { id: 9, pos: [9, 0, 0] })], clock); A.hush(); now += 1 / 60; A.update(0, now); check(held().every(v => v.gain === 0), 'a jump under a card is silent', held().map(v => v.gain));
tick([can(20.02, { id: 9, pos: [9, 0, 0] })]); check(held()[0].gain > 0.4 && held()[0].pos[0] === 9, 'and is heard as the clock goes on, from where it is', held()[0]);
// a fire far off, walked up to: told again as it is heard
tick([]); now += 2; tick([]); asks.length = 0; const fire = (x: number): LastingSound => ({ name: 'fireBed', id: 0, pos: [x, 0, 0], gain: 0.85, hard: 2 / 3, age: Infinity, left: Infinity, reach: 0 });
tick([fire(200)]); tick([fire(200)]); tick([fire(5)]); tick([fire(5)]); check(asks.map(a => a.heard).join() === '1,2' && held().length === 1, 'a fire out of hearing is told as culled, and again as it is heard', asks.map(a => a.heard));
// a room's hum: as far as it reaches, under a card too, and no lane of it
tick([]); now += 2; tick([]); asks.length = 0; const hum: LastingSound = { name: 'servers', id: 0, pos: [0, 1.6, 6], gain: 0.5, hard: 1, age: Infinity, left: Infinity, reach: 12 };
tick([hum]); tick([hum]); now += 5; A.update(0, now); check(held().length === 1 && Math.abs(held()[0].gain - 0.25) < 1e-6 && !asks.length, 'half way to its reach a room hums at half, and goes on under a card', held().map(v => v.gain));
await new Promise(r => setTimeout(r, 1000)); check(held()[0].gain === 0, 'no frame comes, and nothing sounds on');
{ const office = JSON.parse(readFileSync(new URL('../../levels/office.json', import.meta.url), 'utf8')) as { hums: { kind: string; level: number }[] };
  check(HUMS.every(h => office.hums.find(o => o.kind === h.kind)?.level === h.base), 'the listening page plays each hum at the office\'s level', HUMS.map(h => [h.kind, h.base])); }
console.log(red ? `${red} FAILED` : 'all passed');
process.exit(red ? 1 : 0);
