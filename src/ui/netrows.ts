// THE DEBUGGER'S NETWORK PAGE (2026-11-02), on a page that is a client of a networked game and on no other. Two players on one good network see none of
// what a real connection does to a game, so this page's connection can be made worse from here, LIVE, with no reload (net/page.ts NETWAY has where the
// numbers live and how long they are kept; net/delay.ts what each does). A page's numbers are its own: each player sets them on their own page.
//   Connection        a named kind of connection (a row of numbers below), or 'Your own numbers'
//   Delay each way    every message that much later, up and down: the round trip grows by twice this
//   Jitter            …and up to this much more, message by message
//   Loss              the share of messages that wait for a resend, with every message behind them (the wire keeps its order)
//   Stall             nothing passes for half a second (the page plays the ticks it missed at double time) or for three (it catches up as a tab that
//                     slept does), then everything at once
//   Draw your own man ahead   the look ahead (net/foresee.ts FORESEE.on): off = he is drawn where the game has him, a round trip late, to see what it buys
// and a read-out of how the page is doing against the server. Nothing here is read by the simulation: the game's ticks are the server's either way.
import type { Panel } from './panel';
import { NET } from '../net/wire';
import { NETWAY, netTry, netTried, netStall, type NetPlay, type NetTry } from '../net/page';

export const NET_SYSTEM = 'Network';
/** the named kinds: one way's numbers (a round trip is twice the delay). Rough figures for a home connection, not measurements of anybody's. LOSS IS A
 *  SHARE OF MESSAGES, and there are 60 a second each way: at 1.5 in 100 the connection freezes for a moment about once a second each way */
export const NET_KINDS: { name: string; lagMs: number; jitterMs: number; lossPct: number }[] = [
  { name: 'As it is', lagMs: 0, jitterMs: 0, lossPct: 0 },
  { name: 'Same city', lagMs: 10, jitterMs: 4, lossPct: 0 },
  { name: 'Across the country', lagMs: 35, jitterMs: 10, lossPct: 0 },
  { name: 'Across an ocean', lagMs: 80, jitterMs: 15, lossPct: 0 },
  { name: 'Bad wifi', lagMs: 10, jitterMs: 50, lossPct: 1.5 },
  { name: 'Phone hotspot', lagMs: 60, jitterMs: 40, lossPct: 1 },
];
const OWN = 'Your own numbers';
const kindOf = (t: NetTry): number => NET_KINDS.findIndex(k => k.lagMs === t.lagMs && k.jitterMs === t.jitterMs && k.lossPct === t.lossPct);
const tickMs = 1000 / NET.hz;
/** the long stall: well past the ticks from which a page catches up as a seek does (NETPAGE.far), whatever else is on the way */
const STALL_LONG_S = 3;

/** how the page is doing, in words (the read-out row; `__net.state` has the numbers) */
export function netSummary(net: NetPlay): string {
  const s = net.state(), t = netTried(), k = kindOf(t), stall = NETWAY.stallUntil - performance.now();
  const link = k === 0 ? 'the connection as it is' : `${k > 0 ? NET_KINDS[k].name + ': ' : ''}${t.lagMs} ms each way${t.jitterMs ? ` + up to ${t.jitterMs} ms` : ''}${t.lossPct ? `, ${t.lossPct}% resent` : ''}`;
  const role = s.seat < 0 ? 'watching' : `seat ${s.seat + 1}`;
  const state = s.bad ? 'OUT OF STEP with the server' : s.broken ? 'connection lost' : s.bye ? 'the game is over' : s.far ? 'catching up' : s.state;
  const wait = `${s.pending} of your inputs on their way (about ${Math.round(s.pending * tickMs)} ms from a key to the game's answer)`;
  const ahead = !t.predict ? 'your own man is drawn where the game has him' : s.ahead ? `your own man is drawn ${s.ahead.n} tick${s.ahead.n === 1 ? '' : 's'} ahead · ${s.ahead.fixes} correction${s.ahead.fixes === 1 ? '' : 's'}, the biggest ${(s.ahead.fixMax * 100).toFixed(1)} cm` : 'your own man is not ahead right now';
  return `${role} · ${state} · ${s.wordsOk} checks matched\n${link}${stall > 0 ? ` · STALLED for ${(stall / 1000).toFixed(1)} s more` : ''}${s.heldMs >= 1 ? ` · the server's messages are reaching you ${Math.round(s.heldMs)} ms late` : ''}\n${s.seat < 0 ? 'no inputs: this page only watches' : wait}\n${ahead}\n${s.behind} tick${s.behind === 1 ? '' : 's'} waiting to be played${s.seat < 0 || !Number.isFinite(s.lead) ? '' : s.lead > 0 ? ` · your inputs reach the server ${s.lead} tick${s.lead === 1 ? '' : 's'} early (you aim ${s.margin} past it)` : ' · your inputs are reaching the server LATE, so it is repeating your last one'}`;
}

export function netRows(panel: Panel, net: NetPlay): void {
  panel.system(NET_SYSTEM, { icon: '⇄', short: 'Net', title: 'this page\'s connection to the game\'s server: make it worse on purpose, and see how the game holds up' });
  const now = panel.section('Right now', { open: true });
  panel.text(now, () => netSummary(net));
  const sec = panel.section('Make this page\'s connection worse', { open: true, reset: () => { netTry({ lagMs: 0, jitterMs: 0, lossPct: 0 }); netStall(0); }, resetTitle: 'back to the connection as it is' });
  panel.select(sec, 'Connection', [...NET_KINDS.map(k => k.name), OWN], () => { const k = kindOf(netTried()); return k < 0 ? NET_KINDS.length : k; },
    i => { const k = NET_KINDS[i]; if (k) netTry({ lagMs: k.lagMs, jitterMs: k.jitterMs, lossPct: k.lossPct }); },
    'A kind of connection to try. It changes this page only, right away, and stays for this tab until you put it back. The other player\'s page has its own.');
  panel.slider(sec, 'Delay each way (ms)', 0, 400, 5, () => NETWAY.lagMs, v => netTry({ lagMs: v }), 'Every message to and from the server arrives this much later. The round trip grows by twice this.', 0);
  panel.slider(sec, 'Jitter (ms)', 0, 300, 5, () => NETWAY.jitterMs, v => netTry({ jitterMs: v }), 'Each message is late by up to this much more, a different amount every time.', 0);
  panel.slider(sec, 'Loss (%)', 0, 20, 0.5, () => NETWAY.lossPct, v => netTry({ lossPct: v }), 'The share of messages that get lost and have to be sent again. A resent message arrives late by a round trip and a little more, and it holds up every message behind it, so loss feels like short freezes.', 0);
  // (a short one and a long one, on purpose either side of NETPAGE.far: the page gets level again in two different ways, and each button always shows its own)
  panel.buttons(sec, 'Stall', [
    { text: 'Half a second', run: () => netStall(500), title: 'Nothing gets through for half a second, either way, and then everything arrives at once. The page plays what it missed at double speed.' },
    { text: `${STALL_LONG_S} seconds`, run: () => netStall(STALL_LONG_S * 1000), title: `Nothing gets through for ${STALL_LONG_S} seconds, either way, and then everything arrives at once. The page says it's catching up and skips ahead.` }]);
  const la = panel.section('Your own man', { open: true });
  panel.toggleBox(la, 'Draw your own man ahead', () => netTried().predict, v => netTry({ predict: v }), 'On: your own man moves the moment you press a key, ahead of the server. Off: he waits for the server like everything else, so you can feel the difference.', true);
}
