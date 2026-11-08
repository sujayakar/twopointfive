// WHAT TAB SHOWS — the check behind ui/tabcard.ts: with no menu open Tab holds the game on a card of the objective in full and
// the squad (each teammate, what he is doing, what he carries), and Tab again goes back.
//   bun run tools/qa/probes.ts tabcard        (family `ui`)           TABCARD_BREAK=1 = the card is asked of
//                                                                      a game with no mission up: must FAIL
// the maintainer, 10-23: "if you don't have a menu open, tab opens up an objectives modal that also has your squad equipment?"
// WHAT IS ASKED:
//   the list      a room clear's line is its sentence and its rooms, each taken, to do or the one being called; a par or a clock is the note; a line
//                 that lists nothing is its sentence; a run's end line is words under no label
//   the game's    on a room clear with the squad of four: a line a man in the squad's order with what he is doing and the kit he was staged with, who
//                 the orders go to, the squad's counts, none of the keys' lines; a man lost is struck; the markup has the three sections and how to go back
//   the page      main.ts holds the game while the card is up and puts it away under a menu
// NOT HERE: the card as drawn and the key on the real page (a page drive); nothing has been seen on a GPU.
import { blockOf } from './squadwords';
import { readFileSync } from 'node:fs';
import { standUp, ROOT } from './headless.ts';

const DT = 1 / 60;
export async function tabCardCheck(): Promise<boolean> {
  const BRK = process.env.TABCARD_BREAK === '1'; let all = true;
  const verdict = (ok: boolean, what: string, detail: string) => { if (!ok) all = false; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${what}\n    ${detail}`); };
  console.log(`\n=== tabcard: Tab shows the objectives and the squad${BRK ? ' (no mission up: must fail)' : ''} ===`);
  const C = await import(`${ROOT}/src/ui/tabcard.ts`);
  // ---- the list
  { const A = C.tabCard('par 1:00 / 1:30 / 2:00 · objective ▸ clear the structure: ✓ the guard hut · [the store] · the workshop', [], []), B = C.tabCard('objective ▸ reach the server room', [], []), E = C.tabCard('structure taken · 1:12 · GOLD — press shift+enter to restart', [], []), N = C.tabCard('', [], []), L = C.tabCard('training 2/7 ▸ two ways in: the door · the window', [], []);
    const items = A.items.map((i: any) => `${i.done ? '✓' : i.now ? '▸' : '○'} ${i.text}`).join(' | ');
    verdict(A.label === 'objective' && A.note === 'par 1:00 / 1:30 / 2:00' && A.goal === 'clear the structure' && items === '✓ the guard hut | ▸ the store | ○ the workshop' && B.goal === 'reach the server room' && B.items.length === 0 && E.label === '' && E.items.length === 0 && E.goal.startsWith('structure taken') && !/<h4>/.test(C.tabCardHtml(E)) && L.label === 'training 2/7' && L.items.length === 0 && N.goal === '' && !/<section>/.test(C.tabCardHtml(N)),
      'an objective that lists is its sentence and its list, each thing taken, being called or to do; one that does not, a lesson\'s step and a run\'s end are their sentence, the last under no heading; no objective is no section',
      `[${A.label} · ${A.note}] ${A.goal}: ${items} · [${B.label}] ${B.goal} (${B.items.length} listed) · a lesson's step: [${L.label}] ${L.goal} (${L.items.length} listed) · a run's end: [${E.label}] ${E.goal.slice(0, 24)}… · none: '${N.goal}'`); }
  // ---- the game's own, on a room clear with the squad of four
  { const { startFromQuery } = await import(`${ROOT}/src/game/scenarioRun.ts`);
    const W: any = await standUp({ seed: 31 }), g = W.game; g.godMode = true; g.playerInvisible = true;
    const why = BRK ? 'no mission (the break)' : startFromQuery(g, new URLSearchParams('roomclear&squad4')).why; for (let i = 0; i < 30; i++) { g.player.visibility = 0.05; W.step(DT); }
    const S = g.orders.active ? g.orders.squad() : [], K = C.tabCardOf(g), html: string = C.tabCardHtml(K), keys = blockOf(g.orders).flatMap((r: any) => r.opts).filter((o: any) => o.key).map((o: any) => o.text);
    const men = K.men.length === 4 && K.men.every((m: any, i: number) => m.name === S[i].callsign && m.doing.length > 0 && m.kit.length > 8 && !m.down), counts = K.carry.some((c: string) => /^charges \d · bangs \d$/.test(c)) && !K.carry.some((c: string) => keys.includes(c)), who = /orders to/.test(K.who) && !/^squad/.test(K.who);
    // (the kit is the card the man was staged with, not this machine's table: a table changed after the staging changes no line)
    const KT = await import(`${ROOT}/src/game/kit.ts`), b0 = KT.KIT.members[1].bangs; let staged = false; try { KT.KIT.members[1].bangs = b0 === 0 ? 2 : 0; staged = C.tabCardOf(g).men[0]?.kit === K.men[0]?.kit; } finally { KT.KIT.members[1].bangs = b0; }
    // (a man's line is the block's own past his name, what he covers included; his counted things are said as what he set out with, the squad's count being the pool's)
    const lines = blockOf(g.orders)[0]?.opts.slice(0, S.length).map((o: any) => o.text) ?? [], asBlock = K.men.every((m: any, i: number) => lines[i]?.endsWith(`${m.name} · ${m.doing}`)), setOut = K.men.every((m: any) => /· started with \d+ stun, \d+ charge/.test(m.kit) && !/· \d+ stun/.test(m.kit)) && C.staged('a · 1 stun, 0 charges · b') === 'a · started with 1 stun, 0 charges · b';
    const secs = (html.match(/<section>/g) ?? []).length, you = !/<h4>you/.test(html), foot = /<footer>Press <kbd>Tab<\/kbd> to go back to the game\.<\/footer>$/.test(html), listed = K.label === 'objective' && K.items.length >= 2 && /<ul><li class="/.test(html);
    if (S[1]) { S[1].state = 'dead'; } const K2 = C.tabCardOf(g), struck = !!K2.men[1]?.down && /class="man down"/.test(C.tabCardHtml(K2));
    verdict(men && counts && who && staged && asBlock && setOut && you && secs === 2 && foot && listed && struck,
      'on a room clear with the squad of four the card has the rooms as a list, the squad under who the orders go to, a line a man with what he is doing and the kit he was staged with, what the squad carries and how to go back; a man lost is struck',
      `started: ${why ?? 'yes'} · [${K.label}] ${K.goal}: ${K.items.length} listed · ${K.men.map((m: any) => `${m.name} · ${m.doing} (${m.kit.slice(0, 18)}…)`).join(' | ') || 'no men'} · the squad's note '${K.who}' · carried: ${K.carry.join(' / ') || 'nothing said'} · the kit stays the staged card's when the table changes ${staged} · each man's words are the block's past his name ${asBlock} · his counts are said as what he started with ${setOut} · ${secs} sections, the way back said ${foot} · with ${S[1]?.callsign ?? 'nobody'} lost: struck ${struck}`); }
  // ---- the page (main.ts, read as text: the key and the hold are the page's)
  { const main = readFileSync(`${ROOT}/src/main.ts`, 'utf8'), css = readFileSync(`${ROOT}/index.html`, 'utf8');
    const saves = /if \(\(menu\.mode === 'pause' \|\| tabUp\) && [^\n]*input\.hit\('F5'\)[^\n]*if \(tabUp\) tabShow\(false\); else menu\.saveAsked\(\);/.test(main), key = /if \(tabUp\) tabShow\(false\); else if \(!frozen && !menu\.isOpen && !game\.backable\(\)\) tabShow\(true\);/.test(main);
    const holds = /frozen: \(\) => \([^\n]*\|\| tabUp\) && !scripted/.test(main), away = /if \(tabUp && \(menu\.isOpen \|\| optionsOpen\(\) \|\| overlay\.cardUp \|\| scripted\)\) tabShow\(false\);/.test(main) && /if \(tabUp\) \{ tabShow\(false\); input\.pressed\.delete\('Escape'\);/.test(main), calm = /!overlay\.cardUp && !tabUp && !perfsweep\.active/.test(main), sized = /'#tabcard'/.test(readFileSync(`${ROOT}/src/ui/textsize.ts`, 'utf8')), el = /<div id="tabcard"><\/div>/.test(css) && /#tabcard\.on \{ display: block; \}/.test(css);
    verdict(saves && key && holds && away && calm && sized && el, 'the page (read as text, the same under the break): Tab puts the card up only with nothing else open or to back out of and takes it down again, as Esc does; the game is held under it and its frames are not read as load; a menu over it puts it away; it follows the text size',
      `F5 under the card puts it away and saves ${saves} · the key ${key} · the game held ${holds} · put away under a menu, a script or Esc ${away} · held frames not sampled ${calm} · in the text size's layers ${sized} · the element and its rule ${el}`); }
  console.log(`\n=== tabcard: ${all ? 'ALL PASS' : 'FAIL'} ===`);
  return all;
}
