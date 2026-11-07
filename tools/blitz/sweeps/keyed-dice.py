#!/usr/bin/env python3
"""Every draw of the game's one die becomes a keyed draw.

    python3 tools/blitz/sweeps/keyed-dice.py [--check]

Rewrites each `crate::math_dice::sim_random(g)` in the body files of rust/game/src (`p_NN.rs`, `b_NN.rs`; not the die's own
module, not a file's tests) into a keyed draw (rust/game/src/dice.rs `Keyed` and `draw`):

    crate::dice::draw_of(g, "<module>::<function>#<n>", gd)      where the function is handed a man (a GuardId)
    crate::dice::draw_def / draw_op / draw_body(g, "…", x)       … a defender's or an operative's row, or a body (a CharId)
    crate::dice::draw_world(g, "<module>::<function>#<n>")       where it is handed nobody

The site's name is the module, the function, and which draw of that function it is, so a name stays the same when other
functions change. THIS CHANGES PLAY: every draw has another value than the one sequence gave it. Run again on a later tree it
rewrites only what is still drawn the old way; a draw written by hand afterwards names its own site.

`--check` writes nothing, lists the sites and who each is about, and exits 1 if any is still drawn the old way.
"""
import glob, os, re, sys

ROOT = os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', '..', 'rust', 'game', 'src')
OLD = 'crate::math_dice::sim_random(g)'
FN = re.compile(r'^\s*(?:pub(?:\([a-z]+\))? )?fn (\w+)(?:<[^>]*>)?\((.*)$')
SKIP = {'math_dice'}

def who_of(sig):
    """the man a function is handed: (the draw's form, the argument), or None"""
    for form, ty in (('draw_of', 'GuardId'), ('draw_def', 'RoomfightDefId'), ('draw_op', 'RoomfightOpId'), ('draw_body', 'CharId')):
        m = re.search(r'\b(\w+): ' + ty + r'\b', sig)
        if m: return form, m.group(1)
    return None

def main():
    check = '--check' in sys.argv
    left, done, sites = 0, 0, []
    for path in sorted(glob.glob(os.path.join(ROOT, '*', '[pb]_*.rs'))):
        module = os.path.basename(os.path.dirname(path))
        if module in SKIP: continue
        lines = open(path).read().split('\n')
        fn, sig, count, in_tests, changed = '?', '', {}, False, False
        for i, line in enumerate(lines):
            if re.match(r'\s*#\[cfg\(test\)\]', line): in_tests = True
            m = FN.match(line)
            if m: fn, sig = m.group(1), m.group(2)
            if in_tests or OLD not in line or re.match(r'\s*//', line): continue
            while OLD in lines[i]:
                count[fn] = count.get(fn, 0) + 1
                site = '%s::%s#%d' % (module, fn, count[fn])
                who = who_of(sig)
                new = 'crate::dice::%s(g, "%s", %s)' % (who[0], site, who[1]) if who else 'crate::dice::draw_world(g, "%s")' % site
                sites.append('%-44s %s' % (site, who[1] + ' (' + who[0] + ')' if who else 'nobody'))
                lines[i] = lines[i].replace(OLD, new, 1)
                changed = True
                left += 1
        if changed and not check: open(path, 'w').write('\n'.join(lines)); done += 1
    if check:
        print('keyed-dice: %d drawn the old way' % left)
        for s in sites: print('  ' + s)
        sys.exit(1 if left else 0)
    print('keyed-dice: %d draws rewritten in %d files' % (left, done))

main()
