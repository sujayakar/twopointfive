// A LOST GPU DEVICE (src/gpu/device.ts GPU_LOST): what the page does when the browser takes its device away — nothing when the page itself is going (a load to
// another address, the Team & kit picture's window dropped: the browser calls that 'destroyed'), one fresh load of the page for a real loss, and the card only
// for a second loss inside a minute or a third in the tab, with the adapter, the reason and the boot's stage on it. The rule is one pure function and one string: no browser needed.
//   bun run tools/qa/gpulost-check.ts              exit 0 = every check passed, 1 = one failed
//   GPULOST_BREAK=1 bun run tools/qa/gpulost-check.ts     the rule is asked as if the switch were off (?gpuretry=0): must FAIL
//   GENTLE_BREAK=1 bun run tools/qa/gpulost-check.ts      the gentle second try is asked as if its switch were off (?gentle=0): must FAIL
//   PLAINAIR_BREAK=1 bun run tools/qa/gpulost-check.ts    plain air is asked as if its switch were off (?plainair=0): must FAIL
import { GPU_LOST, GENTLE, PLAIN_AIR, gpuStory, flyingFacts, gentleFor, gentleNote, gentleRead, gentleWrite, lostAct, lostWords, plainAirFor, plainAirNote, plainAirRow } from '../../src/gpu/device';

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
// ---- PLAIN AIR (PLAIN_AIR): a driver that stops on the volumetric program is not handed it — the note, the adapter's row, the page's word and the card
// (what the engine hands the browser in plain air, and what the plain text is made of: tools/qa/plainair-check.ts)
{ const p = process.env.PLAINAIR_BREAK === '1' ? false : true, DAY = 86_400_000, t = 500 * DAY;
  const WIN = 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/156.0.0.0 Safari/537.36', MAC = 'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/156.0.0.0 Safari/537.36';
  check(PLAIN_AIR.on === true, 'plain air ships on');
  check(PLAIN_AIR.labels.every(l => plainAirNote(true, true, [l], false, p)), `a real loss with one program in hand, that one the volumetric pass's (${PLAIN_AIR.labels.join(', ')}), leaves the note`);
  check(!plainAirNote(true, true, ['restir'], false, p) && !plainAirNote(true, true, [], false, p), 'one program in hand that is not the pass\'s, or none, leaves no note: there is no plain way to draw those');
  check(!plainAirNote(true, true, ['vol', 'restir'], false, p) && !plainAirNote(true, true, ['compositeSR', 'restir', 'reflect', 'vol', 'gatherTraceS', 'particlesUpdate'], false, p), 'the pass\'s program among others in hand leaves none: the burst does not say which');
  check(!plainAirNote(false, true, ['vol'], false, p), "the page's own end leaves none");
  check(!plainAirNote(true, false, ['vol'], false, p), 'a page that asked for no retry (a benchmark, a page a script drives) leaves none');
  check(!plainAirNote(true, true, ['vol'], true, p), 'a load in plain air already leaves none (its program of that label is the plain one)');
  check(!plainAirNote(true, true, ['vol'], false, false), '?plainair=0: no note');
  const first = gentleRead(gentleWrite(null, t - 1000, PLAIN_AIR.forget)), again = gentleRead(gentleWrite(first, t, PLAIN_AIR.forget));
  check(first?.n === 1 && again?.n === 2 && gentleRead(gentleWrite({ n: 2, at: t - PLAIN_AIR.forget * DAY }, t, PLAIN_AIR.forget))?.n === 1, `the note counts as the gentle one does: a first, one that came again, and a first again ${PLAIN_AIR.forget} days after the last`);
  check(plainAirFor(null, first, t, '', p) !== '', 'a first such loss a second ago: the load is in plain air, whatever the machine');
  check(plainAirFor(null, { n: 1, at: t - PLAIN_AIR.days * DAY + 1 }, t, '', p) !== '' && plainAirFor(null, { n: 1, at: t - PLAIN_AIR.days * DAY }, t, '', p) === '', `… for ${PLAIN_AIR.days} day, then the real program is tried again: one loss with it in hand is not a driver that always stops on it`);
  check(plainAirFor(null, { n: 2, at: t - PLAIN_AIR.days * DAY }, t, '', p) !== '' && plainAirFor(null, { n: 2, at: t - PLAIN_AIR.daysAgain * DAY + 1 }, t, '', p) !== '' && plainAirFor(null, { n: 2, at: t - PLAIN_AIR.daysAgain * DAY }, t, '', p) === '', `a loss that came again keeps it plain for ${PLAIN_AIR.daysAgain} days`);
  check(plainAirFor(null, null, t, '', p) === '' && plainAirFor(null, { n: 3, at: t + DAY }, t, '', p) === '', 'no note, or one from a clock ahead of this one: the load is as any other');
  check(plainAirRow(WIN, 'amd rdna-4') === 'amd rdna-4' && plainAirRow(WIN, 'AMD rdna-4 Radeon RX 9070 XT') === 'amd rdna-4' && plainAirFor(null, null, t, plainAirRow(WIN, 'amd rdna-4'), p) !== '', 'a browser on Windows whose adapter the records name starts in plain air with no note');
  check([plainAirRow(MAC, 'amd rdna-4'), plainAirRow(WIN, 'amd rdna-3'), plainAirRow(WIN, 'nvidia lovelace'), plainAirRow(WIN, ''), plainAirRow('', 'amd rdna-4')].every(r => r === '') && plainAirFor(null, null, t, '', p) === '', '… and no other: that adapter off Windows, its older family, another maker\'s, an adapter with no name, a page with no browser');
  check(plainAirFor('1', null, t, '', p) !== '', '?plainair=1 asks for it with no note and no row');
  check(plainAirFor('0', first, t, 'amd rdna-4', p) === '' && plainAirFor(null, first, t, 'amd rdna-4', false) === '', '?plainair=0: neither the note nor the row is read');
  const whys = [plainAirFor('1', null, t, ''), plainAirFor(null, first, t, ''), plainAirFor(null, null, t, 'amd rdna-4')];
  check(whys.every(w => w.length > 0 && w.length <= 80) && new Set(whys).size === 3, `each of the three reasons is its own sentence, and one a record keeps whole (${whys.map(w => w.length).join(', ')} characters of 80)`);
  gpuStory.adapter = 'acme discrete Some Card'; gpuStory.stage = ''; gpuStory.plainAir = p ? 'the check asked' : '';
  check(lostWords('Gone.', 'unknown', true).includes('smoke and fog drawn the plain way'), 'a load in plain air that is lost says so on its card');
  gpuStory.plainAir = '';
  check(!lostWords('Gone.', 'unknown', true).includes('plain'), '… and any other load does not');
  check(/^Load the page again and the game will /.test(PLAIN_AIR.nextWords) && / your graphics driver /.test(PLAIN_AIR.nextWords) && !/[:;—]/.test(PLAIN_AIR.nextWords), `the card of the loss that wrote the note tells the player what to do, the way one would say it ("${PLAIN_AIR.nextWords}")`); }
console.log(red ? `${red} FAILED` : 'all passed');
process.exit(red ? 1 : 0);
