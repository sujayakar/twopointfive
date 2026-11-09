// WHAT A LESSON'S TOKENS WRITE OUT TO, for the writing check (writing-check.ts): a line or a card names a move by a token (`{grab him}`) and the build writes the token out by
// the keys it has. The game's own is rust/game_rules/src/lesson/p_01.rs (`token_line`, `token_say`, `lesson_token`), which the page shows; this is the same list and the same
// sentences, for a check that runs no game.
//  `{grab him}` · `{grab him from behind}` · `{knock him out}` (the blow in reach) · `{take him out}` (the silent blow from behind). `him` may be any words for the man
//  (`{grab the near guard from behind}`, `{grab yours}`): letters and apostrophes, one space between words. A capital first letter begins a sentence. A name in braces that is
//  none of these is left as written.
const TOKENS: [RegExp, (who: string, held: boolean, bare: boolean) => string][] = [
  [/^grab (.+) from behind$/, (who, held) => held ? `hold left click on ${who} from behind to grab him` : `press [Space] behind ${who} to grab him`],
  [/^grab (.+)$/, (who, held) => held ? `hold left click on ${who} to grab him` : `press [Space] to grab ${who}`],
  [/^knock (.+) out$/, (who, _h, bare) => bare ? `right-click to knock ${who} out` : `right-click to knock ${who} out, or left-click to use the knife`],   // (the click as a verb is hyphened; "hold left click on him" above is the button being held, and is not)
  [/^take (.+) out$/, (who, _h, bare) => bare ? `right-click to knock ${who} out` : `left-click to knife ${who}`],
];
/** the four, as a level writes them of 'him' */
export const LESSON_TOKENS: readonly string[] = ['grab him', 'grab him from behind', 'knock him out', 'take him out'];
const TOKEN = /\{([A-Za-z][A-Za-z' ]*)\}/g, lower = (name: string) => name.charAt(0).toLowerCase() + name.slice(1);
/** is this name (the words between the braces, either capital) one the writer writes out? */
const lessonToken = (name: string): boolean => { const n = lower(name); if (!/^[A-Za-z]+(?: [A-Za-z']+)*$/.test(name)) return false;   // (the letters the writer-out reads, one space between words, none at an end)
  const hit = TOKENS.find(([re]) => re.test(n)); return !!hit && !/(^| )from( |$)/.test(hit[0].exec(n)![1]); };   // (…and a 'from' left in the man's words is a 'from behind' misspelt)
/** a line with its tokens written out: `held` = the grab is held, `bare` = the knife is not to hand */
export const tokenLine = (line: string, held: boolean, bare: boolean): string => line.indexOf('{') < 0 ? line : line.replace(TOKEN, (m, name: string) => {
  const n = lower(name); if (!lessonToken(name)) return m; for (const [re, say] of TOKENS) { const hit = re.exec(n); if (!hit) continue;
    const s = say(hit[1], held, bare); return n !== name ? s.charAt(0).toUpperCase() + s.slice(1) : s; }   // (the capital the author wrote, nothing guessed)
  return m; });
