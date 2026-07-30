(async () => {
  const nav = window.__nav;
  const rows = [];
  for (let iz = 0; iz < nav.h; iz++) {
    let s = "";
    for (let ix = 0; ix < nav.w; ix++) s += nav.blocked[iz * nav.w + ix] ? "#" : ".";
    rows.push(s);
  }
  const from = [-20.08, -5.65], to = [-24.82, -15.75];
  const startCell = nav.nearestOpen(from[0], from[1]);
  const goalCell = nav.nearestOpen(to[0], to[1]);
  const path = nav.findPath(from[0], from[1], to[0], to[1]);
  const from2 = [-11.0, 0.0], to2 = [-24.8, -15.7];
  const path2 = nav.findPath(from2[0], from2[1], to2[0], to2[1]);
  return {
    w: nav.w, h: nav.h, minX: nav.minX, minZ: nav.minZ,
    startCell, goalCell,
    startCenter: startCell >= 0 ? nav.cellCenter(startCell) : null,
    goalCenter: goalCell >= 0 ? nav.cellCenter(goalCell) : null,
    pathLen: path ? path.length : null, path,
    path2Len: path2 ? path2.length : null, path2,
    grid: rows,
  };
})()
