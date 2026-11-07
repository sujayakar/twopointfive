// A STEALTH MISSION'S KIT IS WHAT YOUR CARD STARTS AT THERE (src/game/kit.ts KITDEAL.stealth).
//   bun run tools/qa/probes.ts kitstealth
//   KITSTEALTH_BREAK=1 …: the rule's broken path in kit.ts (the build before: the mission hands its kit over your card) — MUST FAIL.
// What it holds: nothing is handed over a card; a stealth mission puts a stealth card of your own first in the table, issued as the
// mission's kit and a legal fill of your pouches (five sponge rounds to one pouch); the loud card is parked and comes back untouched; a stealth card as
// issued follows the mission and one changed by hand does not; the store keeps the two apart; 'As issued' there is the mission's kit; the Quiet kit's
// card for you is that kit and no pistol alone; and the staging wears the stealth card on a quiet run and the loud one on a loud run.
// SINCE 2026-11-07 the rule is settled on and the check writes neither it nor the five switches it once raised beside it (nor the deal, which it held
// off): the stand's line, the two "OFF: the build before" lines, the Quiet kit's "with the switch off" half and the recording from before the key are out.
import { ROOT } from './headless.ts';
import { rig } from './doortools-probes.ts';

const K = await import(`${ROOT}/src/game/kit.ts`);
const { stickyVisitSet } = await import(`${ROOT}/src/game/stickycam.ts`);

type Card = Record<string, any>;
const J = (x: unknown) => JSON.stringify(x);
const BROKEN = process.env.KITSTEALTH_BREAK === '1';
const hushed = async <T>(fn: () => Promise<T> | T): Promise<T> => { const log = console.log, err = console.error, warn = console.warn; console.log = console.error = console.warn = () => { }; try { return await fn(); } finally { console.log = log; console.error = err; console.warn = warn; } };
const memStore = () => { const m = new Map<string, string>(); return { m, getItem: (k: string) => m.get(k) ?? null, setItem: (k: string, v: string) => { m.set(k, String(v)); } }; };
const you = (): Card => ({ ...K.KIT.members[0] });
const words = (m: Card) => `${m.weapon}${m.rings ? ` · ${m.rings} sponge` : ''}${m.cams ? ` · ${m.cams} cam` : ''}${m.wand ? ' · wand' : ''}${m.smoke ? ` · ${m.smoke} smoke` : ''}${m.bangs ? ` · ${m.bangs} stun` : ''}${m.frags ? ` · ${m.frags} frag` : ''}${m.shells ? ` · ${m.shells} HE` : ''}`;
const cost = (m?: Card) => { const c = m ? K.kitCost(0, m) : K.kitCost(0); return `${c.used}/${c.total}`; };
const FULL = { rings: 5, cams: 2, wand: true }, BARE = { rings: 5, cams: 0, wand: false };

export async function kitStealth(): Promise<boolean> {
  console.log(`\n=== kitstealth: a stealth mission's kit is what your card starts at there${BROKEN ? ' [KITSTEALTH_BREAK=1]' : ''} ===`);
  let all = true; const verdict = (ok: boolean, label: string, detail: string) => { all &&= ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${label}\n    ${detail}`); };
  const was = { big: K.BIGSLOT.on, on: K.KIT.on, slots: K.SLOTS.on };
  const raise = () => { K.kitMission(null); K.BIGSLOT.on = true; K.KIT.on = true; K.SLOTS.on = true; };
  try {
    // ── the card
    raise(); K.kitLoad(null, '');
    { const loud = you(); K.kitMission(FULL); const m = you(), c = K.kitCost(0), d = K.kitDefault(0);
      verdict(m.rings === 5 && m.cams === 2 && m.wand === true && !m.frags && !m.shells && m.weapon === 'rifle' && c.used <= c.total && c.used === 4 && K.slotCost('rings', 5) === 1 && K.slotCost('rings', 1) === 1 && K.slotCost('rings', 0) === 0 && !K.kitChanged(0) && J(d) === J({ ...m, suit: d.suit, trim: d.trim }) && K.kitRingsLent(true) === 0,
        'a stealth mission under the fog puts a stealth card first in the table: five sponge rounds in one pouch, the link\'s two cameras, the wand, nothing loud; it is what you are issued there, and nothing is handed over it', `${words(m)} · pouches ${cost()} · changed ${K.kitChanged(0)} · handed over ${K.kitRingsLent(true)}`);
      K.kitMission(null); verdict(J(you()) === J(loud), '… and the loud card is back as it was when the mission is a loud one', `${words(you())} · pouches ${cost()}`);
      K.kitMission(BARE); const b = you(); K.kitMission({ rings: 5, cams: 3, wand: true }); const t = you(); K.kitMission(null);
      verdict(b.rings === 5 && !b.cams && !b.wand && b.smoke + b.bangs === 3 && K.kitCost(0, b).used === 4 && t.cams === 2 && t.wand && K.kitCost(0, t).used === 4, 'with no fog (no cameras, no wand) the free pouches carry canisters; three cameras and a wand are held to what four pouches carry', `no fog: ${words(b)} (${cost(b)}) · three cameras: ${words(t)} (${cost(t)})`); }

    { const loud = J(you()); K.kitMission(FULL); K.kitLoad(null, ''); const after = J(you()); K.kitMission(null);
      verdict(after === loud && J(you()) === loud, 'the table loaded afresh while a stealth mission\'s screen is up reads the LOUD cards', `${words(JSON.parse(after))}`); }
    // ── on: it follows the mission until a row is changed; the store keeps the two apart
    { const st = memStore(); K.kitLoad(st, ''); const loud = J(you());
      K.kitMission(BARE); K.kitMission(FULL); const followed = you();
      const set = K.kitSet(0, 'cams', 1); const own = you(), kept = JSON.parse(st.m.get([...st.m.keys()][0]) ?? '{}');
      K.kitMission(BARE); const stays = you(); K.kitMission(null); const back = J(you()); K.kitMission(FULL); const again = you(); K.kitMission(null);
      K.kitLoad(st, ''); const loaded = J(you()); K.kitMission(FULL); const reread = you();
      verdict(set && followed.cams === 2 && followed.wand && own.cams === 1 && stays.cams === 1 && again.cams === 1 && reread.cams === 1 && back === loud && loaded === loud && kept.stealth?.cams === 1 && J({ ...kept.members[0], cans: undefined }) === J({ ...JSON.parse(loud), cans: undefined }),
        'a stealth card as issued follows the mission (the fog lever up: the cameras and the wand come); one camera put back by hand is YOURS: it stays through another mission\'s kit, through a loud mission between, and through the store, which keeps the loud card where it was and the stealth one beside it', `followed: ${words(followed)} · own: ${words(own)} · on a mission with no fog: ${words(stays)} · after a loud one: ${words(again)} · reread: ${words(reread)} · store: members[0] ${words(kept.members?.[0] ?? {})} | stealth ${kept.stealth ? words(kept.stealth) : 'none'}`);
      const why = K.kitLoadout('issued', 0), issued = you(), kept2 = JSON.parse(st.m.get([...st.m.keys()][0]) ?? '{}'); K.kitMission(null);
      verdict(why === null && issued.cams === 2 && issued.wand && kept2.stealth === undefined && J(you()) === loud, '\'As issued\' on that screen is the mission\'s kit again, nothing of a stealth card is left in the store, and the loud card never moved', `${words(issued)} · store stealth ${kept2.stealth ? words(kept2.stealth) : 'none'}`); }

    // ── on: the Quiet kit's card for you
    { K.kitLoad(null, ''); const why = K.kitLoadout('quiet'), q = you(); K.kitLoad(null, ''); K.kitMission(FULL); const list = (K.kitLoadouts(0) as { id: string }[]).map(l => l.id); K.kitMission(null);
      verdict(why === null && q.weapon === 'rifle' && q.rings === 5 && q.cams === 2 && q.wand && !q.frags && K.kitCost(0, q).used === 4 && !list.includes('quiet') && list[0] === 'issued',
        'the Quiet kit\'s card for you is a stealth mission\'s kit (the long gun kept, no frag), and a stealth mission\'s row of kits does not list it beside the same card as issued', `${words(q)} (${cost(q)}) · a stealth screen lists ${list.join(', ')}`); }

    // ── the staging
    { K.kitLoad(null, ''); const R: any = await hushed(() => rig(17, { stage: false })); raise(); K.kitLoad(null, ''); const g = R.game, pl = () => g.player;
      const ops = g.guards.filter((gd: any) => gd.unit && gd.team === 'operative'), others = g.guards.filter((gd: any) => !(gd.unit && gd.team === 'operative'));
      const run = (quiet: boolean, cams: number) => { g.soloCard = quiet; g.guards = quiet ? others : [...others, ...ops]; stickyVisitSet(g, cams); K.kitPlayer(g); return { rings: pl().rifle.rings, cams: pl().rifle.attachments.includes('cam') ? pl().rifle.cams : 0, wand: !!pl().wand, smoke: pl().canisters, stun: pl().flashbangs, frags: pl().frags, shells: pl().rifle.shells }; };
      const loud = run(false, 0), quiet = run(true, 2), dark = run(true, 0), loud2 = run(false, 0);
      K.kitMission(FULL); K.kitSet(0, 'rings', 0); K.kitMission(null); const none = run(true, 2); K.kitMission(FULL); K.kitLoadout('issued', 0); K.kitMission(null); run(false, 0);
      verdict(loud.rings === 0 && !loud.wand && loud.frags === 1 && quiet.rings === 5 && quiet.cams === 2 && quiet.wand && quiet.frags === 0 && quiet.shells === 0 && dark.rings === 5 && dark.cams === 0 && !dark.wand && dark.smoke + dark.stun === 3 && J(loud2) === J(loud) && none.rings === 0 && none.cams === 2,
        'STAGED: a loud run wears the loud card and a quiet run alone the stealth card (five rounds, the link\'s cameras, the wand, no frag, no HE); with no fog its canisters; the next loud run is the first again; and sponge rounds taken off the stealth card by hand are NOT handed back', `loud ${J(loud)} · quiet ${J(quiet)} · quiet, no fog ${J(dark)} · loud again ${J(loud2) === J(loud) ? 'the same' : J(loud2)} · sponge row at none: ${none.rings} rounds, ${none.cams} cameras`);
      { K.kitLoad(null, ''); K.kitMission(FULL); K.kitSet(0, 'weapon', 'pistol'); K.kitMission(null); K.kitShow(g); K.kitIssuedPage(true); let les: any; try { les = run(true, 2); } finally { K.kitIssuedPage(false); } const has = pl().hasRifle; K.kitShow(g); K.kitLoad(null, ''); run(false, 0);
        verdict(has === true && les.smoke === 4 && les.stun === 2 && les.frags === 2 && les.shells === 3 && les.wand, 'A LESSON\'S PAGE wears the issue kit whatever STEALTH card this machine keeps (one with no rifle here): the rifle, what a Player is built with, and the mission\'s wand handed over it as before', `rifle ${has} · ${J(les)}`); }
      }
  } finally { K.kitMission(null); K.BIGSLOT.on = was.big; K.KIT.on = was.on; K.SLOTS.on = was.slots; K.kitLoad(null, ''); K.SLOTS.on = was.slots; K.KIT.on = was.on; }
  console.log(`=== kitstealth: ${all ? 'ALL PASS' : 'FAILURES above'} ===`);
  return all;
}
