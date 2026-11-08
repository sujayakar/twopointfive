// WHAT OF THE SNOW IS UP ON THIS PAGE: each layer on as the page opens, each with the word that puts it off on the address (the shaders are compiled with some of them, so they are said once).
export const SNOW_LOOK = { on: true, wisps: true, tracks: true, glow: true, sparkle: true, breath: true, steam: true, grains: true, blast: true, oneAir: true, marks: true, past: true, under: true };
export function snowLookOfQuery(q: URLSearchParams): void {
  const L = SNOW_LOOK, on = (w: string) => q.get(w) !== '0';
  L.on = on('snowlook'); L.wisps = on('snowwisps'); L.tracks = on('snowtracks'); L.glow = on('snowglow'); L.sparkle = on('snowsparkle'); L.grains = on('snowgrains'); L.oneAir = on('snowair');
  L.marks = on('snowmarks'); L.under = on('snowunder'); L.past = on('snowpast'); L.blast = on('snowblast'); L.steam = on('breathsteam'); L.breath = on('snowbreath');
}
