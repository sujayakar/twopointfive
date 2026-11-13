/** what this page is: the commit it was built from ('' = none), and what its game is made of, as one word (tools/blitz/made.ts) */
export interface Built { commit: string; made: string }
declare const __TPS_BUILT__: Built | undefined;

const HEX = /^[0-9a-f]{7,40}$/;
/** a word of the server that hands the page out, '' = it says none, or did not answer */
async function served(word: 'commit' | 'made'): Promise<string> {
  try { const r = await fetch(`/qa/${word}`, { cache: 'no-store', signal: AbortSignal.timeout(3000) }), c = r.ok ? (await r.text()).trim() : ''; return HEX.test(c) ? c : ''; } catch { return ''; }
}
let said: Built = typeof __TPS_BUILT__ === 'undefined' ? { commit: '', made: '' } : __TPS_BUILT__;
/** THIS PAGE, as it was built; a page built as it is served asks its server, and again the next time for what it was not told */
export async function built(): Promise<Built> {
  if (!said.commit || !said.made) said = { commit: said.commit || await served('commit'), made: said.made || await served('made') };
  return said;
}
export const buildName = async (): Promise<string> => (await built()).commit;
/** has the server another page to hand out than this one: loading it again would be another build */
export async function pageOld(): Promise<boolean> { const mine = await buildName(), now = await served('commit'); return !!mine && !!now && mine !== now; }
