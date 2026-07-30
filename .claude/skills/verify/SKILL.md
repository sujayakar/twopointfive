---
name: verify
description: Runtime verification recipe for twopointfive — build, then drive the renderer through the headless SwiftShader harness (counters, reference compare, screenshots).
---

# Verify (twopointfive)

The runtime surface is the WebGPU renderer in a browser. No GPU here, so
verification runs Chromium + SwiftShader via `tools/headless/run.py` (read
`tools/headless/README.md` first). Timings are meaningless; work counters,
compare-to-reference numbers, validation errors and screenshots are the
evidence.

```bash
npm run build                                    # dist/ is what the harness serves
python3 tools/headless/run.py --scenario <file.js> --shot out.png --json out.json
python3 tools/headless/run.py --bench-res 320 200 --bench-cap-s 2600 \
  --scenario tools/headless/scenarios/<counters|compare>.js --json out.json
```

- Every harness invocation runs OUTSIDE the command sandbox
  (`dangerouslyDisableSandbox`, reason: chromium needs its own namespaces).
- `ok: true` in the result blob = init + all frames clean (WGSL/validation
  errors fail the run). Scenario return value is in `.scenario`.
- ~1 s per frame minimum; keep resolutions small (`__renderer.resize`,
  `--bench-res`). A whole-image `__compareToReference` (140/40 frames, 3
  configs) is ~15 min; background it.
- Read the PNG shots and describe what is actually there; diff against a
  same-pose control shot when the effect is subtle.
- Track scenario files live in `tools/headless/scenarios/`; scratch variants
  go in the scratchpad, not the repo.
