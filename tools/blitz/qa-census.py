#!/usr/bin/env python3
# WHAT EVERY CHECK OF tools/qa DOES TO THE GAME BETWEEN TWO TICKS, read off its source: the size of the work to run the checks on a game that steps
# somewhere else (tools/blitz/PLAY.md). A probe (a name `tools/qa/probes.ts <name>` runs) is followed from its line in probes.ts into the function it
# runs and every function of tools/qa that one reaches by name; a stand-alone tool is read whole.
#   python3 tools/blitz/qa-census.py --calls <calls.json> --names <by-name-only.json> --out DIR      (from the repository's root)
#   calls.json       the inventory of functions called from outside the tick, each with what it writes (key, verdict, writes)
#   by-name-only.json  per file, the game's field names it writes by name
# KINDS (the worst thing a check does decides its kind):
#   0      stands no game and calls nothing of the simulation (a check of a tool, a table, the picture's code)
#   i      plays through the host's input, steps and reads (a start or a load between ticks is the host's: a new start)
#   ii     … and writes the game's fields by name between ticks
#   iii    … and calls a rule of the simulation between ticks (a function of src/game, anim, math, sim, scene that writes game state)
#   iv-a   hangs its own function on an object of the game (a wrapper the tick is meant to call): another stepper never calls it. A wrapper on the HUD's
#          `msg` / `say` / `note` alone is not counted here but flagged (`hud`): one out event's arm calling the game's own method would reach it
#   iv-b   calls the simulation's functions with no game stood: it checks the TypeScript function itself
#   iv-c   holds the game to a word made by walking the JavaScript objects (rollback.ts stateHash / canon), or replays a session journal (the sensors' seam)
# It is a reading by name: a call through a variable, a write through a helper of another folder, and the order of a write and the first step are not seen.
import re, os, sys, glob, json, collections
A = sys.argv; arg = lambda n, d=None: A[A.index(n) + 1] if n in A else d
OUT = arg('--out', '.'); os.makedirs(OUT, exist_ok=True)
# --folded <port-list.json>: the switch rows with each one's value on the page and its kind. A `settled` row is a constant in the Rust game (the branch
# the page takes was translated and the other dropped), so a check that puts such a row on its other side checks, on the Rust game, nothing: FOLDED.md
SETTLED = {r['row']: r['page'] for r in json.load(open(arg('--folded')))['rows'] if r.get('kind') == 'settled'} if arg('--folded') else {}
LIT = lambda v: True if v == 'true' else False if v == 'false' else float(v) if re.match(r'^-?\d+(\.\d+)?$', v) else None
def switch_writes(T):
    """every write of a settled row by its registry name in a text: (row, the value written as text, 'off' | 'page' | 'var')"""
    out = set()
    if not SETTLED: return out
    for m in re.finditer(r'(?<![\w.])([A-Z][A-Z0-9_]{2,})((?:\.\w+){1,3})\s*=(?!=)\s*([^;,)\n]{1,60})', T):
        row = m.group(1) + m.group(2)
        if row not in SETTLED: continue
        v = m.group(3).strip(); lit = LIT(v); page = SETTLED[row]
        same = lit is not None and (lit == page or (isinstance(page, bool) and lit in (0.0, 1.0) and bool(lit) == page))
        out.add((row, v[:40], 'var' if lit is None else 'page' if same else 'off'))
    return out
CALLS = json.load(open(arg('--calls')))['functions'] if arg('--calls') else []
BYNAME = json.load(open(arg('--names'))) if arg('--names') else {}
QA = 'tools/qa/'
snake = lambda s: re.sub(r'([a-z0-9])([A-Z])', r'\1_\2', s).lower()

# ---- the simulation's functions: the port's signatures (every function of the tick has one), and the inventory's verdicts
SIG = collections.defaultdict(list)   # snake name -> [(module, args)]
for f in glob.glob('rust/game/src/*/sig.rs'):
    mod = f.split('/')[-2]
    for m in re.finditer(r'pub fn (\w+)\(([^{]*)\)[^{]*\{', open(f).read()): SIG[m.group(1)].append((mod, m.group(2)))
SCRATCH = re.compile(r'^(StaticCollision\.|SkyState\.|BoxWorld\.|AABB\.|AirDust\.|Weather\.|Box\.(albedo|aniso|flags|rough)$|Game\.sim$|PropSystemStats\.)')
INV = {}   # TS name -> {'verdict', 'writes' (not scratch), 'n'}
for f in CALLS:
    n = re.split(r'[#.]', f['key'])[-1]; real = [w for w in (f.get('writes') or []) if not SCRATCH.match(w)]
    if re.match(r'^(Input|FollowCamera|Sensors|FlatState)\.', f['key']): continue   # (the host's own: its input, its camera, the sensors' frame)
    e = INV.setdefault(n, {'keys': [], 'real': 0, 'n': 0, 'verdict': set(), 'dice': False}); e['keys'].append(f['key']); e['real'] = max(e['real'], len(real)); e['n'] = max(e['n'], f.get('writesN') or 0); e['verdict'].add(f['verdict']); e['dice'] = e['dice'] or bool(f.get('dice'))
STARTS = {'loadGltfCast', 'loadGltfCharacter', 'parseLevelData', 'startFromQuery', 'startScenario', 'restartEncounter', 'restart', 'loadState', 'quitScenario', 'restartMission', 'startMission', 'saveState', 'canLoad', 'loadLevel', 'standGame'}
def judge(name):
    """start · rule · ask · unjudged (a ported function that takes the game to write and the inventory never met) · none (not the simulation's)"""
    if name in STARTS: return 'start'
    e = INV.get(name)
    if e: return 'rule' if e['real'] > 0 else 'ask'
    s = SIG.get(snake(name))
    if not s: return 'none'
    return 'unjudged' if any(re.search(r'&mut (crate::game::)?Game\b', a) for _, a in s) else 'ask'
def crossing(name):
    """what of a function's arguments cannot cross to another stepper as bytes: a closure; an object written through that has no id"""
    out = set()
    for _, a in SIG.get(snake(name), []):
        for x in re.split(r',\s*(?=\w+:)', a):
            if ':' not in x: continue
            nm, ty = x.split(':', 1); ty = ty.strip()
            if re.search(r'\bdyn (Fn|FnMut|FnOnce)\b|\bfn\(|impl Fn', ty): out.add(f'{nm.strip()}: a closure')
            elif re.search(r'&mut ', ty) and not re.search(r'&mut (crate::game::)?Game\b', ty) and not re.search(r'&mut (V3|f64|bool|Vec<f64>|\[f64)', ty): out.add(f'{nm.strip()}: {ty} (written through, no id)')
    return sorted(out)

# ---- tools/qa's files, cut into their top-level functions
FROZEN = re.compile(r"src/(game|anim|math|sim|scene)/")
class File:
    def __init__(s, path):
        s.path = path; s.text = open(path, errors='ignore').read(); s.fns = {}; s.frozen = {}; s.local = {}
        starts = [(m.start(), m.group(1)) for m in re.finditer(r'^(?:export\s+)?(?:async\s+)?function\*?\s+(\w+)', s.text, re.M)]
        starts += [(m.start(), m.group(1)) for m in re.finditer(r'^(?:export\s+)?(?:const|let)\s+(\w+)\b', s.text, re.M)]   # (every top-level name: a function held in a const, wrapped or not, and a table)
        starts.sort()
        for i, (at, name) in enumerate(starts): s.fns[name] = s.text[at:(starts[i + 1][0] if i + 1 < len(starts) else len(s.text))]
        s.head = s.text[:starts[0][0]] if starts else s.text
        for m in re.finditer(r"import\s*(?:type\s*)?\{([^}]*)\}\s*from\s*['\"]([^'\"]+)['\"]", s.text): s.imp(m.group(1), m.group(2), ' as ')
        for m in re.finditer(r"\{([^}=]*)\}\s*=\s*(?:await\s+)?\(?\s*(?:await\s+)?import\(\s*[`'\"]([^`'\"]+)[`'\"]", s.text): s.imp(m.group(1), m.group(2), ':')
    def imp(s, names, frm, sep):
        for n in names.split(','):
            n = n.strip()
            if not n or n.startswith('type '): continue
            orig, _, alias = n.partition(sep); orig = orig.strip(); alias = (alias or orig).strip()
            if FROZEN.search(frm): s.frozen[alias] = orig
            elif re.match(r'^\.\/[\w.-]+\.ts$', frm) or '/tools/qa/' in frm: s.local[alias] = (QA + os.path.basename(frm), orig)
FILES = {}
HARNESS = {'headless.ts', 'stand.ts', 'holds.ts', 'expected-red.ts', 'webgpu-stub.ts'}   # (the stand itself: what it does is the host's)
def file(p):
    if p not in FILES and os.path.exists(p): FILES[p] = File(p)
    return FILES.get(p)
def closure(path, fn):
    """the text a probe reaches: its function, the functions of tools/qa it names (in its file and through its file's imports), each file's head once"""
    seen, heads, texts, todo = set(), set(), [], [(path, fn)]
    while todo:
        p, f = todo.pop(); F = file(p)
        if not F or (p, f) in seen or os.path.basename(p) in HARNESS: continue
        seen.add((p, f))
        if p not in heads: heads.add(p); texts.append((p, F.head))
        body = F.text if f is None else F.fns.get(f)
        if body is None and (p, f) == (path, fn): f, body = None, F.text   # (the probe's own function not found by name: its file is read whole)
        if body is None: continue
        texts.append((p, body))
        nxt = [(QA + m.group(1), m.group(2)) for m in re.finditer(r"import\(\s*['`\"]\./([\w.-]+)['`\"]\s*\)\s*\)(?: as any\))?\.(\w+)", body)]
        if f is None: nxt += [(p2, o) for (p2, o) in F.local.values()]
        else: nxt += [(p, n) for n in F.fns if n != f and re.search(r'(?<![\w.])' + re.escape(n) + r'\b', body)] + [F.local[a] for a in F.local if re.search(r'(?<![\w.])' + re.escape(a) + r'\b', body)]
        todo += nxt
    return texts, heads
HOST_LHS = re.compile(r'^(?:W|w|H|P|world)?\.?(?:input|cam|canvas|audioLog|I\d?|inp)\b|^(?:console|process|Math|globalThis|module|exports|opts?|o|args?|res|out|r|R|F|T|S|rec|row|acc|st|stats?|ctx|cfg|env)\.')
OUT_CALLS = re.compile(r'(audio\.(play|footstep|rain)|smoke\.(emit|push|keepHot|clearAll|obstaclesChanged)|particles\.(emit|impulse|setHeat)|decals\.(add|touch|clear)|saves\.\w+|journal\.cut|engine\.\w+|sensors\.\w+)$')   # (what the out events' dispatcher calls on the host's objects, and the host's own hooks)
WRAP_OK = re.compile(r'^(console|process|Math|globalThis|window|document|performance|Date|JSON|Bun)\.')
def read(path, fn):
    texts, heads = closure(path, fn); T = '\n'.join(t for _, t in texts)
    frozen = {};
    for p in heads: frozen.update(FILES[p].frozen)
    names = set();
    for p in heads: names.update(BYNAME.get(p, {}).get('writes', []))
    r = {'stand': bool(re.search(r'\b(standUp|standTwo|runLesson|standGame|runScript)\(|(?<![\w.])play\(|\bnew Game\(', T)), 'steps': len(re.findall(r'\.step\(|\bstep\(\s*W\b|\.update\(\s*(?:dt|DT|1\s*/)|stepSeats\(', T)),
         'rules': set(), 'asks': set(), 'unjudged': set(), 'starts': set(), 'writes': set(), 'wraps': set(), 'word': set(), 'hud': set(), 'switch': switch_writes(T)}
    called = set(n for n in frozen if re.search(r'(?<![\w.])' + re.escape(n) + r'\s*\(', T))
    for m in re.finditer(r'\.([a-z][A-Za-z0-9]+)\s*\(', T):
        n = m.group(1)
        if n in INV and re.search(r'[A-Z]', n) and any('.' in k for k in INV[n]['keys']) and (INV[n]['real'] > 0 or n in STARTS): called.add(n); frozen.setdefault(n, n)
    if re.search(r'\borders\.give\(|\.give\(\s*[\'"`]', T): r['rules'].add('Orders.give')
    for a in called:
        n = frozen.get(a, a); k = judge(n)
        if k == 'rule': r['rules'].add(n)
        elif k == 'ask': r['asks'].add(n)
        elif k == 'unjudged': r['unjudged'].add(n)
        elif k == 'start': r['starts'].add(n)
    for m in re.finditer(r'((?:[\w$\]\[\'"?!]+\.)+)(\w+)\s*(?:\[[^\]\n]{0,40}\]\s*)*(?:[-+*/|&]|\|\||&&|\?\?)?=(?!=)', T):
        lhs, n = m.group(1), m.group(2)
        if n in names and not HOST_LHS.match(lhs + n): r['writes'].add(n)
    for m in re.finditer(r'((?:[\w$\]\[\'"?!]+\.)+)(\w+)\s*=\s*(?:async\s*)?(?:function\b|\([^()\n]{0,80}\)\s*(?::\s*\w+\s*)?=>|\w+\s*=>)', T):
        full = m.group(1) + m.group(2)
        if not WRAP_OK.match(full) and not HOST_LHS.match(full) and not re.match(r'^(?:W|w)\.(?:step|snapshot|restore)$', full) and m.group(2) not in ('onmessage', 'onerror', 'onclose', 'onopen', 'log', 'error', 'warn', 'random') and not OUT_CALLS.search(full): (r['hud'] if m.group(2) in ('msg', 'say', 'note') else r['wraps']).add(full)
    if re.search(r'\bstateHash\(|\bcanon\(|rollbackVar\(|from [\'"].*rollback\.ts', T) and re.search(r'\bstateHash\(|\bcanon\(', T): r['word'].add('stateHash / canon (a walk of the JavaScript objects)')
    if re.search(r'journal\.ts|replay\.ts|SessionRecorder|replaySession|\brecorder\b', T) and re.search(r'\b(replaySession|SessionRecorder|recordSession|replayFile|replayJournal|runReplay)\b', T): r['word'].add('a session journal (the sensors\' seam)')
    sim = r['rules'] | r['asks'] | r['unjudged']
    if r['word']: k = 'iv-c'
    elif r['wraps'] and r['stand']: k = 'iv-a'
    elif not r['stand'] and sim: k = 'iv-b'
    elif not r['stand']: k = '0'
    elif r['rules'] or r['unjudged']: k = 'iii'
    elif r['writes']: k = 'ii'
    else: k = 'i'
    r['kind'] = k; return r

# ---- the probes: each name's line in probes.ts
PT = open(QA + 'probes.ts').read(); TARGET = {}   # probe name -> [(file, fn)]
for m in re.finditer(r"run\('([^']+)',\s*(?:async\s*)?\(\)\s*=>\s*\(+await import\('\./([^']+)'\)\)(?: as any\))?\.(\w+)\(", PT): TARGET.setdefault(m.group(1), []).append((QA + m.group(2), m.group(3)))
for m in re.finditer(r"run\('([^']+)',\s*\(\)\s*=>\s*runScript\('([^']+)'", PT): TARGET.setdefault(m.group(1), []).append((QA + m.group(2), None))
PF = file(QA + 'probes.ts')
for m in re.finditer(r"run\('([^']+)',\s*(?:async\s*)?\(\)\s*=>\s*([^\n]*)", PT):
    if m.group(1) in TARGET: continue
    fns = [n for n in PF.fns if re.search(r'(?<![\w.])' + re.escape(n) + r'\(', m.group(2).split('//')[0])]
    if fns: TARGET[m.group(1)] = [(QA + 'probes.ts', n) for n in fns]
# (a family run by a loop over [name, function] pairs: the pairs are read where they are literals, the file's every function otherwise)
LOOPS = []
for m in re.finditer(r"on\((\w+),\s*'([\w-]+)'[^\n]*run\(\1,[^\n]*import\('\./([^']+)'\)\)(?: as any\))?\[(\w+)\]", PT): LOOPS.append((m.start(), m.group(2), QA + m.group(3)))
for at, fam, path in LOOPS:
    back = PT[max(0, at - 6000):at]
    for a, b in re.findall(r"\[\s*'([\w/-]+)'\s*,\s*'(\w+)'", back):
        if a not in TARGET and (a == fam or a.startswith(fam[:4])): TARGET[a] = [(path, b)]
LIST = {}   # family -> members, from `probes.ts list` (the file given with --list)
ALLN = set()
if arg('--list'):
    for l in open(arg('--list')):
        m = re.match(r'^\s+([\w()-]+):? (.*)$', l)
        if not m or l.startswith('the '): continue
        fam = m.group(1).rstrip(':'); mem = m.group(2).split()
        if fam == '(stand-alone)':
            for x in mem: LIST.setdefault(x, []).append(x); ALLN.add(x)
        else: LIST[fam] = mem; ALLN.update(mem); ALLN.add(fam)
for at, fam, path in LOOPS:
    for n in LIST.get(fam, []):
        if n not in TARGET: TARGET[n] = [(path, None)]
def merge(rs):
    order = ['0', 'i', 'ii', 'iii', 'iv-b', 'iv-a', 'iv-c']; out = {'kind': '0', 'stand': False, 'steps': 0}
    for k in ('rules', 'asks', 'unjudged', 'starts', 'writes', 'wraps', 'word', 'hud', 'switch'): out[k] = set()
    for r in rs:
        for k in ('rules', 'asks', 'unjudged', 'starts', 'writes', 'wraps', 'word', 'hud', 'switch'): out[k] |= r[k]
        out['stand'] = out['stand'] or r['stand']; out['steps'] += r['steps']
        if order.index(r['kind']) > order.index(out['kind']): out['kind'] = r['kind']
    return out
PROBE = {n: merge([read(p, f) for p, f in t]) for n, t in TARGET.items()}
# ---- the stand-alone gate tools, each read whole
TOOLS = ['playthrough.ts', 'tour.ts', 'soak.ts', 'seats-run.ts', 'seats-check.ts', 'coop-check.ts', 'mpserver-check.ts', 'mp-control-check.ts', 'replay.ts', 'samestart-check.ts', 'lesson-run.ts', 'mission-table.ts']
TOOL = {t: read(QA + t, None) for t in TOOLS if os.path.exists(QA + t)}
# ---- the settled switches a check puts on their other side (--folded): by a write of the row (read above), or by the table of holds (tools/qa/holds.ts:
# a probe owed the build before a rule runs with that rule's row off on every stand)
HELD = collections.defaultdict(set)   # probe -> {(row, list)}
if SETTLED and os.path.exists(QA + 'holds.ts'):
    for m in re.finditer(r"\{ list: '(\w+)', names: \[([^\]]*)\], holds: \[([^\n]*)\] \}", open(QA + 'holds.ts').read()):
        rows = [f"{t}.{k or 'on'}" for _, t, k in re.findall(r"(row|atStandUp)\('\w+', '(\w+)'(?:, '(\w+)')?\)", m.group(3))]
        pats = re.findall(r"'([^']+)'", m.group(2))
        for n in PROBE:
            for p in pats:
                hit = (n == p[:-4] or n.startswith(p[:-4] + '-')) if p.endswith('(-*)') else n.startswith(p[:-1]) if p.endswith('*') else n == p
                if hit:
                    for r in rows: HELD[n].add((r, m.group(1)))
OFFNAME = re.compile(r'(^|-)(off|identity|base|frozen|old|asitwas|switch(es)?|hold(s)?)($|-)')
FAM_OF = {}
for fam, mem in LIST.items():
    for n in mem: FAM_OF.setdefault(n, fam)
FOLD = {}
for n, r in PROBE.items():
    off = sorted(x for x in r['switch'] if x[2] == 'off'); var = sorted(x for x in r['switch'] if x[2] == 'var'); held = sorted(x for x in HELD.get(n, ()) if x[0] in SETTLED)
    if not off and not held and not var: continue
    # THE SUBJECT, by reading: a probe owed the build before a rule (the table of holds), or named for the off side, checks the game with the rule off:
    # that game is not in the Rust game. One that puts a row off beside its own subject (to stage it, to keep another rule out of the way) is to be aimed again.
    subj = 'the off side (a game the Rust game does not have)' if held or (OFFNAME.search(n) and (off or var)) else 'a rule, with a settled row pinned off beside it (aim it again)' if off else 'both sides by a variable (an off-against-on line: its off half is the off side)'
    FOLD[n] = {'family': FAM_OF.get(n, '?'), 'kind': r['kind'], 'off': [f'{a} = {b}' for a, b, _ in off], 'var': [f'{a} = {b}' for a, b, _ in var], 'held': [f'{a} ({b})' for a, b in held], 'subject': subj}
if SETTLED:
    byfam = collections.defaultdict(list)
    for n, e in FOLD.items(): byfam[e['family']].append(n)
    cnt = collections.Counter(e['subject'] for e in FOLD.values())
    with open(f'{OUT}/FOLDED.md', 'w') as o:
        o.write('# Checks that put a SETTLED switch on its other side (tools/blitz/qa-census.py --folded, by reading)\n\n')
        o.write(f'A settled row is a constant in the Rust game: the branch the page takes was translated, the other dropped. {len(SETTLED)} rows are settled. Of {len(PROBE)} probes read, {len(FOLD)} write one, by a literal that is not the page\'s value (`off`), by the table of holds (tools/qa/holds.ts: the probe is owed the build before a rule), or by a variable (`var`: both sides in turn, or the old value put back).\n\n')
        o.write('Read by name: a row written through an alias (`const R = RESPONSE`), by `Object.assign`, by a stand word in the environment (`TPS_X=0`) or by the old stand as a whole (`TPS_PROFILE=frozen`) is not seen. The subject is judged by the probe\'s name and the way the row is put off, not by reading what the probe asserts.\n\n')
        o.write('| the subject | probes |\n|---|---:|\n' + ''.join(f'| {k} | {v} |\n' for k, v in cnt.most_common()) + '\n')
        o.write('| family | probes | the off side itself | a rule with a row pinned off | by a variable only | rows |\n|---|---:|---:|---:|---:|---|\n')
        for fam in sorted(byfam, key=lambda f: -len(byfam[f])):
            E = [FOLD[n] for n in byfam[fam]]; rows = collections.Counter(x.split(' ')[0] for e in E for x in e['off'] + e['held'] + e['var'])
            o.write(f"| {fam} | {len(E)} | {sum(1 for e in E if e['subject'].startswith('the off'))} | {sum(1 for e in E if e['subject'].startswith('a rule'))} | {sum(1 for e in E if e['subject'].startswith('both'))} | {', '.join(f'{k} x{v}' for k, v in rows.most_common(6))}{' …' if len(rows) > 6 else ''} |\n")
        for title, pre in (('The off side itself: for the deletion list (not deleted)', 'the off'), ('A rule, with a settled row pinned off beside it: aim again', 'a rule'), ('Both sides by a variable', 'both')):
            o.write(f'\n## {title}\n\n| probe | family | kind | put off by a literal | held (holds.ts) | by a variable |\n|---|---|---|---|---|---|\n')
            for n in sorted(FOLD, key=lambda n: (FOLD[n]['family'], n)):
                e = FOLD[n]
                if e['subject'].startswith(pre): o.write(f"| {n} | {e['family']} | {e['kind']} | {'; '.join(e['off'][:6])} | {'; '.join(e['held'][:6])} | {'; '.join(e['var'][:4])}{' …' if len(e['var']) > 4 else ''} |\n")
    print(f"settled rows put on their other side: {len(FOLD)} probes ({dict(cnt)}) -> {OUT}/FOLDED.md")
# ---- out
ser = lambda r: {k: (sorted(v) if isinstance(v, set) else v) for k, v in r.items()}
fam_rows = {}
for fam, mem in LIST.items():
    ks = collections.Counter(PROBE[n]['kind'] if n in PROBE else '?' for n in mem)
    fam_rows[fam] = {'members': len(mem), 'kinds': dict(ks), 'unmapped': [n for n in mem if n not in PROBE]}
rule_files = collections.defaultdict(set); rule_probes = collections.Counter(); ask_probes = collections.Counter(); unj_probes = collections.Counter(); write_probes = collections.Counter(); wrap_probes = collections.Counter()
for n, r in PROBE.items():
    for f in r['rules']: rule_probes[f] += 1
    for f in r['asks']: ask_probes[f] += 1
    for f in r['unjudged']: unj_probes[f] += 1
    for f in r['writes']: write_probes[f] += 1
    for f in r['wraps']: wrap_probes[f] += 1
# (a rule's count of FILES: every file of tools/qa that names it as a call, whichever probe it serves)
ALLQA = sorted(glob.glob(QA + '**/*.ts', recursive=True)); file_count = collections.Counter()
WANT = {f.split('.')[-1]: f for f in set(rule_probes) | set(unj_probes) | set(ask_probes)}
for p in ALLQA:
    for n in set(re.findall(r'(?<![\w])(\w+)\s*\(', open(p, errors='ignore').read())):
        if n in WANT: file_count[WANT[n]] += 1
fn_row = lambda f, c: {'fn': f, 'probes': c, 'files': file_count[f], 'writes': INV.get(f.split('.')[-1], {}).get('n'), 'rolls_the_die': INV.get(f.split('.')[-1], {}).get('dice'), 'cannot_cross': crossing(f.split('.')[-1])}
J = {'kinds': dict(collections.Counter(r['kind'] for r in PROBE.values())), 'probes_mapped': len(PROBE), 'names_known': len(ALLN), 'unmapped': sorted(n for n in ALLN if n not in PROBE and n not in LIST or (n in LIST and LIST[n] == [n] and n not in PROBE)),
     'tools': {t: ser(r) for t, r in TOOL.items()}, 'families': fam_rows, 'probes': {n: ser(r) for n, r in sorted(PROBE.items())},
     'rules': [fn_row(f, c) for f, c in rule_probes.most_common()], 'unjudged': [fn_row(f, c) for f, c in unj_probes.most_common()], 'asks': [fn_row(f, c) for f, c in ask_probes.most_common()],
     'folded': FOLD, 'writes_by_name': write_probes.most_common(), 'wraps': wrap_probes.most_common(), 'hud_taps': sum(1 for r in PROBE.values() if r['hud']), 'hud_taps_by_kind': dict(collections.Counter(r['kind'] for r in PROBE.values() if r['hud']))}
json.dump(J, open(f'{OUT}/qa-census.json', 'w'), indent=1)
# --kinds <file>: the table TPS_GAME=table reads (tools/blitz/rust-host.ts `byTable`): a probe's kind; the rows of the stand-alone tools are kept as they are
if arg('--kinds'):
    old = json.load(open(arg('--kinds'))) if os.path.exists(arg('--kinds')) else {}
    # (`families`: a known name that only runs its members; `unread`: a known name the census found no line for (a probe of a family dispatched by its own
    # file, or a family's head): a check of that name is stood on the TypeScript tick and counted as having no row. tools/blitz/table-check.ts holds the three
    # lists to `probes.ts list`. A pin of a probe that is gone goes with it.)
    unread = set(J['unmapped']); pins = {n: v for n, v in old.get('pins', {}).items() if n in PROBE}
    json.dump({'what': 'a probe of tools/qa/probes.ts by what it does to the game between two ticks (tools/blitz/qa-census.py --kinds, by reading); `tools`: the stand-alone tools, by hand; `pins`: the checks that ran with a settled switch off its page value; `waits`: the checks seen red on the Rust game and green on the TypeScript tick, by hand, each with what it waits on; `families` and `unread`: the other names probes.ts knows', 'tools': old.get('tools', {}), 'pins': pins, 'waits': {n: v for n, v in old.get('waits', {}).items() if n in PROBE},
               'families': sorted(n for n in ALLN if n not in PROBE and n not in unread), 'unread': sorted(unread), 'probes': {n: r['kind'] for n, r in sorted(PROBE.items())}}, open(arg('--kinds'), 'w'), indent=0)
print(f"{len(PROBE)} probes read of {len(ALLN)} names known ({len(J['unmapped'])} names with no line found); kinds: {J['kinds']}")
print(f"rules called between ticks: {len(rule_probes)} distinct (+ {len(unj_probes)} the inventory never judged, + {len(ask_probes)} asks); {sum(1 for r in J['rules'] + J['unjudged'] if r['cannot_cross'])} take an argument that cannot cross")
for t, r in TOOL.items(): print(f"  {t}: kind {r['kind']} · rules {sorted(r['rules'])[:8]} · unjudged {sorted(r['unjudged'])[:6]} · writes {len(r['writes'])} · wraps {sorted(r['wraps'])[:4]} · {sorted(r['word'])}")
