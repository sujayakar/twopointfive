// THE STAND A PROBE IS OWED BY ITS NAME. Some probes were written against a rule's build before it — a card's one gun row, the grab on its key, the knife — and
// the rule since has a check of its own on both stands. Such a probe runs with the newer rule held off on EVERY stand, as the frozen stand holds it. Which probe is
// owed which words is this file's one table, and `asProbe(name, fn)` is the one way a probe is run under them: the runner (probes.ts run) calls it for every probe,
// and a check file that can be run by itself calls it from its own main (`asProbes` for a file of several). So `bun run tools/qa/<x>-check.ts` and
// `bun run tools/qa/probes.ts <x>` are one stand. (They were two: the runner held the words, the file's own main did not, and a check was red run as a file and
// green in its family — sticky-check and reachup-check were both found so on one day, and twenty more files stood the same way.)
// holds-check.ts holds this file to its word: a new row here, or a new main in a file that holds a probe of the table, is red there until that main runs the
// probe through this file by its name. (A check file none of whose probes is in the table is not read there: run alone its parts have no fence between them.)
//
// A hold is a stand word set to '0' (src/host/standup.ts reads the word as each game is stood up) and, where the probe reads the rule's table before it stands a
// game up, the table's row put off as well. A word whoever runs the tool set himself wins: `TPS_STALK=1 bun run tools/qa/reachup-check.ts` asks the check with
// the stalk on (and `TPS_STALK=0` in front is the hold itself, said by hand: a stand-up raises a row for '1' alone).
import { resolve } from 'node:path';
const ROOT = resolve(import.meta.dir, '../..');   // (not the harness's own: a tool that names its stand before it loads the harness can still import this file first)

/** One held row: the stand word, the knob table and the row's key (src/game/knobs.ts's registry), and whether the row is put off at once as well as the word
 *  (`now`: a probe that reads the table before it stands a game up) or by the word alone, at the next stand-up. */
export type Hold = { readonly word: string; readonly table: string; readonly key: string; readonly now: boolean };
const row = (word: string, table: string, key = 'on', now = true): Hold => ({ word, table, key, now });
const atStandUp = (word: string, table: string, key = 'on'): Hold => row(word, table, key, false);

/** The table: the probe names (as probes.ts dispatches them) and what each is owed. A name can stand in several rows. `list` is the row's name: the one the
 *  area files (docs/agents) call it by, from when each row was a list of its own in probes.ts. A name is written one of three ways: 'reachup' = that probe;
 *  'close-*' = every probe whose name begins so; 'sticky(-*)' = both (the probe of that name and every probe that begins with it and a dash). */
export const HOLDS: readonly { readonly list: string; readonly names: readonly string[]; readonly holds: readonly Hold[] }[] = [
  // THE ORDER MENU'S ROWS AS THEY WERE. These probes list a menu's rows key by key and press them — the pair has no 'wait for my word' row, a floor menu has
  // none, F on a door's first page turns to the tools, F on a greyed row of its own is refused. src/game/planlay.ts (PLANLAY.on, on in the page) puts that row
  // under every menu and makes it LAY the order; the row as the page has it is planlay-check.ts's, on both stands.
  { list: 'ROWS_AS_THEY_WERE', names: ['squadplay(-*)', 'thresholdmenu(-*)', 'orderkeys(-*)', 'pressorders(-*)', 'doormenu(-*)', 'doortools-menu(-*)', 'orders-menu(-*)'], holds: [atStandUp('TPS_PLANLAY', 'PLANLAY')] },
  // WHO TAKES AN ORDER, AS IT WAS. These probes step who the order goes to with T and send red to one door and blue to another by the quick key, and they
  // list a teammate's own menu. src/game/cmdring.ts CMDRING.wholeSquad (on in the page) makes every order of the squad's key or its ring everyone's and
  // takes the choice away; the rule as the page has it is wholesquad-check.ts's, on both stands. (Measured on the page's stand: squadplay 31 FAIL lines with
  // the rule on and 1 with it held off, orderkeys 49 and 30 — the counts they had before the rule went in; pressorders reads the same either way and is not
  // on this list. doormenu-man asks that a teammate under the cursor has his own menu on X and is the man on T: the choice the rule takes away. The family's
  // other lines ask the page's rule: doormenu-probes.ts sweep counts a teammate under the cursor as the squad there.)
  { list: 'WHO_AS_IT_WAS', names: ['squadplay(-*)', 'orderkeys(-*)', 'doormenu-man(-*)'], holds: [atStandUp('TPS_WHOLESQUAD', 'CMDRING', 'wholeSquad')] },
  // THE KIT CARDS AS THEY WERE. These probes set a wall charge or the breaching shotgun through rows of their own and count pouches for them — the
  // card of the build before the big slot (src/game/kit.ts BIGSLOT, on in the page: one big row, the shotgun off the weapon row, big things free of pouches).
  // The card as the page has it is bigslot-check.ts's, on both stands. (owncharge-kit reads both cards itself: the rows a thing first, the big slot at its foot.)
  { list: 'CARDS_AS_THEY_WERE', names: ['kit(-*)', 'pouch(-*)', 'shotgun(-*)', 'squadsmoke(-*)'], holds: [row('TPS_BIGSLOT', 'BIGSLOT')] },
  // THE CARD'S ONE GUN ROW. These probes read or step a card's gun through its 'weapon' row (and hold that no field without a tile moves under a paste) — the
  // card of the build before the two gun slots (src/game/gunslots.ts GUNSLOTS, on in the page: a Primary and a Secondary row where the gun row stood, `weapon`
  // following the primary). The card as the page has it is gunslots-check.ts's, on both stands. (The probes of the kit cards' row above need no word: the
  // slots ride on the big slot.)
  { list: 'GUN_ROW_AS_IT_WAS', names: ['bigslot(-*)', 'kitpaste(-*)', 'kitlent(-*)', 'kitcan(-*)', 'sticky(-*)'], holds: [row('TPS_GUNSLOTS', 'GUNSLOTS')] },
  // YOUR CARD AS IT WAS. pouch-table reads a card's notes and refusals word for word ("no FN2000, no launcher …") and holds every row of every card against a
  // fixture written from the build before the pouches rule. Rules since, each on in the page, change what it reads: the 2010s issue kit renames the rifle
  // and the pistol in those words (src/game/gear.ts GEAR), your own card gains a door charge row (owncharge.ts OWNCHARGE), a camera row (stickycam.ts STICKY)
  // and a wand row (wand.ts WAND), and a card's note names the launcher's key as '2 again' (slotsetup.ts loadKeys). Each has a check of its own for its row.
  { list: 'CARD_AS_IT_WAS', names: ['pouch-table'], holds: [row('TPS_GEAR', 'GEAR'), row('TPS_OWNCHARGE', 'OWNCHARGE'), row('TPS_STICKY', 'STICKY'), row('TPS_WAND', 'WAND'), row('TPS_SLOTSETUP', 'SLOTSETUP')] },
  // THE CARDS IN THE TABLE'S LAYOUT. These probes read a teammate's card of a team of three or four as the table writes it — Red One full as issued, the Blue
  // men empty — to press a pouch too many, to trade one across two cards, to count a stored card. Since the squad's kit is dealt over the men who come (src/game/
  // kit.ts KITDEAL, on in the page) a card reads as it is WORN, and that is kitdeal-check.ts's to hold, on both stands, with a seeded walk of presses of its own.
  { list: 'DEAL_AS_IT_WAS', names: ['kit(-*)', 'pouch(-*)', 'shotgun(-*)', 'squadsmoke(-*)', 'bigslot-card', 'wand-mate'], holds: [row('TPS_KITDEAL', 'KITDEAL')] },
  // THE CARD'S ROWS IN THE ORDER THEY HAD. These probes read where a row stands on a card ('the last of your kit rows') and which tiles a mission leaves idle —
  // the card before it was laid out as a tree (src/game/kittree.ts KITTREE, on in the page: what rides on a gun under it, your light and goggles as rows). The
  // tree's rows are kittree-check.ts's, on both stands.
  { list: 'ROWS_BEFORE_THE_TREE', names: ['wand-kit', 'wand-tile'], holds: [row('TPS_KITTREE', 'KITTREE')] },
  // THE WAYS THE MEN WALKED. Each of these stages a jam it then shows mended — two men sent to one corner who never settle, a man walked across the player — by
  // sending the team over floor where things stand by a door, and reads the jam with its own rule off as its proof that the staging bites. Since a teammate's way
  // goes round a movable thing by a door (src/game/orders.ts ORDERS.propRound, on in the page) the men come by another way and that jam does not happen: the
  // line that shows the fault is red, the lines that show the mend are as they were. The way round is propround-check.ts's, on both stands.
  { list: 'WAYS_AS_THEY_WERE', names: ['filespread', 'noshove'], holds: [atStandUp('TPS_PROPROUND', 'ORDERS', 'propRound')] },
  // …and the pace a teammate takes off the player (src/game/teampace.ts TEAMPACE.pacing, on in the page: his walk is the player's own, not the flat 0.8 of it)
  // for two probes whose FAULT arm was staged at the flat walk: leafstall's found case (rule off, he treads against the back of the open leaf for a second and
  // a half; at the player's walk he is through it in 0.7 s) and noshove's room keeper (rule off, the entry's timing stands a man across the player and the
  // keeper walks out of his way; at the player's walk the entry lands otherwise). Their mend lines are green with the pace on (TPS_PACING=1 in front shows it:
  // only the fault arm goes red).
  { list: 'PACE_AS_IT_WAS', names: ['leafstall', 'noshove'], holds: [atStandUp('TPS_PACING', 'TEAMPACE', 'pacing')] },
  // …and the deal's key `more` (kit.ts KITDEAL.more, on in the page: a team of three or four as issued is LENT a big tool each pair) for the probes written
  // of a four whose big slots stand empty: a locked door that such a four picks, boots or is refused at is, with a lent shotgun or hammer, a door the tool
  // takes — and that is kitdeal-check.ts's and bigslot-check.ts's to hold. The same for the probes that read a door's tools page row for row against the
  // pair's: with a tool in the squad the page has the big slot's row on its own key, and the walk that reads that page as the page opens it, in every
  // scenario, is orderkeys-walk. (kitpaste: its squads' big slots are the table's; a paste over a lent tool is kitdeal-more's line. squadplay-wait, -side: a
  // four's door tools page is read row for row, and the lent shotgun adds 'H shotgun and clear'.)
  { list: 'MORE_AS_IT_WAS', names: ['frameholds-tools', 'doortest', 'doormenu-rows', 'thresholdmenu-rows', 'shutdoor-menu', 'silent-pick', 'silent-tools', 'silent-boot', 'pressorders-squad4', 'pressorders-nightbreach4', 'squadplay-wait', 'squadplay-side', 'kitpaste(-*)'], holds: [row('TPS_KITMORE', 'KITDEAL', 'more')] },
  // PROBES WRITTEN AGAINST ONE CARD FOR EVERY MISSION (kit.ts KITDEAL.stealth): on the page's stand a stealth run wears your STEALTH card, issued as the
  // mission's kit, and a sponge round rides five to a pouch — so a probe that changes the loud card and stages a quiet run alone, or prices sponge rounds a
  // pouch each, runs with the key held off. The rule's own lines are kitstealth's. (owncharge puts its charges on the card a night alone wears: it reads both.
  // frag-player, he40-* and burst-breaks stand a night alone and ask the grenade and the 40 mm HE round themselves, which only the loud card issues.
  // kitanim-midswap: its night alone throws a frag after a smoke, and the stealth card as issued carries no frag.)
  { list: 'STEALTH_AS_IT_WAS', names: ['kittree(-*)', 'frag-player(-*)', 'he40(-*)', 'burst-breaks(-*)', 'kitanim-midswap'], holds: [row('TPS_KITSTEALTH', 'KITDEAL', 'stealth')] },
  // THE GUN'S KEYS AS THEY WERE. These probes step the launcher's load with the rifle's digit pressed again (2 with it up: ring to the grenade, to the
  // camera) — the build before a gun's key again opens its setup (src/game/slotsetup.ts SLOTSETUP, on in the page: the load is the setup's row then, 2 2).
  // (weapons-identity steps nothing: it reads the kit column against the words it was recorded with, and with the switch on the column is the box.)
  // The setup's own lines are slotsetup-check.ts's, on both stands.
  { list: 'GUN_KEYS_AS_THEY_WERE', names: ['rifle-reload(-*)', 'kit-slots(-*)', 'he40(-*)', 'sticky(-*)', 'weapons-identity(-*)'], holds: [row('TPS_SLOTSETUP', 'SLOTSETUP')] },
  // THE GRAB ON ITS KEY, AND THE KNIFE. These probes tap the grab's row beside a man, read the two-button blow (left = the knife), the hand on the hilt or the
  // knife in an arm hold — the build before the held click became the only way to a man (src/game/stalk.ts STALK, on in the page) and before the knife went
  // (src/game/close.ts CLOSE.noKnife, on in the page). The new rules' own lines are stalk-check.ts's and noknife-check.ts's, on both stands. And the right
  // button as it was: these probes' one-frame right click is the knockout or the arm hold's whole choke, and they read the row's words (CLOSE.rmbHold; its own
  // lines are rmbhold-check.ts's). (grab-talk, grab-choke, grab-auto, hold-draw, selector-input, selector-noise and the gunhold-* probes take their man by the
  // row, several of them with the pistol already out — a way in the page no longer has: the click needs empty hands, and the pistol comes out in the hold.
  // kitanim-close reads the hand on the hilt. stealthpose-creep reads the free hand carried low BEHIND a man and out of reach — with the stalk that stretch is
  // the way in's own, the arm coming up by the distance.)
  { list: 'GRAB_ROW_AS_IT_WAS', names: ['handslow', 'heldclicks', 'reachup', 'hold-draw', 'kitanim-close', 'grab-talk', 'grab-choke', 'grab-auto', 'selector-input', 'selector-noise', 'stealthpose-creep', 'close-*', 'gunhold-*'], holds: [row('TPS_STALK', 'STALK'), row('TPS_NOKNIFE', 'CLOSE', 'noKnife'), row('TPS_RMBHOLD', 'CLOSE', 'rmbHold')] },
  // …THE KNIFE AND THE RIGHT BUTTON for three more, which take their man another way. (takedown-kill: the knife's own path across a neck — measured against a
  // man held with the knife; with none he is arched further onto the arm, HOLD.bareArch, and there is no kill. haul: its head gap at the height of the lift was
  // taken on that hold too — 19 cm bare against the 22 it asks. takedown-beats: it reads the held man's right hand coming ONTO the arm as the choke begins — with
  // no knife it is there from the start, HOLD.bareGrip.)
  { list: 'KNIFE_AS_IT_WAS', names: ['takedown-beats', 'takedown-kill', 'haul'], holds: [row('TPS_NOKNIFE', 'CLOSE', 'noKnife'), row('TPS_RMBHOLD', 'CLOSE', 'rmbHold')] },
  // THE GRAB THAT DISARMS. takedown-beats grabs the same man twice and holds his wrists, frame for frame, to the first grab's: the first time the gun is taken
  // out of his hand (character.ts HOLD.disarm, on in the page), the second he has none. Held at the build before the move; the move's own lines are
  // disarm-check.ts's, on both stands.
  { list: 'DISARM_AS_IT_WAS', names: ['takedown-beats'], holds: [row('TPS_DISARM', 'HOLD', 'disarm')] },
];

/** whether a name written in the table (one of its three ways) is this probe's */
export function named(entry: string, probe: string): boolean {
  if (entry.endsWith('(-*)')) { const b = entry.slice(0, -4); return probe === b || probe.startsWith(`${b}-`); }
  return entry.endsWith('-*') ? probe.startsWith(entry.slice(0, -1)) : probe === entry;
}
/** What the probe of this name is owed: every row of the table its name stands in, a word once. */
export function holdsOf(name: string): Hold[] {
  const out: Hold[] = [];
  for (const r of HOLDS) if (r.names.some(e => named(e, name))) for (const h of r.holds) if (!out.some(o => o.word === h.word)) out.push(h);
  return out;
}
/** One row of the table by its name, for a check that holds a rule off round a block of its own lines the way a probe of that row is held. */
export function heldList(list: string): readonly Hold[] { const r = HOLDS.find(x => x.list === list); if (!r) throw new Error(`holds.ts: no row '${list}' (${HOLDS.map(x => x.list).join(' ')})`); return r.holds; }

/** Put these holds on and hand back what takes them off again. (Not headless.ts's `hold`, which is the Rust rows' arm of a check.) A word nobody set is set to '0' (and deleted after); a word whoever runs the tool set himself is
 *  left as he set it — '1' is his to ask and nothing of that row is touched; anything else is the hold itself, as the stand-up reads it (a game stood up has
 *  the row on for '1' alone), so the row is put off all the same: a child of a held probe stands where its parent stands. */
export async function holdOff(holds: readonly Hold[]): Promise<() => void> {
  const env = process.env, { knobRegistry } = await import(`${ROOT}/src/game/knobs.ts`), R = knobRegistry() as Record<string, Record<string, unknown> | undefined>;
  const put: (() => void)[] = [];
  const back = (): void => { for (const b of put.reverse()) b(); put.length = 0; };
  for (const h of holds) { const given = env[h.word];
    const T = R[h.table]; if (!T || typeof T[h.key] !== 'boolean') { back(); throw new Error(`holds.ts: ${h.table}.${h.key} (${h.word}) is no switch of the knob registry — a retired row is taken out of the table`); }
    if (given === '1') continue;
    const was = T[h.key]; if (given === undefined) env[h.word] = '0'; if (h.now) T[h.key] = false;
    put.push(() => { if (given === undefined) delete env[h.word]; if (h.now) T[h.key] = was; }); }
  return back;
}

/** Run one probe as the runner runs it, whoever calls: the words its name is owed held (holdOff, above), and round it the fence that keeps one probe's leftovers
 *  out of the next of the same process — every knob table copied before and put back after (src/game/knobs.ts: a key a probe added is dropped, every value
 *  restored in place; the cut-pieces switch is one of them), 'the plain night wears your card' (kit.ts KIT_SOLO, a flag of the visit in no knob table), and the
 *  loud card first in the table again (kit.ts kitMission: a probe that staged a quiet run leaves your stealth card first). Throws, after everything else is
 *  back, if the tables cannot be put back. */
export async function asProbe<T>(name: string, fn: () => T | Promise<T>): Promise<T> {
  const { knobsCopy, knobsBack } = await import(`${ROOT}/src/game/knobs.ts`), kit = await import(`${ROOT}/src/game/kit.ts`);
  const soloOn = kit.KIT_SOLO.on, knobs = knobsCopy(), release = await holdOff(holdsOf(name));
  let threw: unknown, failed = false;
  try { return await fn(); }
  catch (e) { failed = true; threw = e; throw e; }
  finally {
    try { kit.kitMission(null); } catch { /* (as the runner always ran it: a throw here is not the probe's verdict) */ }
    release(); kit.KIT_SOLO.on = soloOn;
    try { knobsBack(knobs); }
    catch (e: any) { throw new Error(`the knob tables could not be put back after '${name}' — ${e?.stack ?? e}${failed ? ` (and the probe itself threw — ${(threw as any)?.stack ?? threw})` : ''}`); }
  }
}

/** A check file's own main: its probes under the names the runner calls them by, each run as the runner runs it. `want` = the names asked for (none = all, in
 *  the table's order; a main hands it `wanted()`), each whole or without its family ('kit' for 'sticky-kit'). A probe that throws is said and the rest go on, as under the runner. A name
 *  the file has not is said and nothing is run: null (exitCode: 2), so that a mistyped name is never a green run of nothing. ONE THING THE RUNNER DOES MORE: it
 *  also fails a probe that prints a FAIL or an error line and returns true; a file's own verdict is what its probes return. */
export async function asProbes(probes: Readonly<Record<string, () => boolean | Promise<boolean>>>, want: readonly string[] = []): Promise<boolean | null> {
  const names = Object.keys(probes), short = (n: string): string => n.slice(n.indexOf('-') + 1);
  const picked = want.map(w => names.find(n => n === w) ?? names.find(n => short(n) === w)), unknown = want.filter((_, i) => !picked[i]);
  if (unknown.length) { console.log(`unknown: ${unknown.join(' ')} (one or more of: ${names.join(' ')})`); return null; }
  let ok = true;
  for (const name of names) { if (want.length && !picked.includes(name)) continue;
    let r = false; try { r = await asProbe(name, probes[name]); } catch (e: any) { console.log(`error: probe '${name}' threw — ${e?.stack ?? e}`); }
    ok &&= r; }
  return ok;
}
/** the names asked for on a command line: its words that are no flag */
export function wanted(argv: readonly string[] = process.argv.slice(2)): string[] { return argv.filter(a => !a.startsWith('-')); }
/** a main's exit code, the runner's own: 0 all passed · 1 one failed · 2 a name the file has not */
export function exitCode(ok: boolean | null): number { return ok === null ? 2 : ok ? 0 : 1; }
