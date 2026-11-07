// LOOK ONLY — whether destruction dust is drawn as dust. What is dust, and in what colour, rides with the smoke PER CELL (SmokeEmitter.tint → smoke.ts emit → smoke.wgsl's K word →
// the atlas texel's .yzw → render/shaders/dust_look.wgsl, in the view march only). Nothing the solver, the scene's shadow rays or the light probe's sight lines read
// comes from here, and with no tinted puff in the air the march reads smoke as plain smoke (the frame's dust bit is up only while some brick holds dust: SmokeBinding.dusty).

/** A/B only (no panel row; `?nodustlook` on the page's URL, read once): false = no tint kept and the frame's dust bit never up, = the picture as it was */
export const DUST_DRAW = { on: typeof location !== 'undefined' && !/[?&]nodustlook\b/.test(location.search) };   // (the page only)
