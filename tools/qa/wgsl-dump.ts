// EVERY SHADER TEXT THE PAGE'S BOOT MAKES, AS A FILE (the first half of tools/qa/wgsl-compile.cjs, which hands each to a browser's WGSL compiler). The page's
// boot is run over a recording stand-in (tools/qa/pageboot.ts: the Engine, the smoke solver, the Game's own modules and the cut-box set, with the switches
// main.ts raises) under ONE setting of the switches, on the shipped stand, and the distinct texts are written as JSON: [{ label, code }]. The shader rows (the
// compact writings, the split gather: ui/shaderrows.ts) are read once at import and the switches are module state, so one run dumps one setting of each: the
// compile tool runs this once for every pair (`label` = what to call that run's texts).
//   bun run tools/qa/wgsl-dump.ts <out.json> --setting title|snow|look [label]
export {};
process.env.TPS_PROFILE = 'shipped';   // (the page's shaders: the frozen stand holds switches off that are constants of the scene prelude — other texts)
const { pageBoot, settingArg, SETTINGS } = await import('./pageboot.ts');
const argv = process.argv.slice(2), setting = settingArg(argv), rest = argv.filter((a, i) => a !== '--setting' && argv[i - 1] !== '--setting'), out = rest[0], tag = rest[1] ? `, ${rest[1]}` : '';
if (!out || !setting) { console.error('usage: bun run tools/qa/wgsl-dump.ts <out.json> --setting title|snow|look [label]'); process.exit(2); }
const { mods, faults } = await pageBoot(setting);
if (faults.length) { console.error(`wgsl-dump: ${faults.join(' · ')}, so the dump would be short`); process.exit(2); }
const all: { label: string; code: string }[] = [], seen = new Set<string>();
for (const m of mods) if (!seen.has(m.code)) { seen.add(m.code); all.push({ label: `${m.label} (${SETTINGS[setting]}${tag})`, code: m.code }); }
await Bun.write(out, JSON.stringify(all));
console.error(`wgsl-dump: ${all.length} distinct shader texts, ${all.reduce((n, m) => n + m.code.length, 0)} characters, to ${out}`);
process.exit(0);
