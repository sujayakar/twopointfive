// What tools/qa/cards-page.ts loads: the start page alone, with no engine and no level stood, after the lines of the page's boot that the cards read.
import { keysBoot } from '../../src/host/keymap';
import { pageSwitches } from '../../src/host/profile';
import { initDifficulty } from '../../src/game/difficulty';
import { OFFLIST_UI } from '../../src/ui/missions';
import { wireMissions } from '../../src/ui/loadout';
import { kitBoot } from '../../src/ui/kitBoot';
import { openStartPage, startPage } from '../../src/ui/startpage';

const q = new URLSearchParams(location.search);
keysBoot(q); pageSwitches(q); OFFLIST_UI.on = true; wireMissions(); kitBoot(); initDifficulty(q);
document.getElementById('menu')?.classList.remove('on');
openStartPage();
Object.assign(window, { __cards: { page: startPage, open: openStartPage } });
