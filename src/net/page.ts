// THE PAGE AS A CLIENT OF A GAME SERVER (the browser's side of src/net/: see wire.ts for what travels, client.ts for the client's core).
//   /?net=<ws address>&netgame=<the game's id>&netkey=<a seat's key>        (no key: a watcher)
// HOW IT STANDS THE GAME: as the engine replay stands a tape's (replay/recipestand.ts) — a networked game is a run tape written live. The page says hello, takes
// the server's RECIPE from the welcome, and is first sent to the recipe's own address with its seed, so the boot is that game's boot; then main.ts lays the
// recipe's knob tables and kit over this machine's at each point of the boot, exactly where it does for a replay. Once the game stands every seat is refereed
// (step.ts netStand) and the server is told `ready`.
// WHO STEPS THE GAME: this file, through the loop's `tick` seam (host/loop.ts LoopCtx.tick). On each tick of the page the player's input is sampled into a
// record (game/cmd.ts sampleCmd, off the page's own camera) and SENT; the game itself is stepped only through the ticks the server has confirmed, each on the
// records the server decided, by step.ts netStep — the one tick every machine runs. So this page's game is the server's to the bit, and its state word is held
// against the server's every NET.wordEvery ticks. Round those ticks it does the host's half of one (Game.hostAfter: update's own lines).
// THE DOOR (door.ts) is the half of it that knows no game: the address, the seat's key, the socket, how fast the ticks are walked, the look ahead's numbers, the line
// the player is shown. Over ticks nobody sees the picture's queues are let go when it is level again (engineplay.ts unseenDone).
// THE GAME IS THE SERVER'S: nothing on this page writes to it outside a confirmed tick. A card's Restart and a checkpoint's load are refused in words, Quit
// leaves the page (main.ts), no try, streak or best is filed for such a run, a page that parts from the server lets go of its seat, the sandbox flags are the
// recipe's before every tick, the loop's frozen frame leaves the launcher to the tick (host/loop.ts). A debugger row that pokes the game still would: the next
// state word says so.
// HANDS OFF: under a card that freezes the page (the pause card) and when the tab is hidden the page sends one record that holds nothing and carries the game's own
// word for it (wire.ts restOf: 'Paused'), so the server does not walk the man on with his last keys or send a held round; the confirmed ticks are still walked under a card (the game is live: it is not this player's to stop).
// Nothing here is read by the game. The page without ?net never calls into this file past `netFromUrl`'s first line.
import type { Game } from '../game/game';
import type { FollowCamera } from '../game/camera';
import type { Input } from '../game/input';
import type { View } from '../host/hud';
import type { PlayerCmd } from '../game/cmd';
import { sampleCmd } from '../game/cmd';                    // (game exports are called inside functions only: src/game is one import cycle)
import { irMade, irViewer } from '../game/irlaser';
import { activeScenario } from '../game/scenarioRun';
import type { Box } from '../scene/boxes';
import type { Stand } from '../host/ahead';
import { RecipeStand } from '../replay/recipestand';
import { recipeSearch, unseenDone } from '../replay/engineplay';
import { NETDOOR, NetDoor, doorFromUrl, movedTo, keyOff, CANT_PLAY, netTry, netTried, netStall, NETWAY, type NetPageState, type WalkRound } from './door';
import type { WireCmd } from './wire';
import { NET } from './wire';
import { recipeWhy, fromWire, toWire, type Recipe } from './recipe';
import { NET_KEY } from './address';
import { netStep, netStand, netWord, netWatch, netQuiet, netWhole, netAhead, type StepHost } from './step';
import { wordTick, NET_WORD_SEED } from './words';
import { myName, setSeatNames } from '../host/names';

/** what the page hands over once it stands (main.ts): the game, and what a tick's end and a catch-up's tidy touch */
export interface NetHost {
  game: Game;
  engine: StepHost & { settings: Record<string, unknown>; particles?: { clear(): void } | null; resetTaa?(why?: string): void };
  smoke: { update(dt: number): void; emitters: { kind: string }[] } | null;
  audio: { enabled: boolean; setVolumes(): void };
  loop: { tween?: { stale: boolean; rest(): void } | null };
  /** the picture's half of the prediction (host/ahead.ts AheadDraw): told, each page tick, where the game has the player's man and where his records take him */
  ahead?: { capture(id: number, from: Stand, to: Stand): void; rest(): void; last?: unknown; readonly draws: number; hides: (() => ((b: Box) => boolean) | null) | null } | null;
}
/** main()'s question, asked where it asks a replay's. null = no networked game asked for; 'moved' = the page is on its way to the game's own address (stop
 *  booting); else the client, or a refusal the page shows (`refuse` set: the game boots as itself behind the words). */
export async function netFromUrl(q: URLSearchParams): Promise<NetPlay | 'moved' | null> {
  const o = await doorFromUrl(q, myName()); if (!o) return null;
  const P = new NetPlay(o.door);
  if (P.refuse || !P.recipe) { if (q.has(NET_KEY)) keyOff(); return P; }
  return movedTo(o.door, recipeSearch(P.recipe, q, o.own)) ? 'moved' : P;
}

export class NetPlay {
  private rs: RecipeStand | null = null; private host: NetHost | null = null;
  /** the whole game's words after the last word tick, with the ticks walked by then, kept while a check asks for them (`__net.keepWhole(true)`, read as `__net.whole`: which part parted, step.ts netWhole) */
  private whole: { t: number; names: string[]; words: number[] } | null = null; private keepWhole = false;
  private word: number = NET_WORD_SEED; private watch: string | null = null; private first = true;
  /** catching up: the mixer's own switch as it was */
  private audioWas = true;
  /** inside a confirmed tick */
  private stepping = false;
  private cmd: PlayerCmd | null = null;
  /** refused once the page stood (attach): the game it plays on as, by the loop's own tick */
  private alone: NetHost | null = null;
  /** the host's draws (AheadDraw.draws) when the world was last packed for a tick */
  private drawsAt = 0;
  private named: readonly string[] | null = null;

  constructor(readonly door: NetDoor) {
    if (!door.refuse) { if (!this.recipe || recipeWhy(this.recipe)) door.turn(CANT_PLAY); else this.rs = new RecipeStand(this.recipe); }
  }
  get client() { return this.door.client; }
  /** why this page is not a client of the game ('' = it is) */
  get refuse(): string { return this.door.refuse; }
  get heldMs(): number { return this.door.heldMs; }
  close(): void { this.door.close(); }
  state(): NetPageState { return this.door.state(this.word.toString(36)); }
  get recipe(): Recipe | null { return this.client.welcome?.recipe ?? null; }

  // ---- standing the game as the server's recipe says (main.ts asks where it asks a replay's tape)
  tables(): void { this.rs?.tables(); }
  env(settings: Record<string, unknown>): void { this.rs?.env(settings); }
  stood(g: Game): void { this.rs?.stood(g); }

  /** the page stands. Held against the recipe first (a page that booted another mission is no client of this one); then every seat is refereed, a card's
   *  Restart is refused, and the server is told this seat can play */
  attach(h: NetHost): void {
    const g = h.game, c = this.client;
    if (activeScenario(g) !== this.recipe?.scenario || g.seats.length !== c.welcome?.seats) { this.alone = h; this.door.turn(CANT_PLAY); return; }   // (…and plays on as its own game under the words: tick below)
    this.host = h; netStand(g);
    if (c.seat >= 0) g.localSeat = g.seats[c.seat];   // (the seat this page plays is the one it shows — game/seats.ts: its man under the camera, its HUD, its record sampled; a watcher keeps the first. The simulation never reads it)
    // (another player's goggles: an infrared line is in the frame's list on EVERY machine of a game once any player wears his — the Rust tick's ir_viewer asks
    //  every seat's man, so that the machines hold one game. A page whose own man's eyes are bare leaves them out of what it draws)
    if (h.ahead) h.ahead.hides = () => g.seats.length > 1 && !irViewer(g) ? irMade : null;
    const restart = g.restart.bind(g);   // (the player's restart: inside a tick — a key in a record — it is every machine's; from OUTSIDE one — a card's button — it is this page's alone, so not this page's to do)
    g.restart = said => { if (this.stepping) restart(said); else this.door.pass('You can’t restart a co-op game from here.'); };
    const load = g.saves.load.bind(g.saves);   // (…and a checkpoint's load, the same: the words are the card's own 'why it would not load')
    g.saves.load = (...a: Parameters<typeof load>) => this.stepping ? load(...a) : 'You can’t load a checkpoint in a co-op game.';
    if (c.seat >= 0) c.ready();
    document.addEventListener('visibilitychange', () => { if (document.hidden) c.rest(performance.now()); });
    const me = this; (window as unknown as { __net: unknown }).__net = { get state() { return me.state(); }, client: c, ahead: h.ahead ?? null, way: NETWAY, get whole() { return me.whole; }, keepWhole: (on: boolean) => { me.keepWhole = on; }, get tried() { return netTried(); }, try: netTry, stall: netStall };
  }

  /** ONE TICK OF THE PAGE (the loop's `tick`): the player's record sent, the confirmed ticks walked, the host's half of a tick round them (Game.update's) */
  tick(_dt: number, input: Input, cam: FollowCamera, view: View): void {
    const h = this.host; if (!h) { const a = this.alone; if (a) { a.game.update(_dt, input, cam, view); a.engine.lights.update(_dt, a.game.time); } return; }   // (refused once stood: the loop's own tick)
    const g = h.game, c = this.client, now = performance.now();
    this.door.wake(this.round);
    if (c.seat >= 0 && c.state !== 'waiting') { if (!this.door.far) c.beat(toWire(this.cmd = sampleCmd(g.hostSeat().view, input, cam, view, this.cmd)), now); else c.rest(now); }
    this.walk(); this.foresee();
    this.label();
    g.hostAfter(input, cam, view);
  }
  /** a frame the page is frozen on (a card is up): the hands come off, the game goes on */
  rest(input: Input, cam: FollowCamera, view: View): void { const h = this.host; if (!h) return; this.client.rest(performance.now()); this.walk(); this.foresee(); this.label(); h.game.hostAfter(input, cam, view); }
  /** the players' names as the server last said them, for whatever the page writes a player's name on (host/names.ts): its labels, never the game's state */
  private label(): void { const c = this.client; if (this.host && c.names !== this.named) setSeatNames(this.host.game, this.named = c.names); }

  /** the look ahead, at the end of a page tick (foresee.ts): the picture is told where the game has his man and where his records take him (host/ahead.ts) */
  private foresee(): void { const h = this.host!, seat = this.client.seat >= 0 ? h.game.seats[this.client.seat] : null, F = this.door.look(seat && !seat.view.player.down ? cmds => netAhead(h.game, seat, cmds) : null); if (F) h.ahead?.capture(F.id, F.from, F.to); else h.ahead?.rest(); }
  /** how far ahead of where the game has him the player's man is drawn, and where his newest record aims (the loop's `lead`: host/loop.ts); null = he is drawn where the game has him */
  lead(): { by: number[]; aim: [number, number, number] | null } | null { return this.door.fore.last; }

  private walk(): void {
    const h = this.host!, g = h.game, far = this.door.client.behind >= NETDOOR.far || this.door.far;
    const step = (cmds: WireCmd[]) => {
      if (this.first) { this.first = false; this.rs?.tables(); g.fogOfWar.pageLook(); this.rs?.tune(g); }   // (the last word over what the boot wrote after the staging, as a replay's first step has it)
      this.rs?.env(h.engine.settings); this.rs?.flags(g);
      { const d = h.ahead?.draws ?? 0; if (d !== this.drawsAt) { this.drawsAt = d; h.engine.world.relist(); } }   // (THE LIST A TICK READS IS THE LAST TICK'S, as on the server: a draw since then listed the boxes as it DREW them — between two ticks, his own man ahead — and the world's ray reads that list. So the first tick after a draw lists them again as the game has them)
      this.stepping = true; try { netStep(g, h.engine, cmds.map(fromWire)); } finally { this.stepping = false; }
      this.word = netWord(g, this.word); if (wordTick(this.client.confirmed)) this.watch = netWatch(g); if (this.keepWhole && wordTick(this.client.confirmed)) { const A = netWhole(g); if (A) this.whole = { t: this.client.confirmed + 1, ...A }; }   // (inside the client's walk: `confirmed` is still the tick this step ran)
      if (far) { h.smoke?.update(1 / NET.hz); const tw = h.loop.tween; if (tw && !tw.stale) tw.rest(); }   // (unseen ticks: the smoke keeps the game's time, and nothing is drawn between two of them — as under a replay's seek)
    };
    this.door.walk({ step, word: () => this.word.toString(36), watch: () => this.watch }, this.round);
  }
  /** round the confirmed ticks (a replay's seek: replay/engineplay.ts): the mixer shut over ticks nobody sees, and its tidy when the page is level; and one quiet
   *  run a walk (step.ts netQuiet), so that what the picture reads is brought up once as the walk ends */
  private round: WalkRound = {
    far: on => { const h = this.host!; if (on) { this.audioWas = h.audio.enabled; h.audio.enabled = false; h.audio.setVolumes(); } else unseenDone(h, this.audioWas, 'net'); },
    run: on => netQuiet(this.host!.game, on),
  };
}
