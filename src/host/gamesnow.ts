// THE SNOW OF A PAGE THAT HOLDS A GAME, as the F8 sweep's two rows read it (ui/sweepSnow.ts).
import type { Game } from '../game/game';
import * as snowFx from '../game/snowFx';
import { SNOW_LOOK } from '../game/weatherKinds';
import type { SweptSnow } from '../ui/sweepSnow';

export const snowOf = (g: Game): SweptSnow => ({
  look: SNOW_LOOK, wisps: snowFx.SNOWWISPS,
  snowing: () => snowFx.snowing(g), oneAirOn: () => snowFx.oneAirOn(g), wispsOn: () => snowFx.wispsOn(g), lookByte: () => snowFx.wispLookByte(),
  still: () => { const st = snowFx.wispStats(g); return st.active ? '' : st.why; },
});
