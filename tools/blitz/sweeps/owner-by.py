#!/usr/bin/env python3
"""Whose man he is, asked by name.

    python3 tools/blitz/sweeps/owner-by.py [--check]

Who has a man is one word of bits, a bit a tasker. The rules spelled a bit as a number off a table made anew at each ask
(`crate::owner::own().runner`, an f64), and or-ed two through js::to_i32. This program names each by its constant
(`crate::owner::by::RUNNER`, a `By`, or-ed with `|`) and calls `owner::has`, `claim` and `release`, which take a `By`
(and, of `has`, the game shared).

A `let own = crate::owner::own();` goes, and the `own.<field>` of the function it stands in become constants. Run again on
a later tree it turns what is still spelled the old way; `--check` writes nothing and exits 1 if anything is.
"""
import os, re, sys
from bodies import bodies

ROOT = os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', '..', 'rust', 'game_rules', 'src')
FIELDS = ['frag', 'runner', 'morale', 'startle', 'stand', 'squad_leg', 'squad_word', 'last_man', 'lane', 'press', 'walk', 'breaker', 'buddy', 'needs', 'squad']
BY = 'crate::owner::by::'
LET = re.compile(r'^( *)let (\w+) = crate::owner::own\(\);\n', re.M)

def turn(t):
    n = 0
    def field(m):
        nonlocal n; n += 1
        return BY + m.group(1).upper()
    # a table kept in a local: its function's reads of it, to the end of the block the `let` stands in
    while True:
        m = LET.search(t)
        if not m: break
        indent, name = m.group(1), m.group(2)
        close = re.compile(r'^%s\}' % indent[4:], re.M) if len(indent) >= 4 else None
        end = close.search(t, m.end()).start() if close else len(t)
        body = re.sub(r'(?<![\w.])%s\.(%s)\b(?!\()' % (name, '|'.join(FIELDS)), field, t[m.end():end])
        if re.search(r'(?<![\w.])%s\b(?!\s*\()' % name, re.sub(r'//.*', '', body)): sys.exit('owner-by: `%s` is read another way: %s' % (name, t[m.start():m.start() + 120]))
        t = t[:m.start()] + body + t[end:]
    t = re.sub(r'crate::owner::own\(\)\.(%s)\b' % '|'.join(FIELDS), field, t)
    # two or-ed through the language's int32
    t = re.sub(r'\(((?:js::to_i32\(%s\w+\)(?: \| )?)+)\) as f64' % re.escape(BY), lambda m: re.sub(r'js::to_i32\((%s\w+)\)' % re.escape(BY), r'\1', m.group(1)), t)
    for a, b in (('has_him', 'has'), ('claim_man', 'claim'), ('release_man', 'release')):
        t, k = re.subn(r'crate::owner::%s\(' % a, 'crate::owner::%s(' % b, t); n += k
    return t, n

def main():
    check = '--check' in sys.argv
    n = files = 0
    for path, module, _ in bodies(ROOT):
        if module == 'owner': continue
        t = open(path, encoding='utf-8').read()
        u, k = turn(t)
        if not k: continue
        n += k; files += 1
        if not check: open(path, 'w', encoding='utf-8').write(u)
    print('owner-by: %d asks in %d files %s' % (n, files, 'still spelled the old way' if check else 'turned'))
    if check: sys.exit(1 if n else 0)

main()
