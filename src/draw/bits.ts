// Written by the program `half` (rust/game_start/src/bin/half) from a declaration (`bits!`). To change a bit, change it there.

/** what a box is, as bits of its `flags`: some a level file says, some a rule sets as it makes the box */
export const BoxFlag = {
  /** not rasterized into the G-buffer (ceilings, light-only geometry) */
  NoPrimary: 1,
  /** ignored by all rays (decals, glow cards) */
  NoShadow: 2,
  /** rasterized only below the cut height (a stage drops its back wall to a lip when the orbit camera goes behind it); rays see the full box */
  Cutaway: 4,
  /** rebuilt every frame (character proxies, props on bones) */
  Dynamic: 8,
  /** has non-zero emission (lets a shader early-out) */
  Emissive: 16,
  /** its emission is an analytic area light's: rays ignore the emissive term (no double count, no small-emitter aliasing in the cascades) */
  AreaLit: 32,
  /** carries a full 3D rotation (set as it is packed): raster and rays take the quaternion in place of the packed yaw */
  Rot: 64,
  /** polished floor finish (lobby stone, the lino): the gather traces one Fresnel-weighted reflection ray for on-screen pixels of its top face */
  Gloss: 128,
  /** the rules' alone: a door's leaf near closed, which seals sound */
  SoundSeal: 65536,
  /** a pane: no traced ray and no raster triangle sees it, view and shadow segments are tested against it analytically (reflection, tint, dust glow). Movement and sound treat it as a wall, sight passes it, a round or a kick breaks it */
  Glass: 131072,
  /** the rules' alone: a thin thing that stands free (a stovepipe, a window's bar, a lone post), which the fog of war's own sight passes */
  Thin: 262144,
  /** the rules' alone: a window's bar (the kit's upright between a window's bays): a round that meets one breaks it, and a line of fire passes one */
  Bar: 524288,
} as const;

/** a box's word as a shader reads it (`Geo.flags`): what of its own bits a shader asks, whose it is in the second byte, from bit 18 the first row (from 1) of its marks */
export const enum GeoFlag {
  /** not rasterized */
  NoPrimary = 1,
  /** no ray meets it */
  NoShadow = 2,
  /** rasterized only below its cut height */
  Cutaway = 4,
  /** the cylinder about its own y, radius h.x (to a program with the cut rule, the prism in its rows of the cut table). The box's own word has DYNAMIC here, which no shader asks */
  Round = 8,
  /** has non-zero emission */
  Emissive = 16,
  /** its emission is an analytic area light's */
  AreaLit = 32,
  /** fully rotated: its turn is the quaternion, not the packed yaw */
  Rot = 64,
  /** polished floor finish */
  Gloss = 128,
  /** clipped by the planes in its rows of the cut table. The box's own word has SOUND_SEAL here, which is the rules' alone */
  Cut = 65536,
  /** a pane: never in the trace grid nor the raster */
  Glass = 131072,
}

/** what is so of a broken pane (`Pane.says`) */
export const enum PaneFlag {
  /** something of it has fallen out */
  Open = 1,
  /** its cracks run to the frame */
  Run = 2,
  /** it lets go in clumps */
  Dicing = 4,
  /** all of it has fallen out */
  Gone = 8,
  /** its breaks are a web (a laminated sheet) */
  Web = 16,
}

/** how a game is started */
export const enum StartFlag {
  /** the rows that dress the picture are up */
  Dress = 1,
  /** what `ready` makes is the host's to have made */
  Later = 2,
}

/** what a step does beside its tick */
export const enum StepFlag {
  /** the save controller has its turn */
  Saves = 8,
}
