/** what this page is: the commit it was built from ('' = none), and what its game is made of, as one word (tools/blitz/made.ts) */
export interface Built { commit: string; made: string }
declare const __TPS_BUILT__: Built | undefined;

const HEX = /^[0-9a-f]{7,40}$/;
/** a word of the server that hands the page out, '' = it says none, or did not answer */
async function served(word: keyof Built): Promise<string> {
  try { const r = await fetch(`/qa/${word}`, { cache: 'no-store', signal: AbortSignal.timeout(3000) }), c = r.ok ? (await r.text()).trim() : ''; return HEX.test(c) ? c : ''; } catch { return ''; }
}
const said: Built = typeof __TPS_BUILT__ === 'undefined' ? { commit: '', made: '' } : __TPS_BUILT__;
/** a word of this page, as it was built; of one built as it is served, as its server says, and asked again where it did not answer */
const own = async (word: keyof Built): Promise<string> => said[word] ||= await served(word);
export const buildName = (): Promise<string> => own('commit');
export async function built(): Promise<Built> { const [commit, made] = await Promise.all([own('commit'), own('made')]); return { commit, made }; }
/** has the server another page to hand out than this one: loading it again would be another build */
export async function pageOld(): Promise<boolean> { const mine = await buildName(), now = await served('commit'); return !!mine && !!now && mine !== now; }
