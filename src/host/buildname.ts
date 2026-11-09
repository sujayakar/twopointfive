let said = '';
/** the build this page is, as the server that hands it out says (a commit's hash), or '': it says none, or did not answer (it is asked again the next time) */
export async function buildName(): Promise<string> {
  if (!said) try { const r = await fetch('/qa/commit', { cache: 'no-store', signal: AbortSignal.timeout(3000) }), c = r.ok ? (await r.text()).trim() : ''; if (/^[0-9a-f]{7,40}$/.test(c)) said = c; } catch { /* no answer */ }
  return said;
}
