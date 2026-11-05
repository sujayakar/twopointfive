// WRITTEN BY tools/sim/men.ts FROM src/sim/men/schema.ts — never by hand (tools/qa/men-check.ts is red on a difference). A man's columns, as the game reads and writes them.
import type { Vec3 } from '../../math/vec';
import type { Guard } from '../../game/game';   // (a type alone: nothing of the game is loaded from here)
import type { MenStore } from './store';

/** the hash of the layout; the module holds the same one (rust/sim/src/men.rs SCHEMA), and store.ts holds the two to each other as a game starts */
export const MEN_SCHEMA = 0xb8124aa1;
export const MEN_HEAD = 16;
export const MEN_F64 = ["posX","posY","posZ","velX","velY","velZ","bodyYaw","aimYaw","aimPitch","radius","stepPhase","speed","lastPosX","lastPosY","lastPosZ","speedSm","hitsLeft","fireCd","noise","visibility","irradiance","awareness","lastSeenT","sightPosX","sightPosY","sightPosZ","sightVelX","sightVelY","sightVelZ","sightT","reactT","heardUpTo","wp","wait","pathI","repathT","pathGoalX","pathGoalY","pathGoalZ","supV","aimTrack","id","stepTravel","stepOwed","stepAt","since","hurtAt","staggerUntil","dazzledUntil","muzzleAhead","diedAt","killerId","missedAt","muzzleClearT","muzzleEvalT","lineBarkT","lineInT","gunLostAt","torchLostAt","lightDeadUntil","doorDownUntil","reloadT","lastFireT","friendT","hitFlash","doorBashAt","doorBashLean","doorRipUntil","doorHead","doorHeadT","dragYaw","dragNoiseT","dragStuckT","equipT","pace","paceShownT","nvAmount","dazzledAtX","dazzledAtY","dazzledAtZ","ledT","fixT","fixRefX","fixRefY","fixRefZ","searchT","smokeTrans","sawBodyT","alertBest","alertStallT","alertRefX","alertRefY","alertRefZ","chaseT","shotBlockedT","leaderSeenT","leaderSeenPosX","leaderSeenPosY","leaderSeenPosZ","leaderStillT","bodyVisits","bodyVisitT","bodyFan","sweep","beamDirX","beamDirY","beamDirZ","lookPhase","routeI","followT","followGoalX","followGoalY","followGoalZ","stuckT","stuckN","stallRefX","stallRefY","stallRefZ","stallT","lineSide","lineGain","lineStepM","lineStuckT","pieLastT","piePrevX","piePrevY","piePrevZ","piePrevT","canisters","flashbangs","frags","shovedT","kitOwed","codeT","handWait","easeStallT","easeBest","camSeq","supKick","supSrcX","supSrcY","supSrcZ","supSrcId","supSrcT","supLastAdd","supFlinchAt","supPinnedAt","supBand","supDownT","supPinnedT","supCoverX","supCoverY","supCoverZ","supDisplacedAt","supDisplaces","supCoverUntil","supBarkT","supPendingT","aimLastBearing","aimBearingRate","aimSpeedSm","aimMoveSm","aimLeftT","aimLeftTrack","fallAhead"] as const;
export const MEN_U32 = ["gen","truths0","state","team","foe","slot","owner","rank","reason","downKind","downBy","truths1","leader","throwKind","heldKind","bigKit"] as const;
/** the generation column, as an index into the words from the part's start (the store's own) */
export const MEN_GEN = 21124;
const STATE_WORDS = ["patrol","suspicious","alert","search","dead"] as const;
const TEAM_WORDS = ["guard","operative","hostile","player","civilian"] as const;
const SLOT_WORDS = ["pistol","rifle","smoke","flash","frag"] as const;
const DOWNKIND_WORDS = [null,"shot","struck","choked","wounded"] as const;
const DOWNBY_WORDS = [null,"player","guard","world"] as const;
const THROWKIND_WORDS = ["smoke","flash","frag"] as const;
const HELDKIND_WORDS = ["smoke","flash","frag"] as const;
const BIGKIT_WORDS = ["none","shotgun","hammer"] as const;

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
  get alive(): boolean { const S = this.S; return (S.u[S.b4 + 21188 + this.r] & 1) !== 0; } set alive(v: boolean) { const S = this.S; const w = S.u[S.b4 + 21188 + this.r]; S.u[S.b4 + 21188 + this.r] = v ? w | 1 : w & 4294967294; }
  /** What he carries and its state (weapons.ts: the row from his route — the 9 mm service pistol unless the level says otherwise — with its chamber and magazine; guards.ts fires it and reads its cadence, fireWeapon its report / flash / spread, killGuard what falls out of the hand). fireCd: seconds until the trigger will go again (the AI's beat: cadence, the reaction on going alert, the shield's floor). */
  get fireCd(): number { const S = this.S; return S.d[S.b8 + 1090 + this.r]; } set fireCd(v: number) { const S = this.S; S.d[S.b8 + 1090 + this.r] = v; }
  /** an AI target's light meter (guards.ts unitLightMeter — only ever fed for a man some other AI side can be hostile to, i.e. never for staff): the renderer's irradiance at his chest / head when the probe answers (the browser), the CPU estimate of the same direct light otherwise (headless, or the first frames), mapped and eased exactly as Sam's meter is (Game.visibilityOf, damp 6). What a hostile AI e */
  get visibility(): number { const S = this.S; return S.d[S.b8 + 1218 + this.r]; } set visibility(v: number) { const S = this.S; S.d[S.b8 + 1218 + this.r] = v; }
  /** with visibility */
  get irradiance(): number { const S = this.S; return S.d[S.b8 + 1282 + this.r]; } set irradiance(v: number) { const S = this.S; S.d[S.b8 + 1282 + this.r] = v; }
  get id(): number { const S = this.S; return S.d[S.b8 + 2626 + this.r]; } set id(v: number) { const S = this.S; S.d[S.b8 + 2626 + this.r] = v; }
  /** which rule has him (0: nobody) */
  get owner(): number { const S = this.S; return S.u[S.b4 + 21508 + this.r]; } set owner(v: number) { const S = this.S; S.u[S.b4 + 21508 + this.r] = v; }
  /** how hard that rule holds him against another */
  get rank(): number { const S = this.S; return S.u[S.b4 + 21572 + this.r]; } set rank(v: number) { const S = this.S; S.u[S.b4 + 21572 + this.r] = v; }
  /** why it has him */
  get reason(): number { const S = this.S; return S.u[S.b4 + 21636 + this.r]; } set reason(v: number) { const S = this.S; S.u[S.b4 + 21636 + this.r] = v; }
  /** the game time it took him */
  get since(): number { const S = this.S; return S.d[S.b8 + 2882 + this.r]; } set since(v: number) { const S = this.S; S.d[S.b8 + 2882 + this.r] = v; }
  /** game time a blast left him hurt (condition.ts: -1 = never; read through isHurt) */
  get hurtAt(): number { const S = this.S; return S.d[S.b8 + 2946 + this.r]; } set hurtAt(v: number) { const S = this.S; S.d[S.b8 + 2946 + this.r] = v; }
  /** a ring airfoil took him in the body (launcher.ts staggerGuard): doubled over till this time — rooted like the dazzle (reactT), no round from him, but he can SEE, and he comes out of it alert toward the side it came from */
  get staggerUntil(): number { const S = this.S; return S.d[S.b8 + 3010 + this.r]; } set staggerUntil(v: number) { const S = this.S; S.d[S.b8 + 3010 + this.r] = v; }
  /** stun canister with line of sight: no vision, no shooting, rooted to the spot until this time (see Game.detonateStun) */
  get dazzledUntil(): number { const S = this.S; return S.d[S.b8 + 3074 + this.r]; } set dazzledUntil(v: number) { const S = this.S; S.d[S.b8 + 3074 + this.r] = v; }
  /** metres along gunDir from the wrist to the muzzle: the row's (weapons.ts WeaponDef.muzzleAhead — 0.26 for a pistol; whoever hands him a weapon writes it — or, for a man dealt the carbine over his sidearm row (game.ts, tune.carbineGuards), rigProps.carbineMuzzleAhead's 0.5 off the drawn prop). The round, the flash, the laser and armThrough all start at `muzzle`. */
  get muzzleAhead(): number { const S = this.S; return S.d[S.b8 + 3138 + this.r]; } set muzzleAhead(v: number) { const S = this.S; S.d[S.b8 + 3138 + this.r] = v; }
  /** where the charge that blinded him went off (the motion pass turns him away from it — character.ts MOVES; not saved) */
  get dazzledAt(): Vec3 | null { const S = this.S; if ((S.u[S.b4 + 21828 + this.r] & 1024) === 0) return null; const i = S.b8 + this.r; return [S.d[i + 4930], S.d[i + 4994], S.d[i + 5058]]; }
  set dazzledAt(v: Readonly<Vec3> | null) { const S = this.S, w = S.u[S.b4 + 21828 + this.r], i = S.b8 + this.r; if (v === null) { S.u[S.b4 + 21828 + this.r] = w & 4294966271; S.d[i + 4930] = 0; S.d[i + 4994] = 0; S.d[i + 5058] = 0; return; } S.u[S.b4 + 21828 + this.r] = w | 1024; S.d[i + 4930] = v[0]; S.d[i + 4994] = v[1]; S.d[i + 5058] = v[2]; }
  get dazzledAtX(): number { const S = this.S; return S.d[S.b8 + 4930 + this.r]; } set dazzledAtX(v: number) { const S = this.S; S.d[S.b8 + 4930 + this.r] = v; }
  get dazzledAtY(): number { const S = this.S; return S.d[S.b8 + 4994 + this.r]; } set dazzledAtY(v: number) { const S = this.S; S.d[S.b8 + 4994 + this.r] = v; }
  get dazzledAtZ(): number { const S = this.S; return S.d[S.b8 + 5058 + this.r]; } set dazzledAtZ(v: number) { const S = this.S; S.d[S.b8 + 5058 + this.r] = v; }
  /** …where whoever put him down stood (the fall leads him AWAY from there; null = along the shove's own line) · no room that way: the fall keeps to where he stands. Read and cleared with fallAs. */
  get fallTight(): boolean { const S = this.S; return (S.u[S.b4 + 21828 + this.r] & 536870912) !== 0; } set fallTight(v: boolean) { const S = this.S; const w = S.u[S.b4 + 21828 + this.r]; S.u[S.b4 + 21828 + this.r] = v ? w | 536870912 : w & 3758096383; }
  /** m the 'sink' fall lays his pelvis further along his facing (HOLD.layAhead; takedown.ts setFall) */
  get fallAhead(): number { const S = this.S; return S.d[S.b8 + 10498 + this.r]; } set fallAhead(v: number) { const S = this.S; S.d[S.b8 + 10498 + this.r] = v; }
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
  get state(): (typeof STATE_WORDS)[number] { const S = this.S; return STATE_WORDS[S.u[S.b4 + 21252 + this.r]]; } set state(v: (typeof STATE_WORDS)[number]) { const S = this.S, i = STATE_WORDS.indexOf(v); if (i < 0) throw new Error(`state: no such word: ${v}`); S.u[S.b4 + 21252 + this.r] = i; }
  /** muzzle discipline (guards.ts muzzleCheck): a living colleague stands in his line this frame, so the pistol is at the low ready and no round leaves it; clearT = seconds the line has been clear again (it comes back up after a beat, not on the frame); evalT = the game time it was last worked out (the alert branch does it early, before the shot); lineBarkT = when he (or the man in his line) last said  */
  get muzzleDown(): boolean { const S = this.S; return (S.u[S.b4 + 21188 + this.r] & 128) !== 0; } set muzzleDown(v: boolean) { const S = this.S; const w = S.u[S.b4 + 21188 + this.r]; S.u[S.b4 + 21188 + this.r] = v ? w | 128 : w & 4294967167; }
  /** alarm escalation (guards.ts updateEscalation): drawn = the sidearm is out on patrol (level ≥ 1) — handProps / attachFlashlight / killGuard read it where they used to read 'patrol ⇒ torch'; leader = lockdown pairing: trail this man while calm (followT: re-plan clock, ≤ 2 path plans a second; followGoal: his station behind him); post = lockdown: hold this corridor junction instead of walking the rou */
  get drawn(): boolean { const S = this.S; return (S.u[S.b4 + 21188 + this.r] & 256) !== 0; } set drawn(v: boolean) { const S = this.S; const w = S.u[S.b4 + 21188 + this.r]; S.u[S.b4 + 21188 + this.r] = v ? w | 256 : w & 4294967039; }
  /** SIDES (teams.ts): which side he is on — 'guard' for every man the level's routes stand up (hostile to Sam and nobody else: the game as it was); the showcase spawns 'operative' / 'hostile' units (guards.ts spawnUnit). Read wherever "the player" used to be the only possible enemy and "every other guard" the only possible colleague. */
  get team(): (typeof TEAM_WORDS)[number] { const S = this.S; return TEAM_WORDS[S.u[S.b4 + 21316 + this.r]]; } set team(v: (typeof TEAM_WORDS)[number]) { const S = this.S, i = TEAM_WORDS.indexOf(v); if (i < 0) throw new Error(`team: no such word: ${v}`); S.u[S.b4 + 21316 + this.r] = i; }
  get awareness(): number { const S = this.S; return S.d[S.b8 + 1346 + this.r]; } set awareness(v: number) { const S = this.S; S.d[S.b8 + 1346 + this.r] = v; }
  get lastSeenT(): number { const S = this.S; return S.d[S.b8 + 1410 + this.r]; } set lastSeenT(v: number) { const S = this.S; S.d[S.b8 + 1410 + this.r] = v; }
  get sawPlayerThisFrame(): boolean { const S = this.S; return (S.u[S.b4 + 21188 + this.r] & 512) !== 0; } set sawPlayerThisFrame(v: boolean) { const S = this.S; const w = S.u[S.b4 + 21188 + this.r]; S.u[S.b4 + 21188 + this.r] = v ? w | 512 : w & 4294966783; }
  /** a real sighting of his FOE this frame, whoever that is (teams) — sawPlayerThisFrame stays what its name says: eyes on Sam (what the mission's 'spotted' count and the witnesses read), which for the staff is the same thing */
  get sawFoeThisFrame(): boolean { const S = this.S; return (S.u[S.b4 + 21188 + this.r] & 1024) !== 0; } set sawFoeThisFrame(v: boolean) { const S = this.S; const w = S.u[S.b4 + 21188 + this.r]; S.u[S.b4 + 21188 + this.r] = v ? w | 1024 : w & 4294966271; }
  /** where and how fast Sam was going the last frame this man had him in view (even faintly), when that was, and the sighting whose loss has already been led (guards.ts leadFix: the fix he is left with runs on the way Sam was going, once, the frame he loses him) */
  get sightPos(): Vec3 { const S = this.S, i = S.b8 + this.r; return [S.d[i + 1474], S.d[i + 1538], S.d[i + 1602]]; }
  set sightPos(v: Readonly<Vec3>) { const S = this.S, i = S.b8 + this.r; S.d[i + 1474] = v[0]; S.d[i + 1538] = v[1]; S.d[i + 1602] = v[2]; }
  get sightPosX(): number { const S = this.S; return S.d[S.b8 + 1474 + this.r]; } set sightPosX(v: number) { const S = this.S; S.d[S.b8 + 1474 + this.r] = v; }
  get sightPosY(): number { const S = this.S; return S.d[S.b8 + 1538 + this.r]; } set sightPosY(v: number) { const S = this.S; S.d[S.b8 + 1538 + this.r] = v; }
  get sightPosZ(): number { const S = this.S; return S.d[S.b8 + 1602 + this.r]; } set sightPosZ(v: number) { const S = this.S; S.d[S.b8 + 1602 + this.r] = v; }
  /** with sightPos */
  get sightVel(): Vec3 { const S = this.S, i = S.b8 + this.r; return [S.d[i + 1666], S.d[i + 1730], S.d[i + 1794]]; }
  set sightVel(v: Readonly<Vec3>) { const S = this.S, i = S.b8 + this.r; S.d[i + 1666] = v[0]; S.d[i + 1730] = v[1]; S.d[i + 1794] = v[2]; }
  get sightVelX(): number { const S = this.S; return S.d[S.b8 + 1666 + this.r]; } set sightVelX(v: number) { const S = this.S; S.d[S.b8 + 1666 + this.r] = v; }
  get sightVelY(): number { const S = this.S; return S.d[S.b8 + 1730 + this.r]; } set sightVelY(v: number) { const S = this.S; S.d[S.b8 + 1730 + this.r] = v; }
  get sightVelZ(): number { const S = this.S; return S.d[S.b8 + 1794 + this.r]; } set sightVelZ(v: number) { const S = this.S; S.d[S.b8 + 1794 + this.r] = v; }
  /** with sightPos */
  get sightT(): number { const S = this.S; return S.d[S.b8 + 1858 + this.r]; } set sightT(v: number) { const S = this.S; S.d[S.b8 + 1858 + this.r] = v; }
  get reactT(): number { const S = this.S; return S.d[S.b8 + 1922 + this.r]; } set reactT(v: number) { const S = this.S; S.d[S.b8 + 1922 + this.r] = v; }
  get heardUpTo(): number { const S = this.S; return S.d[S.b8 + 1986 + this.r]; } set heardUpTo(v: number) { const S = this.S; S.d[S.b8 + 1986 + this.r] = v; }
  get wp(): number { const S = this.S; return S.d[S.b8 + 2050 + this.r]; } set wp(v: number) { const S = this.S; S.d[S.b8 + 2050 + this.r] = v; }
  get wait(): number { const S = this.S; return S.d[S.b8 + 2114 + this.r]; } set wait(v: number) { const S = this.S; S.d[S.b8 + 2114 + this.r] = v; }
  get pathI(): number { const S = this.S; return S.d[S.b8 + 2178 + this.r]; } set pathI(v: number) { const S = this.S; S.d[S.b8 + 2178 + this.r] = v; }
  get repathT(): number { const S = this.S; return S.d[S.b8 + 2242 + this.r]; } set repathT(v: number) { const S = this.S; S.d[S.b8 + 2242 + this.r] = v; }
  get pathGoal(): Vec3 | null { const S = this.S; if ((S.u[S.b4 + 21188 + this.r] & 2048) === 0) return null; const i = S.b8 + this.r; return [S.d[i + 2306], S.d[i + 2370], S.d[i + 2434]]; }
  set pathGoal(v: Readonly<Vec3> | null) { const S = this.S, w = S.u[S.b4 + 21188 + this.r], i = S.b8 + this.r; if (v === null) { S.u[S.b4 + 21188 + this.r] = w & 4294965247; S.d[i + 2306] = 0; S.d[i + 2370] = 0; S.d[i + 2434] = 0; return; } S.u[S.b4 + 21188 + this.r] = w | 2048; S.d[i + 2306] = v[0]; S.d[i + 2370] = v[1]; S.d[i + 2434] = v[2]; }
  get pathGoalX(): number { const S = this.S; return S.d[S.b8 + 2306 + this.r]; } set pathGoalX(v: number) { const S = this.S; S.d[S.b8 + 2306 + this.r] = v; }
  get pathGoalY(): number { const S = this.S; return S.d[S.b8 + 2370 + this.r]; } set pathGoalY(v: number) { const S = this.S; S.d[S.b8 + 2370 + this.r] = v; }
  get pathGoalZ(): number { const S = this.S; return S.d[S.b8 + 2434 + this.r]; } set pathGoalZ(v: number) { const S = this.S; S.d[S.b8 + 2434 + this.r] = v; }
  /** his foe right now (see Foe): 'player' from birth for the guard side (never anything else — no unit is hostile to staff), chosen each frame by salience among the hostile characters in his sight for the other sides (guards.ts acquireFoe), null = nobody acquired yet / his man is dead and no other shows */
  get foe(): 'player' | Guard | null { const S = this.S, h = S.u[S.b4 + 21380 + this.r]; return h === 0 ? null : h === 1 ? 'player' : S.manAt(h) as Guard; } set foe(v: 'player' | Guard | null) { const S = this.S; S.u[S.b4 + 21380 + this.r] = v === null ? 0 : v === 'player' ? 1 : S.handleOf((v as Guard).r); }
  /** scripted (showcase tour): stand your ground while on patrol / merely suspicious (look, don't walk over) */
  get hold(): boolean { const S = this.S; return (S.u[S.b4 + 21188 + this.r] & 4096) !== 0; } set hold(v: boolean) { const S = this.S; const w = S.u[S.b4 + 21188 + this.r]; S.u[S.b4 + 21188 + this.r] = v ? w | 4096 : w & 4294963199; }
  /** scripted: in a fight, keep the position you were given (shoot from it, cover the last fix from it) instead of closing in */
  get pinned(): boolean { const S = this.S; return (S.u[S.b4 + 21188 + this.r] & 8192) !== 0; } set pinned(v: boolean) { const S = this.S; const w = S.u[S.b4 + 21188 + this.r]; S.u[S.b4 + 21188 + this.r] = v ? w | 8192 : w & 4294959103; }
  get diedAt(): number { const S = this.S; return S.d[S.b8 + 3202 + this.r]; } set diedAt(v: number) { const S = this.S; S.d[S.b8 + 3202 + this.r] = v; }
  /** (dead) how he went down and by whose hand — set once by combat.ts killGuard; null while he lives. isBreathing() reads it: struck / choked = out cold. */
  get downKind(): (typeof DOWNKIND_WORDS)[number] { const S = this.S; return DOWNKIND_WORDS[S.u[S.b4 + 21700 + this.r]]; } set downKind(v: (typeof DOWNKIND_WORDS)[number]) { const S = this.S, i = DOWNKIND_WORDS.indexOf(v); if (i < 0) throw new Error(`downKind: no such word: ${v}`); S.u[S.b4 + 21700 + this.r] = i; }
  /** with downKind */
  get downBy(): (typeof DOWNBY_WORDS)[number] { const S = this.S; return DOWNBY_WORDS[S.u[S.b4 + 21764 + this.r]]; } set downBy(v: (typeof DOWNBY_WORDS)[number]) { const S = this.S, i = DOWNBY_WORDS.indexOf(v); if (i < 0) throw new Error(`downBy: no such word: ${v}`); S.u[S.b4 + 21764 + this.r] = i; }
  /** (dead) the id of the character whose round put him down, −1 = not by a round / unknown (combat.ts fireWeapon stamps it; the engagement record names him) */
  get killerId(): number { const S = this.S; return S.d[S.b8 + 3266 + this.r]; } set killerId(v: number) { const S = this.S; S.d[S.b8 + 3266 + this.r] = v; }
  /** (dead) somebody has discovered this body — counted once for the debrief */
  get found(): boolean { const S = this.S; return (S.u[S.b4 + 21828 + this.r] & 1) !== 0; } set found(v: boolean) { const S = this.S; const w = S.u[S.b4 + 21828 + this.r]; S.u[S.b4 + 21828 + this.r] = v ? w | 1 : w & 4294967294; }
  /** (dead, not found) the net has called this man and got nothing back — when (guards.ts updateRollcall; -1 = never): from then on the others count him as missing (superviseLockdown stops dealing his corpse a role) — and whether the floor has already been raised once on his account (it is, once, when the man sent round his route comes back with nothing) */
  get missedAt(): number { const S = this.S; return S.d[S.b8 + 3330 + this.r]; } set missedAt(v: number) { const S = this.S; S.d[S.b8 + 3330 + this.r] = v; }
  /** with missedAt */
  get missingRaised(): boolean { const S = this.S; return (S.u[S.b4 + 21828 + this.r] & 2) !== 0; } set missingRaised(v: boolean) { const S = this.S; const w = S.u[S.b4 + 21828 + this.r]; S.u[S.b4 + 21828 + this.r] = v ? w | 2 : w & 4294967293; }
  /** with muzzleDown */
  get muzzleClearT(): number { const S = this.S; return S.d[S.b8 + 3394 + this.r]; } set muzzleClearT(v: number) { const S = this.S; S.d[S.b8 + 3394 + this.r] = v; }
  /** with muzzleDown */
  get muzzleEvalT(): number { const S = this.S; return S.d[S.b8 + 3458 + this.r]; } set muzzleEvalT(v: number) { const S = this.S; S.d[S.b8 + 3458 + this.r] = v; }
  /** with muzzleDown */
  get lineBarkT(): number { const S = this.S; return S.d[S.b8 + 3522 + this.r]; } set lineBarkT(v: number) { const S = this.S; S.d[S.b8 + 3522 + this.r] = v; }
  /** alert, licensed to shoot: seconds a colleague has stood in the line of the shot itself (a man crossing is waited out for a beat; one standing there is stepped round) */
  get lineInT(): number { const S = this.S; return S.d[S.b8 + 3586 + this.r]; } set lineInT(v: number) { const S = this.S; S.d[S.b8 + 3586 + this.r] = v; }
  /** a LIVING man whose gun was knocked from his hand at the grab (HOLD.dropGun): dropped = that gun, this = when — his hands are empty (guards.ts inHand) and its light dies on the floor; −1 again once he has it back (player.ts handBack) */
  get gunLostAt(): number { const S = this.S; return S.d[S.b8 + 3650 + this.r]; } set gunLostAt(v: number) { const S = this.S; S.d[S.b8 + 3650 + this.r] = v; }
  /** a LIVING man whose torch was knocked from his hand (the grab): dropped = that torch, this = when — the beam follows it down and sputters out (guards.ts updateFlashlight); on patrol afterwards his hand is empty */
  get torchLostAt(): number { const S = this.S; return S.d[S.b8 + 3714 + this.r]; } set torchLostAt(v: number) { const S = this.S; S.d[S.b8 + 3714 + this.r] = v; }
  get lightDeadUntil(): number { const S = this.S; return S.d[S.b8 + 3778 + this.r]; } set lightDeadUntil(v: number) { const S = this.S; S.d[S.b8 + 3778 + this.r] = v; }
  /** was his light burning the moment it left his hand (killed / grabbed)? a dark weapon light must not sputter ON as it tumbles (review 08-10 #7: the breach hostiles' unlit lights swept the sunlit room the frame each died) */
  get lightLitAtLoss(): boolean { const S = this.S; return (S.u[S.b4 + 21828 + this.r] & 4) !== 0; } set lightLitAtLoss(v: boolean) { const S = this.S; const w = S.u[S.b4 + 21828 + this.r]; S.u[S.b4 + 21828 + this.r] = v ? w | 4 : w & 4294967291; }
  /** a booted door met him where he stood right at it (doorthrow.ts): on a knee till this time, inside the fold the same blow gave him */
  get doorDownUntil(): number { const S = this.S; return S.d[S.b8 + 3842 + this.r]; } set doorDownUntil(v: number) { const S = this.S; S.d[S.b8 + 3842 + this.r] = v; }
  /** a trained man (the response team, game/qrf.ts): the aim-settle model lines him up quicker and his hands are steadier (aim.ts TRAINED — × the settle time, × his row's cone), and qrf.ts keeps his reaction beats short. False for the night staff and the showcase's men: their numbers are untouched. */
  get trained(): boolean { const S = this.S; return (S.u[S.b4 + 21828 + this.r] & 8) !== 0; } set trained(v: boolean) { const S = this.S; const w = S.u[S.b4 + 21828 + this.r]; S.u[S.b4 + 21828 + this.r] = v ? w | 8 : w & 4294967287; }
  /** scripted, SHOWCASE CAST ONLY (director.ts LoadoutSpec.god — the day showcase's two operatives): a round that would have found him passes through him as it passes a colleague (combat.ts fireWeapon). The show's doctrine keeps the opposition's rounds off them by construction (laid on their cover, traced: tools/qa/day-probes.ts); this is the net under a stray, never the mechanism, and nothing the play */
  get god(): boolean { const S = this.S; return (S.u[S.b4 + 21828 + this.r] & 16) !== 0; } set god(v: boolean) { const S = this.S; const w = S.u[S.b4 + 21828 + this.r]; S.u[S.b4 + 21828 + this.r] = v ? w | 16 : w & 4294967279; }
  /** rifles (docs/internal/sc20k-design.md §3): dealt the carbine by the sandbox knob (tune.carbineGuards, written by Game.update each frame) — and the deal IS the weapon: while it stands, `weapon` is the AKS-74U row (bursts, the 30-round box, the harder report, the rifle that drops — guards.ts burstTrigger, combat.ts), the RIG in his hands the carbine and his poses the long gun's; dealt away again, hi */
  get carbine(): boolean { const S = this.S; return (S.u[S.b4 + 21828 + this.r] & 32) !== 0; } set carbine(v: boolean) { const S = this.S; const w = S.u[S.b4 + 21828 + this.r]; S.u[S.b4 + 21828 + this.r] = v ? w | 32 : w & 4294967263; }
  /** with fireCd */
  get reloadT(): number { const S = this.S; return S.d[S.b8 + 3906 + this.r]; } set reloadT(v: number) { const S = this.S; S.d[S.b8 + 3906 + this.r] = v; }
  /** with sightPos */
  get ledT(): number { const S = this.S; return S.d[S.b8 + 5122 + this.r]; } set ledT(v: number) { const S = this.S; S.d[S.b8 + 5122 + this.r] = v; }
  /** when the fix he is working from was PLACED (guards.ts updateGuard stamps it whenever lastKnown jumps): the search reasons about how far Sam can have got since then */
  get fixT(): number { const S = this.S; return S.d[S.b8 + 5186 + this.r]; } set fixT(v: number) { const S = this.S; S.d[S.b8 + 5186 + this.r] = v; }
  /** with fixT */
  get fixRef(): Vec3 | null { const S = this.S; if ((S.u[S.b4 + 21828 + this.r] & 2048) === 0) return null; const i = S.b8 + this.r; return [S.d[i + 5250], S.d[i + 5314], S.d[i + 5378]]; }
  set fixRef(v: Readonly<Vec3> | null) { const S = this.S, w = S.u[S.b4 + 21828 + this.r], i = S.b8 + this.r; if (v === null) { S.u[S.b4 + 21828 + this.r] = w & 4294965247; S.d[i + 5250] = 0; S.d[i + 5314] = 0; S.d[i + 5378] = 0; return; } S.u[S.b4 + 21828 + this.r] = w | 2048; S.d[i + 5250] = v[0]; S.d[i + 5314] = v[1]; S.d[i + 5378] = v[2]; }
  get fixRefX(): number { const S = this.S; return S.d[S.b8 + 5250 + this.r]; } set fixRefX(v: number) { const S = this.S; S.d[S.b8 + 5250 + this.r] = v; }
  get fixRefY(): number { const S = this.S; return S.d[S.b8 + 5314 + this.r]; } set fixRefY(v: number) { const S = this.S; S.d[S.b8 + 5314 + this.r] = v; }
  get fixRefZ(): number { const S = this.S; return S.d[S.b8 + 5378 + this.r]; } set fixRefZ(v: number) { const S = this.S; S.d[S.b8 + 5378 + this.r] = v; }
  get searchT(): number { const S = this.S; return S.d[S.b8 + 5442 + this.r]; } set searchT(v: number) { const S = this.S; S.d[S.b8 + 5442 + this.r] = v; }
  get smokeTrans(): number { const S = this.S; return S.d[S.b8 + 5506 + this.r]; } set smokeTrans(v: number) { const S = this.S; S.d[S.b8 + 5506 + this.r] = v; }
  /** game time he last discovered a body: a short refractory before he can call in another (two lying together are one 'man down', not two in two frames) */
  get sawBodyT(): number { const S = this.S; return S.d[S.b8 + 5570 + this.r]; } set sawBodyT(v: number) { const S = this.S; S.d[S.b8 + 5570 + this.r] = v; }
  /** alert with a fix: nearest he has got to it, how long without getting nearer, the fix that was measured against (a no-progress exit into 'search', guards.ts — re-based when the fix moves), and the seconds this chase has run without eyes on Sam (its hard backstop) */
  get alertBest(): number { const S = this.S; return S.d[S.b8 + 5634 + this.r]; } set alertBest(v: number) { const S = this.S; S.d[S.b8 + 5634 + this.r] = v; }
  /** with alertBest */
  get alertStallT(): number { const S = this.S; return S.d[S.b8 + 5698 + this.r]; } set alertStallT(v: number) { const S = this.S; S.d[S.b8 + 5698 + this.r] = v; }
  /** with alertBest */
  get alertRef(): Vec3 | null { const S = this.S; if ((S.u[S.b4 + 21828 + this.r] & 4096) === 0) return null; const i = S.b8 + this.r; return [S.d[i + 5762], S.d[i + 5826], S.d[i + 5890]]; }
  set alertRef(v: Readonly<Vec3> | null) { const S = this.S, w = S.u[S.b4 + 21828 + this.r], i = S.b8 + this.r; if (v === null) { S.u[S.b4 + 21828 + this.r] = w & 4294963199; S.d[i + 5762] = 0; S.d[i + 5826] = 0; S.d[i + 5890] = 0; return; } S.u[S.b4 + 21828 + this.r] = w | 4096; S.d[i + 5762] = v[0]; S.d[i + 5826] = v[1]; S.d[i + 5890] = v[2]; }
  get alertRefX(): number { const S = this.S; return S.d[S.b8 + 5762 + this.r]; } set alertRefX(v: number) { const S = this.S; S.d[S.b8 + 5762 + this.r] = v; }
  get alertRefY(): number { const S = this.S; return S.d[S.b8 + 5826 + this.r]; } set alertRefY(v: number) { const S = this.S; S.d[S.b8 + 5826 + this.r] = v; }
  get alertRefZ(): number { const S = this.S; return S.d[S.b8 + 5890 + this.r]; } set alertRefZ(v: number) { const S = this.S; S.d[S.b8 + 5890 + this.r] = v; }
  /** with alertBest */
  get chaseT(): number { const S = this.S; return S.d[S.b8 + 5954 + this.r]; } set chaseT(v: number) { const S = this.S; S.d[S.b8 + 5954 + this.r] = v; }
  /** alert with eyes on the player: seconds his line of FIRE (not sight) has been stopped short by something solid — a jamb on his gun-hand side, a leaf, the desk Sam is behind (guards.ts: he closes along the path instead of emptying magazines into the frame) */
  get shotBlockedT(): number { const S = this.S; return S.d[S.b8 + 6018 + this.r]; } set shotBlockedT(v: number) { const S = this.S; S.d[S.b8 + 6018 + this.r] = v; }
  /** lockdown follower: when he last had his leader in plain sight (no wall between, inside 16 m) and where the man was then — twenty seconds without, and the man nowhere near that spot, and he reports him missing (guards.ts partnerWatch) */
  get leaderSeenT(): number { const S = this.S; return S.d[S.b8 + 6082 + this.r]; } set leaderSeenT(v: number) { const S = this.S; S.d[S.b8 + 6082 + this.r] = v; }
  /** with leaderSeenT */
  get leaderSeenPos(): Vec3 { const S = this.S, i = S.b8 + this.r; return [S.d[i + 6146], S.d[i + 6210], S.d[i + 6274]]; }
  set leaderSeenPos(v: Readonly<Vec3>) { const S = this.S, i = S.b8 + this.r; S.d[i + 6146] = v[0]; S.d[i + 6210] = v[1]; S.d[i + 6274] = v[2]; }
  get leaderSeenPosX(): number { const S = this.S; return S.d[S.b8 + 6146 + this.r]; } set leaderSeenPosX(v: number) { const S = this.S; S.d[S.b8 + 6146 + this.r] = v; }
  get leaderSeenPosY(): number { const S = this.S; return S.d[S.b8 + 6210 + this.r]; } set leaderSeenPosY(v: number) { const S = this.S; S.d[S.b8 + 6210 + this.r] = v; }
  get leaderSeenPosZ(): number { const S = this.S; return S.d[S.b8 + 6274 + this.r]; } set leaderSeenPosZ(v: number) { const S = this.S; S.d[S.b8 + 6274 + this.r] = v; }
  /** lockdown follower: seconds the man he trails has not moved (planted beside a leader who never moves again, he turns to look at him — guards.ts followLeader) */
  get leaderStillT(): number { const S = this.S; return S.d[S.b8 + 6338 + this.r]; } set leaderStillT(v: number) { const S = this.S; S.d[S.b8 + 6338 + this.r] = v; }
  /** (dead, found) the aftermath script's bookkeeping ON the body (guards.ts bodyAftermath): how many living men have reached it (1 = the finder covering it, 2 = the one who says '…christ') and when the last did, whether that second-man line and the one closing line have been said, and a counter dealing the directions they widen out along */
  get bodyVisits(): number { const S = this.S; return S.d[S.b8 + 6402 + this.r]; } set bodyVisits(v: number) { const S = this.S; S.d[S.b8 + 6402 + this.r] = v; }
  /** with bodyVisits */
  get bodyVisitT(): number { const S = this.S; return S.d[S.b8 + 6466 + this.r]; } set bodyVisitT(v: number) { const S = this.S; S.d[S.b8 + 6466 + this.r] = v; }
  /** with bodyVisits */
  get bodyRemarked(): boolean { const S = this.S; return (S.u[S.b4 + 21828 + this.r] & 8192) !== 0; } set bodyRemarked(v: boolean) { const S = this.S; const w = S.u[S.b4 + 21828 + this.r]; S.u[S.b4 + 21828 + this.r] = v ? w | 8192 : w & 4294959103; }
  /** with bodyVisits */
  get bodyClosed(): boolean { const S = this.S; return (S.u[S.b4 + 21828 + this.r] & 16384) !== 0; } set bodyClosed(v: boolean) { const S = this.S; const w = S.u[S.b4 + 21828 + this.r]; S.u[S.b4 + 21828 + this.r] = v ? w | 16384 : w & 4294950911; }
  /** with bodyVisits */
  get bodyFan(): number { const S = this.S; return S.d[S.b8 + 6530 + this.r]; } set bodyFan(v: number) { const S = this.S; S.d[S.b8 + 6530 + this.r] = v; }
  get sweep(): number { const S = this.S; return S.d[S.b8 + 6594 + this.r]; } set sweep(v: number) { const S = this.S; S.d[S.b8 + 6594 + this.r] = v; }
  get beamDir(): Vec3 { const S = this.S, i = S.b8 + this.r; return [S.d[i + 6658], S.d[i + 6722], S.d[i + 6786]]; }
  set beamDir(v: Readonly<Vec3>) { const S = this.S, i = S.b8 + this.r; S.d[i + 6658] = v[0]; S.d[i + 6722] = v[1]; S.d[i + 6786] = v[2]; }
  get beamDirX(): number { const S = this.S; return S.d[S.b8 + 6658 + this.r]; } set beamDirX(v: number) { const S = this.S; S.d[S.b8 + 6658 + this.r] = v; }
  get beamDirY(): number { const S = this.S; return S.d[S.b8 + 6722 + this.r]; } set beamDirY(v: number) { const S = this.S; S.d[S.b8 + 6722 + this.r] = v; }
  get beamDirZ(): number { const S = this.S; return S.d[S.b8 + 6786 + this.r]; } set beamDirZ(v: number) { const S = this.S; S.d[S.b8 + 6786 + this.r] = v; }
  /** where his idle looking stands: drawn off the die as he is made */
  get lookPhase(): number { const S = this.S; return S.d[S.b8 + 6850 + this.r]; } set lookPhase(v: number) { const S = this.S; S.d[S.b8 + 6850 + this.r] = v; }
  get routeI(): number { const S = this.S; return S.d[S.b8 + 6914 + this.r]; } set routeI(v: number) { const S = this.S; S.d[S.b8 + 6914 + this.r] = v; }
  /** lockdown pairing: the man he trails while calm */
  get leader(): Guard | null { const S = this.S, h = S.u[S.b4 + 21892 + this.r]; return h === 0 ? null : S.manAt(h) as Guard; } set leader(v: Guard | null) { const S = this.S; S.u[S.b4 + 21892 + this.r] = v === null ? 0 : S.handleOf((v as Guard).r); }
  /** alarm escalation (guards.ts updateEscalation): drawn = the sidearm is out on patrol (level ≥ 1) — handProps / attachFlashlight / killGuard read it where they used to read 'patrol ⇒ torch'; followGoal: his station behind him); post = lockdown: hold this corridor junction instead of walking the route */
  get followT(): number { const S = this.S; return S.d[S.b8 + 6978 + this.r]; } set followT(v: number) { const S = this.S; S.d[S.b8 + 6978 + this.r] = v; }
  /** with followT */
  get followGoal(): Vec3 { const S = this.S, i = S.b8 + this.r; return [S.d[i + 7042], S.d[i + 7106], S.d[i + 7170]]; }
  set followGoal(v: Readonly<Vec3>) { const S = this.S, i = S.b8 + this.r; S.d[i + 7042] = v[0]; S.d[i + 7106] = v[1]; S.d[i + 7170] = v[2]; }
  get followGoalX(): number { const S = this.S; return S.d[S.b8 + 7042 + this.r]; } set followGoalX(v: number) { const S = this.S; S.d[S.b8 + 7042 + this.r] = v; }
  get followGoalY(): number { const S = this.S; return S.d[S.b8 + 7106 + this.r]; } set followGoalY(v: number) { const S = this.S; S.d[S.b8 + 7106 + this.r] = v; }
  get followGoalZ(): number { const S = this.S; return S.d[S.b8 + 7170 + this.r]; } set followGoalZ(v: number) { const S = this.S; S.d[S.b8 + 7170 + this.r] = v; }
  get stuckT(): number { const S = this.S; return S.d[S.b8 + 7234 + this.r]; } set stuckT(v: number) { const S = this.S; S.d[S.b8 + 7234 + this.r] = v; }
  get stuckN(): number { const S = this.S; return S.d[S.b8 + 7298 + this.r]; } set stuckN(v: number) { const S = this.S; S.d[S.b8 + 7298 + this.r] = v; }
  /** walking watchdog (guards.ts notGettingAnywhere): where he last made half a metre of ground from, and the seconds since — a suspicious / searching / body-script leg that goes nowhere is dropped where he stands */
  get stallRef(): Vec3 | null { const S = this.S; if ((S.u[S.b4 + 21828 + this.r] & 32768) === 0) return null; const i = S.b8 + this.r; return [S.d[i + 7362], S.d[i + 7426], S.d[i + 7490]]; }
  set stallRef(v: Readonly<Vec3> | null) { const S = this.S, w = S.u[S.b4 + 21828 + this.r], i = S.b8 + this.r; if (v === null) { S.u[S.b4 + 21828 + this.r] = w & 4294934527; S.d[i + 7362] = 0; S.d[i + 7426] = 0; S.d[i + 7490] = 0; return; } S.u[S.b4 + 21828 + this.r] = w | 32768; S.d[i + 7362] = v[0]; S.d[i + 7426] = v[1]; S.d[i + 7490] = v[2]; }
  get stallRefX(): number { const S = this.S; return S.d[S.b8 + 7362 + this.r]; } set stallRefX(v: number) { const S = this.S; S.d[S.b8 + 7362 + this.r] = v; }
  get stallRefY(): number { const S = this.S; return S.d[S.b8 + 7426 + this.r]; } set stallRefY(v: number) { const S = this.S; S.d[S.b8 + 7426 + this.r] = v; }
  get stallRefZ(): number { const S = this.S; return S.d[S.b8 + 7490 + this.r]; } set stallRefZ(v: number) { const S = this.S; S.d[S.b8 + 7490 + this.r] = v; }
  /** with stallRef */
  get stallT(): number { const S = this.S; return S.d[S.b8 + 7554 + this.r]; } set stallT(v: number) { const S = this.S; S.d[S.b8 + 7554 + this.r] = v; }
  /** the side-step round a colleague (guards.ts lineStep): lineSide = which way the step under way goes (−1 / +1 along the square-off-the-line basis, 0 = none); lineGain / lineStepM = how far last frame's step should have left him from its goal, and how long that step was (a step handed straight back by a chair wedged against the table, a leaf, a man standing on the spot reads as no gain: lineStuckT co */
  get lineSide(): number { const S = this.S; return S.d[S.b8 + 7618 + this.r]; } set lineSide(v: number) { const S = this.S; S.d[S.b8 + 7618 + this.r] = v; }
  /** with lineSide */
  get lineGain(): number { const S = this.S; return S.d[S.b8 + 7682 + this.r]; } set lineGain(v: number) { const S = this.S; S.d[S.b8 + 7682 + this.r] = v; }
  /** with lineSide */
  get lineStepM(): number { const S = this.S; return S.d[S.b8 + 7746 + this.r]; } set lineStepM(v: number) { const S = this.S; S.d[S.b8 + 7746 + this.r] = v; }
  /** with lineSide */
  get lineStuckT(): number { const S = this.S; return S.d[S.b8 + 7810 + this.r]; } set lineStuckT(v: number) { const S = this.S; S.d[S.b8 + 7810 + this.r] = v; }
  /** threshold discipline (guards.ts pieDoorway): the doorway his alert / search path is taking him through right now and where he is in slicing it — null when none; pieLast / pieLastT: the door he last went through that way (or any way, while hunting) and when, so the one he has just come through is not pied again; piePrev / piePrevT: where he stood the last frame that was looked at (the crossing itse */
  get pieLastT(): number { const S = this.S; return S.d[S.b8 + 7874 + this.r]; } set pieLastT(v: number) { const S = this.S; S.d[S.b8 + 7874 + this.r] = v; }
  /** with pieLastT */
  get piePrev(): Vec3 | null { const S = this.S; if ((S.u[S.b4 + 21828 + this.r] & 65536) === 0) return null; const i = S.b8 + this.r; return [S.d[i + 7938], S.d[i + 8002], S.d[i + 8066]]; }
  set piePrev(v: Readonly<Vec3> | null) { const S = this.S, w = S.u[S.b4 + 21828 + this.r], i = S.b8 + this.r; if (v === null) { S.u[S.b4 + 21828 + this.r] = w & 4294901759; S.d[i + 7938] = 0; S.d[i + 8002] = 0; S.d[i + 8066] = 0; return; } S.u[S.b4 + 21828 + this.r] = w | 65536; S.d[i + 7938] = v[0]; S.d[i + 8002] = v[1]; S.d[i + 8066] = v[2]; }
  get piePrevX(): number { const S = this.S; return S.d[S.b8 + 7938 + this.r]; } set piePrevX(v: number) { const S = this.S; S.d[S.b8 + 7938 + this.r] = v; }
  get piePrevY(): number { const S = this.S; return S.d[S.b8 + 8002 + this.r]; } set piePrevY(v: number) { const S = this.S; S.d[S.b8 + 8002 + this.r] = v; }
  get piePrevZ(): number { const S = this.S; return S.d[S.b8 + 8066 + this.r]; } set piePrevZ(v: number) { const S = this.S; S.d[S.b8 + 8066 + this.r] = v; }
  /** with pieLastT */
  get piePrevT(): number { const S = this.S; return S.d[S.b8 + 8130 + this.r]; } set piePrevT(v: number) { const S = this.S; S.d[S.b8 + 8130 + this.r] = v; }
  initGuard(): void { this.diedAt = -1; this.killerId = -1; this.missedAt = -1; this.muzzleEvalT = -1; this.lineBarkT = -100; this.gunLostAt = -1; this.torchLostAt = -1; this.lightDeadUntil = -1; this.lightLitAtLoss = true; this.doorDownUntil = -1; this.reloadT = -1; this.lastSeenT = -100; this.sightT = -100; this.ledT = -100; this.fixT = -100; this.heardUpTo = -1; this.smokeTrans = 1; this.sawBodyT = -100; this.alertBest = 1000000000; this.leaderSeenT = -1; this.bodyVisitT = -1; this.foe = "player"; this.lineGain = -1; this.pieLastT = -100; this.piePrevT = -100; }
}
/** the player's own columns */
export class PlayerCols extends ManCols {
  /** his speed, eased */
  get speedSm(): number { const S = this.S; return S.d[S.b8 + 962 + this.r]; } set speedSm(v: number) { const S = this.S; S.d[S.b8 + 962 + this.r] = v; }
  /** one 9 mm hit and you are down, same as the guards; Enter restarts the encounter */
  get down(): boolean { const S = this.S; return (S.u[S.b4 + 21188 + this.r] & 2) !== 0; } set down(v: boolean) { const S = this.S; const w = S.u[S.b4 + 21188 + this.r]; S.u[S.b4 + 21188 + this.r] = v ? w | 2 : w & 4294967293; }
  get crouch(): boolean { const S = this.S; return (S.u[S.b4 + 21188 + this.r] & 4) !== 0; } set crouch(v: boolean) { const S = this.S; const w = S.u[S.b4 + 21188 + this.r]; S.u[S.b4 + 21188 + this.r] = v ? w | 4 : w & 4294967291; }
  get sprinting(): boolean { const S = this.S; return (S.u[S.b4 + 21188 + this.r] & 8) !== 0; } set sprinting(v: boolean) { const S = this.S; const w = S.u[S.b4 + 21188 + this.r]; S.u[S.b4 + 21188 + this.r] = v ? w | 8 : w & 4294967287; }
  get backpedal(): boolean { const S = this.S; return (S.u[S.b4 + 21188 + this.r] & 16) !== 0; } set backpedal(v: boolean) { const S = this.S; const w = S.u[S.b4 + 21188 + this.r]; S.u[S.b4 + 21188 + this.r] = v ? w | 16 : w & 4294967279; }
  get aimHeld(): boolean { const S = this.S; return (S.u[S.b4 + 21188 + this.r] & 32) !== 0; } set aimHeld(v: boolean) { const S = this.S; const w = S.u[S.b4 + 21188 + this.r]; S.u[S.b4 + 21188 + this.r] = v ? w | 32 : w & 4294967263; }
  /** he starts with the pistol on the thigh (playtest call: hands free by default — Space grabs, a click behind a man is the takedown); E puts the selected item away / draws it again (Chaos Theory: equip / unequip); holstered + LMB behind a guard in reach = the silent takedown (its one and only input), the pistol's paths all check it */
  get holstered(): boolean { const S = this.S; return (S.u[S.b4 + 21188 + this.r] & 64) !== 0; } set holstered(v: boolean) { const S = this.S; const w = S.u[S.b4 + 21188 + this.r]; S.u[S.b4 + 21188 + this.r] = v ? w | 64 : w & 4294967231; }
  /** with down */
  get hitsLeft(): number { const S = this.S; return S.d[S.b8 + 1026 + this.r]; } set hitsLeft(v: number) { const S = this.S; S.d[S.b8 + 1026 + this.r] = v; }
  /** HUD sound meter: the loudest thing you did lately (hearing-model level, decays) */
  get noise(): number { const S = this.S; return S.d[S.b8 + 1154 + this.r]; } set noise(v: number) { const S = this.S; S.d[S.b8 + 1154 + this.r] = v; }
  /** Shift held in a crouch this frame (player.ts CROUCHRUN): the fast crouch walk — set where `sprinting` is, false wherever that is not reached */
  get dash(): boolean { const S = this.S; return (S.u[S.b4 + 21188 + this.r] & 16384) !== 0; } set dash(v: boolean) { const S = this.S; const w = S.u[S.b4 + 21188 + this.r]; S.u[S.b4 + 21188 + this.r] = v ? w | 16384 : w & 4294950911; }
  /** with frags */
  get slot(): (typeof SLOT_WORDS)[number] { const S = this.S; return SLOT_WORDS[S.u[S.b4 + 21444 + this.r]]; } set slot(v: (typeof SLOT_WORDS)[number]) { const S = this.S, i = SLOT_WORDS.indexOf(v); if (i < 0) throw new Error(`slot: no such word: ${v}`); S.u[S.b4 + 21444 + this.r] = i; }
  /** how far the walk cycle has come since the last footfall filed, in turns, either way added up, 1 at the most (player.ts FOOTFALL.once; 1 = far enough, which is where it starts, and where it stays while the key has never been on) */
  get stepTravel(): number { const S = this.S; return S.d[S.b8 + 2690 + this.r]; } set stepTravel(v: number) { const S = this.S; S.d[S.b8 + 2690 + this.r] = v; }
  /** how far the walk cycle has come at a footfall's speed since the last edge of it passed or footfall filed, in turns, either way added up (player.ts FOOTFALL.owed: at half a turn a footfall is owed; 0 where it starts, when he slows, after frames the footfall test was not asked, and while the key is down) */
  get stepOwed(): number { const S = this.S; return S.d[S.b8 + 2754 + this.r]; } set stepOwed(v: number) { const S = this.S; S.d[S.b8 + 2754 + this.r] = v; }
  /** the game clock the footfall test was last asked at with the key up (-1: never) */
  get stepAt(): number { const S = this.S; return S.d[S.b8 + 2818 + this.r]; } set stepAt(v: number) { const S = this.S; S.d[S.b8 + 2818 + this.r] = v; }
  get lastFireT(): number { const S = this.S; return S.d[S.b8 + 3970 + this.r]; } set lastFireT(v: number) { const S = this.S; S.d[S.b8 + 3970 + this.r] = v; }
  /** friends.ts: his raised gun is down for a teammate in its line (FRIENDS.lowReady), and how long that line has been clear */
  get friendLow(): boolean { const S = this.S; return (S.u[S.b4 + 21828 + this.r] & 64) !== 0; } set friendLow(v: boolean) { const S = this.S; const w = S.u[S.b4 + 21828 + this.r]; S.u[S.b4 + 21828 + this.r] = v ? w | 64 : w & 4294967231; }
  /** with friendLow */
  get friendT(): number { const S = this.S; return S.d[S.b8 + 4034 + this.r]; } set friendT(v: number) { const S = this.S; S.d[S.b8 + 4034 + this.r] = v; }
  get hitFlash(): number { const S = this.S; return S.d[S.b8 + 4098 + this.r]; } set hitFlash(v: number) { const S = this.S; S.d[S.b8 + 4098 + this.r] = v; }
  /** a blast that left him up (samstun.ts): folded until / blind until, on the game clock, and where the flash was — the same two stamps a Guard carries */
  get stunPosed(): boolean { const S = this.S; return (S.u[S.b4 + 21828 + this.r] & 128) !== 0; } set stunPosed(v: boolean) { const S = this.S; const w = S.u[S.b4 + 21828 + this.r]; S.u[S.b4 + 21828 + this.r] = v ? w | 128 : w & 4294967167; }
  /** a sprint into a door (sprintdoor.ts): when he last put his shoulder through one and which way it turned him, and until when his hand is ripping one round (the run held to a walk) — on the game clock */
  get doorBashAt(): number { const S = this.S; return S.d[S.b8 + 4162 + this.r]; } set doorBashAt(v: number) { const S = this.S; S.d[S.b8 + 4162 + this.r] = v; }
  /** with doorBashAt */
  get doorBashLean(): number { const S = this.S; return S.d[S.b8 + 4226 + this.r]; } set doorBashLean(v: number) { const S = this.S; S.d[S.b8 + 4226 + this.r] = v; }
  /** with doorBashAt */
  get doorRipUntil(): number { const S = this.S; return S.d[S.b8 + 4290 + this.r]; } set doorRipUntil(v: number) { const S = this.S; S.d[S.b8 + 4290 + this.r] = v; }
  /** with doorBashAt */
  get doorKickTail(): boolean { const S = this.S; return (S.u[S.b4 + 21828 + this.r] & 256) !== 0; } set doorKickTail(v: boolean) { const S = this.S; const w = S.u[S.b4 + 21828 + this.r]; S.u[S.b4 + 21828 + this.r] = v ? w | 256 : w & 4294967039; }
  /** the look his head was last given against that turn, and the game time it was given on (player.ts's sprint facing; game.ts takes it back off the frame nobody renews it) */
  get doorHead(): number { const S = this.S; return S.d[S.b8 + 4354 + this.r]; } set doorHead(v: number) { const S = this.S; S.d[S.b8 + 4354 + this.r] = v; }
  /** with doorHead */
  get doorHeadT(): number { const S = this.S; return S.d[S.b8 + 4418 + this.r]; } set doorHeadT(v: number) { const S = this.S; S.d[S.b8 + 4418 + this.r] = v; }
  /** the body being dragged (F): crouched, facing it, its ankles in your hands (ragdoll pins), the rest trailing along the floor */
  get dragYaw(): number { const S = this.S; return S.d[S.b8 + 4482 + this.r]; } set dragYaw(v: number) { const S = this.S; S.d[S.b8 + 4482 + this.r] = v; }
  /** with dragYaw */
  get dragNoiseT(): number { const S = this.S; return S.d[S.b8 + 4546 + this.r]; } set dragNoiseT(v: number) { const S = this.S; S.d[S.b8 + 4546 + this.r] = v; }
  /** with dragYaw */
  get dragStuckT(): number { const S = this.S; return S.d[S.b8 + 4610 + this.r]; } set dragStuckT(v: number) { const S = this.S; S.d[S.b8 + 4610 + this.r] = v; }
  /** seconds left of the rifle coming off the sling into the hands (or of any change of what is in them that involves it): no round or ring meanwhile */
  get equipT(): number { const S = this.S; return S.d[S.b8 + 4674 + this.r]; } set equipT(v: number) { const S = this.S; S.d[S.b8 + 4674 + this.r] = v; }
  /** walking-pace multiplier set with the mouse wheel, PACE_MIN‥1 of the walk / crouch-walk speed (sprint is its own held modifier); shown as ticks in the kit column */
  get pace(): number { const S = this.S; return S.d[S.b8 + 4738 + this.r]; } set pace(v: number) { const S = this.S; S.d[S.b8 + 4738 + this.r] = v; }
  /** with pace */
  get paceShownT(): number { const S = this.S; return S.d[S.b8 + 4802 + this.r]; } set paceShownT(v: number) { const S = this.S; S.d[S.b8 + 4802 + this.r] = v; }
  get nv(): boolean { const S = this.S; return (S.u[S.b4 + 21828 + this.r] & 512) !== 0; } set nv(v: boolean) { const S = this.S; const w = S.u[S.b4 + 21828 + this.r]; S.u[S.b4 + 21828 + this.r] = v ? w | 512 : w & 4294966783; }
  get nvAmount(): number { const S = this.S; return S.d[S.b8 + 4866 + this.r]; } set nvAmount(v: number) { const S = this.S; S.d[S.b8 + 4866 + this.r] = v; }
  get canisters(): number { const S = this.S; return S.d[S.b8 + 8194 + this.r]; } set canisters(v: number) { const S = this.S; S.d[S.b8 + 8194 + this.r] = v; }
  get flashbangs(): number { const S = this.S; return S.d[S.b8 + 8258 + this.r]; } set flashbangs(v: number) { const S = this.S; S.d[S.b8 + 8258 + this.r] = v; }
  /** fragmentation grenades (frag.ts FRAG.issue; digit 5 then a click — G never throws one) */
  get frags(): number { const S = this.S; return S.d[S.b8 + 8322 + this.r]; } set frags(v: number) { const S = this.S; S.d[S.b8 + 8322 + this.r] = v; }
  /** Sam's FN2000 (docs/internal/sc20k-design.md §5, weapons.ts Rifle: the fn2000 row + its light switch, its selector, its attachments and the rings left) and whether this kit has it at all. Selected with 2 (2 again with it in the hands flips the load its launcher has on the right button: ring ⇄ 40 mm); `holstered` with it selected means SLUNG across his back (rigProps RIG.sc20k.slung) — the same one  */
  get hasRifle(): boolean { const S = this.S; return (S.u[S.b4 + 21828 + this.r] & 131072) !== 0; } set hasRifle(v: boolean) { const S = this.S; const w = S.u[S.b4 + 21828 + this.r]; S.u[S.b4 + 21828 + this.r] = v ? w | 131072 : w & 4294836223; }
  /** the rifle left his hands when he went down (combat.ts hitPlayer dropped it as an item): nothing to draw slung on the body */
  get rifleGone(): boolean { const S = this.S; return (S.u[S.b4 + 21828 + this.r] & 262144) !== 0; } set rifleGone(v: boolean) { const S = this.S; const w = S.u[S.b4 + 21828 + this.r]; S.u[S.b4 + 21828 + this.r] = v ? w | 262144 : w & 4294705151; }
  /** noshove.ts: what the men he met moved him by in this update (x, z), the clock of that update, and where the update found him — written and read inside one frame */
  get shovedT(): number { const S = this.S; return S.d[S.b8 + 8386 + this.r]; } set shovedT(v: number) { const S = this.S; S.d[S.b8 + 8386 + this.r] = v; }
  /** (…and the thing he was fetching when it came — a can by its kind: it is that thing's click only) */
  get kitOwed(): number { const S = this.S; return S.d[S.b8 + 8450 + this.r]; } set kitOwed(v: number) { const S = this.S; S.d[S.b8 + 8450 + this.r] = v; }
  /** the one-time word that the OCP lives on the pistol: said the first time the right button is pressed over a light with a long gun in the hands that has nothing under its barrel (the shotgun in the rifle's slot — player.ts) — after that the button is simply dead there */
  get ocpHinted(): boolean { const S = this.S; return (S.u[S.b4 + 21828 + this.r] & 524288) !== 0; } set ocpHinted(v: boolean) { const S = this.S; const w = S.u[S.b4 + 21828 + this.r]; S.u[S.b4 + 21828 + this.r] = v ? w | 524288 : w & 4294443007; }
  /** the keypad door he is entering the code at (the 'code' row — a sustained verb like the pick: devices.ts updateCoding beeps four keys over CODE_SECS and the pad takes it; a step, Space or Esc breaks off) and how far in he is; null when none */
  get codeT(): number { const S = this.S; return S.d[S.b8 + 8514 + this.r]; } set codeT(v: number) { const S = this.S; S.d[S.b8 + 8514 + this.r] = v; }
  /** handwork.ts: the mains breaker's lever under his hand (the 'breaker' rows with HANDWORK on — `on`: re-arming it; t: s since his hand got there; thrown: the mains have gone over), null when none · s the verb in hand has waited for his hand to reach the thing (a code, the lever) */
  get handWait(): number { const S = this.S; return S.d[S.b8 + 8578 + this.r]; } set handWait(v: number) { const S = this.S; S.d[S.b8 + 8578 + this.r] = v; }
  /** (the quick-throw key's press went to the order menu's frag row: the rest of that press — the key still down, on the page — is not a throw; cleared when the key is up. player.ts) */
  get throwEaten(): boolean { const S = this.S; return (S.u[S.b4 + 21828 + this.r] & 1048576) !== 0; } set throwEaten(v: boolean) { const S = this.S; const w = S.u[S.b4 + 21828 + this.r]; S.u[S.b4 + 21828 + this.r] = v ? w | 1048576 : w & 4293918719; }
  get throwHeld(): boolean { const S = this.S; return (S.u[S.b4 + 21828 + this.r] & 2097152) !== 0; } set throwHeld(v: boolean) { const S = this.S; const w = S.u[S.b4 + 21828 + this.r]; S.u[S.b4 + 21828 + this.r] = v ? w | 2097152 : w & 4292870143; }
  /** the leaf he is easing open by hand (the 'ease' row: started by a tap, it goes on round at the crack rate on its own until it stands open, or a step / Space / Esc stops it where it is — game.ts updateDoors runs it; stallT = seconds it has not moved, a body or a chair against it); null when none */
  get easeStallT(): number { const S = this.S; return S.d[S.b8 + 8642 + this.r]; } set easeStallT(v: number) { const S = this.S; S.d[S.b8 + 8642 + this.r] = v; }
  /** with easeStallT */
  get easeBest(): number { const S = this.S; return S.d[S.b8 + 8706 + this.r]; } set easeBest(v: number) { const S = this.S; S.d[S.b8 + 8706 + this.r] = v; }
  get fHeld(): boolean { const S = this.S; return (S.u[S.b4 + 21828 + this.r] & 4194304) !== 0; } set fHeld(v: boolean) { const S = this.S; const w = S.u[S.b4 + 21828 + this.r]; S.u[S.b4 + 21828 + this.r] = v ? w | 4194304 : w & 4290772991; }
  /** the optical wand (wand.ts): `wand` = it is in his kit (his card's row — kit.ts kitPlayer); `wanding` = he is on his knee at a shut door with it under the leaf (player.ts startWand / updateWanding; a step, Space or Esc pulls it out) */
  get wand(): boolean { const S = this.S; return (S.u[S.b4 + 21828 + this.r] & 8388608) !== 0; } set wand(v: boolean) { const S = this.S; const w = S.u[S.b4 + 21828 + this.r]; S.u[S.b4 + 21828 + this.r] = v ? w | 8388608 : w & 4286578687; }
  /** the last id a camera of his was given (stickycam.ts fireCam: one up each, never reused in a run) */
  get camSeq(): number { const S = this.S; return S.d[S.b8 + 8770 + this.r]; } set camSeq(v: number) { const S = this.S; S.d[S.b8 + 8770 + this.r] = v; }
  /** with frags */
  get throwKind(): (typeof THROWKIND_WORDS)[number] { const S = this.S; return THROWKIND_WORDS[S.u[S.b4 + 21956 + this.r]]; } set throwKind(v: (typeof THROWKIND_WORDS)[number]) { const S = this.S, i = THROWKIND_WORDS.indexOf(v); if (i < 0) throw new Error(`throwKind: no such word: ${v}`); S.u[S.b4 + 21956 + this.r] = i; }
  get heldKind(): (typeof HELDKIND_WORDS)[number] { const S = this.S; return HELDKIND_WORDS[S.u[S.b4 + 22020 + this.r]]; } set heldKind(v: (typeof HELDKIND_WORDS)[number]) { const S = this.S, i = HELDKIND_WORDS.indexOf(v); if (i < 0) throw new Error(`heldKind: no such word: ${v}`); S.u[S.b4 + 22020 + this.r] = i; }
  /** the breaching shotgun or the hammer in his big slot (kit.ts BIGSLOT, his card's; bigkit.ts: a row of a door locked against him). 'none' as built. */
  get bigKit(): (typeof BIGKIT_WORDS)[number] { const S = this.S; return BIGKIT_WORDS[S.u[S.b4 + 22084 + this.r]]; } set bigKit(v: (typeof BIGKIT_WORDS)[number]) { const S = this.S, i = BIGKIT_WORDS.indexOf(v); if (i < 0) throw new Error(`bigKit: no such word: ${v}`); S.u[S.b4 + 22084 + this.r] = i; }
  initPlayer(): void { this.stepTravel = 1; this.stepAt = -1; this.holstered = true; this.hitsLeft = 1; this.lastFireT = -10; this.friendT = -10; this.doorBashAt = -10; this.doorRipUntil = -1; this.doorHeadT = -1; this.pace = 1; this.paceShownT = -10; this.canisters = 4; this.flashbangs = 2; this.shovedT = -1; this.kitOwed = -1; }
}
/** the animator's kept state: a record of its own, on his row */
export class AnimCols {
  S!: MenStore; r = -1;
}
/** what suppression keeps of him (game/suppression.ts): a record of its own, on his row */
export class SupCols {
  S!: MenStore; r = -1;
  /** the scalar, 0‥cap */
  get v(): number { const S = this.S; return S.d[S.b8 + 2498 + this.r]; } set v(v: number) { const S = this.S; S.d[S.b8 + 2498 + this.r] = v; }
  /** envelope of the last increment (jumps to the round's add, decays at kickDecay/s): R4's per-round pulse */
  get kick(): number { const S = this.S; return S.d[S.b8 + 8834 + this.r]; } set kick(v: number) { const S = this.S; S.d[S.b8 + 8834 + this.r] = v; }
  /** the muzzle the last suppressing round left */
  get src(): Vec3 | null { const S = this.S; if ((S.u[S.b4 + 21828 + this.r] & 16777216) === 0) return null; const i = S.b8 + this.r; return [S.d[i + 8898], S.d[i + 8962], S.d[i + 9026]]; }
  set src(v: Readonly<Vec3> | null) { const S = this.S, w = S.u[S.b4 + 21828 + this.r], i = S.b8 + this.r; if (v === null) { S.u[S.b4 + 21828 + this.r] = w & 4278190079; S.d[i + 8898] = 0; S.d[i + 8962] = 0; S.d[i + 9026] = 0; return; } S.u[S.b4 + 21828 + this.r] = w | 16777216; S.d[i + 8898] = v[0]; S.d[i + 8962] = v[1]; S.d[i + 9026] = v[2]; }
  get srcX(): number { const S = this.S; return S.d[S.b8 + 8898 + this.r]; } set srcX(v: number) { const S = this.S; S.d[S.b8 + 8898 + this.r] = v; }
  get srcY(): number { const S = this.S; return S.d[S.b8 + 8962 + this.r]; } set srcY(v: number) { const S = this.S; S.d[S.b8 + 8962 + this.r] = v; }
  get srcZ(): number { const S = this.S; return S.d[S.b8 + 9026 + this.r]; } set srcZ(v: number) { const S = this.S; S.d[S.b8 + 9026 + this.r] = v; }
  /** whose it was (Character id) */
  get srcId(): number { const S = this.S; return S.d[S.b8 + 9090 + this.r]; } set srcId(v: number) { const S = this.S; S.d[S.b8 + 9090 + this.r] = v; }
  /** and when */
  get srcT(): number { const S = this.S; return S.d[S.b8 + 9154 + this.r]; } set srcT(v: number) { const S = this.S; S.d[S.b8 + 9154 + this.r] = v; }
  /** the largest single add lately */
  get lastAdd(): number { const S = this.S; return S.d[S.b8 + 9218 + this.r]; } set lastAdd(v: number) { const S = this.S; S.d[S.b8 + 9218 + this.r] = v; }
  /** the game time the scalar last crossed flinch upward (edges for the anim / barks; the QA probes print them) */
  get flinchAt(): number { const S = this.S; return S.d[S.b8 + 9282 + this.r]; } set flinchAt(v: number) { const S = this.S; S.d[S.b8 + 9282 + this.r] = v; }
  /** …and pinned */
  get pinnedAt(): number { const S = this.S; return S.d[S.b8 + 9346 + this.r]; } set pinnedAt(v: number) { const S = this.S; S.d[S.b8 + 9346 + this.r] = v; }
  /** last frame's band (0 under flinch, 1 flinch, 2 pinned): the rising edges are the events */
  get band(): number { const S = this.S; return S.d[S.b8 + 9410 + this.r]; } set band(v: number) { const S = this.S; S.d[S.b8 + 9410 + this.r] = v; }
  /** guard: seconds of head-down still owed from the last crossing of `pinned` (drawn from SUPPRESSION.headDown) */
  get downT(): number { const S = this.S; return S.d[S.b8 + 9474 + this.r]; } set downT(v: number) { const S = this.S; S.d[S.b8 + 9474 + this.r] = v; }
  /** latched above `pinned`, released under `release`: head down while this OR downT */
  get latched(): boolean { const S = this.S; return (S.u[S.b4 + 21828 + this.r] & 33554432) !== 0; } set latched(v: boolean) { const S = this.S; const w = S.u[S.b4 + 21828 + this.r]; S.u[S.b4 + 21828 + this.r] = v ? w | 33554432 : w & 4261412863; }
  /** seconds spent head-down since s was last under `flinch`: displace at displaceAfter */
  get pinnedT(): number { const S = this.S; return S.d[S.b8 + 9538 + this.r]; } set pinnedT(v: number) { const S = this.S; S.d[S.b8 + 9538 + this.r] = v; }
  /** the cover spot he is shifting to right now (null = not displacing) */
  get cover(): Vec3 | null { const S = this.S; if ((S.u[S.b4 + 21828 + this.r] & 67108864) === 0) return null; const i = S.b8 + this.r; return [S.d[i + 9602], S.d[i + 9666], S.d[i + 9730]]; }
  set cover(v: Readonly<Vec3> | null) { const S = this.S, w = S.u[S.b4 + 21828 + this.r], i = S.b8 + this.r; if (v === null) { S.u[S.b4 + 21828 + this.r] = w & 4227858431; S.d[i + 9602] = 0; S.d[i + 9666] = 0; S.d[i + 9730] = 0; return; } S.u[S.b4 + 21828 + this.r] = w | 67108864; S.d[i + 9602] = v[0]; S.d[i + 9666] = v[1]; S.d[i + 9730] = v[2]; }
  get coverX(): number { const S = this.S; return S.d[S.b8 + 9602 + this.r]; } set coverX(v: number) { const S = this.S; S.d[S.b8 + 9602 + this.r] = v; }
  get coverY(): number { const S = this.S; return S.d[S.b8 + 9666 + this.r]; } set coverY(v: number) { const S = this.S; S.d[S.b8 + 9666 + this.r] = v; }
  get coverZ(): number { const S = this.S; return S.d[S.b8 + 9730 + this.r]; } set coverZ(v: number) { const S = this.S; S.d[S.b8 + 9730 + this.r] = v; }
  /** when that shift began */
  get displacedAt(): number { const S = this.S; return S.d[S.b8 + 9794 + this.r]; } set displacedAt(v: number) { const S = this.S; S.d[S.b8 + 9794 + this.r] = v; }
  /** how many this life */
  get displaces(): number { const S = this.S; return S.d[S.b8 + 9858 + this.r]; } set displaces(v: number) { const S = this.S; S.d[S.b8 + 9858 + this.r] = v; }
  /** until when he holds the spot he reached */
  get coverUntil(): number { const S = this.S; return S.d[S.b8 + 9922 + this.r]; } set coverUntil(v: number) { const S = this.S; S.d[S.b8 + 9922 + this.r] = v; }
  /** his last suppression line: when */
  get barkT(): number { const S = this.S; return S.d[S.b8 + 9986 + this.r]; } set barkT(v: number) { const S = this.S; S.d[S.b8 + 9986 + this.r] = v; }
  /** when a line the clocks held back was held */
  get pendingT(): number { const S = this.S; return S.d[S.b8 + 10050 + this.r]; } set pendingT(v: number) { const S = this.S; S.d[S.b8 + 10050 + this.r] = v; }
  /** was over `pinned` at some point and has not been under 0.02 since (the hard man's 'that all you've got?' when it drains without him being hit) */
  get wasPinned(): boolean { const S = this.S; return (S.u[S.b4 + 21828 + this.r] & 134217728) !== 0; } set wasPinned(v: boolean) { const S = this.S; const w = S.u[S.b4 + 21828 + this.r]; S.u[S.b4 + 21828 + this.r] = v ? w | 134217728 : w & 4160749567; }
  initSup(): void { this.srcId = -1; this.srcT = -100; this.flinchAt = -100; this.pinnedAt = -100; this.displacedAt = -100; this.coverUntil = -1; this.barkT = -100; this.pendingT = -100; }
}
/** his aim's track (game/aim.ts): a record of its own, on his row */
export class AimCols {
  S!: MenStore; r = -1;
  /** 0‥1: how lined-up he is on the target */
  get track(): number { const S = this.S; return S.d[S.b8 + 2562 + this.r]; } set track(v: number) { const S = this.S; S.d[S.b8 + 2562 + this.r] = v; }
  /** shooter → target bearing last frame (NaN = none yet: the jump test) */
  get lastBearing(): number { const S = this.S; return S.d[S.b8 + 10114 + this.r]; } set lastBearing(v: number) { const S = this.S; S.d[S.b8 + 10114 + this.r] = v; }
  /** the target's angular rate across the shooter: its OWN velocity square to the line over the range, smoothed (rad/s): what the target term reads */
  get bearingRate(): number { const S = this.S; return S.d[S.b8 + 10178 + this.r]; } set bearingRate(v: number) { const S = this.S; S.d[S.b8 + 10178 + this.r] = v; }
  /** his own ground speed remembered over feetTau (m/s): the feet term reads max(now, this − steadyPace), so a skid to a halt out of a run still costs the next round or two */
  get speedSm(): number { const S = this.S; return S.d[S.b8 + 10242 + this.r]; } set speedSm(v: number) { const S = this.S; S.d[S.b8 + 10242 + this.r] = v; }
  /** the fastest his feet have gone, let go over AIMCOST.stopTau (m/s; aimcost.ts — 0 while that rule is off): stopping does not clear the cone on the frame */
  get moveSm(): number { const S = this.S; return S.d[S.b8 + 10306 + this.r]; } set moveSm(v: number) { const S = this.S; S.d[S.b8 + 10306 + this.r] = v; }
  /** player: when the reticle left the last man (back onto the SAME man inside a third of a second keeps the track) */
  get leftT(): number { const S = this.S; return S.d[S.b8 + 10370 + this.r]; } set leftT(v: number) { const S = this.S; S.d[S.b8 + 10370 + this.r] = v; }
  /** the track he had on `lastMan` as he left it (aimcost.ts: given back if he is straight back on it across a lamp) */
  get leftTrack(): number { const S = this.S; return S.d[S.b8 + 10434 + this.r]; } set leftTrack(v: number) { const S = this.S; S.d[S.b8 + 10434 + this.r] = v; }
  /** guard: planted for a shot in the shoot band this frame (aimBrake said so): the panel / probes read it */
  get braking(): boolean { const S = this.S; return (S.u[S.b4 + 21828 + this.r] & 268435456) !== 0; } set braking(v: boolean) { const S = this.S; const w = S.u[S.b4 + 21828 + this.r]; S.u[S.b4 + 21828 + this.r] = v ? w | 268435456 : w & 4026531839; }
  initAim(): void { this.lastBearing = NaN; this.leftT = -100; }
}
