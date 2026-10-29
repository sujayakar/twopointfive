// A LOST GPU DEVICE (src/gpu/device.ts GPU_LOST): what the page does when the browser takes its device away — nothing when the page itself is going (a load to
// another address, the Team & kit picture's window dropped: the browser calls that 'destroyed'), one fresh load of the page for a real loss, and the card only
// for a second loss inside a minute or a third in the tab, with the adapter, the reason and the boot's stage on it. The rule is one pure function and one string: no browser needed.
//   bun run tools/qa/gpulost-check.ts              exit 0 = every check passed, 1 = one failed
//   GPULOST_BREAK=1 bun run tools/qa/gpulost-check.ts     the rule is asked as if the switch were off (?gpuretry=0): must FAIL
//   GENTLE_BREAK=1 bun run tools/qa/gpulost-check.ts      the gentle second try is asked as if its switch were off (?gentle=0): must FAIL
import { GPU_LOST, GENTLE, gpuStory, flyingFacts, gentleFor, gentleNote, gentleRead, gentleWrite, lostAct, lostWords } from '../../src/gpu/device';

const on = process.env.GPULOST_BREAK === '1' ? false : true;
let red = 0; const check = (ok: boolean, what: string) => { console.log(`${ok ? '- PASS' : '- FAIL'} — ${what}`); if (!ok) red++; };
const now = 1_000_000, W = GPU_LOST.windowMs;
check(GPU_LOST.on === true, 'the switch ships on');
check(lostAct('destroyed', false, null, now, true, on) === 'quiet', "a device the browser calls 'destroyed' (the page's own end) says nothing");
check(lostAct('unknown', true, null, now, true, on) === 'quiet', 'a loss while the page is leaving says nothing, whatever the reason');
check(lostAct(undefined, true, null, now, true, on) === 'quiet', '… and with no reason given');
check(lostAct('unknown', false, null, now, true, on) === 'reload', 'a real loss with no retry behind it loads the page again');
check(lostAct(undefined, false, null, now, true, on) === 'reload', '… and so does one the browser gives no reason for');
check(lostAct('unknown', false, now - 5_000, now, true, on) === 'card', 'a second loss five seconds after a retry is the card');
check(lostAct('unknown', false, now - W + 1, now, true, on) === 'card', '… and one just inside the window');
check(lostAct('unknown', false, now - W, now, true, on) === 'reload', 'a loss a full window after the last retry is retried again');
check(lostAct('unknown', false, now - 10 * W, now, true, on, GPU_LOST.tries) === 'card', `a tab that has retried ${GPU_LOST.tries} times gets the card, however long ago (a device lost on every load must not load for ever)`);
check(lostAct('unknown', false, now - 10 * W, now, true, on, GPU_LOST.tries - 1) === 'reload', '… and one retry short of that is still retried');
check(lostAct('unknown', false, null, now, false, on) === 'card', 'a page that did not ask for the retry (a tool page) gets the card');
check(lostAct('destroyed', false, null, now, false, on) === 'quiet', '… and still nothing for its own end');
for (const r of ['destroyed', 'unknown', undefined]) check(lostAct(r, r === 'destroyed', now - 1, now, true, false) === 'card', `switch off (?gpuretry=0), reason ${r}: the card at once, as it was`);
gpuStory.adapter = 'acme discrete Some Card'; gpuStory.stage = 'while compiling shaders, 37 of 119';
{ const w = lostWords('The device went away.', 'unknown', true);
  check(w.startsWith('GPU device lost: The device went away.'), "the card opens with the browser's own message");
  check(w.includes('acme discrete Some Card') && w.includes('reason: unknown') && w.includes('37 of 119') && w.includes('loaded again once'), 'the card names the adapter, the reason, the stage and the retry'); }
check(lostWords('x', 'unknown', 2).includes('loaded again 2 times'), 'two retries behind it: the card says two');
gpuStory.stage = '';
{ const w = lostWords('Gone.', undefined, false); check(!w.includes('reason') && !w.includes('loaded again') && !/·\s*$/.test(w), 'no reason, no stage, no retry: none of them is on the card'); }
// ---- A GENTLE SECOND TRY (GENTLE): a loss while shaders compile is noted, and the browser's loads for a while after compile one at a time
{ const g = process.env.GENTLE_BREAK === '1' ? false : true, DAY = 86_400_000, t = 500 * DAY;
  check(GENTLE.on === true, 'the gentle second try ships on');
  check(gentleNote(true, true, true, false, g), 'a real loss while shaders compile leaves the note');
  check(!gentleNote(false, true, true, false, g), "the page's own end leaves none, under ?gpuretry=0 too (its act is the card's there)");
  check(!gentleNote(true, true, false, false, g), 'a loss with nothing compiling leaves none (a driver that fell over in play is not the compiles)');
  check(!gentleNote(true, false, true, false, g), 'a page that asked for no retry (a benchmark, a page a script drives) leaves none');
  check(!gentleNote(true, true, true, true, g), 'a loss on a load that was gentle already leaves none: it says nothing of the burst');
  check(!gentleNote(true, true, true, false, false), '?gentle=0: no note');
  const first = gentleRead(gentleWrite(null, t - 1000)), again = gentleRead(gentleWrite(first, t));
  check(first?.at === t - 1000 && first.n === 1 && again?.n === 2 && again.at === t, `the note keeps how many times and when (${JSON.stringify(first)}, then ${JSON.stringify(again)})`);
  check(gentleRead(gentleWrite({ n: 2, at: t - GENTLE.forget * DAY }, t))?.n === 1 && gentleRead(gentleWrite({ n: 2, at: t - GENTLE.forget * DAY + 1 }, t))?.n === 3 && gentleRead(gentleWrite({ n: 4, at: t + DAY }, t))?.n === 1, `a loss ${GENTLE.forget} days after the last is a first again, and so is one after a note from a clock ahead`);
  check(gentleRead(gentleWrite({ n: 9, at: t - 1 }, t))?.n === 9, 'the count stops at nine');
  check(gentleRead(null) === null && gentleRead('') === null && gentleRead('yes') === null && gentleRead('0:5') === null && gentleRead('3') === null && gentleRead('2:0') === null, 'nothing, or a note that is not one of these, reads as none');
  check(gentleFor(null, first, t, g), 'a first loss a second ago: the load is gentle');
  check(gentleFor(null, { n: 1, at: t - GENTLE.days * DAY + 1 }, t, g) && !gentleFor(null, { n: 1, at: t - GENTLE.days * DAY }, t, g), `… for ${GENTLE.days} day, no longer: one loss is not a machine that always loses`);
  check(gentleFor(null, { n: 2, at: t - GENTLE.days * DAY }, t, g) && gentleFor(null, { n: 2, at: t - GENTLE.daysAgain * DAY + 1 }, t, g) && !gentleFor(null, { n: 2, at: t - GENTLE.daysAgain * DAY }, t, g), `a loss that came again keeps it gentle for ${GENTLE.daysAgain} days`);
  check(!gentleFor(null, null, t, g) && !gentleFor(null, { n: 3, at: t + DAY }, t, g), 'no note, or one from a clock ahead of this one: the load is as any other');
  check(gentleFor('1', null, t, g), '?gentle=1 asks for it with no note');
  check(!gentleFor('0', first, t, g) && !gentleFor(null, first, t, false), '?gentle=0: the note is not read');
  gpuStory.adapter = 'acme discrete Some Card'; gpuStory.stage = ''; gpuStory.gentle = g ? 'the check asked' : '';
  check(lostWords('Gone.', 'unknown', true).includes('shaders compiled one at a time'), 'a gentle load that is lost too says on its card that its shaders compiled one at a time, whatever the stage');
  gpuStory.gentle = '';
  check(!lostWords('Gone.', 'unknown', true).includes('at a time'), '… and any other load does not');
  // what a record keeps of the labels: strings a report's data holds whole (80 characters, no lists)
  const one = flyingFacts(['restir']), many = flyingFacts(Array.from({ length: 16 }, (_, i) => `smoke_advectScalars${i}`)), none = flyingFacts([]);
  check(one.flying === 'restir' && one.flyingOf === 1 && !('flying2' in one), `one program in hand is named whole (${JSON.stringify(one)})`);
  const rows = [many.flying, many.flying2, many.flying3].filter(r => typeof r === 'string') as string[];
  check(many.flyingOf === 16 && rows.length === 3 && rows.every(r => r.length <= 80) && rows.join(' ').split(' ').every((l, i) => l === `smoke_advectScalars${i}`), `sixteen: the count, and as many whole labels in order as three strings of 80 hold (${rows.join(' ').split(' ').length} of 16)`);
  check(Object.keys(none).length === 0, 'none in hand: nothing is added to the record'); }
console.log(red ? `${red} FAILED` : 'all passed');
process.exit(red ? 1 : 0);
