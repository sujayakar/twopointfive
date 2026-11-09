// HudLike — what the frame loop (src/host/drawpage.ts) asks of a HUD, host-neutral: src/ui/overlay.ts's Overlay + bubbles.ts's Bubbles are one
// implementation (the DOM), a native host's display-list overlay / bubbles another. Types only.
import type { HudCam, HudSource } from '../view/hud';

/** the view's size as the game and the HUD read it — an HTMLCanvasElement in the browser, a plain object off the drawable natively (clientWidth / clientHeight in CSS px /
 *  points, width / height the backing store in device px) */
export interface View { clientWidth: number; clientHeight: number; width: number; height: number; }

/** a showcase's chapter list for the timeline (rust/game_rules/src/missions/director chapters()) */
export interface Chapters { list: { title: string; dur: number }[]; i: number; frac: number; }

export interface HudLike {
  /** the showcase caption + sub-line (empty strings = none) */
  setCaption(text: string, sub: string): void;
  /** the showcase timeline (null = none); onPick(k) seeks chapter k */
  setTimeline(chapters: Chapters | null): void;
  /** everything drawn off the game once a frame (markers, meters, rows, tags…) */
  update(src: HudSource, cam: HudCam, view: View): void;
  /** the interactables' light dots (Tab ▸ toggles it) */
  showLightDots: boolean;
}

/** the speech bubbles (ui/bubbles.ts Bubbles): drawn after the HUD. It writes nothing of the game: a line is over when its time is */
export interface BubblesLike { update(src: HudSource, cam: HudCam, view: View): void; }
