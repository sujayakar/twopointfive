// THE MENU'S QUESTIONS (blitz/skeleton/CHANGES-14.md): an order row in the Rust game is data, an arm for each closure with its body behind `order_can` and the like. So on a puppet every row's closures, and the squad's posture,
// are these: each asks the Rust game by the row's ID and what it asked holds the target as bytes by the schema. No TypeScript registration runs on the puppet, and the menu has one source of truth.
//   const asks = installAsks(g, knobs, ask, reapply);   ask = (kind, row, seat, bytes) => bytes (the face's face_ask, io::ask); reapply = what the host does after a question that writes
//   asks.after();                                       after every apply: a row or a posture the apply made gets its closures
//   asks.give(S, id, target?, who?, opts?);             an order from outside a tick, the target as bytes. asks.routeGive() makes it every seat's `S.give`
//   A question that writes (issue, given, give) calls `handFirst` before it and `reapply` after it. handFirst hands over what the host has changed since the last step (its cameras above all): an apply outside a step otherwise writes back the cameras as Rust last had them, and the view jumps a frame
import { bookOf, exportPart } from './export';

export const ASK = { label: 0, can: 1, only: 2, valid: 3, when: 4, gone: 5, word: 6, mark: 7, given: 8, issue: 9, give: 10 } as const;
const TARGET = 'crate::orders::OrderTarget', MEN = 'Vec<GuardId>', OPTS = 'Option<crate::orders::GiveOpts>';
const DEC = new TextDecoder();

/** A number is the face's refusal (`face_ask` answered it: 1 no such order row, 2 no such seat, 3 no such question): no index was taken in the Rust game */
export type AskFn = (kind: number, row: number, seat: number, bytes: Uint8Array) => Uint8Array | number;
class Answer { p = 0; dv: DataView; constructor(readonly b: Uint8Array) { this.dv = new DataView(b.buffer, b.byteOffset, b.byteLength); }
  u8() { return this.b[this.p++]; } u32() { const v = this.dv.getUint32(this.p, true); this.p += 4; return v; }
  str() { const L = this.u32(); const s = DEC.decode(this.b.subarray(this.p, this.p + L)); this.p += L; return s; }
  optStr() { return this.u8() ? this.str() : null; } }

const ASK_NAMES = Object.keys(ASK);
const strBytes = (s: string) => { const u = new TextEncoder().encode(s), o = new Uint8Array(4 + u.length); new DataView(o.buffer).setUint32(0, u.length, true); o.set(u, 4); return o; };
const someBytes = (s: string) => { const t = strBytes(s), o = new Uint8Array(1 + t.length); o[0] = 1; o.set(t, 1); return o; };
/** what a closure reads as "nothing", question by question: the label is the row's own name, no man can, none fit, the row is not offered and is gone, an order through it is refused */
export function askNothing(kind: number, name: string): Uint8Array {
  switch (kind) {
    case ASK.label: return strBytes(name);
    case ASK.can: return someBytes('not available');
    case ASK.only: return new Uint8Array(4);
    case ASK.valid: return Uint8Array.of(1);
    case ASK.when: return Uint8Array.of(0);
    case ASK.gone: return Uint8Array.of(1);
    case ASK.word: return Uint8Array.of(0);
    case ASK.mark: return strBytes('');
    case ASK.given: return new Uint8Array(0);
    default: return someBytes('not available');
  } }

export function installAsks(g: any, knobs: Record<string, unknown>, ask0: AskFn, reapply: () => void, handFirst: () => void = () => {}) {
  const bk = bookOf(g); if (!bk) throw new Error('ask: this game has never been exported, so no row has an ID: export it once first');
  // A REFUSAL IS ANSWERED HERE: the puppet holds a row the Rust game has not got (or a seat, or a question it has no arm for). One console line a row
  const refused = { n: 0, rows: {} as Record<string, string> };
  const ask = (kind: number, row: number, seat: number, b: Uint8Array): Uint8Array => { const a = ask0(kind, row, seat, b); if (typeof a !== 'number') return a; refused.n++;
    const d = a === 1 ? (bk.listOf('OrdersOrderDefId') as any[])[row] : undefined, name = String(d?.name ?? d?.id ?? ''), key = `${a}/${a === 2 ? seat : row}`;
    if (!refused.rows[key]) { refused.rows[key] = `${ASK_NAMES[kind] ?? kind}${name ? ` of '${name}'` : ''}`; console.warn(`the Rust game has no ${a === 2 ? `seat ${seat}` : a === 3 ? `question ${kind}` : `order row ${row}${name ? ` ('${name}')` : ''}`}: '${ASK_NAMES[kind] ?? kind}' answered as nothing`); }
    return askNothing(kind, name); };
  const rowId = (d: object) => bk.idOf('OrdersOrderDefId', d), seatId = (s: object) => bk.idOf('SeatId', s), rows = () => bk.listOf('OrdersOrderDefId') as any[], guards = () => bk.listOf('GuardId') as any[];
  const bytes = (...p: [string, unknown][]) => exportPart(g, knobs, p);
  /** whose menu: the seat whose squad is `S`, or that lists the row (a label is asked with no S) */
  const seatOfS = (S: any) => { const s = g.seats.find((x: any) => x.view.orders === S); if (!s) throw new Error('ask: no seat has that squad'); return seatId(s); };
  const seatOfRow = (d: any) => { const s = g.seats.find((x: any) => x.view.orders.defs.includes(d)); return seatId(s ?? g.seats[0]); };
  const closures = (d: any) => { const r = () => rowId(d);
    d.label = (t: any) => new Answer(ask(ASK.label, r(), seatOfRow(d), bytes([TARGET, t]))).str();
    d.can = (S: any, gd: any, t: any) => new Answer(ask(ASK.can, r(), seatOfS(S), bytes(['GuardId', gd], [TARGET, t]))).optStr() ?? true;
    d.only = (S: any, men: any[], t: any) => { const a = new Answer(ask(ASK.only, r(), seatOfS(S), bytes([MEN, men], [TARGET, t]))); const n = a.u32(), L = guards(), out: any[] = []; for (let i = 0; i < n; i++) out.push(L[a.u32()]); return out; };
    d.valid = (S: any, t: any, men: any[]) => { const a = new Answer(ask(ASK.valid, r(), seatOfS(S), bytes([TARGET, t], [MEN, men]))); const k = a.u8(); return k === 0 ? true : k === 1 ? false : a.str(); };
    d.gone = (S: any, t: any, men: any[]) => new Answer(ask(ASK.gone, r(), seatOfS(S), bytes([TARGET, t], [MEN, men]))).u8() !== 0;
    d.issue = (S: any, men: any[], t: any, o?: any) => { handFirst(); const a = new Answer(ask(ASK.issue, r(), seatOfS(S), bytes([MEN, men], [TARGET, t], [OPTS, o]))); reapply(); return a.optStr() ?? undefined; }; };
  const when = (d: any) => (S: any, t: any, men: any[]) => new Answer(ask(ASK.when, rowId(d), seatOfS(S), bytes([TARGET, t], [MEN, men]))).u8() !== 0;
  const postureOf = (seat: any) => { const s = seatId(seat); return { word: () => new Answer(ask(ASK.word, 0, s, new Uint8Array(0))).optStr(), mark: (def: any) => new Answer(ask(ASK.mark, rowId(def), s, new Uint8Array(0))).str(), given: (def: any) => { handFirst(); ask(ASK.given, rowId(def), s, new Uint8Array(0)); reapply(); } }; };
  const done = new WeakSet<object>(), whens = new WeakMap<object, unknown>(), postures = new WeakSet<object>(); let seen = 0;
  const after = () => { const L = rows(); for (; seen < L.length; seen++) if (!done.has(L[seen])) { done.add(L[seen]); closures(L[seen]); }
    for (const d of L) if (d.standsIn && (!d.standsIn.when || d.standsIn.when !== whens.get(d))) { const w = when(d); whens.set(d, w); d.standsIn.when = w; }   // (`standsIn` is a value: an apply can make it anew)
    for (const s of g.seats) { const S = s.view.orders; if (S.posture && !postures.has(S.posture)) { const P = postureOf(s); postures.add(P); S.posture = P; } } };
  /** an order that did not come from a row's `issue`: S.give, by the row's id, the target as bytes by the schema. Answers the refusal, or null */
  const give = (S: any, id: string, t?: any, who?: any, o?: any): string | null => { handFirst(); const a = new Answer(ask(ASK.give, 0, seatOfS(S), bytes(['String', id], ['Option<' + TARGET + '>', t], ['Option<crate::orders::Who>', who], [OPTS, o]))); reapply(); return a.optStr(); };
  /** every seat's `S.give` goes to the Rust game from here on, so what prices an order (orders.ts: the floor the team has seen) is Rust's and need not be on the puppet */
  const routeGive = () => { for (const s of g.seats) { const S = s.view.orders; S.give = (id: string, t?: any, who?: any, o?: any) => give(S, id, t ?? S.targetAt(g.aimPoint), who, o); } };
  after(); return { after, give, routeGive, refused }; }
