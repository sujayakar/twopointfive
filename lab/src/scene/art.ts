// The signs, drawn once into one atlas: white on clear, colored where they are used.
export const ART = ['04', '07', '12', 'A', 'B', 'R', 'arrow', 'arrowUp', 'stripes', 'ring', 'cross', 'exit', 'atrium', 'studio', 'keep', 'bar', 'dash', 'corner', 'hex', 'runner', 'L1', 'L2', 'hvac', 'caution', 'grid', 'line', 'half', 'dots', 'wave', 'slab', 'plus', 'fade'] as const;
const CELLS = 8, PX = 256;
export function artRect(name: string): [number, number, number, number] { const i = Math.max(0, ART.indexOf(name as never)); return [(i % CELLS) / CELLS, Math.floor(i / CELLS) / CELLS, 1 / CELLS, 1 / CELLS]; }
export function drawArt(): HTMLCanvasElement {
  const c = document.createElement('canvas'); c.width = c.height = CELLS * PX; const x = c.getContext('2d')!;
  ART.forEach((name, i) => {
    x.save(); x.translate((i % CELLS) * PX, Math.floor(i / CELLS) * PX); x.beginPath(); x.rect(2, 2, PX - 4, PX - 4); x.clip(); x.fillStyle = x.strokeStyle = '#fff'; x.textAlign = 'center'; x.textBaseline = 'middle';
    const text = (t: string, size: number, y = 128, weight = 900) => { x.font = `${weight} ${size}px "Helvetica Neue", Arial, sans-serif`; x.fillText(t, 128, y + size * 0.03, 246); };
    const big = (t: string) => text(t, t.length > 1 ? 210 : 260, 132);
    switch (name) {
      case '04': case '07': case '12': case 'A': case 'B': case 'R': case 'L1': case 'L2': big(name); break;
      case 'arrow': x.beginPath(); x.moveTo(20, 96); x.lineTo(140, 96); x.lineTo(140, 40); x.lineTo(240, 128); x.lineTo(140, 216); x.lineTo(140, 160); x.lineTo(20, 160); x.fill(); break;
      case 'arrowUp': x.beginPath(); x.moveTo(128, 16); x.lineTo(226, 130); x.lineTo(166, 130); x.lineTo(166, 240); x.lineTo(90, 240); x.lineTo(90, 130); x.lineTo(30, 130); x.fill(); break;
      case 'stripes': for (let k = -4; k < 9; k++) { x.beginPath(); x.moveTo(k * 48, 256); x.lineTo(k * 48 + 24, 256); x.lineTo(k * 48 + 280, 0); x.lineTo(k * 48 + 256, 0); x.fill(); } break;
      case 'ring': x.lineWidth = 26; x.beginPath(); x.arc(128, 128, 100, 0, 7); x.stroke(); break;
      case 'cross': x.fillRect(104, 20, 48, 216); x.fillRect(20, 104, 216, 48); break;
      case 'exit': text('EXIT', 96); x.lineWidth = 10; x.strokeRect(14, 64, 228, 128); break;
      case 'atrium': text('ATRIUM', 62, 100); text('LEVEL 04', 34, 160, 500); break;
      case 'studio': text('STUDIO', 64, 100); text('AUTHORISED ONLY', 24, 156, 500); break;
      case 'keep': text('KEEP CLEAR', 42); x.fillRect(10, 176, 236, 10); x.fillRect(10, 72, 236, 10); break;
      case 'bar': x.fillRect(6, 6, 244, 244); break;
      case 'dash': for (let k = 0; k < 4; k++) x.fillRect(10 + k * 62, 112, 40, 32); break;
      case 'corner': x.fillRect(8, 8, 240, 30); x.fillRect(8, 8, 30, 240); break;
      case 'hex': x.lineWidth = 20; x.beginPath(); for (let k = 0; k < 6; k++) x.lineTo(128 + 100 * Math.cos(k * Math.PI / 3), 128 + 100 * Math.sin(k * Math.PI / 3)); x.closePath(); x.stroke(); break;
      case 'runner': x.beginPath(); x.arc(168, 44, 24, 0, 7); x.fill(); x.lineWidth = 26; x.lineCap = 'round'; x.lineJoin = 'round'; x.beginPath(); x.moveTo(150, 86); x.lineTo(118, 144); x.lineTo(162, 178); x.lineTo(150, 236); x.moveTo(118, 144); x.lineTo(90, 196); x.lineTo(34, 200); x.moveTo(150, 90); x.lineTo(96, 88); x.lineTo(66, 126); x.moveTo(150, 92); x.lineTo(190, 124); x.lineTo(226, 104); x.stroke(); break;
      case 'hvac': text('HVAC-2', 60, 108); text('400V', 34, 164, 500); break;
      case 'caution': x.lineWidth = 18; x.lineJoin = 'round'; x.beginPath(); x.moveTo(128, 26); x.lineTo(236, 222); x.lineTo(20, 222); x.closePath(); x.stroke(); x.fillRect(118, 92, 20, 72); x.fillRect(118, 180, 20, 20); break;
      case 'grid': x.lineWidth = 4; for (let k = 0; k < 9; k++) { x.strokeRect(0, k * 32, 256, 0.1); x.strokeRect(k * 32, 0, 0.1, 256); } break;
      case 'line': x.fillRect(0, 118, 256, 20); break;
      case 'half': x.fillRect(0, 128, 256, 128); break;
      case 'dots': for (let a = 0; a < 8; a++) for (let b = 0; b < 8; b++) { x.beginPath(); x.arc(16 + a * 32, 16 + b * 32, 7, 0, 7); x.fill(); } break;
      case 'wave': x.lineWidth = 16; for (let r = 0; r < 4; r++) { x.beginPath(); for (let k = 0; k <= 64; k++) x.lineTo(k * 4, 40 + r * 58 + 16 * Math.sin(k * 0.4)); x.stroke(); } break;
      case 'slab': text('04', 250, 136); break;
      case 'plus': for (let a = 0; a < 4; a++) for (let b = 0; b < 4; b++) { x.fillRect(26 + a * 64, 30 + b * 64, 12, 4); x.fillRect(30 + a * 64, 26 + b * 64, 4, 12); } break;
      case 'fade': { const g = x.createLinearGradient(0, 0, 0, 256); g.addColorStop(0, '#ffff'); g.addColorStop(1, '#fff0'); x.fillStyle = g; x.fillRect(0, 0, 256, 256); break; }
    }
    x.restore();
  });
  return c;
}
