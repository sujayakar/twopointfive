//! THE CHARACTER FROM ITS FILES: the mannequin (its skeleton, skin, mesh and clips) and, where a cast file is given, the cast's meshes over it.
//! The files' bytes are handed in: the start reads no file itself.
//! The rows go where the game keeps them: `g.art.rig`, `g.art.clip`, `g.art.mesh`, `g.art.cast` / `_cast_variant`.
use crate::json::{self, Json};
use alloc::{string::String, vec, vec::Vec};
use game::anim_gltf::{CastInfo, CastSide, CastVariant, Clip, GltfCharacter, GltfNode, MeshData, MeshDataJoints, Path, SkinData, Track};
use alloc::sync::Arc;
use game::game::Game;
use game::ids::*;
use game::js::JsMap;

/// One glTF file as it lies on disk: the `.gltf` text, and the bytes of its `.bin` where its buffer is a file beside it (a buffer written into
/// the text as a `data:` address needs none).
#[derive(Clone, Copy)]
pub struct GltfFile<'a> { pub text: &'a str, pub bin: Option<&'a [u8]> }

/// the material slots a cast face is assigned to
pub const SLOT_NAMES: [&str; 11] = ["skin", "cloth_uniform", "cloth_plain", "webbing", "boot", "helmet", "metal_dark", "rubber", "lens", "accent", "eye"];
pub const CAST_MAX_PARTS: usize = 32;

enum Data { F32(Vec<f32>), U8(Vec<u8>), U16(Vec<u16>), U32(Vec<u32>) }
impl Data {
    fn get(&self, i: usize) -> f64 { match self { Data::F32(v) => v[i] as f64, Data::U8(v) => v[i] as f64, Data::U16(v) => v[i] as f64, Data::U32(v) => v[i] as f64 } }
    fn f32(self) -> Vec<f32> { match self { Data::F32(v) => v, _ => panic!("gltf: an accessor that should hold singles holds whole numbers") } }
}
struct Accessor { data: Data, n: usize, count: usize, comp: u32 }

struct Open { g: Json, bins: Vec<Vec<u8>> }

fn base64(t: &[u8]) -> Vec<u8> {
    let mut out: Vec<u8> = Vec::with_capacity(t.len() / 4 * 3);
    let (mut acc, mut bits) = (0u32, 0u32);
    for &c in t {
        let v = match c { b'A'..=b'Z' => c - b'A', b'a'..=b'z' => c - b'a' + 26, b'0'..=b'9' => c - b'0' + 52, b'+' | b'-' => 62, b'/' | b'_' => 63, _ => continue } as u32;
        acc = (acc << 6) | v; bits += 6;
        if bits >= 8 { bits -= 8; out.push((acc >> bits) as u8); }
    }
    out
}

/// the text parsed, every buffer as bytes
fn open(f: GltfFile) -> Open {
    let g = json::parse(f.text).unwrap_or_else(|e| panic!("gltf: {}", e));
    let mut bins: Vec<Vec<u8>> = Vec::new();
    for b in g.get("buffers").items() {
        let uri = b.get("uri").str().unwrap_or("");
        if uri.starts_with("data:") { let at = uri.find(',').map(|i| i + 1).unwrap_or(0); bins.push(base64(uri[at..].as_bytes())); }
        else { bins.push(f.bin.unwrap_or_else(|| panic!("gltf: the buffer '{}' is a file beside the text, and its bytes were not handed in", uri)).to_vec()); }
    }
    Open { g, bins }
}

impl Open {
    /// accessor `i` as a tightly packed array (a byte stride is honored)
    fn accessor(&self, i: f64) -> Accessor {
        let a = self.g.get("accessors").at(i as usize);
        let bv = self.g.get("bufferViews").at(a.get("bufferView").num().expect("gltf: an accessor's view") as usize);
        let n = match a.get("type").str() { Some("SCALAR") => 1, Some("VEC2") => 2, Some("VEC3") => 3, Some("VEC4") => 4, Some("MAT4") => 16, t => panic!("gltf: accessor type {:?}", t) };
        let comp = a.get("componentType").num().unwrap_or(0.0) as u32;
        let cs = match comp { 5120 | 5121 => 1, 5122 | 5123 => 2, 5125 | 5126 => 4, c => panic!("unsupported componentType {}", c) };
        let count = a.get("count").num().unwrap_or(0.0) as usize;
        let buf = &self.bins[bv.get("buffer").num().unwrap_or(0.0) as usize];
        let off = bv.get("byteOffset").num().unwrap_or(0.0) as usize + a.get("byteOffset").num().unwrap_or(0.0) as usize;
        let stride = bv.get("byteStride").num().map(|s| s as usize).unwrap_or(n * cs);
        let at = |k: usize, c: usize| off + k * stride + c * cs;
        let u16_at = |p: usize| u16::from_le_bytes([buf[p], buf[p + 1]]);
        let u32_at = |p: usize| u32::from_le_bytes([buf[p], buf[p + 1], buf[p + 2], buf[p + 3]]);
        let each = |f: &mut dyn FnMut(usize)| for k in 0..count { for c in 0..n { f(at(k, c)); } };
        let data = match comp {
            5126 => { let mut v = Vec::with_capacity(count * n); each(&mut |p| v.push(f32::from_bits(u32_at(p)))); Data::F32(v) }
            5121 => { let mut v = Vec::with_capacity(count * n); each(&mut |p| v.push(buf[p])); Data::U8(v) }
            5123 => { let mut v = Vec::with_capacity(count * n); each(&mut |p| v.push(u16_at(p))); Data::U16(v) }
            5125 => { let mut v = Vec::with_capacity(count * n); each(&mut |p| v.push(u32_at(p))); Data::U32(v) }
            c => panic!("unsupported componentType {}", c),
        };
        Accessor { data, n, count, comp }
    }
}

fn num_or(v: &Json, d: f64) -> f64 { v.num().unwrap_or(d) }
fn v3_or(v: &Json, d: [f64; 3]) -> [f64; 3] { v.v3().unwrap_or(d) }

/// what the mannequin's file gives, before any row is made: a cast shares everything of it but the mesh
struct Base { nodes: Vec<GltfNode>, order: Vec<u32>, skin: SkinData, mesh: MeshData, clips: Vec<Clip>, node_by_name: JsMap<String, f64> }

fn load_base(f: GltfFile) -> Base {
    let o = open(f);
    let g = &o.g;
    // ---- nodes ----
    let mut nodes: Vec<GltfNode> = g.get("nodes").items().iter().map(|nd| GltfNode {
        name: String::from(nd.get("name").str().unwrap_or("")), parent: -1.0, children: nd.get("children").nums(),
        t: v3_or(nd.get("translation"), [0.0, 0.0, 0.0]),
        r: { let r = nd.get("rotation").nums(); if r.len() == 4 { [r[0], r[1], r[2], r[3]] } else { [0.0, 0.0, 0.0, 1.0] } },
        s: v3_or(nd.get("scale"), [1.0, 1.0, 1.0]),
    }).collect();
    for i in 0..nodes.len() { for c in nodes[i].children.clone() { nodes[c as usize].parent = i as f64; } }
    let roots: Vec<f64> = g.get("scenes").at(num_or(g.get("scene"), 0.0) as usize).get("nodes").nums();
    let mut order: Vec<u32> = Vec::new();
    fn visit(nodes: &[GltfNode], order: &mut Vec<u32>, i: f64) { order.push(i as u32); for &c in nodes[i as usize].children.iter() { visit(nodes, order, c); } }
    for &r in roots.iter() { visit(&nodes, &mut order, r); }
    assert!(nodes.len() <= sim::skeleton::MAX_NODES, "a rig of {} nodes, and a man is posed on {} at most", nodes.len(), sim::skeleton::MAX_NODES);
    assert!({ let mut seen = order.clone(); seen.sort_unstable(); seen.into_iter().eq(0..nodes.len() as u32) }, "a rig whose scene does not reach each of its {} nodes once", nodes.len());
    let mut node_by_name: JsMap<String, f64> = JsMap::new();
    for (i, nd) in nodes.iter().enumerate() { node_by_name.set(nd.name.clone(), i as f64); }

    // ---- skin ----
    let sk = g.get("skins").at(0);
    let skin = SkinData { joints: sk.get("joints").nums(), inverse_bind: o.accessor(sk.get("inverseBindMatrices").num().expect("gltf: the skin's inverse bind matrices")).data.f32() };

    // ---- mesh (merge all primitives of the first skinned mesh) ----
    let mesh_node = g.get("nodes").items().iter().position(|nd| nd.has("mesh") && nd.has("skin")).expect("gltf: no skinned mesh");
    let mesh = g.get("meshes").at(g.get("nodes").at(mesh_node).get("mesh").num().unwrap() as usize);
    struct Prim { pos: Accessor, nrm: Accessor, jn: Accessor, wt: Accessor, idx: Accessor, mat: f64, material: u8 }
    let (mut vcount, mut icount) = (0usize, 0usize);
    let prims: Vec<Prim> = mesh.get("primitives").items().iter().enumerate().map(|(pi, p)| {
        let at = p.get("attributes");
        let acc = |k: &str| o.accessor(at.get(k).num().unwrap_or_else(|| panic!("gltf: a primitive with no {}", k)));
        let (pos, nrm, jn, wt, idx) = (acc("POSITION"), acc("NORMAL"), acc("JOINTS_0"), acc("WEIGHTS_0"), o.accessor(p.get("indices").num().expect("gltf: a primitive with no indices")));
        vcount += pos.count; icount += idx.count;
        let m = p.get("material").num();
        Prim { pos, nrm, jn, wt, idx, mat: m.unwrap_or(pi as f64), material: match m { Some(x) if x < 255.0 => x as u8, _ => 255 } }
    }).collect();
    let joints_are_u16 = prims.iter().any(|p| p.jn.comp == 5123);
    let mut md = MeshData {
        positions: vec![0.0; vcount * 3], normals: vec![0.0; vcount * 3], weights: vec![0.0; vcount * 4],
        joints: if joints_are_u16 { MeshDataJoints::Uint16Array(vec![0; vcount * 4]) } else { MeshDataJoints::Uint8Array(vec![0; vcount * 4]) },
        vertex_count: vcount as f64,
        bbox_min: [1e9, 1e9, 1e9], bbox_max: [-1e9, -1e9, -1e9], part: None,
        mat_index: Some(vec![0; vcount]),
    };
    let (mut vo, mut io) = (0usize, 0usize);
    for p in prims {
        let n = p.pos.count;
        let (pos, nrm) = (p.pos.data.f32(), p.nrm.data.f32());
        md.positions[vo * 3..vo * 3 + pos.len()].copy_from_slice(&pos);
        md.normals[vo * 3..vo * 3 + nrm.len()].copy_from_slice(&nrm);
        // (weights may be u8 / u16 normalised in some files; renormalised anyway)
        let wn = match p.wt.comp { 5126 => 1.0, 5121 => 1.0 / 255.0, _ => 1.0 / 65535.0 };
        for k in 0..n {
            let (a, b, c, d) = (p.wt.data.get(k * 4) * wn, p.wt.data.get(k * 4 + 1) * wn, p.wt.data.get(k * 4 + 2) * wn, p.wt.data.get(k * 4 + 3) * wn);
            let s = { let s = a + b + c + d; if s != 0.0 && !s.is_nan() { s } else { 1.0 } };
            let v = vo + k;
            md.weights[v * 4] = (a / s) as f32; md.weights[v * 4 + 1] = (b / s) as f32; md.weights[v * 4 + 2] = (c / s) as f32; md.weights[v * 4 + 3] = (d / s) as f32;
            for c2 in 0..4 { let j = p.jn.data.get(k * 4 + c2); match &mut md.joints { MeshDataJoints::Uint16Array(a) => a[v * 4 + c2] = j as u16, MeshDataJoints::Uint8Array(a) => a[v * 4 + c2] = j as u8 } }
            md.mat_index.as_mut().unwrap()[v] = p.material;
            let (x, y, z) = (md.positions[v * 3] as f64, md.positions[v * 3 + 1] as f64, md.positions[v * 3 + 2] as f64);
            md.bbox_min = [game::m::min(md.bbox_min[0], x), game::m::min(md.bbox_min[1], y), game::m::min(md.bbox_min[2], z)];
            md.bbox_max = [game::m::max(md.bbox_max[0], x), game::m::max(md.bbox_max[1], y), game::m::max(md.bbox_max[2], z)];
        }
        vo += n; io += p.idx.count;
    }

    // ---- animations ----
    let mut clips: Vec<Clip> = Vec::new();
    for an in g.get("animations").items() {
        let mut tracks: Vec<Track> = Vec::new();
        let mut duration = 0.0f64;
        for ch in an.get("channels").items() {
            let smp = an.get("samplers").at(ch.get("sampler").num().unwrap() as usize);
            let path = match ch.get("target").get("path").str() { Some("translation") => Path::Move, Some("rotation") => Path::Turn, Some("scale") => Path::Scale, _ => continue };
            let times = o.accessor(smp.get("input").num().unwrap()).data.f32();
            let values = o.accessor(smp.get("output").num().unwrap()).data.f32();
            duration = game::m::max(duration, times[times.len() - 1] as f64);
            tracks.push(Track { node: ch.get("target").get("node").num().unwrap() as usize, path, times, values, step: smp.get("interpolation").str() == Some("STEP"), line: 0 });
        }
        let name = String::from(an.get("name").str().unwrap_or(""));
        let clip = Clip::new(name.clone(), game::m::max(duration, 1.0 / 30.0), tracks);
        // (a Map: a name met twice keeps its first place and takes the later clip)
        assert!(clip.tracks.iter().all(|t| t.node < nodes.len()), "the clip \"{name}\" moves a node the rig has not");
        match clips.iter_mut().find(|c| c.name == name) { Some(c) => *c = clip, None => clips.push(clip) }
    }
    Base { nodes, order, skin, mesh: md, clips, node_by_name }
}

/// THE CAST's mesh and its parts and variants, over the mannequin's skeleton. Refuses anything off the file's contract.
fn load_cast(f: GltfFile, base: &Base) -> (MeshData, Vec<String>, Vec<CastVariant>) {
    let o = open(f);
    let g = &o.g;
    let base_joint = |name: &str| -> Option<usize> { let mut hit = None; for (k, &node) in base.skin.joints.iter().enumerate() { if base.nodes[node as usize].name == name { hit = Some(k); } } hit };
    let sk = g.get("skins").at(0);
    if sk.is_null() { panic!("cast: no skin"); }
    let jmap: Vec<usize> = sk.get("joints").nums().iter().map(|&n| { let name = g.get("nodes").at(n as usize).get("name").str().unwrap_or(""); base_joint(name).unwrap_or_else(|| panic!("cast: joint '{}' is not on the mannequin's skeleton", name)) }).collect();
    let slot_of = |mi: Option<f64>| -> f64 {
        let full = mi.map(|i| g.get("materials").at(i as usize).get("name").str().unwrap_or("")).unwrap_or("");
        // (`name.replace(/\.\d+$/, '')`: a copy's ".001" taken off)
        let name = match full.rfind('.') { Some(i) if i + 1 < full.len() && full[i + 1..].bytes().all(|c| c.is_ascii_digit()) => &full[..i], _ => full };
        SLOT_NAMES.iter().position(|s| *s == name).unwrap_or_else(|| panic!("cast: material '{}' is not a slot ({})", name, SLOT_NAMES.join(" "))) as f64
    };
    let part_nodes: Vec<&Json> = g.get("nodes").items().iter().filter(|nd| nd.has("mesh") && nd.has("skin")).collect();
    if part_nodes.is_empty() { panic!("cast: no skinned mesh"); }
    if part_nodes.len() > CAST_MAX_PARTS { panic!("cast: {} parts, at most {}", part_nodes.len(), CAST_MAX_PARTS); }
    let parts: Vec<String> = part_nodes.iter().map(|nd| String::from(nd.get("name").str().unwrap_or(""))).collect();
    for (i, p) in parts.iter().enumerate() { if parts[..i].contains(p) { panic!("cast: part names are not unique"); } }
    struct Prim { pos: Accessor, nrm: Accessor, jn: Accessor, wt: Accessor, idx: Accessor, part: usize, slot: f64 }
    let (mut vcount, mut icount) = (0usize, 0usize);
    let mut prims: Vec<Prim> = Vec::new();
    for (part, nd) in part_nodes.iter().enumerate() {
        for p in g.get("meshes").at(nd.get("mesh").num().unwrap() as usize).get("primitives").items() {
            if num_or(p.get("mode"), 4.0) != 4.0 { panic!("cast: part '{}' has a primitive that is not triangles", parts[part]); }
            if nd.get("skin").num() != Some(0.0) { panic!("cast: part '{}' is not on skin 0", parts[part]); }
            let at = p.get("attributes");
            for a in ["POSITION", "NORMAL", "JOINTS_0", "WEIGHTS_0"] { if !at.has(a) { panic!("cast: part '{}' has a primitive with no {}", parts[part], a); } }
            if !p.has("indices") { panic!("cast: part '{}' has a primitive with no indices", parts[part]); }
            let acc = |k: &str| o.accessor(at.get(k).num().unwrap());
            let (pos, nrm, jn, wt, idx) = (acc("POSITION"), acc("NORMAL"), acc("JOINTS_0"), acc("WEIGHTS_0"), o.accessor(p.get("indices").num().unwrap()));
            if pos.comp != 5126 || pos.n != 3 || nrm.comp != 5126 || nrm.n != 3 || jn.n != 4 || wt.n != 4 { panic!("cast: part '{}' has an attribute of the wrong type", parts[part]); }
            if nrm.count != pos.count || jn.count != pos.count || wt.count != pos.count || idx.count % 3 != 0 { panic!("cast: part '{}' has attributes of different lengths, or indices that are not whole triangles", parts[part]); }
            vcount += pos.count; icount += idx.count;
            prims.push(Prim { pos, nrm, jn, wt, idx, part, slot: slot_of(p.get("material").num()) });
        }
    }
    let mut md = MeshData {
        positions: vec![0.0; vcount * 3], normals: vec![0.0; vcount * 3], weights: vec![0.0; vcount * 4], joints: MeshDataJoints::Uint8Array(vec![0; vcount * 4]),
        vertex_count: vcount as f64, bbox_min: [1e9, 1e9, 1e9], bbox_max: [-1e9, -1e9, -1e9], part: Some(vec![0; vcount]),
        mat_index: None,
    };
    let (mut vo, mut io) = (0usize, 0usize);
    for p in prims {
        let n = p.pos.count;
        let (pos, nrm) = (p.pos.data.f32(), p.nrm.data.f32());
        md.positions[vo * 3..vo * 3 + pos.len()].copy_from_slice(&pos);
        md.normals[vo * 3..vo * 3 + nrm.len()].copy_from_slice(&nrm);
        let wn = match p.wt.comp { 5126 => 1.0, 5121 => 1.0 / 255.0, _ => 1.0 / 65535.0 };
        for k in 0..n {
            let v = vo + k;
            let mut sum = 0.0f64;
            for c in 0..4 {
                let x = p.wt.data.get(k * 4 + c) * wn;
                md.weights[v * 4 + c] = x as f32; sum += x;
                let j = if x > 0.0 { *jmap.get(p.jn.data.get(k * 4 + c) as usize).unwrap_or_else(|| panic!("cast: part '{}' is weighted to a joint index past the skin's list", parts[p.part])) } else { 0 };
                if let MeshDataJoints::Uint8Array(a) = &mut md.joints { a[v * 4 + c] = j as u8; }
            }
            if !(sum > 0.0) { panic!("cast: part '{}' has a vertex with no weight", parts[p.part]); }
            // (`md.weights[i] /= sum`: the single read back, divided as a double, stored as a single)
            for c in 0..4 { md.weights[v * 4 + c] = (md.weights[v * 4 + c] as f64 / sum) as f32; }
            md.part.as_mut().unwrap()[v] = p.part as u8;
            for c in 0..3 { let x = md.positions[v * 3 + c] as f64; if !x.is_finite() { panic!("cast: part '{}' has a non-finite position", parts[p.part]); } md.bbox_min[c] = game::m::min(md.bbox_min[c], x); md.bbox_max[c] = game::m::max(md.bbox_max[c], x); }
        }
        for k in 0..p.idx.count { let ix = p.idx.data.get(k) as usize; if ix >= n { panic!("cast: part '{}' indexes past its vertices", parts[p.part]); }  }
        vo += n; io += p.idx.count;
    }
    let decl = g.get("extras").get("cast").get("variants");
    let mut named: Vec<(String, CastSide, Vec<String>)> = if decl.is_null() { vec![(String::from("all"), CastSide::Operator, parts.clone())] } else {
        decl.members().iter().map(|(name, d)| {
            let side = match d.get("side").str() { Some("operator") => CastSide::Operator, Some("defender") => CastSide::Defender, Some("civilian") => CastSide::Civilian, Some("responder") => CastSide::Responder, s => panic!("cast: variant '{}' has side '{}'", name, s.unwrap_or("undefined")) };
            (name.clone(), side, d.get("parts").items().iter().map(|p| String::from(p.str().unwrap_or(""))).collect())
        }).collect()
    };
    // (`Object.keys(decl).sort()`: by UTF-16 code units, which for these names is by bytes)
    named.sort_by(|a, b| a.0.encode_utf16().cmp(b.0.encode_utf16()));
    let variants: Vec<CastVariant> = named.into_iter().map(|(name, side, wears)| {
        let mut mask = 0i32;
        for pn in wears.iter() { let i = parts.iter().position(|p| p == pn).unwrap_or_else(|| panic!("cast: variant '{}' wears '{}', which is not a part", name, pn)); mask |= 1i32.wrapping_shl(i as u32); }
        if mask == 0 { panic!("cast: variant '{}' wears nothing", name); }
        CastVariant { name, side, mask: (mask as u32) as f64 }
    }).collect();
    (md, parts, variants)
}

/// THE CHARACTER A GAME STANDS ON, as rows of `g`: the mannequin, or with `cast` the cast's meshes over its skeleton, skin and clips
/// (a cast off its contract is refused here and not passed over: a test should not stand on another mesh
/// than it asked for). Answers the character's row.
pub fn character(g: &mut Game, mannequin: GltfFile, cast: Option<GltfFile>) -> RigId {
    let base = load_base(mannequin);
    let dressed = cast.map(|c| load_cast(c, &base));
    let art = Arc::make_mut(&mut g.art);
    let mut clips: JsMap<String, ClipId> = JsMap::new();
    for c in base.clips { let name = c.name.clone(); let id = art.clip.push(c); clips.set(name, id); }
    let (mesh, cast) = match dressed {
        None => (art.mesh.push(base.mesh), None),
        Some((md, parts, variants)) => {
            let variants: Vec<AnimGltfCastVariantId> = variants.into_iter().map(|v| art.variant.push(v)).collect();
            (art.mesh.push(md), Some(art.cast.push(CastInfo { parts, variants })))
        }
    };
    let mut rig = GltfCharacter { parent: base.nodes.iter().map(|n| n.parent as i32).collect(), nodes: base.nodes, order: base.order, skin: base.skin, mesh, clips, node_by_name: base.node_by_name, cast, roles: Default::default(), back: None };
    rig.roles = game::character::roles_of(&rig);
    rig.back = game::packside::back_of(art, &rig);
    art.rig.push(rig)
}

