// Simple rigid bodies for thrown / dropped items (smoke canisters, empty magazines): ballistic flight with
// swept collision against the static boxes, bounce + friction, settle. Rendered as small dynamic boxes.
//
// WHERE A THING'S NUMBERS ARE. Every number an item has is a column of the pool's store (src/sim/itemshome.ts ItemStore: the game's own array, or — the
// row RUST.items on — a part of the game's block of the Rust module's memory), and `Item` is a handle on one row of it: `it.pos`, `it.fuse`, `it.settled`
// read and write that row, whoever asks. THE STEP RUNS IN THE RUST MODULE where it can (rust/sim/src/items.rs, over the same columns) and here otherwise
// (`stepRow`); what either leaves is the same to the bit (tools/qa/item-ident.ts, and whole missions in tools/qa/rust-check.ts).
//
// WHAT A STEP DOES TO ANYTHING ELSE IS SAID AFTER IT, by both: a bounce loud enough to be heard (the item's `onBounce`), a share of its momentum for
// the chair it struck (PropSystem.applyImpulse), its time up (its row given up) — listed as the step meets them and done in that order WHEN THE
// WHOLE POOL HAS BEEN STEPPED, whichever side stepped it. Nothing a step reads is changed by them: furniture is met by where it stands, which an
// impulse does not move. So what a callback does can never reach into the step that called it, in either language — and a callback is handed what the
// item was AS IT BOUNCED (how fast, where, off which face); the item itself is by then as the step left it.
//
// THE LIST IS THE GAME'S, AND THE ROWS FOLLOW IT. `Throwables.items` is a plain array, the oldest first, and code all over the game takes an item out
// of it, empties it or puts it back. The pool keeps the list its rows were last laid in; where the list has been changed since, the rows are laid
// anew in the list's order before anything is stepped or spawned (`sync`), and an item no longer in it keeps its numbers as they were.
import { Vec3, Quat, v3, quat } from '../math/vec';
import { StaticCollision } from '../scene/collision';
import { Box, BoxFlag, makeBox, boxOf, MKEY } from '../scene/boxes';
import { SNOWBOG } from './snowBog';
import type { PropSystem, Prop } from './props';
import type { Team } from './teams';
import { GEAR } from './gear';   // (a leaf: which rifle a dropped FN2000-row item is DRAWN as)
import type { Game } from './game';
import { simRandom } from '../math/dice';   // the simulation's die (math/dice.ts): the Game's own when it has one, Math.random otherwise
import { hyp3 } from '../math/vec';
import { EXACT } from '../math/exact';
import { RUST } from '../sim/sim';
import { BOTH } from '../scene/collision';
import type { BoxMirror } from '../scene/boxMirror';
import { ItemStore, ITF, IC, ITEM_KINDS, ITEM_CAP, IP, IE, IEK, ITEM_EV, ITEMS_HOST, ITEMS_STEPPED, itemIo, itemMem, itemDiff } from '../sim/itemshome';

export type ItemKind = (typeof ITEM_KINDS)[number];   // 'smoke' | 'mag' | 'flash' | 'frag' | 'pistol' | 'torch' | 'rifle' | 'ring' | 'shell' (sim/itemshome.ts: a kind is its place in that list)   // pistol / torch / rifle: dropped from a dying hand (combat.ts dropFromHand — the rifle is what the long-gun rows let go of, weapons.ts WeaponDef.drop: a rifleman's whole AKS-74U, or with fill 1 Sam's FN2000) — physical like the rest (tumble, bounce, settle, nudge props), and set dressing once down: nobody picks them up (a guard's magazines would not fit the Five-seveN anyway, and nothing on the floor is ammunition); ring: a spent ring airfoil (launcher.ts hands it over the moment its flight ends — off a wall, off a man — to clatter down and lie there: not retrievable either)

/** The FN2000 on the floor (a 'rifle' item with fill 1 — combat.ts hitPlayer drops it out of Sam's hands): the bullpup's own silhouette in RIG.sc20k's two albedos —
 *  the long polymer hull with the box straight down BEHIND the grip point, the fore-end and the can ahead, the sight block on top — same contact sphere and dead
 *  bounces as the carbine's item, lying on its side once down like every dropped gun. */
export const SC20K_ITEM = {
  hull: { z: -0.06, h: [0.024, 0.04, 0.215] as [number, number, number] },      // centre along the item's length (+ toward the muzzle, 0 = the grip) and half extents: hull + buttpad −0.275‥0.155
  fore: { z: 0.3, h: [0.02, 0.024, 0.155] as [number, number, number] },        // fore-end + suppressor as one length out to the muzzle at 0.455
  mag: { z: -0.115, y: -0.1, h: [0.012, 0.06, 0.016] as [number, number, number] },   // the box straight down behind the grip
  optic: { z: 0.02, y: 0.06, h: [0.017, 0.02, 0.06] as [number, number, number] },
  thick: 0.024,
  polymer: [0.045, 0.05, 0.046] as Vec3, metal: [0.026, 0.026, 0.03] as Vec3,   // rigProps COL.polymer / COL.darkMetal, so the thing on the floor is the thing that was in his hands
  get foreCol(): Vec3 { return this.metal; },   // (the fore-end + can length is drawn in the metal)
};
/** …and the same item as the 2010s issue kit draws it (gear.ts GEAR.on): the M4-pattern carbine of rigProps RIG.m4 — receiver and stock as one dark length, the
 *  dark earth handguard and the can ahead, the box AHEAD of the grip. Picture only: the item's contact sphere, bounces and rest height are the bullpup's. */
export const M4_ITEM = {
  hull: { z: -0.09, h: [0.017, 0.034, 0.2] as [number, number, number] },
  fore: { z: 0.285, h: [0.023, 0.021, 0.17] as [number, number, number] },   // (as thick as the item's rest height, the bullpup's SC20K_ITEM.thick: it lies ON the floor)
  mag: { z: 0.085, y: -0.095, h: [0.011, 0.065, 0.028] as [number, number, number] },
  optic: { z: 0.035, y: 0.052, h: [0.016, 0.02, 0.035] as [number, number, number] },
  polymer: [0.03, 0.03, 0.033] as Vec3, metal: [0.03, 0.03, 0.033] as Vec3, foreCol: [0.2, 0.155, 0.1] as Vec3,   // rigProps COL.anodised (receiver, box, sight) / COL.fde (the handguard)
};
/** a smoke or stun canister's contact sphere and bounce (spawn's numbers for both; frag.ts restOfThrow replays them for the throw preview's rest mark) */
export const CAN_ITEM = { radius: 0.055, restitution: 0.38, friction: 0.35 };
/** A FRAG GRENADE in the air and on the floor (frag.ts: its fuse runs from the hand, and what it does when it runs out): a small heavy ball — a smaller contact
 *  sphere than a can's, livelier off a hard floor and with less grip, so it skips and ROLLS on where a can would stop (the men it lands among see where it is
 *  going) — drawn as an olive body with a pale yellow band and the fuse's stub on top. */
export const FRAG_ITEM = { radius: 0.04, restitution: 0.3, friction: 0.45, half: [0.032, 0.032, 0.038] as [number, number, number], body: [0.13, 0.17, 0.08] as Vec3, band: [0.75, 0.62, 0.1] as Vec3, fuze: [0.3, 0.3, 0.28] as Vec3 };
/** the spent ring on the floor: a rubber ring 6 cm across drawn as three thin crossed slats (a hexagon's worth of ring at this size), pale so it reads on the lino */
export const RING_ITEM = { radius: 0.03, restitution: 0.3, friction: 0.6, life: 60, half: [0.03, 0.006, 0.007] as [number, number, number], albedo: [0.42, 0.4, 0.36] as Vec3 };

/** …and the same spent thing as the 2010s issue kit draws it (gear.ts GEAR.on: the ring airfoil is a 40 mm SPONGE ROUND there): the projectile of a 40 × 46 mm
 *  impact round — a blunt blue foam nose on a dark plastic body with a paler driving band at its base, 44 mm across and 76 mm long (a shade over life size, so
 *  it reads from the camera's height at least as the ring did). THREE boxes, as the ring's three slats, under the ring's own flags; z = along the round, + toward
 *  the nose, 0 = its middle. Picture only: the item's contact sphere, bounces, rest and life are RING_ITEM's, and the flight is launcher.ts RING's. */
export const SPONGE_ITEM = {
  wide: 0.023,   // (half the widest part — the band: what rests on the floor when it lies on its side)
  nose: { z: 0.019, h: [0.022, 0.022, 0.019] as [number, number, number], albedo: [0.07, 0.24, 0.62] as Vec3 },    // blue foam
  body: { z: -0.014, h: [0.021, 0.021, 0.014] as [number, number, number], albedo: [0.05, 0.05, 0.055] as Vec3 },   // black plastic
  band: { z: -0.033, h: [0.023, 0.023, 0.005] as [number, number, number], albedo: [0.3, 0.3, 0.31] as Vec3 },     // the grey driving band
};
/** the sponge round's three boxes about `c`, its length along `rot`'s z (the nose toward +z): in the air off launcher.ts ringBoxes, spent off the item loop below */
export function spongeBoxes(out: Box[], c: Vec3, rot: Quat, yaw: number) {
  const S = SPONGE_ITEM;
  for (const part of [S.body, S.nose, S.band]) out.push(itemBox(v3.add(c, quat.rotate(rot, [0, 0, part.z])), v3.copy(part.h), yaw, rot, v3.copy(part.albedo), BoxFlag.Dynamic | BoxFlag.NoShadow));
}

/** a 40 mm round that did not go off (he40.ts: it met something short of its arming distance) — a stubby gold-nosed slug of aluminium that lands hard, rolls a
 *  little and lies there, inert: nobody picks it up and nothing sets it off */
export const SHELL_ITEM = { radius: 0.035, restitution: 0.22, friction: 0.5, life: 120, half: [0.02, 0.02, 0.05] as [number, number, number], body: [0.42, 0.43, 0.4] as Vec3, nose: [0.72, 0.55, 0.12] as Vec3 };

/** The dropped rifle's body (rifles, docs/internal/sc20k-design.md §3.5): a longer, heavier item than the sidearm — a bigger contact sphere so it does not fall
 *  through the gaps the pistol threads, deader bounces (a slung 2.7 kg carbine lands and skids, it does not hop), more friction, more of its momentum handed to a
 *  chair it strikes; drawn as four parts along its length (receiver, the wood ahead of it out to the booster, the curved box under, the skeleton stock behind —
 *  0.72 m in all, the hand's grip point about a third of the way along) tumbling end over end in the air and lying on its SIDE once down, like the pistol. */
export const RIFLE_ITEM = {
  radius: 0.06, restitution: 0.18, friction: 0.55, life: 240, propShare: 0.3,
  thick: 0.02,                                                    // half-thickness of the receiver: what rests on the floor when it lies on its side
  receiver: { z: -0.02, h: [0.018, 0.03, 0.13] as [number, number, number] },   // centre along the item's length (m, + toward the muzzle) and half extents
  fore: { z: 0.24, h: [0.02, 0.022, 0.13] as [number, number, number] },        // handguard + barrel + booster as one length of wood-and-steel
  mag: { z: 0.06, y: -0.075, h: [0.013, 0.05, 0.03] as [number, number, number], cant: 0.35 },   // the bakelite box out of the receiver floor, canted forward
  stock: { z: -0.27, h: [0.008, 0.026, 0.12] as [number, number, number] },      // the triangle stock, as one thin slab
  steel: [0.05, 0.05, 0.055] as Vec3, wood: [0.2, 0.085, 0.05] as Vec3,          // rigProps COL.gun / COL.furniture, so the thing on the floor is the thing that was in his hands
};

/** WHAT AN ITEM IS, field by field: what `Throwables.spawn` may be handed (its `opts`), and what `Item` gives and takes. */
export interface ItemFields {
  kind: ItemKind; pos: Vec3; vel: Vec3; radius: number; restitution: number; friction: number;
  yaw: number; spin: number; settled: boolean; age: number; fuse: number; fired: boolean; life: number;
  tumble: number; tumbleRate: number;   // end-over-end angle (rad) about the axis across its travel, and its rate: cans and mags flip in the air, damp on each bounce, ease flat when settled
  emitT: number;                 // seconds of emission left (smoke)
  fill: number;                  // magazines: rounds fraction (for size/colour), unused for smoke
  bounces: number;
  heading: number;               // yaw of the horizontal travel (set at the throw, refreshed while it skids): a smoke can vents the way it was thrown
  who: number;                   // character id that threw / dropped it (blamed for furniture it knocks)
  on: Prop | null; onPose: number;   // the prop it came to rest on, if any, and a key of that prop's pose at the time (it drops off once the prop has moved out from under it)
  serial: number;                // a running number per spawned item (motion identity: Throwables.boxes keys the item's parts off it)
  /** a stun can: whose side's it is, said by a caller that takes the thrower off the guard list for the bang (doortools.ts bang, DOORTOOLS.ownCan) — combat.ts
   *  detonateStun reads it ahead of looking the thrower up; unset = looked up, as ever */
  side?: Team;
  /** called on a bounce with the Game the item is in and the item itself: what it does it does to THOSE (a callback that kept a Game or a count of its own in its
   *  closure could not be rolled back, nor go into another game with the item — rollback.ts) */
  onBounce?: (speed: number, p: Vec3, g: Game, it: Item, n: Vec3) => void;   // (n: the face it struck, out of the surface — up for a floor or a table's top; p: where it was as it struck. Called when the pool's step is over — the head of this file: `it` is the item as the step left it, the arguments what it was at the bounce)
  /** onBounce's own count, where it keeps one (combat.ts dropFromHand: the first two clatters are heard) */
  clatters?: number;
  /** in a hand (disarm.ts: the gun Sam has taken out of a man's, until he lets it go): it is placed by whoever carries it, every frame — no gravity, no bounce,
   *  drawn where it is put (yaw, and tumble = how far its muzzle is down); it ages like any item. Unset = an item like any other */
  carried?: boolean;
}


/** AN ITEM: a handle on one row of its pool's columns (the head of this file). Everything an item has is read and written through it, so the Rust module's
 *  step and every rule here see the same numbers. `pos` and `vel` are GIVEN AS NEW ARRAYS and TAKEN WHOLE: `it.pos = p` writes the row; an element of
 *  the array `it.pos` gave is nobody's to write. An item that has left its pool (its time up, given up for a newer one, taken out of the list) keeps its
 *  numbers as they were when it left (`kept`), and reads and writes those. */
export class Item implements ItemFields {
  /** its pool's columns and its row there (`Throwables.items[r]` is this item) — or, out of the pool, its numbers by their places (sim/itemshome.ts IC) */
  private S: ItemStore; private r: number; private kept: number[] | null = null;
  side?: Team;
  onBounce?: (speed: number, p: Vec3, g: Game, it: Item, n: Vec3) => void;
  clatters?: number;
  /** `carried` as it was last set (unset is not false to a save file); its bit of `flags` is what the step reads */
  private hand?: boolean;
  /** `things`: the level's furniture by id (PropSystem.all), for `on` */
  constructor(S: ItemStore, r: number, private readonly things: readonly Prop[]) { this.S = S; this.r = r; }
  /** THE POOL'S OWN (Throwables): which row it stands in; out of the pool with its numbers; its numbers as they stand; into row `r` */
  get row(): number { return this.kept === null ? this.r : -1; }
  leave(): void { if (this.kept === null) { this.kept = this.S.rowOut(this.r); this.r = -1; } }
  numbers(): number[] { return this.kept ?? this.S.rowOut(this.r); }
  stand(r: number): void { this.kept = null; this.r = r; }
  private f(c: number): number { const K = this.kept; if (K !== null) return K[c]; const S = this.S; return S.d[S.b8 + S.at[c] + this.r]; }
  private pf(c: number, v: number): void { const K = this.kept; if (K !== null) { K[c] = v; return; } const S = this.S; S.d[S.b8 + S.at[c] + this.r] = v; }
  private w(c: number): number { const K = this.kept; if (K !== null) return K[c]; const S = this.S; return S.u[S.b4 + S.at[c] + this.r]; }
  private pw(c: number, v: number): void { const K = this.kept; if (K !== null) { K[c] = v >>> 0; return; } const S = this.S; S.u[S.b4 + S.at[c] + this.r] = v; }
  private bit(b: number): boolean { return (this.w(IC.flags) & b) !== 0; }
  private pbit(b: number, on: boolean): void { const f = this.w(IC.flags); this.pw(IC.flags, on ? f | b : f & ~b); }
  get kind(): ItemKind { return ITEM_KINDS[this.w(IC.kind)]; } set kind(v: ItemKind) { this.pw(IC.kind, ITEM_KINDS.indexOf(v)); }
  get pos(): Vec3 { return [this.f(IC.px), this.f(IC.py), this.f(IC.pz)]; }
  set pos(v: Vec3) { this.pf(IC.px, v[0]); this.pf(IC.py, v[1]); this.pf(IC.pz, v[2]); }
  get vel(): Vec3 { return [this.f(IC.vx), this.f(IC.vy), this.f(IC.vz)]; }
  set vel(v: Vec3) { this.pf(IC.vx, v[0]); this.pf(IC.vy, v[1]); this.pf(IC.vz, v[2]); }
  get radius(): number { return this.f(IC.radius); } set radius(v: number) { this.pf(IC.radius, v); }
  get restitution(): number { return this.f(IC.restitution); } set restitution(v: number) { this.pf(IC.restitution, v); }
  get friction(): number { return this.f(IC.friction); } set friction(v: number) { this.pf(IC.friction, v); }
  get yaw(): number { return this.f(IC.yaw); } set yaw(v: number) { this.pf(IC.yaw, v); }
  get spin(): number { return this.f(IC.spin); } set spin(v: number) { this.pf(IC.spin, v); }
  get tumble(): number { return this.f(IC.tumble); } set tumble(v: number) { this.pf(IC.tumble, v); }
  get tumbleRate(): number { return this.f(IC.tumbleRate); } set tumbleRate(v: number) { this.pf(IC.tumbleRate, v); }
  get age(): number { return this.f(IC.age); } set age(v: number) { this.pf(IC.age, v); }
  get life(): number { return this.f(IC.life); } set life(v: number) { this.pf(IC.life, v); }
  get fuse(): number { return this.f(IC.fuse); } set fuse(v: number) { this.pf(IC.fuse, v); }
  get emitT(): number { return this.f(IC.emitT); } set emitT(v: number) { this.pf(IC.emitT, v); }
  get fill(): number { return this.f(IC.fill); } set fill(v: number) { this.pf(IC.fill, v); }
  get heading(): number { return this.f(IC.heading); } set heading(v: number) { this.pf(IC.heading, v); }
  get onPose(): number { return this.f(IC.onPose); } set onPose(v: number) { this.pf(IC.onPose, v); }
  get settled(): boolean { return this.bit(ITF.settled); } set settled(v: boolean) { this.pbit(ITF.settled, !!v); }
  get fired(): boolean { return this.bit(ITF.fired); } set fired(v: boolean) { this.pbit(ITF.fired, !!v); }
  get carried(): boolean | undefined { return this.hand; } set carried(v: boolean | undefined) { this.hand = v; this.pbit(ITF.carried, !!v); }
  get bounces(): number { return this.w(IC.bounces); } set bounces(v: number) { this.pw(IC.bounces, v); }
  get who(): number { return this.w(IC.who) | 0; } set who(v: number) { this.pw(IC.who, v); }
  get serial(): number { return this.w(IC.serial); } set serial(v: number) { this.pw(IC.serial, v); }
  get on(): Prop | null { const k = this.w(IC.on); return k ? this.things[k - 1] ?? null : null; } set on(p: Prop | null) { this.pw(IC.on, p ? p.id + 1 : 0); }
}

export interface ThrowSolution { v0: Vec3; T: number; points: Vec3[]; land: Vec3; }

export const GRAVITY = 9.81;

/** makeBox's own object for an item's part, key for key (yaw, albedo, emissive, flags, owner, cutHeight, then c, h, rot), without its spread; the arguments are
 *  evaluated in the order the literal's fields were (c, h, yaw, rot, albedo, flags) */
function itemBox(c: Vec3, h: Vec3, yaw: number, rot: Quat, albedo: Vec3, flags: number): Box { return boxOf(c, h, yaw, albedo, [0, 0, 0], flags, 0, rot); }

/** A THING MEETS LYING SNOW (snowBog.ts, SNOWBOG.on; `bog` 0‥1 = how boggy the ground it touched is, 0 = a floor): the share of its slide and of its bounce
 *  it keeps, and how much faster than on a floor it may still be going and stop dead. One place for the pool's step and for frag.ts's forecast of it. */
export const bogSlide = (bog: number) => 1 - bog * SNOWBOG.grip;
export const bogBounce = (bog: number) => 1 - bog * SNOWBOG.dead;
export const bogStick = (bog: number) => bog * SNOWBOG.stick;

/** one event onto `ev` (sim/itemshome.ts IE: nine numbers an event) */
function listed(ev: number[], row: number, kind: number, a: number, p0: number, p1: number, p2: number, v0: number, v1: number, v2: number): void { ev.push(kind, row, a, p0, p1, p2, v0, v1, v2); }

/** LYING SNOW UNDER A POOL (the Game's: snowBog.ts): how boggy the ground is where a thing touched down, 0‥1 (`at`), and whether it may be boggy
 *  ANYWHERE now (`may`: false = `at` answers 0 for every point, which is what lets a bounce be worked out where the snow's map is not — the Rust module) */
export interface BogGround { may(): boolean; at(x: number, y: number, z: number): number }

export class Throwables {
  /** THE POOL, the oldest first: the game's list, anybody's to read and to take an item out of (the head of this file) */
  items: Item[] = [];
  /** …and the list as the rows were last laid in it (the pool's own: `sync`) */
  private rows: Item[] = [];
  /** the pool's columns (in the Rust module's memory while RUST.items is on and the game's block holds them) */
  readonly store = new ItemStore();
  /** lying snow (null = floors everywhere, the bounce as it always was) */
  bog: BogGround | null = null;
  /** the next item's serial (Item.serial) */
  private serial = 1;
  /** `props`: loose furniture — items bounce off it and hand over a little momentum */
  constructor(private col: StaticCollision, private props: PropSystem) {}

  /** Closest hit among statics + door leaves and props. */
  private ray(ro: Vec3, rd: Vec3, tmax: number): { t: number; n: Vec3; prop: Prop | null } | null {
    const st = this.col.raycast(ro, rd, tmax); const ph = this.props.raycast(ro, rd, st ? st.t : tmax);
    return ph ? { t: ph.t, n: ph.n, prop: ph.prop } : st ? { t: st.t, n: st.n, prop: null } : null;
  }

  /** THE ROWS ARE THE LIST: where `items` is not the list the rows were last laid in (an item spliced out, the list emptied, put back or put in another
   *  order by a rule outside the pool), every item no longer in it leaves with its numbers and the rows are laid anew in the list's order. Run before
   *  anything that reads rows by their place: a step, a spawn. */
  private sync(): void {
    const L = this.items, R = this.rows; let same = L.length === R.length;
    for (let k = 0; same && k < L.length; k++) same = L[k] === R[k];
    if (same) return;
    for (const it of L) if (!(it instanceof Item)) throw new Error('the pool of thrown things lists something that is no Item (Throwables.spawn makes them)');
    const rows = L.map(it => it.numbers()), still = new Set(L);
    for (const it of R) if (!still.has(it)) it.leave();
    this.store.lay(rows);
    for (let k = 0; k < L.length; k++) L[k].stand(k);
    this.rows = L.slice();
  }

  spawn(kind: ItemKind, pos: Vec3, vel: Vec3, opts: Partial<ItemFields> = {}): Item {
    const can = kind === 'smoke' || kind === 'flash';   // smoke and stun canisters share the body: same arc, same bounces
    const R = RIFLE_ITEM, rifle = kind === 'rifle', ring = kind === 'ring', G = RING_ITEM, frag = kind === 'frag', F = FRAG_ITEM, shell = kind === 'shell', S = SHELL_ITEM;
    const radius = can ? CAN_ITEM.radius : frag ? F.radius : kind === 'pistol' ? 0.05 : kind === 'torch' ? 0.045 : rifle ? R.radius : ring ? G.radius : shell ? S.radius : 0.03; const restitution = can ? CAN_ITEM.restitution : frag ? F.restitution : kind === 'mag' ? 0.25 : rifle ? R.restitution : ring ? G.restitution : shell ? S.restitution : 0.22;
    // (what the item is made with, the dice in the order they were always drawn and before anything is given up for it; `opts` over the rest)
    const made: Partial<ItemFields> = { kind, pos: v3.copy(pos), vel: v3.copy(vel), radius, restitution, friction: can ? CAN_ITEM.friction : kind === 'pistol' || kind === 'torch' ? 0.5 : rifle ? R.friction : ring ? G.friction : frag ? F.friction : shell ? S.friction : 0.35,
      yaw: simRandom() * Math.PI, spin: (simRandom() - 0.5) * 14, tumble: simRandom() * 6.28, tumbleRate: (simRandom() < 0.5 ? -1 : 1) * (7 + simRandom() * 9), settled: false, age: 0, fuse: 1.3, fired: false, life: kind === 'mag' ? 40 : ring ? G.life : shell ? S.life : 60, emitT: 0, fill: 0, bounces: 0, heading: v3.yawOf(vel), who: 0, on: null, onPose: 0, serial: this.serial++, ...opts };
    this.sync();
    // FIFO cap (applied before the push, so the new item can never evict itself), but never evict a canister whose fuse is still running or that is still venting:
    // dropped mags and guns are items too, and a firefight must not swallow a stun charge already deducted from the player's kit — it would simply never go off —
    // nor take the can out from under a smoke screen still pouring from it (its emitter tracks the item: the smoke would go on from nowhere). Spent things first
    // (magazines, rings, a can that has gone off / finished venting), then the oldest dropped gun or torch; thirty-two LIVE cans (never, in play) evict nothing.
    // (Review B7.5: 'fired' alone read a venting smoke can as spent, and with nothing spent in the list the oldest item went whatever it was — the live can included.)
    if (this.items.length >= ITEM_CAP) {
      let i = this.items.findIndex(x => x.kind === 'mag' || x.kind === 'ring' || x.kind === 'shell' || ((x.kind === 'smoke' || x.kind === 'flash' || x.kind === 'frag') && !liveCan(x)));
      if (i < 0) i = this.items.findIndex(x => !liveCan(x));
      if (i >= 0) this.removeAt(i);
    }
    const it = new Item(this.store, this.store.enter(), this.props.all);
    for (const k in made) (it as unknown as Record<string, unknown>)[k] = (made as Record<string, unknown>)[k];   // (every field through its own setter: the row is written)
    this.items.push(it); this.rows.push(it);
    return it;
  }
  /** AN ITEM LEAVES THE POOL BY THE POOL'S OWN RULE (given up for a newer one, its time up): it takes its numbers with it, its row is given up and the
   *  rows behind it close up. (The list and the rows are one here: `sync` has run.) */
  private removeAt(i: number): void {
    const L = this.items, it = L[i];
    it.leave(); this.store.close(i); L.splice(i, 1); this.rows.splice(i, 1);
    for (let k = i; k < L.length; k++) L[k].stand(k);
  }

  /** Launch velocity from `from` to land at `to` with a pleasant arc; also returns sampled preview points until first impact. */
  solve(from: Vec3, to: Vec3, maxSpeed = 11): ThrowSolution {
    const d = v3.sub(to, from); const dist = Math.hypot(d[0], d[2]);
    let T = Math.min(1.25, Math.max(0.42, dist / 7.5));
    let v0: Vec3 = [d[0] / T, (d[1] + 0.5 * GRAVITY * T * T) / T, d[2] / T];
    const sp = v3.len(v0); if (sp > maxSpeed) { v0 = v3.scale(v0, maxSpeed / sp); }
    // preview: integrate until first hit
    const points: Vec3[] = []; let p = v3.copy(from); let v = v3.copy(v0); let land = v3.copy(to);
    const h = 1 / 30;
    for (let i = 0; i < 90; i++) {
      points.push(v3.copy(p));
      const np: Vec3 = [p[0] + v[0] * h, p[1] + v[1] * h - 0.5 * GRAVITY * h * h, p[2] + v[2] * h];
      const seg = v3.sub(np, p); const len = v3.len(seg);
      if (len > 1e-5) { const hit = this.ray(p, v3.scale(seg, 1 / len), len + 0.05); if (hit) { land = v3.mad(p, seg, hit.t / len); points.push(land); break; } }
      if (np[1] < 0.02) { land = [np[0], 0.02, np[2]]; points.push(land); break; }
      v = [v[0], v[1] - GRAVITY * h, v[2]]; p = np; T = i * h;
    }
    return { v0, T, points, land };
  }

  /** `g`: the Game the items are in, handed to a bounce's callback (the model viewer's stage has none, and no item of its own has a callback) */
  update(dt: number, g: Game | null = null) {   // (null: the viewer's stage, which has no game)
    this.sync();
    let r = this.items.length - 1; if (r < 0) return;
    const M = this.world(g), ev: number[] = [];   // (the Rust module's world, where the walk is the module's to take; what the steps list for after)
    if (M !== null) ITEMS_STEPPED.stepsThere++; else ITEMS_STEPPED.stepsHere++;
    while (r >= 0) {
      if (M !== null) {
        // the module walks from row r down as far as it can: to the bottom (−1), or to a row only this side can judge, which is stepped here
        r = BOTH.on ? this.walkBoth(dt, M, r, ev) : this.walkThere(dt, M, r, ev);
        if (r < 0) break;
        ITEMS_STEPPED.host++;
      }
      this.stepRow(r, dt, ev); ITEMS_STEPPED.here++;
      r--;
    }
    this.say(g, ev);
  }
  /** the box world's copy in the Rust module, if the pool's walk is the module's to take now: the row on, the game's own maths standing, the pool's columns
   *  at home in the game's block, and the copy standing with the game's own leaves and the furniture in it */
  private world(g: Game | null): BoxMirror | null {
    if (!RUST.items || g === null || this.store.home === null || !EXACT.on) return null;
    const M = this.col.rulesWorld(this.col.dynamicBoxes);
    return M !== null && M.things(this.props) ? M : null;
  }
  /** THE MODULE'S WALK from row `from` down (rust/sim/src/items.rs update): the hand-over written, the walk asked, the tallies kept, what it listed
   *  copied out of the module's memory onto `ev`. → the row it hands back for this side to step, or −1 */
  private walkThere(dt: number, M: BoxMirror, from: number, ev: number[]): number {
    const io = itemIo(), m = itemMem(), P = io.par, B = this.bog;
    m[P + IP.dt] = dt; m[P + IP.from] = from; m[P + IP.ask] = this.col.glassOpen ? 1 : 0; m[P + IP.bog] = B !== null && B.may() ? 1 : 0; m[P + IP.rifleShare] = RIFLE_ITEM.propShare;
    const a = io.x.items_update(this.store.home!.bind(), M.slot), m2 = itemMem(), T = ITEMS_STEPPED;
    if (a !== ITEMS_HOST && a !== 0) throw new Error(`the Rust module would not walk the thrown things from row ${from} of ${this.store.n}`);
    const n = m2[P + IP.events];
    T.walks++; T.there += m2[P + IP.walked]; T.rays += m2[P + IP.rays]; T.thingRays += m2[P + IP.thingRays]; T.substeps += m2[P + IP.substeps]; T.rested += m2[P + IP.rested]; T.events += n;
    for (let i = io.ev, e = io.ev + n * ITEM_EV; i < e; i++) ev.push(m2[i]);
    return m2[P + IP.row];
  }
  /** THE CHECK'S WALK (collision.ts BOTH): the module's walk with nothing of it kept, then the same rows stepped here from the same columns, and the two
   *  compared — every column of every row and what each listed. What this side left stands. → the row handed back, or −1 */
  private walkBoth(dt: number, M: BoxMirror, from: number, ev: number[]): number {
    const S = this.store, T = ITEMS_STEPPED, before = S.bytes(), evA: number[] = [], evB: number[] = [];
    const row = this.walkThere(dt, M, from, evA), after = S.bytes();
    S.put(before);
    for (let r = from; r > row; r--) this.stepRow(r, dt, evB);
    const mine = S.bytes(); let same = evA.length === evB.length;
    for (let i = 0; same && i < evA.length; i++) same = Object.is(evA[i], evB[i]);
    const diff = itemDiff(S, after, mine, 2);
    T.asked++; BOTH.asked++;
    if (!same || diff.length) { T.differ++; BOTH.differ++; const first = `the thrown things from row ${from} (dt ${dt}): ${!same ? `the module listed ${evA.length / ITEM_EV} events and this side ${evB.length / ITEM_EV}, or not the same ones; ` : ''}${diff.join('; ')}`; if (!T.first) T.first = first; if (!BOTH.first) BOTH.first = first; }
    for (let i = 0; i < evB.length; i++) ev.push(evB[i]);
    return row;
  }
  /** WHAT THE STEPS LISTED, DONE, in the order it was listed (sim/itemshome.ts IE, IEK): a bounce said by the item's own callback, a share of momentum
   *  handed to a thing of furniture, an item whose time is up taken out of the pool */
  private say(g: Game | null, e: readonly number[]): void {
    if (e.length === 0) return;
    const walked = this.items.slice();   // (an event names its item by its row in the list as the steps walked it; what a callback does to the list since is its own)
    for (let o = 0; o < e.length; o += ITEM_EV) {
      const it = walked[e[o + IE.row]], what = e[o + IE.kind];
      if (what === IEK.gone) { this.sync(); const i = this.items.indexOf(it); if (i >= 0) this.removeAt(i); continue; }
      const p: Vec3 = [e[o + IE.p], e[o + IE.p + 1], e[o + IE.p + 2]], v: Vec3 = [e[o + IE.v], e[o + IE.v + 1], e[o + IE.v + 2]];
      if (what === IEK.impulse) this.props.applyImpulse(this.props.all[e[o + IE.a]], p, v, it.who);   // the item's lost momentum (canister ≈ 350 g; a rifle far more) goes into the chair / carton it struck
      else it.onBounce?.(e[o + IE.a], p, g!, it, v);
    }
  }
  /** ONE ROW'S STEP, HERE (rust/sim/src/items.rs update is this, statement for statement, for the rows it takes): row `r` aged, flown, bounced and
   *  brought to rest over the pool's columns; what it does to anything else listed onto `ev` (IE) */
  private stepRow(r: number, dt: number, ev: number[]): void {
    const S = this.store, O = S.O, d = S.d, u = S.u, B = S.b8 + r, W = S.b4 + r;
    d[B + O.age] += dt; const life = d[B + O.life] -= dt;
    if (life <= 0) { listed(ev, r, IEK.gone, 0, 0, 0, 0, 0, 0, 0); return; }
    const flags = u[W + O.flags];
    if (flags & ITF.carried) return;   // (disarm.ts: in Sam's hand — it goes where the hand goes)
    const all = this.props.all;
    let on = u[W + O.on], settled = (flags & ITF.settled) !== 0, dropped = false;
    if (settled) { const tumble = d[B + O.tumble]; d[B + O.tumbleRate] = 0; const flatA = Math.round(tumble / Math.PI) * Math.PI; d[B + O.tumble] = tumble + (flatA - tumble) * Math.min(1, dt * 12);   // comes to rest lying along its axis (0 or half a turn)
      // resting on a chair seat / carton: stays put while that prop is still underneath (re-checked whenever its pose changes — shoved, or snapped home
      // by a reset), drops once it has moved out from under it
      const lies = on ? all[on - 1] : null;
      if (!lies || poseKey(lies) === d[B + O.onPose]) return;
      const under = this.props.raycast([d[B + O.px], d[B + O.py] + 0.05, d[B + O.pz]], [0, -1, 0], 0.15);
      if (under && under.prop === lies) { d[B + O.onPose] = poseKey(lies); return; }
      settled = false; on = 0; dropped = true;
    }
    let px = d[B + O.px], py = d[B + O.py], pz = d[B + O.pz], vx = dropped ? 0 : d[B + O.vx], vy = dropped ? 0 : d[B + O.vy], vz = dropped ? 0 : d[B + O.vz];
    let yaw = d[B + O.yaw], spin = d[B + O.spin], tumble = d[B + O.tumble], tumbleRate = d[B + O.tumbleRate], heading = d[B + O.heading], onPose = d[B + O.onPose], bounces = u[W + O.bounces];
    const radius = d[B + O.radius], restitution = d[B + O.restitution], friction = d[B + O.friction], kind = ITEM_KINDS[u[W + O.kind]], bogs = this.bog;
    const sub = 2; const h = dt / sub;
    for (let s = 0; s < sub && !settled; s++) {
      vy -= GRAVITY * h;
      const m0 = vx * h, m1 = vy * h, m2 = vz * h; const len = hyp3(m0, m1, m2);
      if (len < 1e-6) continue;
      const il = 1 / len, d0 = m0 * il, d1 = m1 * il, d2 = m2 * il;
      // sphere-vs-box CCD from the centre ray: contact happens radius/cos(incidence) before the centre would hit
      let hit = this.ray([px, py, pz], [d0, d1, d2], len + radius * 3.5);
      let back = 0;
      if (hit) { const hn = hit.n; back = radius / Math.max(0.3, -(d0 * hn[0] + d1 * hn[1] + d2 * hn[2])); if (hit.t - back > len) hit = null; }
      if (hit) {
        // place at contact, reflect
        const tHit = Math.max(0, hit.t - back);
        px += d0 * tHit; py += d1 * tHit; pz += d2 * tHit;
        const n0 = hit.n[0], n1 = hit.n[1], n2 = hit.n[2]; const vn = vx * n0 + vy * n1 + vz * n2;
        if (vn < 0) {
          if (hit.prop) { const j = vn * (1 + restitution) * (kind === 'smoke' || kind === 'frag' ? 0.35 : kind === 'rifle' ? RIFLE_ITEM.propShare : 0.1); listed(ev, r, IEK.impulse, hit.prop.id, px, py, pz, n0 * j, n1 * j, n2 * j); }   // the item's lost momentum goes into the chair / carton it struck (handed over after the step)
          const vN0 = n0 * vn, vN1 = n1 * vn, vN2 = n2 * vn, vT0 = vx - vN0, vT1 = vy - vN1, vT2 = vz - vN2;
          const speed = -vn;
          const bog = bogs && n1 > 0.6 && !hit.prop ? bogs.at(px, py - radius, pz) : 0;   // lying snow under it (0 = none, and the lines below are what they were)
          const f = Math.max(0, 1 - friction * (0.5 + 0.5 * Math.abs(n1)));
          vx = vT0 * f + vN0 * -restitution; vy = vT1 * f + vN1 * -restitution; vz = vT2 * f + vN2 * -restitution;
          if (bog > 0) { const up = vx * n0 + vy * n1 + vz * n2, b = bogBounce(bog), sl = bogSlide(bog); vx = (vx - n0 * up) * sl + n0 * (up * b); vy = (vy - n1 * up) * sl + n1 * (up * b); vz = (vz - n2 * up) * sl + n2 * (up * b); }   // … it keeps little of its slide and of its bounce
          spin *= 0.6; tumbleRate *= 0.45; bounces++; if (Math.hypot(vx, vz) > 0.3) heading = Math.atan2(vx, vz);
          if (speed > 0.6) listed(ev, r, IEK.bounce, speed, px, py, pz, n0, n1, n2);
          // settle when slow on an upward-facing surface
          if (n1 > 0.6 && Math.abs(vy) < 0.5 && Math.hypot(vx, vz) < 0.35 + (bog > 0 ? bogStick(bog) : 0)) { settled = true; on = hit.prop ? hit.prop.id + 1 : 0; onPose = hit.prop ? poseKey(hit.prop) : 0; vx = 0; vy = 0; vz = 0; spin = 0; py = py + 0.001; }
        }
        px += n0 * 0.002; py += n1 * 0.002; pz += n2 * 0.002;
      } else {
        px += m0; py += m1; pz += m2;
      }
      if (py < radius) { // safety floor
        const bog = bogs ? bogs.at(px, 0, pz) : 0;
        py = radius; if (vy < 0) { const sp = -vy; vy *= -restitution; vx *= 0.7; vz *= 0.7; if (bog > 0) { vy *= bogBounce(bog); vx *= bogSlide(bog); vz *= bogSlide(bog); } bounces++; if (sp > 0.6) listed(ev, r, IEK.bounce, sp, px, py, pz, 0, 1, 0); }
        if (Math.abs(vy) < 0.5 && Math.hypot(vx, vz) < 0.35 + (bog > 0 ? bogStick(bog) : 0)) { settled = true; vx = 0; vy = 0; vz = 0; }
      }
      yaw += spin * h; tumble += tumbleRate * h;
    }
    d[B + O.px] = px; d[B + O.py] = py; d[B + O.pz] = pz; d[B + O.vx] = vx; d[B + O.vy] = vy; d[B + O.vz] = vz;
    d[B + O.yaw] = yaw; d[B + O.spin] = spin; d[B + O.tumble] = tumble; d[B + O.tumbleRate] = tumbleRate; d[B + O.heading] = heading; d[B + O.onPose] = onPose;
    u[W + O.bounces] = bounces; u[W + O.on] = on; u[W + O.flags] = settled ? flags | ITF.settled : flags & ~ITF.settled;
  }

  /** Boxes for rendering / RT (small; canisters cast shadows, mags do not to keep the grid lean). */
  boxes(out: Box[]) {
    for (const it of this.items) {
      const P = it.pos, kind = it.kind, settled = it.settled, yaw = it.yaw, tumble = it.tumble, radius = it.radius, fill = it.fill, serial = it.serial;   // (its row, read once)
      const n0 = out.length;   // (motion identity: every part this item emits below is keyed off its serial — tagged after the chain)
      if (kind === 'smoke' || kind === 'flash') {
        const standing = settled; const stun = kind === 'flash';
        // smoke can: olive body, red band. stun can: near-black body with a pale band, scorched once it has gone off
        const body: Vec3 = stun ? (it.fired ? [0.03, 0.03, 0.03] : [0.07, 0.075, 0.08]) : [0.25, 0.29, 0.2]; const band: Vec3 = stun ? (it.fired ? [0.12, 0.11, 0.1] : [0.7, 0.7, 0.66]) : [0.6, 0.08, 0.06];
        // A tossed can comes to rest on its SIDE: update() eases the end-over-end tumble to the nearest flat angle once it settles, so the same boxes as
        // in flight are drawn, lowered so the lying body (half-height 0.032) touches the floor instead of floating at the physics radius. (It used to be
        // drawn standing whenever settled — a leftover from before cans tumbled — which is why every can landed upright.)
        const c0: Vec3 = standing ? [P[0], P[1] - radius + 0.034, P[2]] : v3.copy(P);
          const rot = quat.mul(quat.yaw(yaw), quat.axisAngle([1, 0, 0], tumble));
          out.push(itemBox(v3.copy(c0), [0.032, 0.032, 0.07], yaw, rot, body, BoxFlag.Dynamic));
          out.push(itemBox(v3.add(c0, quat.rotate(rot, [0, 0, 0.045])), [0.034, 0.034, 0.012], yaw, rot, band, BoxFlag.Dynamic | BoxFlag.NoShadow));
      } else if (kind === 'frag') {   // a frag grenade (FRAG_ITEM): the olive body, its yellow band, the fuze stub — tumbling in the air, rolling on the floor (it never stands)
        const F = FRAG_ITEM; const c0: Vec3 = settled ? [P[0], P[1] - radius + F.half[0] + 0.002, P[2]] : v3.copy(P);
        const rot = quat.mul(quat.yaw(yaw), quat.axisAngle([1, 0, 0], tumble));
        out.push(itemBox(v3.copy(c0), v3.copy(F.half), yaw, rot, v3.copy(F.body), BoxFlag.Dynamic));
        out.push(itemBox(v3.add(c0, quat.rotate(rot, [0, 0, 0.012])), [0.034, 0.034, 0.006], yaw, rot, v3.copy(F.band), BoxFlag.Dynamic | BoxFlag.NoShadow));
        out.push(itemBox(v3.add(c0, quat.rotate(rot, [0, 0, 0.05])), [0.012, 0.012, 0.014], yaw, rot, v3.copy(F.fuze), BoxFlag.Dynamic | BoxFlag.NoShadow));
      } else if (kind === 'pistol' || kind === 'torch' || kind === 'rifle') {
        // dropped weapon: lies along its local z; in the air it flips end over end, on the floor it comes to rest on its side (tumble eased flat by update)
        // in the air: flips end over end about the across axis; settled: on its SIDE (a quarter roll about its own length) with that face on the floor
        const thick = kind === 'pistol' ? 0.016 : kind === 'rifle' ? (fill === 1 ? SC20K_ITEM.thick : RIFLE_ITEM.thick) : 0.022;
        const rot = settled ? quat.mul(quat.mul(quat.yaw(yaw), quat.axisAngle([1, 0, 0], Math.round(tumble / Math.PI) * Math.PI)), quat.axisAngle([0, 0, 1], Math.PI / 2))
                               : quat.mul(quat.yaw(yaw), quat.axisAngle([1, 0, 0], tumble));
        const c: Vec3 = settled ? [P[0], P[1] - radius + thick + 0.004, P[2]] : v3.copy(P);   // (the physics sphere rests at radius; the model's side face sits on the floor)
        const at = (off: Vec3): Vec3 => v3.add(c, quat.rotate(rot, off));
        if (kind === 'pistol') {   // slide + frame, grip raked back under the rear, suppressor can up front (the game's Five-seveN / the guards' sidearm read the same at this size)
          out.push(itemBox(at([0, 0, 0]), [0.016, 0.026, 0.095], yaw, rot, [0.05, 0.05, 0.055], BoxFlag.Dynamic));
          const gripRot = quat.mul(rot, quat.axisAngle([1, 0, 0], 108 * Math.PI / 180));
          out.push(itemBox(v3.add(at([0, -0.024, -0.045]), quat.rotate(gripRot, [0, 0, 0.047])), [0.014, 0.017, 0.047], yaw, gripRot, [0.05, 0.05, 0.055], BoxFlag.Dynamic));
          if (fill > 0) out.push(itemBox(at([0, 0, 0.17]), [0.019, 0.019, 0.075], yaw, rot, [0.09, 0.09, 0.09], BoxFlag.Dynamic));   // fill = 1: suppressed (the player's)
        } else if (kind === 'rifle' && fill === 1) {   // Sam's FN2000 (SC20K_ITEM): the polymer hull with its box down behind the grip, fore-end and can ahead, the sight block on the spine
          const S = GEAR.on ? M4_ITEM : SC20K_ITEM;   // (the 2010s issue kit's carbine: the same item, drawn as what was in his hands)
          out.push(itemBox(at([0, 0, S.hull.z]), v3.copy(S.hull.h), yaw, rot, v3.copy(S.polymer), BoxFlag.Dynamic));
          out.push(itemBox(at([0, 0, S.fore.z]), v3.copy(S.fore.h), yaw, rot, v3.copy(S.foreCol), BoxFlag.Dynamic));
          out.push(itemBox(at([0, S.mag.y, S.mag.z]), v3.copy(S.mag.h), yaw, rot, v3.copy(S.metal), BoxFlag.Dynamic));
          out.push(itemBox(at([0, S.optic.y, S.optic.z]), v3.copy(S.optic.h), yaw, rot, v3.copy(S.metal), BoxFlag.Dynamic | BoxFlag.NoShadow));
        } else if (kind === 'rifle') {   // the carbine that was in his hands (RIFLE_ITEM): receiver, the wood and steel ahead of it, the canted box under it, the skeleton stock behind
          const R = RIFLE_ITEM;   // (copies: makeBox keeps the arrays it is given, and these numbers are shared by every rifle on the floor)
          out.push(itemBox(at([0, 0, R.receiver.z]), v3.copy(R.receiver.h), yaw, rot, v3.copy(R.steel), BoxFlag.Dynamic));
          out.push(itemBox(at([0, 0, R.fore.z]), v3.copy(R.fore.h), yaw, rot, v3.copy(R.wood), BoxFlag.Dynamic));
          const magRot = quat.mul(rot, quat.axisAngle([1, 0, 0], R.mag.cant));
          out.push(itemBox(at([0, R.mag.y, R.mag.z]), v3.copy(R.mag.h), yaw, magRot, v3.copy(R.wood), BoxFlag.Dynamic));
          out.push(itemBox(at([0, -0.012, R.stock.z]), v3.copy(R.stock.h), yaw, rot, v3.copy(R.steel), BoxFlag.Dynamic | BoxFlag.NoShadow));   // (the open triangle as one thin slab: raster only, so no ray finds a paddle-shaped shadow caster the stock never was)
        } else {
          out.push(itemBox(at([0, 0, 0]), [0.022, 0.022, 0.09], yaw, rot, [0.09, 0.09, 0.1], BoxFlag.Dynamic));
          out.push(itemBox(at([0, 0, 0.095]), [0.026, 0.026, 0.008], yaw, rot, [0.16, 0.16, 0.15], BoxFlag.Dynamic | BoxFlag.NoShadow));   // dark lens: the torch died with its owner
        }
      } else if (kind === 'ring' && GEAR.on) {   // the 2010s issue kit's 40 mm sponge round, spent (SPONGE_ITEM): tumbling end over end in the air, lying on its side once down
        const c: Vec3 = settled ? [P[0], P[1] - radius + SPONGE_ITEM.wide + 0.002, P[2]] : v3.copy(P);
        spongeBoxes(out, c, settled ? quat.yaw(yaw) : quat.mul(quat.yaw(yaw), quat.axisAngle([1, 0, 0], tumble)), yaw);
      } else if (kind === 'ring') {   // a spent ring airfoil: three thin slats crossed at 60° lying flat (or tumbling flat-on in the air) — a small pale ring on the floor
        const G = RING_ITEM; const c: Vec3 = settled ? [P[0], P[1] - radius + G.half[1] + 0.002, P[2]] : v3.copy(P);
        const base = settled ? quat.yaw(yaw) : quat.mul(quat.yaw(yaw), quat.axisAngle([1, 0, 0], tumble));
        for (let k = 0; k < 3; k++) { const rot = quat.mul(base, quat.axisAngle([0, 1, 0], k * Math.PI / 3)); out.push(itemBox(v3.copy(c), v3.copy(G.half), yaw, rot, v3.copy(G.albedo), BoxFlag.Dynamic | BoxFlag.NoShadow)); }
      } else if (kind === 'shell') {   // a dud 40 mm round (SHELL_ITEM): the pale aluminium body with its gold ogive, tumbling in the air, lying on its side once down
        const S = SHELL_ITEM; const c0: Vec3 = settled ? [P[0], P[1] - radius + S.half[0] + 0.002, P[2]] : v3.copy(P);
        const rot = quat.mul(quat.yaw(yaw), quat.axisAngle([1, 0, 0], tumble));
        out.push(itemBox(v3.copy(c0), v3.copy(S.half), yaw, rot, v3.copy(S.body), BoxFlag.Dynamic));
        out.push(itemBox(v3.add(c0, quat.rotate(rot, [0, 0, S.half[2] * 0.62])), [S.half[0] + 0.001, S.half[1] + 0.001, S.half[2] * 0.4], yaw, rot, v3.copy(S.nose), BoxFlag.Dynamic | BoxFlag.NoShadow));
      } else {
        const flat = settled; const box30 = fill > 1, box20k = fill >= 3;   // fill 2: a rifleman's empty 30-round box (guards.ts tickReloadBeat) — longer, and bakelite-coloured like the one on the gun; fill 3: the FN2000's black thirty out of Sam's reload (player.ts); else a pistol magazine (fill 0 empty, 0‥1 part full)
        // fill 4‥7 (gunreload.ts, a gun fed its own way — nobody spawns one with gunlook.ts off): the Glock 18's long magazine · the Mk 46's belt box · the Mk 12's short twenty · the MP7's long thin forty
        const own = fill >= 4 ? ([{ hl: 0.11, hw: 0.016, th: 0.012, col: [0.06, 0.06, 0.065] }, { hl: 0.06, hw: 0.045, th: 0.035, col: [0.16, 0.17, 0.1] }, { hl: 0.065, hw: 0.014, th: 0.012, col: [0.03, 0.03, 0.033] }, { hl: 0.1, hw: 0.011, th: 0.012, col: [0.03, 0.03, 0.033] }] as const)[Math.min(3, fill - 4)] : null;
        const col: Vec3 = own ? [own.col[0], own.col[1], own.col[2]] : box20k ? v3.copy(SC20K_ITEM.metal) : box30 ? v3.copy(RIFLE_ITEM.wood) : fill > 0 ? [0.09, 0.09, 0.1] : [0.06, 0.06, 0.065]; const hl = own ? own.hl : box30 ? 0.095 : 0.055, hw = own ? own.hw : box30 ? 0.014 : 0.018, th = own ? own.th : 0.012;
        if (flat) out.push(makeBox({ c: [P[0], P[1] + th, P[2]], h: [hw, th, hl], yaw: yaw, albedo: col, flags: BoxFlag.Dynamic | BoxFlag.NoShadow }));
        else out.push(itemBox(v3.copy(P), [hw, hl, own ? Math.max(0.03, own.th) : 0.03], yaw, quat.mul(quat.yaw(yaw), quat.axisAngle([1, 0, 0], tumble)), col, BoxFlag.Dynamic | BoxFlag.NoShadow));   // a dropped magazine tumbles too
      }
      for (let k = n0; k < out.length; k++) out[k].mkey = MKEY.item + (serial & 8191) * 8 + Math.min(7, k - n0);   // MOTION IDENTITY per part (scene/boxes.ts MKEY): the g-buffer reprojects the flying can through ITS last transform whatever spawned or died around it in the list
    }
  }
}

const poseKey = (p: Prop) => p.x * 7919 + p.z * 104729 + p.yaw;   // any change of pose changes this (exact float compare; not a hash to trust across different poses, just 'did it move')
/** a canister that is still SOMETHING: its fuse not yet gone, or (smoke) gone and still venting — never scenery for the item cap to sweep up */
const liveCan = (x: Item) => (x.kind === 'smoke' || x.kind === 'flash' || x.kind === 'frag') && (!x.fired || x.emitT > 0);
