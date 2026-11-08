// DOM overlay: guard state markers, the F reticle + interaction panel, light / sound meters, kit column, message log, mission line, tour caption / timeline.
import { railClear, type Box } from './railclear';
import { railCard, type RailCard } from './railcard';
import type { HudLike } from '../host/hud';   // the loop's view of a HUD (src/host/loop.ts stepFrame): this class is the DOM one; a native host's display-list overlay is another
import { v3, clamp, lerp } from '../math/vec';
import { FollowCamera } from '../game/camera';
import { Game, Interactable } from '../game/game';
import type { Character } from '../game/character';
import type { ThrowReachView } from '../view/read';
import { FLOOR } from '../game/floor';
import { RAILS } from '../game/rails';
import { backName } from '../game/backkey';
import { KEYMAP, keyCap, keyText, keysInLine } from '../host/keymap';   // (the player's own keys: every key this HUD names is named as it is bound — the same string back while nothing is rebound)
import { thingDist } from '../game/player';
import { GEAR, gearText } from '../game/gear';
import { HANDRING, EMPTY_WORD, EMPTY_WHY, EMPTY_SHOW_SECS } from '../game/handring';
import { cornerRows, cornerOn } from './corner';
import { emptyThing, emptyWord } from '../game/interact';
/** the reticle's ONE word: the 40 mm's arc is too short to arm (the plain 'RMB  OCP' / 'RMB  RING' / 'RMB  40 MM' words went on the 10-11 play note: "we can remove
 *  the RMB overlay on the cursor" — what the right button holds is read on the kit column's rows) */
const HE40_SHORT = '40 MM · TOO CLOSE TO ARM';
const HE40_SHORT_STROKE = 'rgba(255, 170, 90, 0.95)';
import { CKPT } from '../game/savestate';
import { pageRows } from '../view/pagesaid';
import { fogHides } from '../view/seatview';
import { drawHandBox } from './handbox';
import type { KitView } from '../view/seatview';
import { CmdRingViz } from './cmdring';
import { PlanFloor } from './planfloor';
import { FindMeViz } from './findme';
import { WindViz } from './windviz';
import { SquadThrowViz } from './squadthrowviz';
import { AttentionViz } from './attentionviz';
import { PictureViz } from './pictureviz';
import { menuTheme } from './theme';
import { textZoom } from './textsize';
import { locked } from '../view/seatview';
import { objectiveWords } from './tabcard';
/** the men the big line keeps clear of, found by the ids the view names them by */
const lineMen = (game: Game, ids: readonly number[]) => { const out = game.guards.map(gd => gd.char).filter(c => ids.includes(c.id)); if (ids.includes(game.player.char.id)) out.push(game.player.char); return out; };

const EDGE = 1.1;   // markers / dots / the throw arc stay drawn until they are 10 % of the view past its edge (FollowCamera.project margin)
/** the widest the aiming ring opens, px: moving feet may open it past the 60 it stands at */
const RING_PX = 120;
const MAX_ROWS = 5;     // …at most this many blocks, then '+N more' (player.ts MAX_NEAR verb rows + the cancel row while the selector is open, and one spare)

/** One block of the interaction panel — one ROW of the list (player.ts buildInteractables: a verb Space can do, or a running one): what the thing is (`label`,
 *  shown only on the first row of a thing — `cont` marks the rows of the same thing under it), the key → action pair plus hint lines (`opts`; key '' = a
 *  status / hint line such as 'picking… 40%' or 'or sprint into it'), whether it is within reach (else greyed, with `dist` in metres when known), `off` (a
 *  thrown breaker), and `progress` 0‥1 for a bar (a lock being picked, the drive coming out, the leaf easing round). */
export interface InteractRow { kind: Interactable['kind']; label: string; opts: { key: string; text: string; tone?: string; /** a squad man down: his line is struck */ gone?: boolean }[]; inReach: boolean; off: boolean; dist?: number; progress?: number; cont?: boolean; /** the squad's element this row is about (orders.ts Tone: gold / red / blue) — #interact .t-<tone> */ tone?: string;
  /** (ui/corner.ts, HANDRING.corner) a block under no heading: a line a key gives, or the lines no key gives (its label alone where it has none) · the objective's section · the clock in front of the objective */ one?: boolean; sec?: boolean; note?: string; /** … and the mission's stage, for its colour */ stage?: string }

/** The marker text is authored as 'key␠␠action  ·  key␠␠action  ·  hint' (player.ts buildInteractables): split it back into rows. 'F or sprint into it  kick…'
 *  keeps F as the key and puts the alternative on a hint line under it. (Until Interactable carries structured options this is the one place that knows the convention.) */
export function parseOpts(line2: string): { key: string; text: string }[] {
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

export class Overlay implements HudLike {
  private root = document.getElementById('overlay')!;
  private meters = document.getElementById('meters')!;
  private lightFill = document.querySelector('#meters .light .fill') as HTMLElement; private lightNum = document.querySelector('#meters .light .num') as HTMLElement;
  private soundFill = document.querySelector('#meters .sound .fill') as HTMLElement; private soundNum = document.querySelector('#meters .sound .num') as HTMLElement; private soundPeak = document.querySelector('#meters .sound .peak') as HTMLElement;
  private soundLab = document.querySelector('#meters .sound .lab') as HTMLElement; private floorKey = '';   // 'SOUND · carpet': what your feet are on, dim, after the label (game.floorUnderPlayer)
  private peak = 0; private peakT = 0;
  private log = document.getElementById('log')!; private logKey = '';
  private captionEl = document.getElementById('caption')!; private captionKey = '';
  private tlEl = document.getElementById('timeline')!; private tlKey = ''; private tlFill: HTMLElement | null = null;
  private equip = document.getElementById('equip')!;
  private traj = document.getElementById('traj') as unknown as SVGSVGElement;
  private trajPath: SVGPolylineElement; private trajLand: SVGCircleElement; private trajShort = false;
  private reachEls!: Record<'kill' | 'hurt' | 'stun', { el: SVGPathElement; d: string }>;   // (game.view.aim.throw.reach): what the thing he is aiming will reach, flat on the floor round where it ends
  private trajRoll: SVGPolylineElement; private trajTick: SVGCircleElement;   // (game.view.aim.throw.rest): the ring goes where the can comes to REST, a tick stays on its first touch, the floor between is dotted
  private squadThrow!: SquadThrowViz;
  private attention!: AttentionViz;
  private picture!: PictureViz;
  private equipKey = '';
  private equipKit: KitView | null = null;
  private equipRev = -1;
  private markers: HTMLElement[] = [];
  /** the men on the floor by their characters' ids, this frame: where the game's marks go */
  private men = new Map<number, Character>();
  private fragMarksEl: HTMLElement[] = [];   // (game.view.marks.frags: a defender's live grenade Sam has been cued on)
  private camMarks: HTMLElement[] = [];    // (game.view.marks.cams: a small lens on each of his own stuck sticky cameras while the fog of war is live)
  private seenMarks: HTMLElement[] = [];   // (game.view.marks.seen: a fading '?' where an enemy was last seen)
  /** the orders ring at the cursor (ui/cmdring.ts ← game/cmdring.ts — a prototype: nothing unless CMDRING.on — the ring while its key is down) */
  readonly cmdRing = new CmdRingViz(this.traj);
  private dots: HTMLElement[] = [];
  showLightDots = true;
  /** the wind field's debug view (Tab ▸ Weather & wind ▸ Wind — field view): a canvas of its own in #overlay, drawn only while one of its switches is on */
  readonly windViz = new WindViz(this.root);
  /** the squad's plan on the floor (ui/planfloor.ts ← game/floorplan.ts): ways, marks, fans, who an open menu goes to */
  readonly plan = new PlanFloor(this.traj);
  /** the player's own man made easy to find (ui/findme.ts ← host/findme.ts): a ring at his feet — nothing unless his Options ask */
  readonly findme = new FindMeViz(this.traj);
  /** Tour timeline: one segment per stop (width ∝ duration) with its chapter title, the live one filled to its progress; segments are clickable. */
  setTimeline(ch: { list: { title: string; dur: number }[]; i: number; frac: number } | null) {
    if (!ch) { if (this.tlKey) { this.tlKey = ''; this.tlEl.className = ''; this.tlEl.innerHTML = ''; this.tlFill = null; } return; }
    const key = ch.list.length + '|' + ch.i;
    if (key !== this.tlKey) {
      this.tlKey = key; const total = ch.list.reduce((s, c) => s + c.dur, 0) || 1;
      this.tlEl.innerHTML = ch.list.map((c, k) => `<div class="seg${k < ch.i ? ' done' : k === ch.i ? ' cur' : ''}" title="${k + 1}. ${c.title}" style="flex:${(c.dur / total * 100).toFixed(2)} 1 0"><b></b><span>${c.title}</span></div>`).join('');
      this.tlEl.className = 'on'; this.tlFill = this.tlEl.querySelector('.seg.cur b');
    }
    if (this.tlFill) this.tlFill.style.width = `${(ch.frac * 100).toFixed(1)}%`;
  }
  setCaption(text: string, sub: string) {
    const key = text + '|' + sub + (GEAR.on ? '|g' : '') + (KEYMAP.rev ? `|k${KEYMAP.rev}` : ''); if (key === this.captionKey) return; this.captionKey = key;
    text = gearText(text); sub = gearText(sub);   // (a showcase's caption names the kit as issued)
    this.captionEl.className = text ? 'on' : '';
    if (text) this.captionEl.innerHTML = `<div class="cap">${keyText(text)}</div>${sub ? `<div class="sub">${keyText(sub)}</div>` : ''}`;
  }
  private wish: HTMLElement; private wishKey = '';   // aimweight.ts: the dot on what the reticle snapped to while the circle (where the gun points) is still swinging onto it
  private reticle: HTMLElement; private ring: HTMLElement; private ringKey = -1;   // the ring on the marker F acts on; ring: progress round it (Interactable.progress — a lock being picked, the drive coming out)
  private hint: HTMLElement; private hintKey = '';                                  // the one word beside the ring: the 40 mm's too-close warning (HE40_SHORT)
  private aimPx = 16; private aimStyled = false;                                     // the aim ring's eased diameter under the aim model (px), and whether its inline size / tint are on it (taken off again for the interaction ring / the model off)
  private iaEl = document.getElementById('interact')!; private iaKey = '';         // interaction panel (top-left): rebuilt when its rows change, distance / progress patched in place
  private iaLive: { em: HTMLElement; bar: HTMLElement | null; d: string; p: string; /** (ui/corner.ts: the objective's words, set in place — a clock in them ticks) */ txt?: HTMLElement | null; t?: string }[] = [];
  /** (HANDRING.corner) setInteractions drew the objective's section this frame: the line at the top says nothing */
  private cornerObj = false;
  private menuEl = document.getElementById('menu')!;                               // start / pause / debrief card up ('on'): no interaction panel behind it
  private objEl = document.getElementById('objective')!; private objKey = '\0';   // the mission's one line (top-centre)
  private alarmEl = document.getElementById('alarm')!; private alarmKey = '';       // alarm stage under it (hidden at calm)
  private blindEl = document.getElementById('blind'); private blindK = 0;             // the white of a stun canister in his eyes (game.view.marks.blind) — absent on a page without the element
  private ckEl = document.getElementById('checkpoint'); private ckKey = '';           // the CHECKPOINT tag (game.saves.flash: written / loaded), shown CKPT.flashSecs — absent on a page without the element
  private skyWas = false;
  private railEl: HTMLElement; private railKey = '\0'; private railNote = ''; private objTurn = ''; private railsWas = false; private railRest: Box | null = null; private railRestOf: [string, string, number, number] = ['\0', '', 0, 0]; private railMan = 0; private railLog = ''; private railBoxes: Box[] = [];                           // a tutorial mission's one big line, low in the middle (game/rails.ts prompt)
  private card: RailCard;                                                          // a tutorial step's card, up in the middle with the world held (ui/railcard.ts)
  /** a tutorial mission's card is up (main.ts holds the world still under it) */
  get cardUp(): boolean { return this.card.up; }
  /** a key went down while it may be the card's (main.ts, ahead of the game's own read of it): true = it was */
  cardKey(e: KeyboardEvent): boolean { return this.card.key(e); }
  /** main.ts: what a card does as it goes (the keyboard's focus back to the game) */
  set cardClosed(fn: (() => void) | null) { this.card.onClose = fn; }
  /** main.ts: this page shows no card (a replay's tape) */
  cards = true;
  private objMark: HTMLElement;                                                    // diamond + distance on / toward the current objective
  constructor() {
    menuTheme();   // the fight screen's rules read the menus' tokens (index.html `--h-…`)
    this.reticle = document.createElement('div'); this.reticle.className = 'reticle'; this.reticle.innerHTML = '<b></b><i></i>'; this.reticle.style.display = 'none'; this.root.appendChild(this.reticle);
    this.wish = document.createElement('div'); this.wish.className = 'aimwish'; this.wish.style.display = 'none'; this.root.appendChild(this.wish);
    this.ring = this.reticle.firstChild as HTMLElement; this.hint = this.reticle.lastChild as HTMLElement; this.hint.style.display = 'none';
    this.railEl = document.createElement('div'); this.railEl.id = 'railprompt'; this.root.appendChild(this.railEl);
    this.card = railCard(this.root);
    this.objMark = document.createElement('div'); this.objMark.className = 'objmark'; this.objMark.innerHTML = '<i></i><u><b></b><b></b><b></b></u><span></span>'; this.objMark.style.display = 'none'; this.root.appendChild(this.objMark);
    const NS = 'http://www.w3.org/2000/svg';
    this.trajPath = document.createElementNS(NS, 'polyline') as SVGPolylineElement;
    this.trajPath.setAttribute('fill', 'none'); this.trajPath.setAttribute('class', 'trajpath'); this.trajPath.setAttribute('stroke-width', '2'); this.trajPath.setAttribute('stroke-dasharray', '5 6'); this.trajPath.setAttribute('stroke-linecap', 'round');
    this.trajLand = document.createElementNS(NS, 'circle') as SVGCircleElement;
    this.trajLand.setAttribute('r', '9'); this.trajLand.setAttribute('class', 'trajland'); this.trajLand.setAttribute('stroke-width', '1.5');
    this.trajRoll = document.createElementNS(NS, 'polyline') as SVGPolylineElement;
    this.trajRoll.setAttribute('fill', 'none'); this.trajRoll.setAttribute('class', 'trajpath trajroll'); this.trajRoll.setAttribute('stroke-width', '2'); this.trajRoll.setAttribute('stroke-dasharray', '1 6'); this.trajRoll.setAttribute('stroke-linecap', 'round');
    this.trajTick = document.createElementNS(NS, 'circle') as SVGCircleElement;
    this.trajTick.setAttribute('r', '3'); this.trajTick.setAttribute('class', 'trajland trajtick'); this.trajTick.setAttribute('stroke-width', '1.5');
    this.trajRoll.style.display = 'none'; this.trajTick.style.display = 'none';
    { const mk = (cls: string) => { const el = document.createElementNS(NS, 'path') as SVGPathElement; el.setAttribute('class', `reach ${cls}`); this.traj.appendChild(el); return { el, d: '' }; };   // (under the arc and its ring: the widest first)
      this.reachEls = { hurt: mk('hurt'), kill: mk('kill'), stun: mk('stun') }; }
    this.traj.appendChild(this.trajPath); this.traj.appendChild(this.trajRoll); this.traj.appendChild(this.trajTick); this.traj.appendChild(this.trajLand);
    this.squadThrow = new SquadThrowViz(this.traj, this.trajPath);   // (a squad throw's ring, walk and arc: over the floor's marks, under his own throw arc)
    this.attention = new AttentionViz(this.traj, this.traj.firstChild as Element | null);   // (where a guard's attention is — game/attention.ts: a thread and a ring on the floor, under everything else drawn there)
    this.picture = new PictureViz(this.traj, this.traj.firstChild as Element | null);   // (one man's picture of the fight — game/picture.ts: a debug view, nothing unless PICTURE.show asks)
    this.trajPath.style.display = 'none'; this.trajLand.style.display = 'none';
  }
  update(game: Game, cam: FollowCamera, canvas: HTMLCanvasElement) {
    const tz = textZoom(), w = canvas.clientWidth / tz, h = canvas.clientHeight / tz;   // (the layer's own pixels: ui/textsize.ts)
    // WHAT THE GAME MARKS (game.view.marks): which men, in which glyphs and which colour; where a man stands is what the picture drew. A viewer's own picture with the fog lifted
    // (bared: the engine replay's switch) shows every man the fog hides, and none of the fog's own marks.
    const M = game.view.marks, A = game.view.aim, bare = game.fogOfWar.bared, men = this.men;
    men.clear(); for (const gd of game.guards) men.set(gd.char.id, gd.char); for (const gd of game.civilians) men.set(gd.char.id, gd.char);
    { let n = 0;
      for (const m of M.men) {
        if (m.hidden && !bare) continue;
        const c = men.get(m.man), text = m.debug ? `${m.text} ${m.debug}` : m.text; if (!c || !text) continue;
        const head = c.bones.head ?? c.pos, [x, y, vis] = cam.project([head[0], head[1] + 0.45, head[2]], w, h, EDGE); if (!vis) continue;
        let el = this.markers[n]; if (!el) { el = document.createElement('div'); el.className = 'marker'; this.root.appendChild(el); this.markers.push(el); }
        n++; el.style.display = 'block'; el.style.left = `${x}px`; el.style.top = `${y}px`; el.textContent = text;
        el.className = `marker ${m.tone}${m.civ && m.tone !== 'ko' && m.tone !== 'dead' ? ' civ' : ''}`;
      }
      for (let i = n; i < this.markers.length; i++) this.markers[i].style.display = 'none'; }
    // where an enemy who has gone out of the team's sight was last seen: a '?' on the spot, fading
    { const seen = bare ? [] : M.seen;
      while (this.seenMarks.length < seen.length) { const el = document.createElement('div'); el.className = 'marker lastseen'; el.textContent = '(?)'; this.root.appendChild(el); this.seenMarks.push(el); }
      this.seenMarks.forEach((el, i) => { const s = seen[i]; if (!s) { el.style.display = 'none'; return; }
        const [x, y, vis] = cam.project([s.at[0], s.at[1] + 2.1, s.at[2]], w, h, EDGE); if (!vis) { el.style.display = 'none'; return; }
        el.style.display = 'block'; el.style.left = `${x}px`; el.style.top = `${y}px`; el.style.opacity = String(Math.max(0.15, 1 - s.age / Math.max(0.01, s.life))); }); }
    // his own stuck sticky cameras under the fog of war: a small lens on each, red in the one that sends
    const cams = bare ? [] : M.cams;
    { let n = 0;
      for (const m of cams) {
        const [x, y, vis] = cam.project(m.at, w, h, EDGE); if (!vis) continue;
        let el = this.camMarks[n]; if (!el) { el = document.createElement('div'); this.root.appendChild(el); this.camMarks.push(el); }
        n++; const cls = m.sends ? 'cammark live' : 'cammark'; if (el.className !== cls) el.className = cls;
        const st = el.style; st.display = 'block'; st.left = `${x}px`; st.top = `${y}px`;
      }
      for (let i = n; i < this.camMarks.length; i++) this.camMarks[i].style.display = 'none'; }
    // a defender's live grenade Sam has seen or heard land: a blinking '!' over it with the seconds left, the cue he acts on
    { const marks = M.frags;
      while (this.fragMarksEl.length < marks.length) { const el = document.createElement('div'); el.className = 'marker frag'; this.root.appendChild(el); this.fragMarksEl.push(el); }
      this.fragMarksEl.forEach((el, i) => { const m = marks[i]; if (!m) { el.style.display = 'none'; return; }
        const [x, y, vis] = cam.project([m.at[0], m.at[1] + 0.6, m.at[2]], w, h, EDGE); if (!vis) { el.style.display = 'none'; return; }
        el.style.display = 'block'; el.style.left = `${x}px`; el.style.top = `${y}px`; el.textContent = `! ${Math.max(0, m.left).toFixed(1)}`; el.style.opacity = Math.floor(m.left * 6) % 2 ? '1' : '0.55'; }); }
    // a faint dot on everything F can act on (lights, doors, bodies — so hidden ceiling panels and closed doors are discoverable)
    const its = game.interactables;
    const camMarked = cams.length > 0;   // (his stuck cameras carry their own lens mark: no dot inside it)
    while (this.dots.length < its.length) { const el = document.createElement('div'); el.className = 'idot'; el.style.cssText = 'position:absolute;width:5px;height:5px;border-radius:50%;transform:translate(-50%,-50%);'; this.root.appendChild(el); this.dots.push(el); }   // (the class is only a handle for index.html to hide them wholesale — attract mode — over the inline display this loop keys)
    this.dots.forEach((el, i) => {
      const it = its[i];
      if (!it || !this.showLightDots || it.pos === game.hover?.pos) { el.style.display = 'none'; return; }   // (the highlighted row is a copy of its thing's marker — same pos array — and the ring is on it already)
      const [x, y, vis] = cam.project(it.pos, w, h, EDGE);
      if (!vis || (it.guard && !bare && fogHides(game.view, it.guard.char.id))) { el.style.display = 'none'; return; }
      el.style.display = 'block'; el.style.left = `${x}px`; el.style.top = `${y}px`;
      if (it.kind === 'guard' || (it.kind === 'cam' && camMarked)) { el.style.display = 'none'; return; }   // living guards already carry a state marker, a stuck camera its lens mark; no extra dot
      el.style.background = it.kind === 'door' ? 'rgba(150,190,230,0.5)' : it.kind === 'body' ? 'rgba(230,140,120,0.6)' : it.kind === 'objective' ? 'rgba(var(--h-acc-rgb),0.8)' : it.off ? 'rgba(120,140,160,0.45)' : 'rgba(255,240,180,0.55)';
    });
    // the ring on the highlighted row's thing (what it offers is listed in the interaction panel, below — the ring only says WHICH thing Space means; the selector's cancel row has no thing)
    const hv = game.hover?.kind === 'cancel' || game.hover?.kind === 'charge' ? null : game.hover;   // (charge: ownblow.ts's row is on his list wherever he stands — it is no thing under the cursor, and the gun's ring stays his)
    let styled = false, wishKey = '';
    // Beside the ring: nothing but the 40 mm's too-close warning (a WARNING, not a prompt: the arc the 40 mm would fly is shorter than its arming distance, said before the click). The
    // right button's own words were taken off the cursor on the maintainer's play note; the kit column says what the button holds.
    let rmb = A.tooClose ? HE40_SHORT : '';
    if (hv) {
      const [x, y, vis] = cam.project(hv.pos, w, h, EDGE);
      this.reticle.style.display = vis ? 'block' : 'none'; this.reticle.style.left = `${x}px`; this.reticle.style.top = `${y}px`;
      this.reticle.className = 'reticle' + (hv.off ? ' off' : '') + (hv.inReach ? '' : ' far') + ` k-${hv.kind}`;
      const pr = hv.progress !== undefined && hv.progress > 0 ? Math.round(Math.min(1, hv.progress) * 200) : -1;   // half-percent steps: no style churn between them
      if (pr !== this.ringKey) { this.ringKey = pr; this.ring.style.display = pr >= 0 ? 'block' : 'none'; if (pr >= 0) this.ring.style.setProperty('--p', `${pr / 2}%`); }
      if (A.dot) { const [gx, gy, gv] = cam.project(A.dot.at, w, h, EDGE); if (gv) { wishKey = A.dot.shut ? 'x' : 'c'; this.wish.style.left = `${gx}px`; this.wish.style.top = `${gy}px`; } }   // (the ring is on the thing Space acts on: the gun's own point still shows while it is off the cursor — a round goes there)
    } else if (A.ring) {   // nothing to interact with near him, a gun out: the ring shows what the auto-aim snapped to, or sits round the cursor's own point
      const R = A.ring, [x, y, vis] = cam.project(R.at, w, h, EDGE);
      if (R.wish) { const [wx, wy, wv] = cam.project(R.wish, w, h, EDGE); if (wv) { wishKey = R.on === 'guard' ? 'g' : 'l'; this.wish.style.left = `${wx}px`; this.wish.style.top = `${wy}px`; } }   // what it will settle on, marked while it travels
      this.reticle.style.display = vis ? 'block' : 'none'; this.reticle.style.left = `${x}px`; this.reticle.style.top = `${y}px`;
      this.reticle.className = 'reticle aim' + (R.trail ? ' k-free trail' : ` k-${R.on}`) + (R.shut ? ' shut' : '');   // (it takes the man's / the light's colour only once the gun is on it)
      if (this.ringKey !== -1) { this.ringKey = -1; this.ring.style.display = 'none'; }
      if (R.cone) {   // the ring IS his cone: its diameter the cone's width at the target's distance (spread × range over the metres a CSS pixel covers at the look-at depth), 10‥RING_PX px, eased; brighter and thinner as the track settles
        const want = aimRingPx(R.cone.spread, R.cone.range, cam, h, RING_PX);
        this.aimPx += (want - this.aimPx) * 0.3;
        const st = this.reticle.style, d = `${this.aimPx.toFixed(1)}px`; st.width = d; st.height = d;
        st.opacity = (lerp(0.42, 0.95, R.cone.track) * (R.trail ? 0.7 : 1)).toFixed(2); st.borderWidth = R.cone.track > 0.97 ? '1px' : '1.5px';
        styled = true;
      }
    } else { this.reticle.style.display = 'none'; rmb = ''; }
    if (wishKey !== this.wishKey) { this.wishKey = wishKey; this.wish.style.display = wishKey ? 'block' : 'none'; this.wish.className = 'aimwish' + (wishKey === 'g' ? ' k-guard' : wishKey === 'c' ? ' gun' : wishKey === 'x' ? ' gun shut' : ''); }
    rmb = keyText(rmb);
    if (rmb !== this.hintKey) { this.hintKey = rmb; this.hint.textContent = rmb; this.hint.style.display = rmb ? 'block' : 'none'; }
    if (styled !== this.aimStyled) { this.aimStyled = styled; if (!styled) { const st = this.reticle.style; st.width = st.height = st.opacity = st.borderWidth = ''; this.aimPx = 16; } }   // (the class rules again for the interaction ring / the model off)
    // interaction panel (top-left): the rows Space can do, the highlighted one lit; open (Space held) it says so and grows its cancel row
    { const ia = this.deriveInteractions(game);
      this.setInteractions(ia.rows, ia.primary, ia.more, ia.open, ia.hint, ia.cancel); }
    // light + sound meters (bottom-left): how lit you are (the GPU light meter) and how much noise you are making (peak-hold marker)
    const v = M.light;
    this.lightFill.style.width = `${(v * 100).toFixed(0)}%`; this.lightNum.textContent = game.showDebug ? `${(v * 100) | 0}  E ${M.irradiance.toFixed(3)}` : `${(v * 100) | 0}`;
    this.meters.classList.toggle('hot', v > 0.55);
    const n = Math.min(1, M.noise);
    this.soundFill.style.width = `${(n * 100).toFixed(0)}%`; this.soundNum.textContent = `${(n * 100) | 0}`;
    if (this.peakT > game.time) { this.peakT = game.time; this.peak = n; }   // (a checkpoint loaded put the clock back: the held peak was the abandoned future's)
    if (n >= this.peak) { this.peak = n; this.peakT = game.time; } else if (game.time - this.peakT > 1.2) this.peak = Math.max(n, this.peak - 0.02);
    this.soundPeak.style.left = `calc(${(this.peak * 100).toFixed(0)}% - 1px)`; this.soundPeak.style.opacity = this.peak > 0.02 ? '0.85' : '0';
    const fl = game.tune.floorMaterials ? game.floorUnderPlayer : ('' as const); if (fl !== this.floorKey) { this.floorKey = fl; this.soundLab.innerHTML = !fl ? 'SOUND' : `SOUND <i style="opacity:.5;font-style:normal">· ${FLOOR[fl].label.toUpperCase()}</i>`; }   // rebuilt only when he crosses onto another finish
    // log
    // the radio strip as the game tells it (game.view.radio), older rows dimmed; a shout that reached an ear from a man the fog hides is the strip's own dim row from the
    // start, under its own word. Under them, what the page itself says (view/pagesaid.ts).
    { const rx = game.view.radio, said = pageRows(), clean = (t: string) => t.replace(/[<>&]/g, c => c === '<' ? '&lt;' : c === '>' ? '&gt;' : '&amp;');
      const key = rx.map(m => m.kind + m.text + (m.age > 5 ? '~' : '')).join('|') + '#' + said.join('|');
      if (key !== this.logKey) { this.logKey = key; this.log.innerHTML = rx.map(m => `<span class="rx${m.kind === 'heard' ? ' heard old' : m.age > 5 ? ' old' : ''}"><b>${m.kind === 'heard' ? 'HEARD' : '·)) RADIO'}</b>${clean(m.text)}</span>`).join('') + said.map(t => `<span class="rx said">${clean(t)}</span>`).join(''); } }
    // mission: the objective line, and a diamond + distance on the objective — clamped to the screen edge in its direction when it is off screen (the
    // usual case from this camera), faded out within a couple of metres so it never fights the F reticle / door marker that takes over there
    // A TUTORIAL MISSION (game/rails.ts): its step is the one instruction on screen — big, low in the middle, each key it names drawn as a key ('[B]' in the
    // line), a pip for every step. The small row at the top stands down while it is up, and the seat with nobody on the floor hides the rows of a man's own kit.
    const frame = game.view.lesson?.rails ?? null, rail = frame?.prompt ?? null;
    // (each cap in brackets is one key, asked of the key map by itself: a line of several — '[W] [A] [S] [D]' — follows a rebind of any of them)
    const caps = (line: string) => keysInLine(line, c => `<kbd>${esc(c)}</kbd>`, t => esc(keyText(t))).replace(/(<kbd>[^<]*<\/kbd>)([:,.;!?])/g, '<span class="nb">$1$2</span>');   // (…and a cap is never parted from the mark after it: a second row that begins ': behind the van')
    const railWords = rail ? caps(rail.line) : '', railNote = rail?.note ? caps(rail.note) : '';   // (a note: the words under the line after a tap the step refused)
    { const key = rail ? `${rail.n}|${rail.of}|${rail.watch ? 'w' : ''}|${railWords}` : '';
      if (key !== this.railKey) { this.railKey = key;
        if (!rail) { this.railEl.className = ''; this.railEl.innerHTML = ''; }
        else { const words = railWords;
          let pips = ''; for (let i = 1; i <= rail.of; i++) pips += `<i class="${i < rail.n ? 'done' : i === rail.n ? 'now' : ''}"></i>`;
          this.railEl.innerHTML = `<div class="line${rail.watch ? ' watch' : ''}">${words}</div><div class="note"></div><div class="pips">${pips}</div>`; this.railNote = '';
          this.railEl.className = ''; void this.railEl.offsetWidth; this.railEl.className = 'on'; } }
      this.card.update(this.cards ? frame?.card ?? null : null, caps);
      // (the note is its own piece: it comes and goes under a line that stays put, and the line is shaken once as it comes)
      if (rail && railNote !== this.railNote) { this.railNote = railNote; const n = this.railEl.querySelector?.('.note'), l = this.railEl.querySelector?.('.line') as HTMLElement | null;
        if (n) n.innerHTML = railNote; if (l) { l.classList.remove('nudge'); if (railNote) { void l.offsetWidth; l.classList.add('nudge'); } } }
      const sky = !!frame?.sky; if (sky !== this.skyWas) { this.skyWas = sky; document.body.classList.toggle('skyseat', sky); }
      const on = !!frame; if (on !== this.railsWas) { this.railsWas = on; document.body.classList.toggle('onrails', on); } }   // (…and on any tutorial mission the place the line points at is marked big: index.html .objmark)
    // …and the line stays clear of your own men (ui/railclear.ts): with the view led up the screen, or in a short window, it lay over the player and his
    // partner. Where it lies is measured once a line (offsets are the layout's: no transform, no animation in them). With a ring up it is at home: the
    // ring's own step of it (ui/cmdring.ts) is the only one. At the bottom edge it is where the radio's rows live, so they go up over it (`--raillog`)
    { let s = 0; const men = rail && !this.cmdRing.ringUp && RAILS.lineClear ? lineMen(game, frame!.lineMen) : [], E = this.railEl, O = this.railRestOf;
      if (men.length) {
        if (O[0] !== this.railKey || O[1] !== this.railNote || O[2] !== w || O[3] !== h) { O[0] = this.railKey; O[1] = this.railNote; O[2] = w; O[3] = h;
          const W = E.offsetWidth, H = E.offsetHeight, L = E.offsetLeft, T = E.offsetTop; this.railRest = W > 0 && H > 0 ? { l: L - W / 2, r: L + W / 2, t: T, b: T + H } : null; }
        if (this.railRest) { const B = this.railBoxes; let n = 0;
          for (const c of men) { const p = c.pos, hd = c.bones.head, [fx, fy, fv] = cam.project(p, w, h, 1.2), [hx, hy] = cam.project(hd ? [hd[0], hd[1] + 0.25, hd[2]] : [p[0], p[1] + 1.85, p[2]], w, h, 1.2); if (!fv) continue;
            const hw = Math.max(18, 0.32 * Math.hypot(fx - hx, fy - hy)), q = B[n] ?? (B[n] = { l: 0, r: 0, t: 0, b: 0 }); n++; q.l = Math.min(fx, hx) - hw; q.r = Math.max(fx, hx) + hw; q.t = Math.min(fy, hy); q.b = Math.max(fy, hy) + 8; }
          B.length = n; s = railClear(this.railRest, B, h, this.railMan > 0); } }
      else if (!rail) O[0] = '\0';
      if (s !== this.railMan) { this.railMan = s; E.style.setProperty?.('--railman', `${s.toFixed(0)}px`); }
      // (the rows' place is the line's own top at the bottom edge: written when IT changes — the line is hung from its bottom, so a taller line keeps the same
      //  step down and has a higher top, and the rows stood in its first row of words)
      { const log = s > 0 && this.railRest ? `${(h - this.railRest.t - s + 6).toFixed(0)}px` : '';
        if (log !== this.railLog) { this.railLog = log; const st = document.body.style; if (log) st?.setProperty?.('--raillog', log); else st?.removeProperty?.('--raillog'); } } }
    const ot = rail || this.cornerObj ? '' : objectiveWords(game);   // (HANDRING.corner: the objective is a section of the corner panel, not a line up here)
    if (ot !== this.objKey) { this.objKey = ot; const W = 'objective ▸', cut = ot.indexOf(W);   // the line's own word is set apart as its label (a par clock in front of it is not part of it); the text read back is unchanged
      if (cut < 0) this.objEl.textContent = ot; else this.objEl.innerHTML = `${esc(ot.slice(0, cut))}<b>${W}</b>${esc(ot.slice(cut + W.length))}`;
      this.objEl.className = game.mission.stage; }
    { // alarm stage: level, whole seconds left on its clock, and whether an episode is running — rebuilt only when that key changes
      const lvl = game.quietUtility ? 0 : game.escalation; const left = lvl > 0 && game.escalationT > game.time ? Math.ceil(game.escalationT - game.time) : 0;
      const live = lvl > 0 && game.alarm.episode;
      const key = lvl === 0 ? '' : `${lvl}|${left}|${live ? 1 : 0}`;
      if (key !== this.alarmKey) { this.alarmKey = key;
        if (!key) this.alarmEl.className = ''; else { this.alarmEl.className = `on l${lvl}${live ? ' live' : ''}`; this.alarmEl.innerHTML = `<i></i>${lvl === 1 ? 'ALARM I · heightened' : 'ALARM II · lockdown'}${live ? '' : left ? ` · ${left}s` : ''}`; } }
    }
    { const b = Math.round(M.blind * 50) / 50; if (b !== this.blindK && this.blindEl) { this.blindK = b; this.blindEl.style.opacity = String(0.97 * b); document.body.classList.toggle('blinded', b > 0.5); } }   // blind: the picture whites out, clearing over the dazzle's last seconds
    { // the checkpoint tag: up for a couple of seconds after a save / load (game/savestate.ts Checkpoints.flash), keyed so the DOM is touched only on change
      const F = game.saves.flash; const on = !!F && game.time - F.t >= 0 && game.time - F.t < CKPT.flashSecs; const key = on ? F!.text : '';
      if (key !== this.ckKey && this.ckEl) { this.ckKey = key; this.ckEl.className = on ? 'on' : ''; if (on) this.ckEl.innerHTML = `<b></b>${key}`; } }
    const op = game.objectivePos();
    const skySeat = !!frame?.sky;   // (nobody on the floor is the player: the marker is a place to point at, not a walk to it — no fade near it, no metres)
    const far = op ? v3.distXZ(op, game.player.char.pos) : 0, od = op ? (skySeat || frame?.arrow ? 99 : far) : 0;   // (…nor near a step's arrow: it is drawn for the last pace, where the diamond would have faded out)
    const floor = game.mission.stage === 'drive' && game.hover?.kind !== 'objective' ? 0.35 : 0;   // the rack has no marker of its own: keep a faint diamond on it until the cursor finds it (doors have their own dot / reticle)
    const alpha = op ? Math.max(floor, Math.min(0.9, (od - 1.6) / 1.5)) : 0;
    if (!op || alpha < 0.03 || rail?.watch) this.objMark.style.display = 'none';   // (a line that asks for nothing marks no place to press on)
    else {
      const vp = cam.viewProj; const cw = vp[3] * op[0] + vp[7] * op[1] + vp[11] * op[2] + vp[15];
      let nx = vp[0] * op[0] + vp[4] * op[1] + vp[8] * op[2] + vp[12], ny = vp[1] * op[0] + vp[5] * op[1] + vp[9] * op[2] + vp[13];
      if (cw > 1e-4) { nx /= cw; ny /= cw; } else { const l = Math.hypot(nx, ny) || 1; nx = nx / l * 9; ny = ny / l * 9; }   // behind the eye: keep the undivided clip direction (dividing by w < 0 would mirror it) and force it onto the edge
      const mx = 1 - 2 * 36 / w, my = 1 - 2 * 36 / h; const k = Math.max(1, Math.abs(nx) / mx, Math.abs(ny) / my); nx /= k; ny /= k;   // into a 36 px inset of the view
      const el = this.objMark, mx0 = (nx * 0.5 + 0.5) * w, my0 = (1 - (ny * 0.5 + 0.5)) * h;
      // (…and a tutorial step's mark under a ring that is up, the squad's or his own, is not drawn: it lay over the ring's middle and its inner digits, and
      //  with the ring up there the place is found — rails.ts RAILS.markClear; the rings are last frame's, as drawn)
      if (frame && RAILS.markClear && this.cmdRing.covers(mx0, my0)) el.style.display = 'none';
      else {
      el.style.display = 'block'; el.style.opacity = alpha.toFixed(2);
      // (a tutorial step's `toward`: the mark is an arrow lying from the place to where it points, turned as the view has the two — never at the picture's edge, where the ring says 'that way')
      const to = k > 1 ? null : frame?.arrow ?? null; let turn = '';
      if (to) { const [ax, ay, av] = cam.project(op, w, h, EDGE), [bx, by] = cam.project(to, w, h); if (av && Math.hypot(bx - ax, by - ay) > 4) turn = `${(Math.atan2(by - ay, bx - ax) * 180 / Math.PI).toFixed(1)}deg`; }
      el.className = k > 1 ? 'objmark edge' : turn ? 'objmark arrow' : 'objmark'; if (turn !== this.objTurn) { this.objTurn = turn; el.style.setProperty?.('--turn', turn || '0deg'); }
      el.style.left = `${mx0.toFixed(1)}px`; el.style.top = `${my0.toFixed(1)}px`;
      (el.lastChild as HTMLElement).textContent = skySeat || turn ? '' : `${Math.round(far)}m`;   // (an arrow lies at his feet and says which way, not how far: under it the metres read '99m', the number that keeps it from fading)
      }
    }
    this.cmdRing.update(game, cam, w, h, tz);
    this.squadThrow.update(game, cam, w, h, EDGE);
    this.attention.update(game, cam, w, h, EDGE);
    this.picture.update(game, cam, w, h, EDGE);
    // throw preview: the arc as the game has it, where it comes to rest (none: the launcher's arc) and what it would reach
    const pv = A.throw;
    if (pv) {
      const pts: string[] = [];
      for (const p of pv.points) { const [x, y, vis] = cam.project(p, w, h, EDGE); if (vis) pts.push(`${x.toFixed(1)},${y.toFixed(1)}`); }
      this.trajPath.setAttribute('points', pts.join(' ')); this.trajPath.style.display = 'block';
      if (pv.short !== this.trajShort) { this.trajShort = pv.short; this.trajPath.style.stroke = this.trajLand.style.stroke = pv.short ? HE40_SHORT_STROKE : ''; this.trajPath.style.strokeDasharray = pv.short ? '3 7' : ''; }   // (a 40 mm arc too short to arm: amber, broken — whatever the reticle is busy saying)
      const rest = pv.rest, ring = rest ? rest.at : pv.land;
      this.drawReach(pv.reach, cam, w, h);
      const [lx, ly, lv] = cam.project(ring, w, h, EDGE); this.trajLand.style.display = lv ? 'block' : 'none'; this.trajLand.setAttribute('cx', lx.toFixed(1)); this.trajLand.setAttribute('cy', ly.toFixed(1));
      if (rest) {
        const [tx, ty, tv] = cam.project(pv.land, w, h, EDGE), far = Math.hypot(rest.at[0] - pv.land[0], rest.at[2] - pv.land[2]) > 0.3;   // (a can that stays where it lands needs no tick under its ring)
        this.trajTick.style.display = tv && far ? 'block' : 'none'; this.trajTick.setAttribute('cx', tx.toFixed(1)); this.trajTick.setAttribute('cy', ty.toFixed(1));
        const rp: string[] = []; if (far) for (const p of [pv.land, ...rest.path, rest.at]) { const [x, y, vis] = cam.project(p, w, h, EDGE); if (vis) rp.push(`${x.toFixed(1)},${y.toFixed(1)}`); }
        this.trajRoll.setAttribute('points', rp.join(' ')); this.trajRoll.style.display = rp.length > 1 ? 'block' : 'none';
      } else { this.trajTick.style.display = 'none'; this.trajRoll.style.display = 'none'; }
    } else { this.trajPath.style.display = 'none'; this.trajLand.style.display = 'none'; this.trajTick.style.display = 'none'; this.trajRoll.style.display = 'none'; this.drawReach(null, cam, w, h); }
    this.plan.update(game, cam, w, h);
    this.findme.update(game, cam, w, h, this.plan.youOn);
    // equipment
    this.updateEquip(game);
    // the wind field's debug view (nothing drawn, nothing sampled unless a switch of it is on)
    this.windViz.update(game, cam, w, h);
  }

  /** WHAT THE THING HE IS AIMING WILL REACH (null = nothing to draw): each radius a ring flat on the floor under the point, so the camera's
   *  tilt squashes it as it squashes the floor. A point behind the camera BREAKS the ring there and the rest of it stays (squadthrowviz.ts draws the
   *  squad's the same way); only a whole ring is filled. A ring that has not changed writes nothing. */
  private drawReach(R: ThrowReachView | null, cam: FollowCamera, w: number, h: number) {
    const ring = (E: { el: SVGPathElement; d: string }, r: number) => {
      let d = '', run = 0, whole = true;
      if (R && r > 0) for (let i = 0, N = 64; i <= N; i++) {
        const a = (i / N) * Math.PI * 2, [x, y, vis] = cam.project([R.at[0] + Math.cos(a) * r, 0.03, R.at[2] + Math.sin(a) * r], w, h);
        if (!vis) { run = 0; whole = false; continue; }
        d += `${run++ ? 'L' : 'M'}${x.toFixed(1)} ${y.toFixed(1)}`;
      }
      if (d && whole) d += 'Z';
      if (d !== E.d) { E.d = d; E.el.setAttribute('d', d); E.el.classList.toggle('cut', !whole); }   // (a cut ring has no fill: index.html .reach.cut)
    };
    const E = this.reachEls; ring(E.kill, R?.kill ?? 0); ring(E.hurt, R?.hurt ?? 0); ring(E.stun, R?.stun ?? 0);
  }

  /** Interaction panel: `rows` top to bottom, `primary` = index of the highlighted row (what a Space tap does / letting go performs; -1: none), `more` = how
   *  many were left off the end, `open` = the selector is open (Space held: index.html #interact.open — the frame brightens, a header says let go to act,
   *  the cancel row is among the rows). The DOM is rebuilt only when the rows' text / reach / kind change; the distance and the progress bar are patched in
   *  place every frame. Hidden when there is nothing to list or a menu card is up. update() feeds it from the game's rows (deriveInteractions). */
  setInteractions(rows: InteractRow[], primary = -1, more = 0, open = false, hint = `let go to do it · wheel / ↑↓ chooses · ${backName()} backs out`, cancel = 'cancel — let go, do nothing') {
    this.cornerObj = rows.some(r => r.sec) && !this.menuEl.classList.contains('on');   // (the objective is drawn here this frame: update()'s line at the top says nothing)
    if (!rows.length || this.menuEl.classList.contains('on')) { if (this.iaKey) { this.iaKey = ''; this.iaEl.className = ''; this.iaEl.textContent = ''; this.iaLive = []; } return; }
    const key = rows.map(r => r.sec ? `sec|${r.label}|${r.stage ?? ''}|${r.label || r.note ? 1 : 0}` : `${r.kind}|${r.label}|${r.cont ? 1 : 0}|${r.tone ?? ''}|${r.opts.map(o => `${o.key}>${o.text}>${o.tone ?? ''}${o.gone ? '~' : ''}`).join('/')}|${r.inReach ? 1 : 0}${r.off ? 1 : 0}${r.dist !== undefined ? 1 : 0}${r.progress !== undefined ? 1 : 0}${r.one ? 'o' : ''}`).join('\n') + `#${primary}#${more}#${open ? `1${hint}` : 0}#${cancel}#${KEYMAP.rev}`;
    if (key !== this.iaKey) {
      this.iaKey = key;
      const soft = (o: InteractRow['opts'][number]) => `<div class="hint${o.gone ? ' lost' : ''}${o.tone ? ` t-${o.tone}` : ''}">${o.gone ? `<s>${esc(keyText(o.text))}</s>` : esc(keyText(o.text))}</div>`;   // (a line no key gives; a squad man down: struck)
      this.iaEl.innerHTML = (open ? `<div class="selhint">${esc(keyText(hint))}</div>` : '') + rows.map((r, i) =>
        r.kind === 'cancel' ? `<div class="it k-cancel${i === primary ? ' primary' : ''}"><div class="opt"><kbd>×</kbd>${esc(keyText(cancel))}</div></div>`
        : r.sec ? `<div class="it sec${r.stage ? ` ${esc(r.stage)}` : ''}">${r.label || r.note ? `<div class="what"><span>${esc(r.label)}</span><em></em></div>` : ''}<div class="opt"></div></div>`   // (ui/corner.ts: the objective — its words and its clock are set below, in place)
        : r.one ? `<div class="it one k-${r.kind}${i === primary ? ' primary' : ''}${r.inReach ? '' : ' far'}${r.off ? ' off' : ''}">${r.opts.length ? r.opts.map(o => o.key ? `<div class="opt"><kbd>${esc(keyCap(o.key).toUpperCase())}</kbd>${esc(keyText(o.text))}</div>` : soft(o)).join('') : `<div class="what"><span>${esc(keyText(r.label))}</span><em></em></div>`}${r.progress !== undefined ? '<div class="prog"><b></b></div>' : ''}</div>`   // (… and a block under no heading)
        : `<div class="it k-${r.kind}${i === primary ? ' primary' : ''}${r.inReach ? '' : ' far'}${r.off ? ' off' : ''}${r.cont ? ' cont' : ''}${r.tone ? ` t-${r.tone}` : ''}">${r.cont ? '<div class="what"><em></em></div>' : `<div class="what"><span>${esc(r.label)}</span><em></em></div>`}`
        + r.opts.map((o, j) => o.key ? `<div class="opt${j === 0 && /^space$/i.test(o.key) ? ' verb' : ''}${o.tone ? ` t-${o.tone}` : ''}"><kbd>${esc(keyCap(o.key).toUpperCase())}</kbd>${esc(keyText(o.text))}</div>` : soft(o)).join('')   // (.verb: the row's own Space verb — lit only on the highlighted row; the hint keys under it, LMB / E, are always live and never dimmed)
        + (r.progress !== undefined ? '<div class="prog"><b></b></div>' : '') + '</div>').join('') + (more > 0 ? `<div class="more">+${more} more</div>` : '');
      this.iaEl.className = (open ? 'on open' : 'on') + (rows.some(r => r.one || r.sec) ? ' corner' : '');
      const blocks = [...this.iaEl.querySelectorAll('.it')] as HTMLElement[];
      this.iaLive = rows.map((_, i) => { const el = blocks[i]; return { em: el.querySelector('.what em') as HTMLElement, bar: el.querySelector('.prog b') as HTMLElement | null, d: '', p: '', txt: rows[i].sec ? el.querySelector('.opt') as HTMLElement | null : null, t: '' }; });
    }
    rows.forEach((r, i) => {
      const L = this.iaLive[i];
      const d = r.sec ? r.note ?? '' : r.dist !== undefined ? `${r.dist < 10 ? r.dist.toFixed(1) : Math.round(r.dist)}m` : ''; if (L.em && d !== L.d) { L.d = d; L.em.textContent = d; }
      if (L.txt) { const t = r.opts[0]?.text ?? ''; if (t !== L.t) { L.t = t; L.txt.textContent = t; } }
      if (L.bar) { const p = `${(Math.round(Math.min(1, Math.max(0, r.progress ?? 0)) * 200) / 2).toFixed(1)}%`; if (p !== L.p) { L.p = p; L.bar.style.width = p; } }   // half-percent steps, like the ring
    });
  }

  /** What the panel lists this frame, from the same data Space reads: player.ts's rows (Game.nearby — one per verb, the running verb alone while it runs, the
   *  cancel row at the foot while the selector is open) in their stable priority order, the highlighted one (Game.hover) marked primary; rows of the same thing
   *  under its first are continuations (no second header). Light fixtures are never rows (they are ranged: the pistol's business). */
  private deriveInteractions(game: Game): { rows: InteractRow[]; primary: number; more: number; open: boolean; hint?: string; cancel?: string } {
    const hv = game.hover, p = game.player.char.pos;
    const dist = (it: Interactable) => thingDist(it, p);   // doors: leaf or doorway centre, as the reach test measures it
    // (handring.ts: the corner shows ONE line, what the key would do now — the highlighted row, or a running verb's own; none while the ring's lit wedge
    // is the whistle — and nothing where it does nothing; the rest of what is on offer is the ring's. A line no key gives stays under it)
    const lead = hv && hv.kind !== 'cancel' && (hv.verb || hv.running) && !game.player.sel.ring?.whistle ? hv : null;
    const hint = (): Interactable[] => game.nearby.filter(r => !r.verb && r !== lead).slice(0, 1);   // (a line no key gives: the status of a committed move, or what the clicks do to the man in reach — close.ts. It is drawn when it is the ONLY row too: with the held click the one way to a man (stalk.ts) his line stands alone, and the highlight resting on it took it off the page)
    const list: Interactable[] = !HANDRING.on || game.puppet ? game.nearby : lead ? [lead, ...hint()] : hint();
    const thing = (it: Interactable) => `${it.kind}:${it.door?.def.name ?? it.guard?.callsign ?? ''}`;
    const cap = MAX_ROWS + (list.some(it => it.kind === 'cam') ? 1 : 0);   // (stickycam.ts: a stuck camera's 'take it back' is one row past every row the list had before it — it is always drawn)
    const rows = list.slice(0, cap).map((it, i): InteractRow => { const d = dist(it); return { kind: it.kind, label: it.line1, opts: it.kind === 'cancel' ? [] : parseOpts(it.line2), inReach: it.inReach, off: it.off, dist: it.inReach || it.kind === 'light' || it.kind === 'cancel' ? undefined : d, progress: it.progress, cont: i > 0 && it.kind !== 'cancel' && thing(list[i - 1]) === thing(it) }; });
    // (handring.ts `empty`: the key tapped with no row to give — said here for a moment, where his rows come up. `near`: beside a thing the word names
    // it, with how far off a door a step short is; every such tap answers, and the word is up only while it is still what a tap would say — a step into
    // reach, or away, takes it down. With `near` off, as it was: the log's newest line, while that is the word)
    if (HANDRING.empty && !list.length && !game.puppet && !game.player.down) { const S = game.player.sel, young = (t: number) => game.time >= t && game.time - t < EMPTY_SHOW_SECS;
      if (!HANDRING.near) { const m = game.messages[game.messages.length - 1]; if (m?.text === EMPTY_WORD && young(m.t)) rows.push({ kind: 'wall', label: EMPTY_WORD, opts: [], inReach: false, off: false }); }
      else if (S.word && young(S.wordT)) { const T = emptyThing(game);
        if (emptyWord(game, T) === S.word) rows.push({ kind: T ? T.it.kind : 'wall', label: S.word, opts: [], inReach: false, off: false, dist: T && T.why === EMPTY_WHY.far ? T.d : undefined }); } }
    const primary = hv ? list.indexOf(hv) : -1;
    if (cornerOn(!!game.puppet)) {   // (ui/corner.ts: the objective, his key's line, the squad key's line, and what the block and a door's heading said that it would lose: a key's line kept back where the mission's frame keeps the squad's notes back, a warning never)
      const V = game.view.squad, shut = locked(game.view, 'menu'), quick = game.orders.quickLine();
      const men: InteractRow['opts'] = (V?.mates ?? []).map(M => ({ key: '', text: `${V!.picked ? (M.addressed ? '▸ ' : '· ') : ''}${M.name} · ${M.doing}`, tone: M.pair ?? undefined, gone: M.down }));
      const squad: InteractRow[] = V ? [{ kind: 'objective', label: '', opts: [...men, ...(shut ? [] : V.notes), ...(quick ? [quick] : [])], inReach: true, off: false }] : [];
      const C = cornerRows(rows, primary, squad, game.view.lesson?.rails?.prompt ? '' : objectiveWords(game), game.view.corner.filter(o => !o.key || (!!V && !shut)), game.mission.stage);
      return { rows: C.rows, primary: C.primary, more: 0, open: false }; }
    return { rows, primary: primary >= 0 && primary < cap ? primary : -1, more: Math.max(0, list.length - cap), open: game.player.listOpen };   // (up as a ring — cmdring.ts CMDRING.hands — the panel's list is the list at rest)
  }


  /** Kit column (bottom-left, standing on the meters): ui/handbox.ts draws what the game tells it, and again only when that (or a key's binding) has changed. A new
   *  record every tick is no change: the OCP's charge is drawn in twentieths. */
  private updateEquip(game: Game) {
    const kit = game.view.kit; if (kit === this.equipKit && KEYMAP.rev === this.equipRev) return;
    this.equipKit = kit; this.equipRev = KEYMAP.rev;
    const key = JSON.stringify(kit, (k, v) => k === 'charge' ? Math.round(v * 20) : v) + KEYMAP.rev;
    if (key !== this.equipKey) { this.equipKey = key; drawHandBox(this.equip, kit); }
  }
}

function esc(t: string): string { return t.replace(/&/g, '&amp;').replace(/</g, '&lt;'); }
/** The aim ring's diameter (CSS px) for a cone of `spread` rad (± half per axis: the cone is spread wide) at `dist` metres: that width over the metres one CSS pixel
 *  covers at the camera's look-at depth, clamped 10‥`cap` (60; aimcost.ts hands `ringPx` while moving costs the shot). Pure — the QA probes read it (aim-probes.ts). */
export function aimRingPx(spread: number, dist: number, cam: { distance: number; fov: number }, viewH: number, cap = 60): number {
  const mPerCssPx = 2 * cam.distance * Math.tan(cam.fov / 2) / Math.max(200, viewH);
  return clamp(spread * dist / Math.max(mPerCssPx, 1e-4), 10, Math.max(10, cap));
}
