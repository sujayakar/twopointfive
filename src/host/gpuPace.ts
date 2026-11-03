// A TRIAL, `&pace` ON THE ADDRESS (off without it; never seen on a GPU): THE PAGE WAITS FOR THE GPU BEFORE IT STARTS A FRAME.
// The maintainer on a 120 Hz screen: "despite saying >100 fps it feels kinda laggy". His recordings, the plain build and the look alike: with the GPU at 17 ms a
// frame the display asks for one every 8 ms, nothing makes the page wait, and the gaps between frames go 5, 15, 2, 53 ms: three or four frames queued, then a
// stall. 54 frames a second in bursts, with gaps of 26 ms and more holding four fifths of the time (the HUD's fps, an average of 1 ÷ gap, read 170).
// Under the word a display frame is PASSED OVER WHOLE while `limit` frames are still on the GPU (the queue's onSubmittedWorkDone says when one has left): the
// frame loop's clock is not touched, so the next frame steps the real time since the last. `&pace` = 2 (one being drawn, one waiting behind it: the GPU never
// idles, a frame is at most two old), `&pace=1` = a frame starts only when the last has left the GPU (the freshest picture; the GPU idles until the display's
// next tick). The lasting answer (the ladder's target on a fast display, the HUD's number) is not this file's.
export interface GpuPace { /** asked at the top of a display frame: true = pass this one over */ busy(): boolean; /** after a frame's work is submitted */ sent(): void; /** frames on the GPU now · display frames passed over · frames sent */ readonly inFlight: number; readonly passed: number; readonly frames: number }
export function gpuPace(queue: { onSubmittedWorkDone(): Promise<unknown> }, limit: number): GpuPace {
  let inFlight = 0, passed = 0, frames = 0; const left = () => { inFlight = Math.max(0, inFlight - 1); };
  return {
    busy() { if (inFlight < limit) return false; passed++; return true; },
    sent() { inFlight++; frames++; queue.onSubmittedWorkDone().then(left, left); },   // (a lost device rejects: the frame has left all the same)
    get inFlight() { return inFlight; }, get passed() { return passed; }, get frames() { return frames; },
  };
}
/** the word on the address → how many frames may be on the GPU when another starts (0 = the trial is off) */
export function gpuPaceLimit(q: URLSearchParams): number { if (!q.has('pace')) return 0; const v = q.get('pace'); return v === '0' ? 0 : v === '1' ? 1 : 2; }
