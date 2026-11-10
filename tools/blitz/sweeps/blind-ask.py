#!/usr/bin/env python3
""""Is he blind" and "is he reeling" asked in one place.

    python3 tools/blitz/sweeps/blind-ask.py [--check]

A man cannot see until `dazzled_until`, and reels until `stagger_until`. The question was written out wherever it was
asked, in four shapes (`g.time < g.gd[m].dazzled_until`, `g.time >= …`, and each under a `!( )`). This program turns each
into `crate::control::man::blind(g, m)` or `crate::control::man::staggered(g, m)` (control/man.rs), with the `!` the shape
calls for.

Only a comparison of the game's own clock (`g.time`) with the field of a man named by his id (`g.gd[<id>]`) is turned.
A function that has taken the clock into a local of its own (`now`, `t`), or the man into a borrow (`man.dazzled_until`),
is left and listed: the program cannot see that the local is the clock, or lend the game while the man is borrowed. A
write of either field, and a read of how LONG is left (`dazzled_until - g.time`), are not the question and stay.

The two clocks are never NaN (they are set from the game's clock and a number of seconds), so `>=` is `!( < )`.

With `--check` nothing is written; the exit is 1 if a line of a shape this program turns is still written out.
"""
import glob, os, re, sys

ROOT = os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', '..', 'rust', 'game_rules', 'src')
ASK = {'dazzled_until': 'blind', 'stagger_until': 'staggered'}
MAN = r'g\.gd\[((?:[^\[\]]|\[[^\[\]]*\])+)\]'                 # g.gd[<id>], the id with at most one pair of brackets inside
LT = re.compile(r'(!\(\s*)?g\.time\s*(<|>=)\s*' + MAN + r'\.(dazzled_until|stagger_until)\b(\s*\))?')
LEFT = re.compile(r'\b(now|t|time|clock)\s*(<|>=)\s*[\w.\[\]()]*\.(dazzled_until|stagger_until)\b|\bg\.time\s*(<|>=)\s*(?!g\.gd\[)[\w.\[\]()]*\.(dazzled_until|stagger_until)\b')

def turn(m):
    neg_open, op, man, field, neg_close = m.group(1), m.group(2), m.group(3), m.group(4), m.group(5)
    wrapped = bool(neg_open and neg_close)
    yes = (op == '<') != wrapped                              # `<` asks "is he"; a `!( )` round it, or `>=`, asks "is he not"
    call = 'crate::control::man::%s(g, %s)' % (ASK[field], man.strip())
    text = call if yes else '!' + call
    # (half a `!( … )`: the comparison is one term of a longer one inside the brackets, which stay)
    if neg_open and not neg_close: return neg_open + text
    if neg_close and not neg_open: return text + neg_close
    return text

def main():
    check = '--check' in sys.argv
    n = files = 0; left = []
    for path in sorted(glob.glob(os.path.join(ROOT, '**', '*.rs'), recursive=True)):
        rel = os.path.relpath(path, ROOT)
        if rel.endswith('types.rs') or rel == os.path.join('people', 'control', 'man.rs'): continue
        t = open(path, encoding='utf-8').read()
        u, k = LT.subn(turn, t)
        for i, line in enumerate(u.split('\n')):
            if LEFT.search(line): left.append('%s:%d' % (rel, i + 1))
        if not k: continue
        n += k; files += 1
        if not check: open(path, 'w', encoding='utf-8').write(u)
    print('blind-ask: %d questions in %d files %s; %d lines left that ask through a local clock or a borrowed man' % (n, files, 'still written out' if check else 'turned', len(left)))
    if left and '--left' in sys.argv: print('   ' + '\n   '.join(left))
    if check: sys.exit(1 if n else 0)

main()
