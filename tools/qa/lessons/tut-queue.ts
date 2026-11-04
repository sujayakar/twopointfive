// The scripted player for the tutorial's fourth mission (levels/tut-queue.json; tools/qa/lesson-run.ts runs it as the page opens it: a tutorial mission
// is always stood up on the shipped stand). He is on the floor with his four and has his feet, his rifle, the command key and the word; what each step
// asks goes through the input only — a walk to where the thing a line names is on the picture, the cursor on a door or a spot, the command key tapped
// with or without Shift held, the word. Game state is READ to time a press and to find where a thing is on the picture, never to do the deed.
import type { Bot } from '../lesson-run';

export const query = `?roomclear=1&squad4${process.env.TUTQUEUE_HOUR ? `&hour=${process.env.TUTQUEUE_HOUR}` : ''}`;   // (TUTQUEUE_HOUR=dusk: the level at another hour — the page's row gives the mission its own)
/** the frozen stand's arm of the gate: the ring's key, the machinery the goal rule rides on, and the rule itself, which that stand holds off */
export const raises = { TPS_QRING: '1', TPS_QPLAN: '1', TPS_GOALS: '1', TPS_GOALRING: '1', TPS_QONLY: '1', TPS_RINGFLAT: '1', TPS_RAILFLAT: '1' };
export const ended = (g: any) => g.lesson.finished;
/** `TUTQUEUE_LOOSE=1`: two stacks more than a second apart, or a teammate dazzled by the team's own can, is only said (else it fails the run) */
const LOOSE = process.env.TUTQUEUE_LOOSE === '1';
/** `split` is the way back from the wrong wheel on `gateB`: with a pair on each gate as that step ends it is done as it comes up (`TUTQUEUE_PLAIN=1` plays it) */
export const idleOk: string[] = process.env.TUTQUEUE_PLAIN ? [] : ['split'];

const O = (B: Bot) => B.g.orders;
const squad = (B: Bot): any[] => O(B).men('all');
const px = (B: Bot, p: number[]): number[] => { const [x, y] = B.W.cam.project([p[0], p[1] ?? 0, p[2]], B.W.canvas.clientWidth, B.W.canvas.clientHeight); return [x, y]; };
const stackedOn = (B: Bot, m: any): string | null => { const A = O(B).of(m); return A?.order === 'stack' && A.door ? A.door.def.name : null; };
const cleared = (B: Bot, id: string) => !!B.g.roomClear.rooms.find((r: any) => r.def.id === id)?.cleared;
const under = (B: Bot) => O(B).targetUnder(B.I, B.cam, B.W.canvas);
const names = (l: any[]) => l.map(m => m.callsign).join(', ') || 'nobody';
const stepOf = (B: Bot, id: string): any => (B.g.level.scenarios.lesson.steps as any[]).find(T => T.id === id);

/** is a place on the picture, clear of its edges and of the squad's panel on the left */
function shown(B: Bot, p: number[]): boolean { const w = B.W.canvas.clientWidth, h = B.W.canvas.clientHeight, [x, y] = px(B, p); return x > 240 && x < w - 60 && y > 70 && y < h - 110; }
/** HE WALKS UNTIL THE THING THE LINE NAMES IS ON HIS PICTURE (a hand follows the arrow at the picture's edge), and says how far that was */
function see(B: Bot, p: number[], what: string) {
  if (shown(B, p)) return;
  const s0 = B.pos(); B.go([p[0], 0, p[2]], 30, { stop: () => shown(B, p) }); B.wait(0.3);
  if (!shown(B, p)) B.fail(`${what} at (${p[0].toFixed(1)}, ${p[2].toFixed(1)}) never came onto the picture: he stands at (${B.pos()[0].toFixed(1)}, ${B.pos()[2].toFixed(1)})`);
  B.say(`  he walked ${Math.hypot(B.pos()[0] - s0[0], B.pos()[2] - s0[2]).toFixed(1)} m to have ${what} on the picture`);
}
/** the cursor on a step's own mark: the ring the line points at (the frame puts an order given on the ring on the thing itself: rails.ts markUnder) */
function onMark(B: Bot, id: string, what: string): number[] { const at = stepOf(B, id).at as number[]; see(B, at, what); B.aim(at); B.step(2); return at; }
/** the command key tapped where the cursor is, Shift held or not, as a hand presses it */
function key(B: Bot, shift: boolean) {
  if (!shift) { B.press('KeyB', 4); return; }
  B.hold(['ShiftLeft'], 3 / 60); B.I.pressed.add('KeyB'); B.hold(['ShiftLeft', 'KeyB'], 2 / 60); B.hold(['ShiftLeft'], 3 / 60); B.step(3);
}
const on = (B: Bot, door: string) => squad(B).filter(m => stackedOn(B, m) === door);
const setOn = (B: Bot, doors: string[]) => doors.every(d => on(B, d).length > 0) && squad(B).filter(m => stackedOn(B, m) !== null).every(m => O(B).of(m)?.arrived);

/** the plain key on a door: everybody stacks on it */
function all(B: Bot, id: string, door: string) {
  onMark(B, id, `the door '${door}'`); const t = under(B); if (t.kind !== 'door' || t.door !== B.door(door)) B.fail(`the cursor on the ring of step '${id}' is not on the door '${door}' (${t.kind})`);
  key(B, false); B.step(6);
  if (on(B, door).length !== squad(B).length) B.fail(`the key on '${door}' stacked ${names(on(B, door))} of ${squad(B).length}`);
}
/** Shift with the key on a second door: one pair is parted off to it and the other stays on the first */
function part(B: Bot, id: string, door: string, first: string) {
  onMark(B, id, `the door '${door}'`); const t = under(B); if (t.kind !== 'door' || t.door !== B.door(door)) B.fail(`the cursor on the ring of step '${id}' is not on the door '${door}' (${t.kind})`);
  const up0 = O(B).plan?.up; key(B, true); B.step(6);
  const a = on(B, first), b = on(B, door);
  if (O(B).plan?.up || up0) B.fail('Shift with the command key put plan mode up');
  if (a.length !== 2 || b.length !== 2 || O(B).pairOf(b[0]) !== O(B).pairOf(b[1])) B.fail(`Shift with the key on '${door}': on '${first}' ${names(a)}, on '${door}' ${names(b)} — not one whole pair on each`);
  B.say(`  Shift with the key on '${door}': ${names(b)} go to it, ${names(a)} stay on '${first}'`);
}
/** the word, once every stack stands on its door: ONE press, and men are through every door of it together */
function word(B: Bot, doors: string[], room: string) {
  if (!B.until(() => setOn(B, doors) || cleared(B, room), 70)) B.say('  the stacks never all stood on their doors: the word anyway');
  if (cleared(B, room)) B.fail(`${room} fell before the word: a stack went in without it, and nothing of the word could be measured`);
  if (B.g.lesson.step?.id !== (room === 'hall' ? 'word2' : 'word')) B.until(() => B.g.lesson.step?.id === (room === 'hall' ? 'word2' : 'word'), 12);
  B.wait(0.4);
  const stacks: Record<string, any[]> = {}; for (const d of doors) stacks[d] = on(B, d);
  B.press('KeyF', 3); B.step(6);
  const t0 = B.t(), inAt: Record<string, number> = {}, daz0 = new Map(squad(B).map(m => [m, m.dazzledUntil as number])), hit = new Set<string>();
  const R = B.g.roomClear.rooms.find((r: any) => r.def.id === room)?.def.bounds, inside = (m: any) => !!R && m.char.pos[0] > R.x0 + 0.2 && m.char.pos[0] < R.x1 - 0.2 && m.char.pos[2] > R.z0 + 0.2 && m.char.pos[2] < R.z1 - 0.2;
  const watch = () => { for (const d of doors) if (!(d in inAt) && (stacks[d] ?? []).some(inside)) inAt[d] = B.t() - t0; for (const [m, u] of daz0) if (m.dazzledUntil > u + 1e-6) hit.add(m.callsign); return false; };
  B.until(() => watch() || cleared(B, room), 40); for (let i = 0; i < 180; i++) { watch(); B.step(); }
  const ts = doors.map(d => inAt[d]), out = doors.filter((_, i) => ts[i] === undefined), apart = doors.length > 1 && !out.length ? Math.max(...ts) - Math.min(...ts) : NaN;
  const said = `the word at ${room}: ${doors.map((d, i) => `${d} ${ts[i] === undefined ? 'NOBODY IN' : `first man in at ${ts[i].toFixed(1)} s`}`).join(' · ')}${doors.length < 2 ? '' : out.length ? ' · not together: a stack never went in' : ` · ${apart.toFixed(1)} s apart`} · teammates dazzled as they went in: ${hit.size}${hit.size ? ` (${[...hit].join(', ')})` : ''}`;
  if ((out.length || (!LOOSE && (apart > 1 || hit.size)))) B.fail(said);
  B.say(`  ${said}`);
  for (let k = 0; k < 2 && !cleared(B, room); k++) { B.wait(0.6); B.press('KeyF', 3); B.until(() => cleared(B, room), 40); }
}
/** who watches what: the men on a cover order, by the point each is turned to */
function watches(B: Bot): string { const by = new Map<string, string[]>(); for (const m of squad(B)) { const A = O(B).of(m); const k = A?.order === 'cover' && A.watch ? `(${A.watch[0].toFixed(1)}, ${A.watch[2].toFixed(1)})` : (A?.order ?? 'no order'); by.set(k, [...(by.get(k) ?? []), m.callsign]); } return [...by].map(([k, l]) => `${l.join(', ')} → ${k}`).join(' · '); }
/** what the hall cost, said: the script's own notes for it and the teammates the frame spared a down */
function told(B: Bot, n0: number) {
  const notes = (B.g.director.log as { kind: string; what: string }[]).filter(r => r.kind === 'note' && /hall|stack|gun/.test(r.what) && !r.what.startsWith('signal:lesson')).map(r => r.what);
  B.say(`  the hall: ${notes.join(' · ') || '(no note)'} · teammates put on a knee: ${(B.g.rails.log as string[]).slice(n0).filter(l => l.includes('spared')).length}`);
}

/** he walks to a step's ring, and both things the next two lines name are on his picture from it with the view at rest (else the spot is the level's to move) */
function spot(B: Bot, id: string, next: string[], via: number[][] = []) {
  const at = stepOf(B, id).at as number[]; for (const v of via) B.go(v, 30, { noDoors: true, near: 0.6 }); B.go(at, 40, { noDoors: true }); B.until(() => B.g.lesson.step?.id !== id, 6);
  B.aim(B.pos()); B.wait(0.5);
  const off = next.filter(n => !shown(B, stepOf(B, n).at)); if (off.length) B.fail(`from the ring of '${id}' the mark of ${off.map(n => `'${n}'`).join(' and ')} is off the picture: ${off.map(n => px(B, stepOf(B, n).at).map(v => v.toFixed(0)).join(',')).join(' · ')}`);
  B.say(`  at the ring of '${id}' (${B.pos()[0].toFixed(1)}, ${B.pos()[2].toFixed(1)}): ${next.map(n => `'${n}' at px ${px(B, stepOf(B, n).at).map(v => v.toFixed(0)).join(',')}`).join(' · ')}`);
}
export const steps: Record<string, (B: Bot) => void> = {
  corner: B => { spot(B, 'corner', ['door1', 'door2']); },
  door1: B => { all(B, 'door1', 'guard'); if (process.env.TUTQUEUE_TWICE) { B.until(() => setOn(B, ['guard']), 40); B.wait(1); onMark(B, 'door1', 'guard'); key(B, false); B.wait(3); B.say(`  the key again on a stack that stands: ${cleared(B, 'guardroom') ? 'THEY WENT IN' : 'they wait'} (${squad(B).map(m => O(B).of(m)?.order).join(' ')})`); } },
  door2: B => { part(B, 'door2', 'guard_side', 'guard'); B.until(() => B.g.lesson.step?.id !== 'door2', 60); },
  word: B => { word(B, ['guard', 'guard_side'], 'guardroom'); B.wait(1.5); },
  front: B => { all(B, 'front', 'hall'); },
  around: B => { spot(B, 'around', ['back', 'word2'], [[23.3, 0, 13.4], [23.3, 0, 20.4]]); },   // (round the hall's blind west wall: a body walked into the front door opens it, and the gun is on that doorway)
  back: B => { part(B, 'back', 'hall_back', 'hall'); B.until(() => B.g.lesson.step?.id !== 'back', 70); },
  word2: B => { const n0 = B.g.rails.log.length; word(B, ['hall', 'hall_back'], 'hall'); told(B, n0); B.wait(1.5); },
  call: B => { B.say(`  as the hall falls: ${watches(B)}`); B.aim(B.pos()); B.step(2); const t = under(B); key(B, false); B.step(6); B.say(`  the key on himself (${t.kind}${t.man ? ' a man' : ''}): ${watches(B)}`); },
  yard: B => { spot(B, 'yard', ['gateA', 'gateB'], process.env.TUTQUEUE_WEST ? [[23.3, 0, 20.4], [23.3, 0, 13.0]] : []); B.wait(Number(process.env.TUTQUEUE_WAIT ?? 2));
    for (const m of squad(B)) B.say(`    ${m.callsign} (${m.char.pos[0].toFixed(1)}, ${m.char.pos[2].toFixed(1)}) line to the road gate's spot ${O(B).hasLine(m, { kind: 'floor', at: stepOf(B, 'gateA').at })} · the back gate's ${O(B).hasLine(m, { kind: 'floor', at: stepOf(B, 'gateB').at })}`); },
  gateA: B => { onMark(B, 'gateA', 'the road gate'); key(B, true); B.step(10); B.say(`  Shift with the key by the road gate: ${watches(B)}`); },
  // (the wheel for the part: a Shift tap gives nothing on this step; the pair kept down past the hold is the ring, named for the men a tap would have sent, and the wedge picked is theirs)
  gateB: B => {
    // (`TUTQUEUE_PLAIN=1`, the wrong wheel: the key kept down with NO Shift is everyone's, and its second wedge turns all four to this gate — the step goes by all the same and `split` puts it right)
    if (process.env.TUTQUEUE_PLAIN) { onMark(B, 'gateB', 'the back gate'); B.I.pressed.add('KeyB'); B.hold(['KeyB'], 0.7); const M = O(B).menu; if (!M || M.part) B.fail(`the key kept down with no Shift opened ${M ? 'the ring for a part' : 'no ring'}`);
      B.I.pressed.add('Digit2'); B.hold(['KeyB', 'Digit2'], 2 / 60); B.hold(['KeyB'], 3 / 60); B.step(10); B.say(`  the key kept down with NO Shift, its second wedge: ${watches(B)}`); return; }
    onMark(B, 'gateB', 'the back gate'); const before = watches(B); key(B, true); B.step(10); const note = B.g.rails.prompt()?.note ?? '';
    if (watches(B) !== before || !/held down/.test(note)) B.fail(`a Shift tap on the step that teaches the key kept down: ${watches(B)} · the note "${note}"`);
    onMark(B, 'gateB', 'the back gate'); B.hold(['ShiftLeft'], 3 / 60); B.I.pressed.add('KeyB'); B.hold(['ShiftLeft', 'KeyB'], 0.7); const M = O(B).menu;
    if (!M?.part) B.fail(`Shift and the key kept down by the back gate opened ${M ? 'the ring for everyone' : 'no ring'}`);
    const line = B.g.rails.prompt()?.line ?? ''; if (!/^Press/.test(line)) B.fail(`the ring is up on the step's spot and the line is still "${line}"`);
    B.say(`  a Shift tap there: nothing, and "${note}" · kept down, the ring by the back gate: for ${M.men.map((m: any) => m.callsign).join(', ')} · rows ${M.rows.map((r: any) => r.id).join(', ')} · the line "${line}"`);
    B.I.pressed.add('Digit2'); B.hold(['ShiftLeft', 'KeyB', 'Digit2'], 2 / 60); B.hold(['ShiftLeft', 'KeyB'], 3 / 60); B.step(10); B.say(`  its second wedge: ${watches(B)}`); },
  split: B => { if (!process.env.TUTQUEUE_PLAIN) return; if (B.g.lesson.step?.id !== 'split') B.fail(`all four on the back gate and the step that is up is '${B.g.lesson.step?.id}'`);
    onMark(B, 'split', 'the road gate'); key(B, true); B.step(10); B.say(`  Shift with the key by the road gate again: ${watches(B)}`); },
  hold: B => { B.until(() => (cleared(B, 'road') && cleared(B, 'lane')) || B.g.lesson.finished, 120); B.wait(1); },
};
