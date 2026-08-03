import { GroupSpec } from "../ui/panel";
import { SYNTHETIC, buildIndirectBox } from "../scene/demo";
import { DemoDeps, bootDemo, sl, tg } from "./common";

// ---------------------------------------------------------------------------
// /demo/indirect — a colour-bleed box for looking at bounce light on its own.
//
// The reported artefact is "shine the torch at a nearby wall and the surfaces
// around it light up in patches that come and go". That gesture is the whole
// interaction here: walk up to a wall, sweep the beam, and watch the floor.
// Set the debug view to "indirect only" and nothing else is in the frame.
//
// The auto-sweep exists because the artefact is about a MOVING light and a
// hand on a mouse is not a repeatable stimulus — two runs at different slider
// settings need the beam to travel the same arc at the same rate.
// ---------------------------------------------------------------------------

const sweep = { on: false, degrees: 55, period: 4 };

function groups(d: DemoDeps): GroupSpec[] {
  return [
    {
      title: "repro",
      items: [
        tg("auto-sweep beam", () => sweep.on, (v) => (sweep.on = v)),
        sl("sweep width (deg)", 5, 90, 1,
          () => sweep.degrees, (v) => (sweep.degrees = v)),
        sl("sweep period (s)", 0.5, 12, 0.25,
          () => sweep.period, (v) => (sweep.period = v)),
        // The medium is off by default here: fog and smoke both scatter, and
        // in-scatter along the beam reads a lot like bounce light on a wall.
        // Anything blamed on indirect should be reproducible with this at 0.
        sl("ambient fog", 0, 1, 0.05,
          () => d.settings.fogAmount, (v) => (d.settings.fogAmount = v)),
      ],
    },
  ];
}

void bootDemo({
  // ?scene=furnace|onePanel|manyStrips|bleed — the ladder in scene/demo.ts.
  // A query param rather than a runtime switch because materials and the BVH
  // are baked at renderer init; swapping scenes means a reload either way.
  build: SYNTHETIC[new URLSearchParams(location.search).get("scene") ?? ""]
    ?? buildIndirectBox,
  groups,
  help: "WASD walk · right-drag orbit · wheel zoom · F torch · ` panel\n"
    + "set view to \"indirect only\" · atrous passes 0 shows the raw field",

  // Fog and smoke both scatter, and in-scatter along the beam reads a lot like
  // bounce light on a wall. Start with the medium out of the picture so
  // anything blamed on indirect has to survive without it.
  init(d) {
    d.settings.fogAmount = 0;
    // The default exposure is calibrated for the office, which is mostly dark
    // concrete. This room is small, white and close to the torch, so it clips
    // to flat white at that setting and the bounce — the entire subject — is
    // the first thing lost.
    d.settings.exposure = 0.09;
    // The shared default is framed for the smoke hall. This room is 8 m across
    // and closed, so the camera has to stay inside its x/z footprint or it
    // looks at the unlit outside of a wall: at distance 8 the pitch must keep
    // the horizontal radius (8*cos(pitch) = 3.6 m) under the 4 m half-extent.
    // The height that buys is above the ceiling, which is why the ceiling is
    // FLAG_NO_CAMERA — camera rays pass through it and see the room below.
    d.camera.distance = 8;
    d.camera.pitch = 1.1;
    d.focus.x = 0;
    d.focus.z = 0;
  },

  step(d, _dt, elapsed) {
    if (!sweep.on) return;
    // Overrides the cursor aim set by the shared step: a fixed arc across the
    // -x (red) wall, so the same patches are crossed on every run.
    const half = (sweep.degrees * Math.PI) / 360;
    const a = Math.sin((elapsed / sweep.period) * Math.PI * 2) * half;
    const yaw = Math.PI + a;
    d.torch.dir = {
      x: Math.sin(yaw),
      y: -0.35,
      z: Math.cos(yaw),
    };
    const l = Math.hypot(d.torch.dir.x, d.torch.dir.y, d.torch.dir.z) || 1;
    d.torch.dir = {
      x: d.torch.dir.x / l, y: d.torch.dir.y / l, z: d.torch.dir.z / l,
    };
  },
});
