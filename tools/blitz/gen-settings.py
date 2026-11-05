#!/usr/bin/env python3
"""gen-settings.py: writes rust/game/src/root_settings/b_01.rs — settings_set(s, key, vals): one arm a plain field of RenderSettings (the host's settings, which the
bridge does not carry and a tick reads: tools/blitz/host.ts hands them over as text and host_api's host_engine_fill sets them by name).
Run it again when RenderSettings' fields change:    python3 tools/blitz/gen-settings.py [--check]"""
import re,os,sys
ROOT=os.path.normpath(os.path.join(os.path.dirname(os.path.abspath(__file__)),'..','..'))
OUT=f'{ROOT}/rust/game/src/root_settings/b_01.rs'
t=open(f'{ROOT}/rust/game/src/root_settings/types.rs').read()
body=re.search(r'pub struct RenderSettings \{(.*?)\n\}',t,re.S).group(1)
arms=[]
for m in re.finditer(r'pub (r#)?(\w+): ([^,\n]+),',body):
    f=('r#' if m.group(1) else '')+m.group(2); ty=m.group(3).strip(); k=m.group(2).replace('_','')
    if ty=='f64': arms.append(f'        "{k}" => s.{f} = v[0],')
    elif ty=='bool': arms.append(f'        "{k}" => s.{f} = v[0] != 0.0,')
    elif ty=='V3': arms.append(f'        "{k}" => s.{f} = [v[0], v[1], v[2]],')
    elif ty=='[f64; 4]': arms.append(f'        "{k}" => s.{f} = [v[0], v[1], v[2], v[3]],')
    elif ty=='Vec<f64>': arms.append(f'        "{k}" => s.{f} = v.to_vec(),')
TEXT=('''//! THE HOST'S SETTINGS BY NAME, for a host that fills them from outside (written by tools/blitz/gen-settings.py from RenderSettings' fields; run it again)
#![allow(unused_imports, unused_variables)]
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// one field of the settings by its TypeScript name in lower case (a number, a yes/no as 1 / 0, or a row of numbers). False: no plain field of that name.
pub fn settings_set(s: &mut RenderSettings, key: &str, v: &[f64]) -> bool {
    if v.is_empty() { return false; }
    match key {
'''+'\n'.join(arms)+'''
        _ => return false,
    }
    true
}
''')
if '--check' in sys.argv:
    same=open(OUT).read()==TEXT; print('rust/game/src/root_settings/b_01.rs is', 'what would be written' if same else 'NOT what would be written: run tools/blitz/gen-settings.py'); sys.exit(0 if same else 1)
open(OUT,'w').write(TEXT)
print(len(arms),'arms written: rust/game/src/root_settings/b_01.rs')
