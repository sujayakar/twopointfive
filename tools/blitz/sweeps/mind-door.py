#!/usr/bin/env python3
"""A man's awareness and state written through one door, with the writer's name.

    python3 tools/blitz/sweeps/mind-door.py [--check]

In the body files of rust/game/src (`p_NN.rs`, `b_NN.rs`; not a file's tests) it does two things, in this order.

1. A second name for a man is taken away where his mind is written through it:

       let man = &mut g.gd[gd];              (the line goes)
       man.awareness = 0.0; man.path = ...   g.gd[gd].awareness = 0.0; g.gd[gd].path = ...

   only where the name is bound on a line of its own to `&mut g.gd[<a name>]`, and every use of it up to the end of its
   block (or to where the same name is bound to a man again) is a field of it or hands it whole to a function
   (`no_path(d)` becomes `no_path(&mut g.gd[gd])`). Anything else is listed and left for a hand.

2. Every `g.gd[<man>].awareness = <value>;` and `g.gd[<man>].state = <value>;` becomes a call of rust/game/src/mind.rs:

       g.gd[x].awareness = m::max(g.gd[x].awareness, V);   crate::mind::raise(g, x, V, "<module>");
       g.gd[x].awareness = m::min(g.gd[x].awareness, V);   crate::mind::cap(g, x, V, "<module>");
       g.gd[x].awareness = <anything else>;                crate::mind::set(g, x, <it>, "<module>");
       g.gd[x].state = V;                                  crate::mind::set_state(g, x, V, "<module>");

   <module> is the folder the file is in. A man or a value that reads `g` is taken into a local first, because the call
   borrows the game. Each call writes the same value at the same place, so play is unchanged.

Run again on a later tree it rewrites only what is still written the old way. `--check` writes nothing, lists what a hand
has to look at, and exits 1 if anything is still written the old way.
"""
import glob, os, re, sys

ROOT = os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', '..', 'rust', 'game', 'src')
MAN = r'((?:[^\[\]]|\[[^\[\]]*\])+)'
WRITE = re.compile(r'g\.gd\[' + MAN + r'\]\.(awareness|state) (\+|-)?= ([^;]+);')
LET = re.compile(r'^(\s*)let (?:mut )?(\w+) = &mut g\.gd\[(\w+)\];\s*$')
READS_G = re.compile(r'\bg\b')
# a value that calls something other than plain arithmetic is shown by --check: the call is made before the man's
# awareness is read, where the old line read it first, which is the same only if the call does not write his mind
PLAIN_CALL = re.compile(r'\b(?:crate::)?(?:m|v3)::\w+\(|\bif\b|\bSome\(')

def bare(line):
    """a line without its strings, chars and trailing comment, for counting braces"""
    line = re.sub(r'"(?:[^"\\]|\\.)*"', '""', line)
    line = re.sub(r"'(?:[^'\\]|\\.)'", "' '", line)
    return line.split('//')[0]

def unname(lines, notes, where):
    """step 1 on one file's lines; returns how many names were taken away"""
    n, i = 0, 0
    while i < len(lines):
        m = LET.match(lines[i])
        if not m: i += 1; continue
        name, ix = m.group(2), m.group(3)
        depth, j = 0, i + 1
        again = False
        while j < len(lines):
            b = bare(lines[j])
            # the same name bound again to a man ends this one's reach on the line before
            r = LET.match(lines[j])
            if r and depth == 0 and r.group(2) == name: again = True; j -= 1; break
            # the block ends on the line where its closing brace is; what stands before that brace is still in it
            closed = False
            for ch in b:
                if ch == '{': depth += 1
                elif ch == '}':
                    depth -= 1
                    if depth < 0: closed = True; break
            if closed: break
            j += 1
        scope = lines[i + 1:j + 1]
        use = re.compile(r'(?<![.\w])' + re.escape(name) + r'\b')
        field = re.compile(r'(?<![.\w])' + re.escape(name) + r'\.(?!\.)')
        # the name handed whole to a function (`no_path(d)`) is the man himself
        handed = re.compile(r'(?<=[(,])(\s*)' + re.escape(name) + r'(?=\s*[,)])')
        text = '\n'.join(bare(l) for l in scope)
        if not re.search(r'(?<![.\w])' + re.escape(name) + r'\.(awareness|state) (?:\+|-)?= ', text): i += 1; continue
        why = None
        if len(use.findall(text)) != len(field.findall(text)) + len(handed.findall(text)): why = 'the name is used whole'
        elif re.search(r'\blet (?:mut )?(?:%s|%s)\b' % (re.escape(name), re.escape(ix)), text): why = 'a name is bound again'
        elif re.search(r'(?<![.\w])%s = ' % re.escape(ix), text): why = 'the man is changed'
        if why:
            notes.append('%s:%d: a second name left (%s): %s' % (where, i + 1, why, lines[i].strip()))
            i += 1; continue
        for k in range(i + 1, j + 1): lines[k] = handed.sub(r'\1&mut g.gd[%s]' % ix, field.sub('g.gd[%s].' % ix, lines[k]))
        del lines[i]
        n += 1
    return n

def calls(module, notes, where):
    def one(m):
        man, what, op, value = m.group(1), m.group(2), m.group(3), m.group(4).strip()
        own = 'g.gd[%s].awareness' % man
        if what == 'state':
            if op: return m.group(0)
            verb, arg = 'set_state', value
        elif op:
            verb, arg = 'set', '%s %s (%s)' % (own, op, value)
        else:
            verb, arg = 'set', value
            for fn, v in (('max', 'raise'), ('min', 'cap')):
                for pref in ('m::', 'crate::m::'):
                    head = '%s%s(%s, ' % (pref, fn, own)
                    if value.startswith(head) and closes_last(value[len(pref) + len(fn):]): verb, arg = v, value[len(head):-1]
        if verb in ('raise', 'cap') and re.search(r'\w\(', PLAIN_CALL.sub('', arg)):
            notes.append('%s: a value that calls (read it): %s' % (where, m.group(0)[:140]))
        pre, man_arg = '', man
        if READS_G.search(man): pre += 'let who = %s; ' % man; man_arg = 'who'
        if READS_G.search(arg): pre += 'let to = %s; ' % arg; arg = 'to'
        body = 'crate::mind::%s(g, %s, %s, "%s");' % (verb, man_arg, arg, module)
        return '{ %s%s }' % (pre, body) if pre else body
    return one

def closes_last(s):
    """s starts at an opening parenthesis: true if that parenthesis closes at the very end of s"""
    depth = 0
    for k, ch in enumerate(s):
        if ch == '(': depth += 1
        elif ch == ')':
            depth -= 1
            if depth == 0: return k == len(s) - 1
    return False

LEFT = re.compile(r'\.awareness (?:\+|-|\*)?= |\.state = (?:crate::game::GuardState|GuardState|St)::')

def main():
    check = '--check' in sys.argv
    names, writes, files, notes = 0, 0, 0, []
    for path in sorted(glob.glob(os.path.join(ROOT, '*', '[pb]_*.rs'))):
        module = os.path.basename(os.path.dirname(path))
        where = '%s/%s' % (module, os.path.basename(path))
        whole = open(path).read()
        cut = re.search(r'^\s*#\[cfg\(test\)\]', whole, re.M)
        text, tests = (whole[:cut.start()], whole[cut.start():]) if cut else (whole, '')
        lines = text.split('\n')
        n1 = unname(lines, notes, where)
        new, n2 = WRITE.subn(calls(module, notes, where), '\n'.join(lines))
        # a write that was alone in a block of its own needs no second pair of braces
        new = re.sub(r'\{ (\{ (?:let \w+ = [^{};]*; )+crate::mind::\w+\([^{};]*\); \}) \}', r'\1', new)
        n2 = sum(1 for _ in re.finditer(r'crate::mind::', new)) - sum(1 for _ in re.finditer(r'crate::mind::', text))
        for i, line in enumerate(new.split('\n'), 1):
            if LEFT.search(bare(line)): notes.append('%s:%d: still written in place (by hand): %s' % (where, i, line.strip()[:140]))
        names += n1; writes += n2
        if (n1 or n2) and not check: open(path, 'w').write(new + tests); files += 1
    if check:
        print('mind-door: %d written the old way, %d second names to take away' % (writes, names))
        for n in notes: print('  ' + n)
        sys.exit(1 if writes or names else 0)
    print('mind-door: %d writes are calls, %d second names taken away, in %d files' % (writes, names, files))
    for n in notes: print('  ' + n)

main()
