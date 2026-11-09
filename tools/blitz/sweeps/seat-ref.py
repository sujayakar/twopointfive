#!/usr/bin/env python3
"""A seat as a value: a function that holds the game shared and depends on the seat takes a `SeatRef`.

    python3 tools/blitz/sweeps/seat-ref.py [--check]

A seat's own things are nineteen fields of the game (FIELDS), those of whichever seat stands on it. A function that reads
one through a shared game (`g: &Game`) answers for the standing seat without saying so, and cannot be asked about another.
This program gives every such function (but the seats' own machinery, KEEP) a second parameter `seat: &SeatRef`
(rust/game_types/src/seats/seat_ref.rs; `seat_ref` where the function already has a `seat` of its own) and turns its `g.<field>` into
`seat.<field>`; then it mends the callers BY THE COMPILER'S OWN LIST: `cargo check --message-format=json` names every call
that is an argument short and where it is. A caller that has a SeatRef passes it on; a shared caller that has none gains
the parameter itself (so the next round names ITS callers); a caller that holds the game exclusively, or has no game,
writes `&crate::seats::here(g)`. It runs the compiler again
until the list is empty. What it cannot mend (a reader used as a value, a lifetime) it prints and stops at, for a hand.

With `--check` nothing is written and the exit is 1 if a function that holds the game shared, and is not of KEEP, names
one of the nineteen through it.
"""
import glob, json, os, re, subprocess, sys

ROOT = os.path.normpath(os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', '..'))
RULES = os.path.join(ROOT, 'rust', 'game_rules', 'src')
FIELDS = ['seat', 'player', 'orders', 'door_tools', 'work', 'wall_charge', 'aim_point', 'aim_target', 'aim_guard', 'aim_friend', 'aim_snap',
          'hover', 'interactables', 'nearby', 'act', 'use_door', 'view_proj', 'threat_proximity', 'torch_on_t']
TYPE = '&crate::seats::SeatRef'
# the fields a SeatRef holds by reference: `&g.nearby` is `seat.nearby`
BORROWED = ['orders', 'door_tools', 'work', 'wall_charge', 'hover', 'interactables', 'nearby', 'act', 'view_proj']
# THE SEATS' OWN MACHINERY: the functions that may name the nineteen through a shared game, because they are how a seat is found or
# built, or answer about every seat at once (in a game of one seat, "the seat on the game" is their short way to say "the only seat")
KEEP = {
    'seats/p_01.rs': ['men', 'set_of_game', 'all_down', 'host_man', 'man_by_id', 'view_of_id', 'faced_man', 'faced_view', 'view_of_body', 'view_of', 'orders_of'],
    'cmd/p_01.rs': ['seat_man'],      # the man of ANY seat by its id: from the game where it stands, from where it is kept
}
# the crates whose callers the compiler is asked for, in the order they build
CRATES = ['game_rules', 'game', 'game_start']

def read(p): return open(p, encoding='utf-8').read()
def write(p, t): open(p, 'w', encoding='utf-8').write(t)

def skip(t, i):
    """`i` at the start of a string, a char, or a comment: the index just past it; else None"""
    c = t[i]
    if c == '/' and t.startswith('//', i): j = t.find('\n', i); return len(t) if j < 0 else j
    if c == '/' and t.startswith('/*', i):
        d, j = 1, i + 2
        while d and j < len(t):
            if t.startswith('/*', j): d += 1; j += 2
            elif t.startswith('*/', j): d -= 1; j += 2
            else: j += 1
        return j
    if c == '"' or (c == 'b' and t.startswith('b"', i)):
        j = i + (2 if c == 'b' else 1)
        while t[j] != '"': j += 2 if t[j] == '\\' else 1
        return j + 1
    if c == 'r' and re.match(r'r#*"', t[i:]) and (i == 0 or not (t[i - 1].isalnum() or t[i - 1] == '_')):
        h = re.match(r'r(#*)"', t[i:]).group(1); j = t.find('"' + h, i + 2 + len(h)); return j + 1 + len(h)
    if c == "'":
        m = re.match(r"'(\\.[^']*|[^'\\])'", t[i:])       # a char; a lifetime has no closing quote there
        return i + m.end() if m else None
    return None

def close(t, i, o, c):
    """`i` at an opening `o`: the index of its closing `c`, strings and comments passed over"""
    d = 0
    while i < len(t):
        j = skip(t, i)
        if j is not None: i = j; continue
        if t[i] == o: d += 1
        elif t[i] == c:
            d -= 1
            if d == 0: return i
        i += 1
    raise ValueError('no closing ' + c)

FN = re.compile(r'\bfn\s+(\w+)\s*(<[^>(]*>)?\s*\(')
SHARED = re.compile(r'\s*(?:mut\s+)?(\w+)\s*:\s*&\s*(?:\'\w+\s+)?(?:crate::game::)?Game\b')

def functions(t):
    """every function with a body: (name, index of its `(`, index of its `)`, body start, body end, the shared game's name or None)"""
    out, i = [], 0
    code = mask(t)
    for m in FN.finditer(code):
        po = m.end() - 1; pc = close(t, po, '(', ')')
        k = pc + 1
        while k < len(t) and code[k] not in '{;': k += 1
        if k >= len(t) or code[k] == ';': continue
        be = close(t, k, '{', '}')
        g = SHARED.match(t, po + 1)
        out.append((m.group(1), po, pc, k, be, g.group(1) if g else None))
    return out

def mask(t):
    """the text with every string, char and comment blanked, so a pattern reads code only"""
    out, i, last = [], 0, 0
    while i < len(t):
        j = skip(t, i) if t[i] in '/"\'br' else None
        if j is None: i += 1; continue
        out.append(t[last:i]); out.append(' ' * (j - i)); i = last = j
    out.append(t[last:])
    return ''.join(out)

def name_for(code, po, be):
    """the parameter's name in this function: `seat`, or `seat_ref` where the function has a `seat` already"""
    return 'seat_ref' if re.search(r'(?<![.\w])seat\b(?!\s*\()', code[po:be]) else 'seat'

def held(t, po, pc):
    """the name this function has a SeatRef under, or None"""
    m = re.search(r'(\w+)\s*:\s*' + re.escape(TYPE) + r'\b', t[po:pc])
    return m.group(1) if m else None

def gain(code, po, pc, name):
    """the parameter, under `name`, written after the first (the game): the edit"""
    first, d = po + 1, 0
    while first < pc:
        c = code[first]
        if c in '([{<': d += 1
        elif c in ')]}>' and code[first - 1] != '-': d -= 1
        elif c == ',' and d == 0: break
        first += 1
    return (first, first, ', %s: %s' % (name, TYPE))

def named(body, g):
    return re.compile(r'(?<![\w.])' + re.escape(g) + r'\.(' + '|'.join(FIELDS) + r')\b(?!\s*\()')

def files():
    for p in sorted(glob.glob(os.path.join(RULES, '**', '*.rs'), recursive=True)):
        rel = os.path.relpath(p, RULES)
        if rel.endswith('types.rs'): continue
        yield p, rel

def readers(check):
    """part one: the shared functions that name a seat field"""
    n = uses = 0; left = []
    for p, rel in files():
        t = read(p); code = mask(t); edits = []
        for name, po, pc, bs, be, g in functions(t):
            if not g or name in KEEP.get(rel.replace(os.sep, '/'), []): continue
            pat = named(code[bs:be], g)
            hits = [m for m in pat.finditer(code, bs, be)]
            if not hits: continue
            # (a function nested in this one is its own row of the list: its names are not this one's)
            n += 1; uses += len(hits); left.append('%s: %s' % (rel, name))
            if check: continue
            sr = name_for(code, po, be); edits.append(gain(code, po, pc, sr))
            for m in hits:
                amp = re.search(r'&\s*$', code[bs:m.start()])
                whole = m.group(1) in BORROWED and amp and not re.match(r'\s*[.\[]', code[m.end():m.end() + 4])
                a = bs + amp.start() if whole else m.start()
                edits.append((a, m.end(), sr + '.' + ('id' if m.group(1) == 'seat' else m.group(1))))
        if edits and not check:
            for a, b, s in sorted(edits, reverse=True): t = t[:a] + s + t[b:]
            write(p, t)
    return n, uses, left

def compiler(crate):
    r = subprocess.run(['cargo', 'check', '--tests', '--message-format=json'], cwd=os.path.join(ROOT, 'rust', crate), capture_output=True, text=True)
    out = []
    for line in r.stdout.split('\n'):
        if not line.startswith('{'): continue
        m = json.loads(line)
        if m.get('reason') == 'compiler-message' and m['message']['level'] == 'error': out.append(m['message'])
    return out

def callers():
    """part two: the compiler's list, until it is empty"""
    rounds = 0
    for crate in CRATES:
        while True:
            errs = compiler(crate); rounds += 1
            if not errs: break
            short = [e for e in errs if (e.get('code') or {}).get('code') == 'E0061']
            if not short:
                print('seat-ref: %s has %d errors that are not a call an argument short; the first three:' % (crate, len(errs)))
                for e in errs[:3]: print('   ' + e['rendered'].split('\n')[0] + '   ' + (e['spans'][0]['file_name'] + ':' + str(e['spans'][0]['line_start']) if e['spans'] else ''))
                sys.exit(1)
            # WHERE each call is, from the compiler (its offer to write the call out covers the call's parentheses). WHAT is written is this
            # program's: the argument goes straight after the game, where the parameter went. The compiler's own text is not used: where two
            # arguments could be either of two parameters (a `None`) it guesses, and its guess drops one.
            byfile = {}
            for e in short:
                at = [s for c in e['children'] if c['message'].startswith('provide the argument') for s in c['spans'] if s.get('suggested_replacement')]
                if at: byfile.setdefault(os.path.normpath(os.path.join(ROOT, 'rust', crate, at[0]['file_name'])), set()).add(at[0]['byte_start'])
            if not byfile:
                print('seat-ref: %s: %d calls an argument short and none the compiler places' % (crate, len(short))); sys.exit(1)
            n = 0
            for p, starts in byfile.items():
                raw = open(p, 'rb').read(); t = raw.decode('utf-8'); fs = functions(t); code = mask(t)
                gains, edits = {}, []
                for a in sorted(starts):
                    po = len(raw[:a].decode('utf-8'))            # (the compiler counts bytes)
                    if t[po] != '(': sys.exit('seat-ref: %s: the compiler placed a call where there is no `(`' % p)
                    pc = close(t, po, '(', ')')
                    # the end of the first argument: the first comma at this depth, or the closing parenthesis
                    k, d = po + 1, 0
                    while k < pc:
                        c = code[k]
                        if c in '([{': d += 1
                        elif c in ')]}': d -= 1
                        elif c == ',' and d == 0: break
                        k += 1
                    first = t[po + 1:k].strip()
                    inside = [f for f in fs if f[3] <= po <= f[4]]
                    own = min(inside, key=lambda f: f[4] - f[3]) if inside else None
                    has = own and held(t, own[1], own[2])
                    if has: arg = has
                    elif own and own[5] and own[0] not in KEEP.get(os.path.relpath(p, RULES).replace(os.sep, '/'), []): arg = gains.setdefault((own[0], own[1]), name_for(code, own[1], own[4]))
                    else:
                        # the game as the call itself has it, borrowed shared: `g`, or `&g` where the call wrote `&mut g`
                        game = '&' + first[len('&mut'):].strip() if first.startswith('&mut') else first
                        arg = '&%s::seats::here(%s)' % ('crate' if crate in ('game_rules', 'game') else 'game', game)
                        if first.startswith('&mut'): edits.append((po + 1, k, game))       # (a reader holds the game shared: it is handed so)
                    edits.append((k, k, ', ' + arg)); n += 1
                for a, b, s in sorted(edits, reverse=True): t = t[:a] + s + t[b:]
                # a shared caller with no `sr` gains the parameter itself: the next round names ITS callers
                for (name, _), sr in gains.items():
                    code = mask(t)
                    for f in functions(t):
                        if f[0] == name and f[5] and not held(t, f[1], f[2]) and re.search(r'(?<![.\w])%s\s*[,)]' % sr, code[f[3]:f[4]]):
                            at, _, text = gain(code, f[1], f[2], sr); t = t[:at] + text + t[at:]; break
                write(p, t)
            print('seat-ref: %s round %d: %d calls mended in %d files' % (crate, rounds, n, len(byfile)))
    return rounds

def main():
    check = '--check' in sys.argv
    n, uses, left = readers(check)
    if check:
        for x in left[:20]: print('   ' + x)
        print('seat-ref: %d shared functions that are not the seats\' own machinery name a seat field through the game (%d names)' % (n, uses))
        sys.exit(1 if n else 0)
    print('seat-ref: %d shared functions took the parameter, %d names turned' % (n, uses))
    callers()

main()
