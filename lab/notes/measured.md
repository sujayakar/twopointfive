# What has been measured, and what misled

(A10G unless said. Error = median over pixels of the difference from the reference, after the same curve.)

- **Where detail goes** (roof by day, share of the reference's contrast on matte surfaces at 1, 2, 4 px): light before any filter 88, 93, 95 %; after the denoiser 83, 86, 90; after the upscaler 64, 79, 88. Upscaler's kernel since tightened (1 px to about 72).
- **What a bounce sends on** must be the reference's own mean, coat and matte (`returns` in src/wgsl/bounce.wgsl): 4 % too much a bounce made white rooms 7 to 10 % too bright. Mended: roof by day 4.8 to 4.0 % error, rooms by day 4.8 to 3.7. Night and dusk rose by 1 point: another error had been cancelling it (open).
- **The kept light alone**: mean within 3 % of paths over a view, 19 % off entry by entry; rooms nobody looks at are about 9 % dark, because only the camera's bounce rays found entries. The coarse level leaks: without it the roof's mean is 0.036 stops nearer.
- **Direction of the bounce** (its first moment, denoised with it): car paint's shaded side goes from plainly wrong to near the reference. Costs about nothing.
- **The strongest light at a pixel asked by itself**: 0.05 to 0.3 ms, sharper lamp shadows.
- **Leaves**: bounce rays that meet them one by one near by cost 5 ms in the garden and gained 0.03 stops. Torches that met them one by one cost 3 to 5 ms; a raster of them from the torch costs little.
- **Small things in a tree of their own**, met only by rays narrower than they are (8,656 of the street's 13,182 pieces): 0.36 ms of 15, no change in error.
- **A learned upscaler** (a network at every output pixel, lab/learn/pix.py; 1,144 weights, 0.38 ms more at 1080p): on frames it was never shown, 10 % less error and 5 % less flicker than by hand; four constants from the same training gain 6 % of it. In the page against the reference: the network is no better in motion than by hand and flickers 1.7 times as much; the constants gain 6 to 8 % in motion at the same 1.7 times. What both learn is to trust history less. Not worth it as it stands.

Traps
- Two boxes differ by up to a third in time: compare inside one run.
- The upscaler darkens noise (it averages after a curve), so a method without a denoiser looks darker and crisper than it is. Use `rawlin` and `denlin`, which average as light.
- A lamp left on at the origin was in every measure before 9 November: older errors are of another scene.
- The reference's noise counts in the smallest size of detail: read 2 px and up.
- A `//` comment in the middle of a one-line WGSL statement swallows the rest.
- 16 sampled textures and 4 storage textures a stage, with the scene's own 6 counted.
