// ---------------------------------------------------------------------------
// Packs the built app into one self-contained HTML fragment.
//
// Artifact pages are served under a strict CSP that blocks every external host,
// and are wrapped in a <!doctype><head></head><body> skeleton at publish time.
// So the output here is deliberately *not* a whole document: it is the styles,
// the body markup and the script, with every asset carried inline.
//
// The rig is the interesting part. It normally arrives over two fetches, and
// there is no origin to fetch from once this is a single file — so both files
// are embedded as data URIs and a small shim answers those two requests. The
// shim lives here rather than in the app so the runtime keeps exactly one way
// of loading the rig.
//
// Usage: node tools/build-artifact.mjs
// ---------------------------------------------------------------------------

import { execFileSync } from "node:child_process";
import { readFileSync, writeFileSync, readdirSync } from "node:fs";
import { join, dirname } from "node:path";
import { fileURLToPath } from "node:url";

const ROOT = join(dirname(fileURLToPath(import.meta.url)), "..");

execFileSync("npx", ["vite", "build"], { cwd: ROOT, stdio: "inherit" });

const html = readFileSync(join(ROOT, "dist/index.html"), "utf8");

const jsName = readdirSync(join(ROOT, "dist/assets")).find((f) => f.endsWith(".js"));
if (!jsName) throw new Error("no bundle in dist/assets");
const js = readFileSync(join(ROOT, "dist/assets", jsName), "utf8");

const rigJson = readFileSync(join(ROOT, "public/rig.json"), "utf8");
const rigBin = readFileSync(join(ROOT, "public/rig.bin"));

// Everything between <style> and </style>, and the contents of <body>.
const style = html.match(/<style>([\s\S]*?)<\/style>/)[1];
const body = html
  .match(/<body>([\s\S]*?)<\/body>/)[1]
  .replace(/<script[\s\S]*?<\/script>/g, "")
  .trim();

const shim = `
// The rig normally arrives over two fetches. There is no origin here, so answer
// those two requests from data carried in the page and let everything else
// through untouched.
(() => {
  const RIG_JSON = ${JSON.stringify(rigJson)};
  const RIG_BIN = "${rigBin.toString("base64")}";
  const bin = () => {
    const raw = atob(RIG_BIN);
    const out = new Uint8Array(raw.length);
    for (let i = 0; i < raw.length; i++) out[i] = raw.charCodeAt(i);
    return out.buffer;
  };
  const real = window.fetch.bind(window);
  window.fetch = (input, init) => {
    const url = typeof input === "string" ? input : input.url;
    if (url.endsWith("rig.json")) {
      return Promise.resolve(new Response(RIG_JSON, {
        status: 200, headers: { "Content-Type": "application/json" },
      }));
    }
    if (url.endsWith("rig.bin")) {
      return Promise.resolve(new Response(bin(), { status: 200 }));
    }
    return real(input, init);
  };
})();
`;

const out = `<style>${style}</style>
${body}
<script type="module">
${shim}
${js}
</script>
`;

const dest = join(ROOT, "dist/artifact.html");
writeFileSync(dest, out);
console.log(`${dest}  ${(out.length / 1024).toFixed(0)} KB`);
