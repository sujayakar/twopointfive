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
