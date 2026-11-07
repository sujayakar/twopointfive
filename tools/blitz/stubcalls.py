#!/usr/bin/env python3
"""Every call in rust/game that lands on a skeleton stub (a `todo!()` of sig.rs that no p_NN.rs / b_NN.rs gives a body, or whose body's name is missing from
   the module's `pub use` list), by module, with who calls it: the list of panics waiting in play.
   tools/blitz/stubcalls.py [module-regex of the CALLERS]"""
import re, os, sys, glob, collections
J=os.path.join(os.path.dirname(os.path.abspath(__file__)),'..','..','rust','game','src')
want=re.compile(sys.argv[1]) if len(sys.argv)>1 else None
stub=set(); have=set(); unnamed=set()
for d in sorted(os.listdir(J)):
    p=f'{J}/{d}'
    if not os.path.isdir(p): continue
    sig=open(f'{p}/sig.rs',encoding='utf8').read() if os.path.exists(f'{p}/sig.rs') else ''
    for m in re.finditer(r'^pub fn (\w+)[^\n]*\{ todo!\(\) \}', sig, re.M): stub.add((d,m.group(1)))
    # (a body counts only if the module's mod.rs names it: `pub use p_NN::{…}` wins over the glob of sig.rs's stubs, and a body it does not name is never called)
    mod=open(f'{p}/mod.rs',encoding='utf8').read() if os.path.exists(f'{p}/mod.rs') else ''
    named=set(x.strip().split(' as ')[-1] for m in re.finditer(r'pub use [pb]_\d+::\{([^}]*)\}', mod) for x in m.group(1).split(',') if x.strip())
    for f in glob.glob(f'{p}/[pb]_*.rs'):
        for m in re.finditer(r'^pub fn (\w+)', open(f,encoding='utf8').read(), re.M):
            if m.group(1) in named: have.add((d,m.group(1)))
            elif (d,m.group(1)) in stub: unnamed.add((d,m.group(1),os.path.basename(f)))
stub-=have
calls=collections.defaultdict(set)
for d in sorted(os.listdir(J)):
    p=f'{J}/{d}'
    if not os.path.isdir(p) or (want and not want.search(d)): continue
    for f in glob.glob(f'{p}/[pb]_*.rs'):
        t=open(f,encoding='utf8').read(); t=t.split('#[cfg(test)]')[0]
        for m in re.finditer(r'crate::(\w+)::(\w+)\(', t):
            if (m.group(1),m.group(2)) in stub: calls[(m.group(1),m.group(2))].add(f'{d}/{os.path.basename(f)[:-3]}')
        for m in re.finditer(r'(?<![\w:])(\w+)\(', t):
            if (d,m.group(1)) in stub: calls[(d,m.group(1))].add(f'{d}/{os.path.basename(f)[:-3]}')
bym=collections.defaultdict(list)
for (mo,fn),who in calls.items(): bym[mo].append((fn,sorted(who)))
for mo in sorted(bym): print(mo+': '+' · '.join(f'{fn} <{",".join(w[:3])}>' for fn,w in sorted(bym[mo])))
print(len(calls),'stubs called;',len(stub),'stubs in all')
for d,fn,f in sorted(unnamed): print(f'A BODY THE MODULE DOES NOT NAME (the stub still answers): {d}/{f} {fn}: add it to {d}/mod.rs `pub use {f[:-3]}::{{…}}`')
