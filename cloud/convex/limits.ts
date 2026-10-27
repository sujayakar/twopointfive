// What a page may send, and how much of it: every public function checks its arguments against these before it writes. The page has no secret, so
// anybody can call them; the bounds are what keeps one caller from filling the tables with one request.
import { ConvexError } from "convex/values";

export const LIMITS = {
  /** an id (pid, sid): this many characters at most, of these */
  idMax: 64,
  id: /^[A-Za-z0-9][A-Za-z0-9_-]*$/,
  /** a pid is dice, not a name: at least this long */
  pidMin: 16,
  /** short strings (a scenario, a commit, a kind) and long ones (a page's query, a user agent) */
  wordMax: 120,
  lineMax: 2000,
  /** one chunk of tape (a document holds 1 MiB), chunks in one call, the pieces of one visit, the parts of one piece */
  chunkMax: 700 * 1024,
  chunksMax: 64,
  seqMax: 200000,
  partsMax: 64,
  /** a browser's settings as one text */
  prefsMax: 512 * 1024,
  /** telemetry records in one call, and one record's `data` as JSON text */
  eventsMax: 50,
  eventMax: 16 * 1024,
  /** one saved file (cloud/convex/files.ts), and the files of one visit */
  fileMax: 32 * 1024 * 1024,
  filesMax: 64,
};

export const bad = (why: string): never => {
  throw new ConvexError(why);
};
export const pidOk = (pid: string): string =>
  pid.length >= LIMITS.pidMin && pid.length <= LIMITS.idMax && LIMITS.id.test(pid) ? pid : bad("pid: not an id");
export const sidOk = (sid: string): string =>
  sid.length > 0 && sid.length <= LIMITS.idMax && LIMITS.id.test(sid) ? sid : bad("sid: not an id");
export const word = (s: string, what: string, max = LIMITS.wordMax): string =>
  s.length <= max ? s : bad(`${what}: longer than ${max}`);
export const count = (n: number, what: string, max: number, min = 0): number =>
  Number.isInteger(n) && n >= min && n <= max ? n : bad(`${what}: not a whole number in ${min}..${max}`);
/** a record's bag is plain JSON and nothing else: null, true / false, a finite number, a text, a list or a map of those (bytes and big integers
 *  would slip past a bound that measures JSON text), nested no deeper than `depth` */
export const plain = (x: unknown, what: string, depth = 6): void => {
  if (x === null || typeof x === "boolean" || typeof x === "string") return;
  if (typeof x === "number") {
    if (!Number.isFinite(x)) bad(`${what}: not a finite number`);
    return;
  }
  if (depth <= 0 || typeof x !== "object") bad(`${what}: not plain JSON`);
  if (Array.isArray(x)) {
    for (const y of x) plain(y, what, depth - 1);
    return;
  }
  const proto = Object.getPrototypeOf(x);
  if (proto !== Object.prototype && proto !== null) bad(`${what}: not plain JSON`);
  for (const y of Object.values(x as object)) plain(y, what, depth - 1);
};
export const amount = (n: number, what: string): number => (Number.isFinite(n) && n >= 0 ? n : bad(`${what}: not a number`));
