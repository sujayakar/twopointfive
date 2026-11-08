// A PAGE VISIT'S NAME, as its records and its kept tape say it.
/** a name for this page visit: when it began, and enough dice that two tabs opened in one second differ */
export function sessionId(): string {
  let r = ''; try { const a = new Uint32Array(1); crypto.getRandomValues(a); r = a[0].toString(36); } catch { r = Math.floor(Math.random() * 0xffffffff).toString(36); }
  return `${new Date().toISOString().replace(/[-:.]/g, '').slice(0, 15)}-${r}`;
}
