// THE VIEW, READ: the types of rust/game_types/src/view and their bytes, as plain values. Written by tools/wire (gen) from the table; never edited by hand. It imports nothing of the game:
// a page that holds no game reads the same record.
import type { Vec3 } from '../math/vec';

class In { p = 0; dv: DataView; constructor(readonly b: Uint8Array) { this.dv = new DataView(b.buffer, b.byteOffset, b.byteLength); }
  float64(): number { const v = this.dv.getFloat64(this.p, true); this.p += 8; return v; }
  float32(): number { const v = this.dv.getFloat32(this.p, true); this.p += 4; return v; }
  uint32(): number { const v = this.dv.getUint32(this.p, true); this.p += 4; return v; }
  int32(): number { const v = this.dv.getInt32(this.p, true); this.p += 4; return v; }
  uint16(): number { const v = this.dv.getUint16(this.p, true); this.p += 2; return v; }
  int16(): number { const v = this.dv.getInt16(this.p, true); this.p += 2; return v; }
  uint8(): number { const v = this.dv.getUint8(this.p); this.p += 1; return v; }
  int8(): number { const v = this.dv.getInt8(this.p); this.p += 1; return v; }
  bool(): boolean { return this.b[this.p++] !== 0; }
  str(): string { const n = this.uint32(), s = DEC.decode(this.b.subarray(this.p, this.p + n)); this.p += n; return s; }
  list<T>(f: () => T): T[] { const n = this.uint32(), o: T[] = []; for (let i = 0; i < n; i++) o.push(f()); return o; }
  word<T>(W: readonly T[], of: string): T { const k = this.uint16(); if (k >= W.length) throw new Error(`the view: no such word of ${of} (${k})`); return W[k]; } }
const DEC = new TextDecoder();

export type MissionOf = 'stages' | 'script' | 'lesson' | 'defuse' | 'roomclear' | 'rescue' | 'nightbreach';
const MissionOfWords: readonly MissionOf[] = ['stages', 'script', 'lesson', 'defuse', 'roomclear', 'rescue', 'nightbreach'];
function readMissionOfIn(I: In): MissionOf { return I.word(MissionOfWords, 'MissionOf'); }
export interface MissionView { of: MissionOf; text: string; pos: Vec3 | null }
function readMissionViewIn(I: In): MissionView { return { of: readMissionOfIn(I), text: I.str(), pos: I.bool() ? [I.float64(), I.float64(), I.float64()] : null }; }
export function readMissionView(b: Uint8Array): MissionView { const I = new In(b), r = readMissionViewIn(I); if (I.p !== b.length) throw new Error(`the view: MissionView is ${b.length} bytes and ${I.p} were read`); return r; }
export type RadioKind = 'net' | 'heard';
const RadioKindWords: readonly RadioKind[] = ['net', 'heard'];
function readRadioKindIn(I: In): RadioKind { return I.word(RadioKindWords, 'RadioKind'); }
export interface RadioRow { kind: RadioKind; text: string; age: number }
function readRadioRowIn(I: In): RadioRow { return { kind: readRadioKindIn(I), text: I.str(), age: I.float64() }; }
export function readRadioRow(b: Uint8Array): RadioRow { const I = new In(b), r = readRadioRowIn(I); if (I.p !== b.length) throw new Error(`the view: RadioRow is ${b.length} bytes and ${I.p} were read`); return r; }
export type BubbleTone = 'calm' | 'sus' | 'alert' | 'own';
const BubbleToneWords: readonly BubbleTone[] = ['calm', 'sus', 'alert', 'own'];
function readBubbleToneIn(I: In): BubbleTone { return I.word(BubbleToneWords, 'BubbleTone'); }
export type BubblePlace = 'over' | 'overOrEdge' | 'edge';
const BubblePlaceWords: readonly BubblePlace[] = ['over', 'overOrEdge', 'edge'];
function readBubblePlaceIn(I: In): BubblePlace { return I.word(BubblePlaceWords, 'BubblePlace'); }
export interface BubbleView { man: number | null; text: string; edge: string; age: number; dur: number; radio: boolean; tone: BubbleTone; place: BubblePlace; at: Vec3 }
function readBubbleViewIn(I: In): BubbleView { return { man: I.bool() ? I.float64() : null, text: I.str(), edge: I.str(), age: I.float64(), dur: I.float64(), radio: I.bool(), tone: readBubbleToneIn(I), place: readBubblePlaceIn(I), at: [I.float64(), I.float64(), I.float64()] }; }
export function readBubbleView(b: Uint8Array): BubbleView { const I = new In(b), r = readBubbleViewIn(I); if (I.p !== b.length) throw new Error(`the view: BubbleView is ${b.length} bytes and ${I.p} were read`); return r; }
export interface TickView { mission: MissionView | null; radio: RadioRow[]; bubbles: BubbleView[]; me: number; place: string }
function readTickViewIn(I: In): TickView { return { mission: I.bool() ? readMissionViewIn(I) : null, radio: I.list(() => readRadioRowIn(I)), bubbles: I.list(() => readBubbleViewIn(I)), me: I.float64(), place: I.str() }; }
export function readTickView(b: Uint8Array): TickView { const I = new In(b), r = readTickViewIn(I); if (I.p !== b.length) throw new Error(`the view: TickView is ${b.length} bytes and ${I.p} were read`); return r; }
export type UnderKind = 'floor' | 'door' | 'none' | 'foe';
const UnderKindWords: readonly UnderKind[] = ['floor', 'door', 'none', 'foe'];
function readUnderKindIn(I: In): UnderKind { return I.word(UnderKindWords, 'UnderKind'); }
export interface UnderWall { index: number; alongX: boolean; u: number; side: number; ok: boolean; why: string; outer: boolean }
function readUnderWallIn(I: In): UnderWall { return { index: I.float64(), alongX: I.bool(), u: I.float64(), side: I.float64(), ok: I.bool(), why: I.str(), outer: I.bool() }; }
export function readUnderWall(b: Uint8Array): UnderWall { const I = new In(b), r = readUnderWallIn(I); if (I.p !== b.length) throw new Error(`the view: UnderWall is ${b.length} bytes and ${I.p} were read`); return r; }
export type WorkThingKind = 'breaker' | 'device' | 'objective';
const WorkThingKindWords: readonly WorkThingKind[] = ['breaker', 'device', 'objective'];
function readWorkThingKindIn(I: In): WorkThingKind { return I.word(WorkThingKindWords, 'WorkThingKind'); }
export interface WorkThing { kind: WorkThingKind; face: Vec3; n: Vec3; stand: Vec3 | null; name: string; dev: number | null }
function readWorkThingIn(I: In): WorkThing { return { kind: readWorkThingKindIn(I), face: [I.float64(), I.float64(), I.float64()], n: [I.float64(), I.float64(), I.float64()], stand: I.bool() ? [I.float64(), I.float64(), I.float64()] : null, name: I.str(), dev: I.bool() ? I.float64() : null }; }
export function readWorkThing(b: Uint8Array): WorkThing { const I = new In(b), r = readWorkThingIn(I); if (I.p !== b.length) throw new Error(`the view: WorkThing is ${b.length} bytes and ${I.p} were read`); return r; }
export interface Under { kind: UnderKind; at: Vec3; door: string | null; side: number; man: number; wall: UnderWall | null; thing: WorkThing | null }
function readUnderIn(I: In): Under { return { kind: readUnderKindIn(I), at: [I.float64(), I.float64(), I.float64()], door: I.bool() ? I.str() : null, side: I.float64(), man: I.float64(), wall: I.bool() ? readUnderWallIn(I) : null, thing: I.bool() ? readWorkThingIn(I) : null }; }
export function readUnder(b: Uint8Array): Under { const I = new In(b), r = readUnderIn(I); if (I.p !== b.length) throw new Error(`the view: Under is ${b.length} bytes and ${I.p} were read`); return r; }
export interface CursorView { under: Under | null }
function readCursorViewIn(I: In): CursorView { return { under: I.bool() ? readUnderIn(I) : null }; }
export function readCursorView(b: Uint8Array): CursorView { const I = new In(b), r = readCursorViewIn(I); if (I.p !== b.length) throw new Error(`the view: CursorView is ${b.length} bytes and ${I.p} were read`); return r; }
export type ChronKind = 'down' | 'shot' | 'blackout' | 'heard' | 'objective' | 'grab' | 'ocp' | 'thrown' | 'radio' | 'spotted' | 'contact' | 'lost' | 'calm' | 'alarm' | 'lightShot' | 'interrogated' | 'keyTaken' | 'ko' | 'kill' | 'bodyFound' | 'doorKicked' | 'lockPicked' | 'hit' | 'needBreak' | 'civilianSpotted' | 'civilianReported' | 'civilianComplied' | 'civilianKO' | 'civilianKilled' | 'cameraSpotted' | 'cameraAlarm' | 'laserTripped' | 'deviceDown' | 'codeLearned' | 'codeEntered' | 'checkpoint';
const ChronKindWords: readonly ChronKind[] = ['down', 'shot', 'blackout', 'heard', 'objective', 'grab', 'ocp', 'thrown', 'radio', 'spotted', 'contact', 'lost', 'calm', 'alarm', 'lightShot', 'interrogated', 'keyTaken', 'ko', 'kill', 'bodyFound', 'doorKicked', 'lockPicked', 'hit', 'needBreak', 'civilianSpotted', 'civilianReported', 'civilianComplied', 'civilianKO', 'civilianKilled', 'cameraSpotted', 'cameraAlarm', 'laserTripped', 'deviceDown', 'codeLearned', 'codeEntered', 'checkpoint'];
function readChronKindIn(I: In): ChronKind { return I.word(ChronKindWords, 'ChronKind'); }
export interface DebriefRow { t: number; kind: ChronKind; room: string; pos: Vec3; text: string; down: boolean }
function readDebriefRowIn(I: In): DebriefRow { return { t: I.float64(), kind: readChronKindIn(I), room: I.str(), pos: [I.float64(), I.float64(), I.float64()], text: I.str(), down: I.bool() }; }
export function readDebriefRow(b: Uint8Array): DebriefRow { const I = new In(b), r = readDebriefRowIn(I); if (I.p !== b.length) throw new Error(`the view: DebriefRow is ${b.length} bytes and ${I.p} were read`); return r; }
export interface DebriefRoom { name: string; label: string; x0: number; x1: number; z0: number; z1: number }
function readDebriefRoomIn(I: In): DebriefRoom { return { name: I.str(), label: I.str(), x0: I.float64(), x1: I.float64(), z0: I.float64(), z1: I.float64() }; }
export function readDebriefRoom(b: Uint8Array): DebriefRoom { const I = new In(b), r = readDebriefRoomIn(I); if (I.p !== b.length) throw new Error(`the view: DebriefRoom is ${b.length} bytes and ${I.p} were read`); return r; }
export interface RunCounts { elapsed: number; stage: string; done: boolean; failed: boolean; sandbox: boolean; blackout: boolean; seen: number; seenUp: number; alarms: number; esc1: number; esc2: number; bodies: number; kills: number; kos: number; shots: number; shotsNearMen: number; lightsShot: number; ocps: number; kicked: number; picked: number; keys: number; intel: number; questioned: number; roster: number; hits: number; down: boolean; loads: number; thrown: number; grabs: number; dropped: number; devices: number; civSeen: number; civReported: number; civComplied: number; civKo: number; civKilled: number }
function readRunCountsIn(I: In): RunCounts { return { elapsed: I.float64(), stage: I.str(), done: I.bool(), failed: I.bool(), sandbox: I.bool(), blackout: I.bool(), seen: I.float64(), seenUp: I.float64(), alarms: I.float64(), esc1: I.float64(), esc2: I.float64(), bodies: I.float64(), kills: I.float64(), kos: I.float64(), shots: I.float64(), shotsNearMen: I.float64(), lightsShot: I.float64(), ocps: I.float64(), kicked: I.float64(), picked: I.float64(), keys: I.float64(), intel: I.float64(), questioned: I.float64(), roster: I.float64(), hits: I.float64(), down: I.bool(), loads: I.float64(), thrown: I.float64(), grabs: I.float64(), dropped: I.float64(), devices: I.float64(), civSeen: I.float64(), civReported: I.float64(), civComplied: I.float64(), civKo: I.float64(), civKilled: I.float64() }; }
export function readRunCounts(b: Uint8Array): RunCounts { const I = new In(b), r = readRunCountsIn(I); if (I.p !== b.length) throw new Error(`the view: RunCounts is ${b.length} bytes and ${I.p} were read`); return r; }
export interface ScoreLine { label: string; pts: number }
function readScoreLineIn(I: In): ScoreLine { return { label: I.str(), pts: I.float64() }; }
export function readScoreLine(b: Uint8Array): ScoreLine { const I = new In(b), r = readScoreLineIn(I); if (I.p !== b.length) throw new Error(`the view: ScoreLine is ${b.length} bytes and ${I.p} were read`); return r; }
export interface Badge { id: string; name: string; on: boolean; hint: string }
function readBadgeIn(I: In): Badge { return { id: I.str(), name: I.str(), on: I.bool(), hint: I.str() }; }
export function readBadge(b: Uint8Array): Badge { const I = new In(b), r = readBadgeIn(I); if (I.p !== b.length) throw new Error(`the view: Badge is ${b.length} bytes and ${I.p} were read`); return r; }
export type DebriefState = 'down' | 'done' | 'sofar';
const DebriefStateWords: readonly DebriefState[] = ['down', 'done', 'sofar'];
function readDebriefStateIn(I: In): DebriefState { return I.word(DebriefStateWords, 'DebriefState'); }
export interface Debrief { counts: RunCounts; score: number; lines: ScoreLine[]; rating: string; blurb: string; badges: Badge[]; summary: string; state: DebriefState; scored: boolean; grade: string }
function readDebriefIn(I: In): Debrief { return { counts: readRunCountsIn(I), score: I.float64(), lines: I.list(() => readScoreLineIn(I)), rating: I.str(), blurb: I.str(), badges: I.list(() => readBadgeIn(I)), summary: I.str(), state: readDebriefStateIn(I), scored: I.bool(), grade: I.str() }; }
export function readDebrief(b: Uint8Array): Debrief { const I = new In(b), r = readDebriefIn(I); if (I.p !== b.length) throw new Error(`the view: Debrief is ${b.length} bytes and ${I.p} were read`); return r; }
export type RecapCause = 'blast' | 'round' | 'other' | 'unknown';
const RecapCauseWords: readonly RecapCause[] = ['blast', 'round', 'other', 'unknown'];
function readRecapCauseIn(I: In): RecapCause { return I.word(RecapCauseWords, 'RecapCause'); }
export interface RecapDeath { t: number; victim: number; name: string; side: string; player: boolean; at: [number, number]; gaze: number; cause: RecapCause; by: number; byName: string; bySide: string; byAt: [number, number] | null; from: [number, number] | null; to: [number, number]; weapon: string; dist: number; off: number; blind: boolean; loud: boolean }
function readRecapDeathIn(I: In): RecapDeath { return { t: I.float64(), victim: I.float64(), name: I.str(), side: I.str(), player: I.bool(), at: [I.float64(), I.float64()], gaze: I.float64(), cause: readRecapCauseIn(I), by: I.float64(), byName: I.str(), bySide: I.str(), byAt: I.bool() ? [I.float64(), I.float64()] : null, from: I.bool() ? [I.float64(), I.float64()] : null, to: [I.float64(), I.float64()], weapon: I.str(), dist: I.float64(), off: I.float64(), blind: I.bool(), loud: I.bool() }; }
export function readRecapDeath(b: Uint8Array): RecapDeath { const I = new In(b), r = readRecapDeathIn(I); if (I.p !== b.length) throw new Error(`the view: RecapDeath is ${b.length} bytes and ${I.p} were read`); return r; }
export type RecapManKind = 'player' | 'civ' | 'man';
const RecapManKindWords: readonly RecapManKind[] = ['player', 'civ', 'man'];
function readRecapManKindIn(I: In): RecapManKind { return I.word(RecapManKindWords, 'RecapManKind'); }
export interface RecapMan { id: number; name: string; side: string; kind: RecapManKind }
function readRecapManIn(I: In): RecapMan { return { id: I.float64(), name: I.str(), side: I.str(), kind: readRecapManKindIn(I) }; }
export function readRecapMan(b: Uint8Array): RecapMan { const I = new In(b), r = readRecapManIn(I); if (I.p !== b.length) throw new Error(`the view: RecapMan is ${b.length} bytes and ${I.p} were read`); return r; }
export interface RecapDoor { hinge: [number, number]; width: number }
function readRecapDoorIn(I: In): RecapDoor { return { hinge: [I.float64(), I.float64()], width: I.float64() }; }
export function readRecapDoor(b: Uint8Array): RecapDoor { const I = new In(b), r = readRecapDoorIn(I); if (I.p !== b.length) throw new Error(`the view: RecapDoor is ${b.length} bytes and ${I.p} were read`); return r; }
export interface RecapFrame { t: number; n: number; men: number[]; doors: number[] }
function readRecapFrameIn(I: In): RecapFrame { return { t: I.float64(), n: I.float64(), men: I.list(() => I.float32()), doors: I.list(() => I.float32()) }; }
export function readRecapFrame(b: Uint8Array): RecapFrame { const I = new In(b), r = readRecapFrameIn(I); if (I.p !== b.length) throw new Error(`the view: RecapFrame is ${b.length} bytes and ${I.p} were read`); return r; }
export type RecapShotKind = 'blast' | 'bang' | 'round';
const RecapShotKindWords: readonly RecapShotKind[] = ['blast', 'bang', 'round'];
function readRecapShotKindIn(I: In): RecapShotKind { return I.word(RecapShotKindWords, 'RecapShotKind'); }
export interface RecapShot { t: number; kind: RecapShotKind; who: number; from: [number, number]; to: [number, number] | null; at: [number, number] | null; weapon: string | null; loud: boolean | null }
function readRecapShotIn(I: In): RecapShot { return { t: I.float64(), kind: readRecapShotKindIn(I), who: I.float64(), from: [I.float64(), I.float64()], to: I.bool() ? [I.float64(), I.float64()] : null, at: I.bool() ? [I.float64(), I.float64()] : null, weapon: I.bool() ? I.str() : null, loud: I.bool() ? I.bool() : null }; }
export function readRecapShot(b: Uint8Array): RecapShot { const I = new In(b), r = readRecapShotIn(I); if (I.p !== b.length) throw new Error(`the view: RecapShot is ${b.length} bytes and ${I.p} were read`); return r; }
export interface RecapTake { death: RecapDeath; t0: number; t1: number; cast: RecapMan[]; doors: RecapDoor[]; frames: RecapFrame[]; shots: RecapShot[]; deaths: RecapDeath[] }
function readRecapTakeIn(I: In): RecapTake { return { death: readRecapDeathIn(I), t0: I.float64(), t1: I.float64(), cast: I.list(() => readRecapManIn(I)), doors: I.list(() => readRecapDoorIn(I)), frames: I.list(() => readRecapFrameIn(I)), shots: I.list(() => readRecapShotIn(I)), deaths: I.list(() => readRecapDeathIn(I)) }; }
export function readRecapTake(b: Uint8Array): RecapTake { const I = new In(b), r = readRecapTakeIn(I); if (I.p !== b.length) throw new Error(`the view: RecapTake is ${b.length} bytes and ${I.p} were read`); return r; }
export interface DeathRecap { takes: RecapTake[]; main: number }
function readDeathRecapIn(I: In): DeathRecap { return { takes: I.list(() => readRecapTakeIn(I)), main: I.float64() }; }
export function readDeathRecap(b: Uint8Array): DeathRecap { const I = new In(b), r = readDeathRecapIn(I); if (I.p !== b.length) throw new Error(`the view: DeathRecap is ${b.length} bytes and ${I.p} were read`); return r; }
export interface DebriefView { debrief: Debrief; rows: DebriefRow[]; dropped: number; path: number[]; rooms: DebriefRoom[]; recap: DeathRecap | null }
function readDebriefViewIn(I: In): DebriefView { return { debrief: readDebriefIn(I), rows: I.list(() => readDebriefRowIn(I)), dropped: I.float64(), path: I.list(() => I.float32()), rooms: I.list(() => readDebriefRoomIn(I)), recap: I.bool() ? readDeathRecapIn(I) : null }; }
export function readDebriefView(b: Uint8Array): DebriefView { const I = new In(b), r = readDebriefViewIn(I); if (I.p !== b.length) throw new Error(`the view: DebriefView is ${b.length} bytes and ${I.p} were read`); return r; }
