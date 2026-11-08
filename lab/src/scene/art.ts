// The signs, drawn once into one atlas: white on clear, colored where they are used.
export const ART = ['04', '07', '12', 'A', 'B', 'R', 'arrow', 'arrowUp', 'stripes', 'ring', 'cross', 'exit', 'atrium', 'studio', 'keep', 'bar', 'dash', 'corner', 'hex', 'runner', 'L1', 'L2', 'hvac', 'caution', 'grid', 'line', 'half', 'dots', 'wave', 'slab', 'plus', 'fade', ...Array.from({ length: 56 }, (_, i) => 'p' + i), 'corner24', 'noodle', 'bar9', 'hotel', 'karaoke', 'pharmacy', 'neonArrow', 'parking', 'drinks', 'poster0', 'poster1', 'poster2', 'poster3', 'price', 'mag0', 'mag1', 'mag2', 'mag3', 'push', 'zebra', 'menu', 'lantern', 'noEntry', 'shutter'] as const;
const CELLS = 16, ROWS = 8, PX = 256;
export function artRect(name: string): [number, number, number, number] { const i = Math.max(0, ART.indexOf(name as never)); return [(i % CELLS) / CELLS, Math.floor(i / CELLS) / ROWS, 1 / CELLS, 1 / ROWS]; }
export function drawArt(): HTMLCanvasElement {
  const c = document.createElement('canvas'); c.width = CELLS * PX; c.height = ROWS * PX; const x = c.getContext('2d')!;
  ART.forEach((name, i) => {
    x.save(); x.translate((i % CELLS) * PX, Math.floor(i / CELLS) * PX); x.beginPath(); x.rect(2, 2, PX - 4, PX - 4); x.clip(); x.fillStyle = x.strokeStyle = '#fff'; x.textAlign = 'center'; x.textBaseline = 'middle';
    const text = (t: string, size: number, y = 128, weight = 900) => { x.font = `${weight} ${size}px "Helvetica Neue", Arial, sans-serif`; x.fillText(t, 128, y + size * 0.03, 246); };
    const big = (t: string) => text(t, t.length > 1 ? 210 : 260, 132);
    if (/^p\d+$/.test(name)) { label(x, Number(name.slice(1))); x.restore(); return; } if (sign(x, name)) { x.restore(); return; }
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

// ---- print: every package its own colors, word and picture. All of it invented
const rnd = (seed: number) => { let s = seed * 2654435761 >>> 0 || 1; return () => { s ^= s << 13; s >>>= 0; s ^= s >>> 17; s ^= s << 5; s >>>= 0; return s / 4294967296; }; };
const WORDS = ['MIRO', 'ZEST', 'Kumo', 'ORBIT', 'Pika!', 'EMBER', 'Lumi', 'Moss', 'TANGO', 'Halo', 'YUZU', 'Onda', 'RUSK', 'Bark', 'SUNNY', 'Deli Go', 'POLAR', 'Nori Pop', 'KELP', 'Hush', 'VOLT', 'Maple', 'GRIT', 'Tofu-ya', 'Plum', 'REX', 'Cocoa', 'Sora', 'DUNE', 'Fig & Co', 'BOLT', 'Riso'], SUBS = ['hot & crisp', 'sea salt', 'extra rich', 'no sugar', 'with sesame', 'spicy noodle', 'fresh daily', '500 ml', 'family size', 'new taste', 'limited'];
const PAL = [['#e8231f', '#ffd23f', '#fff6e0'], ['#0c7c59', '#f6e27f', '#ffffff'], ['#1b4ea0', '#ffffff', '#ffb703'], ['#f28c0f', '#3a1a05', '#fff3d6'], ['#141414', '#e9c46a', '#e8231f'], ['#f2e9d8', '#b5352c', '#2f4858'], ['#7cc6e8', '#14436b', '#ffffff'], ['#f7b2c4', '#8c1d4c', '#ffffff'], ['#6a3fa0', '#ffe066', '#ffffff'], ['#a8d04a', '#1f4d1a', '#ffffff'], ['#ffffff', '#e8231f', '#1b4ea0'], ['#5b2c16', '#f4d9a8', '#e07a1f']];
function label(x: CanvasRenderingContext2D, k: number) {
  const r = rnd(k + 17), [a, b, c] = PAL[k % PAL.length]!, word = WORDS[k % WORDS.length]!, kind = k % 6; x.fillStyle = a!; x.fillRect(0, 0, 256, 256); const font = (px: number, w = 900) => { x.font = `${w} ${px}px "Arial Black", "Helvetica Neue", Arial, sans-serif`; };
  if (kind === 0 || kind === 3) { x.fillStyle = b!; x.beginPath(); x.ellipse(128, 168, 92, 62, 0, 0, 7); x.fill(); x.fillStyle = c!; for (let i = 0; i < 9; i++) { x.beginPath(); x.ellipse(70 + r() * 116, 140 + r() * 56, 8 + r() * 14, 6 + r() * 8, r() * 3, 0, 7); x.fill(); } }
  if (kind === 1) { x.fillStyle = b!; for (let i = 0; i < 7; i++) x.fillRect(i * 40 - 8, 0, 18, 256); x.fillStyle = a!; x.fillRect(0, 60, 256, 110); }
  if (kind === 2) { x.fillStyle = b!; x.beginPath(); x.moveTo(0, 150); for (let i = 0; i <= 16; i++) x.lineTo(i * 16, 150 + 18 * Math.sin(i * 0.9)); x.lineTo(256, 256); x.lineTo(0, 256); x.fill(); x.fillStyle = c!; x.beginPath(); x.arc(186, 190, 34, 0, 7); x.fill(); }
  if (kind === 4) { x.fillStyle = b!; x.beginPath(); for (let i = 0; i < 24; i++) { const q = i % 2 ? 74 : 116; x.lineTo(128 + q * Math.cos(i * 0.2618), 128 + q * Math.sin(i * 0.2618)); } x.fill(); }
  if (kind === 5) { x.fillStyle = b!; x.fillRect(0, 0, 256, 74); x.fillStyle = c!; for (let i = 0; i < 4; i++) { x.beginPath(); x.arc(48 + i * 54, 186, 24, 0, 7); x.fill(); x.fillStyle = i % 2 ? c! : b!; } }
  x.textAlign = 'center'; x.textBaseline = 'middle'; const y = kind === 5 ? 38 : kind === 4 ? 128 : 70; font(Math.min(74, 400 / word.length)); x.lineWidth = 9; x.lineJoin = 'round'; x.strokeStyle = kind === 4 || kind === 5 ? a! : b!; x.strokeText(word, 128, y, 236); x.fillStyle = kind === 4 ? c! : kind === 5 ? a! : c!; x.fillText(word, 128, y, 236);
  font(22, 700); x.fillStyle = kind === 2 || kind === 0 ? a! : c!; x.fillText(SUBS[Math.floor(r() * SUBS.length)]!, 128, kind === 4 ? 176 : 236, 220); x.fillStyle = '#111'; x.fillRect(206, 214, 40, 26); x.fillStyle = '#fff'; for (let i = 0; i < 9; i++) if (r() > 0.4) x.fillRect(208 + i * 4, 216, 2, 22);
}
function sign(x: CanvasRenderingContext2D, n: string): boolean {
  const wide = (aspect: number, f: () => void) => { x.save(); x.translate(128, 128); x.scale(1 / aspect, 1); f(); x.restore(); }, T = (t: string, px: number, color: string, y = 0, w = 900) => { x.fillStyle = color; x.font = `${w} ${px}px "Arial Black", "Helvetica Neue", Arial, sans-serif`; x.textAlign = 'center'; x.textBaseline = 'middle'; x.fillText(t, 0, y); }, bg = (c: string) => { x.fillStyle = c; x.fillRect(0, 0, 256, 256); };
  switch (n) {
    case 'corner24': bg('#ffffff'); x.fillStyle = '#12a05c'; x.fillRect(0, 0, 256, 46); x.fillStyle = '#f28c0f'; x.fillRect(0, 210, 256, 46); wide(9, () => { T('CORNER 24', 138, '#e8231f', 4); }); x.fillStyle = '#e8231f'; x.fillRect(6, 60, 16, 136); x.fillRect(234, 60, 16, 136); return true;
    case 'noodle': bg('#b5121b'); wide(0.4, () => { ['N', 'O', 'O', 'D', 'L', 'E'].forEach((c, i) => T(c, 38, '#fff3c4', -105 + i * 42)); }); return true;
    case 'bar9': bg('#10142a'); wide(1.6, () => { T('BAR', 120, '#ff5fa8', -50); T('No. 9', 90, '#6fe3ff', 66); }); return true;
    case 'hotel': bg('#1a0f2e'); wide(0.35, () => { ['H', 'O', 'T', 'E', 'L'].forEach((c, i) => T(c, 46, '#ffd36a', -100 + i * 50)); }); return true;
    case 'karaoke': bg('#0f3b8c'); wide(0.38, () => { ['S', 'I', 'N', 'G', '24'].forEach((c, i) => T(c, 44, '#ffffff', -100 + i * 50)); }); return true;
    case 'pharmacy': bg('#063d22'); x.fillStyle = '#4dff9a'; x.fillRect(98, 30, 60, 196); x.fillRect(30, 98, 196, 60); return true;
    case 'neonArrow': bg('#1c0a0a'); x.strokeStyle = '#ffb03a'; x.lineWidth = 22; x.lineJoin = 'round'; x.beginPath(); x.moveTo(40, 60); x.lineTo(40, 150); x.lineTo(150, 150); x.moveTo(120, 100); x.lineTo(196, 150); x.lineTo(120, 200); x.stroke(); return true;
    case 'parking': bg('#1b4ea0'); x.save(); x.translate(128, 128); T('P', 220, '#ffffff', 6); x.restore(); return true;
    case 'drinks': bg('#eef4f8'); for (let j = 0; j < 4; j++) for (let i = 0; i < 7; i++) { const r = rnd(j * 7 + i + 3); x.fillStyle = PAL[Math.floor(r() * PAL.length)]![0]!; x.fillRect(16 + i * 33, 14 + j * 58, 22, 40); x.fillStyle = '#ffffff'; x.fillRect(16 + i * 33, 28 + j * 58, 22, 9); x.fillStyle = j % 2 ? '#e8231f' : '#1b4ea0'; x.fillRect(19 + i * 33, 57 + j * 58, 16, 8); } x.fillStyle = '#333'; x.fillRect(0, 244, 256, 12); return true;
    case 'poster0': case 'poster1': case 'poster2': case 'poster3': { const k = Number(n[6]), p = PAL[(k * 3 + 2) % PAL.length]!, r = rnd(k + 90); bg(p[0]!); x.fillStyle = p[1]!; x.beginPath(); x.arc(128, 104, 74, 0, 7); x.fill(); x.fillStyle = p[2]!; for (let i = 0; i < 6; i++) { x.beginPath(); x.moveTo(128, 104); x.lineTo(60 + r() * 136, 30 + r() * 150); x.lineTo(60 + r() * 136, 30 + r() * 150); x.fill(); } x.save(); x.translate(128, 128); T(['LIVE', 'LOST CAT', 'SALE', 'VOTE'][k]!, 46, p[2]!, 78); x.restore(); return true; }
    case 'price': bg('#f4f4ee'); for (let i = 0; i < 4; i++) { x.fillStyle = i % 3 === 1 ? '#ffe24a' : '#ffffff'; x.fillRect(8 + i * 62, 40, 54, 176); x.fillStyle = '#222'; x.font = '900 46px Arial'; x.textAlign = 'center'; x.fillText(String([120, 98, 340, 215][i]), 35 + i * 62, 120, 50); x.fillStyle = '#d02020'; x.fillRect(14 + i * 62, 166, 42, 10); } return true;
    case 'mag0': case 'mag1': case 'mag2': case 'mag3': { const k = Number(n[3]), p = PAL[(k * 5 + 1) % PAL.length]!; bg(p[2]!); x.fillStyle = p[0]!; x.fillRect(0, 70, 256, 186); x.fillStyle = p[1]!; x.beginPath(); x.ellipse(128, 150, 50, 66, 0, 0, 7); x.fill(); x.fillRect(70, 200, 116, 56); x.save(); x.translate(128, 128); T(['HARBOR', 'CIRCUIT', 'ROOST', 'WEEKLY'][k]!, 50, p[0]!, -92); x.restore(); return true; }
    case 'push': x.fillStyle = '#fff'; x.fillRect(30, 90, 196, 76); x.save(); x.translate(128, 128); T('AUTO', 60, '#12a05c', 4); x.restore(); return true;
    case 'zebra': x.fillStyle = '#fff'; for (let i = 0; i < 6; i++) x.fillRect(i * 44 + 4, 0, 24, 256); return true;
    case 'menu': bg('#241a12'); for (let i = 0; i < 9; i++) { x.fillStyle = '#f0e2c0'; x.fillRect(20, 20 + i * 25, 90 + (i * 37) % 70, 9); x.fillStyle = '#e07a1f'; x.fillRect(200, 20 + i * 25, 36, 9); } return true;
    case 'lantern': x.strokeStyle = '#1a0606'; x.lineWidth = 16; x.lineCap = 'round'; for (let i = 0; i < 3; i++) { x.beginPath(); for (let k = 0; k <= 20; k++) x.lineTo(82 + i * 46 + 12 * Math.sin(k * 0.55), 40 + k * 5); x.stroke(); } x.beginPath(); x.arc(128, 160, 70, 0.1, 3.04); x.closePath(); x.stroke(); return true;
    case 'noEntry': x.fillStyle = '#d4141a'; x.beginPath(); x.arc(128, 128, 120, 0, 7); x.fill(); x.fillStyle = '#fff'; x.fillRect(40, 104, 176, 48); return true;
    case 'shutter': bg('#b9bec2'); x.fillStyle = '#8d949a'; for (let i = 0; i < 16; i++) x.fillRect(0, i * 16, 256, 5); x.fillStyle = '#c23b3b'; x.font = '900 60px Arial'; x.textAlign = 'center'; x.fillText('ZORK', 128, 150); return true;
  }
  return false;
}
