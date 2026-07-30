(async () => {
  window.__renderer.resize(384, 240);
  await window.__renderStill(3);
  const nav = window.__nav;
  let blocked = 0;
  for (let i = 0; i < nav.blocked.length; i++) blocked += nav.blocked[i];
  return {
    probeDebug: Array.from(window.__renderer.probeDebug),
    visibility: { level: window.__visibility.level, lux: window.__visibility.illuminance },
    nav: { w: nav.w, h: nav.h, cells: nav.blocked.length, blocked },
    guards: window.__detection.snapshot(),
  };
})()
