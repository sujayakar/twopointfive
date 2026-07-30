// Cold-start sanity: the probe reads a real number, the nav raster is neither
// empty nor solid, and four guards exist. Everything else has its own scenario.
(async () => {
  window.__pause(true);
  window.__renderer.resize(384, 240);
  await window.__renderStill(6, 50);
  const nav = window.__nav;
  let blocked = 0;
  for (let i = 0; i < nav.blocked.length; i++) blocked += nav.blocked[i];
  const dbg = Array.from(window.__renderer.probeDebug).map((x) => +x.toFixed(3));
  const guards = window.__detection.snapshot();
  const failures = [];
  if (!(dbg[0] > 0)) failures.push(`probe sees ${dbg[0]} lights`);
  if (!(blocked > 0 && blocked < nav.blocked.length)) failures.push(`nav raster degenerate: ${blocked}/${nav.blocked.length} blocked`);
  if (guards.length !== 4) failures.push(`expected 4 guards, got ${guards.length}`);
  return {
    ok: failures.length === 0, failures,
    probeDebug: dbg,
    visibility: { level: +window.__visibility.level.toFixed(3), lux: +window.__visibility.illuminance.toFixed(4) },
    nav: { w: nav.w, h: nav.h, cells: nav.blocked.length, blocked },
    guards,
  };
})()
