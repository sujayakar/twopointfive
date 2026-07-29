import { DEFAULT_SETTINGS, RenderSettings } from "../engine/renderer";

// ---------------------------------------------------------------------------
// Settings persistence.
//
// Without this the game opens every session at defaults tuned on one machine
// in one room, which is a bad first thirty seconds: the player either sees a
// black screen or spends them in the debug panel. Calibration in particular is
// worth asking for exactly once and then never again.
//
// The stored blob is advisory. Anything the current build does not recognise
// is dropped, and anything it does recognise is only accepted if the type
// matches the default, so an old or hand-edited entry degrades to defaults
// rather than poisoning the renderer with a string where it wants a number.
// ---------------------------------------------------------------------------

const KEY = "twopointfive.settings";
/** Superseded by the blob below; still read once so early testers keep their value. */
const LEGACY_EXPOSURE_KEY = "twopointfive.exposure";

/**
 * Bumping this discards every stored blob.
 *
 * Only needed when a field's *meaning* changes under a name it already had —
 * unknown and mistyped fields are already dropped on load, so adding or
 * removing settings does not need a bump.
 */
const VERSION = 1;

/**
 * Bumped when a setting's *meaning* changes rather than its name.
 *
 * Discarding the whole blob would be heavy-handed — it would throw away a
 * brightness calibration the player was asked to sit through — so instead the
 * keys listed below are dropped from any blob written before this revision and
 * fall back to the current default.
 */
const REVISION = 2;

/**
 * Keys whose stored value is stale as of REVISION.
 *
 * `volumetric` because it used to scale one beam and now scales every torch in
 * the level: a value dialled in against the player's flashlight alone reads as
 * fog once four guards are also casting shafts.
 */
const STALE_KEYS: (keyof RenderSettings)[] = ["volumetric"];

/**
 * Modes, not preferences.
 *
 * Restoring these would be actively hostile: `reference` pins the renderer to
 * a slow ground-truth path, and coming back to a debug view or a stuck night
 * vision tube reads as the game being broken rather than as a setting.
 */
const VOLATILE: (keyof RenderSettings)[] = ["debugView", "reference", "nightVision"];

interface Stored {
  version: number;
  /** See REVISION. Absent on blobs written before it existed. */
  revision?: number;
  /** Set once the player has been through calibration, however they left it. */
  calibrated: boolean;
  settings: Partial<RenderSettings>;
}

function read(): Stored | null {
  let raw: string | null = null;
  try {
    raw = localStorage.getItem(KEY);
  } catch {
    // Private mode, or storage disabled. Not worth failing a game over.
    return null;
  }
  if (raw === null) return null;
  try {
    const p = JSON.parse(raw) as Stored;
    if (!p || typeof p !== "object" || p.version !== VERSION) return null;
    if (typeof p.settings !== "object" || p.settings === null) return null;
    return p;
  } catch {
    return null;
  }
}

/** True when the player has never completed calibration on this browser. */
export function needsCalibration(): boolean {
  const s = read();
  if (s) return !s.calibrated;
  // An early tester who already calibrated under the old key should not be
  // asked again.
  try {
    return localStorage.getItem(LEGACY_EXPOSURE_KEY) === null;
  } catch {
    return true;
  }
}

/**
 * Copies stored values over `into`, in place.
 *
 * Mutates rather than returning a fresh object because everything downstream
 * — the panel's getters, the frame loop, the adaptive resolution controller —
 * closes over the one settings object created at startup.
 */
export function loadInto(into: RenderSettings): void {
  const stored = read();
  if (!stored) {
    try {
      const legacy = Number(localStorage.getItem(LEGACY_EXPOSURE_KEY));
      if (Number.isFinite(legacy) && legacy > 0) into.exposure = legacy;
    } catch { /* ignore */ }
    return;
  }
  const defaults = DEFAULT_SETTINGS as unknown as Record<string, unknown>;
  const target = into as unknown as Record<string, unknown>;
  const stale = (stored.revision ?? 0) < REVISION ? STALE_KEYS : [];
  for (const [k, v] of Object.entries(stored.settings)) {
    if (stale.includes(k as keyof RenderSettings)) continue;
    // hasOwnProperty, not `in`. JSON.parse makes "__proto__" a real own
    // property, Object.entries hands it over, and `"__proto__" in defaults` is
    // true by inheritance — so `in` let it through and the assignment below
    // replaced the settings object's prototype outright. Verified before the
    // fix: Object.getPrototypeOf(settings) came back as the attacker's object.
    if (!Object.prototype.hasOwnProperty.call(defaults, k)) continue;
    if (VOLATILE.includes(k as keyof RenderSettings)) continue;
    // Type must match the default, so a corrupted entry cannot smuggle a
    // string into a slot the uniform packer expects to be a number.
    if (typeof v !== typeof defaults[k]) continue;
    if (typeof v === "number" && !Number.isFinite(v)) continue;
    target[k] = v;
  }
}

function persistable(s: RenderSettings): Partial<RenderSettings> {
  const out: Record<string, unknown> = {};
  for (const [k, v] of Object.entries(s)) {
    if (VOLATILE.includes(k as keyof RenderSettings)) continue;
    out[k] = v;
  }
  return out as Partial<RenderSettings>;
}

/**
 * Writes settings back, coalescing bursts.
 *
 * Driven by polling from the frame loop rather than by hooking every setter:
 * settings are mutated from the debug panel, the quality presets, the
 * brightness slider and the adaptive resolution controller, and a dirty check
 * catches all four without threading a callback through any of them. The
 * adaptive controller in particular retunes `resolutionScale` continuously,
 * which is exactly the case the delay exists for.
 */
export class SettingsPersister {
  private last = "";
  private dueAt = 0;
  private calibrated: boolean;

  /** Milliseconds a change must hold still before it is written. */
  private static readonly DELAY = 900;

  constructor() {
    this.calibrated = !needsCalibration();
  }

  /** Records that calibration has been seen, so it is not shown again. */
  markCalibrated(settings: RenderSettings): void {
    if (this.calibrated) return;
    this.calibrated = true;
    this.write(settings);
  }

  /** Call once per frame; cheap when nothing has changed. */
  poll(settings: RenderSettings, nowMs: number): void {
    const snap = JSON.stringify(persistable(settings));
    if (snap !== this.last) {
      this.last = snap;
      this.dueAt = nowMs + SettingsPersister.DELAY;
      return;
    }
    if (this.dueAt !== 0 && nowMs >= this.dueAt) {
      this.dueAt = 0;
      this.write(settings);
    }
  }

  private write(settings: RenderSettings): void {
    const blob: Stored = {
      version: VERSION,
      revision: REVISION,
      calibrated: this.calibrated,
      settings: persistable(settings),
    };
    try {
      localStorage.setItem(KEY, JSON.stringify(blob));
    } catch { /* quota or private mode; the game still runs */ }
  }
}

/** Clears everything, so the next load is a first run again. */
export function resetSettings(): void {
  try {
    localStorage.removeItem(KEY);
    localStorage.removeItem(LEGACY_EXPOSURE_KEY);
  } catch { /* ignore */ }
}
