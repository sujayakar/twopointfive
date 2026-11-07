// WHAT WAS SAID, READ FROM THE GAME'S OWN STATE — for a check that asks what the player was told or what a man said.
//
// Until 2026-11-07 a check learned this by wrapping two functions on the game object it holds: `game.msg = t => { said.push(t); msg(t); }` and the same for
// `game.say`. That object is the tools' copy of the game since the page plays the Rust game, and the Rust tick calls neither wrap: a line is said INSIDE the
// Rust game and arrives here as state. So every "and he is told …" line read an empty list and failed, whatever the game had said.
// What does arrive, every step, is what the page itself draws from:
//   game.messages   the log's last rows ({ text, t }, the newest five)                       → `said`
//   a man's bubble  ({ text, t, radio }: his last line, over his head or over the net)       → `spoke` ({ who, text, radio })
// `listen(game)` keeps both as lists that only grow. Call `poll()` once after every step (a rig's own step is the place): the log keeps five rows and a bubble
// one line, so a check that steps many frames between polls can miss what was said in between. Nothing here writes to the game.
export interface Heard {
  /** every row the log has shown since `listen`, in order */
  said: string[];
  /** every line a man has said since `listen` (`who` = his callsign, or his name, or '?'; `man` = the man himself, as the game's copy has him) */
  spoke: { who: string; text: string; radio: boolean; man: unknown }[];
  /** read the game's state now: call it after every step */
  poll(): void;
}

export function listen(game: any): Heard {
  const said: string[] = [], spoke: Heard['spoke'] = [];
  // (the log's rows carry the game's time and are in its order; the copy's row objects are made again every step, so a row is known by its time and its words)
  let lastT = -Infinity, atLast: string[] = [];
  const bubbleAt = new Map<unknown, string>();
  const H: Heard = { said, spoke,
    poll() {
      for (const m of (game.messages ?? []) as { text: string; t: number }[]) {
        if (m.t < lastT) continue;
        if (m.t === lastT) { if (atLast.includes(m.text)) continue; atLast.push(m.text); } else { lastT = m.t; atLast = [m.text]; }
        said.push(m.text);
      }
      (game.guards as any[]).forEach((gd, i) => {
        const b = gd.bubble, key = gd.id ?? gd.callsign ?? i, is = b ? `${b.t}|${b.text}` : ''; if (!b || bubbleAt.get(key) === is) return;
        bubbleAt.set(key, is); spoke.push({ who: gd.callsign ?? gd.name ?? '?', text: b.text, radio: !!b.radio, man: gd });
      });
    } };
  H.poll();   // (what stood in the log before the listening began is not news: it is taken as seen, and dropped)
  said.length = 0; spoke.length = 0;
  return H;
}
