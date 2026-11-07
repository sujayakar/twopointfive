// WHERE A CALL ENDS IN A FILE'S TEXT, for the lints that read the tools as text (shipped-check.ts bareChildren, holds-check.ts). `from` is the index just
// past the call's opening bracket; the answer is the index just past the bracket that shuts it. Quotes and template strings are stepped over whole (a hole
// `${…}` in a template by its own braces), and so is a comment, to the end of its line or to its `*/`. WHAT IT CANNOT SEE: a regex literal. A quote or a
// bracket inside one is read as code, so `x.replace(/'/g, '')` inside a call moves where the call is taken to end. No lexer short of the compiler's tells a
// regex from a division; a lint that reads a call across such a literal says so in its own note.
export function callEnd(src: string, from: number): number {
  let i = from, depth = 1, q = '';
  for (; i < src.length && depth > 0; i++) { const c = src[i];
    if (q) { if (c === '\\') i++; else if (c === q) q = ''; else if (q === '`' && c === '$' && src[i + 1] === '{') { let h = 1; for (i += 2; i < src.length && h > 0; i++) { if (src[i] === '{') h++; else if (src[i] === '}') h--; } i--; } continue; }
    if (c === '/' && src[i + 1] === '/') { const e = src.indexOf('\n', i); i = e < 0 ? src.length : e; continue; }
    if (c === '/' && src[i + 1] === '*') { const e = src.indexOf('*/', i + 2); i = e < 0 ? src.length : e + 1; continue; }
    if (c === "'" || c === '"' || c === '`') q = c; else if (c === '(') depth++; else if (c === ')') depth--; }
  return i;
}
