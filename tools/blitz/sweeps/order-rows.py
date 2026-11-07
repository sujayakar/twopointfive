#!/usr/bin/env python3
"""A row of the orders' menu made through one call, so that a row listed again is the same row.

    python3 tools/blitz/sweeps/order-rows.py [--check]

Rewrites every `g.tab.orders_order_def.push(<def>)` in the body files of rust/game_rules/src (`p_NN.rs`, `b_NN.rs`; not a
file's tests) into `crate::orders::squad_orders_row(g, <def>)`. The call writes the row this seat's menu has for that id in
place and pushes only an id this menu has never had (orders/p_02.rs), where a push left the row before it in the table for good.
The menu a player sees is the same; the table no longer grows with every re-listing, and its rows are numbered the same
on every machine. Run again on a later tree it rewrites only what is still pushed.

Two pushes stay, and are read past: the call's own, and the wall tools' digitless copy of a row whose digit their own row
has taken (`wall_charge_undigit`), which is a second row of the same id on purpose, made once and kept in a map of its own.
With `--check` nothing is written and the exit is 1 if any other row is still pushed.
"""
import glob, os, re, sys

ROOT = os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', '..', 'rust', 'game_rules', 'src')
PUSH = 'g.tab.orders_order_def.push('
CALL = 'crate::orders::squad_orders_row(g, '
# the pushes that stay, by file: the call's own, and the wall tools' stand-in
STAY = {
    'orders/p_02.rs': ['-> OrdersOrderDefId { g.tab.orders_order_def.push(def) }'],
    'wallcharge/p_02.rs': ['let x = g.tab.orders_order_def.push(row);'],
}

def main():
    check = '--check' in sys.argv
    n, files = 0, 0
    for path in sorted(glob.glob(os.path.join(ROOT, '*', '[pb]_[0-9][0-9].rs'))):
        rel = os.path.relpath(path, ROOT)
        text = open(path).read()
        cut = text.find('#[cfg(test)]')
        body, tail = (text, '') if cut < 0 else (text[:cut], text[cut:])
        keep = STAY.get(rel, [])
        held = body
        for i, line in enumerate(keep): held = held.replace(line, '\0%d\0' % i)
        k = held.count(PUSH)
        if not k: continue
        n += k; files += 1
        if check:
            for i, line in enumerate(held.split('\n'), 1):
                if PUSH in line: print('%s:%d: %s' % (rel, i, line.strip()[:110]))
            continue
        out = held.replace(PUSH, CALL)
        for i, line in enumerate(keep): out = out.replace('\0%d\0' % i, line)
        open(path, 'w').write(out + tail)
        print('%s: %d' % (rel, k))
    print('order-rows: %d row%s %s in %d file%s' % (n, '' if n == 1 else 's', 'still pushed' if check else 'made through the one call', files, '' if files == 1 else 's'))
    sys.exit(1 if check and n else 0)

main()
