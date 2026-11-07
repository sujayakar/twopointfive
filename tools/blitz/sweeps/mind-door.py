#!/usr/bin/env python3
"""A man's awareness, state and fix written through one door, with the writer's name.

    python3 tools/blitz/sweeps/mind-door.py [--check]

In the body files of rust/game/src (`p_NN.rs`, `b_NN.rs`; not a file's tests) it does four things, in this order.

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

3. Every write of his fix (where he thinks his man is) becomes a call too. A write that also counted itself as a new
   fix is `fix`; one that did not is `fix_keep`:

       g.gd[x].last_known = V; g.gd[x].fix_stamp = g.gd[x].fix_stamp.wrapping_add(1);   crate::mind::fix(g, x, V, "<module>");
       g.gd[x].last_known = V;                                                           crate::mind::fix_keep(g, x, V, "<module>");

4. A private function left with nothing in it but one such call (`fn set_fix(g, gd, at) { crate::mind::fix(g, gd,
   Some(at), ..); }`: there were seven, under four names) is taken away, and each call of it becomes the call it made.

<module> is the folder the file is in. A man or a value that reads `g` is taken into a local first, because the call
borrows the game. Each call writes the same value at the same place, so play is unchanged.

Run again on a later tree it rewrites only what is still written the old way. `--check` writes nothing, lists what a hand
has to look at, and exits 1 if anything is still written the old way.
"""
import glob, os, re, sys

ROOT = os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', '..', 'rust', 'game', 'src')
MAN = r'((?:[^\[\]]|\[[^\[\]]*\])+)'
WRITE = re.compile(r'g\.gd\[' + MAN + r'\]\.(awareness|state) (\+|-)?= ([^;]+);')
FIX_NEW = re.compile(r'g\.gd\[' + MAN + r'\]\.last_known = ([^;]+);\s*g\.gd\[\1\]\.fix_stamp (?:= g\.gd\[\1\]\.fix_stamp\.wrapping_add\(1\)|\+= 1);')
FIX_KEPT = re.compile(r'g\.gd\[' + MAN + r'\]\.last_known = ([^;]+);')
# a private function that only forwards to mind::fix, with its doc lines
FORWARD = re.compile(r'(?:^[ \t]*///[^\n]*\n)*^fn (\w+)\(g: &mut (?:crate::game::)?Game, (\w+): GuardId, (\w+): V3\) \{\n\s*crate::mind::fix\(g, (\w+), ([^;\n]+), "(\w+)"\);\n\}\n', re.M)
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
        if not re.search(r'(?<![.\w])' + re.escape(name) + r'\.(awareness|state|last_known|fix_stamp) (?:\+|-)?= ', text): i += 1; continue
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

def fix_call(module, verb):
    def one(m):
        man, value = m.group(1), m.group(2).strip()
        pre, man_arg = '', man
        if READS_G.search(man): pre += 'let who = %s; ' % man; man_arg = 'who'
        if READS_G.search(value): pre += 'let at = %s; ' % value; value = 'at'
        body = 'crate::mind::%s(g, %s, %s, "%s");' % (verb, man_arg, value, module)
        return '{ %s%s }' % (pre, body) if pre else body
    return one

def args_of(s, start):
    """the arguments of a call whose opening parenthesis is at s[start]: (the arguments split at top-level commas, the index after the closing parenthesis)"""
    depth, args, last = 0, [], start + 1
    for k in range(start, len(s)):
        ch = s[k]
        if ch in '([{': depth += 1
        elif ch in ')]}':
            depth -= 1
            if depth == 0: args.append(s[last:k].strip()); return args, k + 1
        elif ch == ',' and depth == 1: args.append(s[last:k].strip()); last = k + 1
    return None, start

def unforward(text, notes, where):
    """step 4 on one file's text; returns (the text, how many functions were taken away)"""
    n = 0
    while True:
        f = next((f for f in FORWARD.finditer(text) if f.group(4) == f.group(2) and len(re.findall(r'\b%s\b' % re.escape(f.group(3)), f.group(5))) == 1), None)
        if not f: return text, n
        name, at, value, module = f.group(1), f.group(3), f.group(5), f.group(6)
        text = text[:f.start()] + text[f.end():]
        call = re.compile(r'(?<![\w:.])%s\(' % re.escape(name))
        pos = 0
        while True:
            c = call.search(text, pos)
            if not c: break
            args, end = args_of(text, c.end() - 1)
            if not args or len(args) != 3 or args[0] != 'g':
                notes.append('%s: a call of %s the program cannot read (by hand): %s' % (where, name, text[c.start():c.start() + 80].split('\n')[0])); pos = c.end(); continue
            new = 'crate::mind::fix(g, %s, %s, "%s")' % (args[1], re.sub(r'\b%s\b' % re.escape(at), lambda _: args[2], value), module)
            text = text[:c.start()] + new + text[end:]
            pos = c.start() + len(new)
        n += 1

def closes_last(s):
    """s starts at an opening parenthesis: true if that parenthesis closes at the very end of s"""
    depth = 0
    for k, ch in enumerate(s):
        if ch == '(': depth += 1
        elif ch == ')':
            depth -= 1
            if depth == 0: return k == len(s) - 1
    return False

LEFT = re.compile(r'\.awareness (?:\+|-|\*)?= |\.state = (?:crate::game::GuardState|GuardState|St)::|\.last_known = |\.fix_stamp (?:\+)?= ')

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
        new = FIX_NEW.sub(fix_call(module, 'fix'), new)
        new = FIX_KEPT.sub(fix_call(module, 'fix_keep'), new)
        new, gone = unforward(new, notes, where)
        n2 = sum(1 for _ in re.finditer(r'crate::mind::', new)) - sum(1 for _ in re.finditer(r'crate::mind::', text)) + gone
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
