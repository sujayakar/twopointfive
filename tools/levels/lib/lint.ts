// Level LINT: the design checks a file's own faults (rust/game_start faults.rs) do not make — budgets, overlaps, things inside other things. Pure (no GPU, no DOM): tools/qa/level-lint.ts is the CLI over it
// (`bun run tools/qa/level-lint.ts levels/office.json`, exit 1 on errors).
//
// What stands on the floor and can be walked to is asked of the game itself (rust/game_start levels: stands, doors). It loads the level and reports
// findings as { severity, code, message, ids } where ids are the DATA ids (boxes[i] ⇄ data.boxes[i], lights / doors / routes likewise), so a
// finding names the thing in the file. Severity: 'error' = the game will misbehave (a light inside a wall),
// 'warn' = probably unintended (duplicate slabs, budgets getting tight),
// 'info' = worth knowing. `allow` silences a code for listed ids (or '*') with a reason — for legitimately odd authored things.
import type { Vec3 } from '../../../src/math/vec';
import { LevelData, levelFaults, DEVICE_DEFAULTS, floorUsers } from './levelData';
import { loadLevel } from './levelLoad';
import { Box, BoxFlag, rayBox } from '../../../src/scene/boxes';
import { WORLD } from '../../../src/scene/world';
import type { Level } from './level';
import { MarkingDef, MARK_TABLE_ROWS, MARK_SLOT_MAX, MAX_MARKS_PER_BOX, markRecordRows } from '../../../src/scene/markings';

export type Severity = 'error' | 'warn' | 'info';
export interface Finding { severity: Severity; code: string; message: string; ids: string[] }
export interface LintOpts {
  /** code → ids exempted ('*' = the whole code), each with the reason it is fine */
  allow?: Record<string, { ids: string[] | '*'; reason: string }>;
  /** budgets (defaults: boxes 1500 warn / 3500 error; enabled analytic lights 24 warn; all enabled GPU lights 52 warn / 64 error) */
  boxWarn?: number; boxError?: number; lightWarn?: number; slotWarn?: number; slotError?: number;
}
export interface LintReport { findings: Finding[]; errors: number; warnings: number; infos: number; ms: number; stats: Record<string, number | string> }

/** point strictly inside an (optionally yawed) box, with a margin shrinking the box on every side */
function insideBox(p: Vec3, b: Box, margin = 0): boolean {
  if (b.rot) return false;   // (the few tilted things — a bike frame — are never containers)
  const cs = Math.cos(b.yaw), sn = Math.sin(b.yaw); const dx = p[0] - b.c[0], dz = p[2] - b.c[2];
  const lx = cs * dx - sn * dz, lz = sn * dx + cs * dz, ly = p[1] - b.c[1];
  return Math.abs(lx) < b.h[0] - margin && Math.abs(ly) < b.h[1] - margin && Math.abs(lz) < b.h[2] - margin;
}
/** XZ footprint containment (yawed rectangle), any height */
/** the first solid along a ray, as a look or a beam meets one */
function firstHit(boxes: readonly Box[], from: Vec3, dir: Vec3, max: number): { t: number } | null { let t = max, hit = false; for (const b of boxes) { if (b.flags & BoxFlag.NoShadow) continue; const h = rayBox(from, dir, b, t); if (h && h.t < t) { t = h.t; hit = true; } } return hit ? { t } : null; }
const fmt = (p: readonly number[]) => `(${p.map(v => (+v.toFixed(2))).join(', ')})`;

/** How many analytic area lights rust/game_rules/src/game will derive from the switchables (its rule: thin ≤ 6 cm, face ≥ 0.01 m², not the rack LEDs / beacon domes). */
export function derivedAreaLightCount(level: Level): { total: number; lit: number } {
  let total = 0, lit = 0;
  for (const sw of level.switchables) {
    if (sw.group === 'server_leds' || sw.group === 'emergency') continue;
    for (const bi of sw.boxes) {
      const h = level.boxes[bi].h; const thin = h[0] <= h[1] && h[0] <= h[2] ? 0 : h[1] <= h[2] ? 1 : 2; if (h[thin] > 0.06) continue;
      const area = 4 * h[(thin + 1) % 3] * h[(thin + 2) % 3]; if (area < 0.01) continue;
      total++; if (sw.on) lit++;
    }
  }
  return { total, lit };
}

export function lintLevel(data: LevelData, opts: LintOpts = {}): LintReport {
  const t0 = typeof performance !== 'undefined' ? performance.now() : Date.now();
  const findings: Finding[] = [];
  const allowed = (code: string, ids: string[]) => { const a = opts.allow?.[code]; return !!a && (a.ids === '*' || (ids.length > 0 && ids.every(id => (a.ids as string[]).includes(id)))); };
  const add = (severity: Severity, code: string, message: string, ids: string[] = []) => { if (!allowed(code, ids)) findings.push({ severity, code, message, ids }); };
  const done = (stats: Record<string, number | string>): LintReport => {
    const t1 = typeof performance !== 'undefined' ? performance.now() : Date.now();
    const order: Record<Severity, number> = { error: 0, warn: 1, info: 2 }; findings.sort((a, b) => order[a.severity] - order[b.severity]);
    return { findings, errors: findings.filter(f => f.severity === 'error').length, warnings: findings.filter(f => f.severity === 'warn').length, infos: findings.filter(f => f.severity === 'info').length, ms: t1 - t0, stats };
  };

  // 0. the schema first: lint assumes a valid document (every ref resolves) — schema errors are reported and nothing else runs
  const verrs = levelFaults(data);
  if (verrs.length) { for (const e of verrs.slice(0, 50)) add('error', 'schema', e, idsInMessage(e, data)); return done({ schema: verrs.length }); }
  let level: Level;
  try { level = loadLevel(data, { validate: false }); } catch (e) { add('error', 'load', String((e as Error).message ?? e)); return done({}); }
  const boxes = level.boxes; const boxId = (i: number) => data.boxes[i]?.id ?? `box#${i}`;

  // 1. boxes: extents, duplicates, bounds
  { const seen = new Map<string, number>();
    const B = data.meta.bounds ?? { min: WORLD.min, max: WORLD.max };
    boxes.forEach((b, i) => {
      for (let a = 0; a < 3; a++) if (!(b.h[a] > 0)) { add('error', 'box-extent', `${boxId(i)}: zero extent on ${'xyz'[a]} (h = ${fmt(b.h)}) — invisible to the raster and a degenerate slab for rays`, [boxId(i)]); break; }
      const key = [b.c[0], b.c[1], b.c[2], b.h[0], b.h[1], b.h[2], b.yaw, ...(b.rot ?? [])].map(v => Math.round(v * 1000)).join(',');   // (rot included: the bike's wheels are a square and its 45° diamond on one centre)
      const j = seen.get(key); if (j !== undefined) add('warn', 'box-duplicate', `${boxId(i)} duplicates ${boxId(j)} (same centre / size / yaw within 1 mm) — coplanar faces z-fight and the copy costs rays`, [boxId(i), boxId(j)]); else seen.set(key, i);
      const cs = Math.abs(Math.cos(b.yaw)), sn = Math.abs(Math.sin(b.yaw)); const ex = cs * b.h[0] + sn * b.h[2], ez = sn * b.h[0] + cs * b.h[2];
      if (b.c[0] - ex < B.min[0] - 0.01 || b.c[0] + ex > B.max[0] + 0.01 || b.c[2] - ez < B.min[2] - 0.01 || b.c[2] + ez > B.max[2] + 0.01 || b.c[1] - b.h[1] < B.min[1] - 0.01 || b.c[1] + b.h[1] > B.max[1] + 0.01)
        add('warn', 'box-bounds', `${boxId(i)} reaches outside the traceable world ${fmt(B.min)}‥${fmt(B.max)} — rays miss whatever pokes out`, [boxId(i)]);
    }); }

  // 2. lights: inside solids, ranges, intensities, budgets
  { const solids = boxes.map((b, i) => ({ b, i })).filter(({ b }) => !(b.flags & BoxFlag.NoShadow));
    data.lights.forEach((L, i) => {
      const l = level.lights[i];
      if (l.kind !== 'dir') {
        const inside = solids.find(({ b, i: bi }) => bi !== l.fixtureBox && insideBox(l.pos, b, 0.002));
        if (inside) add('error', 'light-in-solid', `light '${L.id}' at ${fmt(l.pos)} is inside ${boxId(inside.i)} — every shadow ray from it starts in the dark`, [`light:${L.id}`, boxId(inside.i)]);
        if (!(l.range > 0)) add('error', 'light-range', `light '${L.id}': range ${l.range} — it can light nothing`, [`light:${L.id}`]);
        else if (l.range < 1) add('warn', 'light-range', `light '${L.id}': range ${l.range} m is very short`, [`light:${L.id}`]);
        if (!(l.intensity > 0)) add('warn', 'light-intensity', `light '${L.id}': intensity ${l.intensity} — never uploaded (dark)`, [`light:${L.id}`]);
        else if (l.intensity > 1000) add('warn', 'light-intensity', `light '${L.id}': intensity ${l.intensity} is far beyond anything on the floor (street lamps are 260) — exposure will fight it`, [`light:${L.id}`]);
        if (l.kind === 'spot' && !(l.outerDeg > 0)) add('warn', 'light-cone', `spot '${L.id}': outer cone ${l.outerDeg}° — a spot with no cone lights nothing`, [`light:${L.id}`]);
        if (l.pos[1] < 0 || l.pos[1] > (data.floors[0]?.ceiling ?? WORLD.ceilingY) + 3.5) add('warn', 'light-height', `light '${L.id}' at y ${l.pos[1].toFixed(2)} — under the floor or far above the roof`, [`light:${L.id}`]);
      } else if (!(l.intensity > 0)) add('warn', 'light-intensity', `directional '${L.id}': intensity ${l.intensity}`, [`light:${L.id}`]);
    });
    const analyticOn = level.lights.filter(l => l.enabled && l.kind !== 'dir').length;
    const area = derivedAreaLightCount(level);
    const cones = data.devices.filter(v => v.kind === 'camera').length;   // each live camera throws a haze cone (rust/game_rules/src/devices, tune.deviceCones): a slot apiece
    const slots = level.lights.filter(l => l.enabled).length + area.lit + cones + /* torches, rail light, a flash or two */ 6;
    const lightWarn = opts.lightWarn ?? 24, slotWarn = opts.slotWarn ?? 52, slotError = opts.slotError ?? 64;
    if (analyticOn > lightWarn) add('warn', 'light-budget', `${analyticOn} enabled analytic point / spot lights (budget ${lightWarn}) — every pixel scores every light in reach`, []);
    if (slots > slotError) add('error', 'light-slots', `${slots} GPU light slots wanted at boot (${level.lights.filter(l => l.enabled).length} analytic + ${area.lit} lit fixture area lights + ${cones} camera cones + ~6 hand lights) — the renderer uploads at most ${slotError}; the rest go dark`, []);
    else if (slots > slotWarn) add('warn', 'light-slots', `${slots} of ${slotError} GPU light slots in use at boot (${area.lit} of them fixture area lights, ${cones} camera cones) — little headroom for flashes / beacons`, []);
    // a switchable lights its boxes through baseEmissive OR through their EMISSIVE MARKINGS (scene/markings.ts: the vending fronts, rack / printer LEDs, the microwave
    // clock carry their radiance in a marking's palette and the switch drives Box.markGain — the box's own emissive is 0 by construction), so 'dark' = neither
    const boxIx = new Map(data.boxes.map((b, i) => [b.id, i]));
    const marksEmit = (id: string) => (data.boxes[boxIx.get(id) ?? -1]?.markings ?? []).some(m => m.pal.some((p, k) => !!p.emit && p.emit[0] + p.emit[1] + p.emit[2] > 0 && m.cells.some(r => r.includes('abc'[k]))));
    data.switchables.forEach(s => { if (!(s.baseEmissive[0] + s.baseEmissive[1] + s.baseEmissive[2] > 0) && !s.boxes.some(marksEmit)) add('warn', 'switchable-dark', `switchable '${s.id}': baseEmissive is black and none of its boxes carries an emissive marking — switching it on shows nothing`, [`sw:${s.id}`]); });
  }

  // 3. box budget
  { const n = boxes.length + level.props.reduce((s, p) => s + p.parts.length, 0);
    const warnAt = opts.boxWarn ?? 1500, errAt = opts.boxError ?? 3500;
    if (n > errAt) add('error', 'box-budget', `${n} boxes (statics + prop parts) — the GPU list holds 4096 with the dynamics still to come`, []);
    else if (n > warnAt) add('warn', 'box-budget', `${n} boxes (statics + prop parts), budget ${warnAt} — the trace grid's busiest cells get slow long before the hard cap`, []); }
  // 3b. the marking table: BoxWorld registers each DISTINCT list (content + albedo + size, boxes.ts markSlotOf) once, append-only, into MARK_TABLE_ROWS rows; a list
  //     that does not fit gets slot 0 = silently not drawn. The level's own lists must leave the game headroom (door leaves, props' parts register theirs at run time too).
  let markRows = 0;
  { const keys = new Set<string>(); let rows = 0, lists = 0; const heavy: { id: string; rows: number }[] = [];
    const take = (id: string, marks: readonly MarkingDef[] | undefined, albedo: readonly number[], h: readonly number[]) => {
      if (!marks?.length) return; const key = `${JSON.stringify(marks)}|${albedo.join(',')}|${h.join(',')}`; if (keys.has(key)) return; keys.add(key);
      let r = 0; for (const m of marks.slice(0, MAX_MARKS_PER_BOX)) r += markRecordRows(m); rows += r; lists++; heavy.push({ id, rows: r });
    };
    boxes.forEach((b, i) => take(boxId(i), b.marks, b.albedo, b.h));
    level.props.forEach((p, i) => p.parts.forEach(q => take(`prop:${data.props[i]?.id ?? i}`, q.marks, q.albedo, q.h)));
    markRows = rows; heavy.sort((a, b) => b.rows - a.rows); const top = heavy.slice(0, 4).map(x => `${x.id} ${x.rows}`).join(' · ');
    if (rows > MARK_TABLE_ROWS || lists + 1 > MARK_SLOT_MAX) add('error', 'mark-rows', `the level's ${lists} distinct marking lists need ${rows} table rows — more than the ${MARK_TABLE_ROWS} the renderer holds (MARK_TABLE_ROWS): the lists registered last are silently NOT DRAWN (biggest: ${top})`, []);
    else if (rows > MARK_TABLE_ROWS * 0.75) add('warn', 'mark-rows', `the level's ${lists} distinct marking lists take ${rows} of ${MARK_TABLE_ROWS} marking-table rows (${Math.round(100 * rows / MARK_TABLE_ROWS)} %) — little headroom for what registers at run time; past the cap lists silently stop drawing (biggest: ${top})`, []); }

  // 8. devices: cameras and beam ends inside solids / outside the world, a beam through a wall, a keypad away from its door or on the wrong face, a keypad whose
  //    code nobody can give up (every questioned man — and the sticky-note pair — means the pad nearest HIM: a pad nearest to no patrol point is unobtainable but by luck)
  { const solids = boxes.map((b, i) => ({ b, i })).filter(({ b }) => !(b.flags & BoxFlag.NoShadow));
    const B = data.meta.bounds ?? { min: WORLD.min, max: WORLD.max };
    const inWorld = (p: readonly number[]) => p[0] >= B.min[0] && p[0] <= B.max[0] && p[2] >= B.min[2] && p[2] <= B.max[2] && p[1] >= B.min[1] && p[1] <= B.max[1] + 0.5;
    const solidAt = (p: Vec3) => solids.find(({ b }) => insideBox(p, b, 0.002));
    const pads = data.devices.filter(v => v.kind === 'keypad');
    const doorMid = (id: string): Vec3 | null => { const d = data.doors.find(x => x.id === id); return d ? [d.hinge[0] + Math.cos(d.closedDir) * d.width / 2, 0, d.hinge[1] + Math.sin(d.closedDir) * d.width / 2] : null; };
    const nearestPad = (p: readonly number[]) => { let best: string | null = null, bd = Infinity; for (const k of pads) { const m = doorMid(k.door); if (!m) continue; const dd = Math.hypot(m[0] - p[0], m[2] - p[2]); if (dd < bd) { bd = dd; best = k.id; } } return best; };
    for (const v of data.devices) {
      const ref = `device:${v.id}`;
      if (v.kind === 'camera') {
        const hit = solidAt(v.pos as Vec3); if (hit) add('error', 'camera-in-solid', `camera '${v.id}' at ${fmt(v.pos)} is inside ${boxId(hit.i)} — its picture is the inside of a wall`, [ref, boxId(hit.i)]);
        if (!inWorld(v.pos)) add('warn', 'device-bounds', `camera '${v.id}' at ${fmt(v.pos)} is outside the traceable world`, [ref]);
        if ((v.range ?? DEVICE_DEFAULTS.range) < 2) add('warn', 'camera-range', `camera '${v.id}': range ${v.range} m — it sees the wall it hangs on`, [ref]);
        if (v.sweep && Math.abs(v.sweep[1] - v.sweep[0]) < 1e-3) add('info', 'camera-sweep', `camera '${v.id}': sweep from = to — it pans nowhere (drop the sweep for a fixed camera)`, [ref]);
      } else if (v.kind === 'laser') {
        for (const [end, p] of [['emitter', v.from], ['receiver', v.to]] as const) { const hit = solidAt(p as Vec3); if (hit) add('error', 'laser-in-solid', `laser '${v.id}': its ${end} ${fmt(p)} is inside ${boxId(hit.i)} — mount it on the wall's face, not in it`, [ref, boxId(hit.i)]); if (!inWorld(p)) add('warn', 'device-bounds', `laser '${v.id}': ${end} ${fmt(p)} outside the traceable world`, [ref]); }
        // the beam through solid geometry between its ends (a partition in the way = a beam nobody can ever break past the wall)
        { const a = v.from as Vec3, b = v.to as Vec3; const L = Math.hypot(b[0] - a[0], b[1] - a[1], b[2] - a[2]); if (L > 0.2) { const dir: Vec3 = [(b[0] - a[0]) / L, (b[1] - a[1]) / L, (b[2] - a[2]) / L]; const st = firstHit(boxes, [a[0] + dir[0] * 0.06, a[1] + dir[1] * 0.06, a[2] + dir[2] * 0.06], dir, L - 0.12); if (st) add('warn', 'laser-blocked', `laser '${v.id}': solid geometry ${fmt([a[0] + dir[0] * (st.t + 0.06), a[1] + dir[1] * (st.t + 0.06), a[2] + dir[2] * (st.t + 0.06)])} stands in the beam ${st.t.toFixed(2)} m from the emitter — it will always draw cut short there and nothing beyond it can break it`, [ref]); } }
        if (v.from[1] > 0.55 && v.from[1] < 1.18) add('info', 'laser-height', `laser '${v.id}' at y ${v.from[1].toFixed(2)}: too high to be a shin beam, too low to crouch under (the crouch-walk\'s crown rides at ~1.12) — everybody breaks it, standing or not; 0.35 or ≥ 1.2 say what they mean`, [ref]);
      } else {
        const m = doorMid(v.door); const d = data.doors.find(x => x.id === v.door);
        { const hit = solidAt(v.pos as Vec3); if (hit) add('warn', 'keypad-in-solid', `keypad '${v.id}' at ${fmt(v.pos)} is inside ${boxId(hit.i)} — the plate is buried in the wall (mount it 1–2 cm proud of the face: the sheet's 'snap beside the latch jamb' does)`, [ref, boxId(hit.i)]); }
        if (m && d) {
          const dist = Math.hypot(m[0] - v.pos[0], m[2] - v.pos[2]);
          if (dist > 2.5) add('warn', 'keypad-far', `keypad '${v.id}' is ${dist.toFixed(1)} m from its door '${v.door}' — nobody standing at the door can be at the pad`, [ref, `door:${v.door}`]);
          const perp = Math.cos(d.closedDir) * (v.pos[2] - d.hinge[1]) - Math.sin(d.closedDir) * (v.pos[0] - d.hinge[0]); const side = perp >= 0 ? 1 : -1;   // (rust/game_rules/src/doors side())
          if (side !== (d.keySide ?? 1)) add('warn', 'keypad-side', `keypad '${v.id}' hangs on the ${side > 0 ? '+' : '−'} face of door '${v.door}' but the lock bites on the ${(d.keySide ?? 1) > 0 ? '+' : '−'} face (keySide) — the pad is on the side that always opens`, [ref, `door:${v.door}`]);
          if (v.pos[1] < 0.9 || v.pos[1] > 1.8) add('info', 'keypad-height', `keypad '${v.id}' at y ${v.pos[1].toFixed(2)} — pads sit at hand height (1.4)`, [ref]);
        }
        const reachable = data.routes.some(r => r.points.some(q => nearestPad(q) === v.id));
        if (!reachable) add('warn', 'keypad-orphan', `keypad '${v.id}': no patrol route has a point whose nearest pad is this one — no man will ever give ITS code up (nor the pair say it); the player can only kick '${v.door}'${data.routes.length ? '' : ' (the level has no routes at all)'}`, [ref]);
      }
    } }
  // 9b. floors: what stands on each (levelData.ts floorUsers); a floor nobody names is worth a line (an orphan of a deleted wing)
  const perFloor = data.floors.map(f => `${f.id} ${floorUsers(data, f.id).length}`).join(' · ');
  data.floors.forEach((f, i) => { if (i > 0 && floorUsers(data, f.id).length === 0) add('info', 'floor-empty', `floor '${f.id}'${f.name ? ` (${f.name})` : ''} at y ${f.y}: nothing is on it — delete it, or it is a wing still to be built`, [`floor:${f.id}`]); });
  // 9. mission marks against their rooms
  { const M = data.mission; for (const o of M.objectives) if (o.kind === 'reach') { const d = data.doors.find(x => x.id === o.door); if (d) { const ux = Math.cos(d.closedDir), uz = Math.sin(d.closedDir); const mx = d.hinge[0] + ux * d.width / 2, mz = d.hinge[1] + uz * d.width / 2; const A = o.area; const dist = Math.max(A.x0 - mx, mx - A.x1, A.z0 - mz, mz - A.z1); if (dist > 1.5) add('warn', 'objective-door', `objective '${o.id}': its door '${o.door}' is ${dist.toFixed(1)} m outside the area's edge — the HUD arrow will point away from the room`, [`mark:${o.id}`]); } } }
  return done({ boxes: boxes.length, props: level.props.length, lights: level.lights.length, areaLights: derivedAreaLightCount(level).lit, doors: data.doors.length, routes: data.routes.length, devices: data.devices.length, markRows, floors: perFloor });
}

/** ids quoted in a fault ('box_0012', "light 'desk_lamp'") → selectable refs, best effort */
export function idsInMessage(msg: string, data: LevelData): string[] {
  const out: string[] = []; const m = /^(\w+)\[(\d+)\]/.exec(msg);
  if (m) { const arr = (data as any)[m[1]]; const e = Array.isArray(arr) ? arr[Number(m[2])] : null; if (e?.id) out.push(m[1] === 'boxes' ? e.id : `${m[1].replace(/s$/, '')}:${e.id}`); }
  return out;
}

/** One-line summary + the findings as printable lines (CLI / the editor's log). */
export function formatLint(r: LintReport): string[] {
  const lines = [`lint: ${r.errors} error(s), ${r.warnings} warning(s), ${r.infos} info — ${Object.entries(r.stats).map(([k, v]) => `${k} ${v}`).join(' · ')} (${r.ms.toFixed(0)} ms)`];
  for (const f of r.findings) lines.push(`  ${f.severity.toUpperCase().padEnd(5)} ${f.code.padEnd(18)} ${f.message}${f.ids.length ? `  [${f.ids.slice(0, 6).join(' ')}${f.ids.length > 6 ? ' …' : ''}]` : ''}`);
  return lines;
}
