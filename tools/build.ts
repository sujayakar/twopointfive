// Static production build into ./dist (bundle + copy public/). Two pages: the game (index.html) and the character viewer (viewer.html).
import { $ } from "bun";
await $`rm -rf dist`;
const res = await Bun.build({ entrypoints: ["./index.html", "./viewer.html"], outdir: "./dist", minify: true, target: "browser", sourcemap: "linked", loader: { ".wgsl": "text" } });
if (!res.success) { for (const m of res.logs) console.error(m); process.exit(1); }
await $`cp -R public/. dist/`;
console.log("built:", res.outputs.map(o => o.path).join("\n"));
