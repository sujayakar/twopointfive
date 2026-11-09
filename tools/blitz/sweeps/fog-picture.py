#!/usr/bin/env python3
"""The fog of war's record cut in two: what the team knows, and the picture's block.

    python3 tools/blitz/sweeps/fog-picture.py [--check]

`fogofwar::FogOfWar` held two things in one part of the game's state. What the team knows (who is seen, which doors,
lamps and loose things are remembered and how, who is left out) is the game's: rules read it and every machine of one
game holds it the same. The picture's block (the renderer's rows, the lit fog's list of lamps cut by distance from the
seat a machine shows, a lamp's kept glow, the list kept for a viewer's draw with the fog off) is one machine's. This program moves the second into a
record of its own, `fogofwar::FogPicture` at `g.fog_picture`, which the wire tool lists among the host's own, so it has
no word of the state.

What it does, each step skipped where it is already done:
  1. the types (rust/game_types): the fields of FIELDS leave `FogOfWar` for the new `FogPicture`, with their starting
     values; `full_on` (true while the fog is on and the pass has run) is renamed `live`; `Game` gains `fog_picture`.
  2. the bodies (every .rs of rust/ that a person writes): `fog_of_war.<field>` becomes `fog_picture.<field>`, the fog's
     own `full_on` becomes `live`, and a game stood anew starts with a blank picture.
  3. the wire (tools/wire/wire.ts, by its own commands): the fields off `FogOfWar`'s line, a line for `FogPicture`, and
     `game::Game.fog_picture` under the page's name `fogOfWar`: the page keeps ONE fog object, the host's view of both
     records, so nothing under src/ changes. Each field stays on the lists of tools/wire/fields.json it was on.
     `fog_picture` joins HOSTS_OWN.

Run again on a later tree it sweeps only the bodies (a new line that names a moved field through `fog_of_war`).
With `--check` nothing is written and the exit is 1 if a body still does.
"""
import glob, json, os, re, subprocess, sys

ROOT = os.path.normpath(os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', '..'))
FIELDS = ['frame', 'lamp_pick', 'lamp_n', 'lit_stats', 'face_recs', 'full_stats', 'viewer', 'look_frames', 'raw_list', 'raw_at']
TYPES = os.path.join(ROOT, 'rust/game_types/src/fogofwar/types.rs')
GAME = os.path.join(ROOT, 'rust/game_types/src/game/types.rs')
STAND = os.path.join(ROOT, 'rust/game_rules/src/tick/game/stand.rs')
WIRE = os.path.join(ROOT, 'tools/wire/wire.ts')
LISTS = os.path.join(ROOT, 'tools/wire/fields.json')
# files a tool writes, and the build's own folders
NOT_BODIES = ('/target/', '/prebuilt/', 'game_wire/src/io.rs', 'game_wire/src/puppet.rs', 'game/src/state_words.rs', 'game/src/call_gen.rs')

def read(p): return open(p, encoding='utf-8').read()
def write(p, t): open(p, 'w', encoding='utf-8').write(t)
def once(t, a, b, what):
    if t.count(a) != 1: sys.exit('fog-picture: %s: expected one `%s`, found %d' % (what, a.strip()[:60], t.count(a)))
    return t.replace(a, b)

def types():
    t = read(TYPES)
    s0 = t.index('pub struct FogOfWar {'); s1 = t.index('\n}\n', s0)
    b0 = t.index('        FogOfWar {\n', s1); b1 = t.index('\n        }\n', b0)
    struct, blank = t[s0:s1], t[b0:b1]
    decl, init = [], []
    for f in FIELDS:
        m = re.search(r'^    pub %s: .*,\n' % f, struct, re.M); n = re.search(r'^            %s: .*,\n' % f, blank, re.M)
        if not m or not n: sys.exit('fog-picture: FogOfWar has no field %s' % f)
        decl.append(m.group(0)); init.append(n.group(0))
        struct = struct.replace(m.group(0), ''); blank = blank.replace(n.group(0), '')
    struct = once(struct, '    pub full_on: bool,\n', '    /// the fog is on and the pass has run: the team\'s rule is live (`ours_on`)\n    pub live: bool,\n', 'FogOfWar')
    blank = once(blank, '            full_on: false,\n', '            live: false,\n', 'FogOfWar::blank')
    t = t[:s0] + struct + t[s1:b0] + blank + t[b1:]
    picture = ''.join([
        '\n/// THE PICTURE\'S BLOCK OF THE FOG OF WAR: one machine\'s, made from the game\'s state every tick and read by nothing in it (fogofwar/picture.rs).\n',
        '/// The game lists it among the host\'s own, so it has no word of the state: two machines that show different seats need not agree on it.\n',
        '#[derive(Clone, Debug)]\npub struct FogPicture {\n', ''.join(decl), '}\nimpl FogPicture {\n    pub fn blank() -> Self {\n        FogPicture {\n', ''.join(init),
        '        }\n    }\n}\nimpl Default for FogPicture { fn default() -> Self { Self::blank() } }\n'])
    t = once(t, 'impl Default for FogOfWar { fn default() -> Self { Self::blank() } }\n', 'impl Default for FogOfWar { fn default() -> Self { Self::blank() } }\n' + picture, 'types.rs')
    write(TYPES, t)
    g = read(GAME)
    g = once(g, '    pub fog_of_war: crate::fogofwar::FogOfWar,\n', '    pub fog_of_war: crate::fogofwar::FogOfWar,\n    pub fog_picture: crate::fogofwar::FogPicture,\n', 'Game')
    g = once(g, '            fog_of_war: crate::fogofwar::FogOfWar::blank(),\n', '            fog_of_war: crate::fogofwar::FogOfWar::blank(),\n            fog_picture: crate::fogofwar::FogPicture::blank(),\n', 'Game::blank')
    write(GAME, g)
    s = read(STAND)
    s = once(s, '    g.fog_of_war = crate::fogofwar::fog_of_war_new(g);\n', '    g.fog_of_war = crate::fogofwar::fog_of_war_new(g);\n    g.fog_picture = crate::fogofwar::FogPicture::blank();\n', 'the stand')
    write(STAND, s)
    return decl

def bodies(check):
    moved = re.compile(r'\bfog_of_war\.(%s)\b' % '|'.join(FIELDS))
    n = files = 0
    for path in sorted(glob.glob(os.path.join(ROOT, 'rust', '**', '*.rs'), recursive=True)):
        if any(x in path for x in NOT_BODIES): continue
        t = read(path)
        u, k = moved.subn(r'fog_picture.\1', t)
        if '/fogofwar/' in path and '/game_types/' not in path:
            u, k2 = re.subn(r'\bfull_on\b', 'live', u); k += k2
        if not k: continue
        n += k; files += 1
        if not check: write(path, u)
    print('fog-picture: %d names in %d files %s' % (n, files, 'still to move' if check else 'moved'))
    return n

def tool(*a):
    r = subprocess.run(['bun', 'run', WIRE] + list(a), cwd=ROOT, capture_output=True, text=True)
    if r.returncode: sys.exit('fog-picture: wire %s: %s' % (' '.join(a[:2]), (r.stderr or r.stdout).strip()))
    print('  ' + r.stdout.strip().split('\n')[-1])

def wire(decl):
    before = json.loads(read(LISTS))
    ty = {m.group(1): m.group(2) for m in (re.match(r'    pub (\w+): (.*),\n', d) for d in decl)}
    plain = [f for f in FIELDS if ',' not in ty[f]]          # (the tool's list of a struct's fields is cut at commas)
    tool('add-struct', 'fogofwar::FogPicture', ','.join('%s:%s' % (f, ty[f]) for f in plain))
    for f in FIELDS:
        if f not in plain: tool('add', 'fogofwar::FogPicture.%s' % f, '--ty', ty[f])
    tool('drop', *['fogofwar::FogOfWar.%s' % f for f in FIELDS + ['full_on']])
    tool('add', 'fogofwar::FogOfWar.live', '--ty', 'bool', '--ts', 'fullOn')
    tool('add', 'game::Game.fog_picture', '--ty', 'crate::fogofwar::FogPicture', '--ts', 'fogOfWar')
    # THE LISTS: each field on the lists it was on, in the place it had; the new part beside the old one
    new = {'fogofwar::FogOfWar.%s' % f: ['fogofwar::FogPicture.%s' % f] for f in FIELDS}
    new['fogofwar::FogOfWar.full_on'] = ['fogofwar::FogOfWar.live']
    new['game::Game.fog_of_war'] = ['game::Game.fog_of_war', 'game::Game.fog_picture']
    L = {k: ([y for x in v for y in (new.get(x, [x]) if isinstance(x, str) else [x])] if isinstance(v, list) else v) for k, v in before.items()}
    write(LISTS, re.sub(r'^ +', '', json.dumps(L, indent=1, ensure_ascii=False), flags=re.M))
    w = read(WIRE)
    w = once(w, "const HOSTS_OWN = new Set(['m.owner', ", "const HOSTS_OWN = new Set(['fog_picture', 'm.owner', ", 'HOSTS_OWN')
    write(WIRE, w)
    tool('gen')

def main():
    check = '--check' in sys.argv
    done = 'pub struct FogPicture' in read(TYPES)
    if check: sys.exit(1 if bodies(True) or not done else 0)
    decl = None if done else types()
    bodies(False)
    if decl: wire(decl)

main()
