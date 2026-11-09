#!/usr/bin/env python3
"""What the tick reaches outside a seat, naming a seat's field through the game or asking `seats::here(g)`.

    python3 tools/blitz/sweeps/seat-tick.py [--json FILE]

A game of two runs the world's lines of the tick (game/tick.rs, outside `in_seat`) with the first seat standing, so such a function answers for the first player
alone. Found by names (a name mentioned is taken as called), so it over-reaches and misses a call through a stored function. Left out: the seats' own machinery
(seat-ref.py KEEP), the arm of a one-seat test (`one_book`, `seats.len() == 1`), and what is handed to `in_seat`, `with_seat` and their like.
Two lists: functions only the world's lines reach, and the calls from those into a function that a seat's own loop reaches too.
"""
import glob, os, re, sys, json, collections
ROOT = os.path.normpath(os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', '..'))
JSON = sys.argv[sys.argv.index('--json') + 1] if '--json' in sys.argv else None
RULES = os.path.join(ROOT, 'rust', 'game_rules', 'src')
FIELDS = ['seat', 'player', 'orders', 'door_tools', 'work', 'wall_charge', 'aim_point', 'aim_target', 'aim_guard', 'aim_friend', 'aim_snap', 'hover', 'interactables', 'nearby', 'act', 'use_door', 'view_proj', 'threat_proximity', 'torch_on_t']
FIELD_RE = re.compile(r'\bg\.(' + '|'.join(FIELDS) + r')\b(?!\s*\()')
HERE_RE = re.compile(r'\b(?:crate::)?seats::here\(\s*(?:&mut\s+|&\s*)?g\s*\)|\bhere\(\s*g\s*\)')
OPEN_RE = re.compile(r'\b(?:crate::seats::|seats::)?(in_seat|with_seat|each_seat|on_first_bare)\s*\(')
KEEP = {('seats','here'),('seats','seat'),('seats','set_of_game'),('seats','all_down'),('seats','host_man'),('seats','man_by_id'),('seats','view_of_id'),('seats','faced_man'),('seats','faced_view'),('seats','view_of_body'),('seats','view_of'),('seats','orders_of'),('cmd','seat_man')}
def blank(t):
    # strings and comments out, same length
    out = list(t); i = 0; n = len(t)
    while i < n:
        c = t[i]
        if t.startswith('//', i):
            j = t.find('\n', i); j = n if j < 0 else j
            for k in range(i, j): out[k] = ' '
            i = j; continue
        if t.startswith('/*', i):
            j = t.find('*/', i); j = n if j < 0 else j + 2
            for k in range(i, j):
                if t[k] != '\n': out[k] = ' '
            i = j; continue
        if c == '"':
            j = i + 1
            while j < n and t[j] != '"': j += 2 if t[j] == '\\' else 1
            for k in range(i + 1, min(j, n)):
                if t[k] != '\n': out[k] = ' '
            i = j + 1; continue
        if c == "'":
            m = re.match(r"'(\\.[^']*|[^'\\])'", t[i:i+12])
            if m:
                for k in range(i + 1, i + m.end() - 1): out[k] = ' '
                i += m.end(); continue
        i += 1
    return ''.join(out)
def match(t, i, a, b):
    d = 0
    while i < len(t):
        if t[i] == a: d += 1
        elif t[i] == b:
            d -= 1
            if d == 0: return i
        i += 1
    return len(t) - 1
Fn = collections.namedtuple('Fn', 'mod name file line body start params')
fns = []; by_name = collections.defaultdict(list)
for p in sorted(glob.glob(RULES + '/**/*.rs', recursive=True)):
    rel = os.path.relpath(p, RULES); mod = (rel.split(os.sep) + [''])[1].removesuffix('.rs')
    t = blank(open(p, encoding='utf-8').read())
    for m in re.finditer(r'\bfn\s+([a-z_0-9]+)\s*(<[^(]*>)?\s*\(', t):
        par = match(t, m.end() - 1, '(', ')')
        b = t.find('{', par); semi = t.find(';', par)
        if b < 0 or (0 <= semi < b): continue
        e = match(t, b, '{', '}')
        f = Fn(mod, m.group(1), rel, t.count('\n', 0, m.start()) + 1, t[b:e + 1], b, t[m.end():par])
        fns.append(f); by_name[(mod, m.group(1))].append(f)
def lineof(f, i): return f.line + f.body.count('\n', 0, i)
ONE = re.compile(r'\bif\s+(?:crate::seats::|seats::)?one_book\(\s*g\s*\)\s*\{|\bif\s+g\.seats\.len\(\)\s*(?:==\s*1|<=\s*1|<\s*2)\s*\{|\bif\s+(?:n|one|one_seat)(?:\s*==\s*1)?\s*\{')
OPENERS = set()
def split_open(body):
    """the body with the spans of closures handed to in_seat/with_seat/each_seat blanked, and the arms of a one-seat test"""
    spans = []
    for m in ONE.finditer(body):
        b = body.find('{', m.start()); spans.append((m.start(), match(body, b, '{', '}')))
    for m in re.finditer(r'\b([a-z_0-9]+)\s*\(\s*g\b', body):
        if m.group(1) in OPENERS: spans.append((m.start(), match(body, body.find('(', m.start()), '(', ')')))
    for m in OPEN_RE.finditer(body):
        if m.group(1) == 'on_first_bare': continue
        e = match(body, m.end() - 1, '(', ')')
        spans.append((m.start(), e))
    out = list(body)
    for a, b in spans:
        for k in range(a, b + 1):
            if body[k] != '\n': out[k] = ' '
    return ''.join(out)
CALL = re.compile(r'(?:\bcrate::([a-z_0-9]+)(?:::[a-z_0-9]+)*::|\bsuper::|\b)([a-z_][a-z_0-9]*)\s*(?:::<[^>]*>)?\(')
RESERVED = {'if','while','for','match','return','Some','Ok','Err','matches','assert','panic','vec','format','let','in','move','loop','fn','as'}
METHOD = re.compile(r'\.\s*([a-z_][a-z_0-9]*)\s*\(')
def calls(f, body):
    out = []
    for m in CALL.finditer(body):
        name = m.group(2)
        if name in RESERVED: continue
        s = m.start()
        if s > 0 and body[s - 1] == '.': continue
        mod = m.group(1) or f.mod
        if (mod, name) in by_name: out.append((mod, name, s))
    return out
for f in fns:
    if 'FnMut' in f.params and re.search(r'\bwith_seat\b', f.body): OPENERS.add(f.name)
OPENERS -= {'with_seat', 'each_seat'}
# what touches a seat directly
touch = {}
for f in fns:
    if (f.mod, f.name) in KEEP or f.mod == 'seats' and f.file.endswith('seat_ref.rs'): continue
    body = split_open(f.body)
    hits = [(m.group(1), lineof(f, m.start())) for m in FIELD_RE.finditer(body)] + [('here(g)', lineof(f, m.start())) for m in HERE_RE.finditer(body)]
    if hits: touch[(f.mod, f.name)] = hits
graph = {}
for key, L in by_name.items():
    if key in KEEP: graph[key] = []; continue
    s = set()
    for f in L:
        for mod, name, _ in calls(f, split_open(f.body)): s.add((mod, name))
    graph[key] = sorted(s)
tick = by_name[('game', 'step_seats_on_first')][0]
body = split_open(tick.body)
rows = []
seen_roots = []
for mod, name, s in calls(tick, body):
    l = lineof(tick, s)
    if (mod, name) in KEEP: continue
    seen_roots.append((l, mod, name))
seen_roots.sort()
# ---- the split: what a seat's own loop reaches, and what only the world's lines reach
raw = tick.body
seatcalls = []
for m in OPEN_RE.finditer(raw):
    if m.group(1) in ('in_seat', 'with_seat'):
        e = match(raw, m.end() - 1, '(', ')')
        seatcalls += [(c[0], c[1]) for c in calls(tick, raw[m.end():e])]
def reach(starts):
    par = {k: None for k in starts}; q = list(starts)
    while q:
        k = q.pop()
        for c in graph.get(k, []):
            if c not in par: par[c] = k; q.append(c)
    return par
S = reach(seatcalls)
worldroots = [(m, n) for _, m, n in seen_roots]
W = reach(worldroots)
only = {k: W[k] for k in W if k not in S}
def path(k, P):
    out = []
    while k: out.append(k[1]); k = P[k]
    return list(reversed(out))
rows = []
for k in only:
    if k in touch:
        rows.append((k, touch[k], path(k, W)))
edges = collections.defaultdict(list)
for k in only:
    for c in graph.get(k, []):
        if c in S and (c in touch or any(x in touch for x in reach([c]))): edges[c].append(k)
out = {'only': [{'fn': f'{k[0]}::{k[1]}', 'hits': h, 'path': p} for k, h, p in sorted(rows)],
       'edges': {f'{c[0]}::{c[1]}': sorted(f'{x[0]}::{x[1]}' for x in v) for c, v in sorted(edges.items())}}
if JSON: json.dump(out, open(JSON, 'w'), indent=0)
for r in out['only']: print(f"{r['fn']}  [{','.join(sorted({h[0] for h in r['hits']}))}]  {' > '.join(r['path'])}")
for c, v in out['edges'].items(): print(f"{c}  <-  {', '.join(v)}")
print(f"{len(out['only'])} functions only the world's lines reach; {len(out['edges'])} calls into what a seat's loop reaches too", file=sys.stderr)
