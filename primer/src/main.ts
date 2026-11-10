import { Lab } from './lab';
import { setKnob } from './page/knobs';
import { Page } from './page/page';

async function start(): Promise<void> {
  const app = document.getElementById('app')!;
  const canvas = document.createElement('canvas');
  const errors: string[] = [];
  const onError = (text: string) => {
    if (errors.length < 5) errors.push(text);
    console.error(text);
  };
  let lab: Lab;
  try {
    lab = await Lab.open(canvas, onError);
  } catch (error) {
    app.innerHTML = '';
    const sorry = document.createElement('p');
    sorry.id = 'sorry';
    sorry.textContent = error instanceof Error ? error.message : String(error);
    app.append(sorry);
    return;
  }
  const page = new Page(lab, app);
  Object.assign(window, { lab, page, setKnob });
  const next = () => {
    try {
      if (!(window as { holdFrames?: boolean }).holdFrames) lab.frame();
    } catch (error) {
      onError(String(error));
    }
    requestAnimationFrame(next);
  };
  requestAnimationFrame(next);
}
void start();
