// The headless page of the object bench: window.judgeObject(name, { fault, quick, parts })
import { OBJECTS } from './objects/index';
import { FAULTS } from './objects/faults';
import { judge, type Opts } from './objects/judge';
import { judged } from './objects/marks';
(globalThis as never as { judgeObject: unknown }).judgeObject = async (name: string, o: Opts & { fault?: string } = {}) => { const kind = OBJECTS[name]; if (!kind) throw new Error('no object ' + name); const f = o.fault ? FAULTS.find(q => q.name === o.fault) : undefined; if (o.fault && !f) throw new Error('no fault ' + o.fault); const m = f ? f.plant(kind.make()) : kind.make(), r = await judge(m, o); return { ...r, fault: f ? { name: f.name, what: f.what, caught: f.caught } : null, found: r.found.map(q => ({ ...q, ...judged(q, m.levels[0]!.shape) })) }; };
