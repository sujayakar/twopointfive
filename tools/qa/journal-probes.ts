// The 'journal' family (probes.ts: `bun run tools/qa/probes.ts journal` — also in 'all'): the SESSION JOURNAL (src/game/journal.ts + sensors.ts + digest.ts) held
// to its claim without a browser. What is checked:
//   journal-roundtrip  (a) a 90-second scripted session is RECORDED headless (the sensors live over the fake GPU that never answers — the harness's stand-in — Sam
//                          walked, crouched, a whistle that fetches Kowalski, a canister thrown, the pistol drawn and a lobby panel shot out, a sprint, then a long
//                          crouch in the corner while the floor works it out — ALL of it through the synthetic input path once the recording is up: a poke into the
//                          game past frame 0 is not an input and would not replay), the file passes its own check and its size per frame is printed stream by
//                          stream; REPLAYED into a fresh game it runs to the last frame with every digest agreeing and not one sensor read diverging;
//   journal-pagelike   (a2) the same script recorded as ON THE PAGE where the harness can imitate it — a GPU that ANSWERS every query a frame late with f32s, a
//                          ragged rAF dt, the HUD nulling expired bubbles after each update, the stop deferred as F10's — replays identically with no probe;
//   journal-perturb    (b) the same file with ONE sensor's values forged from a frame on (the guards' smoke sight lines to Sam made opaque, then Sam's meter made
//                          bright) diverges — a digest mismatch and / or a sensor divergence at or after that frame, never before it — and the field is named;
//   journal-knob       (c) the same file replayed by a build whose gameplay differs (a knob nudged after the file's own were applied: the men walk 15 % faster)
//                          mismatches, naming a guard's field — what an INTENDED change looks like (re-record); while a presentation-only change (the torch's
//                          emitter radius on the replay's engine — a look, not an input) replays identically;
//   journal-fixtures   (d) every session under tools/qa/sessions/ (the committed fixtures — the first was this family's own recording when it was written, and still throws by the quick-throw key) replays identically on
//                          this build: the gate a refactor must pass. `--write` files (a)'s recording there as selftest.json.gz.
import { standUp, ROOT } from './headless.ts';
const { mulberry32 } = await import(`${ROOT}/src/math/dice.ts`);

type Vec3 = [number, number, number];
const DT = 1 / 60;
const argv = process.argv.slice(2);
const WRITE = argv.includes('--write'), VERBOSE = argv.includes('--verbose');
let familyFails = 0;
export function journalFailures(): number { return familyFails; }

/** A stand-in for the GPU that ANSWERS (the 'page' variant): every key asked one frame is answered the next with a value off its own die — an irradiance for a point
 *  query (skewed dark, the odd bright one), a transmittance for a segment — as f32s, the way the probe's readback hands them over; `results` is the table forget()
 *  deletes from. Nothing of it exists in a replay: the journal must carry every value the game took. */
export function answeringProbe(seed: number) {
  const rnd = mulberry32(seed); const asked = new Map<string, boolean>(); const results = new Map<string, [number, number, number]>();
  const probe = { query(k: string) { asked.set(k, false); }, querySegment(k: string) { asked.set(k, true); }, get(k: string) { return results.get(k); }, results, hasPending: false, encode() { /* no GPU */ }, afterSubmit() { /* no GPU */ } };
  const tick = () => { for (const [k, seg] of asked) { const r = rnd(); results.set(k, [Math.fround(seg ? (r < 0.15 ? 0.05 + r : 0.75 + 0.25 * r) : (r < 0.9 ? r * r * 0.08 : 0.4 + 3 * rnd())), 0, 0]); } asked.clear(); };
  return { probe, tick, results };
}

/** WHAT THE SCRIPTED SESSION DID, of what its title says it does: a script is input, and input a stand does not take leaves a recording that replays perfectly
 *  and holds nothing. Twice it did: the canister went by a key the page ignores, and the pistol was holstered before its round on every stand. */
export interface Deeds { whistled: boolean; thrown: boolean; fired: boolean }
/** the deeds a recording lacks, as words for a failing line ([] = it did them all) */
export const undone = (d: Deeds): string[] => [...(d.whistled ? [] : ['never whistled']), ...(d.thrown ? [] : ['never threw its canister']), ...(d.fired ? [] : ['never fired its round'])];
/** JOURNALSCRIPT_BREAK=1 = the red path of the script's deeds: its trigger is never pulled, as when a stand ignores a key the script presses: the canister is not
 *  thrown and the round is not fired, and every probe that records the script must say so */
const SCRIPT_BREAK = process.env.JOURNALSCRIPT_BREAK === '1';
/** RECORD the scripted session headless → the file (parsed back through JSON, as off a disk) + notes. Everything after request() is INPUT: keys, cursor, clicks.
 *  `page` = as on the game page where the harness can imitate it: a GPU that answers (answeringProbe, a frame late), a ragged rAF dt, the HUD's bubble expiry after
 *  every update (it did, until the HUD stopped writing the game: 2026-11-07) with the file flagged env.hud, and the stop DEFERRED to the end of the next update as F10's is. */
export async function recordScripted(seed = 6011, page = false, seconds = 90): Promise<{ file: any; notes: string[]; game: any; deeds: Deeds }> {
  Math.random = mulberry32(seed);
  const W = await standUp(); const { game, input } = W; const notes: string[] = [];
  const gpu = page ? answeringProbe(seed ^ 0x9e3779b9) : null; if (gpu) game.probe = gpu.probe;
  const dtDie = mulberry32(seed + 7); const dtOf = () => page ? 1 / 60 + (dtDie() - 0.5) * 0.006 : DT;   // (a 60 Hz rAF's dt wanders a few hundred µs either way; the odd long frame)
  // staged BEFORE the recording (all of it lands in the checkpoint): Sam a little into the lobby, dark-ish, god mode (a firefight must not end the take), autosave on
  game.godMode = true; game.saves.enabled = true; game.player.visibility = 0.12;
  for (let i = 0; i < 30; i++) { gpu?.tick(); W.step(DT); }
  const ok = game.journal.request({ seed: 424242, note: `journal-probes scripted session (${page ? 'page-like: answering GPU, ragged dt, HUD' : 'headless'})`, hud: page });
  if (!ok) throw new Error('journal.request refused: ' + game.journal.summary());
  const hold = new Set<string>();
  const press = (k: string) => input.pressed.add(k);
  /** the trigger, pulled and let go over two frames — or, on the red path, two frames with no trigger */
  const click = () => { if (!SCRIPT_BREAK) { input.clicked |= 1; input.buttons |= 1; } stepN(2); input.buttons &= ~1; };
  const aimAt = (p: Vec3) => W.cursorAt(p);
  let f = 0; const stepN = (n: number) => { for (let i = 0; i < n; i++) { for (const k of hold) input.keys.add(k); gpu?.tick(); W.step(dtOf()); f++; } };
  const setHold = (...ks: string[]) => { for (const k of [...input.keys]) if (!ks.includes(k)) input.keys.delete(k); hold.clear(); for (const k of ks) hold.add(k); };
  // t 0–2.5 s: walk east across the lobby
  setHold('KeyD'); stepN(150);
  // crouch, creep north-east a little
  press('KeyC'); setHold('KeyD', 'KeyW'); stepN(90); setHold(); stepN(20);
  notes.push(`t ${game.time.toFixed(1)}: Sam at (${game.player.char.pos[0].toFixed(1)}, ${game.player.char.pos[2].toFixed(1)}) crouched ${game.player.crouch}`);
  // whistle (Kowalski walks the corridor's west end a few metres north), then slip away south-west
  const seq0: number = game.eventSeq; press('KeyV'); stepN(20); const whistled = game.eventSeq > seq0 && game.events.some((e: any) => e.kind === 'whistle');
  setHold('KeyS', 'KeyA'); stepN(110); setHold(); stepN(40);
  notes.push(`t ${game.time.toFixed(1)}: whistled; ${game.guards.map((g: any) => `${g.callsign} ${g.state} ${g.awareness.toFixed(2)}`).join(', ')}`);
  // a smoke canister lobbed east, the way every stand throws one: its slot, the cursor out on the lobby floor, the trigger. (It was the quick throw, a held G,
  // which the page does not take: on the page's stand the session recorded no throw at all and nothing said so. `deeds` says so now.)
  const cans0: number = game.player.canisters;
  press('Digit3'); stepN(10); aimAt([12.5, 0, 17.5]); stepN(4); click(); stepN(114);
  const thrown = game.player.canisters === cans0 - 1;
  notes.push(`t ${game.time.toFixed(1)}: canisters left ${game.player.canisters}${thrown ? '' : ' (NOT THROWN)'}, items ${game.items.items.map((it: any) => it.kind).join(',') || 'none'}`);
  // the pistol out, a lobby ceiling panel shot (the auto-aim snaps to the light), holstered again. (The script pressed E after the pistol's digit, which put the
  // gun AWAY: the round was never fired, on any stand, and the 'light shot out' of the family's title never was. The digit alone draws.)
  press('Digit1'); stepN(46);
  const panel = game.targets.find((t: any) => t.name === 'panel_lobby_2') ?? game.targets.find((t: any) => t.kind === 'analytic' && t.on);
  const rounds0: number = game.player.pistol.roundsReady;
  if (panel) { aimAt(panel.pos); stepN(4); click(); stepN(30); }
  const fired = game.player.pistol.roundsReady === rounds0 - 1;
  const deeds: Deeds = { whistled, thrown, fired };
  notes.push(`t ${game.time.toFixed(1)}: shot at ${panel?.name}${fired ? '' : ' (NOT FIRED)'} → on ${panel?.on} broken ${panel?.broken}; pistol ${game.player.pistol.chamber}+${game.player.pistol.mag}`);
  press('KeyE'); stepN(30);
  // sprint south-west into the corner, then a long crouched wait while the floor reacts (search, radio, calm)
  if (game.player.crouch) press('KeyC'); setHold('ShiftLeft', 'KeyS', 'KeyA'); stepN(120); setHold(); stepN(10); press('KeyC'); stepN(30);
  aimAt([6, 0, 12]); stepN(60);
  const FRAMES = seconds * 60; stepN(Math.max(0, FRAMES - f - (page ? 1 : 0)));
  notes.push(`t ${game.time.toFixed(1)}: end — ${game.guards.map((g: any) => `${g.callsign} ${g.state}`).join(', ')}; escalation ${game.escalation}; chronicle ${game.chronicle.events.length} rows; Sam hits left ${game.player.hitsLeft}; meter ${game.player.visibility.toFixed(2)}`);
  let file0: any;
  if (page) { game.journal.stop('selftest (deferred, as F10)'); if (!game.journal.stopping) throw new Error('a deferred stop did not defer'); stepN(1); file0 = game.journal.last; }   // F10's road: asked between frames, finalised at the end of the next update
  else file0 = game.journal.stop('selftest', true);
  if (!file0 || game.journal.state !== 'idle') throw new Error('journal.stop returned no file: ' + game.journal.summary());
  const file = JSON.parse(JSON.stringify(file0));   // (through the disk's eye)
  return { file, notes, game, deeds };
}







if (import.meta.main) {
  const which = argv.find(a => !a.startsWith('--')) ?? 'all';
  console.log(`\n=== journal: ${familyFails ? `${familyFails} FAIL` : 'ALL PASS'} ===`);
  process.exit(familyFails ? 1 : 0);
}
