// WHAT OPTIONS SETS OF A GAME IN THE MODULE (ui/options.ts OptionsGame): read of the view's row, set by a command done at once, the card it is set on holding the world still.
import type { OptionsGame } from '../ui/options';
import type { DrawGame } from './drawgame';
import { Section, View } from '../draw/sections';
import { keepAutosave } from './sandbox';

export function settingsOf(game: DrawGame): OptionsGame {
  const D = game.drawn, C = game.commands, set = (word: View): boolean => D.u[D.at[Section.view] + word] !== 0, n = (v: boolean): number => v ? 1 : 0;
  let hints = true;
  return {
    get god() { return set(View.god); }, set god(v) { C.god(n(v)); game.tell(); },
    get thinks() { return set(View.thinks); }, set thinks(v) { C.ai(n(v)); game.tell(); },
    get ammo() { return set(View.ammo); }, set ammo(v) { C.ammo(n(v)); game.tell(); },
    get autosave() { return set(View.autosave); }, set autosave(v) { keepAutosave(v); C.autosave(n(v)); game.tell(); },
    get wallFeel() { return set(View.wallFeel); },
    get wallPress() { return set(View.wallPress); }, set wallPress(v) { game.turn('wallsnap.press', n(v)); game.tell(); },
    get fallIn() { return set(View.fallIn); },
    get hints() { return hints; }, set hints(v) { hints = v; },
  };
}
