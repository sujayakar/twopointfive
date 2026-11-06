#!/usr/bin/env python3
# A LOOP THAT ONLY STEPS becomes one call: `for (let i = 0; i < N; i++) W.step(DT);` → `W.steps(N, DT);` (tools/qa/headless.ts: the same steps; a host whose
# game steps somewhere else brings the state across once as the run ends, tools/blitz/PLAY.md). Only the exact shape: the loop's own counter from 0, a bound
# that does not name the counter, a body that is the one call on the harness's world (a receiver named W, W2, w, or ending .W), arguments that do not name the
# counter. Everything else is left as it is.
#   python3 tools/blitz/quiet-loops.py [--write] [files…]        (no file: every file under tools/qa; without --write it only counts)
import re, sys, glob
W = '--write' in sys.argv; files = [a for a in sys.argv[1:] if not a.startswith('--')] or sorted(glob.glob('tools/qa/*.ts') + glob.glob('tools/qa/*/*.ts'))
PAT = re.compile(r'for \(let (\w+) = 0; \1 < ([^;{}]+?); \1\+\+\) (?:\{ ?)?((?:\w+\.)*(?:W\d?|w))\.step\(([^(){};]*)\);( ?\})?')
tot = 0; nf = 0
for f in files:
    if f.endswith('headless.ts'): continue
    s = open(f).read(); n = 0
    def sub(m):
        global n
        v, bound, recv, args, close = m.group(1), m.group(2).strip(), m.group(3), m.group(4).strip(), m.group(5)
        opened = '{' in m.group(0)[:m.group(0).index(recv)].split(')')[-1]
        if re.search(r'\b' + re.escape(v) + r'\b', bound + ' ' + args) or not args or opened != bool(close): return m.group(0)
        n += 1; return f'{recv}.steps({bound}, {args});'
    t = PAT.sub(sub, s)
    if n: tot += n; nf += 1
    if n and W: open(f, 'w').write(t)
    if n and len(files) < 12: print(f'{f}: {n}')
print(f"{tot} loops that only step in {nf} files{' rewritten' if W else ' (counted; --write rewrites them)'}")
