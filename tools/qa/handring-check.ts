// HIS OWN ACTIONS ARE A RING THAT STAYS — the check behind HANDRING.on (src/game/handring.ts; the rule in interact.ts updateInteract, the whistle's row in
// player.ts buildInteractables) and the page's board for it (src/host/keymap.ts `handWhen`, HAND). ON in the page (?handring=0 = the list on a held Space,
// the whistle on its key, as it was), OFF in the module and on the frozen stand (TPS_HANDRING=1).
//   bun run tools/qa/probes.ts handring        (family `player`)        HANDRING_BREAK=1 = the rule and the board's layer left off and the same lines asked: must FAIL
//                                                                         HANDRING_BREAK=lone | loneWrap | bare | onThing | turn | corner = that one rule left off with the rest on: its own block must FAIL
// the maintainer, 10-22: "radial menu for our own actions, remove existing menu in top left and awkward scrolling thing", "move whistle to our actions and
// remove v binding", "move our actions to E", "i think same would go for our actions (1-5 with the ring open)".
// WHAT IS ASKED:
//   the tap       at a door a tap does the top row (the leaf moves) and puts no ring up; on open floor a tap does nothing (the whistle is never a tap)
//                 but say so, once: 'nothing in reach' (the key looked dead); a second tap straight after says no more, and with the rule off none is said
//   beside a thing a tap that finds no row beside a thing names the thing and why (handring.ts `near`; the maintainer on the stealth mission: "nothing in
//                 reach to close the door?"): a stride short of a door 'door · step closer', every tap (one line of it in the log); on the quiet tutorial mission, at the open back
//                 door with shutting a leaf taken off its frame's rows (as it was when he played it), 'door · not in this mission' (from a step short of it too); sprinting inside a door's reach 'door · stop sprinting'; a second tap with
//                 the leaf in his hand 'door · not now'; open floor, a door behind a wall and a door with a living man nearer still 'nothing in
//                 reach'; `near` off, the old word everywhere
//   the ring      the key kept down: nothing before HANDRING.holdSecs, then the ring with a wedge for the door's row and the whistle on the LAST wedge; let
//                 go in the middle and it stays with no key held; his feet are his own under it
//   early         a digit pressed with the key down and the ring not yet up (inside the hold) is its wedge at once, never the slot or the gun's setup
//   a digit       standing, the last wedge's digit whistles at once and the ring is gone — and it was no weapon slot; a digit with no wedge does nothing
//                 and the ring stays; the whistle's digit, or a click, before the breath is back lights it and the ring stays
//   a click       standing, with the pistol up: a left click does the lit wedge and no round leaves the gun
//   the cursor    the key kept down, the cursor out toward the last wedge the moment the ring is up, the key let go: that wedge is done (no second press)
//   under it      a ring that merely stands is nothing in hand: the wheel is his pace, the squad's key is free, a save is allowed
//   a save        a save kept in memory under a standing ring is put back with no ring (savestate.ts loadMem): his first click after is his own
//   put away      the key again does the lit wedge; a right click puts the ring away with nothing done; so does the back key
//   the rows      the whistle is a wedge and no row: the list and the highlight are as they were (nothing that reads them meets it); on open floor the
//                 ring is that one wedge; the whistle's old code still whistles (a recording, a check)
//   off           the rule off: the held key opens the list as it did (no ring), the digits are his slots
//   the board     while the page says so (and empty hands are a slot): the E key gives the interaction's code, the Space key and the V key give nothing,
//                 'Space  open' reads 'E  open' and 'E puts it away' still reads as the slot's key; without the slot, or with the rule off, Space and V are
//                 itself and V still gives nothing (the whistle is the ring's with or without the slot); the whistle's row can be given a key; a
//                 lesson's line for the whistle reads in the ring's words
//   recordings    a recording carries the switch; one from before it replays with it off; the frozen stand holds it off by name
// Headless: rows, the ring's view, what was done. The corner's one line and the ring as drawn are the page's (never seen on a GPU). NOBODY HAS PLAYED IT.
import { ROOT } from './headless.ts';
import { world } from './wallsnap-probes.ts';
export async function handRingCheck(): Promise<boolean> {
  const BRK = process.env.HANDRING_BREAK === '1', ONE = process.env.HANDRING_BREAK ?? '', left = (k: string) => BRK || ONE === k; let all = true;
  if (ONE && !BRK && !['lone', 'loneWrap', 'bare', 'onThing', 'turn', 'corner'].includes(ONE)) { console.log(`HANDRING_BREAK=${ONE}: not 1 and not a rule's name (lone, loneWrap, bare, onThing, turn, corner)`); return false; }   // (HANDRING_BREAK=lone|loneWrap|bare|onThing|turn|corner: that one rule left off, the rest on — its own block must fail; `lone` fails the long label's block too, whose cursor is read on a pill)
  const verdict = (ok: boolean, what: string, detail: string) => { all &&= ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${what}\n    ${detail}`); };
  console.log(`\n=== handring: his own actions are a ring that stays${BRK ? ' (the rule and the board\'s layer off: must fail)' : ONE ? ` (HANDRING.${ONE} off, the rest on: its block must fail)` : ''} ===`);
  const { HANDRING } = await import(`${ROOT}/src/game/handring.ts`); const { HANDSLOT } = await import(`${ROOT}/src/game/handslot.ts`); const { CMDRING } = await import(`${ROOT}/src/game/cmdring.ts`);
  const { WALLSNAP } = await import(`${ROOT}/src/game/wallsnap.ts`); const { handRingView } = await import(`${ROOT}/src/game/interact.ts`);
  const { WHISTLE_STATS, WHISTLE } = await import(`${ROOT}/src/game/whistle.ts`); const { BACKKEY } = await import(`${ROOT}/src/game/backkey.ts`);
  const { knobsSnapshot, applyKnobs } = await import(`${ROOT}/src/game/journal.ts`); const { FROZEN_HOLDS } = await import(`${ROOT}/src/host/profile.ts`);
  const K = await import(`${ROOT}/src/host/keymap.ts`); const { TWOGEARS } = await import(`${ROOT}/src/game/player.ts`);
  const { playerBusy } = await import(`${ROOT}/src/game/savestate.ts`);
  const was = { on: HANDRING.on, early: HANDRING.early, empty: HANDRING.empty, near: HANDRING.near, lone: HANDRING.lone, loneWrap: HANDRING.loneWrap, onThing: HANDRING.onThing, bare: HANDRING.bare, turn: HANDRING.turn, corner: HANDRING.corner, back: BACKKEY.tab, slot: HANDSLOT.on, cmd: CMDRING.on, hands: CMDRING.hands, ws: WALLSNAP.on, press: WALLSNAP.press, kOn: K.KEYMAP.on, slotWhen: K.KEYMAP.slotWhen, handWhen: K.KEYMAP.handWhen, qFree: K.KEYMAP.qFree, qCmd: K.KEYMAP.qCmd };
  const log = console.log, err = console.error, warn = console.warn;
  const quiet = async <T>(f: () => T | Promise<T>): Promise<T> => { console.log = console.error = console.warn = () => { }; try { return await f(); } finally { console.log = log; console.error = err; console.warn = warn; } };
  const G: any = globalThis, hadLS = G.localStorage; const store = new Map<string, string>();
  G.localStorage = { getItem: (k: string) => store.get(k) ?? null, setItem: (k: string, v: string) => { store.set(k, String(v)); }, removeItem: (k: string) => { store.delete(k); } };
  try {
    const S = await quiet(() => world(true)); const { pl, g, step, tap, place, input, click, keys, setCursor } = S as any; WALLSNAP.press = false; CMDRING.on = false; CMDRING.hands = false; BACKKEY.tab = true;   // (the back key is Tab, as the page has it)
    const rule = (on: boolean) => { HANDRING.on = HANDRING.early = HANDRING.empty = HANDRING.near = on && !BRK; HANDRING.lone = on && !left('lone'); HANDRING.loneWrap = on && !left('loneWrap'); HANDRING.onThing = on && !left('onThing'); HANDRING.bare = on && !left('bare'); HANDRING.turn = on && !left('turn'); HANDRING.corner = on && !left('corner'); };
    const KEY = 'Space', LAST = CMDRING.wedges, HOLD = Math.ceil(HANDRING.holdSecs * 60);
    const door = g.doors.list.find((d: any) => d.def.name === 'conference'), fc = door.frameCentre;
    const rows = () => g.nearby.map((r: any) => `${r.kind}:${r.verb ?? '-'}`).join(' | ') || 'none';
    const view = () => { const v = handRingView(g); return v ? `ring [${v.wedges.map((w: any) => `${w.key} ${w.text}`).join(' · ')}] lit ${v.lit}${pl.sel.ring?.free ? ' standing' : ''}` : 'no ring'; };
    const up = () => !!handRingView(g), standing = () => !!pl.sel.ring?.free && pl.sel.open;
    const down = (n: number) => { input.pressed.add(KEY); input.keys.add(KEY); step(n); }, lift = (n = 2) => { input.keys.delete(KEY); step(n); };
    /** the ring up and standing: the key kept down past the hold, let go with the cursor in its middle */
    const stand = () => { down(HOLD + 4); lift(); };
    const breath = () => step(Math.ceil(WHISTLE.cooldown * 60) + 10);   // (the whistle's breath back)
    const floor = (kit: 'pistol' | 'rifle' | 'empty' = 'empty') => place(30, 2.4, 180, kit), atDoor = () => place(fc[0], fc[2] + 1.0, 180);
    const leaf = () => door.angle as number;
    // ---- the tap
    { rule(true); atDoor(); const r0 = rows(), a0 = leaf(); tap(KEY); step(1); const noRing = !up(); step(40); const moved = Math.abs(leaf() - a0) > 0.05;
      const words = () => g.messages.filter((m: any) => m.text === 'nothing in reach').length as number;
      rule(false); floor(); g.messages.length = 0; tap(KEY); step(20); const offWord = words(); rule(true);
      floor(); const r1 = rows(), b0 = WHISTLE_STATS.blown; g.messages.length = 0; tap(KEY); step(20); const quietTap = WHISTLE_STATS.blown === b0, word1 = words(); tap(KEY); step(20); const word2 = words();
       }
    // ---- the ring
    { rule(true); atDoor(); down(HOLD - 4); const early = up(); step(8); const v1 = view(), V = handRingView(g), hasDoor = !!V?.wedges.some((w: any) => w.key === 1), hasWhistle = !!V?.wedges.some((w: any) => w.key === LAST && /whistle/.test(w.text));
      lift(); const stays = standing(), v2 = view(); const z0 = pl.char.pos[2]; keys('KeyW'); step(30); keys(); const walked = Math.abs(pl.char.pos[2] - z0) > 0.2, still = standing(); tap('Tab'); step(2);
      verdict(!early && hasDoor && hasWhistle && stays && walked && still, 'the key kept down puts the ring up after the hold, the whistle on the last wedge; let go in the middle it stays; he can walk under it',
        `before the hold: ring ${early} · after: ${v1} · let go: ${v2} · W for half a second under it moved him ${Math.abs(pl.char.pos[2] - z0).toFixed(2)} m, the ring still standing ${still}`); }
    // ---- a digit inside the hold
    { rule(true); floor('pistol'); breath(); const s0 = pl.slot, b0 = WHISTLE_STATS.blown; down(6); const notYet = !up(); tap(`Digit${LAST}`); step(2); const blew = WHISTLE_STATS.blown - b0, kept = pl.slot === s0 && !pl.holstered && !pl.setup, gone = !up(); lift(); step(4);
      atDoor(); place(fc[0], fc[2] + 1.0, 180, 'pistol'); step(4);  down(6); tap('Digit1'); step(2);  lift(); step(4); tap('Tab'); step(2);
      HANDRING.early = false; floor('pistol'); breath();  down(6); tap(`Digit${LAST}`); step(2);  lift(); step(4); tap('Tab'); step(2);
       }
    // ---- a digit
    { rule(true); floor('pistol'); breath(); stand(); const s0 = pl.slot, b0 = WHISTLE_STATS.blown; tap('Digit3'); step(3); const none = up() && WHISTLE_STATS.blown === b0 && pl.slot === s0;
      tap(`Digit${LAST}`); step(3); const blew = WHISTLE_STATS.blown - b0, gone = !up(), slot = pl.slot === s0 && !pl.holstered;
      stand(); tap(`Digit${LAST}`); step(3); const grey = up() && WHISTLE_STATS.blown - b0 === 1, canSave = !/choosing/.test(String(g.saves?.why?.() ?? '')); click(); step(3); const greyClick = up() && WHISTLE_STATS.blown - b0 === 1; tap('Tab'); step(2); void canSave;
      verdict(none && blew === 1 && gone && slot && grey && greyClick, 'standing, a digit does its wedge at once and is no weapon slot; a digit with no wedge does nothing; a wedge that cannot be done now is lit and the ring stays',
        `3 (no wedge): the ring up ${none} · ${LAST}: whistled ${blew}, the ring gone ${gone}, the pistol still in hand ${slot} · ${LAST} again before the breath is back: the ring stays, nothing blown ${grey} · a click then: the same ${greyClick}`); }
    // ---- a click
    { rule(true); floor('pistol'); breath(); stand(); const b0 = WHISTLE_STATS.blown, r0 = pl.pistol.mag + (pl.pistol.chamber ? 1 : 0), lit = handRingView(g)?.lit; click(); step(6); const r1 = pl.pistol.mag + (pl.pistol.chamber ? 1 : 0);
      verdict(WHISTLE_STATS.blown - b0 === 1 && r1 === r0 && !up() && lit === LAST, 'standing, a left click does the lit wedge and no round leaves the gun',
        `lit ${lit} · the click whistled ${WHISTLE_STATS.blown - b0} · rounds ${r0} → ${r1} · the ring gone ${!up()}`); }
    // ---- the cursor
    { rule(true); atDoor(); breath(); const a0 = leaf(), b0 = WHISTLE_STATS.blown; setCursor(null); down(HOLD + 2); const R = pl.sel.ring, lit0 = handRingView(g)?.lit;   // (the cursor goes out the moment the ring is up: the hold was the wait)
      const ang = (-90 + (LAST - 1) * 360 / LAST) * Math.PI / 180; if (R) { input.mouseX = R.ax; input.mouseY = R.ay; step(1); input.mouseX = R.ax + Math.cos(ang) * 70; input.mouseY = R.ay + Math.sin(ang) * 70; } step(3); /* (from its middle, wherever it came up: on the thing with handring.ts onThing, at the cursor without) */ const lit1 = handRingView(g)?.lit; lift(3);
      verdict(lit0 === 1 && lit1 === LAST && WHISTLE_STATS.blown - b0 === 1 && !up() && Math.abs(leaf() - a0) < 0.01, 'the key kept down, the cursor out toward the last wedge, the key let go: that wedge is done',
        `lit ${lit0} → ${lit1} with the cursor out · let go: whistled ${WHISTLE_STATS.blown - b0}, the ring gone ${!up()}, the leaf did not move ${Math.abs(leaf() - a0) < 0.01}`); }
    // ---- put away
    { rule(true); floor(); breath(); stand(); const b0 = WHISTLE_STATS.blown; tap(KEY); step(3); const again = WHISTLE_STATS.blown - b0, g1 = !up(); input.keys.delete(KEY);
      breath(); stand(); const b1 = WHISTLE_STATS.blown; input.clicked |= 4; step(3); const rmb = !up() && WHISTLE_STATS.blown === b1;
      stand(); const u2 = up(); tap('Tab'); step(3); const tab = u2 && !up() && WHISTLE_STATS.blown === b1;
      atDoor(); const a0 = leaf(); stand(); tap('Tab'); step(20); const tabDoor = !up() && Math.abs(leaf() - a0) < 0.01 && g.nearby.indexOf(g.hover) === 0;
      verdict(again === 1 && g1 && rmb && tab && tabDoor, 'the key again does the lit wedge; a right click and the back key put the ring away with nothing done',
        `the key again: whistled ${again}, gone ${g1} · right click: away, nothing done ${rmb} · Tab: away, nothing done ${tab} · Tab at the door: the leaf did not move, the top row is the tap's again ${tabDoor}`); }
    // ---- under it
    { rule(true); atDoor(); stand(); const gears = TWOGEARS.on; TWOGEARS.on = false;   // (the wheel's own rule: two gears — player.ts TWOGEARS — gives the wheel to nobody, so it is held off for this line and put back)
      const p0 = pl.pace; input.wheel = 100; step(1); input.wheel = 0; const paced = pl.pace < p0 && standing(), sq = g.orders.backFree(), sv = playerBusy(g) !== 'choosing an action', ch = !pl.choosing;
      input.wheel = -100; step(1); input.wheel = 0; TWOGEARS.on = gears; tap('Tab'); step(2);
       }
    // ---- a save in memory
    { rule(true); atDoor(); stand(); const stood = standing(), C: any = g.saves, r = C.writeMem(g, 'manual', 'ring'), key = r.info?.key; tap('Tab'); step(2); const away = !up();
      const why = key ? C.load(g, key) : 'no save'; const back = !!pl.sel.ring || pl.sel.open; step(2); const b0 = WHISTLE_STATS.blown, a0 = leaf(); click(); step(6); const clean = !up() && WHISTLE_STATS.blown === b0 && Math.abs(leaf() - a0) < 0.01;
      verdict(stood && r.ok && away && why === null && !back && clean, 'a save kept in memory under a standing ring (the whole game as it stood) is put back with no ring: his first click after the load is his own',
        `the ring stood at the save ${stood}, saved ${r.ok}${r.why ? ` (${r.why})` : ''} · Tab put it away ${away} · the load: ${why ?? 'done'} · a ring after it ${back} · a click then did no wedge ${clean}`); }
    // ---- the rows
    { rule(true); floor(); const alone = rows(); atDoor(); const withDoor = rows(), top = g.hover?.verb, topKind = g.hover?.kind; breath(); const b0 = WHISTLE_STATS.blown; tap('KeyV'); step(3); const old = WHISTLE_STATS.blown - b0;
      floor(); down(HOLD + 4); const onFloor = view(); lift(); tap('Tab'); step(2);
      verdict(alone === 'none' && !/whistle/.test(withDoor) && topKind === 'door' && old === 1 && /^ring \[5 whistle\] lit 5/.test(onFloor), 'the whistle is a wedge and no row: the list and the highlight are as they were; on open floor the ring is that one wedge; its old code still whistles',
        `open floor rows [${alone}], its ring: ${onFloor} · at the door [${withDoor}], the tap's row ${top} · the old code blew ${old}`); }
    // ---- off
    { rule(false); floor('pistol'); const none = rows(); atDoor(); const r = rows(); down(20); const list = pl.listOpen, noRing = !up(); lift(40);
      floor('pistol'); tap('Digit2'); step(60); 
       }
    // ---- the board
    { K.KEYMAP.on = true; K.KEYMAP.qFree = false; K.KEYMAP.qCmd = false; K.setKeys({}); let slot = true, hand = true; K.KEYMAP.slotWhen = () => slot; K.KEYMAP.handWhen = BRK ? null : () => hand;
      const codes = () => `E ${K.gameCode('KeyE') || '∅'} · Space ${K.gameCode('Space') || '∅'} · V ${K.gameCode('KeyV') || '∅'}`;
      const REST = ['KeyW', 'KeyA', 'KeyS', 'KeyD', 'KeyR', 'KeyZ', 'KeyL', 'KeyN', 'KeyB', 'KeyF', 'Digit1', 'Digit2', 'Digit3', 'Digit4', 'Digit5', 'ShiftLeft', 'KeyK'];
      const on = K.gameCode('KeyE') === 'Space' && K.gameCode('Space') === '' && K.gameCode('KeyV') === '' && K.gameCode('Backquote') === 'Digit0' && REST.every(c => K.gameCode(c) === c), seenOn = codes();
      const w1 = K.keyText('Space  pull it shut'), w2 = K.keyText('E puts it away. A grab needs empty hands'), w3 = K.keyText('press E to holster the gun first, then Space to question him');
      const w4 = K.keyText('Space held, then 5, whistles the cook out'), w5 = K.keyText('V whistles him over');
      const words = w4 === 'E held, then 5, whistles the cook out' && w5 === 'V whistles him over' && K.capNow('Space') === 'E' && K.keyCap('Space') === 'E' && w1 === 'E  pull it shut' && w2 === '` puts it away. A grab needs empty hands' && w3 === 'press ` to holster the gun first, then E to question him';
      const b = K.bindKey('KeyV', 'KeyG'), bound = b.ok && K.gameCode('KeyG') === 'KeyV' && K.capNow('KeyV') === 'G'; K.resetKeys(); const reset = K.gameCode('KeyV') === '' && K.gameCode('KeyE') === 'Space';
      slot = false; const noSlot = codes(), plain = K.gameCode('KeyE') === 'KeyE' && K.gameCode('Space') === 'Space' && K.gameCode('KeyV') === '';
      slot = true; hand = false; const offCodes = codes(), off = !BRK && K.gameCode('KeyE') === '' && K.gameCode('Space') === 'Space' && K.gameCode('KeyV') === 'KeyV' && K.keyText('Space  pull it shut') === 'Space  pull it shut';
      verdict(on && words && bound && reset && plain && off, 'the page\'s board: his own actions on E, Space and V give nothing, the words follow; the whistle can be given a key; without the slot the ring is on Space; rule off, as it was',
        `on: ${seenOn}, the rest themselves ${on} · words ${words} ("${w1}", "${w3.slice(27)}") · the whistle put on G ${bound}, Reset ${reset} · no slot: ${noSlot} ${plain} · rule off: ${offCodes} ${off}`); }
    // ---- the lesson's line
    { const { lessonLine } = await import(`${ROOT}/src/game/lesson.ts`), LINE = 'V whistles the cook out. Then hide at once, at the mark';
      rule(true); const on = `▸ ${lessonLine(LINE)}`; rule(false); const off = `▸ ${lessonLine(LINE)}`;
       }
    // ---- recordings
    { HANDRING.on = true; const snap: any = JSON.parse(JSON.stringify(knobsSnapshot(g)));
      const old = JSON.parse(JSON.stringify(snap)); delete old.tables.HANDRING;
      const offOld = await quiet(() => { applyKnobs(g, old); const off = HANDRING.on === false; applyKnobs(g, snap); return off && HANDRING.on === true; });
       }
    // ---- on its thing (HANDRING.onThing: where it stands and what the cursor lights · HANDRING.bare: no line under it)
    
    // ---- turned to the switch (HANDRING.turn: lightswitch.ts, handwork.ts — the row's own call, as a tap or a ring's wedge makes it)
    
    // ---- the corner panel (HANDRING.corner: ui/corner.ts — the reduction; the panel as drawn is the page's, never seen here)
    { const { cornerRows, objectiveParts, cornerOn } = await import(`${ROOT}/src/ui/corner.ts`), PROFILE = await import(`${ROOT}/src/host/profile.ts`), OV = await import(`${ROOT}/src/ui/overlay.ts`);
      rule(true); const soft = (o: any) => o.gone ? `~${o.text}~` : o.text;
      const show = (R: any[]) => R.map((r: any) => (r.sec ? `{${r.label}${r.note ? ` (${r.note})` : ''}: ${r.opts[0].text}}` : r.opts.length ? r.opts.map((o: any) => o.key ? `[${o.key}] ${o.text}` : soft(o)).join(' / ') : `'${r.label}'`) + (r.dist !== undefined ? ` ${r.dist}m` : '') + (r.progress !== undefined ? ' ▬' : '')).join(' | ') || 'nothing';
      // (a row as the page makes it: the thing's name and state over what parseOpts reads off the row's own second line — player.ts buildInteractables)
      const made = (kind: string, label: string, line2: string, over: any = {}) => ({ kind, label, opts: OV.parseOpts(line2), inReach: true, off: false, ...over });
      // (the panel of his picture: a door's row under its heading, the squad's title, four men — one lost — what they carry, the quick key's line)
      const door = made('door', 'door · open', 'Space  push it shut  ·  or sprint into it');
      const men = ['Gold 1 · stacked on the lobby door', 'Gold 2 · stacked on the lobby door', 'Blue 1 · covering the hall', 'Blue 2 · down'].map((text, i) => ({ key: '', text, gone: i === 3 }));
      const squad = [{ kind: 'objective', label: 'squad · loud · orders to gold', inReach: true, off: false, tone: 'gold', opts: [...men, { key: '', text: 'charges 2 · bangs 3' }, { key: '', text: 'frags 2' }, { key: '', text: 'charge set · lobby door — press F to blow it' }, { key: 'Q', text: 'move there · that spot' }] }];
      const A = cornerRows([door], 0, squad, 'par 1:00 / 1:30 / 2:00 · objective ▸ clear the back office · 2 left', [{ key: 'F', text: 'blow the charge · lobby door' }], 'infiltrate'), a = show(A.rows), flat = JSON.stringify(A.rows);
      const three = a === '{objective (par 1:00 / 1:30 / 2:00): clear the back office · 2 left} | [Space] push it shut | [F] blow the charge · lobby door | [Q] move there · that spot | ~Blue 2 · down~' && A.primary === 1 && A.rows[0].stage === 'infiltrate' && A.rows.slice(1).every((r: any) => r.one && r.label === '') && !/Gold|Blue 1|charges 2|frags|squad ·|door · open|sprint|press F/.test(flat);
      // (each only while it has something to say)
      const B = cornerRows([], -1, [{ ...squad[0], opts: men.slice(0, 3) }], ''), C = cornerRows([], -1, [{ ...squad[0], opts: squad[0].opts.filter((o: any) => !o.gone) }], 'objective ▸ reach the server room'), D = cornerRows([door], 0, [], '');
      // (a man in reach: both clicks · a verb running: its own words and its bar, no heading · one with no bar · a committed move's line · the word an
      //  empty tap left, with how far · the old list's cancel row is not one — each second line as player.ts words it)
      const man = made('guard', 'guard', 'LMB  knock him out  ·  RMB  grab him'), pick = made('door', 'door · locked', 'picking… 40%  ·  Space or a step breaks off', { progress: 0.4 }), wand = made('door', 'door · shut', 'looking under it — the cursor turns the lens  ·  Space or a step pulls it out');
      const word = { kind: 'door', label: 'door · step closer', opts: [], inReach: false, off: false, dist: 1.4 }, cancel = { kind: 'cancel', label: '', opts: [], inReach: true, off: false };
      const E = cornerRows([man, cancel], -1, [], ''), F = cornerRows([pick], 0, [], ''), W = cornerRows([wand], 0, [], ''), H = cornerRows([word], -1, [], '');
      const shapes = show(E.rows) === '[LMB] knock him out | [RMB] grab him' && E.primary === -1 && show(F.rows) === 'picking… 40% / Space or a step breaks off ▬' && show(W.rows) === 'looking under it — the cursor turns the lens / Space or a step pulls it out' && show(H.rows) === "'door · step closer' 1.4m";
      // (the line's own word is its label, whatever it is; a run's end is all words under none)
      const P1 = objectiveParts('training 2/7 ▸ press the key at the door'), P2 = objectiveParts('structure taken · 3:21 · GOLD — press shift+enter to restart'), P3 = objectiveParts('objective ▸ 0:42 · the charge in the vault');
      const parts = P1?.label === 'training 2/7' && P1.note === '' && P1.text === 'press the key at the door' && P2?.label === '' && P2.text.startsWith('structure taken') && P3?.label === 'objective' && P3.text === '0:42 · the charge in the vault' && objectiveParts('  ') === null;
      // (the page's own feed, asked with no page: what the panel is handed here, at a door with the squad — the three things; with the rule off the list as it was)
      const fed = (on: boolean, here = false) => { rule(on); HANDRING.on = !BRK; if (!here) { atDoor(); breath(); } const D = OV.Overlay.prototype['deriveInteractions'].call({}, g); return { rows: D.rows as any[], primary: D.primary as number, top: !D.rows.some((r: any) => r.sec) }; };   // (`top`: no section in what the panel is handed, so the line at the top still says it — setInteractions blanks that line only for a section it drew)
      const line = g.objectiveText(), on = fed(true), off = fed(false); rule(true);
      const wired = on.rows.every((r: any) => r.sec || r.one) && on.rows.some((r: any) => r.one && r.opts[0]?.key) && on.primary >= 0 && !!on.rows[on.primary]?.one && (!line || (on.rows[0].sec === true && !on.top)) && off.rows.every((r: any) => !r.sec && !r.one) && off.rows[0]?.label.length > 0 && off.top;
      // (a charge set on this door by hand, him a stride from it and then well away: whatever the block's line says of a key, or of who it would catch,
      //  the corner's notes say — a key's line under that key, a warning under none, nothing where the block's line says neither — and the panel has it)
      const DT: any = g.doorTools, leafD = g.doors.list.find((d: any) => d.def.name === 'conference'), { SAM_BY } = await import(`${ROOT}/src/game/ownblow.ts`), set0 = DT.set.length;
      const chargeAt = (near: boolean) => { rule(true); if (near) atDoor(); else floor(); step(20);
        const hn: string = (DT.hudNotes() as any[]).map((n: any) => typeof n === 'string' ? n : n.text).find((t: string) => /^charge set/.test(t)) ?? '', cn: any[] = DT.cornerNotes(), key = /press \w+ to blow it/.test(hn), warn = (hn.match(/(TOO CLOSE|BEHIND THE DOOR):.*$/) ?? [''])[0];
        const notes = key ? cn.length === 1 && cn[0].key.length > 0 && /^blow the charge/.test(cn[0].text) && cn[0].text.endsWith(warn) : warn ? cn.length === 1 && cn[0].key === '' && /^charge set/.test(cn[0].text) && cn[0].text.endsWith(warn) : cn.length === 0;
        const want = cn.filter((o: any) => !o.key || g.orders.active).map((o: any) => o.text), got = fed(true, true).rows.filter((r: any) => r.one).flatMap((r: any) => r.opts.map((o: any) => o.text)), panel = want.every((t: string) => got.includes(t)) && (want.length > 0 || !got.some((t: string) => /^charge set|TOO CLOSE|BEHIND THE DOOR/.test(t)));   // (his own charge's row, 'blow the charge' on his key, is his list's and stands either way)
        return { ok: notes && panel, say: `the block '${hn}' → ${cn.length ? cn.map((o: any) => `[${o.key}] ${o.text}`).join(' | ') : 'nothing'}, the panel ${panel ? 'has it so' : `HAS [${got.join(' | ')}] FOR [${want.join(' | ')}]`}${notes ? '' : ' (THE NOTES ARE NOT THE BLOCK\'S)'}` }; };
      DT.set.push({ door: leafD, side: 1, t: g.time, by: SAM_BY }); const cNear = chargeAt(true), cFar = chargeAt(false); DT.set.length = set0; step(20); const cNone = DT.cornerNotes().length === 0; rule(true);
      
      // (the panel's own gate: the three things with the rule on, the list as it was under the tour's puppet, with his ring's rule off, with this one off)
      const gate = cornerOn(false), gatePup = !cornerOn(true); HANDRING.on = false; const gateRing = !cornerOn(false); rule(true); HANDRING.corner = false; const gateOff = !cornerOn(false); rule(true);
      const all: any = JSON.parse(JSON.stringify(knobsSnapshot(g))), km = { ...K.KEYMAP };
      const page = await quiet(() => { PROFILE.pageSwitches(new URLSearchParams('')); const on = HANDRING.corner; PROFILE.pageSwitches(new URLSearchParams('corner=0')); const off = HANDRING.corner, rest = HANDRING.on && HANDRING.lone; applyKnobs(g, all); Object.assign(K.KEYMAP, km); return `${on ? 'on' : 'OFF'} as the page opens, ${off ? 'STILL ON' : 'off'} under ?corner=0${rest ? '' : ' (AND THE REST WITH IT)'}`; });
      const old = JSON.parse(JSON.stringify(all)); delete old.tables.HANDRING.corner;
      const offOld = await quiet(() => { applyKnobs(g, old); const o = HANDRING.corner === false; applyKnobs(g, all); return o && HANDRING.corner === !left('corner'); });
       }
    // ---- beside a thing (last: the mission stood up here puts every switch on its stand, and they are put back after it)
    
  } finally { HANDRING.on = was.on; HANDRING.early = was.early; HANDRING.empty = was.empty; HANDRING.near = was.near; HANDRING.lone = was.lone; HANDRING.loneWrap = was.loneWrap; HANDRING.onThing = was.onThing; HANDRING.bare = was.bare; HANDRING.corner = was.corner; HANDRING.turn = was.turn; BACKKEY.tab = was.back; HANDSLOT.on = was.slot; CMDRING.on = was.cmd; CMDRING.hands = was.hands; WALLSNAP.on = was.ws; WALLSNAP.press = was.press; K.setKeys({}); K.KEYMAP.on = was.kOn; K.KEYMAP.slotWhen = was.slotWhen; K.KEYMAP.handWhen = was.handWhen; K.KEYMAP.qFree = was.qFree; K.KEYMAP.qCmd = was.qCmd; G.localStorage = hadLS; console.log = log; console.error = err; console.warn = warn; }
  console.log(`\n=== handring: ${all ? 'ALL PASS' : 'FAIL'} ===`);
  return all;
}
