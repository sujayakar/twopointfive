// THE TABLE OF DECIMAL POWERS FOR THE RUST MATHS (rust/sim/src/pow10.rs; maths.rs log10 reads it as src/math/exact.ts log10 reads `Number(`1e${k}`)`).
//   bun run tools/sim/pow10.ts            writes rust/sim/src/pow10.rs (then `bun run tools/sim/build.ts`: the crate's sources moved)
//   bun run tools/sim/pow10.ts --check    exit 0 = the file is what this writes · 1 = it is not
// Each row is the double the LANGUAGE reads for that decimal, by its bits: the table is the language's own parse, written down once, so the Rust side never
// parses a decimal and never rounds one its own way. 1e-323 is the smallest power that is not zero, 1e308 the largest that is not infinite.
import * as fs from 'node:fs';
import * as path from 'node:path';

const FILE = path.resolve(import.meta.dir ?? new URL('.', import.meta.url).pathname, '../../rust/sim/src/pow10.rs'), MIN = -323, MAX = 308, PER = 6;

export function pow10Text(): string {
  const f = new Float64Array(1), u = new BigUint64Array(f.buffer), rows: string[] = [];
  for (let k = MIN; k <= MAX; k++) { f[0] = Number(`1e${k}`); if (!(f[0] > 0) || !Number.isFinite(f[0])) throw new Error(`1e${k} is not a power the language can hold`); rows.push('0x' + u[0].toString(16).padStart(16, '0')); }
  const lines: string[] = []; for (let i = 0; i < rows.length; i += PER) lines.push('    ' + rows.slice(i, i + PER).join(', ') + ',');
  return [
    '//! The decimal powers 1e-323 … 1e308 as the language reads them (maths.rs log10): each one the double nearest the decimal, by its bits.',
    '//! Written by tools/sim/pow10.ts; the check in tools/qa/rust-check.ts holds every row to `Number(`1e${k}`)`.',
    `pub const POW10_MIN: i32 = ${MIN};`,
    `pub const POW10: [u64; ${rows.length}] = [`,
    ...lines,
    '];',
    '',
  ].join('\n');
}

if (import.meta.main) {
  const text = pow10Text();
  if (process.argv.includes('--check')) {
    const same = fs.existsSync(FILE) && fs.readFileSync(FILE, 'utf8') === text;
    console.log(same ? 'rust/sim/src/pow10.rs is the table this tool writes' : 'rust/sim/src/pow10.rs IS NOT the table this tool writes: bun run tools/sim/pow10.ts, then bun run tools/sim/build.ts');
    process.exit(same ? 0 : 1);
  }
  fs.writeFileSync(FILE, text);
  console.log(`wrote ${path.relative(process.cwd(), FILE)}: ${MAX - MIN + 1} powers`);
}
