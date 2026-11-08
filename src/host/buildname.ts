let asked: Promise<string> | null = null;
/** the build this page is, as the server that hands it out says (a commit's hash), or '': it says none. Asked once a page */
export const buildName = (): Promise<string> => asked ??= (async () => { try { const r = await fetch('/qa/commit', { cache: 'no-store', signal: AbortSignal.timeout(3000) }), c = r.ok ? (await r.text()).trim() : ''; return /^[0-9a-f]{7,40}$/.test(c) ? c : ''; } catch { return ''; } })();
