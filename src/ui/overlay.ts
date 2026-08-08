// DOM overlay: guard state markers, the F reticle + interaction panel, light / sound meters, kit column, message log, mission line, tour caption / timeline.
import { v3 } from '../math/vec';
import { FollowCamera } from '../game/camera';
import { Game, Player, Interactable } from '../game/game';

const EDGE = 1.1;   // markers / dots / the throw arc stay drawn until they are 10 % of the view past its edge (FollowCamera.project margin)
const NEARBY_M = 3.0;   // interaction panel: things this close (planar, metres) are listed even when not under the cursor / not yet in reach
const MAX_ROWS = 4;     // …at most this many blocks, then '+N more'

/** One block of the interaction panel: what the thing is (`label`, the marker's first line), the key → action pairs it offers right now (`opts`; key '' = a
 *  status / hint line such as 'step closer' or 'picking… 40%'), whether it is within reach (else greyed, with `dist` in metres when known), `off` (a dead
 *  fixture), and `progress` 0‥1 for a bar (a lock being picked, the drive coming out). */
export interface InteractRow { kind: Interactable['kind']; label: string; opts: { key: string; text: string }[]; inReach: boolean; off: boolean; dist?: number; progress?: number; }

/** The marker text is authored as 'key␠␠action  ·  key␠␠action  ·  hint' (player.ts buildInteractables): split it back into rows. 'F or sprint into it  kick…'
 *  keeps F as the key and puts the alternative on a hint line under it. (Until Interactable carries structured options this is the one place that knows the convention.) */
function parseOpts(line2: string): { key: string; text: string }[] {
  const out: { key: string; text: string }[] = [];
  for (const raw of line2.split(/\s+·\s+/)) {
    const seg = raw.trim(); if (!seg) continue;
    const m = /^(.*?\S)\s{2,}(\S.*)$/.exec(seg);
    if (!m) { out.push({ key: '', text: seg }); continue; }
    const or = /^(\S+) or (.+)$/.exec(m[1]);
    out.push({ key: or ? or[1] : m[1], text: m[2].trim() });
    if (or) out.push({ key: '', text: `or ${or[2]}` });
  }
  return out;
}

export class Overlay {
  private root = document.getElementById('overlay')!;
  private meters = document.getElementById('meters')!;
  private lightFill = document.querySelector('#meters .light .fill') as HTMLElement; private lightNum = document.querySelector('#meters .light .num') as HTMLElement;
  private soundFill = document.querySelector('#meters .sound .fill') as HTMLElement; private soundNum = document.querySelector('#meters .sound .num') as HTMLElement; private soundPeak = document.querySelector('#meters .sound .peak') as HTMLElement;
  private peak = 0; private peakT = 0;
  private log = document.getElementById('log')!;
  private captionEl = document.getElementById('caption')!; private captionKey = '';
  private tlEl = document.getElementById('timeline')!; private tlKey = ''; private tlFill: HTMLElement | null = null; private tlOnPick: ((k: number) => void) | null = null;
  private equip = document.getElementById('equip')!;
  private traj = document.getElementById('traj') as unknown as SVGSVGElement;
  private trajPath: SVGPolylineElement; private trajLand: SVGCircleElement;
  private equipKey = '';
  private markers: HTMLElement[] = [];
  private dots: HTMLElement[] = [];
  showLightDots = true;
  /** Tour timeline: one segment per stop (width ∝ duration) with its chapter title, the live one filled to its progress; segments are clickable. */
  setTimeline(ch: { list: { title: string; dur: number }[]; i: number; frac: number } | null, onPick: (k: number) => void) {
    this.tlOnPick = onPick;
    if (!ch) { if (this.tlKey) { this.tlKey = ''; this.tlEl.className = ''; this.tlEl.innerHTML = ''; this.tlFill = null; } return; }
    const key = ch.list.length + '|' + ch.i;
    if (key !== this.tlKey) {
      this.tlKey = key; const total = ch.list.reduce((s, c) => s + c.dur, 0) || 1;
      this.tlEl.innerHTML = ch.list.map((c, k) => `<div class="seg${k < ch.i ? ' done' : k === ch.i ? ' cur' : ''}" data-k="${k}" title="${k + 1}. ${c.title} — click or ←/→" style="flex:${(c.dur / total * 100).toFixed(2)} 1 0"><b></b><span>${c.title}</span></div>`).join('');
      this.tlEl.className = 'on'; this.tlFill = this.tlEl.querySelector('.seg.cur b');
      if (!this.tlEl.dataset.wired) { this.tlEl.dataset.wired = '1'; this.tlEl.addEventListener('click', ev => { const seg = (ev.target as HTMLElement).closest('.seg') as HTMLElement | null; if (seg && this.tlOnPick) this.tlOnPick(Number(seg.dataset.k)); }); }
    }
    if (this.tlFill) this.tlFill.style.width = `${(ch.frac * 100).toFixed(1)}%`;
  }
  setCaption(text: string, sub: string) {
    const key = text + '|' + sub; if (key === this.captionKey) return; this.captionKey = key;
    this.captionEl.className = text ? 'on' : '';
    if (text) this.captionEl.innerHTML = `<div class="cap">${text}</div>${sub ? `<div class="sub">${sub}</div>` : ''}`;
  }
  private reticle: HTMLElement; private ring: HTMLElement; private ringKey = -1;   // the ring on the marker F acts on; ring: progress round it (Interactable.progress — a lock being picked, the drive coming out)
  private iaEl = document.getElementById('interact')!; private iaKey = '';         // interaction panel (top-left): rebuilt when its rows change, distance / progress patched in place
  private iaLive: { em: HTMLElement; bar: HTMLElement | null; d: string; p: string }[] = [];
  private menuEl = document.getElementById('menu')!;                               // start / pause / debrief card up ('on'): no interaction panel behind it
  private objEl = document.getElementById('objective')!; private objKey = '\0';   // the mission's one line (top-centre)
  private alarmEl = document.getElementById('alarm')!; private alarmKey = '';       // alarm stage under it (hidden at calm)
  private objMark: HTMLElement;                                                    // diamond + distance on / toward the current objective
  constructor() {
    this.reticle = document.createElement('div'); this.reticle.className = 'reticle'; this.reticle.innerHTML = '<b></b>'; this.reticle.style.display = 'none'; this.root.appendChild(this.reticle);
    this.ring = this.reticle.firstChild as HTMLElement;
    this.objMark = document.createElement('div'); this.objMark.className = 'objmark'; this.objMark.innerHTML = '<i></i><span></span>'; this.objMark.style.display = 'none'; this.root.appendChild(this.objMark);
    const NS = 'http://www.w3.org/2000/svg';
    this.trajPath = document.createElementNS(NS, 'polyline') as SVGPolylineElement;
    this.trajPath.setAttribute('fill', 'none'); this.trajPath.setAttribute('stroke', 'rgba(150,240,220,0.85)'); this.trajPath.setAttribute('stroke-width', '2'); this.trajPath.setAttribute('stroke-dasharray', '5 6'); this.trajPath.setAttribute('stroke-linecap', 'round');
    this.trajLand = document.createElementNS(NS, 'circle') as SVGCircleElement;
    this.trajLand.setAttribute('r', '9'); this.trajLand.setAttribute('fill', 'rgba(150,240,220,0.12)'); this.trajLand.setAttribute('stroke', 'rgba(150,240,220,0.9)'); this.trajLand.setAttribute('stroke-width', '1.5');
    this.traj.appendChild(this.trajPath); this.traj.appendChild(this.trajLand);
    this.trajPath.style.display = 'none'; this.trajLand.style.display = 'none';
  }
  update(game: Game, cam: FollowCamera, canvas: HTMLCanvasElement) {
    const w = canvas.clientWidth, h = canvas.clientHeight;
    // guards
    while (this.markers.length < game.guards.length) { const el = document.createElement('div'); el.className = 'marker'; this.root.appendChild(el); this.markers.push(el); }
    game.guards.forEach((g, i) => {
      const el = this.markers[i];
      const head = g.char.bones.head ?? g.char.pos; const [x, y, vis] = cam.project([head[0], head[1] + 0.45, head[2]], w, h, EDGE);
      if (!vis) { el.style.display = 'none'; return; }
      el.style.display = 'block'; el.style.left = `${x}px`; el.style.top = `${y}px`;
      let text = '', cls = 'calm';
      if (g.state === 'dead') { text = '✕'; cls = 'dead'; }
      else if (g.state === 'alert') { text = '!'; cls = 'alert'; }
      else if (g.state === 'suspicious' || g.state === 'search') { text = '?'; cls = 'sus'; }
      else if (g.awareness > 0.05) { text = '·'.repeat(1 + Math.floor(g.awareness * 6)); cls = 'sus'; }
      if (g.state !== 'dead' && game.time < g.dazzledUntil) text = '✹' + text;   // dazzled by a stun canister
      if (game.showDebug) text += ` ${g.state} ${(g.awareness * 100) | 0}%${game.time < g.dazzledUntil ? ` dazzled ${(g.dazzledUntil - game.time).toFixed(1)}s` : ''}`;
      el.textContent = text; el.className = 'marker ' + cls;
    });
    // a faint dot on everything F can act on (lights, doors, bodies — so hidden ceiling panels and closed doors are discoverable)
    const its = game.interactables;
    while (this.dots.length < its.length) { const el = document.createElement('div'); el.style.cssText = 'position:absolute;width:5px;height:5px;border-radius:50%;transform:translate(-50%,-50%);'; this.root.appendChild(el); this.dots.push(el); }
    this.dots.forEach((el, i) => {
      const it = its[i];
      if (!it || !this.showLightDots || it === game.hover) { el.style.display = 'none'; return; }
      const [x, y, vis] = cam.project(it.pos, w, h, EDGE);
      if (!vis) { el.style.display = 'none'; return; }
      el.style.display = 'block'; el.style.left = `${x}px`; el.style.top = `${y}px`;
      if (it.kind === 'guard') { el.style.display = 'none'; return; }   // living guards already carry a state marker; no extra dot
      el.style.background = it.kind === 'door' ? 'rgba(150,190,230,0.5)' : it.kind === 'pistol' ? 'rgba(200,200,150,0.6)' : it.kind === 'body' ? 'rgba(230,140,120,0.6)' : it.kind === 'objective' ? 'rgba(143,240,180,0.75)' : it.off ? 'rgba(120,140,160,0.45)' : 'rgba(255,240,180,0.55)';
    });
    // the ring on the hovered marker (what it offers is listed in the interaction panel, below — the ring only says WHICH thing F means)
    const hv = game.hover;
    if (hv) {
      const [x, y, vis] = cam.project(hv.pos, w, h, EDGE);
      this.reticle.style.display = vis ? 'block' : 'none'; this.reticle.style.left = `${x}px`; this.reticle.style.top = `${y}px`;
      this.reticle.className = 'reticle' + (hv.off ? ' off' : '') + (hv.inReach ? '' : ' far') + ` k-${hv.kind}`;
      const pr = hv.progress !== undefined && hv.progress > 0 ? Math.round(Math.min(1, hv.progress) * 200) : -1;   // half-percent steps: no style churn between them
      if (pr !== this.ringKey) { this.ringKey = pr; this.ring.style.display = pr >= 0 ? 'block' : 'none'; if (pr >= 0) this.ring.style.setProperty('--p', `${pr / 2}%`); }
    } else if (game.aimSnap && game.player.slot === 1 && !game.player.holstered && !game.player.down) {   // nothing to interact with near him, pistol out: the ring shows what the auto-aim snapped to (a light: right click OCPs it, click shoots it; a man's chest)
      const [x, y, vis] = cam.project(game.aimSnap, w, h, EDGE);
      this.reticle.style.display = vis ? 'block' : 'none'; this.reticle.style.left = `${x}px`; this.reticle.style.top = `${y}px`;
      this.reticle.className = 'reticle aim' + (game.aimGuard ? ' k-guard' : ' k-light');
      if (this.ringKey !== -1) { this.ringKey = -1; this.ring.style.display = 'none'; }
    } else this.reticle.style.display = 'none';
    // interaction panel (top-left): the hovered thing first and lit — it is what F acts on — then whatever else is within reach or a few steps off
    { const ia = this.deriveInteractions(game); this.setInteractions(ia.rows, ia.primary, ia.more); }
    // light + sound meters (bottom-left): how lit you are (the GPU light meter) and how much noise you are making (peak-hold marker)
    const v = game.player.visibility;
    this.lightFill.style.width = `${(v * 100).toFixed(0)}%`; this.lightNum.textContent = game.showDebug ? `${(v * 100) | 0}  E ${game.player.irradiance.toFixed(3)}` : `${(v * 100) | 0}`;
    this.meters.classList.toggle('hot', v > 0.55);
    const n = Math.min(1, game.player.noise);
    this.soundFill.style.width = `${(n * 100).toFixed(0)}%`; this.soundNum.textContent = `${(n * 100) | 0}`;
    if (n >= this.peak) { this.peak = n; this.peakT = game.time; } else if (game.time - this.peakT > 1.2) this.peak = Math.max(n, this.peak - 0.02);
    this.soundPeak.style.left = `calc(${(this.peak * 100).toFixed(0)}% - 1px)`; this.soundPeak.style.opacity = this.peak > 0.02 ? '0.85' : '0';
    // log
    this.log.textContent = game.messages.filter(m => game.time - m.t < 6).map(m => m.text).join('\n');
    // mission: the objective line, and a diamond + distance on the objective — clamped to the screen edge in its direction when it is off screen (the
    // usual case from this camera), faded out within a couple of metres so it never fights the F reticle / door marker that takes over there
    const ot = game.objectiveText();
    if (ot !== this.objKey) { this.objKey = ot; this.objEl.textContent = ot; this.objEl.className = game.mission.stage; }
    { // alarm stage: level, whole seconds left on its clock, and whether an episode is running — rebuilt only when that key changes
      const lvl = game.quietUtility ? 0 : game.escalation; const left = lvl > 0 && game.escalationT > game.time ? Math.ceil(game.escalationT - game.time) : 0;
      const live = lvl > 0 && game.alarm.episode;
      const key = lvl === 0 ? '' : `${lvl}|${left}|${live ? 1 : 0}`;
      if (key !== this.alarmKey) { this.alarmKey = key;
        if (!key) this.alarmEl.className = ''; else { this.alarmEl.className = `on l${lvl}${live ? ' live' : ''}`; this.alarmEl.innerHTML = `<i></i>${lvl === 1 ? 'ALARM I · heightened' : 'ALARM II · lockdown'}${live ? '' : left ? ` · ${left}s` : ''}`; } }
    }
    const op = game.objectivePos();
    const od = op ? v3.distXZ(op, game.player.char.pos) : 0;
    const floor = game.mission.stage === 'drive' && game.hover?.kind !== 'objective' ? 0.35 : 0;   // the rack has no marker of its own: keep a faint diamond on it until the cursor finds it (doors have their own dot / reticle)
    const alpha = op ? Math.max(floor, Math.min(0.9, (od - 1.6) / 1.5)) : 0;
    if (!op || alpha < 0.03) this.objMark.style.display = 'none';
    else {
      const vp = cam.viewProj; const cw = vp[3] * op[0] + vp[7] * op[1] + vp[11] * op[2] + vp[15];
      let nx = vp[0] * op[0] + vp[4] * op[1] + vp[8] * op[2] + vp[12], ny = vp[1] * op[0] + vp[5] * op[1] + vp[9] * op[2] + vp[13];
      if (cw > 1e-4) { nx /= cw; ny /= cw; } else { const l = Math.hypot(nx, ny) || 1; nx = nx / l * 9; ny = ny / l * 9; }   // behind the eye: keep the undivided clip direction (dividing by w < 0 would mirror it) and force it onto the edge
      const mx = 1 - 2 * 36 / w, my = 1 - 2 * 36 / h; const k = Math.max(1, Math.abs(nx) / mx, Math.abs(ny) / my); nx /= k; ny /= k;   // into a 36 px inset of the view
      const el = this.objMark; el.style.display = 'block'; el.style.opacity = alpha.toFixed(2); el.className = k > 1 ? 'objmark edge' : 'objmark';
      el.style.left = `${((nx * 0.5 + 0.5) * w).toFixed(1)}px`; el.style.top = `${((1 - (ny * 0.5 + 0.5)) * h).toFixed(1)}px`;
      (el.lastChild as HTMLElement).textContent = `${Math.round(od)}m`;
    }
    // throw preview
    const pv = game.player.throwPreview;
    if (pv && pv.points.length > 1) {
      const pts: string[] = [];
      for (const p of pv.points) { const [x, y, vis] = cam.project(p, w, h, EDGE); if (vis) pts.push(`${x.toFixed(1)},${y.toFixed(1)}`); }
      this.trajPath.setAttribute('points', pts.join(' ')); this.trajPath.style.display = 'block';
      const [lx, ly, lv] = cam.project(pv.land, w, h, EDGE); this.trajLand.style.display = lv ? 'block' : 'none'; this.trajLand.setAttribute('cx', lx.toFixed(1)); this.trajLand.setAttribute('cy', ly.toFixed(1));
    } else { this.trajPath.style.display = 'none'; this.trajLand.style.display = 'none'; }
    // equipment
    this.updateEquip(game);
  }

  /** Interaction panel: `rows` top to bottom, `primary` = index of the block F acts on right now (-1: none), `more` = how many were left off the end.
   *  The DOM is rebuilt only when the rows' text / reach / kind change; the distance and the progress bar are patched in place every frame. Hidden when
   *  there is nothing to list or a menu card is up. update() feeds it from the game's own interactables (deriveInteractions); an input layer that keeps
   *  its own highlighted option can call it instead. */
  setInteractions(rows: InteractRow[], primary = -1, more = 0) {
    if (!rows.length || this.menuEl.classList.contains('on')) { if (this.iaKey) { this.iaKey = ''; this.iaEl.className = ''; this.iaEl.textContent = ''; this.iaLive = []; } return; }
    const key = rows.map(r => `${r.kind}|${r.label}|${r.opts.map(o => `${o.key}>${o.text}`).join('/')}|${r.inReach ? 1 : 0}${r.off ? 1 : 0}${r.dist !== undefined ? 1 : 0}${r.progress !== undefined ? 1 : 0}`).join('\n') + `#${primary}#${more}`;
    if (key !== this.iaKey) {
      this.iaKey = key;
      this.iaEl.innerHTML = rows.map((r, i) =>
        `<div class="it k-${r.kind}${i === primary ? ' primary' : ''}${r.inReach ? '' : ' far'}${r.off ? ' off' : ''}"><div class="what"><span>${esc(r.label)}</span><em></em></div>`
        + r.opts.map(o => o.key ? `<div class="opt"><kbd>${esc(o.key.toUpperCase())}</kbd>${esc(o.text)}</div>` : `<div class="hint">${esc(o.text)}</div>`).join('')
        + (r.progress !== undefined ? '<div class="prog"><b></b></div>' : '') + '</div>').join('') + (more > 0 ? `<div class="more">+${more} more</div>` : '');
      this.iaEl.className = 'on';
      this.iaLive = rows.map((_, i) => { const el = this.iaEl.children[i] as HTMLElement; return { em: el.querySelector('.what em') as HTMLElement, bar: el.querySelector('.prog b') as HTMLElement | null, d: '', p: '' }; });
    }
    rows.forEach((r, i) => {
      const L = this.iaLive[i];
      const d = r.dist !== undefined ? `${r.dist < 10 ? r.dist.toFixed(1) : Math.round(r.dist)}m` : ''; if (d !== L.d) { L.d = d; L.em.textContent = d; }
      if (L.bar) { const p = `${(Math.round(Math.min(1, Math.max(0, r.progress ?? 0)) * 200) / 2).toFixed(1)}%`; if (p !== L.p) { L.p = p; L.bar.style.width = p; } }   // half-percent steps, like the ring
    });
  }

  /** What the panel lists this frame, from the same data the F key reads: the hovered marker (game.hover — the one F acts on) first, marked primary while it is
   *  in reach, then — in live play — everything else within reach or NEARBY_M of you (in-reach first, then by distance); light fixtures only ever as the hovered
   *  one (they are ranged: every panel on the floor is 'available'). While the tour runs, only the hovered marker, exactly as the reticle text it replaces behaved. */
  private deriveInteractions(game: Game): { rows: InteractRow[]; primary: number; more: number } {
    const hv = game.hover, p = game.player.char.pos;
    const dist = (it: Interactable) => { let d = v3.distXZ(it.pos, p); if (it.door) d = Math.min(d, v3.distXZ(it.door.frameCentre, p)); return d; };   // doors: leaf or doorway centre, as the reach test measures it
    const picked: { it: Interactable; d: number }[] = hv ? [{ it: hv, d: dist(hv) }] : [];
    if (!document.body.classList.contains('touring')) {
      const rest: { it: Interactable; d: number }[] = [];
      for (const it of game.interactables) { if (it === hv || it.kind === 'light') continue; const d = dist(it); if (it.inReach || d < NEARBY_M) rest.push({ it, d }); }
      rest.sort((a, b) => a.it.inReach !== b.it.inReach ? (a.it.inReach ? -1 : 1) : a.d - b.d);
      picked.push(...rest);
    }
    const rows = picked.slice(0, MAX_ROWS).map(({ it, d }): InteractRow => ({ kind: it.kind, label: it.line1, opts: parseOpts(it.line2), inReach: it.inReach, off: it.off, dist: it.inReach || it.kind === 'light' ? undefined : d, progress: it.progress }));
    return { rows, primary: hv && hv.inReach ? 0 : -1, more: Math.max(0, picked.length - MAX_ROWS) };   // lit only while F would actually do it: hovered AND in reach (a hovered door three metres off heads the list greyed, with its distance)
  }

  /** Kit column (bottom-left, standing on the meters): key strip, condition pips, then the three slots top to bottom. Rebuilt only when something it shows changes. */
  private updateEquip(game: Game) {
    const pl = game.player; const gun = pl.pistol;
    const paceShow = game.time - pl.paceShownT < 2.5 || pl.pace < 0.99;   // pace ticks: shown for a moment after a wheel step, and kept while below full pace
    const key = [pl.slot, gun.chamber, gun.mag, gun.spare.join(','), gun.reloading ? 1 : 0, pl.canisters, pl.flashbangs, pl.throwKind, gun.lightOn ? 1 : 0, pl.nv ? 1 : 0, Math.round(pl.ocp.charge * 20), pl.hitsLeft, pl.down ? 1 : 0, game.godMode ? 1 : 0, pl.holstered ? 1 : 0, paceShow ? Math.round(pl.pace * 100) : -1].join('|');
    if (key === this.equipKey) return; this.equipKey = key;
    const el = this.equip; el.textContent = '';
    const mk = (tag: string, cls?: string, text?: string) => { const e = document.createElement(tag); if (cls) e.className = cls; if (text !== undefined) e.textContent = text; return e; };
    // key strip: rail light, night vision, OCP + its charge
    const gad = mk('div', 'gadgets');
    gad.appendChild(mk('span', 'g' + (gun.lightOn ? ' on' : ''), 'L LIGHT'));
    gad.appendChild(mk('span', 'g' + (pl.nv ? ' on' : ''), 'N NIGHT VISION'));
    const oc = mk('span', 'g' + (pl.ocp.ready() ? ' on' : ''), 'RMB OCP'); const bar = mk('span', 'ocpbar'); const bb = mk('b'); bb.style.width = `${Math.round(pl.ocp.charge * 100)}%`; bar.appendChild(bb); oc.appendChild(bar); gad.appendChild(oc);
    el.appendChild(gad);
    // walking pace (mouse wheel): a row of ticks, PACE_MIN‥1 — lit up to the current notch; shown after a wheel step and while below full pace
    if (paceShow) { const pr = mk('div', 'pace'); pr.appendChild(mk('span', 'g', 'PACE')); const ticks = mk('span', 'ticks'); const N = 6; const lit = Math.round((pl.pace - 0.35) / 0.65 * (N - 1)) + 1; for (let i = 0; i < N; i++) ticks.appendChild(mk('i', i < lit ? 'on' : '')); pr.appendChild(ticks); pr.appendChild(mk('span', 'g', 'wheel')); el.appendChild(pr); }
    // condition: MAX_HITS pips (one: a single 9 mm round downs you); god mode shows a note instead (.god — the tour, which runs in god mode, hides that row)
    const cond = mk('div', 'cond' + (pl.down ? ' down' : '') + (game.godMode ? ' god' : ''));
    if (game.godMode) cond.appendChild(mk('span', 'g on', 'GOD MODE'));
    else { cond.appendChild(mk('span', 'g', pl.down ? 'DOWN — ENTER' : 'CONDITION')); const pips = mk('span', 'pips'); for (let i = 0; i < Player.MAX_HITS; i++) pips.appendChild(mk('i', i < pl.hitsLeft ? '' : 'lost')); cond.appendChild(pips); }
    el.appendChild(cond);
    // slot 1: pistol
    const s1 = mk('div', 'slot' + (pl.slot === 1 ? (pl.holstered ? ' sel holstered' : ' sel') : ''));
    s1.appendChild(mk('span', 'key', '1')); s1.appendChild(mk('span', 'name', gun.reloading ? 'Five-seveN · reloading' : 'Five-seveN'));
    const rounds = mk('span', 'rounds');
    const inMag = gun.mag ?? 0;
    for (let i = 0; i < gun.magCapacity; i++) rounds.appendChild(mk('i', i < inMag ? '' : 'spent'));
    if (gun.chamber) rounds.appendChild(mk('i', 'chamber'));
    s1.appendChild(rounds);
    s1.appendChild(mk('span', '', `${inMag}${gun.chamber ? '+1' : '+0'}`));
    const mags = mk('span', 'mags');
    for (const r of gun.spareSorted()) { const m = mk('span', 'mag'); const f = mk('b'); f.style.height = `${Math.round((r / gun.magCapacity) * 100)}%`; m.appendChild(f); m.title = `${r} rounds`; mags.appendChild(m); }
    s1.appendChild(mags);
    el.appendChild(s1);
    // slots 2 / 3: smoke and stun canisters. "G" marks the one the quick-throw key will use (the throwable selected last)
    const gKind = pl.throwKind === 'flash' && pl.flashbangs > 0 ? 'flash' : pl.canisters > 0 ? 'smoke' : pl.flashbangs > 0 ? 'flash' : '';
    const s2 = mk('div', 'slot' + (pl.slot === 2 ? (pl.holstered ? ' sel holstered' : ' sel') : ''));
    s2.appendChild(mk('span', 'key', '2')); s2.appendChild(mk('span', 'name', 'smoke'));
    const cans = mk('span', 'cans'); for (let i = 0; i < pl.canisters; i++) cans.appendChild(mk('i')); s2.appendChild(cans);
    s2.appendChild(mk('span', '', pl.canisters ? `×${pl.canisters}${gKind === 'smoke' ? '  G quick-throw' : ''}` : 'empty'));
    el.appendChild(s2);
    const s3 = mk('div', 'slot' + (pl.slot === 3 ? (pl.holstered ? ' sel holstered' : ' sel') : ''));
    s3.appendChild(mk('span', 'key', '3')); s3.appendChild(mk('span', 'name', 'stun'));
    const bangs = mk('span', 'cans bangs'); for (let i = 0; i < pl.flashbangs; i++) bangs.appendChild(mk('i')); s3.appendChild(bangs);
    s3.appendChild(mk('span', '', pl.flashbangs ? `×${pl.flashbangs}${gKind === 'flash' ? '  G quick-throw' : ''}` : 'empty'));
    el.appendChild(s3);
  }
}

function esc(t: string): string { return t.replace(/&/g, '&amp;').replace(/</g, '&lt;'); }
