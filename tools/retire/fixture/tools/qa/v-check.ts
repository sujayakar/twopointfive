// A check that stands on another check's rig by importing it: a change to the rig reaches this one (tools/retire/lines.ts run --importers).
import { check } from './t-check.ts';
if ((import.meta as { main?: boolean }).main) await check();
