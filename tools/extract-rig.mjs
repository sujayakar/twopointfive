// ---------------------------------------------------------------------------
// Offline rig + animation extractor for the Quaternius Universal Animation
// Library (CC0).
//
// Run: node tools/extract-rig.mjs
// Emits: public/rig.json (header) and public/rig.bin (tracks), served as static
// assets so the runtime can fetch them without bundling 171KB into the JS.
//
// The source GLBs are ~15 MB and are mostly skinned mesh data we do not use,
// since the character is rendered as boxes bound to bones. This keeps only what
// a blocky rig needs:
//
//   * finger bones are dropped (40 of the 65 joints are fingers)
//   * only a stealth-relevant subset of the 86 clips is kept
//   * tracks are resampled onto a fixed frame rate, so runtime sampling is an
//     array index instead of a per-channel keyframe binary search
//   * rotations are quantised to int16, which is plenty for a quaternion
//   * translation is stored only for bones that actually translate; for a rigid
//     skeleton the rest is constant, and the script verifies that rather than
//     assuming it
// ---------------------------------------------------------------------------

import { readFileSync, writeFileSync, mkdirSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const ROOT = join(dirname(fileURLToPath(import.meta.url)), "..");
const OUT_DIR = join(ROOT, "public");

const SOURCES = [
  { file: "assets/Unreal-Godot_UAL1_Standard.glb", clips: [
    "Idle_Loop", "Idle_Torch_Loop", "Walk_Loop", "Walk_Formal_Loop",
    "Jog_Fwd_Loop", "Sprint_Loop",
    "Crouch_Idle_Loop", "Crouch_Fwd_Loop",
    // The library has a complete pistol set and no long-gun clips at all, so
    // every armed character in this game is a pistol character.
    "Pistol_Idle_Loop", "Pistol_Aim_Neutral", "Pistol_Aim_Down", "Pistol_Aim_Up",
    "Pistol_Shoot", "Pistol_Reload",
    "Roll", "Interact", "Death01", "Hit_Chest",
  ]},
  { file: "assets/Unreal-Godot_UAL2_Standard.glb", clips: [
    "Idle_FoldArms_Loop", "Idle_TalkingPhone_Loop", "Slide_Start", "Slide_Loop",
    "Slide_Exit", "ClimbUp_1m",
    // Silent takedown and body carrying. The library has no grab or takedown
    // clip, so a takedown is staged from the attacker's melee swing against the
    // victim's knockback — see guards.ts.
    "Melee_Hook", "Hit_Knockback", "Walk_Carry_Loop",
  ]},
];

/** Bones a blocky rig actually binds boxes to, plus their parent chain. */
const KEEP_BONES = new Set([
  "root", "pelvis", "spine_01", "spine_02", "spine_03", "neck_01", "Head",
  "clavicle_l", "upperarm_l", "lowerarm_l", "hand_l",
  "clavicle_r", "upperarm_r", "lowerarm_r", "hand_r",
  "thigh_l", "calf_l", "foot_l", "ball_l",
  "thigh_r", "calf_r", "foot_r", "ball_r",
]);

const FPS = 30;
/** Below this, a translation track is constant enough to fold into the rest pose. */
const TRANSLATION_EPS = 1e-3;

// --- GLB / glTF reading -----------------------------------------------------

function parseGlb(path) {
  const buf = readFileSync(path);
  let off = 12;
  let json = null;
  let bin = null;
  while (off < buf.length) {
    const clen = buf.readUInt32LE(off);
    const ctype = buf.readUInt32LE(off + 4);
    off += 8;
    if (ctype === 0x4e4f534a) json = JSON.parse(buf.subarray(off, off + clen).toString("utf8"));
    else if (ctype === 0x004e4942) bin = buf.subarray(off, off + clen);
    off += clen;
  }
  return { json, bin };
}

const COMPONENT = {
  5120: [Int8Array, 1], 5121: [Uint8Array, 1], 5122: [Int16Array, 2],
  5123: [Uint16Array, 2], 5125: [Uint32Array, 4], 5126: [Float32Array, 4],
};
const NUM_COMPONENTS = { SCALAR: 1, VEC2: 2, VEC3: 3, VEC4: 4, MAT4: 16 };

function readAccessor(json, bin, index) {
  const acc = json.accessors[index];
  const [Ctor, compSize] = COMPONENT[acc.componentType];
  const n = NUM_COMPONENTS[acc.type];
  const view = json.bufferViews[acc.bufferView];
  const base = (view.byteOffset ?? 0) + (acc.byteOffset ?? 0);
  const stride = view.byteStride ?? 0;

  const out = new Float32Array(acc.count * n);
  if (stride === 0 || stride === compSize * n) {
    // Tightly packed — one view over the whole range.
    const src = new Ctor(bin.buffer, bin.byteOffset + base, acc.count * n);
    out.set(src);
  } else {
    for (let i = 0; i < acc.count; i++) {
      const src = new Ctor(bin.buffer, bin.byteOffset + base + i * stride, n);
      out.set(src, i * n);
    }
  }
  return out;
}

// --- math -------------------------------------------------------------------

function quatSlerp(a, ai, b, bi, t, out, oi) {
  let ax = a[ai], ay = a[ai + 1], az = a[ai + 2], aw = a[ai + 3];
  let bx = b[bi], by = b[bi + 1], bz = b[bi + 2], bw = b[bi + 3];
  let cos = ax * bx + ay * by + az * bz + aw * bw;
  if (cos < 0) { cos = -cos; bx = -bx; by = -by; bz = -bz; bw = -bw; }
  let s0, s1;
  if (cos > 0.9995) {
    s0 = 1 - t; s1 = t;
  } else {
    const theta = Math.acos(cos);
    const sin = Math.sin(theta);
    s0 = Math.sin((1 - t) * theta) / sin;
    s1 = Math.sin(t * theta) / sin;
  }
  let x = s0 * ax + s1 * bx, y = s0 * ay + s1 * by;
  let z = s0 * az + s1 * bz, w = s0 * aw + s1 * bw;
  const len = Math.hypot(x, y, z, w) || 1;
  out[oi] = x / len; out[oi + 1] = y / len; out[oi + 2] = z / len; out[oi + 3] = w / len;
}

/** Samples a keyframed track at time t. Assumes sorted input times. */
function sampleTrack(times, values, comps, t, isQuat, out, oi) {
  const n = times.length;
  if (n === 0) return;
  if (t <= times[0]) {
    for (let c = 0; c < comps; c++) out[oi + c] = values[c];
    return;
  }
  if (t >= times[n - 1]) {
    const base = (n - 1) * comps;
    for (let c = 0; c < comps; c++) out[oi + c] = values[base + c];
    return;
  }
  let lo = 0, hi = n - 1;
  while (hi - lo > 1) {
    const mid = (lo + hi) >> 1;
    if (times[mid] <= t) lo = mid; else hi = mid;
  }
  const span = times[hi] - times[lo];
  const f = span > 1e-9 ? (t - times[lo]) / span : 0;
  if (isQuat) {
    quatSlerp(values, lo * comps, values, hi * comps, f, out, oi);
  } else {
    for (let c = 0; c < comps; c++) {
      out[oi + c] = values[lo * comps + c] * (1 - f) + values[hi * comps + c] * f;
    }
  }
}

// --- extraction -------------------------------------------------------------

function buildSkeleton(json) {
  const parentOf = new Map();
  json.nodes.forEach((n, i) => {
    for (const c of n.children ?? []) parentOf.set(c, i);
  });

  // Keep only whitelisted bones, ordered parents-before-children so a single
  // forward pass can compose world transforms at runtime.
  const wanted = json.nodes
    .map((n, i) => ({ n, i }))
    .filter(({ n }) => KEEP_BONES.has(n.name));

  const depth = ({ i }) => {
    let d = 0, cur = i;
    while (parentOf.has(cur)) { cur = parentOf.get(cur); d++; }
    return d;
  };
  wanted.sort((a, b) => depth(a) - depth(b));

  const indexOfNode = new Map(wanted.map((w, k) => [w.i, k]));
  const bones = wanted.map(({ n, i }) => {
    // Walk up until we find an ancestor that survived the whitelist.
    let p = parentOf.get(i);
    while (p !== undefined && !indexOfNode.has(p)) p = parentOf.get(p);
    return {
      name: n.name,
      parent: p === undefined ? -1 : indexOfNode.get(p),
      node: i,
      t: n.translation ?? [0, 0, 0],
      r: n.rotation ?? [0, 0, 0, 1],
      s: n.scale ?? [1, 1, 1],
    };
  });
  return { bones, indexOfNode };
}

const clipsOut = [];
const chunks = [];
let byteCursor = 0;
let skeleton = null;
const droppedTranslations = new Set();

for (const src of SOURCES) {
  const { json, bin } = parseGlb(join(ROOT, src.file));
  const built = buildSkeleton(json);

  if (!skeleton) {
    skeleton = built;
  } else {
    // Both libraries advertise the same rig; if that ever stops being true the
    // clips are not interchangeable and blending across them would be garbage.
    const a = skeleton.bones.map((b) => b.name).join(",");
    const b = built.bones.map((x) => x.name).join(",");
    if (a !== b) throw new Error(`rig mismatch in ${src.file}: bone list differs`);
  }

  const nBones = skeleton.bones.length;

  for (const clipName of src.clips) {
    const anim = json.animations.find((a) => a.name === clipName);
    if (!anim) {
      console.warn(`  ! clip not found, skipping: ${clipName}`);
      continue;
    }

    // Gather channels for the bones we kept.
    const tracks = new Map(); // boneIndex -> { T?, R?, S? }
    let duration = 0;
    for (const ch of anim.channels) {
      const bi = built.indexOfNode.get(ch.target.node);
      if (bi === undefined) continue;
      const s = anim.samplers[ch.sampler];
      const times = readAccessor(json, bin, s.input);
      const values = readAccessor(json, bin, s.output);
      duration = Math.max(duration, times[times.length - 1]);
      const slot = tracks.get(bi) ?? {};
      slot[ch.target.path] = { times, values };
      tracks.set(bi, slot);
    }

    const frames = Math.max(2, Math.round(duration * FPS) + 1);
    const rot = new Int16Array(frames * nBones * 4);
    const pos = new Float32Array(frames * nBones * 3);

    const tmpR = new Float32Array(4);
    const tmpT = new Float32Array(3);

    for (let f = 0; f < frames; f++) {
      const t = (f / (frames - 1)) * duration;
      for (let b = 0; b < nBones; b++) {
        const trk = tracks.get(b);
        const rest = skeleton.bones[b];

        if (trk?.rotation) {
          sampleTrack(trk.rotation.times, trk.rotation.values, 4, t, true, tmpR, 0);
        } else {
          tmpR.set(rest.r);
        }
        const ro = (f * nBones + b) * 4;
        for (let c = 0; c < 4; c++) {
          rot[ro + c] = Math.max(-32767, Math.min(32767, Math.round(tmpR[c] * 32767)));
        }

        if (trk?.translation) {
          sampleTrack(trk.translation.times, trk.translation.values, 3, t, false, tmpT, 0);
        } else {
          tmpT.set(rest.t);
        }
        const po = (f * nBones + b) * 3;
        pos[po] = tmpT[0]; pos[po + 1] = tmpT[1]; pos[po + 2] = tmpT[2];
      }
    }

    // Which bones actually translate? For a rigid skeleton only the root should,
    // and folding the rest into the rest pose is a large saving — but verify
    // instead of assuming, and report anything surprising.
    const translating = [];
    for (let b = 0; b < nBones; b++) {
      let maxDev = 0;
      const rest = skeleton.bones[b].t;
      for (let f = 0; f < frames; f++) {
        const po = (f * nBones + b) * 3;
        maxDev = Math.max(maxDev,
          Math.abs(pos[po] - rest[0]),
          Math.abs(pos[po + 1] - rest[1]),
          Math.abs(pos[po + 2] - rest[2]));
      }
      if (maxDev > TRANSLATION_EPS) translating.push(b);
      else droppedTranslations.add(skeleton.bones[b].name);
    }

    const posPacked = new Float32Array(frames * translating.length * 3);
    for (let f = 0; f < frames; f++) {
      translating.forEach((b, k) => {
        const from = (f * nBones + b) * 3;
        const to = (f * translating.length + k) * 3;
        posPacked[to] = pos[from];
        posPacked[to + 1] = pos[from + 1];
        posPacked[to + 2] = pos[from + 2];
      });
    }

    const rotBytes = Buffer.from(rot.buffer, rot.byteOffset, rot.byteLength);
    const posBytes = Buffer.from(posPacked.buffer, posPacked.byteOffset, posPacked.byteLength);

    clipsOut.push({
      name: clipName,
      frames,
      duration,
      loop: clipName.endsWith("_Loop"),
      rotOffset: byteCursor,
      posOffset: byteCursor + rotBytes.length,
      posBones: translating,
    });
    chunks.push(rotBytes, posBytes);
    byteCursor += rotBytes.length + posBytes.length;

    const kb = ((rotBytes.length + posBytes.length) / 1024).toFixed(1);
    console.log(`  ${clipName.padEnd(24)} ${frames.toString().padStart(4)} frames  ${kb.padStart(7)} KB  translating bones: ${translating.length}`);
  }
}

mkdirSync(OUT_DIR, { recursive: true });
const bin = Buffer.concat(chunks);
writeFileSync(join(OUT_DIR, "rig.bin"), bin);

const header = {
  fps: FPS,
  bones: skeleton.bones.map((b) => ({ name: b.name, parent: b.parent, t: b.t, r: b.r, s: b.s })),
  clips: clipsOut,
};
writeFileSync(join(OUT_DIR, "rig.json"), JSON.stringify(header));

console.log(`\nbones kept: ${skeleton.bones.length} (fingers dropped)`);
console.log(`clips: ${clipsOut.length}`);
console.log(`rig.bin: ${(bin.length / 1024).toFixed(0)} KB`);
console.log(`rig.json: ${(JSON.stringify(header).length / 1024).toFixed(1)} KB`);
console.log(`\nbones with constant translation (folded into rest pose): ${droppedTranslations.size}`);
