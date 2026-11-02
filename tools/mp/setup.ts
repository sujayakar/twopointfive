// A SETUP MADE INTO A RECIPE (tools/mp/stand.ts freshRecipe), in a process of its own: prints one line, `MPSETUP <the recipe as JSON>`.
//   bun run tools/mp/setup.ts '{"search":"?roomclear&squad4","seed":7}'
import { freshRecipe, type Setup } from './stand.ts';
const arg = process.argv[2];
if (!arg) { console.error('usage: bun run tools/mp/setup.ts \'{"search":"?roomclear&squad4","seed":7}\''); process.exit(2); }
let setup: Setup; try { setup = JSON.parse(arg) as Setup; } catch (e) { console.error(`setup.ts: the setup is not JSON (${(e as Error).message})`); process.exit(2); }
try { const recipe = await freshRecipe(setup); console.log('MPSETUP ' + JSON.stringify(recipe)); process.exit(0); }
catch (e) { console.error(`setup.ts: ${(e as Error).message}`); process.exit(1); }
