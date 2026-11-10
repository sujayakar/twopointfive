// THE VIEW, READ: the types of rust/game_types/src/view and their bytes, as plain values. Written by tools/draw/gen.ts. To change one, change it there.
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

export type MissionOf = 'stages' | 'script' | 'lesson' | 'roomclear' | 'rescue' | 'nightbreach';
const MissionOfWords: readonly MissionOf[] = ['stages', 'script', 'lesson', 'roomclear', 'rescue', 'nightbreach'];
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
export interface RailPrompt { line: string; n: number; of: number; note: string | null; watch: boolean }
function readRailPromptIn(I: In): RailPrompt { return { line: I.str(), n: I.uint32(), of: I.uint32(), note: I.bool() ? I.str() : null, watch: I.bool() }; }
export function readRailPrompt(b: Uint8Array): RailPrompt { const I = new In(b), r = readRailPromptIn(I); if (I.p !== b.length) throw new Error(`the view: RailPrompt is ${b.length} bytes and ${I.p} were read`); return r; }
export interface RailCard { title: string | null; text: string }
function readRailCardIn(I: In): RailCard { return { title: I.bool() ? I.str() : null, text: I.str() }; }
export function readRailCard(b: Uint8Array): RailCard { const I = new In(b), r = readRailCardIn(I); if (I.p !== b.length) throw new Error(`the view: RailCard is ${b.length} bytes and ${I.p} were read`); return r; }
export interface RailsView { prompt: RailPrompt | null; card: RailCard | null; arrow: Vec3 | null; lineMen: number[]; open: string[]; rows: string[] | null; orders: string[] | null; stalkShut: boolean; teachesHold: boolean; noLasers: boolean }
function readRailsViewIn(I: In): RailsView { return { prompt: I.bool() ? readRailPromptIn(I) : null, card: I.bool() ? readRailCardIn(I) : null, arrow: I.bool() ? [I.float64(), I.float64(), I.float64()] : null, lineMen: I.list(() => I.float64()), open: I.list(() => I.str()), rows: I.bool() ? I.list(() => I.str()) : null, orders: I.bool() ? I.list(() => I.str()) : null, stalkShut: I.bool(), teachesHold: I.bool(), noLasers: I.bool() }; }
export function readRailsView(b: Uint8Array): RailsView { const I = new In(b), r = readRailsViewIn(I); if (I.p !== b.length) throw new Error(`the view: RailsView is ${b.length} bytes and ${I.p} were read`); return r; }
export interface LessonView { step: number; of: number; id: string | null; finished: boolean; over: boolean; settling: boolean; asksView: boolean; rails: RailsView | null }
function readLessonViewIn(I: In): LessonView { return { step: I.uint32(), of: I.uint32(), id: I.bool() ? I.str() : null, finished: I.bool(), over: I.bool(), settling: I.bool(), asksView: I.bool(), rails: I.bool() ? readRailsViewIn(I) : null }; }
export function readLessonView(b: Uint8Array): LessonView { const I = new In(b), r = readLessonViewIn(I); if (I.p !== b.length) throw new Error(`the view: LessonView is ${b.length} bytes and ${I.p} were read`); return r; }
export type KitThing = 'hands' | 'pistol' | 'rifle' | 'smoke' | 'flash' | 'frag';
const KitThingWords: readonly KitThing[] = ['hands', 'pistol', 'rifle', 'smoke', 'flash', 'frag'];
function readKitThingIn(I: In): KitThing { return I.word(KitThingWords, 'KitThing'); }
export interface KitSlot { thing: KitThing; key: string; name: string; count: number; absent: boolean; spent: boolean; up: boolean; taken: boolean }
function readKitSlotIn(I: In): KitSlot { return { thing: readKitThingIn(I), key: I.str(), name: I.str(), count: I.float64(), absent: I.bool(), spent: I.bool(), up: I.bool(), taken: I.bool() }; }
export function readKitSlot(b: Uint8Array): KitSlot { const I = new In(b), r = readKitSlotIn(I); if (I.p !== b.length) throw new Error(`the view: KitSlot is ${b.length} bytes and ${I.p} were read`); return r; }
export interface KitWord { text: string; on: boolean; out: boolean }
function readKitWordIn(I: In): KitWord { return { text: I.str(), on: I.bool(), out: I.bool() }; }
export function readKitWord(b: Uint8Array): KitWord { const I = new In(b), r = readKitWordIn(I); if (I.p !== b.length) throw new Error(`the view: KitWord is ${b.length} bytes and ${I.p} were read`); return r; }
export interface KitOcp { charge: number; ready: boolean }
function readKitOcpIn(I: In): KitOcp { return { charge: I.float64(), ready: I.bool() }; }
export function readKitOcp(b: Uint8Array): KitOcp { const I = new In(b), r = readKitOcpIn(I); if (I.p !== b.length) throw new Error(`the view: KitOcp is ${b.length} bytes and ${I.p} were read`); return r; }
export interface KitGun { mag: number; capacity: number; chamber: boolean; ready: number; dry: boolean; reloading: boolean; selector: string | null; load: KitWord | null; light: boolean; ocp: KitOcp | null; spares: number[]; loose: number | null }
function readKitGunIn(I: In): KitGun { return { mag: I.float64(), capacity: I.float64(), chamber: I.bool(), ready: I.float64(), dry: I.bool(), reloading: I.bool(), selector: I.bool() ? I.str() : null, load: I.bool() ? readKitWordIn(I) : null, light: I.bool(), ocp: I.bool() ? readKitOcpIn(I) : null, spares: I.list(() => I.float64()), loose: I.bool() ? I.float64() : null }; }
export function readKitGun(b: Uint8Array): KitGun { const I = new In(b), r = readKitGunIn(I); if (I.p !== b.length) throw new Error(`the view: KitGun is ${b.length} bytes and ${I.p} were read`); return r; }
export interface KitSetupRow { key: string; words: KitWord[] }
function readKitSetupRowIn(I: In): KitSetupRow { return { key: I.str(), words: I.list(() => readKitWordIn(I)) }; }
export function readKitSetupRow(b: Uint8Array): KitSetupRow { const I = new In(b), r = readKitSetupRowIn(I); if (I.p !== b.length) throw new Error(`the view: KitSetupRow is ${b.length} bytes and ${I.p} were read`); return r; }
export interface KitSetup { rows: KitSetupRow[]; back: string }
function readKitSetupIn(I: In): KitSetup { return { rows: I.list(() => readKitSetupRowIn(I)), back: I.str() }; }
export function readKitSetup(b: Uint8Array): KitSetup { const I = new In(b), r = readKitSetupIn(I); if (I.p !== b.length) throw new Error(`the view: KitSetup is ${b.length} bytes and ${I.p} were read`); return r; }
export interface KitBig { name: string; place: string; offered: boolean }
function readKitBigIn(I: In): KitBig { return { name: I.str(), place: I.str(), offered: I.bool() }; }
export function readKitBig(b: Uint8Array): KitBig { const I = new In(b), r = readKitBigIn(I); if (I.p !== b.length) throw new Error(`the view: KitBig is ${b.length} bytes and ${I.p} were read`); return r; }
export interface KitView { slots: KitSlot[]; gun: KitGun | null; setup: KitSetup | null; big: KitBig | null; hitsLeft: number; hitsMax: number; down: boolean; god: boolean; nightVision: boolean | null; keyTag: string | null }
function readKitViewIn(I: In): KitView { return { slots: I.list(() => readKitSlotIn(I)), gun: I.bool() ? readKitGunIn(I) : null, setup: I.bool() ? readKitSetupIn(I) : null, big: I.bool() ? readKitBigIn(I) : null, hitsLeft: I.float64(), hitsMax: I.float64(), down: I.bool(), god: I.bool(), nightVision: I.bool() ? I.bool() : null, keyTag: I.bool() ? I.str() : null }; }
export function readKitView(b: Uint8Array): KitView { const I = new In(b), r = readKitViewIn(I); if (I.p !== b.length) throw new Error(`the view: KitView is ${b.length} bytes and ${I.p} were read`); return r; }
export type HeadTone = 'calm' | 'ko' | 'dead' | 'alert' | 'sus';
const HeadToneWords: readonly HeadTone[] = ['calm', 'ko', 'dead', 'alert', 'sus'];
function readHeadToneIn(I: In): HeadTone { return I.word(HeadToneWords, 'HeadTone'); }
export interface ManMark { man: number; text: string; tone: HeadTone; civ: boolean; hidden: boolean; debug: string }
function readManMarkIn(I: In): ManMark { return { man: I.float64(), text: I.str(), tone: readHeadToneIn(I), civ: I.bool(), hidden: I.bool(), debug: I.str() }; }
export function readManMark(b: Uint8Array): ManMark { const I = new In(b), r = readManMarkIn(I); if (I.p !== b.length) throw new Error(`the view: ManMark is ${b.length} bytes and ${I.p} were read`); return r; }
export interface SeenMark { man: number; at: Vec3; age: number; life: number }
function readSeenMarkIn(I: In): SeenMark { return { man: I.float64(), at: [I.float64(), I.float64(), I.float64()], age: I.float64(), life: I.float64() }; }
export function readSeenMark(b: Uint8Array): SeenMark { const I = new In(b), r = readSeenMarkIn(I); if (I.p !== b.length) throw new Error(`the view: SeenMark is ${b.length} bytes and ${I.p} were read`); return r; }
export interface CamMark { at: Vec3; sends: boolean }
function readCamMarkIn(I: In): CamMark { return { at: [I.float64(), I.float64(), I.float64()], sends: I.bool() }; }
export function readCamMark(b: Uint8Array): CamMark { const I = new In(b), r = readCamMarkIn(I); if (I.p !== b.length) throw new Error(`the view: CamMark is ${b.length} bytes and ${I.p} were read`); return r; }
export interface FragMark { at: Vec3; left: number }
function readFragMarkIn(I: In): FragMark { return { at: [I.float64(), I.float64(), I.float64()], left: I.float64() }; }
export function readFragMark(b: Uint8Array): FragMark { const I = new In(b), r = readFragMarkIn(I); if (I.p !== b.length) throw new Error(`the view: FragMark is ${b.length} bytes and ${I.p} were read`); return r; }
export type FloorWord = 'carpet' | 'lino' | 'stone' | 'tile' | 'asphalt' | 'grass';
const FloorWordWords: readonly FloorWord[] = ['carpet', 'lino', 'stone', 'tile', 'asphalt', 'grass'];
function readFloorWordIn(I: In): FloorWord { return I.word(FloorWordWords, 'FloorWord'); }
export interface MarksView { men: ManMark[]; seen: SeenMark[]; cams: CamMark[]; frags: FragMark[]; blind: number; light: number; irradiance: number; noise: number; stalk: number | null; stalkWaits: boolean; floor: FloorWord | null }
function readMarksViewIn(I: In): MarksView { return { men: I.list(() => readManMarkIn(I)), seen: I.list(() => readSeenMarkIn(I)), cams: I.list(() => readCamMarkIn(I)), frags: I.list(() => readFragMarkIn(I)), blind: I.float64(), light: I.float64(), irradiance: I.float64(), noise: I.float64(), stalk: I.bool() ? I.float64() : null, stalkWaits: I.bool(), floor: I.bool() ? readFloorWordIn(I) : null }; }
export function readMarksView(b: Uint8Array): MarksView { const I = new In(b), r = readMarksViewIn(I); if (I.p !== b.length) throw new Error(`the view: MarksView is ${b.length} bytes and ${I.p} were read`); return r; }
export type RingOn = 'free' | 'light' | 'guard';
const RingOnWords: readonly RingOn[] = ['free', 'light', 'guard'];
function readRingOnIn(I: In): RingOn { return I.word(RingOnWords, 'RingOn'); }
export interface AimCone { spread: number; track: number; range: number; hands: number; settle: number; crossing: number; feet: number }
function readAimConeIn(I: In): AimCone { return { spread: I.float64(), track: I.float64(), range: I.float64(), hands: I.float64(), settle: I.float64(), crossing: I.float64(), feet: I.float64() }; }
export function readAimCone(b: Uint8Array): AimCone { const I = new In(b), r = readAimConeIn(I); if (I.p !== b.length) throw new Error(`the view: AimCone is ${b.length} bytes and ${I.p} were read`); return r; }
export interface AimRing { at: Vec3; on: RingOn; trail: boolean; shut: boolean; wish: Vec3 | null; cone: AimCone | null }
function readAimRingIn(I: In): AimRing { return { at: [I.float64(), I.float64(), I.float64()], on: readRingOnIn(I), trail: I.bool(), shut: I.bool(), wish: I.bool() ? [I.float64(), I.float64(), I.float64()] : null, cone: I.bool() ? readAimConeIn(I) : null }; }
export function readAimRing(b: Uint8Array): AimRing { const I = new In(b), r = readAimRingIn(I); if (I.p !== b.length) throw new Error(`the view: AimRing is ${b.length} bytes and ${I.p} were read`); return r; }
export interface GunDot { at: Vec3; shut: boolean }
function readGunDotIn(I: In): GunDot { return { at: [I.float64(), I.float64(), I.float64()], shut: I.bool() }; }
export function readGunDot(b: Uint8Array): GunDot { const I = new In(b), r = readGunDotIn(I); if (I.p !== b.length) throw new Error(`the view: GunDot is ${b.length} bytes and ${I.p} were read`); return r; }
export interface ThrowReachView { at: Vec3; kill: number; hurt: number; stun: number }
function readThrowReachViewIn(I: In): ThrowReachView { return { at: [I.float64(), I.float64(), I.float64()], kill: I.float64(), hurt: I.float64(), stun: I.float64() }; }
export function readThrowReachView(b: Uint8Array): ThrowReachView { const I = new In(b), r = readThrowReachViewIn(I); if (I.p !== b.length) throw new Error(`the view: ThrowReachView is ${b.length} bytes and ${I.p} were read`); return r; }
export interface ThrowRest { at: Vec3; path: Vec3[] }
function readThrowRestIn(I: In): ThrowRest { return { at: [I.float64(), I.float64(), I.float64()], path: I.list(() => [I.float64(), I.float64(), I.float64()]) }; }
export function readThrowRest(b: Uint8Array): ThrowRest { const I = new In(b), r = readThrowRestIn(I); if (I.p !== b.length) throw new Error(`the view: ThrowRest is ${b.length} bytes and ${I.p} were read`); return r; }
export interface ThrowView { points: Vec3[]; land: Vec3; short: boolean; rest: ThrowRest | null; reach: ThrowReachView | null }
function readThrowViewIn(I: In): ThrowView { return { points: I.list(() => [I.float64(), I.float64(), I.float64()]), land: [I.float64(), I.float64(), I.float64()], short: I.bool(), rest: I.bool() ? readThrowRestIn(I) : null, reach: I.bool() ? readThrowReachViewIn(I) : null }; }
export function readThrowView(b: Uint8Array): ThrowView { const I = new In(b), r = readThrowViewIn(I); if (I.p !== b.length) throw new Error(`the view: ThrowView is ${b.length} bytes and ${I.p} were read`); return r; }
export interface AimView { ring: AimRing | null; dot: GunDot | null; tooClose: boolean; throw: ThrowView | null }
function readAimViewIn(I: In): AimView { return { ring: I.bool() ? readAimRingIn(I) : null, dot: I.bool() ? readGunDotIn(I) : null, tooClose: I.bool(), throw: I.bool() ? readThrowViewIn(I) : null }; }
export function readAimView(b: Uint8Array): AimView { const I = new In(b), r = readAimViewIn(I); if (I.p !== b.length) throw new Error(`the view: AimView is ${b.length} bytes and ${I.p} were read`); return r; }
export type BigCarried = 'shotgun' | 'hammer';
const BigCarriedWords: readonly BigCarried[] = ['shotgun', 'hammer'];
function readBigCarriedIn(I: In): BigCarried { return I.word(BigCarriedWords, 'BigCarried'); }
export interface WornView { man: number; goggles: boolean; big: BigCarried | null }
function readWornViewIn(I: In): WornView { return { man: I.float64(), goggles: I.bool(), big: I.bool() ? readBigCarriedIn(I) : null }; }
export function readWornView(b: Uint8Array): WornView { const I = new In(b), r = readWornViewIn(I); if (I.p !== b.length) throw new Error(`the view: WornView is ${b.length} bytes and ${I.p} were read`); return r; }
export type Pair = 'red' | 'blue';
const PairWords: readonly Pair[] = ['red', 'blue'];
function readPairIn(I: In): Pair { return I.word(PairWords, 'Pair'); }
export interface MateLine { man: number; name: string; doing: string; addressed: boolean; pair: Pair | null; down: boolean; kit: string }
function readMateLineIn(I: In): MateLine { return { man: I.float64(), name: I.str(), doing: I.str(), addressed: I.bool(), pair: I.bool() ? readPairIn(I) : null, down: I.bool(), kit: I.str() }; }
export function readMateLine(b: Uint8Array): MateLine { const I = new In(b), r = readMateLineIn(I); if (I.p !== b.length) throw new Error(`the view: MateLine is ${b.length} bytes and ${I.p} were read`); return r; }
export interface Note { key: string; text: string }
function readNoteIn(I: In): Note { return { key: I.str(), text: I.str() }; }
export function readNote(b: Uint8Array): Note { const I = new In(b), r = readNoteIn(I); if (I.p !== b.length) throw new Error(`the view: Note is ${b.length} bytes and ${I.p} were read`); return r; }
export interface SquadView { posture: string | null; who: string; picked: boolean; mates: MateLine[]; notes: Note[]; quick: Note | null }
function readSquadViewIn(I: In): SquadView { return { posture: I.bool() ? I.str() : null, who: I.str(), picked: I.bool(), mates: I.list(() => readMateLineIn(I)), notes: I.list(() => readNoteIn(I)), quick: I.bool() ? readNoteIn(I) : null }; }
export function readSquadView(b: Uint8Array): SquadView { const I = new In(b), r = readSquadViewIn(I); if (I.p !== b.length) throw new Error(`the view: SquadView is ${b.length} bytes and ${I.p} were read`); return r; }
export type ThrowKind = 'frag' | 'stun' | 'shell';
const ThrowKindWords: readonly ThrowKind[] = ['frag', 'stun', 'shell'];
function readThrowKindIn(I: In): ThrowKind { return I.word(ThrowKindWords, 'ThrowKind'); }
export type Tone = 'red' | 'blue' | 'gold';
const ToneWords: readonly Tone[] = ['red', 'blue', 'gold'];
function readToneIn(I: In): Tone { return I.word(ToneWords, 'Tone'); }
export interface ThrowLine { man: number; kind: ThrowKind; from: Vec3; points: Vec3[]; land: Vec3; walk: Vec3[] | null; preview: boolean; kill: number; wound: number; tone: Tone | null }
function readThrowLineIn(I: In): ThrowLine { return { man: I.float64(), kind: readThrowKindIn(I), from: [I.float64(), I.float64(), I.float64()], points: I.list(() => [I.float64(), I.float64(), I.float64()]), land: [I.float64(), I.float64(), I.float64()], walk: I.bool() ? I.list(() => [I.float64(), I.float64(), I.float64()]) : null, preview: I.bool(), kill: I.float64(), wound: I.float64(), tone: I.bool() ? readToneIn(I) : null }; }
export function readThrowLine(b: Uint8Array): ThrowLine { const I = new In(b), r = readThrowLineIn(I); if (I.p !== b.length) throw new Error(`the view: ThrowLine is ${b.length} bytes and ${I.p} were read`); return r; }
export interface RingWedge { key: number; text: string; off: boolean; opens: boolean }
function readRingWedgeIn(I: In): RingWedge { return { key: I.float64(), text: I.str(), off: I.bool(), opens: I.bool() }; }
export function readRingWedge(b: Uint8Array): RingWedge { const I = new In(b), r = readRingWedgeIn(I); if (I.p !== b.length) throw new Error(`the view: RingWedge is ${b.length} bytes and ${I.p} were read`); return r; }
export interface RingPin { at: Vec3; ox: number; oy: number; edgeX: number; edgeY: number }
function readRingPinIn(I: In): RingPin { return { at: [I.float64(), I.float64(), I.float64()], ox: I.float64(), oy: I.float64(), edgeX: I.float64(), edgeY: I.float64() }; }
export function readRingPin(b: Uint8Array): RingPin { const I = new In(b), r = readRingPinIn(I); if (I.p !== b.length) throw new Error(`the view: RingPin is ${b.length} bytes and ${I.p} were read`); return r; }
export interface RingView { x: number; y: number; zoom: number; lit: number; tone: Tone | null; caption: string; wedges: RingWedge[]; pin: RingPin | null }
function readRingViewIn(I: In): RingView { return { x: I.float64(), y: I.float64(), zoom: I.float64(), lit: I.float64(), tone: I.bool() ? readToneIn(I) : null, caption: I.str(), wedges: I.list(() => readRingWedgeIn(I)), pin: I.bool() ? readRingPinIn(I) : null }; }
export function readRingView(b: Uint8Array): RingView { const I = new In(b), r = readRingViewIn(I); if (I.p !== b.length) throw new Error(`the view: RingView is ${b.length} bytes and ${I.p} were read`); return r; }
export interface HandRingView { x: number; y: number; zoom: number; lit: number; wedges: RingWedge[]; pin: RingPin | null }
function readHandRingViewIn(I: In): HandRingView { return { x: I.float64(), y: I.float64(), zoom: I.float64(), lit: I.float64(), wedges: I.list(() => readRingWedgeIn(I)), pin: I.bool() ? readRingPinIn(I) : null }; }
export function readHandRingView(b: Uint8Array): HandRingView { const I = new In(b), r = readHandRingViewIn(I); if (I.p !== b.length) throw new Error(`the view: HandRingView is ${b.length} bytes and ${I.p} were read`); return r; }
export type ThingKind = 'cam' | 'guard' | 'breaker' | 'door' | 'light' | 'body' | 'objective' | 'held' | 'wall' | 'switch' | 'charge' | 'cancel';
const ThingKindWords: readonly ThingKind[] = ['cam', 'guard', 'breaker', 'door', 'light', 'body', 'objective', 'held', 'wall', 'switch', 'charge', 'cancel'];
function readThingKindIn(I: In): ThingKind { return I.word(ThingKindWords, 'ThingKind'); }
export interface EmptyView { word: string; kind: ThingKind; dist: number | null }
function readEmptyViewIn(I: In): EmptyView { return { word: I.str(), kind: readThingKindIn(I), dist: I.bool() ? I.float64() : null }; }
export function readEmptyView(b: Uint8Array): EmptyView { const I = new In(b), r = readEmptyViewIn(I); if (I.p !== b.length) throw new Error(`the view: EmptyView is ${b.length} bytes and ${I.p} were read`); return r; }
export interface PlanWay { from: Vec3; own: Vec3[] | null; stops: Vec3[] }
function readPlanWayIn(I: In): PlanWay { return { from: [I.float64(), I.float64(), I.float64()], own: I.bool() ? I.list(() => [I.float64(), I.float64(), I.float64()]) : null, stops: I.list(() => [I.float64(), I.float64(), I.float64()]) }; }
export function readPlanWay(b: Uint8Array): PlanWay { const I = new In(b), r = readPlanWayIn(I); if (I.p !== b.length) throw new Error(`the view: PlanWay is ${b.length} bytes and ${I.p} were read`); return r; }
export interface PlanMan { man: number; tag: string; tone: Tone | null; age: number; way: PlanWay | null; mark: Vec3; there: boolean; held: boolean; aim: number | null; watch: Vec3 | null; onDoor: boolean }
function readPlanManIn(I: In): PlanMan { return { man: I.float64(), tag: I.str(), tone: I.bool() ? readToneIn(I) : null, age: I.float64(), way: I.bool() ? readPlanWayIn(I) : null, mark: [I.float64(), I.float64(), I.float64()], there: I.bool(), held: I.bool(), aim: I.bool() ? I.float64() : null, watch: I.bool() ? [I.float64(), I.float64(), I.float64()] : null, onDoor: I.bool() }; }
export function readPlanMan(b: Uint8Array): PlanMan { const I = new In(b), r = readPlanManIn(I); if (I.p !== b.length) throw new Error(`the view: PlanMan is ${b.length} bytes and ${I.p} were read`); return r; }
export interface PlanWho { man: number; at: Vec3; tone: Tone | null }
function readPlanWhoIn(I: In): PlanWho { return { man: I.float64(), at: [I.float64(), I.float64(), I.float64()], tone: I.bool() ? readToneIn(I) : null }; }
export function readPlanWho(b: Uint8Array): PlanWho { const I = new In(b), r = readPlanWhoIn(I); if (I.p !== b.length) throw new Error(`the view: PlanWho is ${b.length} bytes and ${I.p} were read`); return r; }
export type KeyCap = 'none' | 'dark' | 'lit';
const KeyCapWords: readonly KeyCap[] = ['none', 'dark', 'lit'];
function readKeyCapIn(I: In): KeyCap { return I.word(KeyCapWords, 'KeyCap'); }
export type WaitWhy = 'stack' | 'tool' | 'charge' | 'thr' | 'drill' | 'wall' | 'pair';
const WaitWhyWords: readonly WaitWhy[] = ['stack', 'tool', 'charge', 'thr', 'drill', 'wall', 'pair'];
function readWaitWhyIn(I: In): WaitWhy { return I.word(WaitWhyWords, 'WaitWhy'); }
export interface PlanDoor { door: string | null; a: Vec3; b: Vec3; tone: Tone | null; key: string | null; now: boolean; cap: KeyCap; why: WaitWhy }
function readPlanDoorIn(I: In): PlanDoor { return { door: I.bool() ? I.str() : null, a: [I.float64(), I.float64(), I.float64()], b: [I.float64(), I.float64(), I.float64()], tone: I.bool() ? readToneIn(I) : null, key: I.bool() ? I.str() : null, now: I.bool(), cap: readKeyCapIn(I), why: readWaitWhyIn(I) }; }
export function readPlanDoor(b: Uint8Array): PlanDoor { const I = new In(b), r = readPlanDoorIn(I); if (I.p !== b.length) throw new Error(`the view: PlanDoor is ${b.length} bytes and ${I.p} were read`); return r; }
export interface PlanThrow { man: number; tag: string; tone: Tone | null; kind: ThrowKind; preview: boolean; from: Vec3; walk: Vec3[]; arc: Vec3[]; land: Vec3; reach: number }
function readPlanThrowIn(I: In): PlanThrow { return { man: I.float64(), tag: I.str(), tone: I.bool() ? readToneIn(I) : null, kind: readThrowKindIn(I), preview: I.bool(), from: [I.float64(), I.float64(), I.float64()], walk: I.list(() => [I.float64(), I.float64(), I.float64()]), arc: I.list(() => [I.float64(), I.float64(), I.float64()]), land: [I.float64(), I.float64(), I.float64()], reach: I.float64() }; }
export function readPlanThrow(b: Uint8Array): PlanThrow { const I = new In(b), r = readPlanThrowIn(I); if (I.p !== b.length) throw new Error(`the view: PlanThrow is ${b.length} bytes and ${I.p} were read`); return r; }
export interface SeatPlan { men: PlanMan[]; who: PlanWho[]; you: Vec3 | null; doors: PlanDoor[]; throws: PlanThrow[]; mode: boolean }
function readSeatPlanIn(I: In): SeatPlan { return { men: I.list(() => readPlanManIn(I)), who: I.list(() => readPlanWhoIn(I)), you: I.bool() ? [I.float64(), I.float64(), I.float64()] : null, doors: I.list(() => readPlanDoorIn(I)), throws: I.list(() => readPlanThrowIn(I)), mode: I.bool() }; }
export function readSeatPlan(b: Uint8Array): SeatPlan { const I = new In(b), r = readSeatPlanIn(I); if (I.p !== b.length) throw new Error(`the view: SeatPlan is ${b.length} bytes and ${I.p} were read`); return r; }
export interface PlanView { own: SeatPlan; others: SeatPlan[] }
function readPlanViewIn(I: In): PlanView { return { own: readSeatPlanIn(I), others: I.list(() => readSeatPlanIn(I)) }; }
export function readPlanView(b: Uint8Array): PlanView { const I = new In(b), r = readPlanViewIn(I); if (I.p !== b.length) throw new Error(`the view: PlanView is ${b.length} bytes and ${I.p} were read`); return r; }
export interface RowOpt { key: string; text: string }
function readRowOptIn(I: In): RowOpt { return { key: I.str(), text: I.str() }; }
export function readRowOpt(b: Uint8Array): RowOpt { const I = new In(b), r = readRowOptIn(I); if (I.p !== b.length) throw new Error(`the view: RowOpt is ${b.length} bytes and ${I.p} were read`); return r; }
export interface ListRow { kind: ThingKind; label: string; opts: RowOpt[]; inReach: boolean; off: boolean; dist: number | null; progress: number | null; cont: boolean }
function readListRowIn(I: In): ListRow { return { kind: readThingKindIn(I), label: I.str(), opts: I.list(() => readRowOptIn(I)), inReach: I.bool(), off: I.bool(), dist: I.bool() ? I.float64() : null, progress: I.bool() ? I.float64() : null, cont: I.bool() }; }
export function readListRow(b: Uint8Array): ListRow { const I = new In(b), r = readListRowIn(I); if (I.p !== b.length) throw new Error(`the view: ListRow is ${b.length} bytes and ${I.p} were read`); return r; }
export interface ThingRing { kind: ThingKind; at: Vec3; man: number | null; up: number; off: boolean; inReach: boolean; progress: number | null }
function readThingRingIn(I: In): ThingRing { return { kind: readThingKindIn(I), at: [I.float64(), I.float64(), I.float64()], man: I.bool() ? I.float64() : null, up: I.float64(), off: I.bool(), inReach: I.bool(), progress: I.bool() ? I.float64() : null }; }
export function readThingRing(b: Uint8Array): ThingRing { const I = new In(b), r = readThingRingIn(I); if (I.p !== b.length) throw new Error(`the view: ThingRing is ${b.length} bytes and ${I.p} were read`); return r; }
export interface ThingDot { kind: ThingKind; at: Vec3; man: number | null; up: number; off: boolean; hidden: boolean; ringed: boolean }
function readThingDotIn(I: In): ThingDot { return { kind: readThingKindIn(I), at: [I.float64(), I.float64(), I.float64()], man: I.bool() ? I.float64() : null, up: I.float64(), off: I.bool(), hidden: I.bool(), ringed: I.bool() }; }
export function readThingDot(b: Uint8Array): ThingDot { const I = new In(b), r = readThingDotIn(I); if (I.p !== b.length) throw new Error(`the view: ThingDot is ${b.length} bytes and ${I.p} were read`); return r; }
export interface InteractView { rows: ListRow[]; lit: number; open: boolean; ring: ThingRing | null; dots: ThingDot[] }
function readInteractViewIn(I: In): InteractView { return { rows: I.list(() => readListRowIn(I)), lit: I.float64(), open: I.bool(), ring: I.bool() ? readThingRingIn(I) : null, dots: I.list(() => readThingDotIn(I)) }; }
export function readInteractView(b: Uint8Array): InteractView { const I = new In(b), r = readInteractViewIn(I); if (I.p !== b.length) throw new Error(`the view: InteractView is ${b.length} bytes and ${I.p} were read`); return r; }
export interface AlarmView { level: number; left: number; live: boolean }
function readAlarmViewIn(I: In): AlarmView { return { level: I.float64(), left: I.float64(), live: I.bool() }; }
export function readAlarmView(b: Uint8Array): AlarmView { const I = new In(b), r = readAlarmViewIn(I); if (I.p !== b.length) throw new Error(`the view: AlarmView is ${b.length} bytes and ${I.p} were read`); return r; }
export type Stage = 'drive' | 'infiltrate' | 'exfil' | 'done' | 'failed';
const StageWords: readonly Stage[] = ['drive', 'infiltrate', 'exfil', 'done', 'failed'];
function readStageIn(I: In): Stage { return I.word(StageWords, 'Stage'); }
export interface LoneLook { r: number; h: number; pad: number; letter: number; max: number; lead: number }
function readLoneLookIn(I: In): LoneLook { return { r: I.float64(), h: I.float64(), pad: I.float64(), letter: I.float64(), max: I.float64(), lead: I.float64() }; }
export function readLoneLook(b: Uint8Array): LoneLook { const I = new In(b), r = readLoneLookIn(I); if (I.p !== b.length) throw new Error(`the view: LoneLook is ${b.length} bytes and ${I.p} were read`); return r; }
export interface RingLook { places: number; hole: number; rim: number; boxX: number; boxY: number; lone: LoneLook }
function readRingLookIn(I: In): RingLook { return { places: I.float64(), hole: I.float64(), rim: I.float64(), boxX: I.float64(), boxY: I.float64(), lone: readLoneLookIn(I) }; }
export function readRingLook(b: Uint8Array): RingLook { const I = new In(b), r = readRingLookIn(I); if (I.p !== b.length) throw new Error(`the view: RingLook is ${b.length} bytes and ${I.p} were read`); return r; }
export interface PlanLook { freshSecs: number; walkDim: number; restDim: number; ringM: number; sectorM: number; sectorDeg: number; reaskSecs: number; others: number }
function readPlanLookIn(I: In): PlanLook { return { freshSecs: I.float64(), walkDim: I.float64(), restDim: I.float64(), ringM: I.float64(), sectorM: I.float64(), sectorDeg: I.float64(), reaskSecs: I.float64(), others: I.float64() }; }
export function readPlanLook(b: Uint8Array): PlanLook { const I = new In(b), r = readPlanLookIn(I); if (I.p !== b.length) throw new Error(`the view: PlanLook is ${b.length} bytes and ${I.p} were read`); return r; }
export interface NetLook { edge: boolean; near: number; edgeIn: number }
function readNetLookIn(I: In): NetLook { return { edge: I.bool(), near: I.float64(), edgeIn: I.float64() }; }
export function readNetLook(b: Uint8Array): NetLook { const I = new In(b), r = readNetLookIn(I); if (I.p !== b.length) throw new Error(`the view: NetLook is ${b.length} bytes and ${I.p} were read`); return r; }
export interface AttentionLook { ringM: number; wayReask: number }
function readAttentionLookIn(I: In): AttentionLook { return { ringM: I.float64(), wayReask: I.float64() }; }
export function readAttentionLook(b: Uint8Array): AttentionLook { const I = new In(b), r = readAttentionLookIn(I); if (I.p !== b.length) throw new Error(`the view: AttentionLook is ${b.length} bytes and ${I.p} were read`); return r; }
export type BackKey = 'tab' | 'esc';
const BackKeyWords: readonly BackKey[] = ['tab', 'esc'];
function readBackKeyIn(I: In): BackKey { return I.word(BackKeyWords, 'BackKey'); }
export interface HudLook { ring: RingLook; plan: PlanLook; net: NetLook; attention: AttentionLook; lineClear: boolean; markClear: boolean; back: BackKey }
function readHudLookIn(I: In): HudLook { return { ring: readRingLookIn(I), plan: readPlanLookIn(I), net: readNetLookIn(I), attention: readAttentionLookIn(I), lineClear: I.bool(), markClear: I.bool(), back: readBackKeyIn(I) }; }
export function readHudLook(b: Uint8Array): HudLook { const I = new In(b), r = readHudLookIn(I); if (I.p !== b.length) throw new Error(`the view: HudLook is ${b.length} bytes and ${I.p} were read`); return r; }
export type AttentionMode = 'go' | 'look' | 'lent';
const AttentionModeWords: readonly AttentionMode[] = ['go', 'look', 'lent'];
function readAttentionModeIn(I: In): AttentionMode { return I.word(AttentionModeWords, 'AttentionMode'); }
export interface AttentionLine { man: number; at: Vec3; left: number; mode: AttentionMode; ending: boolean; worn: boolean; walking: boolean; hidden: boolean; walks: boolean; own: Vec3[] | null }
function readAttentionLineIn(I: In): AttentionLine { return { man: I.float64(), at: [I.float64(), I.float64(), I.float64()], left: I.float64(), mode: readAttentionModeIn(I), ending: I.bool(), worn: I.bool(), walking: I.bool(), hidden: I.bool(), walks: I.bool(), own: I.bool() ? I.list(() => [I.float64(), I.float64(), I.float64()]) : null }; }
export function readAttentionLine(b: Uint8Array): AttentionLine { const I = new In(b), r = readAttentionLineIn(I); if (I.p !== b.length) throw new Error(`the view: AttentionLine is ${b.length} bytes and ${I.p} were read`); return r; }
export interface TickView { mission: MissionView; lesson: LessonView | null; radio: RadioRow[]; bubbles: BubbleView[]; me: number; place: string; kit: KitView; marks: MarksView; aim: AimView; worn: WornView[]; squad: SquadView | null; corner: Note[]; throws: ThrowLine[]; ring: RingView | null; hand: HandRingView | null; empty: EmptyView | null; plan: PlanView; live: boolean; interact: InteractView; alarm: AlarmView | null; stage: Stage; look: HudLook; attention: AttentionLine[] }
function readTickViewIn(I: In): TickView { return { mission: readMissionViewIn(I), lesson: I.bool() ? readLessonViewIn(I) : null, radio: I.list(() => readRadioRowIn(I)), bubbles: I.list(() => readBubbleViewIn(I)), me: I.float64(), place: I.str(), kit: readKitViewIn(I), marks: readMarksViewIn(I), aim: readAimViewIn(I), worn: I.list(() => readWornViewIn(I)), squad: I.bool() ? readSquadViewIn(I) : null, corner: I.list(() => readNoteIn(I)), throws: I.list(() => readThrowLineIn(I)), ring: I.bool() ? readRingViewIn(I) : null, hand: I.bool() ? readHandRingViewIn(I) : null, empty: I.bool() ? readEmptyViewIn(I) : null, plan: readPlanViewIn(I), live: I.bool(), interact: readInteractViewIn(I), alarm: I.bool() ? readAlarmViewIn(I) : null, stage: readStageIn(I), look: readHudLookIn(I), attention: I.list(() => readAttentionLineIn(I)) }; }
export function readTickView(b: Uint8Array): TickView { const I = new In(b), r = readTickViewIn(I); if (I.p !== b.length) throw new Error(`the view: TickView is ${b.length} bytes and ${I.p} were read`); return r; }
export type UnderKind = 'floor' | 'door' | 'none' | 'foe';
const UnderKindWords: readonly UnderKind[] = ['floor', 'door', 'none', 'foe'];
function readUnderKindIn(I: In): UnderKind { return I.word(UnderKindWords, 'UnderKind'); }
export interface UnderWall { index: number; alongX: boolean; u: number; side: number; ok: boolean; why: string; outer: boolean }
function readUnderWallIn(I: In): UnderWall { return { index: I.float64(), alongX: I.bool(), u: I.float64(), side: I.float64(), ok: I.bool(), why: I.str(), outer: I.bool() }; }
export function readUnderWall(b: Uint8Array): UnderWall { const I = new In(b), r = readUnderWallIn(I); if (I.p !== b.length) throw new Error(`the view: UnderWall is ${b.length} bytes and ${I.p} were read`); return r; }
export type WorkThingKind = 'breaker' | 'objective';
const WorkThingKindWords: readonly WorkThingKind[] = ['breaker', 'objective'];
function readWorkThingKindIn(I: In): WorkThingKind { return I.word(WorkThingKindWords, 'WorkThingKind'); }
export interface WorkThing { kind: WorkThingKind; face: Vec3; n: Vec3; stand: Vec3 | null; name: string }
function readWorkThingIn(I: In): WorkThing { return { kind: readWorkThingKindIn(I), face: [I.float64(), I.float64(), I.float64()], n: [I.float64(), I.float64(), I.float64()], stand: I.bool() ? [I.float64(), I.float64(), I.float64()] : null, name: I.str() }; }
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
export interface RecapDeath { t: number; victim: number; name: string; side: string; player: boolean; at: [number, number]; gaze: number; cause: RecapCause; by: number; byName: string; bySide: string; byAt: [number, number] | null; from: [number, number] | null; to: [number, number]; weapon: string; dist: number; off: number; blind: boolean; loud: boolean; serial: number }
function readRecapDeathIn(I: In): RecapDeath { return { t: I.float64(), victim: I.float64(), name: I.str(), side: I.str(), player: I.bool(), at: [I.float64(), I.float64()], gaze: I.float64(), cause: readRecapCauseIn(I), by: I.float64(), byName: I.str(), bySide: I.str(), byAt: I.bool() ? [I.float64(), I.float64()] : null, from: I.bool() ? [I.float64(), I.float64()] : null, to: [I.float64(), I.float64()], weapon: I.str(), dist: I.float64(), off: I.float64(), blind: I.bool(), loud: I.bool(), serial: I.uint32() }; }
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
export interface DebriefView { debrief: Debrief; rows: DebriefRow[]; dropped: number; path: number[]; recap: DeathRecap | null }
function readDebriefViewIn(I: In): DebriefView { return { debrief: readDebriefIn(I), rows: I.list(() => readDebriefRowIn(I)), dropped: I.float64(), path: I.list(() => I.float32()), recap: I.bool() ? readDeathRecapIn(I) : null }; }
export function readDebriefView(b: Uint8Array): DebriefView { const I = new In(b), r = readDebriefViewIn(I); if (I.p !== b.length) throw new Error(`the view: DebriefView is ${b.length} bytes and ${I.p} were read`); return r; }
export type Approach = 'doors' | 'quiet' | 'walls' | 'dark';
const ApproachWords: readonly Approach[] = ['doors', 'quiet', 'walls', 'dark'];
function readApproachIn(I: In): Approach { return I.word(ApproachWords, 'Approach'); }
export interface RunView { begun: boolean; over: boolean; void: boolean; lesson: boolean; mission: string; level: string; won: boolean; way: Approach | null; clean: boolean; secs: number; course: string; par: number | null }
function readRunViewIn(I: In): RunView { return { begun: I.bool(), over: I.bool(), void: I.bool(), lesson: I.bool(), mission: I.str(), level: I.str(), won: I.bool(), way: I.bool() ? readApproachIn(I) : null, clean: I.bool(), secs: I.float64(), course: I.str(), par: I.bool() ? I.float64() : null }; }
export function readRunView(b: Uint8Array): RunView { const I = new In(b), r = readRunViewIn(I); if (I.p !== b.length) throw new Error(`the view: RunView is ${b.length} bytes and ${I.p} were read`); return r; }
export interface CardLoad { mob: number; con: number }
function readCardLoadIn(I: In): CardLoad { return { mob: I.float64(), con: I.float64() }; }
export function readCardLoad(b: Uint8Array): CardLoad { const I = new In(b), r = readCardLoadIn(I); if (I.p !== b.length) throw new Error(`the view: CardLoad is ${b.length} bytes and ${I.p} were read`); return r; }
export interface CardBar { now: number; issued: number }
function readCardBarIn(I: In): CardBar { return { now: I.float64(), issued: I.float64() }; }
export function readCardBar(b: Uint8Array): CardBar { const I = new In(b), r = readCardBarIn(I); if (I.p !== b.length) throw new Error(`the view: CardBar is ${b.length} bytes and ${I.p} were read`); return r; }
export interface CardPick { id: string; word: string; note: string; why: string; now: boolean; can: boolean; icon: string; bars: number[] }
function readCardPickIn(I: In): CardPick { return { id: I.str(), word: I.str(), note: I.str(), why: I.str(), now: I.bool(), can: I.bool(), icon: I.str(), bars: I.list(() => I.float64()) }; }
export function readCardPick(b: Uint8Array): CardPick { const I = new In(b), r = readCardPickIn(I); if (I.p !== b.length) throw new Error(`the view: CardPick is ${b.length} bytes and ${I.p} were read`); return r; }
export type CardKind = 'count' | 'flag' | 'choice';
const CardKindWords: readonly CardKind[] = ['count', 'flag', 'choice'];
function readCardKindIn(I: In): CardKind { return I.word(CardKindWords, 'CardKind'); }
export interface CardRow { field: string; label: string; kind: CardKind; value: string; n: number; word: string; note: string; cost: string; under: string; look: boolean; none: boolean; fixed: boolean; changed: boolean; idle: boolean; less: string; more: string; can: boolean; icon: string; noise: number; takes: CardLoad | null; would: CardLoad | null; bars: CardBar[]; picks: CardPick[] }
function readCardRowIn(I: In): CardRow { return { field: I.str(), label: I.str(), kind: readCardKindIn(I), value: I.str(), n: I.float64(), word: I.str(), note: I.str(), cost: I.str(), under: I.str(), look: I.bool(), none: I.bool(), fixed: I.bool(), changed: I.bool(), idle: I.bool(), less: I.str(), more: I.str(), can: I.bool(), icon: I.str(), noise: I.float64(), takes: I.bool() ? readCardLoadIn(I) : null, would: I.bool() ? readCardLoadIn(I) : null, bars: I.list(() => readCardBarIn(I)), picks: I.list(() => readCardPickIn(I)) }; }
export function readCardRow(b: Uint8Array): CardRow { const I = new In(b), r = readCardRowIn(I); if (I.p !== b.length) throw new Error(`the view: CardRow is ${b.length} bytes and ${I.p} were read`); return r; }
export interface CardPouches { used: number; total: number }
function readCardPouchesIn(I: In): CardPouches { return { used: I.float64(), total: I.float64() }; }
export function readCardPouches(b: Uint8Array): CardPouches { const I = new In(b), r = readCardPouchesIn(I); if (I.p !== b.length) throw new Error(`the view: CardPouches is ${b.length} bytes and ${I.p} were read`); return r; }
export interface CardView { who: number; changed: boolean; pouches: CardPouches; big: CardPouches; load: CardLoad; loadIssued: CardLoad; rows: CardRow[]; issued: boolean; resetWhy: string; copied: boolean; clip: boolean; pasteWhy: string; words: string; suit: Vec3 | null; trim: Vec3 | null }
function readCardViewIn(I: In): CardView { return { who: I.float64(), changed: I.bool(), pouches: readCardPouchesIn(I), big: readCardPouchesIn(I), load: readCardLoadIn(I), loadIssued: readCardLoadIn(I), rows: I.list(() => readCardRowIn(I)), issued: I.bool(), resetWhy: I.str(), copied: I.bool(), clip: I.bool(), pasteWhy: I.str(), words: I.str(), suit: I.bool() ? [I.float64(), I.float64(), I.float64()] : null, trim: I.bool() ? [I.float64(), I.float64(), I.float64()] : null }; }
export function readCardView(b: Uint8Array): CardView { const I = new In(b), r = readCardViewIn(I); if (I.p !== b.length) throw new Error(`the view: CardView is ${b.length} bytes and ${I.p} were read`); return r; }
export interface KeptText { key: string; text: string }
function readKeptTextIn(I: In): KeptText { return { key: I.str(), text: I.str() }; }
export function readKeptText(b: Uint8Array): KeptText { const I = new In(b), r = readKeptTextIn(I); if (I.p !== b.length) throw new Error(`the view: KeptText is ${b.length} bytes and ${I.p} were read`); return r; }
export interface Pressed { ok: boolean; why: string; passed: string; changed: string[]; kept: KeptText[] }
function readPressedIn(I: In): Pressed { return { ok: I.bool(), why: I.str(), passed: I.str(), changed: I.list(() => I.str()), kept: I.list(() => readKeptTextIn(I)) }; }
export function readPressed(b: Uint8Array): Pressed { const I = new In(b), r = readPressedIn(I); if (I.p !== b.length) throw new Error(`the view: Pressed is ${b.length} bytes and ${I.p} were read`); return r; }
export interface TeamSize { n: number; ok: boolean; own: boolean; words: string; note: string }
function readTeamSizeIn(I: In): TeamSize { return { n: I.float64(), ok: I.bool(), own: I.bool(), words: I.str(), note: I.str() }; }
export function readTeamSize(b: Uint8Array): TeamSize { const I = new In(b), r = readTeamSizeIn(I); if (I.p !== b.length) throw new Error(`the view: TeamSize is ${b.length} bytes and ${I.p} were read`); return r; }
export interface TeamKit { id: string; name: string; now: boolean; offered: boolean }
function readTeamKitIn(I: In): TeamKit { return { id: I.str(), name: I.str(), now: I.bool(), offered: I.bool() }; }
export function readTeamKit(b: Uint8Array): TeamKit { const I = new In(b), r = readTeamKitIn(I); if (I.p !== b.length) throw new Error(`the view: TeamKit is ${b.length} bytes and ${I.p} were read`); return r; }
export interface TeamThing { field: string; value: string; label: string; n: number; can: boolean; icon: string }
function readTeamThingIn(I: In): TeamThing { return { field: I.str(), value: I.str(), label: I.str(), n: I.float64(), can: I.bool(), icon: I.str() }; }
export function readTeamThing(b: Uint8Array): TeamThing { const I = new In(b), r = readTeamThingIn(I); if (I.p !== b.length) throw new Error(`the view: TeamThing is ${b.length} bytes and ${I.p} were read`); return r; }
export interface TeamMan { here: boolean; title: string; sub: string; pair: string; changed: boolean; words: string; pouches: CardPouches; gun: string; icon: string; can: boolean; clip: boolean; things: TeamThing[]; worn: string }
function readTeamManIn(I: In): TeamMan { return { here: I.bool(), title: I.str(), sub: I.str(), pair: I.str(), changed: I.bool(), words: I.str(), pouches: readCardPouchesIn(I), gun: I.str(), icon: I.str(), can: I.bool(), clip: I.bool(), things: I.list(() => readTeamThingIn(I)), worn: I.str() }; }
export function readTeamMan(b: Uint8Array): TeamMan { const I = new In(b), r = readTeamManIn(I); if (I.p !== b.length) throw new Error(`the view: TeamMan is ${b.length} bytes and ${I.p} were read`); return r; }
export interface TeamView { size: number; sizes: TeamSize[]; note: string; cards: boolean; asIssued: boolean; pool: string; walls: boolean; kits: TeamKit[]; men: TeamMan[]; frags: number; smoke: number; bangs: number; link: string }
function readTeamViewIn(I: In): TeamView { return { size: I.float64(), sizes: I.list(() => readTeamSizeIn(I)), note: I.str(), cards: I.bool(), asIssued: I.bool(), pool: I.str(), walls: I.bool(), kits: I.list(() => readTeamKitIn(I)), men: I.list(() => readTeamManIn(I)), frags: I.float64(), smoke: I.float64(), bangs: I.float64(), link: I.str() }; }
export function readTeamView(b: Uint8Array): TeamView { const I = new In(b), r = readTeamViewIn(I); if (I.p !== b.length) throw new Error(`the view: TeamView is ${b.length} bytes and ${I.p} were read`); return r; }
export interface SettingChoice { id: string; label: string; title: string; on: boolean; shown: boolean }
function readSettingChoiceIn(I: In): SettingChoice { return { id: I.str(), label: I.str(), title: I.str(), on: I.bool(), shown: I.bool() }; }
export function readSettingChoice(b: Uint8Array): SettingChoice { const I = new In(b), r = readSettingChoiceIn(I); if (I.p !== b.length) throw new Error(`the view: SettingChoice is ${b.length} bytes and ${I.p} were read`); return r; }
export interface Setting { key: string; label: string; live: boolean; shown: boolean; moved: boolean; note: string; choices: SettingChoice[] }
function readSettingIn(I: In): Setting { return { key: I.str(), label: I.str(), live: I.bool(), shown: I.bool(), moved: I.bool(), note: I.str(), choices: I.list(() => readSettingChoiceIn(I)) }; }
export function readSetting(b: Uint8Array): Setting { const I = new In(b), r = readSettingIn(I); if (I.p !== b.length) throw new Error(`the view: Setting is ${b.length} bytes and ${I.p} were read`); return r; }
export interface StartRow { id: string; title: string; kind: string; group: string; groupTitle: string; groupLine: string; place: string; blurb: string; lean: string; leanWords: string; level: string; levelWords: string; squad: number; teamWords: string; teamTag: string; unplayed: boolean; offList: boolean; asIssued: boolean; lesson: boolean; two: boolean; ways: string[]; pick: string; line: string; pickSquad: number; link: string }
function readStartRowIn(I: In): StartRow { return { id: I.str(), title: I.str(), kind: I.str(), group: I.str(), groupTitle: I.str(), groupLine: I.str(), place: I.str(), blurb: I.str(), lean: I.str(), leanWords: I.str(), level: I.str(), levelWords: I.str(), squad: I.float64(), teamWords: I.str(), teamTag: I.str(), unplayed: I.bool(), offList: I.bool(), asIssued: I.bool(), lesson: I.bool(), two: I.bool(), ways: I.list(() => I.str()), pick: I.str(), line: I.str(), pickSquad: I.float64(), link: I.str() }; }
export function readStartRow(b: Uint8Array): StartRow { const I = new In(b), r = readStartRowIn(I); if (I.p !== b.length) throw new Error(`the view: StartRow is ${b.length} bytes and ${I.p} were read`); return r; }
export interface StartView { rows: StartRow[]; named: string; pick: string; playing: boolean; first: string }
function readStartViewIn(I: In): StartView { return { rows: I.list(() => readStartRowIn(I)), named: I.str(), pick: I.str(), playing: I.bool(), first: I.str() }; }
export function readStartView(b: Uint8Array): StartView { const I = new In(b), r = readStartViewIn(I); if (I.p !== b.length) throw new Error(`the view: StartView is ${b.length} bytes and ${I.p} were read`); return r; }
export interface MissionRow { id: string; title: string; kind: string; squad: number; group: string; level: string; hour: string; place: string; blurb: string; scene: string; teaches: string[]; only: string; defenders: number; squads: boolean; ways: string[]; untuned: boolean; offList: boolean; line: string; link: string }
function readMissionRowIn(I: In): MissionRow { return { id: I.str(), title: I.str(), kind: I.str(), squad: I.float64(), group: I.str(), level: I.str(), hour: I.str(), place: I.str(), blurb: I.str(), scene: I.str(), teaches: I.list(() => I.str()), only: I.str(), defenders: I.float64(), squads: I.bool(), ways: I.list(() => I.str()), untuned: I.bool(), offList: I.bool(), line: I.str(), link: I.str() }; }
export function readMissionRow(b: Uint8Array): MissionRow { const I = new In(b), r = readMissionRowIn(I); if (I.p !== b.length) throw new Error(`the view: MissionRow is ${b.length} bytes and ${I.p} were read`); return r; }
export interface MissionsView { rows: MissionRow[] }
function readMissionsViewIn(I: In): MissionsView { return { rows: I.list(() => readMissionRowIn(I)) }; }
export function readMissionsView(b: Uint8Array): MissionsView { const I = new In(b), r = readMissionsViewIn(I); if (I.p !== b.length) throw new Error(`the view: MissionsView is ${b.length} bytes and ${I.p} were read`); return r; }
export interface SheetView { id: string; settings: Setting[]; link: string; base: string; asPick: boolean; quiet: boolean }
function readSheetViewIn(I: In): SheetView { return { id: I.str(), settings: I.list(() => readSettingIn(I)), link: I.str(), base: I.str(), asPick: I.bool(), quiet: I.bool() }; }
export function readSheetView(b: Uint8Array): SheetView { const I = new In(b), r = readSheetViewIn(I); if (I.p !== b.length) throw new Error(`the view: SheetView is ${b.length} bytes and ${I.p} were read`); return r; }
export interface BriefView { id: string; title: string; kind: string; groupTitle: string; place: string; blurb: string; leanWords: string; team: string; level: string; scene: string; teaches: string[]; way: number }
function readBriefViewIn(I: In): BriefView { return { id: I.str(), title: I.str(), kind: I.str(), groupTitle: I.str(), place: I.str(), blurb: I.str(), leanWords: I.str(), team: I.str(), level: I.str(), scene: I.str(), teaches: I.list(() => I.str()), way: I.float64() }; }
export function readBriefView(b: Uint8Array): BriefView { const I = new In(b), r = readBriefViewIn(I); if (I.p !== b.length) throw new Error(`the view: BriefView is ${b.length} bytes and ${I.p} were read`); return r; }
export interface ShortView { now: string; nowScenario: boolean; next: string; nextMission: string; title: string; line: string; link: string; otherLink: string }
function readShortViewIn(I: In): ShortView { return { now: I.str(), nowScenario: I.bool(), next: I.str(), nextMission: I.str(), title: I.str(), line: I.str(), link: I.str(), otherLink: I.str() }; }
export function readShortView(b: Uint8Array): ShortView { const I = new In(b), r = readShortViewIn(I); if (I.p !== b.length) throw new Error(`the view: ShortView is ${b.length} bytes and ${I.p} were read`); return r; }
