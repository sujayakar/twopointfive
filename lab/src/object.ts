// The headless page of the object bench: window.judgeObject(name, { fault, quick, parts })
import { OBJECTS } from './objects/index';
import { variant } from './objects/variants';
import { FAULTS } from './objects/faults';
import { judge, type Opts } from './objects/judge';
import { judged } from './objects/marks';
(globalThis as never as { judgeObject: unknown }).judgeObject = async (name: string, o: Opts & { fault?: string } = {}) => { const made = variant(OBJECTS, name); if (!made) throw new Error('no object ' + name); const f = o.fault ? FAULTS.find(q => q.name === o.fault) : undefined; if (o.fault && !f) throw new Error('no fault ' + o.fault); const m = f ? f.plant(made) : made, r = await judge(m, o); return { ...r, fault: f ? { name: f.name, what: f.what, caught: f.caught } : null, found: r.found.map(q => ({ ...q, ...judged(q, m.levels[0]!.shape) })) }; };
