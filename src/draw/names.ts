// Written by the program `half` (rust/game_start/src/bin/half) from an enum's words (`Words`). To change a name, change it there.

/** a sound played once, which the game asks for or the mixer picks, by its number in Rust */
export const SFX = ['pistolSuppressed', 'pistolLoud', 'carbineReport', 'rifleSuppressed', 'launcher', 'dryFire', 'magOut', 'magIn', 'rack', 'magDrop', 'footSoft', 'footHard', 'ocp', 'nvOn', 'nvOff', 'click', 'throw', 'canisterBounce', 'bulletImpact', 'bodyHit', 'bodyFall', 'lightBreak', 'lightOff', 'deny', 'equip', 'smokePop', 'alertSting', 'radio', 'doorCreak', 'doorBang', 'doorLatch', 'doorBash', 'doorRattle', 'lockPick', 'lockOpen', 'lockBreak', 'powerDown', 'powerUp', 'stunBang', 'earRing', 'propScrape', 'propThud', 'grabScuffle', 'chokeStrain', 'chokeLimp', 'whipThud', 'shove', 'murmur', 'heldProtest', 'whistle', 'stairDoor', 'stairBoots', 'casingTink', 'thunderNear', 'thunderFar', 'glassCrack', 'glassTinkle', 'glassShatter', 'glassCrunch', 'padBeep', 'padOpen', 'alarmChirp', 'wallCrack', 'wallThump', 'wallDebris', 'wallOn', 'wallOff', 'grabCatch', 'gunStrip', 'gunDown', 'mk12Suppressed', 'mp7Suppressed', 'mk46Report', 'm1014Report', 'm870Report', 'hk45Suppressed', 'g18Report', 'beltCoverUp', 'beltBoxOff', 'beltBoxOn', 'beltLay', 'beltCoverDown', 'chargeLmg', 'shellIn', 'boltDrop', 'chairRoll', 'fireBed', 'fireCrack'] as const;
export type SfxName = (typeof SFX)[number];

/** a sound that goes on, by its number in Rust */
export const LASTINGS = ['chairRoll', 'fireBed', 'smokeHiss', 'hum', 'servers', 'buzz', 'wind'] as const;
export type LastingName = (typeof LASTINGS)[number];

/** a key the game asks for, by its code, by its number in Rust */
export const KEYS = ['KeyW', 'KeyA', 'KeyS', 'KeyD', 'KeyQ', 'KeyE', 'KeyR', 'KeyF', 'KeyC', 'KeyL', 'KeyZ', 'KeyV', 'KeyN', 'KeyB', 'KeyT', 'Digit0', 'Digit1', 'Digit2', 'Digit3', 'Digit4', 'Digit5', 'Digit6', 'Digit7', 'Digit8', 'Digit9', 'Space', 'ShiftLeft', 'ShiftRight', 'Enter', 'NumpadEnter', 'Tab', 'ArrowUp', 'ArrowDown', 'BracketLeft', 'BracketRight', 'F5', 'Blur', 'Paused', 'Thawed'] as const;
export type GameKey = (typeof KEYS)[number];

/** what a box may say it is made of, by its number in Rust */
export const STUFFS = ['carpet', 'grass', 'rubber', 'metal', 'glass', 'wood', 'plastic', 'fabric', 'steel', 'enamel', 'tarmac', 'mud wall', 'plaster', 'block', 'earth', 'gravel', 'slab', 'sheet', 'cloth', 'paint', 'veneer', 'carpaint', 'stucco', 'skim', 'gloss block', 'board concrete', 'cast panel', 'paver', 'brushed', 'louvre', 'plain', 'waits'] as const;
export type StuffName = (typeof STUFFS)[number];

/** what a particle is, by its number in Rust */
export const PKINDS = ['rain', 'dust', 'casing', 'sheet', 'spark', 'debris', 'shard'] as const;
export type PKindName = (typeof PKINDS)[number];

/** how particles are thrown, by its number in Rust */
export const EMIT_SHAPES = ['cone', 'burst', 'ring', 'puff', 'rainDisc'] as const;
export type EmitShapeName = (typeof EMIT_SHAPES)[number];

/** what smoke comes from, by its number in Rust */
export const SMOKE_KINDS = ['flash', 'wisp', 'canister', 'burst', 'cloud', 'jet', 'trail', 'vent', 'puff', 'breath'] as const;
export type SmokeKindName = (typeof SMOKE_KINDS)[number];

/** what a floor is, by its number in Rust */
export const FLOORS = ['carpet', 'lino', 'stone', 'tile', 'asphalt', 'grass'] as const;
export type FloorName = (typeof FLOORS)[number];

/** how the air is pushed, by its number in Rust */
export const PUSH_SHAPES = ['point', 'line', 'body', 'jet', 'outward', 'round', 'stride'] as const;
export type PushShapeName = (typeof PUSH_SHAPES)[number];

/** which of the smoke's lattices take it, by its number in Rust */
export const LATTICES = ['every', 'fine', 'coarse'] as const;
export type LatticeName = (typeof LATTICES)[number];

/** what a source of moving air is, by its number in Rust */
export const WIND_KINDS = ['jet', 'storm'] as const;
export type WindKindName = (typeof WIND_KINDS)[number];

/** what a light is by day, by its number in Rust */
export const LIGHT_CLASSES = ['none', 'fixture', 'hand', 'vehicle', 'flash', 'face'] as const;
export type LightClassName = (typeof LIGHT_CLASSES)[number];

/** whose table a man wears, by its number in Rust */
export const SIDES = ['civilian', 'operator', 'defender', 'responder'] as const;
export type SideName = (typeof SIDES)[number];
