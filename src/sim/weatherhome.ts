// THE WEATHER THAT PLAY READS, FROM TYPESCRIPT — the host's half of rust/sim/src/wind.rs, weather.rs, air.rs and snow.rs: where the weather's table stands in a game's
// block, and the one seam a check listens at (tools/qa/weather-ident.ts). The rules are the module's and nowhere else; game/weather.ts, wind.ts, wet.ts, snowBog.ts and
// airDust.ts are their thin callers, and what they draw reads the table through them. No game import: like the rest of src/sim, this is below the game.
import { sim, f64 } from './sim';
import { simPart, simTenant, P_WEATHER, type SimState } from './state';

/** the columns of the weather's table, by name (rust/sim/src/weather.rs col) */
export const WX = { live: 0, storm: 1, rainRate: 2, rainNow: 3, wet: 4, thunderUntil: 5, nextFlashAt: 6, lastBarkT: 7, fogOn: 8, fogSigma: 9, fogH: 10, fogLeak: 11, roofsN: 12, roofs: 13, thundersN: 14, thunders: 15, dice: 16,
  airOn: 20, airLeak: 21, airAcc: 22, airCur: 23, airCells: 24, airEdges: 25, airS: 26, airCellList: 27, airEdgeList: 28, airIndoor: 29, airStat: 30, airLeaf: 31, airSig: 32 } as const;
/** the weather's bit among the block's homes (a tenant's: state.ts) */
const WX_BIT = 1 << 10;
const OFF: number[] = [];
/** where column `c` stands, in bytes from the block's own start */
export function wxAt(c: number): number { let o = OFF[c]; if (o === undefined) o = OFF[c] = sim().weather_col(c); return simPart(P_WEATHER) + o; }
export const wxGet = (st: SimState, c: number): number => st.d[wxAt(c) >>> 3];
export const wxSet = (st: SimState, c: number, v: number): void => { st.d[wxAt(c) >>> 3] = v; };
// (always at home: it has no TypeScript state to come from and no row to go back on. A tenant all the same, so that a block is never shut under the table when the last row goes down)
simTenant({ name: 'weather', bit: WX_BIT, wants: () => true, into() {}, outOf() {} });
/** the module's scratch, as doubles of its memory: a question's few numbers in or out (never kept across a call that can allocate: none can) */
export const wxScratch = (): number => sim().sim_scratch() >>> 3;

/** rows of the roofs' table, of the thunder list, and the plan's size: what the module holds, for whoever holds their own to it */
export const wxCap = (i: 0 | 1 | 2 | 3): number => sim().weather_cap(i);

/** how many of `boxes` are roofs at all, by the rule roofsOf makes the table with (a level with more than the table holds is refused as it loads: game/levelfit.ts) */
export function roofsCounted(boxes: readonly { c: readonly number[]; h: readonly number[] }[], ceiling: number): number {
  const x = sim(); x.weather_roofs_begin(); for (const b of boxes) x.weather_roofs_box(b.c[0], b.c[1], b.c[2], b.h[0], b.h[1], b.h[2], ceiling); return x.weather_roofs_seen();
}

/** the module's knob table made current for a question asked of it now (game/rustknobs.ts sets `here`: the list as it stands, unless a tick has handed it already) */
export const WX_KNOBS = { here: (): void => {} };

/** THE SEAM A CHECK LISTENS AT (tools/qa/weather-ident.ts): off, one test at each thin call and nothing else. `said` is handed each answer with what was asked and,
 *  while the TypeScript stands beside the module (`both`), the TypeScript's own answer to the same question; it hands back the answer the game is to use (the same,
 *  unless the check's red path has bent it). Not a rule of play. */
export const WX_WATCH = {
  on: false, both: false,
  said(_name: string, answer: number, _asked: readonly number[], _own: number | null): number { return answer; },
  saidMany(_name: string, _answers: number[], _asked: readonly number[], _own: readonly number[] | null): void {},
};

/** copy `n` doubles from the scratch */
export function wxOut(n: number): number[] { const m = f64(), a = wxScratch(), out: number[] = []; for (let i = 0; i < n; i++) out.push(m[a + i]); return out; }
