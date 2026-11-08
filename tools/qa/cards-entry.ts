// What tools/qa/cards-page.ts loads: the start page alone, with no engine and no level stood, after the lines of the page's boot that the cards read.
import { keysBoot } from '../../src/host/keymap';
import { openStartPage, startPage } from '../../src/ui/startpage';
import { Desk, desk, setDesk, type DeskFace } from '../../src/view/desk';
import { Commands } from '../../src/draw/commands';
import { machineBefore } from '../../src/host/machine';

const q = new URLSearchParams(location.search);
keysBoot(q);
{ const { instance } = await WebAssembly.instantiate(await (await fetch('/game_face.wasm')).arrayBuffer(), { env: { face_roll: () => 0 } });
  const X = instance.exports as unknown as DeskFace & { memory: WebAssembly.Memory }, C = new Commands();
  setDesk(new Desk(X, X.memory, (key, text) => { if (text) localStorage.setItem(key, text); else localStorage.removeItem(key); }));
  machineBefore(q, '', C); desk().do(C); }
document.getElementById('menu')?.classList.remove('on');
openStartPage();
Object.assign(window, { __cards: { page: startPage, open: openStartPage } });
