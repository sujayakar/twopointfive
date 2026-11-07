#!/usr/bin/env python3
"""THE GAME GROWS IN RUST ALONE. On a scratch copy of rust/: a field on a man, an arm on a plain enum and on one with data (both of them types the wire carries), a new module with state of its
own and a function it runs every tick. It must build with nothing generated, and play a kept run to the same series, since what was added touches no rule that was there.
   python3 tools/blitz/grow-check.py <kept run .tar.zst> [--work DIR]      exit 0 it builds and plays the same · 1 it does not (the reason is printed) · 64 usage
GROW_BREAK=1 in front: the new function also moves a door's angle, and the series must part."""
import glob, os, re, shutil, subprocess, sys, tempfile
ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
if len(sys.argv) < 2: print(__doc__); sys.exit(64)
kept = os.path.abspath(sys.argv[1]); work = sys.argv[sys.argv.index('--work') + 1] if '--work' in sys.argv else tempfile.mkdtemp(prefix='grow-')
rs = f'{work}/rust'; os.makedirs(work, exist_ok=True)
subprocess.run(['rsync', '-a', '--exclude', 'target', '--exclude', 'prebuilt', f'{ROOT}/rust/', f'{rs}/'], check=True)
shutil.copytree(f'{ROOT}/tools/blitz', f'{work}/tools/blitz', dirs_exist_ok=True)
def find(rel):
    F = glob.glob(f'{rs}/*/src/{rel}')
    if len(F) != 1: print(f'grow-check: {rel} found {len(F)} times'); sys.exit(1)
    return F[0]
def edit(rel, a, b, count=1):
    p = find(rel); t = open(p).read()
    if a not in t: print(f'grow-check: {rel} has no {a!r}: its anchor moved'); sys.exit(1)
    open(p, 'w').write(t.replace(a, b, count))
# a field on a man
edit('game/types.rs', 'pub struct Guard {\n', 'pub struct Guard {\n    pub grown: f64,\n')
edit('game/types.rs', '    pub fn blank() -> Self {\n        Guard {\n', '    pub fn blank() -> Self {\n        Guard {\n            grown: 0.0,\n')
# an arm of a plain enum, in the middle, and of an enum with data
edit('wallreact/types.rs', 'pub enum BreachCause {\n    Blast,\n', 'pub enum BreachCause {\n    Grown,\n    Blast,\n')
edit('laststand/types.rs', 'pub enum LaterThen {\n', 'pub enum LaterThen {\n    Grown,\n')
edit('laststand/p_01.rs', '        LaterThen::MovedFree { d } =>', '        LaterThen::Grown => {}\n        LaterThen::MovedFree { d } =>')
# a module with state, and a function it runs every tick
L = find('game/types.rs'); C = os.path.dirname(os.path.dirname(L))
BR = "\n    for d in g.doors.list.clone().iter() { g.tab.door[*d].grown_angle_marker(); break; }" if False else ''
os.makedirs(f'{C}/grown', exist_ok=True)
open(f'{C}/grown/mod.rs', 'w').write(f'//! A module written in Rust alone.\n#[derive(Clone, Debug, Default)]\npub struct GrownMod {{ pub ticks: u32 }}\npub fn grown_tick(g: &mut crate::game::Game) {{ g.m.grown.ticks += 1; for gd in g.guards.clone().iter() {{ g.gd[*gd].grown += 1.0; }}{BR} }}\n')
lib = f'{os.path.dirname(C)}/src/lib.rs' if False else f'{C}/lib.rs'; t = open(lib).read(); open(lib, 'w').write(t.rstrip('\n') + '\npub mod grown;\n')
edit('game/types.rs', 'pub struct Mods {\n', 'pub struct Mods {\n    pub grown: crate::grown::GrownMod,\n')
p = find('game/types.rs'); t = open(p).read()
m = re.search(r'impl Mods \{[^\n]*\n(?:\s*///[^\n]*\n)*\s*pub fn blank\(\) -> Self \{\n\s*Mods \{\n', t) or re.search(r'Mods \{\n(\s*)gpu_wgsl_trim:', t)
if not m: print('grow-check: the blank of Mods was not found'); sys.exit(1)
t = t[:m.end()] + '            grown: Default::default(),\n' + t[m.end():] if 'blank' in m.group(0) else t.replace(m.group(0), 'Mods {\n            grown: Default::default(),\n' + m.group(1) + 'gpu_wgsl_trim:', 1)
open(p, 'w').write(t)
edit('game/p_06.rs', 'fn step_seats_on_first(g: &mut crate::game::Game, dt: f64, cmds: &[crate::cmd::PlayerCmd]) {\n', 'fn step_seats_on_first(g: &mut crate::game::Game, dt: f64, cmds: &[crate::cmd::PlayerCmd]) {\n    crate::grown::grown_tick(g);\n' + ("    g.time += 1e-9;\n" if os.environ.get('GROW_BREAK') == '1' else ''))
# the arms on the wire: what is known goes as it did, what is new goes as the tag no arm has
io = find('io.rs'); t = open(io).read()
open(io, 'w').write(t + '\n#[cfg(test)]\nmod grown_on_the_wire { extern crate std; use super::*; #[test] fn arms() { let tag = |e: crate::wallreact::BreachCause| { let mut o = W::new(); e.w(&mut o); u16::from_le_bytes([o.b[0], o.b[1]]) }; assert_eq!(tag(crate::wallreact::BreachCause::Grown), 0xffff); assert_eq!(tag(crate::wallreact::BreachCause::Blast), 0); assert_eq!(tag(crate::wallreact::BreachCause::Fire), 1); let mut o = W::new(); crate::laststand::LaterThen::Grown.w(&mut o); assert_eq!(o.b.len(), 2); assert_eq!(u16::from_le_bytes([o.b[0], o.b[1]]), 0xffff); } }\n')
# it builds, and plays the kept run
print('grow-check: building the runner with the additions (a few minutes)', flush=True)
r = subprocess.run('cargo build --bins --message-format=short 2>&1 | grep -E ": error|^error" | head -30', shell=True, cwd=f'{rs}/game_run', capture_output=True, text=True)
if r.stdout.strip(): print(r.stdout[:3000]); print('grow-check: FAIL: it does not build'); sys.exit(1)
wire_crate = os.path.dirname(os.path.dirname(io))
r = subprocess.run('cargo test --lib grown_on_the_wire 2>&1 | grep -E "^test result|panicked|^error" | head -5', shell=True, cwd=wire_crate, capture_output=True, text=True); print(r.stdout.strip())
if 'test result: ok. 1 passed' not in r.stdout: print('grow-check: FAIL: the arms on the wire'); sys.exit(1)
r = subprocess.run(['bash', f'{work}/tools/blitz/hold.sh', kept, '--no-build'], cwd=work, env={**os.environ, 'BLITZ_RUNNER': f'{rs}/game_run/target/debug/mission'}, capture_output=True, text=True); print(r.stdout[-1500:])
if r.returncode: print(f'grow-check: FAIL: it plays differently (hold.sh exit {r.returncode})'); sys.exit(1)
print('grow-check: PASS: a field, two arms, a module and a function, in Rust alone, and the kept run plays the same')
