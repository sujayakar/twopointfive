// WRITTEN BY tools/sim/men.ts FROM src/sim/men/schema.ts — never by hand (tools/qa/men-check.ts is red on a difference). A man's columns, as the game reads and writes them.
import type { Vec3 } from '../../math/vec';
import type { MenStore } from './store';

/** the hash of the layout; the module holds the same one (rust/sim/src/men.rs SCHEMA), and store.ts holds the two to each other as a game starts */
export const MEN_SCHEMA = 0xb81963e5;
export const MEN_HEAD = 16;
export const MEN_F64 = ["posX","posY","posZ","velX","velY","velZ","bodyYaw","aimYaw","aimPitch","radius","stepPhase","speed","lastPosX","lastPosY","lastPosZ","speedSm","hitsLeft","fireCd","noise","stepTravel","stepOwed","stepAt","since","hurtAt","staggerUntil","dazzledUntil","muzzleAhead","diedAt","killerId","missedAt","muzzleClearT","muzzleEvalT","lineBarkT","lineInT","gunLostAt","torchLostAt","lightDeadUntil","doorDownUntil","reloadT","lastFireT","friendT","hitFlash","doorBashAt","doorBashLean","doorRipUntil","doorHead","doorHeadT","dragYaw","dragNoiseT","dragStuckT","equipT","pace","paceShownT","nvAmount"] as const;
export const MEN_U32 = ["gen","truths0","state","owner","rank","reason","downKind","downBy","truths1"] as const;
/** the generation column, as an index into the words from the part's start (the store's own) */
export const MEN_GEN = 6916;
const STATE_WORDS = ["patrol","suspicious","alert","search","dead"] as const;
const DOWNKIND_WORDS = [null,"shot","struck","choked","wounded"] as const;
const DOWNBY_WORDS = [null,"player","guard","world"] as const;

/** what every man is: which store his columns are in and his row. Character and Guard extend these, so a column reads as a field */
export class ManCols {
  S!: MenStore; r = -1;
  get pos(): Vec3 { const S = this.S, i = S.b8 + this.r; return [S.d[i + 2], S.d[i + 66], S.d[i + 130]]; }
  set pos(v: Readonly<Vec3>) { const S = this.S, i = S.b8 + this.r; S.d[i + 2] = v[0]; S.d[i + 66] = v[1]; S.d[i + 130] = v[2]; }
  get posX(): number { const S = this.S; return S.d[S.b8 + 2 + this.r]; } set posX(v: number) { const S = this.S; S.d[S.b8 + 2 + this.r] = v; }
  get posY(): number { const S = this.S; return S.d[S.b8 + 66 + this.r]; } set posY(v: number) { const S = this.S; S.d[S.b8 + 66 + this.r] = v; }
  get posZ(): number { const S = this.S; return S.d[S.b8 + 130 + this.r]; } set posZ(v: number) { const S = this.S; S.d[S.b8 + 130 + this.r] = v; }
  get vel(): Vec3 { const S = this.S, i = S.b8 + this.r; return [S.d[i + 194], S.d[i + 258], S.d[i + 322]]; }
  set vel(v: Readonly<Vec3>) { const S = this.S, i = S.b8 + this.r; S.d[i + 194] = v[0]; S.d[i + 258] = v[1]; S.d[i + 322] = v[2]; }
  get velX(): number { const S = this.S; return S.d[S.b8 + 194 + this.r]; } set velX(v: number) { const S = this.S; S.d[S.b8 + 194 + this.r] = v; }
  get velY(): number { const S = this.S; return S.d[S.b8 + 258 + this.r]; } set velY(v: number) { const S = this.S; S.d[S.b8 + 258 + this.r] = v; }
  get velZ(): number { const S = this.S; return S.d[S.b8 + 322 + this.r]; } set velZ(v: number) { const S = this.S; S.d[S.b8 + 322 + this.r] = v; }
  get bodyYaw(): number { const S = this.S; return S.d[S.b8 + 386 + this.r]; } set bodyYaw(v: number) { const S = this.S; S.d[S.b8 + 386 + this.r] = v; }
  get aimYaw(): number { const S = this.S; return S.d[S.b8 + 450 + this.r]; } set aimYaw(v: number) { const S = this.S; S.d[S.b8 + 450 + this.r] = v; }
  get aimPitch(): number { const S = this.S; return S.d[S.b8 + 514 + this.r]; } set aimPitch(v: number) { const S = this.S; S.d[S.b8 + 514 + this.r] = v; }
  get radius(): number { const S = this.S; return S.d[S.b8 + 578 + this.r]; } set radius(v: number) { const S = this.S; S.d[S.b8 + 578 + this.r] = v; }
  /** where his walk stands in its cycle */
  get stepPhase(): number { const S = this.S; return S.d[S.b8 + 642 + this.r]; } set stepPhase(v: number) { const S = this.S; S.d[S.b8 + 642 + this.r] = v; }
  get alive(): boolean { const S = this.S; return (S.u[S.b4 + 6980 + this.r] & 1) !== 0; } set alive(v: boolean) { const S = this.S; const w = S.u[S.b4 + 6980 + this.r]; S.u[S.b4 + 6980 + this.r] = v ? w | 1 : w & 4294967294; }
  /** What he carries and its state (weapons.ts: the row from his route — the 9 mm service pistol unless the level says otherwise — with its chamber and magazine; guards.ts fires it and reads its cadence, fireWeapon its report / flash / spread, killGuard what falls out of the hand). fireCd: seconds until the trigger will go again (the AI's beat: cadence, the reaction on going alert, the shield's floor). */
  get fireCd(): number { const S = this.S; return S.d[S.b8 + 1090 + this.r]; } set fireCd(v: number) { const S = this.S; S.d[S.b8 + 1090 + this.r] = v; }
  /** which rule has him (0: nobody) */
  get owner(): number { const S = this.S; return S.u[S.b4 + 7108 + this.r]; } set owner(v: number) { const S = this.S; S.u[S.b4 + 7108 + this.r] = v; }
  /** how hard that rule holds him against another */
  get rank(): number { const S = this.S; return S.u[S.b4 + 7172 + this.r]; } set rank(v: number) { const S = this.S; S.u[S.b4 + 7172 + this.r] = v; }
  /** why it has him */
  get reason(): number { const S = this.S; return S.u[S.b4 + 7236 + this.r]; } set reason(v: number) { const S = this.S; S.u[S.b4 + 7236 + this.r] = v; }
  /** the game time it took him */
  get since(): number { const S = this.S; return S.d[S.b8 + 1410 + this.r]; } set since(v: number) { const S = this.S; S.d[S.b8 + 1410 + this.r] = v; }
  /** game time a blast left him hurt (condition.ts: -1 = never; read through isHurt) */
  get hurtAt(): number { const S = this.S; return S.d[S.b8 + 1474 + this.r]; } set hurtAt(v: number) { const S = this.S; S.d[S.b8 + 1474 + this.r] = v; }
  /** a ring airfoil took him in the body (launcher.ts staggerGuard): doubled over till this time — rooted like the dazzle (reactT), no round from him, but he can SEE, and he comes out of it alert toward the side it came from */
  get staggerUntil(): number { const S = this.S; return S.d[S.b8 + 1538 + this.r]; } set staggerUntil(v: number) { const S = this.S; S.d[S.b8 + 1538 + this.r] = v; }
  /** stun canister with line of sight: no vision, no shooting, rooted to the spot until this time (see Game.detonateStun) */
  get dazzledUntil(): number { const S = this.S; return S.d[S.b8 + 1602 + this.r]; } set dazzledUntil(v: number) { const S = this.S; S.d[S.b8 + 1602 + this.r] = v; }
  /** metres along gunDir from the wrist to the muzzle: the row's (weapons.ts WeaponDef.muzzleAhead — 0.26 for a pistol; whoever hands him a weapon writes it — or, for a man dealt the carbine over his sidearm row (game.ts, tune.carbineGuards), rigProps.carbineMuzzleAhead's 0.5 off the drawn prop). The round, the flash, the laser and armThrough all start at `muzzle`. */
  get muzzleAhead(): number { const S = this.S; return S.d[S.b8 + 1666 + this.r]; } set muzzleAhead(v: number) { const S = this.S; S.d[S.b8 + 1666 + this.r] = v; }
  initMan(): void { this.radius = 0.3; this.alive = true; this.hurtAt = -1; this.staggerUntil = -1; this.dazzledUntil = -1; this.muzzleAhead = 0.26; }
}
/** a Guard record's own columns (a guard, a teammate, a civilian) */
export class GuardCols extends ManCols {
  /** how fast his feet are going (m/s) */
  get speed(): number { const S = this.S; return S.d[S.b8 + 706 + this.r]; } set speed(v: number) { const S = this.S; S.d[S.b8 + 706 + this.r] = v; }
  /** where his watchdog last saw him */
  get lastPos(): Vec3 { const S = this.S, i = S.b8 + this.r; return [S.d[i + 770], S.d[i + 834], S.d[i + 898]]; }
  set lastPos(v: Readonly<Vec3>) { const S = this.S, i = S.b8 + this.r; S.d[i + 770] = v[0]; S.d[i + 834] = v[1]; S.d[i + 898] = v[2]; }
  get lastPosX(): number { const S = this.S; return S.d[S.b8 + 770 + this.r]; } set lastPosX(v: number) { const S = this.S; S.d[S.b8 + 770 + this.r] = v; }
  get lastPosY(): number { const S = this.S; return S.d[S.b8 + 834 + this.r]; } set lastPosY(v: number) { const S = this.S; S.d[S.b8 + 834 + this.r] = v; }
  get lastPosZ(): number { const S = this.S; return S.d[S.b8 + 898 + this.r]; } set lastPosZ(v: number) { const S = this.S; S.d[S.b8 + 898 + this.r] = v; }
  get state(): (typeof STATE_WORDS)[number] { const S = this.S; return STATE_WORDS[S.u[S.b4 + 7044 + this.r]]; } set state(v: (typeof STATE_WORDS)[number]) { const S = this.S, i = STATE_WORDS.indexOf(v); if (i < 0) throw new Error(`state: no such word: ${v}`); S.u[S.b4 + 7044 + this.r] = i; }
  /** muzzle discipline (guards.ts muzzleCheck): a living colleague stands in his line this frame, so the pistol is at the low ready and no round leaves it; clearT = seconds the line has been clear again (it comes back up after a beat, not on the frame); evalT = the game time it was last worked out (the alert branch does it early, before the shot); lineBarkT = when he (or the man in his line) last said  */
  get muzzleDown(): boolean { const S = this.S; return (S.u[S.b4 + 6980 + this.r] & 128) !== 0; } set muzzleDown(v: boolean) { const S = this.S; const w = S.u[S.b4 + 6980 + this.r]; S.u[S.b4 + 6980 + this.r] = v ? w | 128 : w & 4294967167; }
  /** alarm escalation (guards.ts updateEscalation): drawn = the sidearm is out on patrol (level ≥ 1) — handProps / attachFlashlight / killGuard read it where they used to read 'patrol ⇒ torch'; leader = lockdown pairing: trail this man while calm (followT: re-plan clock, ≤ 2 path plans a second; followGoal: his station behind him); post = lockdown: hold this corridor junction instead of walking the rou */
  get drawn(): boolean { const S = this.S; return (S.u[S.b4 + 6980 + this.r] & 256) !== 0; } set drawn(v: boolean) { const S = this.S; const w = S.u[S.b4 + 6980 + this.r]; S.u[S.b4 + 6980 + this.r] = v ? w | 256 : w & 4294967039; }
  get diedAt(): number { const S = this.S; return S.d[S.b8 + 1730 + this.r]; } set diedAt(v: number) { const S = this.S; S.d[S.b8 + 1730 + this.r] = v; }
  /** (dead) how he went down and by whose hand — set once by combat.ts killGuard; null while he lives. isBreathing() reads it: struck / choked = out cold. */
  get downKind(): (typeof DOWNKIND_WORDS)[number] { const S = this.S; return DOWNKIND_WORDS[S.u[S.b4 + 7300 + this.r]]; } set downKind(v: (typeof DOWNKIND_WORDS)[number]) { const S = this.S, i = DOWNKIND_WORDS.indexOf(v); if (i < 0) throw new Error(`downKind: no such word: ${v}`); S.u[S.b4 + 7300 + this.r] = i; }
  /** (dead) how he went down and by whose hand — set once by combat.ts killGuard; null while he lives. isBreathing() reads it: struck / choked = out cold. */
  get downBy(): (typeof DOWNBY_WORDS)[number] { const S = this.S; return DOWNBY_WORDS[S.u[S.b4 + 7364 + this.r]]; } set downBy(v: (typeof DOWNBY_WORDS)[number]) { const S = this.S, i = DOWNBY_WORDS.indexOf(v); if (i < 0) throw new Error(`downBy: no such word: ${v}`); S.u[S.b4 + 7364 + this.r] = i; }
  /** (dead) the id of the character whose round put him down, −1 = not by a round / unknown (combat.ts fireWeapon stamps it; the engagement record names him) */
  get killerId(): number { const S = this.S; return S.d[S.b8 + 1794 + this.r]; } set killerId(v: number) { const S = this.S; S.d[S.b8 + 1794 + this.r] = v; }
  /** (dead) somebody has discovered this body — counted once for the debrief */
  get found(): boolean { const S = this.S; return (S.u[S.b4 + 7428 + this.r] & 1) !== 0; } set found(v: boolean) { const S = this.S; const w = S.u[S.b4 + 7428 + this.r]; S.u[S.b4 + 7428 + this.r] = v ? w | 1 : w & 4294967294; }
  /** (dead, not found) the net has called this man and got nothing back — when (guards.ts updateRollcall; -1 = never): from then on the others count him as missing (superviseLockdown stops dealing his corpse a role) — and whether the floor has already been raised once on his account (it is, once, when the man sent round his route comes back with nothing) */
  get missedAt(): number { const S = this.S; return S.d[S.b8 + 1858 + this.r]; } set missedAt(v: number) { const S = this.S; S.d[S.b8 + 1858 + this.r] = v; }
  /** (dead, not found) the net has called this man and got nothing back — when (guards.ts updateRollcall; -1 = never): from then on the others count him as missing (superviseLockdown stops dealing his corpse a role) — and whether the floor has already been raised once on his account (it is, once, when the man sent round his route comes back with nothing) */
  get missingRaised(): boolean { const S = this.S; return (S.u[S.b4 + 7428 + this.r] & 2) !== 0; } set missingRaised(v: boolean) { const S = this.S; const w = S.u[S.b4 + 7428 + this.r]; S.u[S.b4 + 7428 + this.r] = v ? w | 2 : w & 4294967293; }
  /** muzzle discipline (guards.ts muzzleCheck): a living colleague stands in his line this frame, so the pistol is at the low ready and no round leaves it; clearT = seconds the line has been clear again (it comes back up after a beat, not on the frame); evalT = the game time it was last worked out (the alert branch does it early, before the shot); lineBarkT = when he (or the man in his line) last said  */
  get muzzleClearT(): number { const S = this.S; return S.d[S.b8 + 1922 + this.r]; } set muzzleClearT(v: number) { const S = this.S; S.d[S.b8 + 1922 + this.r] = v; }
  /** muzzle discipline (guards.ts muzzleCheck): a living colleague stands in his line this frame, so the pistol is at the low ready and no round leaves it; clearT = seconds the line has been clear again (it comes back up after a beat, not on the frame); evalT = the game time it was last worked out (the alert branch does it early, before the shot); lineBarkT = when he (or the man in his line) last said  */
  get muzzleEvalT(): number { const S = this.S; return S.d[S.b8 + 1986 + this.r]; } set muzzleEvalT(v: number) { const S = this.S; S.d[S.b8 + 1986 + this.r] = v; }
  /** muzzle discipline (guards.ts muzzleCheck): a living colleague stands in his line this frame, so the pistol is at the low ready and no round leaves it; clearT = seconds the line has been clear again (it comes back up after a beat, not on the frame); evalT = the game time it was last worked out (the alert branch does it early, before the shot); lineBarkT = when he (or the man in his line) last said  */
  get lineBarkT(): number { const S = this.S; return S.d[S.b8 + 2050 + this.r]; } set lineBarkT(v: number) { const S = this.S; S.d[S.b8 + 2050 + this.r] = v; }
  /** alert, licensed to shoot: seconds a colleague has stood in the line of the shot itself (a man crossing is waited out for a beat; one standing there is stepped round) */
  get lineInT(): number { const S = this.S; return S.d[S.b8 + 2114 + this.r]; } set lineInT(v: number) { const S = this.S; S.d[S.b8 + 2114 + this.r] = v; }
  /** a LIVING man whose gun was knocked from his hand at the grab (HOLD.dropGun): dropped = that gun, this = when — his hands are empty (guards.ts inHand) and its light dies on the floor; −1 again once he has it back (player.ts handBack) */
  get gunLostAt(): number { const S = this.S; return S.d[S.b8 + 2178 + this.r]; } set gunLostAt(v: number) { const S = this.S; S.d[S.b8 + 2178 + this.r] = v; }
  /** a LIVING man whose torch was knocked from his hand (the grab): dropped = that torch, this = when — the beam follows it down and sputters out (guards.ts updateFlashlight); on patrol afterwards his hand is empty */
  get torchLostAt(): number { const S = this.S; return S.d[S.b8 + 2242 + this.r]; } set torchLostAt(v: number) { const S = this.S; S.d[S.b8 + 2242 + this.r] = v; }
  get lightDeadUntil(): number { const S = this.S; return S.d[S.b8 + 2306 + this.r]; } set lightDeadUntil(v: number) { const S = this.S; S.d[S.b8 + 2306 + this.r] = v; }
  /** was his light burning the moment it left his hand (killed / grabbed)? a dark weapon light must not sputter ON as it tumbles (review 08-10 #7: the breach hostiles' unlit lights swept the sunlit room the frame each died) */
  get lightLitAtLoss(): boolean { const S = this.S; return (S.u[S.b4 + 7428 + this.r] & 4) !== 0; } set lightLitAtLoss(v: boolean) { const S = this.S; const w = S.u[S.b4 + 7428 + this.r]; S.u[S.b4 + 7428 + this.r] = v ? w | 4 : w & 4294967291; }
  /** a booted door met him where he stood right at it (doorthrow.ts): on a knee till this time, inside the fold the same blow gave him */
  get doorDownUntil(): number { const S = this.S; return S.d[S.b8 + 2370 + this.r]; } set doorDownUntil(v: number) { const S = this.S; S.d[S.b8 + 2370 + this.r] = v; }
  /** a trained man (the response team, game/qrf.ts): the aim-settle model lines him up quicker and his hands are steadier (aim.ts TRAINED — × the settle time, × his row's cone), and qrf.ts keeps his reaction beats short. False for the night staff and the showcase's men: their numbers are untouched. */
  get trained(): boolean { const S = this.S; return (S.u[S.b4 + 7428 + this.r] & 8) !== 0; } set trained(v: boolean) { const S = this.S; const w = S.u[S.b4 + 7428 + this.r]; S.u[S.b4 + 7428 + this.r] = v ? w | 8 : w & 4294967287; }
  /** scripted, SHOWCASE CAST ONLY (director.ts LoadoutSpec.god — the day showcase's two operatives): a round that would have found him passes through him as it passes a colleague (combat.ts fireWeapon). The show's doctrine keeps the opposition's rounds off them by construction (laid on their cover, traced: tools/qa/day-probes.ts); this is the net under a stray, never the mechanism, and nothing the play */
  get god(): boolean { const S = this.S; return (S.u[S.b4 + 7428 + this.r] & 16) !== 0; } set god(v: boolean) { const S = this.S; const w = S.u[S.b4 + 7428 + this.r]; S.u[S.b4 + 7428 + this.r] = v ? w | 16 : w & 4294967279; }
  /** rifles (docs/internal/sc20k-design.md §3): dealt the carbine by the sandbox knob (tune.carbineGuards, written by Game.update each frame) — and the deal IS the weapon: while it stands, `weapon` is the AKS-74U row (bursts, the 30-round box, the harder report, the rifle that drops — guards.ts burstTrigger, combat.ts), the RIG in his hands the carbine and his poses the long gun's; dealt away again, hi */
  get carbine(): boolean { const S = this.S; return (S.u[S.b4 + 7428 + this.r] & 32) !== 0; } set carbine(v: boolean) { const S = this.S; const w = S.u[S.b4 + 7428 + this.r]; S.u[S.b4 + 7428 + this.r] = v ? w | 32 : w & 4294967263; }
  /** What he carries and its state (weapons.ts: the row from his route — the 9 mm service pistol unless the level says otherwise — with its chamber and magazine; guards.ts fires it and reads its cadence, fireWeapon its report / flash / spread, killGuard what falls out of the hand). fireCd: seconds until the trigger will go again (the AI's beat: cadence, the reaction on going alert, the shield's floor). */
  get reloadT(): number { const S = this.S; return S.d[S.b8 + 2434 + this.r]; } set reloadT(v: number) { const S = this.S; S.d[S.b8 + 2434 + this.r] = v; }
  initGuard(): void { this.diedAt = -1; this.killerId = -1; this.missedAt = -1; this.muzzleEvalT = -1; this.lineBarkT = -100; this.gunLostAt = -1; this.torchLostAt = -1; this.lightDeadUntil = -1; this.lightLitAtLoss = true; this.doorDownUntil = -1; this.reloadT = -1; }
}
/** the player's own columns */
export class PlayerCols extends ManCols {
  /** his speed, eased */
  get speedSm(): number { const S = this.S; return S.d[S.b8 + 962 + this.r]; } set speedSm(v: number) { const S = this.S; S.d[S.b8 + 962 + this.r] = v; }
  /** one 9 mm hit and you are down, same as the guards; Enter restarts the encounter */
  get down(): boolean { const S = this.S; return (S.u[S.b4 + 6980 + this.r] & 2) !== 0; } set down(v: boolean) { const S = this.S; const w = S.u[S.b4 + 6980 + this.r]; S.u[S.b4 + 6980 + this.r] = v ? w | 2 : w & 4294967293; }
  get crouch(): boolean { const S = this.S; return (S.u[S.b4 + 6980 + this.r] & 4) !== 0; } set crouch(v: boolean) { const S = this.S; const w = S.u[S.b4 + 6980 + this.r]; S.u[S.b4 + 6980 + this.r] = v ? w | 4 : w & 4294967291; }
  get sprinting(): boolean { const S = this.S; return (S.u[S.b4 + 6980 + this.r] & 8) !== 0; } set sprinting(v: boolean) { const S = this.S; const w = S.u[S.b4 + 6980 + this.r]; S.u[S.b4 + 6980 + this.r] = v ? w | 8 : w & 4294967287; }
  get backpedal(): boolean { const S = this.S; return (S.u[S.b4 + 6980 + this.r] & 16) !== 0; } set backpedal(v: boolean) { const S = this.S; const w = S.u[S.b4 + 6980 + this.r]; S.u[S.b4 + 6980 + this.r] = v ? w | 16 : w & 4294967279; }
  get aimHeld(): boolean { const S = this.S; return (S.u[S.b4 + 6980 + this.r] & 32) !== 0; } set aimHeld(v: boolean) { const S = this.S; const w = S.u[S.b4 + 6980 + this.r]; S.u[S.b4 + 6980 + this.r] = v ? w | 32 : w & 4294967263; }
  /** he starts with the pistol on the thigh (playtest call: hands free by default — Space grabs, a click behind a man is the takedown); E puts the selected item away / draws it again (Chaos Theory: equip / unequip); holstered + LMB behind a guard in reach = the silent takedown (its one and only input), the pistol's paths all check it */
  get holstered(): boolean { const S = this.S; return (S.u[S.b4 + 6980 + this.r] & 64) !== 0; } set holstered(v: boolean) { const S = this.S; const w = S.u[S.b4 + 6980 + this.r]; S.u[S.b4 + 6980 + this.r] = v ? w | 64 : w & 4294967231; }
  /** one 9 mm hit and you are down, same as the guards; Enter restarts the encounter */
  get hitsLeft(): number { const S = this.S; return S.d[S.b8 + 1026 + this.r]; } set hitsLeft(v: number) { const S = this.S; S.d[S.b8 + 1026 + this.r] = v; }
  /** HUD sound meter: the loudest thing you did lately (hearing-model level, decays) */
  get noise(): number { const S = this.S; return S.d[S.b8 + 1154 + this.r]; } set noise(v: number) { const S = this.S; S.d[S.b8 + 1154 + this.r] = v; }
  /** how far the walk cycle has come since the last footfall filed, in turns, either way added up, 1 at the most (player.ts FOOTFALL.once; 1 = far enough, which is where it starts, and where it stays while the key has never been on) */
  get stepTravel(): number { const S = this.S; return S.d[S.b8 + 1218 + this.r]; } set stepTravel(v: number) { const S = this.S; S.d[S.b8 + 1218 + this.r] = v; }
  /** how far the walk cycle has come at a footfall's speed since the last edge of it passed or footfall filed, in turns, either way added up (player.ts FOOTFALL.owed: at half a turn a footfall is owed; 0 where it starts, when he slows, after frames the footfall test was not asked, and while the key is down) */
  get stepOwed(): number { const S = this.S; return S.d[S.b8 + 1282 + this.r]; } set stepOwed(v: number) { const S = this.S; S.d[S.b8 + 1282 + this.r] = v; }
  /** the game clock the footfall test was last asked at with the key up (-1: never) */
  get stepAt(): number { const S = this.S; return S.d[S.b8 + 1346 + this.r]; } set stepAt(v: number) { const S = this.S; S.d[S.b8 + 1346 + this.r] = v; }
  get lastFireT(): number { const S = this.S; return S.d[S.b8 + 2498 + this.r]; } set lastFireT(v: number) { const S = this.S; S.d[S.b8 + 2498 + this.r] = v; }
  /** friends.ts: his raised gun is down for a teammate in its line (FRIENDS.lowReady), and how long that line has been clear */
  get friendLow(): boolean { const S = this.S; return (S.u[S.b4 + 7428 + this.r] & 64) !== 0; } set friendLow(v: boolean) { const S = this.S; const w = S.u[S.b4 + 7428 + this.r]; S.u[S.b4 + 7428 + this.r] = v ? w | 64 : w & 4294967231; }
  /** friends.ts: his raised gun is down for a teammate in its line (FRIENDS.lowReady), and how long that line has been clear */
  get friendT(): number { const S = this.S; return S.d[S.b8 + 2562 + this.r]; } set friendT(v: number) { const S = this.S; S.d[S.b8 + 2562 + this.r] = v; }
  get hitFlash(): number { const S = this.S; return S.d[S.b8 + 2626 + this.r]; } set hitFlash(v: number) { const S = this.S; S.d[S.b8 + 2626 + this.r] = v; }
  /** a blast that left him up (samstun.ts): folded until / blind until, on the game clock, and where the flash was — the same two stamps a Guard carries */
  get stunPosed(): boolean { const S = this.S; return (S.u[S.b4 + 7428 + this.r] & 128) !== 0; } set stunPosed(v: boolean) { const S = this.S; const w = S.u[S.b4 + 7428 + this.r]; S.u[S.b4 + 7428 + this.r] = v ? w | 128 : w & 4294967167; }
  /** a sprint into a door (sprintdoor.ts): when he last put his shoulder through one and which way it turned him, and until when his hand is ripping one round (the run held to a walk) — on the game clock */
  get doorBashAt(): number { const S = this.S; return S.d[S.b8 + 2690 + this.r]; } set doorBashAt(v: number) { const S = this.S; S.d[S.b8 + 2690 + this.r] = v; }
  /** a sprint into a door (sprintdoor.ts): when he last put his shoulder through one and which way it turned him, and until when his hand is ripping one round (the run held to a walk) — on the game clock */
  get doorBashLean(): number { const S = this.S; return S.d[S.b8 + 2754 + this.r]; } set doorBashLean(v: number) { const S = this.S; S.d[S.b8 + 2754 + this.r] = v; }
  /** a sprint into a door (sprintdoor.ts): when he last put his shoulder through one and which way it turned him, and until when his hand is ripping one round (the run held to a walk) — on the game clock */
  get doorRipUntil(): number { const S = this.S; return S.d[S.b8 + 2818 + this.r]; } set doorRipUntil(v: number) { const S = this.S; S.d[S.b8 + 2818 + this.r] = v; }
  /** a sprint into a door (sprintdoor.ts): when he last put his shoulder through one and which way it turned him, and until when his hand is ripping one round (the run held to a walk) — on the game clock */
  get doorKickTail(): boolean { const S = this.S; return (S.u[S.b4 + 7428 + this.r] & 256) !== 0; } set doorKickTail(v: boolean) { const S = this.S; const w = S.u[S.b4 + 7428 + this.r]; S.u[S.b4 + 7428 + this.r] = v ? w | 256 : w & 4294967039; }
  /** the look his head was last given against that turn, and the game time it was given on (player.ts's sprint facing; game.ts takes it back off the frame nobody renews it) */
  get doorHead(): number { const S = this.S; return S.d[S.b8 + 2882 + this.r]; } set doorHead(v: number) { const S = this.S; S.d[S.b8 + 2882 + this.r] = v; }
  /** the look his head was last given against that turn, and the game time it was given on (player.ts's sprint facing; game.ts takes it back off the frame nobody renews it) */
  get doorHeadT(): number { const S = this.S; return S.d[S.b8 + 2946 + this.r]; } set doorHeadT(v: number) { const S = this.S; S.d[S.b8 + 2946 + this.r] = v; }
  /** the body being dragged (F): crouched, facing it, its ankles in your hands (ragdoll pins), the rest trailing along the floor */
  get dragYaw(): number { const S = this.S; return S.d[S.b8 + 3010 + this.r]; } set dragYaw(v: number) { const S = this.S; S.d[S.b8 + 3010 + this.r] = v; }
  /** the body being dragged (F): crouched, facing it, its ankles in your hands (ragdoll pins), the rest trailing along the floor */
  get dragNoiseT(): number { const S = this.S; return S.d[S.b8 + 3074 + this.r]; } set dragNoiseT(v: number) { const S = this.S; S.d[S.b8 + 3074 + this.r] = v; }
  /** the body being dragged (F): crouched, facing it, its ankles in your hands (ragdoll pins), the rest trailing along the floor */
  get dragStuckT(): number { const S = this.S; return S.d[S.b8 + 3138 + this.r]; } set dragStuckT(v: number) { const S = this.S; S.d[S.b8 + 3138 + this.r] = v; }
  /** seconds left of the rifle coming off the sling into the hands (or of any change of what is in them that involves it): no round or ring meanwhile */
  get equipT(): number { const S = this.S; return S.d[S.b8 + 3202 + this.r]; } set equipT(v: number) { const S = this.S; S.d[S.b8 + 3202 + this.r] = v; }
  /** walking-pace multiplier set with the mouse wheel, PACE_MIN‥1 of the walk / crouch-walk speed (sprint is its own held modifier); shown as ticks in the kit column */
  get pace(): number { const S = this.S; return S.d[S.b8 + 3266 + this.r]; } set pace(v: number) { const S = this.S; S.d[S.b8 + 3266 + this.r] = v; }
  /** walking-pace multiplier set with the mouse wheel, PACE_MIN‥1 of the walk / crouch-walk speed (sprint is its own held modifier); shown as ticks in the kit column */
  get paceShownT(): number { const S = this.S; return S.d[S.b8 + 3330 + this.r]; } set paceShownT(v: number) { const S = this.S; S.d[S.b8 + 3330 + this.r] = v; }
  get nv(): boolean { const S = this.S; return (S.u[S.b4 + 7428 + this.r] & 512) !== 0; } set nv(v: boolean) { const S = this.S; const w = S.u[S.b4 + 7428 + this.r]; S.u[S.b4 + 7428 + this.r] = v ? w | 512 : w & 4294966783; }
  get nvAmount(): number { const S = this.S; return S.d[S.b8 + 3394 + this.r]; } set nvAmount(v: number) { const S = this.S; S.d[S.b8 + 3394 + this.r] = v; }
  initPlayer(): void { this.stepTravel = 1; this.stepAt = -1; this.holstered = true; this.hitsLeft = 1; this.lastFireT = -10; this.friendT = -10; this.doorBashAt = -10; this.doorRipUntil = -1; this.doorHeadT = -1; this.pace = 1; this.paceShownT = -10; }
}
/** the animator's kept state */
export class AnimCols {
  S!: MenStore; r = -1;
}
