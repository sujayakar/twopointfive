// The stand's holds, as src/host/standup.ts has them: a row the frozen stand keeps at its old value, with the word that raises it.
import { K, N } from '../game/k';
const envOf = (): Record<string, string | undefined> => (globalThis as { process?: { env?: Record<string, string | undefined> } }).process?.env ?? {};
export function pinned(env?: string): boolean { return (envOf().TPS_PROFILE ?? 'frozen') === 'frozen' || (env !== undefined && envOf()[env] !== undefined); }
export function standGame(): void {
  if (pinned('TPS_ON')) K.on = envOf().TPS_ON === '1';   // the rule: off, as every stored run had it
  if (pinned('TPS_EXTRA')) K.extra = envOf().TPS_EXTRA === '1';
  if (pinned('TPS_KEEP')) K.stay = envOf().TPS_KEEP === '1';
  if (pinned()) N.reach = 3;                              // the old reach
  if (pinned()) N.wide = 2;
}
