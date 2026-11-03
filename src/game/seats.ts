// SEATS — one or two players in one game (co-op groundwork; nothing here is networked). A SEAT is a player's place in the game: his man, the two teammates he
// orders, his input and lens (cmd.ts Seat: the tick's record loaded), his order menu and its queue, the reticle and the rows under it. `Game.seats` is fixed when
// the game is stood up (standGame's `seats`), and `Game.stepSeats(dt, cmds)` steps every seat from its own record.
//
// HOW A SECOND PLAYER FITS A GAME WRITTEN FOR ONE. The player's code — player.ts, aim.ts, interact.ts, the kit, orders.ts, doortools.ts, some hundred files — is
// written against `g.player`, `g.orders`, `g.aimPoint`. It is run for each seat, unedited, over that seat's VIEW of the game: the Game itself in every respect
// but the fields that are one player's own (SEAT_KEYS), which read and write as that seat's. It is squadcast.ts's device (a second cast of the squad's code over
// a view) made the rule for players: a function handed a view works for that view's man, a module built over a view (a seat's SquadOrders, its DoorTools) is that
// seat's for good, and a closure that kept the `g` it was handed still names the right man when it runs later (a can's bounce, a charge's blast).
//   - A GAME OF ONE SEAT has no view: its seat's view IS the Game and the fields are the Game's own, as they always were. Nothing below runs.
//   - A GAME OF TWO: every seat has a view and a record of its own (`Seat.own`), the first seat too. The Game's own fields of those names become accessors.
//     Inside a tick they answer the FIRST seat's (old code that still asks the Game for "the player" gets the first man, on every machine alike) and every such
//     ask is counted (`SEAT_ASKS.misses`); under `SEAT_ASKS.strict` it throws instead, which is how tools/qa/seats-check.ts proves that a squad mission's tick
//     asks no such thing. Outside a tick they answer the seat this host shows (`Game.localSeat`): the HUD's and the camera's "the player".
//   - What is the WORLD's is asked of the world: the men's side of a rule takes the man (`men(g)`, `manOf`), a teammate's orders are his seat's (`ordersOf`).
//   - State kept beside the game in a side table: a `GameSideMap` (math/side.ts) is the game's whoever asks; a plain `SideMap` keyed by the game keeps a row a
//     view, which is right for what is one player's (SEAT_TABLES below lists them; the check is red on any other).
// Imports: types, and values used inside functions only (src/game is one import cycle).
import type { Game, Guard, Player } from './game';
import type { Character } from './character';
import type { SquadOrders, Assignment } from './orders';
import { Seat } from './cmd';
import { RAW, CAST_VIEW } from '../math/side';
import { PLAYER_ID, NOBODY_ID } from './consts';
import { Player as PlayerMan } from './game';
import { railLightDef } from './rigProps';
import { v3 } from '../math/vec';
import type { Vec3 } from '../math/vec';

/** `max`: how many seats a game may be stood up with. */
export const SEATS = {
  max: 2,
  /** how far from the first man a later seat's man is stood where nothing says otherwise (m) */
  apart: 0.9,
  /** an enemy with nobody as his foe keeps the player he has in mind until another is this much nearer (m): no flicker between two who stand together */
  keep: 2 };
/** HOW MANY SEATS AN ADDRESS ASKS FOR: `?seats=2` (1 with no such word, or one that is no count; never more than SEATS.max). The one reading of the word:
 *  standGame's, with no count handed to it, of the page's own address; a host that stands a game from a recipe hands it the recipe's (format.ts RunTape.recipe.search). */
export function seatsOfSearch(search: string | null | undefined): number {
  const m = /(?:^|[?&])seats=(\d+)(?:&|$)/.exec(search ?? ''), n = m ? parseInt(m[1], 10) : 1;
  return Math.max(1, Math.min(SEATS.max, n || 1));
}
/** the scenario kinds a game of two can start (seatsRefuse) */
const SEAT_KINDS: readonly string[] = ['roomclear', 'nightbreach'];
/** A CHECK'S WORDS AND COUNTS, nothing a rule reads. `strict`: the Game's own field of a seat's name, asked in a game of more than one seat, throws (off, it answers
 *  the first seat's and counts). `misses` / `missed`: those asks so far, and the fields asked. `sites`: every such ask by where it came from ('file:line' → count;
 *  null = not kept). `shared`: the break word, read once as a game is stood up — every seat's view answers the FIRST seat's record, which is one player in two bodies. */
export const SEAT_ASKS = { strict: false, shared: false, misses: 0, missed: new Set<string>(), sites: null as Map<string, number> | null };

/** THE GAME'S FIELDS THAT ARE ONE PLAYER'S OWN, by name. A view answers these from its seat's record; everything else is the one Game's.
 *  his man · his squad's kit (the menu and its orders, the door tools, the wall charge, the things a teammate works: each built over his view) · his seat ·
 *  what his cursor is on and what he may do there · the row he performed this tick · his view's matrix · how near a threat is to him
 *  (NOT a seat's: `puppet`, a show's hands on the player — a show has the whole floor; `mPerPx`, the host's picture) */
export const SEAT_KEYS = ['seat', 'player', 'orders', 'doorTools', 'work', 'wallCharge',
  'aimPoint', 'aimTarget', 'aimGuard', 'aimFriend', 'aimSnap', 'hover', 'interactables', 'nearby', 'act', 'useDoor',
  'viewProj', 'threatProximity', 'torchOnT'] as const;
const IS_SEAT_KEY = new Set<PropertyKey>(SEAT_KEYS);

/** THE NAMED SIDE TABLES THAT ARE A SEAT'S (keyed by the view they were handed: a row a player). A plain table handed a view that is not listed here is a
 *  mistake — world state forked in two — and tools/qa/seats-check.ts is red on it: such a table is a `GameSideMap`. */
export const SEAT_TABLES: readonly string[] = ['blastFeel.STATE', 'blastLook.STATE', 'close.STATE', 'guards.groundTrack', 'holdfire.states', 'player.lightAt', 'rearwatch.books', 'samlook.STATE',
  'showwhy.states', 'silent.seats', 'silent.states', 'squad40.states', 'squadsmoke.states', 'squadthrow.states', 'stalk.STATE', 'tapstep.STATE', 'unseen.states', 'wandorder.states'];

type Rec = Record<PropertyKey, unknown>;

/** a seat's view of `g`: the seat's own fields from `own`, the rest the Game's. A method called on it runs with the view as `this`, a getter likewise, so what
 *  they ask of "the player" is this seat's; a write to anything that is not the seat's goes to the Game. */
function seatView(g: Game, own: Rec): Game {
  return new Proxy(g, {
    get: (t, k, r) => k === RAW ? t : IS_SEAT_KEY.has(k) ? own[k] : Reflect.get(t, k, r),
    set: (t, k, v) => { if (IS_SEAT_KEY.has(k)) { own[k] = v; return true; } return Reflect.set(t, k, v, t); },
    has: (t, k) => Reflect.has(t, k),
  });
}

/** THE GAME'S SEATS, made as its constructor begins (before anything is built over a view). One: the Game is its own seat's view. More: each seat a view and a
 *  record; the first takes what the Game's fields hold now, the others a fresh copy of the same, and the Game's own fields of those names answer for the first. */
export function seatsStand(g: Game, n: number): Seat[] {
  if (!(n >= 1 && n <= SEATS.max) || n !== Math.floor(n)) throw new Error(`a game is stood up with 1 to ${SEATS.max} seats, not ${n}`);
  const G = g as unknown as Rec;
  if (n === 1) { const s = new Seat(0); s.view = g; G.seat = s; return [s]; }
  const seats: Seat[] = [], blank: Rec = {};
  for (const k of SEAT_KEYS) blank[k] = G[k];
  for (let i = 0; i < n; i++) {
    const s = new Seat(i), own: Rec = {};
    for (const k of SEAT_KEYS) { const v = blank[k]; own[k] = Array.isArray(v) ? v.slice() : v; }
    own.seat = s; s.own = own as Record<string, unknown>; s.view = seatView(g, SEAT_ASKS.shared && i ? seats[0].own as Rec : own); seats.push(s);   // (`shared`: the check's break word)
  }
  // (the Game's own fields of those names. INSIDE a tick: the first seat's, and a miss — the simulation has no "the player". OUTSIDE it — the host's HUD, its
  //  camera, a panel: the seat this host shows, Game.localSeat, which is what such code means by "the player")
  const first = seats[0].own as Rec, shown = (): Rec => g.ticking ? (first) : ((g.localSeat ?? seats[0]).own as Rec);
  for (const k of SEAT_KEYS) Object.defineProperty(g, k, { configurable: true, enumerable: false,
    get() { if (g.ticking) miss(k); return shown()[k]; }, set(v: unknown) { if (g.ticking) miss(k); shown()[k] = v; } });
  return seats;
}
function miss(k: string): void {
  if (SEAT_ASKS.strict) throw new Error(`seats: the Game's own '${k}' was asked in a game of more than one seat (a player's code is handed his seat's view; the world asks men(g) / manOf / ordersOf)`);
  SEAT_ASKS.misses++; SEAT_ASKS.missed.add(k);
  if (SEAT_ASKS.sites) { const at = (new Error().stack ?? '').split('\n').slice(3, 6).map(l => l.replace(/^.*[\\/]src[\\/]/, '').replace(/\)?$/, '').replace(/^\s*at\s+/, '')).join(' < '); const key = `${k} @ ${at}`; SEAT_ASKS.sites.set(key, (SEAT_ASKS.sites.get(key) ?? 0) + 1); }
}

/** the Game a view looks at (itself, for the Game) */
export function rawGame(g: Game): Game { return (g as unknown as { [RAW]?: Game })[RAW] ?? g; }

// ---------------------------------------------------------------- the men
/** the character id of seat `i`'s man: the first is PLAYER_ID, as ever; the next ids past NOBODY_ID are the other seats' (the staff start at 10) */
export const seatManId = (i: number): number => i === 0 ? PLAYER_ID : NOBODY_ID + i;
/** is `id` a player's man's (whoever's, whether or not that seat is in this game) */
export const isManId = (id: number): boolean => id === PLAYER_ID || (id > NOBODY_ID && id < NOBODY_ID + SEATS.max);
/** every seat's man, in seat order */
export function men(g: Game): Player[] { const S = g.seats; if (S.length === 1) return [g.player]; const out: Player[] = []; for (const s of S) out.push(s.view.player); return out; }
/** the men still up */
export function menUp(g: Game): Player[] { return men(g).filter(p => !p.down); }
/** is every player down (a game of one: its player is) */
export function allDown(g: Game): boolean { const S = g.seats; if (S.length === 1) return g.player.down; for (const s of S) if (!s.view.player.down) return false; return true; }
/** is any player down */
export function anyDown(g: Game): boolean { const S = g.seats; if (S.length === 1) return g.player.down; for (const s of S) if (s.view.player.down) return true; return false; }
/** `f` for every seat, over its view — how the world runs a thing written for the one player (the Game it was handed, in a game of one) */
/** WHICH MISSIONS SEAT TWO: the scenarios whose own rules have been taken through for more than one player. The others are written round one commander (the
 *  hostage rescue's word and its route flow, the quiet mission's one partner) and refuse a game of two until they are. */
/** can a mission of this kind be played by two (the lobby asks before a game is made: ui/coop.ts) */
export const seatsKind = (kind: string): boolean => SEAT_KINDS.includes(kind);
export function seatsRefuse(g: Game, kind: string): string | null { return g.seats.length > 1 && !SEAT_KINDS.includes(kind) ? 'it takes one player today' : null; }
/** the man whose eyes and ears this host's picture and speakers are (a host that plays no seat: the first) */
export function hostMan(g: Game): Player { return g.seats.length === 1 ? g.player : g.hostSeat().view.player; }
/** the man a writer with no man of its own names (a debrief row's place, the gun a kill is filed under): the view's, handed a seat's view; of the Game itself,
 *  the first player's */
export function manHere(g: Game): Player { return g.seats.length === 1 || rawGame(g) !== g ? g.player : g.seats[0].view.player; }
export function eachSeat(g: Game, f: (v: Game) => void): void { const S = g.seats; if (S.length === 1) f(g); else for (const s of S) f(s.view); }
/** the first answer any seat's view gives `f` (null: none has one) — a thing each player has his own of (his wall charges, the things his teammates work), asked
 *  for whoever's it is; the Game it was handed, in a game of one */
export function firstOf<T>(g: Game, f: (v: Game) => T | null | undefined): T | null { const S = g.seats; if (S.length === 1) return f(g) ?? null; for (const s of S) { const r = f(s.view); if (r != null) return r; } return null; }
/** the seat whose man this character is (null: nobody's — a guard, a teammate, a civilian) */
export function seatOfChar(g: Game, c: Character | null | undefined): Seat | null { if (!c) return null; for (const s of g.seats) if (s.view.player.char === c) return s; return null; }
/** … and the man himself */
export function manOf(g: Game, c: Character | null | undefined): Player | null { const S = g.seats; if (S.length === 1) return c && g.player.char === c ? g.player : null; return seatOfChar(g, c)?.view.player ?? null; }
/** the seat whose man carries character id `id` (null: no player's) */
export function seatOfId(g: Game, id: number): Seat | null { if (!isManId(id)) return null; for (const s of g.seats) if (s.view.player.char.id === id) return s; return null; }
export function manById(g: Game, id: number): Player | null { const S = g.seats; if (S.length === 1) return g.player.char.id === id ? g.player : null; return seatOfId(g, id)?.view.player ?? null; }
/** the game as the player with this id sees it (null: no player's id) — what a thing done BY a man is done over */
export function viewOfId(g: Game, id: number): Game | null { const S = g.seats; if (S.length === 1) return g.player.char.id === id ? g : null; return seatOfId(g, id)?.view ?? null; }
/** the seat of a man */
export function seatOfMan(g: Game, pl: Player): Seat { for (const s of g.seats) if (s.view.player === pl) return s; return g.seats[0]; }
/** the living man nearest a place (null: every player is down) — what a rule that is about "the player near here" asks */
export function nearestMan(g: Game, at: ArrayLike<number>): Player | null {
  let best: Player | null = null, bd = Infinity;
  for (const s of g.seats) { const p = s.view.player; if (p.down) continue; const d = Math.hypot(p.char.pos[0] - at[0], p.char.pos[2] - at[2]); if (d < bd) { bd = d; best = p; } }
  return best;
}

/** A LATER SEAT'S MAN AND HIS LIGHT, made as the Game's constructor ends (the first seat's are made where they always were, so a game of one draws its dice and
 *  adds its lights in the order it always did) */
export function standMen(g: Game): void {
  for (const s of g.seats) { if (!s.index) continue; const v = s.view;
    v.player = new PlayerMan(g.ch, spotBeside(g, v3.copy(g.level.playerSpawn), s.index), g.fn2000, s.index);
    v.player.light = g.engine.lights.add(railLightDef(v.player.char.id, g.tune.playerLight)); }
}
/** where seat `i`'s man stands when the first stands on `at`: `at` itself for the first; for a later one the first free floor a body fits round him, `SEATS.apart`
 *  off with nothing solid between (and `at` when a cupboard has none) */
export function spotBeside(g: Game, at: Vec3, i: number): Vec3 {
  if (!i) return at;
  for (let k = 0; k < 8; k++) { const a = Math.PI / 2 + k * Math.PI / 4, p: Vec3 = [at[0] + Math.cos(a) * SEATS.apart * i, at[1], at[2] + Math.sin(a) * SEATS.apart * i], q = v3.copy(p);
    g.col.collideCircle(q, 0.3, 0.2, 1.5, 1);
    if (Math.hypot(q[0] - p[0], q[2] - p[2]) < 0.02 && !g.col.clampToWorld(q, 0.3) && !g.col.solidBlocked([at[0], 0.45, at[2]], [p[0], 0.45, p[2]])) return p; }
  return v3.copy(at);
}
/** A STAGING PUTS THE PLAYERS ON ITS START: the first man on `at` facing `yaw`, as a staging always put its one player; each later seat's man a stride behind the
 *  first of the teammates he was dealt (the team is cut into the seats' pairs here if it was not), facing as they do — or beside the first man when he has none.
 *  `each`: what the staging does to a player it has placed, done to every one. */
export function stageMen(g: Game, at: Vec3, yaw: number, each?: (v: Game, pl: Player) => void): void {
  dealSeats(g);
  const taken: Vec3[] = [at, ...g.guards.filter(gd => gd.state !== 'dead').map(gd => gd.char.pos)];
  for (const s of g.seats) { const v = s.view; let p = at;
    if (s.index) { const lead = g.guards.find(gd => !!gd.unit && gd.team === 'operative' && gd.unit.seat === s.index);
      p = lead ? g.nightBreach.standBy(lead.char.pos, taken, lead.char.bodyYaw) : spotBeside(g, at, s.index); taken.push(p); }
    v.teleportPlayer(v3.copy(p)); v.player.char.bodyYaw = v.player.char.aimYaw = yaw; each?.(v, v.player); }
}

/** A SCENARIO'S OPENING CUT: every seat's lens snapped to its own man, not flown (the seat's `takeCut` hands it to whoever hosts that player's picture) */
export function cutToMen(g: Game): void { for (const s of g.seats) { const c = s.cam; c.target = v3.copy(s.view.followPos()); c.desired = v3.copy(c.target); } }

// ---------------------------------------------------------------- who an enemy is up against
/** THE MAN AN ENEMY HAS IN MIND. The night staff's and a defender's own code (guards.ts) is up against ONE player: his eye's meter, the place he last had him,
 *  his foe are single. So each of them keeps one of the players in mind at a time (`Guard.faces`: a seat) and is stepped over that seat's view: the man he is
 *  after for as long as that man is up, else the nearest player still up. (What it does not give: one enemy building a sighting of both players at once.
 *  The room fight's own targets, the rounds and the blasts are not his code and take every man as he is: roomfight.ts, combat.ts.) */
export function facedSeat(g: Game, gd: Guard): Seat {
  const S = g.seats, cur = S[gd.faces ?? 0] ?? S[0];
  if (gd.foe === 'player' && !cur.view.player.down) return cur;
  let best = cur, bd = Infinity;
  for (const s of S) { const p = s.view.player; if (p.down) continue; const d = v3.distXZ(p.char.pos, gd.char.pos) - (s === cur ? SEATS.keep : 0); if (d < bd) { bd = d; best = s; } }
  if (best !== cur) gd.faces = best.index;
  return best;
}
/** the player an enemy has in mind as the tick stands (facedSeat chose him as the man's own step began), and the game as that player sees it: what the world's
 *  own code about an enemy and 'the player' asks (the one player, and the Game handed in, in a game of one) */
export function facedMan(g: Game, gd: Guard): Player { const S = g.seats; return S.length === 1 ? g.player : (S[gd.faces ?? 0] ?? S[0]).view.player; }
export function facedView(g: Game, gd: Guard): Game { const S = g.seats; return S.length === 1 ? g : (S[gd.faces ?? 0] ?? S[0]).view; }
/** the view a man of the floor is stepped over in a game of more than one seat: his own seat's for a teammate, the faced man's for anybody else */
/** the game as the man behind this body has it, for a world rule handed the bare Game (a round fired from the room fight): a player's man → his seat's; a
 *  teammate → his seat's; anybody else → the seat he has in mind. Never moves `faces`. A view handed in is kept. */
export function viewOfBody(g: Game, c: { id: number }): Game {
  if (g.seats.length === 1 || rawGame(g) !== g) return g;
  const mine = viewOfId(g, c.id); if (mine) return mine;
  const gd = g.guards.find(x => x.char.id === c.id); if (!gd) return g.seats[0].view;
  return gd.unit && gd.team === 'operative' ? seatOf(g, gd).view : facedView(g, gd);
}
export function viewFor(g: Game, gd: Guard): Game { return gd.unit && gd.team === 'operative' ? seatOf(g, gd).view : facedSeat(g, gd).view; }

// ---------------------------------------------------------------- whose teammate
/** THE DEAL: the team's roster, in its order, is cut into as many runs as there are seats — the squad of four is two pairs, one pair a seat; three are two and
 *  one; a pair is a man each. Written on each man's papers (`Guard.unit.seat`) whenever the roster is not the one last dealt, so a staging, a restart and a man
 *  spawned later are all dealt by the one rule. A game of one seat deals nobody: every teammate is the one seat's. Returns whether it dealt. */
export function dealSeats(g: Game): boolean {
  const n = g.seats.length; if (n === 1) return false;
  let fresh = 0; const roster: Guard[] = [], has: number[] = new Array(n).fill(0);
  for (const gd of g.guards) if (gd.unit && gd.team === 'operative') { roster.push(gd); if (gd.unit.seat === undefined) fresh++; else has[Math.min(n - 1, gd.unit.seat)]++; }
  if (!fresh) return false;
  const k = roster.length, per = Math.ceil(k / n);
  if (fresh === k) roster.forEach((gd, i) => { gd.unit!.seat = Math.min(n - 1, Math.floor(i / per)); });   // (a fresh team, in roster order: a pair stays a pair)
  else for (const gd of roster) if (gd.unit!.seat === undefined) { let to = 0; for (let i = 1; i < n; i++) if (has[i] < has[to]) to = i; gd.unit!.seat = to; has[to]++; }   // (a man who joins a dealt team goes to the seat with the fewest: nobody already dealt changes hands)
  return true;
}
/** the seat a teammate takes his orders from (the first seat's, for a man nobody was dealt: a game of one, a man of another cast) */
export function seatOf(g: Game, gd: Guard): Seat { return g.seats[gd.unit?.seat ?? 0] ?? g.seats[0]; }
/** is this teammate `s`'s to order */
export function seatHas(s: Seat, gd: Guard): boolean { return (gd.unit?.seat ?? 0) === s.index; }
/** ONE BOOK: the game handed in has one squad's orders and they are `g.orders` — a game of one seat, or another cast's view of any game (squadcast.ts: the
 *  response team run through the squad's code). Otherwise the team's orders are in a book a seat, and the functions below read across them. */
const oneBook = (g: Game): boolean => g.seats.length === 1 || (g as unknown as { [CAST_VIEW]?: boolean })[CAST_VIEW] === true;
/** the man a teammate follows: his seat's */
export function leaderOf(g: Game, gd: Guard): Player { return oneBook(g) ? g.player : seatOf(g, gd).view.player; }
/** the game as a teammate's own player sees it */
export function viewOf(g: Game, gd: Guard): Game { return oneBook(g) ? g : seatOf(g, gd).view; }
/** the orders that move a teammate: his seat's book */
export function ordersOf(g: Game, gd: Guard): SquadOrders { return oneBook(g) ? g.orders : seatOf(g, gd).view.orders; }
/** the order he is under, in whichever seat's book (null: none) */
export function orderOf(g: Game, gd: Guard): Assignment | null { return oneBook(g) ? g.orders.of(gd) : seatOf(g, gd).view.orders.of(gd); }
/** is there a squad to order at all, any seat's */
export function teamActive(g: Game): boolean { if (oneBook(g)) return !!g.orders?.active; for (const s of g.seats) if (s.view.orders.active) return true; return false; }
/** every seat's teammates, in the guard list's order — the TEAM's roster, the down among them (one book: its `squad()`) */
export function teamSquad(g: Game): Guard[] { return oneBook(g) ? g.orders.squad() : g.guards.filter(gd => !!gd.unit && gd.team === 'operative'); }
/** … and the living of them (one book: its `men()`) */
export function teamMen(g: Game): Guard[] { return oneBook(g) ? g.orders.men() : g.guards.filter(gd => !!gd.unit && gd.team === 'operative' && gd.state !== 'dead'); }
/** is the team in more than one pair (one book: its `paired`) */
export function teamPaired(g: Game): boolean { if (oneBook(g)) return g.orders.paired; const ids = new Set<string>(); for (const gd of g.guards) if (gd.unit?.pair && gd.team === 'operative') ids.add(gd.unit.pair); return ids.size > 1; }
/** A SEAT'S SHARE OF WHAT THE SQUAD BRINGS (`v`: the seat's view). The stun canisters, the charges, the smoke are one count for the squad; with two players a pair
 *  carries its men's part of it, dealt in roster order so the shares add up to the count. A game of one: the count. */
export function seatShare(v: Game, n: number): number {
  if (oneBook(v)) return n;
  const me = v.seat.index; let lo = 0, mine = 0, all = 0;
  for (const gd of v.guards) if (gd.unit && gd.team === 'operative') { const s = gd.unit.seat ?? 0; all++; if (s < me) lo++; else if (s === me) mine++; }
  if (!all) return me === 0 ? n : 0;
  return Math.floor(n * (lo + mine) / all) - Math.floor(n * lo / all);
}
/** the last order anybody gave, off the books' logs (null: none yet) */
export function lastOrder(g: Game): SquadOrders['log'][number] | null {
  if (oneBook(g)) { const lg = g.orders?.log ?? []; return lg[lg.length - 1] ?? null; }
  let best: SquadOrders['log'][number] | null = null; for (const s of g.seats) { const lg = s.view.orders.log, e = lg[lg.length - 1]; if (e && (!best || e.t > best.t)) best = e; } return best;
}
/** every seat's book, in seat order (one book: that one) */
export function teamBooks(g: Game): SquadOrders[] { return oneBook(g) ? [g.orders] : g.seats.map(s => s.view.orders); }
