// A REPLAY THAT CANNOT BE PLAYED HERE, said over a page that has nothing else to show: why, and the way back.
export function refusedCard(why: string): void {
  const card = document.createElement('div'), say = document.createElement('div'), back = document.createElement('a');
  card.id = 'taperefuse'; card.style.cssText = 'position:fixed;inset:0;z-index:90;display:flex;flex-direction:column;align-items:center;justify-content:center;gap:22px;padding:24px;box-sizing:border-box;background:var(--h-base,#0c0b09);color:var(--h-ink,#d2c9b5);font:18px/1.5 var(--h-font,sans-serif);text-align:center';
  say.textContent = why; say.style.maxWidth = '46ch'; back.textContent = '‹ Main menu'; back.href = location.pathname; back.style.cssText = 'color:var(--h-hi,#f0e9d9);border:1px solid rgba(233,226,214,.3);border-radius:3px;padding:8px 18px;text-decoration:none;font-size:15px';
  card.append(say, back); document.body.append(card); back.focus();
}
