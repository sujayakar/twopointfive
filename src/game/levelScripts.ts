// THE LEVELS THAT CARRY A SCRIPT (director.ts), by the level file's own name: the Game reads it off the level's name as it makes its Director. Each script file is `levels/<name>.script.ts`, imports nothing
// from src at run time, and carries one play program named 'tutorial' that game/rails.ts starts with the mission (a level that is not played on rails never
// starts it: with the frame's switch off the level is its scenario alone).
import type { LevelScriptFn } from './director';
import tutTeam from '../../levels/tut-team.script';
import tutQueue from '../../levels/tut-queue.script';
import tutKit from '../../levels/tut-kit.script';
import tutRoad from '../../levels/tut-road.script';
import tutQuiet from '../../levels/tut-quiet.script';
import tutPair from '../../levels/tut-pair.script';

export const LEVEL_SCRIPTS: Record<string, LevelScriptFn> = { 'tut-team': tutTeam, 'tut-queue': tutQueue, 'tut-road': tutRoad, 'tut-kit': tutKit, 'tut-quiet': tutQuiet, 'tut-pair': tutPair };
