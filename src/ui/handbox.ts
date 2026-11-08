// THE KIT'S COLUMN: A STRIP OF WHAT THE KEYS BRING AND THE BOX OF WHAT IS IN HIS HANDS. Everything it says is the game's (game.view.kit, rust/game_rules/src/kit
// `kit_view`); this file holds how it looks and imports nothing of the game.
// The maintainer, 10-22, of Chaos Theory's panel: "scct had a really compact current equipment screen, probably too compact for us but we can aspire to it".
//   A STRIP of the places, each where it is whatever is selected: its key and what the key brings (a gun's name and rounds ready, the cans
//   themselves). The one that is up is a tab, and under it
//   ONE BOX of one line for it: the magazine as pips (a reload is written across them, and they go dim under it), what is set on the gun as plain
//   words (the selector's position, the load on the launcher and how many, the light while it is on), and the rounds in the gun over what its
//   magazine holds with a little box for each spare magazine beside it, each as full as that magazine is (the maintainer of the box that read 31 / 60:
//   "we should say 31/30 and then two spare magazines on the side"); a gun fed a shell at a time keeps its two numbers. A can: the cans and how many.
//   THE SETUP POPS UP ABOVE THE COLUMN (the maintainer, 10-22, after playing the box that carried its rows always: "to make it
//   feel like the radial menu it should be something that pops up above the equipment menu rather than always taking up space"). The gun's key again
//   and a small panel stands over the column: a row each for what can be set, its digit on a warm cap, its positions beside it with the one that is
//   up a filled plate, and the back key's cap in its corner. The tab's and the box's edges light and the slots whose digits are rows for now go dim
//   in the strip. Nothing of the column moves when it opens, no line of words explains it, and nothing of it is drawn while it is shut. The warm
//   colour is the open setup's alone.
//   EMPTY HANDS have a place at the head of the strip: with everything put away it is the tab, and the box under it is empty.
// Every slot is still one `.slot` with its `.key`, in the kit's order: what reads the column by those names finds them.
import type { KitGun, KitSetup, KitSlot, KitThing, KitView } from '../view/seatview';
import { capNow, keyCap, keyText, keysOf } from '../host/keymap';

const CSS = `
#equip.hand { width: 380px; min-width: 0; }
#equip.hand .strip { display: grid; grid-template-columns: auto auto 1fr 1fr 1fr; gap: 3px; align-items: stretch; }
#equip.hand .strip.h6 { grid-template-columns: auto auto auto 1fr 1fr 1fr; }
#equip.hand .strip .slot.hands .mt { display: inline-block; width: 7px; height: 10px; border: 1px dashed var(--h-soft); border-radius: 2px; }
#equip.hand .strip .slot { box-sizing: border-box; gap: 5px; padding: 4px 6px; justify-content: center; min-width: 0; }
#equip.hand .strip .slot .name { letter-spacing: 0.04em; }
#equip.hand .strip .slot .n { color: var(--h-soft); font: 600 10px/1 var(--h-mono); min-width: 2ch; text-align: right; }
#equip.hand .strip .slot.none { opacity: 0.35; }
#equip.hand .strip .slot.none .name { font-size: 10px; letter-spacing: 0.02em; }
#equip.hand .strip.h6 .slot { padding-left: 5px; padding-right: 5px; }
#equip.hand .strip .slot.sel { opacity: 1; border-bottom-color: transparent; border-radius: 3px 3px 0 0; box-shadow: none; background: rgba(var(--h-plate),0.82); padding-bottom: 7px; margin-bottom: -4px; position: relative; z-index: 1; }
#equip.hand.setup .strip .slot.sel { border-color: var(--h-acc-hi); border-bottom-color: transparent; }
#equip.hand.setup .strip .slot.sel .key { color: var(--h-acc-hi); }
#equip.hand.setup .strip .slot.row { opacity: 0.3; }
#equip.hand .hb { position: relative; box-sizing: border-box; display: flex; flex-direction: column; justify-content: space-between; gap: 6px; min-height: 54px; padding: 8px 9px 8px; border: 1px solid rgba(var(--h-edge),0.7); background: rgba(var(--h-plate),0.82); border-radius: 3px; box-shadow: 0 0 8px rgba(0,0,0,0.4); }
#equip.hand .hb.empty { opacity: 0.6; border-style: dashed; }
#equip.hand .hb.setup { border-color: var(--h-acc-hi); box-shadow: 0 0 10px rgba(var(--h-acc-rgb),0.35); }
#equip.hand .hb-top { display: flex; align-items: flex-end; gap: 8px; min-height: 16px; }
#equip.hand .hb-top .rounds { position: static; }
#equip.hand .hb-top .name { font-size: 13px; }
#equip.hand .hb-top .re { font: 700 9px/1 var(--h-head); letter-spacing: 0.14em; color: var(--h-soft); }
#equip.hand .hb-n { margin-left: auto; font: 700 17px/0.85 var(--h-mono); color: var(--h-hi); letter-spacing: 0.02em; white-space: nowrap; }
#equip.hand .hb-n i { font: 600 11px/1 var(--h-mono); color: var(--h-soft); margin-left: 5px; }
#equip.hand .hb-n i.of { margin-left: 0; }
#equip.hand .hb-top .mags { margin-left: -3px; gap: 2px; align-self: center; flex: none; }   /* (two boxes and 31/30 take the line's room that 31 / 60 took: the words for what is set are not cut) */
#equip.hand .hb-top .mag { width: 4px; }
#equip.hand .hb-n.dry { color: #ff9c8c; }
#equip.hand .hb-top .tag.ocp { margin-left: 4px; }
#equip.hand > .slot.big { justify-content: flex-start; }
#equip.hand.pop .hb { min-height: 32px; justify-content: center; padding: 7px 9px; }
#equip.hand.pop .hb-top { align-items: center; }
#equip.hand .hb-mag { position: relative; display: inline-flex; align-items: center; }
#equip.hand .hb-mag.re .rounds { opacity: 0.22; }
#equip.hand .hb-mag .re { position: absolute; left: 0; top: 50%; transform: translateY(-50%); white-space: nowrap; color: var(--h-hi); }
#equip.hand .hb-st { display: inline-flex; align-items: center; gap: 6px; min-width: 0; overflow: hidden; white-space: nowrap; }
#equip.hand .hb-st .o { font: 700 10px/13px var(--h-head); letter-spacing: 0.03em; color: var(--h-soft); text-shadow: none; }
#equip.hand .hb-st .o.on { color: var(--h-hi); }
#equip.hand .hb-st .o.out { opacity: 0.45; }
#equip.hand .hp { position: absolute; left: 0; bottom: 100%; margin-bottom: 7px; box-sizing: border-box; min-width: 170px; display: flex; flex-direction: column; gap: 6px; padding: 9px 44px 9px 9px; border: 1px solid var(--h-acc-hi); background: rgba(var(--h-plate),0.92); border-radius: 3px; box-shadow: 0 0 10px rgba(var(--h-acc-rgb),0.35); transform-origin: 0 100%; }
#equip.hand .hp.in { animation: hp-in 0.09s ease-out; }
@keyframes hp-in { from { opacity: 0; transform: scale(0.92); } to { opacity: 1; transform: none; } }
#equip.hand .hp-row { display: flex; align-items: center; gap: 4px; white-space: nowrap; }
#equip.hand .hp kbd { box-sizing: border-box; display: inline-block; min-width: 16px; height: 16px; padding: 0 3px; margin-right: 5px; text-align: center; font: 700 11px/14px var(--h-mono); color: var(--h-on); background: var(--h-acc-hi); border: 1px solid var(--h-acc-hi); border-radius: 2px; }
#equip.hand .hp kbd.x { position: absolute; right: 7px; top: 7px; margin: 0; min-width: 0; height: 13px; padding: 0 3px; font: 700 8px/11px var(--h-mono); letter-spacing: 0.06em; color: var(--h-soft); background: none; border-color: rgba(var(--h-edge),0.45); }
#equip.hand .hp .o { font: 700 11px/16px var(--h-head); letter-spacing: 0.04em; padding: 0 4px; border-radius: 2px; color: var(--h-faint); text-shadow: none; }
#equip.hand .hp .o.on { color: var(--h-on); background: var(--h-hi); }
#equip.hand .hp .o.out { opacity: 0.45; }
`;
function styles() { if (document.getElementById?.('handbox-css')) return; const s = document.createElement('style'); s.id = 'handbox-css'; s.textContent = CSS; (document.head ?? document.body)?.appendChild?.(s); }

const mk = (tag: string, cls?: string, text?: string) => { const e = document.createElement(tag); if (cls) e.className = cls; if (text !== undefined) e.textContent = cls === 'key' ? keyCap(text) : keyText(text); return e; };   // (.key: a slot's own cap, as the column's)
const CAN_CLASS: Partial<Record<KitThing, string>> = { smoke: 'cans', flash: 'cans bangs', frag: 'cans frags' };

/** what is set on a gun, as the box's plain words: the selector's position on a gun that has two, the load on the launcher and how many of it, the light while it is on */
function state(gun: KitGun): HTMLElement {
  const st = mk('span', 'hb-st');
  if (gun.selector) st.appendChild(mk('span', 'o', gun.selector));
  if (gun.load) st.appendChild(mk('span', 'o' + (gun.load.out ? ' out' : ''), gun.load.text));
  if (gun.light) st.appendChild(mk('span', 'o on', keysOf('KeyL').length ? 'L LIGHT' : 'LIGHT'));   // (its key named only while its row has one: host/keymap.ts setFree)
  return st;
}
/** the open setup, above the column: a row for each thing its gun can have set, the row's digit on a cap and its positions beside it (the one that is
 *  up a filled plate; a load he is out of dim), and the back key's cap in the corner */
function popup(setup: KitSetup): HTMLElement {
  const pop = mk('div', 'hp');
  for (const r of setup.rows) {
    const row = mk('div', 'hp-row'), k = mk('kbd'); k.textContent = capNow(r.key); row.appendChild(k);
    for (const w of r.words) row.appendChild(mk('span', `o${w.on ? ' on' : ''}${w.out ? ' out' : ''}`, w.text));
    pop.appendChild(row);
  }
  const x = mk('kbd', 'x'); x.textContent = setup.back; pop.appendChild(x);
  return pop;
}
/** was the pop-up drawn at the last draw (it comes in once, not at every redraw while it stands) */
let popUp = false;
/** a gun's magazine as pips, each round there or spent (the column's own drawing: a belt of a hundred, a magazine of forty: a pip stands for five rounds, for two) */
function magazine(gun: KitGun): HTMLElement {
  const per = gun.capacity > 60 ? 5 : gun.capacity > 34 ? 2 : 1;
  const rounds = mk('span', 'rounds'); for (let i = 0; i < Math.ceil(gun.capacity / per); i++) rounds.appendChild(mk('i', i < Math.ceil(gun.mag / per) ? '' : 'spent')); if (gun.chamber) rounds.appendChild(mk('i', 'chamber'));
  return rounds;
}
/** his spare magazines: a little box each, filled as far as that magazine is (the column's own drawing: index.html #equip .mags) */
function spares(gun: KitGun): HTMLElement {
  const mags = mk('span', 'mags');
  for (const r of gun.spares) { const m = mk('span', 'mag'), b = mk('b'); b.style.height = `${Math.round((r / gun.capacity) * 100)}%`; m.appendChild(b); mags.appendChild(m); }
  return mags;
}
const pips = (s: KitSlot) => { const c = mk('span', CAN_CLASS[s.thing]); for (let i = 0; i < s.count; i++) c.appendChild(mk('i')); return c; };

/** a place in the strip: what its key brings (the one that is up is the box's tab) */
function slotOf(s: KitSlot): HTMLElement {
  const key = () => { const k = mk('span', 'key'); k.textContent = capNow(s.key); return k; };
  if (s.thing === 'hands') { const e = mk('div', 'slot hands' + (s.up ? ' sel' : '')); e.appendChild(key()); e.appendChild(mk('i', 'mt')); return e; }
  if (s.absent) { const e = mk('div', 'slot absent'); e.appendChild(key()); e.appendChild(mk('span', 'name', s.name)); return e; }
  const e = mk('div', 'slot' + (s.up ? ' sel' : '') + (s.spent ? ' none' : '') + (s.taken ? ' row' : '')); e.appendChild(key());
  if (s.thing === 'pistol' || s.thing === 'rifle') { e.appendChild(mk('span', 'name', s.name)); e.appendChild(mk('span', 'n', String(s.count))); }
  else if (s.spent) e.appendChild(mk('span', 'name', s.name)); else e.appendChild(pips(s));
  return e;
}
/** the box under the strip: the gun that is up, or the cans */
function gunBox(gun: KitGun, open: boolean): HTMLElement {
  const box = mk('div', `hb${open ? ' setup' : ''}`), top = mk('div', 'hb-top');
  const m = mk('span', 'hb-mag' + (gun.reloading ? ' re' : '')); m.appendChild(magazine(gun)); if (gun.reloading) m.appendChild(mk('span', 're', 'RELOADING')); top.appendChild(m);   // (the reload across the pips: what is set stays on the line through it)
  if (gun.ocp) { const oc = mk('span', 'tag ocp' + (gun.ocp.ready ? ' on' : ''), 'OCP'), bar = mk('span', 'ocpbar'), bb = mk('b'); bb.style.width = `${Math.round(gun.ocp.charge * 100)}%`; bar.appendChild(bb); oc.appendChild(bar); top.appendChild(oc); }
  top.appendChild(state(gun));
  const n = mk('span', 'hb-n' + (gun.dry ? ' dry' : '')); n.appendChild(mk('b', '', String(gun.ready)));
  if (gun.loose === null) { n.appendChild(mk('i', 'of', `/${gun.capacity}`)); top.appendChild(n); top.appendChild(spares(gun)); }   // (31/30 and a box a spare magazine)
  else { n.appendChild(mk('i', '', `/ ${gun.loose}`)); top.appendChild(n); }   // (a gun fed a shell at a time carries loose shells, and keeps the count of them)
  box.appendChild(top);
  return box;
}
function canBox(s: KitSlot): HTMLElement {
  const box = mk('div', 'hb'), top = mk('div', 'hb-top'), n = mk('span', 'hb-n');
  top.appendChild(pips(s)); n.appendChild(mk('b', '', `×${s.count}`)); top.appendChild(n); box.appendChild(top);
  return box;
}

/** the kit's column as the box and its strip: `el` is #equip, emptied and drawn again (the caller keys the redraw). A game no host steps has no places, and no column */
export function drawHandBox(el: HTMLElement, kit: KitView): void {
  styles(); el.textContent = ''; if (!kit.slots.length) return;
  el.classList?.add('hand'); el.classList?.add('pop');   // (classList: the headless stand-in's elements have none)
  // condition (and the key taken off a questioned man), the goggles
  const cond = mk('div', 'cond' + (kit.down ? ' down' : '') + (kit.god ? ' god' : ''));
  if (kit.god) cond.appendChild(mk('span', 'g on', 'GOD MODE'));
  else { cond.appendChild(mk('span', 'g', kit.down ? 'DOWN — ENTER' : 'CONDITION')); const hits = mk('span', 'pips'); for (let i = 0; i < kit.hitsMax; i++) hits.appendChild(mk('i', i < kit.hitsLeft ? '' : 'lost')); cond.appendChild(hits); }
  if (kit.nightVision !== null) { const g = mk('span', 'g' + (kit.nightVision ? ' on' : ''), 'N NIGHT VISION'); g.style.marginLeft = '10px'; cond.appendChild(g); }
  if (kit.keyTag) cond.appendChild(mk('span', 'keytag', kit.keyTag));
  el.appendChild(cond);
  const strip = mk('div', 'strip h6'), open = !!kit.setup, up = kit.slots.find(s => s.up);
  el.classList?.toggle('setup', open);
  for (const s of kit.slots) strip.appendChild(slotOf(s));
  el.appendChild(strip);
  if (kit.gun) el.appendChild(gunBox(kit.gun, open));
  else if (up) el.appendChild(up.thing === 'hands' ? mk('div', 'hb empty') : canBox(up));
  const standing = kit.setup ? popup(kit.setup) : null;
  if (standing) { if (!popUp) standing.classList?.add('in'); el.appendChild(standing); }   // (above the column: it is placed off the column's own top edge, so nothing under it moves)
  popUp = !!standing;
  if (kit.big) {   // the big slot: worked at a door on the door's own key
    const sb = mk('div', 'slot big' + (kit.big.offered ? ' sel' : ''));
    sb.appendChild(mk('span', 'key', 'SPACE')); sb.appendChild(mk('span', 'name', kit.big.name));
    sb.appendChild(mk('span', '', kit.big.place)); el.appendChild(sb);
  }
}
