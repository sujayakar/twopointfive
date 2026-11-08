#!/usr/bin/env python3
"""A man's post flags written through one door, with the writer's name.

    python3 tools/blitz/sweeps/post-flags.py [--check]

Rewrites every `g.gd[<man>].hold = <value>;` and `g.gd[<man>].pinned = <value>;` in the body files of rust/game_rules/src
(`p_NN.rs`, `b_NN.rs`; not a file's tests) into `crate::control::set_hold(g, <man>, <value>, "<module>");` (`set_pinned`), where
<module> is the folder the file is in. The call writes the same flag at the same place and notes who wrote it
(control::PostNote), so play is unchanged. A value or a man that reads `g` is taken into a local first, because
the call borrows the game. Run again on a later tree it rewrites only what is still written the old way.

It does not touch a flag written through another name (`let man = &mut g.gd[gd]; man.hold = false;`): `--check`
lists every such line, and those are rewritten by hand. With `--check` nothing is written and the exit is 1 if
anything is still written the old way.
"""
import os, re, sys
from bodies import bodies

ROOT = os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', '..', 'rust', 'game_rules', 'src')
MAN = r'((?:[^\[\]]|\[[^\[\]]*\])+)'
DIRECT = re.compile(r'g\.gd\[' + MAN + r'\]\.(hold|pinned) = ([^;]+);')
# a flag of a Guard written through another name: the names these files use for a `&mut Guard`
ALIAS = re.compile(r'\b(man|m|o|x|d|gdm)\.(hold|pinned) = (true|false)\b')
SKIP = {'control'}

def rewrite(module, text):
    def one(m):
        man, flag, value = m.group(1), m.group(2), m.group(3).strip()
        call = 'crate::control::set_%s' % flag
        pre, man_arg, value_arg = '', man, value
        if re.search(r'\bg\b', man): pre += 'let who = %s; ' % man; man_arg = 'who'
        if re.search(r'\bg\b', value): pre += 'let on = %s; ' % value; value_arg = 'on'
        body = '%s(g, %s, %s, "%s");' % (call, man_arg, value_arg, module)
        return '{ %s%s }' % (pre, body) if pre else body
    return DIRECT.subn(one, text)

def main():
    check = '--check' in sys.argv
    left, done, aliases = 0, 0, []
    for path, module, _ in bodies(ROOT):
        if module in SKIP: continue
        whole = open(path).read()
        # (a file's tests set a man up by plain writes, and hold the game in a Box: they are left as they are)
        cut = re.search(r'^\s*#\[cfg\(test\)\]', whole, re.M)
        text, tests = (whole[:cut.start()], whole[cut.start():]) if cut else (whole, '')
        new, n = rewrite(module, text)
        new += tests
        for i, line in enumerate(text.split('\n'), 1):
            if ALIAS.search(line) and 'Guard' not in line: aliases.append('%s/%s:%d: %s' % (module, os.path.basename(path), i, line.strip()[:150]))
        if n and not check: open(path, 'w').write(new)
        (left, done) = (left + n, done) if check else (left, done + n)
    if check:
        print('post-flags: %d written the old way' % left)
        for a in aliases: print('  through another name (by hand; check each is a Guard): ' + a)
        sys.exit(1 if left else 0)
    print('post-flags: %d rewritten' % done)

main()
