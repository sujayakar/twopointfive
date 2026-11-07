#!/usr/bin/env python3
# THE SWEEP'S TABLE (tools/blitz/PLAY.md, "The sweep"): every family of tools/qa/probes.ts run three ways, a probe a row, and from it the rows
# `TPS_GAME=table` reads (tools/blitz/qa-kinds.json: `sweep`, `pins`, `both`).
#
#   the runs (one process a family or a stand-alone name, each capped; the outputs in DIR/<way>/<unit>.txt, a line a unit in DIR/done.<way>.txt
#   `<unit> <way> exit <code> <seconds> s`):
#     a   QA_VERDICTS=1 bun run tools/qa/probes.ts <unit>          the TypeScript tick, no word
#     b   QA_FOLD=1 bun run tools/qa/probes.ts <unit>              the TypeScript tick under the fold: THE REFERENCE
#     c   TPS_GAME=rust bun run tools/qa/probes.ts <unit>          the Rust game
#   python3 tools/blitz/qa-sweep.py DIR [--kinds tools/blitz/qa-kinds.json] [--md SWEEP.md] [--list <probes.ts list output>] [--write]
#
# A PROBE'S ROW: on the Rust game where it is GREEN there, or RED there and on the reference WITH THE SAME FAILING LINES (staging debt: not the Rust
# game's); kept on the TypeScript tick where it is green on the reference and red on the Rust game (the Rust game's or the bridge's red: listed with its
# first failing line and a kind), where both are red on other lines, and where the Rust game was not measured (cut at the cap, a process that died).
# THE KIND OF A RUST-ONLY RED IS A LEAD, read off the run and not proved: a panic · the host refuses · a planted answer (the probe's file replaces a
# function to answer for it: SURFACE.md) · a row Rust has not got · a rule not on the surface (counted for the unit's whole process) · a write the scan
# does not see (the census's kind ii with none of the above) · a difference in play.
import json, re, os, sys, glob, collections
A = sys.argv[1:]
def arg(k, d=None):
    return A[A.index(k) + 1] if k in A else d
OUT = A[0] if A and not A[0].startswith('--') else '.'
HERE = os.path.dirname(os.path.abspath(__file__)); ROOT = os.path.dirname(os.path.dirname(HERE))
KINDS = arg('--kinds', HERE + '/qa-kinds.json')

def units_done(w):
    d = {}; p = f'{OUT}/done.{w}.txt'
    if not os.path.exists(p): return d
    for l in open(p):
        m = re.match(r'(\S+) (\w) exit (\d+) (\d+) s', l)
        if m: d[m.group(1)] = {'exit': int(m.group(3)), 's': int(m.group(4))}
        m = re.match(r'(\S+) (\w) covered', l)
        if m: d[m.group(1)] = {'covered': True}
    return d
FAIL = re.compile(r'^\s*- FAIL — (.*)')
ERR = re.compile(r"(the Rust game's tick \d+ panicked[^\n]{0,220}|panicked[^\n]{0,220}|rust-host: [^\n]{0,280}|\b(?:TypeError|RangeError|ReferenceError|RuntimeError|Error): [^\n]{0,220})")
SURF = re.compile(r'the call surface: (\d+) crossed · (\d+) ran the TypeScript with no host \((.*?)\) · (\d+) on a row Rust has not got \((.*?)\) · (\d+) not on the surface \((.*)\)')
def parse(w):
    P = {}; U = {}
    for f in sorted(glob.glob(f'{OUT}/{w}/*.txt')):
        unit = os.path.basename(f)[:-4]; fails = []; errs = []; u = {'probes': [], 'surface': None}
        for l in open(f, errors='replace').read().split('\n'):
            m = FAIL.match(l)
            if m: fails.append(m.group(1)[:220]); continue
            if l.startswith('the call surface:'):
                m = SURF.match(l)
                if m: u['surface'] = {'crossed': int(m.group(1)), 'noHost': int(m.group(2)), 'noRow': int(m.group(4)), 'noRowWho': m.group(5)[:300], 'unlisted': int(m.group(6)), 'unlistedWho': m.group(7)[:400]}
                continue
            s = l.strip()
            if s.startswith('{"fold":') or s.startswith('{"plain":'):
                try: j = json.loads(s)
                except Exception: continue
                k = 'fold' if 'fold' in j else 'plain'; r = j[k]; name = r.get('probe')
                if name:
                    row = {'red': bool(r.get('red')), 'unit': unit, 'fails': fails, 'errs': errs[:3]}
                    if k == 'fold': row['refused'] = sorted(set(f"{x['row']}={json.dumps(x['value'])}" for x in r.get('refused', [])))
                    if name not in P or unit == name: P[name] = row   # (a probe two units ran: its own unit's run, else the first seen)
                    u['probes'].append(name)
                fails = []; errs = []; continue
            m = ERR.search(l)
            if m and len(errs) < 6: errs.append(m.group(1)[:280])
        u['tail'] = errs[:2]; U[unit] = u
    for u, d in units_done(w).items(): U.setdefault(u, {'probes': [], 'surface': None, 'tail': []}).update(d)
    return P, U
R = {w: parse(w) for w in 'abc'}
K = json.load(open(KINDS)) if os.path.exists(KINDS) else {'probes': {}}
# the probes' files (probes.ts: `run('<name>', … import('./<file>.ts')`) and the files that plant an answer (SURFACE.md)
FILE = {}
try:
    for m in re.finditer(r"run\('([^']+)'[^\n]*?import\('\./([\w./-]+\.ts)'\)", open(ROOT + '/tools/qa/probes.ts').read()): FILE.setdefault(m.group(1), m.group(2))
except Exception: pass
PLANT = set()
try:
    t = open(HERE + '/SURFACE.md').read(); t = t[t.index('## Probes that plant an answer'):]
    PLANT = set(m.group(1) for m in re.finditer(r'\| tools/qa/([\w./-]+\.ts):\d+ \|', t))
except Exception: pass
num = lambda s: re.sub(r'\d+(?:\.\d+)?', '#', s)
def kind_of(name, c, unit):
    errs = ' '.join(c['errs']); S = (unit or {}).get('surface') or {}
    if 'panicked' in errs: return 'a panic'
    if 'rust-host:' in errs: return 'the host refuses'
    if FILE.get(name) in PLANT: return 'a planted answer'
    if S.get('noRow'): return 'a row Rust has not got'
    if S.get('unlisted'): return 'a rule not on the surface'
    if K.get('probes', {}).get(name) == 'ii': return 'a write the scan does not see'
    return 'a difference in play'
Pa, Ua = R['a']; Pb, Ub = R['b']; Pc, Uc = R['c']
names = sorted(set(K.get('probes', {})) | set(Pa) | set(Pb) | set(Pc))
rows = {}
for n in names:
    a, b, c = Pa.get(n), Pb.get(n), Pc.get(n); r = {'a': None if a is None else ('red' if a['red'] else 'green'), 'b': None if b is None else ('red' if b['red'] else 'green'), 'c': None if c is None else ('red' if c['red'] else 'green')}
    if c: r['refused'] = c.get('refused', [])
    elif b: r['refused'] = b.get('refused', [])
    if c is None: r.update(on='ts', as_='not measured on the Rust game')
    elif not c['red']: r.update(on='rust', as_='green on the Rust game' if (b is None or not b['red']) else 'green on the Rust game, RED on the reference')
    elif b is None: r.update(on='ts', as_='red on the Rust game, the reference not measured', first=(c['fails'] or c['errs'] or ['?'])[0])
    elif b['red']:
        same = c['fails'] == b['fails']; near = [num(x) for x in c['fails']] == [num(x) for x in b['fails']]
        if same or near: r.update(on='rust', as_='red on both, the same failing lines' + ('' if same else ' but for their numbers'))
        else: r.update(on='ts', as_='red on both, other failing lines', first=next((x for x in c['fails'] if x not in b['fails']), (c['errs'] or ['?'])[0]))
    else: r.update(on='ts', as_='RED ON THE RUST GAME ALONE', kind=kind_of(n, c, Uc.get(c['unit'])), first=(c['fails'] or c['errs'] or ['?'])[0], err=(c['errs'] or [''])[0])
    rows[n] = r
json.dump({'rows': rows, 'units': {w: R[w][1] for w in 'abc'}}, open(f'{OUT}/rows.json', 'w'))
C = collections.Counter(r['as_'] for r in rows.values()); probes = [n for n in names if n in K.get('probes', {})] or names
onr = [n for n in probes if rows[n]['on'] == 'rust']; alone = {n: r for n, r in rows.items() if r['as_'] == 'RED ON THE RUST GAME ALONE'}
byk = collections.defaultdict(list)
for n, r in alone.items(): byk[r['kind']].append(n)
L = []
def unit_line(w, label):
    U = R[w][1]; ran = [u for u in U.values() if 'exit' in u]
    return f"| {label} | {len(ran)} | {sum(1 for u in U.values() if u.get('covered'))} | {sum(1 for u in ran if u['exit'] in (124, 137))} | {len(R[w][0])} | {sum(1 for p in R[w][0].values() if p['red'])} | {sum(u['s'] for u in ran)} |"
L += ['# The sweep: every family three ways', '', '| way | units run | names a family had already run | cut at the cap | probes with a verdict | red | process seconds |', '|---|---|---|---|---|---|---|', unit_line('a', 'a: the TypeScript tick, no word'), unit_line('b', 'b: the TypeScript tick under the fold (the reference)'), unit_line('c', 'c: the Rust game'), '']
L += [f'**{len(onr)} of the {len(probes)} probes stand on the Rust game by these rows.**', '', '| the row | probes |', '|---|---|'] + [f'| {k} | {v} |' for k, v in C.most_common()] + ['']
S = [u['surface'] for u in Uc.values() if u.get('surface')]
if S: L += [f"The call surface over the {len(S)} processes of way c that printed their exit line: {sum(s['crossed'] for s in S)} crossed · {sum(s['noHost'] for s in S)} ran the TypeScript with no host · {sum(s['noRow'] for s in S)} on a row Rust has not got · {sum(s['unlisted'] for s in S)} not on the surface.", '']
L += ['## Red on the Rust game alone, by kind (a lead read off the run, not a proof)', '', '| kind | probes |', '|---|---|'] + [f'| {k} | {len(v)} |' for k, v in sorted(byk.items(), key=lambda x: -len(x[1]))] + ['']
for k, v in sorted(byk.items(), key=lambda x: -len(x[1])):
    L += [f'### {k} ({len(v)})', '', '| probe | first failing line | what was thrown |', '|---|---|---|']
    for n in sorted(v): L.append(f"| {n} | {alone[n]['first'][:150].replace('|', '/')} | {alone[n].get('err', '')[:170].replace('|', '/')} |")
    L.append('')
other = {n: r for n, r in rows.items() if r['as_'] == 'red on both, other failing lines'}
L += [f'## Red on both, on other failing lines ({len(other)}): kept', '', '| probe | the first line the Rust game fails that the reference does not |', '|---|---|'] + [f"| {n} | {r['first'][:200].replace('|', '/')} |" for n, r in sorted(other.items())] + ['']
gr = sorted(n for n, r in rows.items() if r['as_'] == 'green on the Rust game, RED on the reference')
L += [f'## Green on the Rust game and red on the reference ({len(gr)})', '', ' '.join(gr), '']
nm = sorted(n for n in probes if rows[n]['as_'] == 'not measured on the Rust game')
L += [f'## Not measured on the Rust game ({len(nm)}): kept', '', ' '.join(nm), '']
cut = sorted(u for u, d in Uc.items() if d.get('exit') in (124, 137))
L += [f'## Units of way c cut at the cap ({len(cut)})', '', ' '.join(cut), '']
md = '\n'.join(L)
if arg('--md'): open(arg('--md'), 'w').write(md)
print('\n'.join(L[:40]))
if '--write' in A:
    K['sweep'] = {n: ({'on': r['on'], 'as': r['as_']} | ({'why': f"{r['kind']}: {r['first'][:120]}"} if 'kind' in r else {})) for n, r in sorted(rows.items())}
    # (`pins`: a check that writes a settled switch row, by the fold's own line: what it tried and was refused; `both`: said as such a check stands a game)
    K['pins'] = {n: r['refused'] for n, r in sorted(rows.items()) if r.get('refused')}
    K['both'] = {n: 'seen red on the Rust game and on the folded TypeScript tick with the same failing lines, in the sweep' for n, r in sorted(rows.items()) if r['as_'].startswith('red on both, the same')}
    K.pop('waits', None)
    json.dump(K, open(KINDS, 'w'), indent=0)
    print(f'wrote {KINDS}: {len(K["sweep"])} rows, {len(K["pins"])} pins, {len(K["both"])} both')
