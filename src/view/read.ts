// Written by the program `half` (rust/game_start/src/bin/half): THE VIEW, READ, the types an answer is of (rust/game_types/src/view and what they name) and their bytes, as plain values. To change one, change it there.
import type { Vec3 } from '../math/vec';

export class In { p = 0; dv: DataView; constructor(readonly b: Uint8Array) { this.dv = new DataView(b.buffer, b.byteOffset, b.byteLength); }
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
  /** a list of numbers as an array of its own */
  run<T>(A: { new (b: ArrayBuffer): T; readonly BYTES_PER_ELEMENT: number }): T { const n = this.uint32() * A.BYTES_PER_ELEMENT, at = this.b.byteOffset + this.p; this.p += n; return new A((this.b.buffer as ArrayBuffer).slice(at, at + n)); }
  list<T>(f: () => T): T[] { const n = this.uint32(), o: T[] = []; for (let i = 0; i < n; i++) o.push(f()); return o; }
  word<T>(W: readonly T[], of: string): T { const k = this.uint16(); if (k >= W.length) throw new Error(`the view: no such word of ${of} (${k})`); return W[k]; } }
const DEC = new TextDecoder();

export type HandsEnd = 'taken' | 'sam' | 'cap' | 'spent';
const HandsEndWords: readonly HandsEnd[] = ['taken', 'sam', 'cap', 'spent'];
function readHandsEndIn(I: In): HandsEnd { return I.word(HandsEndWords, 'HandsEnd'); }
export interface HandsSaid { t: number; end: HandsEnd | null; given: number; shotAt: number; up: number; of: number; left: number }
export function readHandsSaidIn(I: In): HandsSaid { return { t: I.float64(), end: I.bool() ? readHandsEndIn(I) : null, given: I.uint32(), shotAt: I.float64(), up: I.uint32(), of: I.uint32(), left: I.uint32() }; }
export interface AheadView { id: number; down: boolean; at: Vec3; yaw: number; wall: [number, number] | null; near: number; free: boolean; pos: Vec3; aimYaw: number; follow: Vec3; aim: Vec3 | null; path: Vec3[] }
export function readAheadViewIn(I: In): AheadView { return { id: I.float64(), down: I.bool(), at: [I.float64(), I.float64(), I.float64()], yaw: I.float64(), wall: I.bool() ? [I.float64(), I.float64()] : null, near: I.float64(), free: I.bool(), pos: [I.float64(), I.float64(), I.float64()], aimYaw: I.float64(), follow: [I.float64(), I.float64(), I.float64()], aim: I.bool() ? [I.float64(), I.float64(), I.float64()] : null, path: I.list(() => [I.float64(), I.float64(), I.float64()]) }; }
export type MissionOf = 'stages' | 'script' | 'lesson' | 'roomclear' | 'rescue' | 'nightbreach';
const MissionOfWords: readonly MissionOf[] = ['stages', 'script', 'lesson', 'roomclear', 'rescue', 'nightbreach'];
function readMissionOfIn(I: In): MissionOf { return I.word(MissionOfWords, 'MissionOf'); }
export interface MissionView { of: MissionOf; text: string; pos: Vec3 | null }
function readMissionViewIn(I: In): MissionView { return { of: readMissionOfIn(I), text: I.str(), pos: I.bool() ? [I.float64(), I.float64(), I.float64()] : null }; }
export interface RailPrompt { line: string; n: number; of: number; note: string | null; watch: boolean }
function readRailPromptIn(I: In): RailPrompt { return { line: I.str(), n: I.uint32(), of: I.uint32(), note: I.bool() ? I.str() : null, watch: I.bool() }; }
export interface RailCard { title: string | null; text: string }
function readRailCardIn(I: In): RailCard { return { title: I.bool() ? I.str() : null, text: I.str() }; }
export interface RailsView { prompt: RailPrompt | null; card: RailCard | null; arrow: Vec3 | null; lineMen: number[]; open: string[]; rows: string[] | null; orders: string[] | null; stalkShut: boolean; teachesHold: boolean; noLasers: boolean }
function readRailsViewIn(I: In): RailsView { return { prompt: I.bool() ? readRailPromptIn(I) : null, card: I.bool() ? readRailCardIn(I) : null, arrow: I.bool() ? [I.float64(), I.float64(), I.float64()] : null, lineMen: I.list(() => I.float64()), open: I.list(() => I.str()), rows: I.bool() ? I.list(() => I.str()) : null, orders: I.bool() ? I.list(() => I.str()) : null, stalkShut: I.bool(), teachesHold: I.bool(), noLasers: I.bool() }; }
export interface LessonView { step: number; of: number; id: string | null; finished: boolean; over: boolean; settling: boolean; asksView: boolean; rails: RailsView | null }
function readLessonViewIn(I: In): LessonView { return { step: I.uint32(), of: I.uint32(), id: I.bool() ? I.str() : null, finished: I.bool(), over: I.bool(), settling: I.bool(), asksView: I.bool(), rails: I.bool() ? readRailsViewIn(I) : null }; }
export type RadioKind = 'net' | 'heard';
const RadioKindWords: readonly RadioKind[] = ['net', 'heard'];
function readRadioKindIn(I: In): RadioKind { return I.word(RadioKindWords, 'RadioKind'); }
export interface RadioRow { kind: RadioKind; text: string; age: number }
function readRadioRowIn(I: In): RadioRow { return { kind: readRadioKindIn(I), text: I.str(), age: I.float64() }; }
export type BubbleTone = 'calm' | 'sus' | 'alert' | 'own';
const BubbleToneWords: readonly BubbleTone[] = ['calm', 'sus', 'alert', 'own'];
function readBubbleToneIn(I: In): BubbleTone { return I.word(BubbleToneWords, 'BubbleTone'); }
export type BubblePlace = 'over' | 'overOrEdge' | 'edge';
const BubblePlaceWords: readonly BubblePlace[] = ['over', 'overOrEdge', 'edge'];
function readBubblePlaceIn(I: In): BubblePlace { return I.word(BubblePlaceWords, 'BubblePlace'); }
export interface BubbleView { man: number | null; text: string; edge: string; age: number; dur: number; radio: boolean; tone: BubbleTone; place: BubblePlace; at: Vec3 }
function readBubbleViewIn(I: In): BubbleView { return { man: I.bool() ? I.float64() : null, text: I.str(), edge: I.str(), age: I.float64(), dur: I.float64(), radio: I.bool(), tone: readBubbleToneIn(I), place: readBubblePlaceIn(I), at: [I.float64(), I.float64(), I.float64()] }; }
export type KitThing = 'hands' | 'pistol' | 'rifle' | 'smoke' | 'flash' | 'frag';
const KitThingWords: readonly KitThing[] = ['hands', 'pistol', 'rifle', 'smoke', 'flash', 'frag'];
function readKitThingIn(I: In): KitThing { return I.word(KitThingWords, 'KitThing'); }
export interface KitSlot { thing: KitThing; key: string; name: string; count: number; absent: boolean; spent: boolean; up: boolean; taken: boolean }
function readKitSlotIn(I: In): KitSlot { return { thing: readKitThingIn(I), key: I.str(), name: I.str(), count: I.float64(), absent: I.bool(), spent: I.bool(), up: I.bool(), taken: I.bool() }; }
export interface KitWord { text: string; on: boolean; out: boolean }
function readKitWordIn(I: In): KitWord { return { text: I.str(), on: I.bool(), out: I.bool() }; }
export interface KitOcp { charge: number; ready: boolean }
function readKitOcpIn(I: In): KitOcp { return { charge: I.float64(), ready: I.bool() }; }
export interface KitGun { mag: number; capacity: number; chamber: boolean; ready: number; dry: boolean; reloading: boolean; selector: string | null; load: KitWord | null; light: boolean; ocp: KitOcp | null; spares: number[]; loose: number | null }
function readKitGunIn(I: In): KitGun { return { mag: I.float64(), capacity: I.float64(), chamber: I.bool(), ready: I.float64(), dry: I.bool(), reloading: I.bool(), selector: I.bool() ? I.str() : null, load: I.bool() ? readKitWordIn(I) : null, light: I.bool(), ocp: I.bool() ? readKitOcpIn(I) : null, spares: I.list(() => I.float64()), loose: I.bool() ? I.float64() : null }; }
export interface KitSetupRow { key: string; words: KitWord[] }
function readKitSetupRowIn(I: In): KitSetupRow { return { key: I.str(), words: I.list(() => readKitWordIn(I)) }; }
export interface KitSetup { rows: KitSetupRow[]; back: string }
function readKitSetupIn(I: In): KitSetup { return { rows: I.list(() => readKitSetupRowIn(I)), back: I.str() }; }
export interface KitBig { name: string; place: string; offered: boolean }
function readKitBigIn(I: In): KitBig { return { name: I.str(), place: I.str(), offered: I.bool() }; }
export interface KitView { slots: KitSlot[]; gun: KitGun | null; setup: KitSetup | null; big: KitBig | null; hitsLeft: number; hitsMax: number; down: boolean; god: boolean; nightVision: boolean | null; keyTag: string | null }
function readKitViewIn(I: In): KitView { return { slots: I.list(() => readKitSlotIn(I)), gun: I.bool() ? readKitGunIn(I) : null, setup: I.bool() ? readKitSetupIn(I) : null, big: I.bool() ? readKitBigIn(I) : null, hitsLeft: I.float64(), hitsMax: I.float64(), down: I.bool(), god: I.bool(), nightVision: I.bool() ? I.bool() : null, keyTag: I.bool() ? I.str() : null }; }
export type HeadTone = 'calm' | 'ko' | 'dead' | 'alert' | 'sus';
const HeadToneWords: readonly HeadTone[] = ['calm', 'ko', 'dead', 'alert', 'sus'];
function readHeadToneIn(I: In): HeadTone { return I.word(HeadToneWords, 'HeadTone'); }
export interface ManMark { man: number; text: string; tone: HeadTone; civ: boolean; hidden: boolean; debug: string }
function readManMarkIn(I: In): ManMark { return { man: I.float64(), text: I.str(), tone: readHeadToneIn(I), civ: I.bool(), hidden: I.bool(), debug: I.str() }; }
export interface SeenMark { man: number; at: Vec3; age: number; life: number }
function readSeenMarkIn(I: In): SeenMark { return { man: I.float64(), at: [I.float64(), I.float64(), I.float64()], age: I.float64(), life: I.float64() }; }
export interface CamMark { at: Vec3; sends: boolean }
function readCamMarkIn(I: In): CamMark { return { at: [I.float64(), I.float64(), I.float64()], sends: I.bool() }; }
export interface FragMark { at: Vec3; left: number }
function readFragMarkIn(I: In): FragMark { return { at: [I.float64(), I.float64(), I.float64()], left: I.float64() }; }
export type FloorWord = 'carpet' | 'lino' | 'stone' | 'tile' | 'asphalt' | 'grass';
const FloorWordWords: readonly FloorWord[] = ['carpet', 'lino', 'stone', 'tile', 'asphalt', 'grass'];
function readFloorWordIn(I: In): FloorWord { return I.word(FloorWordWords, 'FloorWord'); }
export interface MarksView { men: ManMark[]; seen: SeenMark[]; cams: CamMark[]; frags: FragMark[]; blind: number; light: number; irradiance: number; noise: number; stalk: number | null; stalkWaits: boolean; floor: FloorWord | null }
function readMarksViewIn(I: In): MarksView { return { men: I.list(() => readManMarkIn(I)), seen: I.list(() => readSeenMarkIn(I)), cams: I.list(() => readCamMarkIn(I)), frags: I.list(() => readFragMarkIn(I)), blind: I.float64(), light: I.float64(), irradiance: I.float64(), noise: I.float64(), stalk: I.bool() ? I.float64() : null, stalkWaits: I.bool(), floor: I.bool() ? readFloorWordIn(I) : null }; }
export type RingOn = 'free' | 'light' | 'guard';
const RingOnWords: readonly RingOn[] = ['free', 'light', 'guard'];
function readRingOnIn(I: In): RingOn { return I.word(RingOnWords, 'RingOn'); }
export interface AimCone { spread: number; track: number; range: number; hands: number; settle: number; crossing: number; feet: number }
function readAimConeIn(I: In): AimCone { return { spread: I.float64(), track: I.float64(), range: I.float64(), hands: I.float64(), settle: I.float64(), crossing: I.float64(), feet: I.float64() }; }
export interface AimRing { at: Vec3; on: RingOn; trail: boolean; shut: boolean; wish: Vec3 | null; cone: AimCone | null }
function readAimRingIn(I: In): AimRing { return { at: [I.float64(), I.float64(), I.float64()], on: readRingOnIn(I), trail: I.bool(), shut: I.bool(), wish: I.bool() ? [I.float64(), I.float64(), I.float64()] : null, cone: I.bool() ? readAimConeIn(I) : null }; }
export interface GunDot { at: Vec3; shut: boolean }
function readGunDotIn(I: In): GunDot { return { at: [I.float64(), I.float64(), I.float64()], shut: I.bool() }; }
export interface ThrowRest { at: Vec3; path: Vec3[] }
function readThrowRestIn(I: In): ThrowRest { return { at: [I.float64(), I.float64(), I.float64()], path: I.list(() => [I.float64(), I.float64(), I.float64()]) }; }
export interface ThrowReachView { at: Vec3; kill: number; hurt: number; stun: number }
function readThrowReachViewIn(I: In): ThrowReachView { return { at: [I.float64(), I.float64(), I.float64()], kill: I.float64(), hurt: I.float64(), stun: I.float64() }; }
export interface ThrowView { points: Vec3[]; land: Vec3; short: boolean; rest: ThrowRest | null; reach: ThrowReachView | null }
function readThrowViewIn(I: In): ThrowView { return { points: I.list(() => [I.float64(), I.float64(), I.float64()]), land: [I.float64(), I.float64(), I.float64()], short: I.bool(), rest: I.bool() ? readThrowRestIn(I) : null, reach: I.bool() ? readThrowReachViewIn(I) : null }; }
export interface AimView { ring: AimRing | null; dot: GunDot | null; tooClose: boolean; throw: ThrowView | null }
function readAimViewIn(I: In): AimView { return { ring: I.bool() ? readAimRingIn(I) : null, dot: I.bool() ? readGunDotIn(I) : null, tooClose: I.bool(), throw: I.bool() ? readThrowViewIn(I) : null }; }
export type BigTool = 'shotgun' | 'hammer';
const BigToolWords: readonly BigTool[] = ['shotgun', 'hammer'];
function readBigToolIn(I: In): BigTool { return I.word(BigToolWords, 'BigTool'); }
export interface WornView { man: number; goggles: boolean; big: BigTool | null }
function readWornViewIn(I: In): WornView { return { man: I.float64(), goggles: I.bool(), big: I.bool() ? readBigToolIn(I) : null }; }
export type Pair = 'red' | 'blue';
const PairWords: readonly Pair[] = ['red', 'blue'];
function readPairIn(I: In): Pair { return I.word(PairWords, 'Pair'); }
export interface MateLine { man: number; name: string; doing: string; addressed: boolean; pair: Pair | null; down: boolean; kit: string }
function readMateLineIn(I: In): MateLine { return { man: I.float64(), name: I.str(), doing: I.str(), addressed: I.bool(), pair: I.bool() ? readPairIn(I) : null, down: I.bool(), kit: I.str() }; }
export interface Note { key: string; text: string }
function readNoteIn(I: In): Note { return { key: I.str(), text: I.str() }; }
export interface SquadView { posture: string | null; who: string; picked: boolean; mates: MateLine[]; notes: Note[]; quick: Note | null }
function readSquadViewIn(I: In): SquadView { return { posture: I.bool() ? I.str() : null, who: I.str(), picked: I.bool(), mates: I.list(() => readMateLineIn(I)), notes: I.list(() => readNoteIn(I)), quick: I.bool() ? readNoteIn(I) : null }; }
export type ThrowKind = 'frag' | 'stun' | 'shell';
const ThrowKindWords: readonly ThrowKind[] = ['frag', 'stun', 'shell'];
function readThrowKindIn(I: In): ThrowKind { return I.word(ThrowKindWords, 'ThrowKind'); }
export type Tone = 'red' | 'blue' | 'gold';
const ToneWords: readonly Tone[] = ['red', 'blue', 'gold'];
function readToneIn(I: In): Tone { return I.word(ToneWords, 'Tone'); }
export interface ThrowLine { man: number; kind: ThrowKind; from: Vec3; points: Vec3[]; land: Vec3; walk: Vec3[] | null; preview: boolean; kill: number; wound: number; tone: Tone | null }
function readThrowLineIn(I: In): ThrowLine { return { man: I.float64(), kind: readThrowKindIn(I), from: [I.float64(), I.float64(), I.float64()], points: I.list(() => [I.float64(), I.float64(), I.float64()]), land: [I.float64(), I.float64(), I.float64()], walk: I.bool() ? I.list(() => [I.float64(), I.float64(), I.float64()]) : null, preview: I.bool(), kill: I.float64(), wound: I.float64(), tone: I.bool() ? readToneIn(I) : null }; }
export interface RingWedge { key: number; text: string; off: boolean; opens: boolean }
function readRingWedgeIn(I: In): RingWedge { return { key: I.float64(), text: I.str(), off: I.bool(), opens: I.bool() }; }
export interface RingPin { at: Vec3; ox: number; oy: number; edgeX: number; edgeY: number }
function readRingPinIn(I: In): RingPin { return { at: [I.float64(), I.float64(), I.float64()], ox: I.float64(), oy: I.float64(), edgeX: I.float64(), edgeY: I.float64() }; }
export interface RingView { x: number; y: number; zoom: number; lit: number; tone: Tone | null; caption: string; wedges: RingWedge[]; pin: RingPin | null }
function readRingViewIn(I: In): RingView { return { x: I.float64(), y: I.float64(), zoom: I.float64(), lit: I.float64(), tone: I.bool() ? readToneIn(I) : null, caption: I.str(), wedges: I.list(() => readRingWedgeIn(I)), pin: I.bool() ? readRingPinIn(I) : null }; }
export interface HandRingView { x: number; y: number; zoom: number; lit: number; wedges: RingWedge[]; pin: RingPin | null }
function readHandRingViewIn(I: In): HandRingView { return { x: I.float64(), y: I.float64(), zoom: I.float64(), lit: I.float64(), wedges: I.list(() => readRingWedgeIn(I)), pin: I.bool() ? readRingPinIn(I) : null }; }
export type ThingKind = 'cam' | 'guard' | 'breaker' | 'door' | 'light' | 'body' | 'objective' | 'held' | 'wall' | 'switch' | 'charge' | 'cancel';
const ThingKindWords: readonly ThingKind[] = ['cam', 'guard', 'breaker', 'door', 'light', 'body', 'objective', 'held', 'wall', 'switch', 'charge', 'cancel'];
function readThingKindIn(I: In): ThingKind { return I.word(ThingKindWords, 'ThingKind'); }
export interface EmptyView { word: string; kind: ThingKind; dist: number | null }
function readEmptyViewIn(I: In): EmptyView { return { word: I.str(), kind: readThingKindIn(I), dist: I.bool() ? I.float64() : null }; }
export interface PlanWay { from: Vec3; own: Vec3[] | null; stops: Vec3[] }
function readPlanWayIn(I: In): PlanWay { return { from: [I.float64(), I.float64(), I.float64()], own: I.bool() ? I.list(() => [I.float64(), I.float64(), I.float64()]) : null, stops: I.list(() => [I.float64(), I.float64(), I.float64()]) }; }
export interface PlanMan { man: number; tag: string; tone: Tone | null; age: number; way: PlanWay | null; mark: Vec3; there: boolean; held: boolean; aim: number | null; watch: Vec3 | null; onDoor: boolean }
function readPlanManIn(I: In): PlanMan { return { man: I.float64(), tag: I.str(), tone: I.bool() ? readToneIn(I) : null, age: I.float64(), way: I.bool() ? readPlanWayIn(I) : null, mark: [I.float64(), I.float64(), I.float64()], there: I.bool(), held: I.bool(), aim: I.bool() ? I.float64() : null, watch: I.bool() ? [I.float64(), I.float64(), I.float64()] : null, onDoor: I.bool() }; }
export interface PlanWho { man: number; at: Vec3; tone: Tone | null }
function readPlanWhoIn(I: In): PlanWho { return { man: I.float64(), at: [I.float64(), I.float64(), I.float64()], tone: I.bool() ? readToneIn(I) : null }; }
export type KeyCap = 'none' | 'dark' | 'lit';
const KeyCapWords: readonly KeyCap[] = ['none', 'dark', 'lit'];
function readKeyCapIn(I: In): KeyCap { return I.word(KeyCapWords, 'KeyCap'); }
export type WaitWhy = 'stack' | 'tool' | 'charge' | 'thr' | 'drill' | 'wall' | 'pair';
const WaitWhyWords: readonly WaitWhy[] = ['stack', 'tool', 'charge', 'thr', 'drill', 'wall', 'pair'];
function readWaitWhyIn(I: In): WaitWhy { return I.word(WaitWhyWords, 'WaitWhy'); }
export interface PlanDoor { door: string | null; a: Vec3; b: Vec3; tone: Tone | null; key: string | null; now: boolean; cap: KeyCap; why: WaitWhy }
function readPlanDoorIn(I: In): PlanDoor { return { door: I.bool() ? I.str() : null, a: [I.float64(), I.float64(), I.float64()], b: [I.float64(), I.float64(), I.float64()], tone: I.bool() ? readToneIn(I) : null, key: I.bool() ? I.str() : null, now: I.bool(), cap: readKeyCapIn(I), why: readWaitWhyIn(I) }; }
export interface PlanThrow { man: number; tag: string; tone: Tone | null; kind: ThrowKind; preview: boolean; from: Vec3; walk: Vec3[]; arc: Vec3[]; land: Vec3; reach: number }
function readPlanThrowIn(I: In): PlanThrow { return { man: I.float64(), tag: I.str(), tone: I.bool() ? readToneIn(I) : null, kind: readThrowKindIn(I), preview: I.bool(), from: [I.float64(), I.float64(), I.float64()], walk: I.list(() => [I.float64(), I.float64(), I.float64()]), arc: I.list(() => [I.float64(), I.float64(), I.float64()]), land: [I.float64(), I.float64(), I.float64()], reach: I.float64() }; }
export interface SeatPlan { men: PlanMan[]; who: PlanWho[]; you: Vec3 | null; doors: PlanDoor[]; throws: PlanThrow[]; mode: boolean }
function readSeatPlanIn(I: In): SeatPlan { return { men: I.list(() => readPlanManIn(I)), who: I.list(() => readPlanWhoIn(I)), you: I.bool() ? [I.float64(), I.float64(), I.float64()] : null, doors: I.list(() => readPlanDoorIn(I)), throws: I.list(() => readPlanThrowIn(I)), mode: I.bool() }; }
export interface PlanView { own: SeatPlan; others: SeatPlan[] }
function readPlanViewIn(I: In): PlanView { return { own: readSeatPlanIn(I), others: I.list(() => readSeatPlanIn(I)) }; }
export interface RowOpt { key: string; text: string }
function readRowOptIn(I: In): RowOpt { return { key: I.str(), text: I.str() }; }
export interface ListRow { kind: ThingKind; label: string; opts: RowOpt[]; inReach: boolean; off: boolean; dist: number | null; progress: number | null; cont: boolean }
function readListRowIn(I: In): ListRow { return { kind: readThingKindIn(I), label: I.str(), opts: I.list(() => readRowOptIn(I)), inReach: I.bool(), off: I.bool(), dist: I.bool() ? I.float64() : null, progress: I.bool() ? I.float64() : null, cont: I.bool() }; }
export interface ThingRing { kind: ThingKind; at: Vec3; man: number | null; up: number; off: boolean; inReach: boolean; progress: number | null }
function readThingRingIn(I: In): ThingRing { return { kind: readThingKindIn(I), at: [I.float64(), I.float64(), I.float64()], man: I.bool() ? I.float64() : null, up: I.float64(), off: I.bool(), inReach: I.bool(), progress: I.bool() ? I.float64() : null }; }
export interface ThingDot { kind: ThingKind; at: Vec3; man: number | null; up: number; off: boolean; hidden: boolean; ringed: boolean }
function readThingDotIn(I: In): ThingDot { return { kind: readThingKindIn(I), at: [I.float64(), I.float64(), I.float64()], man: I.bool() ? I.float64() : null, up: I.float64(), off: I.bool(), hidden: I.bool(), ringed: I.bool() }; }
export interface InteractView { rows: ListRow[]; lit: number; open: boolean; ring: ThingRing | null; dots: ThingDot[] }
function readInteractViewIn(I: In): InteractView { return { rows: I.list(() => readListRowIn(I)), lit: I.float64(), open: I.bool(), ring: I.bool() ? readThingRingIn(I) : null, dots: I.list(() => readThingDotIn(I)) }; }
export interface AlarmView { level: number; left: number; live: boolean }
function readAlarmViewIn(I: In): AlarmView { return { level: I.float64(), left: I.float64(), live: I.bool() }; }
export type Stage = 'drive' | 'infiltrate' | 'exfil' | 'done' | 'failed';
const StageWords: readonly Stage[] = ['drive', 'infiltrate', 'exfil', 'done', 'failed'];
function readStageIn(I: In): Stage { return I.word(StageWords, 'Stage'); }
export interface LoneLook { r: number; h: number; pad: number; letter: number; max: number; lead: number }
function readLoneLookIn(I: In): LoneLook { return { r: I.float64(), h: I.float64(), pad: I.float64(), letter: I.float64(), max: I.float64(), lead: I.float64() }; }
export interface RingLook { places: number; hole: number; rim: number; boxX: number; boxY: number; lone: LoneLook }
function readRingLookIn(I: In): RingLook { return { places: I.float64(), hole: I.float64(), rim: I.float64(), boxX: I.float64(), boxY: I.float64(), lone: readLoneLookIn(I) }; }
export interface PlanLook { freshSecs: number; walkDim: number; restDim: number; ringM: number; sectorM: number; sectorDeg: number; reaskSecs: number; others: number }
function readPlanLookIn(I: In): PlanLook { return { freshSecs: I.float64(), walkDim: I.float64(), restDim: I.float64(), ringM: I.float64(), sectorM: I.float64(), sectorDeg: I.float64(), reaskSecs: I.float64(), others: I.float64() }; }
export interface NetLook { edge: boolean; near: number; edgeIn: number }
function readNetLookIn(I: In): NetLook { return { edge: I.bool(), near: I.float64(), edgeIn: I.float64() }; }
export interface AttentionLook { ringM: number; wayReask: number }
function readAttentionLookIn(I: In): AttentionLook { return { ringM: I.float64(), wayReask: I.float64() }; }
export interface HudLook { ring: RingLook; plan: PlanLook; net: NetLook; attention: AttentionLook; lineClear: boolean; markClear: boolean }
function readHudLookIn(I: In): HudLook { return { ring: readRingLookIn(I), plan: readPlanLookIn(I), net: readNetLookIn(I), attention: readAttentionLookIn(I), lineClear: I.bool(), markClear: I.bool() }; }
export type AttentionMode = 'go' | 'look' | 'lent';
const AttentionModeWords: readonly AttentionMode[] = ['go', 'look', 'lent'];
function readAttentionModeIn(I: In): AttentionMode { return I.word(AttentionModeWords, 'AttentionMode'); }
export interface AttentionLine { man: number; at: Vec3; left: number; mode: AttentionMode; ending: boolean; worn: boolean; walking: boolean; hidden: boolean; walks: boolean; own: Vec3[] | null }
function readAttentionLineIn(I: In): AttentionLine { return { man: I.float64(), at: [I.float64(), I.float64(), I.float64()], left: I.float64(), mode: readAttentionModeIn(I), ending: I.bool(), worn: I.bool(), walking: I.bool(), hidden: I.bool(), walks: I.bool(), own: I.bool() ? I.list(() => [I.float64(), I.float64(), I.float64()]) : null }; }
export interface TickView { mission: MissionView; lesson: LessonView | null; radio: RadioRow[]; bubbles: BubbleView[]; me: number; place: string; kit: KitView; marks: MarksView; aim: AimView; worn: WornView[]; squad: SquadView | null; corner: Note[]; throws: ThrowLine[]; ring: RingView | null; hand: HandRingView | null; empty: EmptyView | null; plan: PlanView; live: boolean; interact: InteractView; alarm: AlarmView | null; stage: Stage; look: HudLook; attention: AttentionLine[] }
export function readTickViewIn(I: In): TickView { return { mission: readMissionViewIn(I), lesson: I.bool() ? readLessonViewIn(I) : null, radio: I.list(() => readRadioRowIn(I)), bubbles: I.list(() => readBubbleViewIn(I)), me: I.float64(), place: I.str(), kit: readKitViewIn(I), marks: readMarksViewIn(I), aim: readAimViewIn(I), worn: I.list(() => readWornViewIn(I)), squad: I.bool() ? readSquadViewIn(I) : null, corner: I.list(() => readNoteIn(I)), throws: I.list(() => readThrowLineIn(I)), ring: I.bool() ? readRingViewIn(I) : null, hand: I.bool() ? readHandRingViewIn(I) : null, empty: I.bool() ? readEmptyViewIn(I) : null, plan: readPlanViewIn(I), live: I.bool(), interact: readInteractViewIn(I), alarm: I.bool() ? readAlarmViewIn(I) : null, stage: readStageIn(I), look: readHudLookIn(I), attention: I.list(() => readAttentionLineIn(I)) }; }
export type UnderKind = 'floor' | 'door' | 'none' | 'foe';
const UnderKindWords: readonly UnderKind[] = ['floor', 'door', 'none', 'foe'];
function readUnderKindIn(I: In): UnderKind { return I.word(UnderKindWords, 'UnderKind'); }
export interface UnderWall { index: number; alongX: boolean; u: number; side: number; ok: boolean; why: string; outer: boolean }
function readUnderWallIn(I: In): UnderWall { return { index: I.float64(), alongX: I.bool(), u: I.float64(), side: I.float64(), ok: I.bool(), why: I.str(), outer: I.bool() }; }
export type WorkThingKind = 'breaker' | 'objective';
const WorkThingKindWords: readonly WorkThingKind[] = ['breaker', 'objective'];
function readWorkThingKindIn(I: In): WorkThingKind { return I.word(WorkThingKindWords, 'WorkThingKind'); }
export interface WorkThing { kind: WorkThingKind; face: Vec3; n: Vec3; stand: Vec3 | null; name: string }
function readWorkThingIn(I: In): WorkThing { return { kind: readWorkThingKindIn(I), face: [I.float64(), I.float64(), I.float64()], n: [I.float64(), I.float64(), I.float64()], stand: I.bool() ? [I.float64(), I.float64(), I.float64()] : null, name: I.str() }; }
export interface Under { kind: UnderKind; at: Vec3; door: string | null; side: number; man: number; wall: UnderWall | null; thing: WorkThing | null }
function readUnderIn(I: In): Under { return { kind: readUnderKindIn(I), at: [I.float64(), I.float64(), I.float64()], door: I.bool() ? I.str() : null, side: I.float64(), man: I.float64(), wall: I.bool() ? readUnderWallIn(I) : null, thing: I.bool() ? readWorkThingIn(I) : null }; }
export interface CursorView { under: Under | null }
export function readCursorViewIn(I: In): CursorView { return { under: I.bool() ? readUnderIn(I) : null }; }
export interface RunCounts { elapsed: number; stage: string; done: boolean; failed: boolean; sandbox: boolean; blackout: boolean; seen: number; seenUp: number; alarms: number; esc1: number; esc2: number; bodies: number; kills: number; kos: number; shots: number; shotsNearMen: number; lightsShot: number; ocps: number; kicked: number; picked: number; keys: number; intel: number; questioned: number; roster: number; hits: number; down: boolean; loads: number; thrown: number; grabs: number; dropped: number; devices: number; civSeen: number; civReported: number; civComplied: number; civKo: number; civKilled: number }
function readRunCountsIn(I: In): RunCounts { return { elapsed: I.float64(), stage: I.str(), done: I.bool(), failed: I.bool(), sandbox: I.bool(), blackout: I.bool(), seen: I.float64(), seenUp: I.float64(), alarms: I.float64(), esc1: I.float64(), esc2: I.float64(), bodies: I.float64(), kills: I.float64(), kos: I.float64(), shots: I.float64(), shotsNearMen: I.float64(), lightsShot: I.float64(), ocps: I.float64(), kicked: I.float64(), picked: I.float64(), keys: I.float64(), intel: I.float64(), questioned: I.float64(), roster: I.float64(), hits: I.float64(), down: I.bool(), loads: I.float64(), thrown: I.float64(), grabs: I.float64(), dropped: I.float64(), devices: I.float64(), civSeen: I.float64(), civReported: I.float64(), civComplied: I.float64(), civKo: I.float64(), civKilled: I.float64() }; }
export interface ScoreLine { label: string; pts: number }
function readScoreLineIn(I: In): ScoreLine { return { label: I.str(), pts: I.float64() }; }
export interface Badge { id: string; name: string; on: boolean; hint: string }
function readBadgeIn(I: In): Badge { return { id: I.str(), name: I.str(), on: I.bool(), hint: I.str() }; }
export type DebriefState = 'down' | 'done' | 'sofar';
const DebriefStateWords: readonly DebriefState[] = ['down', 'done', 'sofar'];
function readDebriefStateIn(I: In): DebriefState { return I.word(DebriefStateWords, 'DebriefState'); }
export interface Debrief { counts: RunCounts; score: number; lines: ScoreLine[]; rating: string; blurb: string; badges: Badge[]; summary: string; state: DebriefState; scored: boolean; grade: string }
function readDebriefIn(I: In): Debrief { return { counts: readRunCountsIn(I), score: I.float64(), lines: I.list(() => readScoreLineIn(I)), rating: I.str(), blurb: I.str(), badges: I.list(() => readBadgeIn(I)), summary: I.str(), state: readDebriefStateIn(I), scored: I.bool(), grade: I.str() }; }
export type ChronKind = 'down' | 'shot' | 'blackout' | 'heard' | 'objective' | 'grab' | 'ocp' | 'thrown' | 'radio' | 'spotted' | 'contact' | 'lost' | 'calm' | 'alarm' | 'lightShot' | 'interrogated' | 'keyTaken' | 'ko' | 'kill' | 'bodyFound' | 'doorKicked' | 'lockPicked' | 'hit' | 'needBreak' | 'civilianSpotted' | 'civilianReported' | 'civilianComplied' | 'civilianKO' | 'civilianKilled' | 'cameraSpotted' | 'cameraAlarm' | 'laserTripped' | 'deviceDown' | 'codeLearned' | 'codeEntered' | 'checkpoint';
const ChronKindWords: readonly ChronKind[] = ['down', 'shot', 'blackout', 'heard', 'objective', 'grab', 'ocp', 'thrown', 'radio', 'spotted', 'contact', 'lost', 'calm', 'alarm', 'lightShot', 'interrogated', 'keyTaken', 'ko', 'kill', 'bodyFound', 'doorKicked', 'lockPicked', 'hit', 'needBreak', 'civilianSpotted', 'civilianReported', 'civilianComplied', 'civilianKO', 'civilianKilled', 'cameraSpotted', 'cameraAlarm', 'laserTripped', 'deviceDown', 'codeLearned', 'codeEntered', 'checkpoint'];
function readChronKindIn(I: In): ChronKind { return I.word(ChronKindWords, 'ChronKind'); }
export interface DebriefRow { t: number; kind: ChronKind; room: string; pos: Vec3; text: string; down: boolean }
function readDebriefRowIn(I: In): DebriefRow { return { t: I.float64(), kind: readChronKindIn(I), room: I.str(), pos: [I.float64(), I.float64(), I.float64()], text: I.str(), down: I.bool() }; }
export type RecapCause = 'blast' | 'round' | 'other' | 'unknown';
const RecapCauseWords: readonly RecapCause[] = ['blast', 'round', 'other', 'unknown'];
function readRecapCauseIn(I: In): RecapCause { return I.word(RecapCauseWords, 'RecapCause'); }
export interface RecapDeath { t: number; victim: number; name: string; side: string; player: boolean; at: [number, number]; gaze: number; cause: RecapCause; by: number; byName: string; bySide: string; byAt: [number, number] | null; from: [number, number] | null; to: [number, number]; weapon: string; dist: number; off: number; blind: boolean; loud: boolean; serial: number }
function readRecapDeathIn(I: In): RecapDeath { return { t: I.float64(), victim: I.float64(), name: I.str(), side: I.str(), player: I.bool(), at: [I.float64(), I.float64()], gaze: I.float64(), cause: readRecapCauseIn(I), by: I.float64(), byName: I.str(), bySide: I.str(), byAt: I.bool() ? [I.float64(), I.float64()] : null, from: I.bool() ? [I.float64(), I.float64()] : null, to: [I.float64(), I.float64()], weapon: I.str(), dist: I.float64(), off: I.float64(), blind: I.bool(), loud: I.bool(), serial: I.uint32() }; }
export type RecapManKind = 'player' | 'civ' | 'man';
const RecapManKindWords: readonly RecapManKind[] = ['player', 'civ', 'man'];
function readRecapManKindIn(I: In): RecapManKind { return I.word(RecapManKindWords, 'RecapManKind'); }
export interface RecapMan { id: number; name: string; side: string; kind: RecapManKind }
function readRecapManIn(I: In): RecapMan { return { id: I.float64(), name: I.str(), side: I.str(), kind: readRecapManKindIn(I) }; }
export interface RecapDoor { hinge: [number, number]; width: number }
function readRecapDoorIn(I: In): RecapDoor { return { hinge: [I.float64(), I.float64()], width: I.float64() }; }
export interface RecapFrame { t: number; n: number; men: number[]; doors: number[] }
function readRecapFrameIn(I: In): RecapFrame { return { t: I.float64(), n: I.float64(), men: I.list(() => I.float32()), doors: I.list(() => I.float32()) }; }
export type RecapShotKind = 'blast' | 'bang' | 'round';
const RecapShotKindWords: readonly RecapShotKind[] = ['blast', 'bang', 'round'];
function readRecapShotKindIn(I: In): RecapShotKind { return I.word(RecapShotKindWords, 'RecapShotKind'); }
export interface RecapShot { t: number; kind: RecapShotKind; who: number; from: [number, number]; to: [number, number] | null; at: [number, number] | null; weapon: string | null; loud: boolean | null }
function readRecapShotIn(I: In): RecapShot { return { t: I.float64(), kind: readRecapShotKindIn(I), who: I.float64(), from: [I.float64(), I.float64()], to: I.bool() ? [I.float64(), I.float64()] : null, at: I.bool() ? [I.float64(), I.float64()] : null, weapon: I.bool() ? I.str() : null, loud: I.bool() ? I.bool() : null }; }
export interface RecapTake { death: RecapDeath; t0: number; t1: number; cast: RecapMan[]; doors: RecapDoor[]; frames: RecapFrame[]; shots: RecapShot[]; deaths: RecapDeath[] }
function readRecapTakeIn(I: In): RecapTake { return { death: readRecapDeathIn(I), t0: I.float64(), t1: I.float64(), cast: I.list(() => readRecapManIn(I)), doors: I.list(() => readRecapDoorIn(I)), frames: I.list(() => readRecapFrameIn(I)), shots: I.list(() => readRecapShotIn(I)), deaths: I.list(() => readRecapDeathIn(I)) }; }
export interface DeathRecap { takes: RecapTake[]; main: number }
function readDeathRecapIn(I: In): DeathRecap { return { takes: I.list(() => readRecapTakeIn(I)), main: I.float64() }; }
export interface DebriefView { debrief: Debrief; rows: DebriefRow[]; dropped: number; path: number[]; recap: DeathRecap | null }
export function readDebriefViewIn(I: In): DebriefView { return { debrief: readDebriefIn(I), rows: I.list(() => readDebriefRowIn(I)), dropped: I.float64(), path: I.list(() => I.float32()), recap: I.bool() ? readDeathRecapIn(I) : null }; }
export type MissionKind = 'nightbreach' | 'roomclear' | 'rescue' | 'quiet';
const MissionKindWords: readonly MissionKind[] = ['nightbreach', 'roomclear', 'rescue', 'quiet'];
function readMissionKindIn(I: In): MissionKind { return I.word(MissionKindWords, 'MissionKind'); }
export type Approach = 'doors' | 'quiet' | 'walls' | 'dark';
const ApproachWords: readonly Approach[] = ['doors', 'quiet', 'walls', 'dark'];
function readApproachIn(I: In): Approach { return I.word(ApproachWords, 'Approach'); }
export type OutlineHue = 'you' | 'purple' | 'red' | 'blue';
const OutlineHueWords: readonly OutlineHue[] = ['you', 'purple', 'red', 'blue'];
function readOutlineHueIn(I: In): OutlineHue { return I.word(OutlineHueWords, 'OutlineHue'); }
export type ManState = 'dead' | 'out' | 'up';
const ManStateWords: readonly ManState[] = ['dead', 'out', 'up'];
function readManStateIn(I: In): ManState { return I.word(ManStateWords, 'ManState'); }
export interface VerdictMan { id: number; name: string; tone: OutlineHue | null; human: boolean; state: ManState; downAt: number; downRoom: string; downBy: string; downPos: [number, number] | null; drops: number[]; path: number[] }
function readVerdictManIn(I: In): VerdictMan { return { id: I.float64(), name: I.str(), tone: I.bool() ? readOutlineHueIn(I) : null, human: I.bool(), state: readManStateIn(I), downAt: I.float64(), downRoom: I.str(), downBy: I.str(), downPos: I.bool() ? [I.float64(), I.float64()] : null, drops: I.list(() => I.float64()), path: I.list(() => I.float64()) }; }
export interface RunVerdictFoes { down: number; of: number; fell: number[] }
function readRunVerdictFoesIn(I: In): RunVerdictFoes { return { down: I.float64(), of: I.float64(), fell: I.list(() => I.float64()) }; }
export interface RunVerdictHostages { alive: number; of: number }
function readRunVerdictHostagesIn(I: In): RunVerdictHostages { return { alive: I.float64(), of: I.float64() }; }
export interface RunVerdict { kind: MissionKind | null; won: boolean; word: string; why: string; secs: number; approach: Approach | null; void: boolean; clean: boolean; men: VerdictMan[]; lost: number; of: number; foes: RunVerdictFoes; hostages: RunVerdictHostages | null; start: [number, number] }
function readRunVerdictIn(I: In): RunVerdict { return { kind: I.bool() ? readMissionKindIn(I) : null, won: I.bool(), word: I.str(), why: I.str(), secs: I.float64(), approach: I.bool() ? readApproachIn(I) : null, void: I.bool(), clean: I.bool(), men: I.list(() => readVerdictManIn(I)), lost: I.float64(), of: I.float64(), foes: readRunVerdictFoesIn(I), hostages: I.bool() ? readRunVerdictHostagesIn(I) : null, start: [I.float64(), I.float64()] }; }
export type Medal = 'none' | 'gold' | 'silver' | 'bronze';
const MedalWords: readonly Medal[] = ['none', 'gold', 'silver', 'bronze'];
function readMedalIn(I: In): Medal { return I.word(MedalWords, 'Medal'); }
export interface ParLine { medal: Medal; line: string }
function readParLineIn(I: In): ParLine { return { medal: readMedalIn(I), line: I.str() }; }
export interface RunMark { glyph: string; cls: string; x: number; z: number; title: string }
function readRunMarkIn(I: In): RunMark { return { glyph: I.str(), cls: I.str(), x: I.float64(), z: I.float64(), title: I.str() }; }
export interface WalkedFloor { w: number; h: number; cell: number; rows: string }
function readWalkedFloorIn(I: In): WalkedFloor { return { w: I.uint32(), h: I.uint32(), cell: I.float64(), rows: I.str() }; }
export interface EndView { verdict: RunVerdict | null; par: ParLine | null; holder: string; marks: RunMark[]; floor: WalkedFloor }
export function readEndViewIn(I: In): EndView { return { verdict: I.bool() ? readRunVerdictIn(I) : null, par: I.bool() ? readParLineIn(I) : null, holder: I.str(), marks: I.list(() => readRunMarkIn(I)), floor: readWalkedFloorIn(I) }; }
export interface RunView { begun: boolean; over: boolean; void: boolean; lesson: boolean; mission: string; level: string; won: boolean; way: Approach | null; clean: boolean; secs: number; course: string; par: number | null }
export function readRunViewIn(I: In): RunView { return { begun: I.bool(), over: I.bool(), void: I.bool(), lesson: I.bool(), mission: I.str(), level: I.str(), won: I.bool(), way: I.bool() ? readApproachIn(I) : null, clean: I.bool(), secs: I.float64(), course: I.str(), par: I.bool() ? I.float64() : null }; }
export interface OutcomeView { won: boolean; word: string; why: string; secs: number; approach: Approach | null; lost: number; of: number; foesDown: number; foesOf: number; hostages: RunVerdictHostages | null; void: boolean; clean: boolean }
export function readOutcomeViewIn(I: In): OutcomeView { return { won: I.bool(), word: I.str(), why: I.str(), secs: I.float64(), approach: I.bool() ? readApproachIn(I) : null, lost: I.float64(), of: I.float64(), foesDown: I.float64(), foesOf: I.float64(), hostages: I.bool() ? readRunVerdictHostagesIn(I) : null, void: I.bool(), clean: I.bool() }; }
export interface FloorView { bounds: [number, number, number, number]; boxes: number[] }
export function readFloorViewIn(I: In): FloorView { return { bounds: [I.float64(), I.float64(), I.float64(), I.float64()], boxes: I.list(() => I.float64()) }; }
export type MapKind = 'player' | 'man' | 'civ';
const MapKindWords: readonly MapKind[] = ['player', 'man', 'civ'];
function readMapKindIn(I: In): MapKind { return I.word(MapKindWords, 'MapKind'); }
export interface MapMan { id: number; x: number; z: number; aim: number; team: string; kind: MapKind; down: boolean; name: string; state: string }
function readMapManIn(I: In): MapMan { return { id: I.float64(), x: I.float64(), z: I.float64(), aim: I.float64(), team: I.str(), kind: readMapKindIn(I), down: I.bool(), name: I.str(), state: I.str() }; }
export interface MapDoor { hx: number; hz: number; ux: number; uz: number; width: number; locked: boolean; broken: boolean; off: boolean; name: string }
function readMapDoorIn(I: In): MapDoor { return { hx: I.float64(), hz: I.float64(), ux: I.float64(), uz: I.float64(), width: I.float64(), locked: I.bool(), broken: I.bool(), off: I.bool(), name: I.str() }; }
export interface MapView { men: MapMan[]; doors: MapDoor[] }
export function readMapViewIn(I: In): MapView { return { men: I.list(() => readMapManIn(I)), doors: I.list(() => readMapDoorIn(I)) }; }
export type GuardState = 'patrol' | 'suspicious' | 'alert' | 'search' | 'dead';
const GuardStateWords: readonly GuardState[] = ['patrol', 'suspicious', 'alert', 'search', 'dead'];
function readGuardStateIn(I: In): GuardState { return I.word(GuardStateWords, 'GuardState'); }
export interface TallyMan { state: GuardState; lit: boolean }
function readTallyManIn(I: In): TallyMan { return { state: readGuardStateIn(I), lit: I.bool() }; }
export type QrfState = 'idle' | 'requested' | 'inbound' | 'arriving' | 'deployed';
const QrfStateWords: readonly QrfState[] = ['idle', 'requested', 'inbound', 'arriving', 'deployed'];
function readQrfStateIn(I: In): QrfState { return I.word(QrfStateWords, 'QrfState'); }
export type MissionStage = 'drive' | 'infiltrate' | 'exfil' | 'done' | 'failed';
const MissionStageWords: readonly MissionStage[] = ['drive', 'infiltrate', 'exfil', 'done', 'failed'];
function readMissionStageIn(I: In): MissionStage { return I.word(MissionStageWords, 'MissionStage'); }
export interface MissionStats { qrfWho: string | null; elapsed: number; alerts: number; spotted: number; bodies: number; shots: number; kills: number; knockouts: number; keys: number; blackout: boolean; sandbox: boolean; qrf: number; qrfAt: number }
function readMissionStatsIn(I: In): MissionStats { return { qrfWho: I.bool() ? I.str() : null, elapsed: I.float64(), alerts: I.float64(), spotted: I.float64(), bodies: I.float64(), shots: I.float64(), kills: I.float64(), knockouts: I.float64(), keys: I.float64(), blackout: I.bool(), sandbox: I.bool(), qrf: I.float64(), qrfAt: I.float64() }; }
export interface Tally { men: TallyMan[]; qrf: QrfState; escalation: number; autosaves: number; manual: number; loads: number; stage: MissionStage; alarm: boolean; alerts: number; clear: boolean; kind: MissionKind | null; stats: MissionStats }
export function readTallyIn(I: In): Tally { return { men: I.list(() => readTallyManIn(I)), qrf: readQrfStateIn(I), escalation: I.float64(), autosaves: I.float64(), manual: I.float64(), loads: I.float64(), stage: readMissionStageIn(I), alarm: I.bool(), alerts: I.float64(), clear: I.bool(), kind: I.bool() ? readMissionKindIn(I) : null, stats: readMissionStatsIn(I) }; }
export interface KnobRow { name: string; yes: boolean; value: number }
export function readKnobRowIn(I: In): KnobRow { return { name: I.str(), yes: I.bool(), value: I.float64() }; }
export interface ShownRow { name: string; label: string; section: string; title: string; lo: number; hi: number; step: number; scale: number; def: number; kept: boolean; adv: boolean }
export function readShownRowIn(I: In): ShownRow { return { name: I.str(), label: I.str(), section: I.str(), title: I.str(), lo: I.float64(), hi: I.float64(), step: I.float64(), scale: I.float64(), def: I.float64(), kept: I.bool(), adv: I.bool() }; }
export type ActionKind = 'press' | 'each' | 'choice' | 'count' | 'reload';
const ActionKindWords: readonly ActionKind[] = ['press', 'each', 'choice', 'count', 'reload'];
function readActionKindIn(I: In): ActionKind { return I.word(ActionKindWords, 'ActionKind'); }
export interface ActionRow { name: string; label: string; section: string; title: string; kind: ActionKind; words: string[]; now: string; lo: number; hi: number; step: number; start: number; word: string }
export function readActionRowIn(I: In): ActionRow { return { name: I.str(), label: I.str(), section: I.str(), title: I.str(), kind: readActionKindIn(I), words: I.list(() => I.str()), now: I.str(), lo: I.float64(), hi: I.float64(), step: I.float64(), start: I.float64(), word: I.str() }; }
export interface Met { t: number; n: Vec3 }
export function readMetIn(I: In): Met { return { t: I.float64(), n: [I.float64(), I.float64(), I.float64()] }; }
export interface SoundPath { direct: boolean; routed: boolean; pathLen: number; via: Vec3; apparent: Vec3; muffle: number; carry: number }
export function readSoundPathIn(I: In): SoundPath { return { direct: I.bool(), routed: I.bool(), pathLen: I.float64(), via: [I.float64(), I.float64(), I.float64()], apparent: [I.float64(), I.float64(), I.float64()], muffle: I.float64(), carry: I.float64() }; }
export type SlotInfoSlot = 'auto' | 'manual';
const SlotInfoSlotWords: readonly SlotInfoSlot[] = ['auto', 'manual'];
function readSlotInfoSlotIn(I: In): SlotInfoSlot { return I.word(SlotInfoSlotWords, 'SlotInfoSlot'); }
export interface SaveRow { key: string; label: string; slot: SlotInfoSlot; regret: boolean; missionT: number; savedAt: number }
function readSaveRowIn(I: In): SaveRow { return { key: I.str(), label: I.str(), slot: readSlotInfoSlotIn(I), regret: I.bool(), missionT: I.float64(), savedAt: I.float64() }; }
export interface SavesView { elapsed: number; hand: boolean; offer: string; rows: SaveRow[] }
export function readSavesViewIn(I: In): SavesView { return { elapsed: I.float64(), hand: I.bool(), offer: I.str(), rows: I.list(() => readSaveRowIn(I)) }; }
export interface SavesTold { tag: string; lines: string[] }
export function readSavesToldIn(I: In): SavesTold { return { tag: I.str(), lines: I.list(() => I.str()) }; }
export interface Recovered { game: number; label: string }
export function readRecoveredIn(I: In): Recovered { return { game: I.uint32(), label: I.str() }; }
export interface CardPouches { used: number; total: number }
function readCardPouchesIn(I: In): CardPouches { return { used: I.float64(), total: I.float64() }; }
export interface CardLoad { mob: number; con: number }
function readCardLoadIn(I: In): CardLoad { return { mob: I.float64(), con: I.float64() }; }
export type CardKind = 'count' | 'flag' | 'choice';
const CardKindWords: readonly CardKind[] = ['count', 'flag', 'choice'];
function readCardKindIn(I: In): CardKind { return I.word(CardKindWords, 'CardKind'); }
export interface CardBar { now: number; issued: number }
function readCardBarIn(I: In): CardBar { return { now: I.float64(), issued: I.float64() }; }
export interface CardPick { id: string; word: string; note: string; why: string; now: boolean; can: boolean; icon: string; bars: number[] }
function readCardPickIn(I: In): CardPick { return { id: I.str(), word: I.str(), note: I.str(), why: I.str(), now: I.bool(), can: I.bool(), icon: I.str(), bars: I.list(() => I.float64()) }; }
export interface CardRow { field: string; label: string; kind: CardKind; value: string; n: number; word: string; note: string; cost: string; under: string; look: boolean; none: boolean; fixed: boolean; changed: boolean; idle: boolean; less: string; more: string; can: boolean; icon: string; noise: number; takes: CardLoad | null; would: CardLoad | null; bars: CardBar[]; picks: CardPick[] }
function readCardRowIn(I: In): CardRow { return { field: I.str(), label: I.str(), kind: readCardKindIn(I), value: I.str(), n: I.float64(), word: I.str(), note: I.str(), cost: I.str(), under: I.str(), look: I.bool(), none: I.bool(), fixed: I.bool(), changed: I.bool(), idle: I.bool(), less: I.str(), more: I.str(), can: I.bool(), icon: I.str(), noise: I.float64(), takes: I.bool() ? readCardLoadIn(I) : null, would: I.bool() ? readCardLoadIn(I) : null, bars: I.list(() => readCardBarIn(I)), picks: I.list(() => readCardPickIn(I)) }; }
export interface CardView { who: number; changed: boolean; pouches: CardPouches; big: CardPouches; load: CardLoad; loadIssued: CardLoad; rows: CardRow[]; issued: boolean; resetWhy: string; copied: boolean; clip: boolean; pasteWhy: string; words: string; suit: Vec3 | null; trim: Vec3 | null }
function readCardViewIn(I: In): CardView { return { who: I.float64(), changed: I.bool(), pouches: readCardPouchesIn(I), big: readCardPouchesIn(I), load: readCardLoadIn(I), loadIssued: readCardLoadIn(I), rows: I.list(() => readCardRowIn(I)), issued: I.bool(), resetWhy: I.str(), copied: I.bool(), clip: I.bool(), pasteWhy: I.str(), words: I.str(), suit: I.bool() ? [I.float64(), I.float64(), I.float64()] : null, trim: I.bool() ? [I.float64(), I.float64(), I.float64()] : null }; }
export function readCardView(b: Uint8Array): CardView { const I = new In(b), r = readCardViewIn(I); if (I.p !== b.length) throw new Error(`the view: CardView is ${b.length} bytes and ${I.p} were read`); return r; }
export interface KeptText { key: string; text: string }
function readKeptTextIn(I: In): KeptText { return { key: I.str(), text: I.str() }; }
export interface Pressed { ok: boolean; why: string; passed: string; changed: string[]; kept: KeptText[] }
function readPressedIn(I: In): Pressed { return { ok: I.bool(), why: I.str(), passed: I.str(), changed: I.list(() => I.str()), kept: I.list(() => readKeptTextIn(I)) }; }
export function readPressed(b: Uint8Array): Pressed { const I = new In(b), r = readPressedIn(I); if (I.p !== b.length) throw new Error(`the view: Pressed is ${b.length} bytes and ${I.p} were read`); return r; }
export interface TeamSize { n: number; ok: boolean; own: boolean; words: string; note: string }
function readTeamSizeIn(I: In): TeamSize { return { n: I.float64(), ok: I.bool(), own: I.bool(), words: I.str(), note: I.str() }; }
export interface TeamKit { id: string; name: string; now: boolean; offered: boolean }
function readTeamKitIn(I: In): TeamKit { return { id: I.str(), name: I.str(), now: I.bool(), offered: I.bool() }; }
export interface TeamThing { field: string; value: string; label: string; n: number; can: boolean; icon: string }
function readTeamThingIn(I: In): TeamThing { return { field: I.str(), value: I.str(), label: I.str(), n: I.float64(), can: I.bool(), icon: I.str() }; }
export interface TeamMan { here: boolean; title: string; sub: string; pair: string; changed: boolean; words: string; pouches: CardPouches; gun: string; icon: string; can: boolean; clip: boolean; things: TeamThing[]; worn: string }
function readTeamManIn(I: In): TeamMan { return { here: I.bool(), title: I.str(), sub: I.str(), pair: I.str(), changed: I.bool(), words: I.str(), pouches: readCardPouchesIn(I), gun: I.str(), icon: I.str(), can: I.bool(), clip: I.bool(), things: I.list(() => readTeamThingIn(I)), worn: I.str() }; }
export interface TeamView { size: number; sizes: TeamSize[]; note: string; cards: boolean; asIssued: boolean; pool: string; walls: boolean; kits: TeamKit[]; men: TeamMan[]; frags: number; smoke: number; bangs: number; link: string }
function readTeamViewIn(I: In): TeamView { return { size: I.float64(), sizes: I.list(() => readTeamSizeIn(I)), note: I.str(), cards: I.bool(), asIssued: I.bool(), pool: I.str(), walls: I.bool(), kits: I.list(() => readTeamKitIn(I)), men: I.list(() => readTeamManIn(I)), frags: I.float64(), smoke: I.float64(), bangs: I.float64(), link: I.str() }; }
export function readTeamView(b: Uint8Array): TeamView { const I = new In(b), r = readTeamViewIn(I); if (I.p !== b.length) throw new Error(`the view: TeamView is ${b.length} bytes and ${I.p} were read`); return r; }
export interface StartRow { id: string; title: string; kind: string; group: string; groupTitle: string; groupLine: string; place: string; blurb: string; lean: string; leanWords: string; level: string; levelWords: string; squad: number; teamWords: string; teamTag: string; unplayed: boolean; offList: boolean; asIssued: boolean; lesson: boolean; two: boolean; ways: string[]; pick: string; line: string; pickSquad: number; link: string }
function readStartRowIn(I: In): StartRow { return { id: I.str(), title: I.str(), kind: I.str(), group: I.str(), groupTitle: I.str(), groupLine: I.str(), place: I.str(), blurb: I.str(), lean: I.str(), leanWords: I.str(), level: I.str(), levelWords: I.str(), squad: I.float64(), teamWords: I.str(), teamTag: I.str(), unplayed: I.bool(), offList: I.bool(), asIssued: I.bool(), lesson: I.bool(), two: I.bool(), ways: I.list(() => I.str()), pick: I.str(), line: I.str(), pickSquad: I.float64(), link: I.str() }; }
export interface StartView { rows: StartRow[]; named: string; pick: string; playing: boolean; first: string }
function readStartViewIn(I: In): StartView { return { rows: I.list(() => readStartRowIn(I)), named: I.str(), pick: I.str(), playing: I.bool(), first: I.str() }; }
export function readStartView(b: Uint8Array): StartView { const I = new In(b), r = readStartViewIn(I); if (I.p !== b.length) throw new Error(`the view: StartView is ${b.length} bytes and ${I.p} were read`); return r; }
export interface SettingChoice { id: string; label: string; title: string; on: boolean; shown: boolean }
function readSettingChoiceIn(I: In): SettingChoice { return { id: I.str(), label: I.str(), title: I.str(), on: I.bool(), shown: I.bool() }; }
export interface Setting { key: string; label: string; live: boolean; shown: boolean; moved: boolean; note: string; choices: SettingChoice[] }
function readSettingIn(I: In): Setting { return { key: I.str(), label: I.str(), live: I.bool(), shown: I.bool(), moved: I.bool(), note: I.str(), choices: I.list(() => readSettingChoiceIn(I)) }; }
export interface SheetView { id: string; settings: Setting[]; link: string; base: string; asPick: boolean; quiet: boolean }
function readSheetViewIn(I: In): SheetView { return { id: I.str(), settings: I.list(() => readSettingIn(I)), link: I.str(), base: I.str(), asPick: I.bool(), quiet: I.bool() }; }
export function readSheetView(b: Uint8Array): SheetView { const I = new In(b), r = readSheetViewIn(I); if (I.p !== b.length) throw new Error(`the view: SheetView is ${b.length} bytes and ${I.p} were read`); return r; }
export interface ShortView { now: string; nowScenario: boolean; next: string; nextMission: string; title: string; line: string; link: string; otherLink: string }
function readShortViewIn(I: In): ShortView { return { now: I.str(), nowScenario: I.bool(), next: I.str(), nextMission: I.str(), title: I.str(), line: I.str(), link: I.str(), otherLink: I.str() }; }
export function readShortView(b: Uint8Array): ShortView { const I = new In(b), r = readShortViewIn(I); if (I.p !== b.length) throw new Error(`the view: ShortView is ${b.length} bytes and ${I.p} were read`); return r; }
export interface BriefView { id: string; title: string; kind: string; groupTitle: string; place: string; blurb: string; leanWords: string; team: string; level: string; scene: string; teaches: string[]; way: number }
function readBriefViewIn(I: In): BriefView { return { id: I.str(), title: I.str(), kind: I.str(), groupTitle: I.str(), place: I.str(), blurb: I.str(), leanWords: I.str(), team: I.str(), level: I.str(), scene: I.str(), teaches: I.list(() => I.str()), way: I.float64() }; }
export function readBriefView(b: Uint8Array): BriefView { const I = new In(b), r = readBriefViewIn(I); if (I.p !== b.length) throw new Error(`the view: BriefView is ${b.length} bytes and ${I.p} were read`); return r; }
export interface MissionRow { id: string; title: string; kind: string; squad: number; group: string; level: string; hour: string; place: string; blurb: string; scene: string; teaches: string[]; only: string; defenders: number; squads: boolean; ways: string[]; untuned: boolean; offList: boolean; line: string; link: string }
function readMissionRowIn(I: In): MissionRow { return { id: I.str(), title: I.str(), kind: I.str(), squad: I.float64(), group: I.str(), level: I.str(), hour: I.str(), place: I.str(), blurb: I.str(), scene: I.str(), teaches: I.list(() => I.str()), only: I.str(), defenders: I.float64(), squads: I.bool(), ways: I.list(() => I.str()), untuned: I.bool(), offList: I.bool(), line: I.str(), link: I.str() }; }
export interface MissionsView { rows: MissionRow[] }
function readMissionsViewIn(I: In): MissionsView { return { rows: I.list(() => readMissionRowIn(I)) }; }
export function readMissionsView(b: Uint8Array): MissionsView { const I = new In(b), r = readMissionsViewIn(I); if (I.p !== b.length) throw new Error(`the view: MissionsView is ${b.length} bytes and ${I.p} were read`); return r; }
