#!/usr/bin/env python3
"""A helper that draws from the die is handed the man it draws for.

    python3 tools/blitz/sweeps/dice-who.py [--check]

Some draws are made by a small helper (`pick(g, range)`: a number in a range) that many rules call. The helper does not
know whom it draws for, so under keyed draws (rust/game_rules/src/dice.rs) all its callers would share one site and nobody,
and the order two men reached it in one tick would still be play. This program gives each helper below a second
parameter, `who: crate::dice::Who`, and writes at each call the man its caller is about:

    pick(g, s.react)        in a function handed `d: RoomfightDefId`     pick(g, crate::dice::Who::Def(d), s.react)
                            … `o: RoomfightOpId` / `gd: GuardId` / `c: CharId`    Who::Op(o) / Who::Man(gd) / Who::Body(c)
                            in a function that is itself handed `who`     pick(g, who, …)

and where the caller is handed nobody, the man named for that call in BY_HAND below (read from the code by a person).
A call it cannot place is listed and left; the tree does not compile until it has a row.

NO CHANGE OF PLAY by itself: the helpers do not use `who` until tools/blitz/sweeps/keyed-dice.py rewrites their draw
(it keys a draw in a function handed `who` by him). Run again on a later tree it changes only what is not yet done.
"""
import os, re, sys
from bodies import bodies

ROOT = os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', '..', 'rust', 'game_rules', 'src')
WHO = 'crate::dice::Who'
# (module, helper): every helper that draws and does not know for whom
HELPERS = [('buddy', 'pick'), ('digin', 'pick'), ('digin', 'paced'), ('digin', 'one_of'), ('doze', 'span'), ('enemysquads', 'pick'),
           ('pressure', 'pick'), ('roomfight', 'pick'), ('gunhold', 'next_peek')]
# (module, the function the call is in, which call of that helper in that function): whom it is about
BY_HAND = {
    # a colleague answers a man's call after a delay: the delay is the answerer's
    ('buddy', 'send', 'pick', 1): WHO + '::Def(a.x)',
    # a wedged door tried from outside: the man at the handle (the first call only keeps his die's place and goes with the keyed draws)
    ('digin', 'watch', 'one_of', 1): WHO + '::Man(by.gd.unwrap())',
    ('digin', 'watch', 'one_of', 2): WHO + '::Man(who)',
    # a man at a desk: when he next speaks
    ('doze', 'doze_tick', 'span', 1): WHO + '::Man(gd)',
    ('doze', 'doze_tick', 'span', 2): WHO + '::Man(gd)',
    # each man sent to a squad's fight comes after his own delay
    ('enemysquads', 'contact', 'pick', 1): WHO + '::Man(gd)',
    # a squad thinks before it decides: the squad's own
    ('enemysquads', 'tick_news', 'pick', 1): WHO + '::Group(q.0)',
}
FN = re.compile(r'^\s*(?:pub(?:\([a-z]+\))? )?fn (\w+)(?:<[^>]*>)?\((.*)$')
TYPES = (('Def', 'RoomfightDefId'), ('Op', 'RoomfightOpId'), ('Man', 'GuardId'), ('Body', 'CharId'))

def handed(sig):
    if re.search(r'\bwho: (?:crate::dice::)?Who\b', sig): return 'who'
    for form, ty in TYPES:
        m = re.search(r'\b(\w+): (?:crate::\w+::)?' + ty + r'\b', sig)
        if m: return '%s::%s(%s)' % (WHO, form, m.group(1))
    return None

def main():
    check = '--check' in sys.argv
    todo, notes, files = 0, [], 0
    by_module = {}
    for module, helper in HELPERS: by_module.setdefault(module, []).append(helper)
    for module, helpers in sorted(by_module.items()):
        for path, _, where in bodies(ROOT, module):
            lines = open(path).read().split('\n')
            fn, sig, nth, changed = '?', '', {}, False
            for i, line in enumerate(lines):
                if re.match(r'\s*#\[cfg\(test\)\]', line): break
                m = FN.match(line)
                if m:
                    fn, sig, nth = m.group(1), m.group(2), {}
                    if fn in helpers and 'who: ' not in sig:
                        new = re.sub(r'(fn %s(?:<[^>]*>)?\(g: &mut (?:crate::game::)?Game, )' % fn, r'\1who: %s, ' % WHO, line, count=1)
                        if new == line: notes.append('%s:%d: the helper is not handed the game first: %s' % (where, i + 1, line.strip()[:120])); continue
                        lines[i] = line = new; sig = FN.match(new).group(2); changed = True; todo += 1
                if re.match(r'\s*//', line): continue
                for helper in helpers:
                    if FN.match(lines[i]) and FN.match(lines[i]).group(1) == helper: continue
                    call = re.compile(r'(?<![\w:.])%s\(g, ' % helper)
                    pos = 0
                    while True:
                        c = call.search(lines[i], pos)
                        if not c: break
                        pos = c.end()
                        nth[helper] = nth.get(helper, 0) + 1   # (every call counts, done or not, so a row of BY_HAND means the same on a second run)
                        if re.match(r'who, |crate::dice::Who', lines[i][pos:]): continue
                        who = BY_HAND.get((module, fn, helper, nth[helper])) or handed(sig)
                        todo += 1
                        if not who:
                            notes.append('%s:%d: whom is this about? add a row (%r, %r, %r, %d): %s' % (where, i + 1, module, fn, helper, nth[helper], lines[i].strip()[:110]))
                            continue
                        lines[i] = lines[i][:pos] + who + ', ' + lines[i][pos:]
                        pos += len(who) + 2; changed = True
            if changed and not check: open(path, 'w').write('\n'.join(lines)); files += 1
    print('dice-who: %d %s%s' % (todo, 'to do' if check else 'done', '' if check else ' in %d files' % files))
    for n in notes: print('  ' + n)
    sys.exit(1 if (check and todo) or notes else 0)

main()
