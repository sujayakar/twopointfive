// What the page leaves every table at, as tools/qa/stand-dump.ts gives it for the game: the tables after the page's switches ran on an empty address.
import { pageSwitches } from '../src/host/profile';
import { knobRegistry } from '../src/game/knobs';
pageSwitches(new URLSearchParams(''));
console.log(JSON.stringify({ tables: knobRegistry() }));
