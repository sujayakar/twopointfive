#!/usr/bin/env python3
"""gen-knobs.py: writes rust/game/src/rustknobs/b_01.rs — knobs_to_rust, a line a path of rust/sim/src/knobs.rs's list, each found by WALKING THE RUST GAME'S TYPES
(rustknobs.ts finds a path in the registry at run time; Rust has no registry to walk, so the walk is done here and its result is the code).
Run it again whenever rust/sim/src/knobs.rs's list of paths or a knob table's type changes (the written body panics by name when the list is not the one it was made from):
    python3 tools/blitz/gen-knobs.py [--check]        --check writes nothing and exits 1 if the file on disk is not what would be written"""
import re,glob,os,sys,json
ROOT=os.path.normpath(os.path.join(os.path.dirname(os.path.abspath(__file__)),'..','..'))
SK=f'{ROOT}/rust/game'
src=open(f'{ROOT}/rust/sim/src/knobs.rs').read()
paths=re.findall(r'= "([^"]+)"',re.search(r'table! \{ Knob, KNOBS, KNOB_NAMES;(.*?)\n\}',src,re.S).group(1))
structs={}  # 'crate::mod::Name' -> {field: type}
for f in glob.glob(f'{SK}/src/*.rs')+glob.glob(f'{SK}/src/*/types.rs'):   # (the generated types alone: a part's own structs are not the state's)
    rel=os.path.relpath(f,f'{SK}/src'); mod=rel.split('/')[0].replace('.rs','')
    t=open(f).read()
    for m in re.finditer(r'pub struct (\w+)\s*\{(.*?)\n\}',t,re.S):
        fields={}
        for fm in re.finditer(r'^\s*pub (r#)?(\w+): (.+?),\s*$',m.group(2),re.M): fields[fm.group(2)]=(('r#' if fm.group(1) else '')+fm.group(2), fm.group(3))
        structs[f'crate::{mod}::{m.group(1)}']=(fields,mod)
def norm(s): return s.replace('_','').replace('#','').lower()
def qual(ty,mod):
    ty=ty.strip()
    if ty in ('f64','bool','i32','usize','u32','V3','Quat','String'): return ty
    if ty.startswith('crate::'): return ty
    if re.match(r'^\w+$',ty): return f'crate::{mod}::{ty}'
    return ty
def step(expr,ty,mod,key):
    ty=ty.strip()
    if ty=='V3': ty='[f64; 3]'
    if ty=='Quat': ty='[f64; 4]'
    m=re.match(r'^Option<(.*)>$',ty)
    if m: return step(f'{expr}.as_ref().unwrap()',m.group(1),mod,key)
    m=re.match(r'^\[(.*); *\d+\]$',ty) or re.match(r'^Vec<(.*)>$',ty)
    if m:
        assert key.isdigit(),(expr,ty,key); return f'{expr}[{key}]',m.group(1),mod
    m=re.match(r'^(?:crate::js::)?JsMap<String, *(.*)>$',ty)
    if m: return f'{expr}.get(String::from("{key}")).unwrap()',m.group(1),mod
    m=re.match(r'^(?:crate::js::)?JsMap<(crate::[\w:]+), *(.*)>$',ty)
    if m:   # keyed by an enum of words
        var=''.join(w[:1].upper()+w[1:] for w in re.split(r'[-_ ]',key))
        return f'{expr}.get({m.group(1)}::{var}).unwrap()',m.group(2),mod
    q=qual(ty,mod)
    if q in structs:
        fields,smod=structs[q]
        for fn,(rf,ft) in fields.items():
            if norm(fn)==norm(key): return f'{expr}.{rf}',ft,smod
        raise KeyError(f'{q} has no field for "{key}": {list(fields)[:40]}')
    raise KeyError(f'cannot step into {ty} ({q}) with "{key}" at {expr}')
# (which field of Knobs holds which table of the TypeScript's registry: the bridge's schema says, `B.knob("<NAME>")` a field)
SCHEMA=json.load(open(f'{ROOT}/tools/blitz/schema.json'))
knobtab={}
for f in SCHEMA['types']['crate::knobs::Knobs']['fields']:
    m=re.match(r'^B\.knob\("([^"]+)"\)$',f.get('expr') or '')
    if m: knobtab[m.group(1)]='g.k.'+f['rust']
kfields=structs['crate::knobs::Knobs'][0]
ROOT_OVERRIDE={'LONG_STEADY':('g.m.character.long_steady','crate::character::CharacterModLONGSTEADY','character')}
roots={}  # name -> (binding, expr, type, mod)
lets=[]
def anim_root(name):
    if name in ROOT_OVERRIDE: return ROOT_OVERRIDE[name]
    low=name.lower()
    for mod in ('character','rig_props'):
        t=open(f'{SK}/src/{mod}/types.rs').read()
        m=re.search(rf'pub const {name}: (\w+) = ',t)
        if m: return (f'crate::{mod}::{name}',m.group(1),mod)
        m=re.search(rf'pub fn {low}\(\) -> ([^{{]+?) \{{',t)
        if m:
            if low not in roots: lets.append(f'    let t_{low} = crate::{mod}::{low}();'); roots[low]=1
            return (f't_{low}',m.group(1).strip(),mod)
    raise KeyError(name)
out=[];bad=[]
for p in paths:
    ks=p.split('.')
    try:
        if ks[0]=='ANIM': expr,ty,mod=anim_root(ks[1]); ks=ks[2:]
        else:
            r=knobtab[ks[0]]; f=r.split('.')[-1]; expr,ty,mod=r,kfields[f][1],'knobs'; ks=ks[1:]
        for k in ks: expr,ty,mod=step(expr,ty,mod,k)
        ty=ty.strip()
        if ty=='f64': v=expr
        elif ty=='bool': v=f'(if {expr} {{ 1.0 }} else {{ 0.0 }})'
        elif ty in('i32','usize','u32'): v=f'({expr} as f64)'
        else: raise KeyError(f'ends at a {ty}')
        out.append((p,v))
    except (KeyError,AssertionError) as e: bad.append((p,str(e)[:200]))
print(len(out),'resolved;',len(bad),'not')
for b in bad[:40]: print('  ',b)
if bad and '--force' not in sys.argv: sys.exit(1)
OUT=f'{ROOT}/rust/game/src/rustknobs/b_01.rs'
body='\n'.join(f'        {v},   // {i} {p}' for i,(p,v) in enumerate(out))
names='\\n'.join(p for p,_ in out)+'\\n'
TEXT=(f'''//! knobs_to_rust (src/game/rustknobs.ts): WRITTEN BY tools/blitz/gen-knobs.py from rust/sim/src/knobs.rs's list of paths and the game's types. Do not edit: run it again.
//! rustknobs.ts finds each path of the module's list in the knob registry (or, under ANIM, in character.ts animTables) as the list is first read, and writes every value
//! as a tick opens. Rust has no registry to walk at run time, so the walk is the generator's and each path is its own line here; the list the lines were made from is
//! held against the module's own the first time (the TypeScript's error for a path that is not a number is a compile error here).
#![allow(unused_imports, unused_variables)]
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{{self, JsMap, JsSet}};
use crate::m;
use crate::v3::{{self, Quat, V3}};
use alloc::{{boxed::Box, string::String, vec, vec::Vec}};

/// the list these lines were made from: rust/sim/src/knobs.rs KNOB_NAMES as the generator read it
const MADE_FROM: &str = "{names}";

/// AS A TICK OPENS: every knob of the list, as it stands now, into the module.   (rustknobs.ts knobsToRust)
pub fn knobs_to_rust(g: &mut Game) {{
    let n = ::sim::knobs::len(0);
    if g.m.rustknobs.rust_knobs.lines.is_none() {{
        // (rustknobs.ts rustKnobLines: the list is read once and each path proven; here the proof is that the lines below were made from this very list)
        let names = unsafe {{ core::slice::from_raw_parts(::sim::knobs::addr(1) as *const u8, ::sim::knobs::len(1)) }};
        if names != MADE_FROM.as_bytes() || n != {len(out)} {{ panic!("rust knobs: the module's list of paths is not the one knobs_to_rust was written from: run tools/blitz/gen-knobs.py again"); }}
        g.m.rustknobs.rust_knobs.lines = Some(Vec::new());
        g.m.rustknobs.rust_knobs.at = (::sim::knobs::addr(0) / 8) as f64;
    }}
{chr(10).join(lets)}
    let v: [f64; {len(out)}] = [
{body}
    ];
    let at = ::sim::knobs::addr(0) as *mut f64;
    for i in 0..n {{ unsafe {{ at.add(i).write(v[i]); }} }}
    g.m.rustknobs.rust_knobs.written += 1.0;
}}

''')
if '--check' in sys.argv:
    same=open(OUT).read()==TEXT; print('rust/game/src/rustknobs/b_01.rs is', 'what would be written' if same else 'NOT what would be written: run tools/blitz/gen-knobs.py'); sys.exit(0 if same else 1)
open(OUT,'w').write(TEXT)
print('written: rust/game/src/rustknobs/b_01.rs')
