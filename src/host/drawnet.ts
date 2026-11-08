// A NETWORKED GAME ON THE PAGE THAT DRAWS (net/door.ts has the server and how fast, net/modgame.ts the game): what the player did goes up as a record, the ticks the server has confirmed are walked, and his own
// man is drawn ahead of them by the records still on their way. Nothing of this page is written to the game but its records.
import { HAND, sampleHand, type HandCamera, type HandCanvas, type HandInput } from '../draw/hand';
import { CANT_PLAY, doorFromUrl, keyOff, movedTo, NETWAY, netStall, netTried, netTry, type NetDoor, type WalkRound } from '../net/door';
import type { Foreseen } from '../net/foresee';
import { NetGame, type NetFace } from '../net/modgame';
import { commandBytes, startWhy, type NetStart } from '../net/start';
import type { WireCmd } from '../net/wire';
import { recipeSearch } from '../tape/address';
import type { DrawGame, Start } from './drawgame';
import { myName } from './myname';

/** THE GAME AN ADDRESS NAMES, as its server says it: null = it names none, or this page is no client of it (the words are up, and the page comes up as itself) · 'moved' = the page is on its way to the
 *  game's own address, for a page reads its picture off its address */
export async function netAsked(q: URLSearchParams): Promise<{ door: NetDoor; start: NetStart } | 'moved' | null> {
  const o = await doorFromUrl(q, myName()); if (!o) return null;
  const { door, own } = o, start = door.client.welcome?.start ?? null;
  if (!door.refuse && (!start || startWhy(start))) door.turn(CANT_PLAY);
  if (door.refuse || !start) { keyOff(); return null; }
  return movedTo(door, recipeSearch({ search: start.words, seed: start.seed }, q, own)) ? 'moved' : { door, start };
}

export class DrawNet {
  private net!: NetGame; private level = false; private ahead: Foreseen | null = null;
  /** `unseen`: ticks nobody sees begin (nothing of the last ones seen sounds on) */
  private round: WalkRound;
  constructor(readonly door: NetDoor, private said: NetStart, private game: DrawGame, unseen: () => void) { this.round = { far: on => { if (on) unseen(); else this.level = true; } }; }

  /** the game every machine of it starts, on this build's own parts and the level the start names */
  start(own: Pick<Start, 'level' | 'mannequin' | 'mannequinBin' | 'cast' | 'castLook'>): void {
    const S = this.said, C = this.game.commands, G = this.game, c = this.door.client;
    C.take(); C.put(commandBytes(S)); G.start({ ...own, words: S.words, seed: S.seed, seats: S.seats }); G.walked = true;
    this.net = new NetGame(G.X as unknown as NetFace, G.memory, G.h);
    if (c.seat >= 0) G.X.face_show_seat(G.h, c.seat);
    G.show();
  }
  /** this page can play: the server begins when every seat can */
  ready(): void {
    const c = this.door.client; if (c.seat >= 0) c.ready();
    document.addEventListener('visibilitychange', () => { if (document.hidden) c.rest(performance.now()); });
    const me = this; (window as unknown as { __net: unknown }).__net = { client: c, get state() { return me.state(); }, whole: () => me.net.whole(true), man: (seat = Math.max(0, c.seat)) => me.net.ahead(seat, []), way: NETWAY, get tried() { return netTried(); }, try: netTry, stall: netStall };
  }
  state(): ReturnType<NetDoor['state']> { return this.door.state(this.net.word()); }

  /** ONE TICK OF THE PAGE: the player's record sent (`off`: a card is up, and his hands are off), the confirmed ticks walked, `after` each that is seen. True = the page is level again after ticks nobody saw */
  tick(input: HandInput, cam: HandCamera, canvas: HandCanvas, off: boolean, after: () => void): boolean {
    const { door, game, net } = this, c = door.client, now = performance.now(), N = game.numbersOf(), keys = sampleHand(N, input, cam, canvas), seat = Math.max(0, c.seat);
    door.wake(this.round);
    if (c.seat >= 0 && c.state !== 'waiting') { if (off || door.far) c.rest(now); else c.beat(recordOf(N, keys), now); }
    let over = false;
    const n = door.walk({ step: cmds => { const seen = !door.far; game.shownHand(seat, keys); net.step(cmds); game.stepped(seen, over); over = true; if (seen) after(); }, word: () => net.word(), watch: () => net.watch() }, this.round);
    const level = this.level; this.level = false;
    if (level) game.show();
    this.ahead = door.look(c.seat >= 0 ? cmds => net.ahead(c.seat, cmds) : null);
    game.lead(this.ahead, n === 0);
    return level;
  }
  /** how far ahead of where the game has him his man is drawn, and where his newest record aims (null: he is drawn where the game has him) */
  get lead(): { by: number[]; aim: [number, number, number] | null } | null { return this.ahead; }
}

/** a hand as it travels */
function recordOf(N: Float64Array, keys: string): WireCmd {
  const [held, down] = keys.split('\n').map(l => l.split(' ').filter(Boolean)), out: WireCmd = { k: held, b: N[HAND.buttons], m: [N[HAND.mx], N[HAND.my]], s: [N[HAND.w], N[HAND.h], N[HAND.pw], N[HAND.ph]],
    v: [N[HAND.target], N[HAND.target + 1], N[HAND.target + 2], N[HAND.yaw], N[HAND.pitch], N[HAND.distance], N[HAND.fov], N[HAND.aspect], N[HAND.aimLead]] };
  if (down.length) out.p = down; if (N[HAND.clicked]) out.c = N[HAND.clicked]; if (N[HAND.wheel]) out.w = N[HAND.wheel];
  return out;
}
