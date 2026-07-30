---
name: verify
description: Drive twopointfive end to end on this GPU-less box — build, run headless Chromium+SwiftShader, observe frames/counters/screenshots. Use before landing any renderer or shader change.
---

# verify (twopointfive)

There is no GPU here. Verification = the headless SwiftShader harness in
`tools/headless/` (read its README first: hooks, `--bench-res`, why
timings are meaningless and counters are not).

## Recipe that works

```bash
npm run build                                   # dist/ is what the harness serves
python3 tools/headless/run.py --scenario S.js --json out.json --shot out.png
python3 tools/headless/run.py --bench 3 --bench-res 384 240 --json bench.json
```

- Chromium must run OUTSIDE the Bash-tool sandbox (it needs its own
  namespaces): every harness call needs the sandbox disabled with reason
  "chromium needs its own namespaces outside the command sandbox".
- ~1 s per frame minimum regardless of resolution; keep frames few, size
  small. A run's `errors: []` + a screenshot you actually Read is the
  minimum evidence; work counters (`__settings.counters = true`, then
  `__stats.counters` / `__bench` JSON) are the machine-independent cost
  evidence.
- Scenario = one JS expression evaluated in the page. Existing ones:
  `tools/headless/scenarios/crouch-matrix.js` (Deliverable-0 numbers),
  `corner-shot.js` (headline still; set `window.__cornerMode/View/Exposure`
  in a wrapper — a scenario file takes no arguments).
- Correctness gate for estimator changes: `__compareToReference(configs,
  refFrames, testFrames, refOverrides)` at a small `__benchResolution`; for
  an absolute arbiter render the same pose with `__settings.reference =
  true` (no reuse, no accumulator heuristics) and read `readHDR()`.
- Never `npm install` inside a worktree — `node_modules` is a symlink to
  the trunk clone.
