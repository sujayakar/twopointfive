// ROUND THINGS ARE ROUND ("Round things into the game": drums, tyres, stoves, the oven).
// A level file says a thing is round (BoxData.round): ONE box that is the cylinder about its own y, radius h[0] (= h[2]), half height h[1].
//
// ONE MODEL — everybody meets the same round:
//   · the picture: the g-buffer's cut pass draws it (a twelve-sided prism standing round the cylinder; its sides shaded with one turning normal, its marks run
//     round it, a rolled rim on its lid — gbuffer_cut.wgsl ROUND);
//   · GPU rays: every ray program traces it as the picture's own prism, through the cut test and the eight rows boxes.ts wrote for the picture (common.wgsl
//     isectBox, the prelude's ROUND_AS_CUT): up to 3.5 % of the radius further out at the twelve corners, the side's normal still the cylinder's;
//   · CPU rays and segments: boxes.ts rayBox, rust/game_rules/src/world/scene_collision.rs line_box (rounds, aim, sight, sound, throw arcs, the cursor);
//   · MOVEMENT: rust/game_rules/src/world/scene_collision.rs collide_circle / circleTouches push a body off the circle, and the walk grid is baked by the same question.
// What still meets the box round it (a square of side 2 × radius, its sides along the world's axes): the trace grid's and the ray grid's registration (a superset:
// it only costs a test), a shoved prop's, a ragdoll's and a piece of debris's own contact, the smoke solver's obstacle bake, the replay's map, and a blast, which
// takes it as that box (rust/game_rules/src/breakage/furnblast.rs). A round thing a round can chew through (rust/game_rules/src/breakage/wallbreak: one in a cover paint) is ONE cell: it goes whole, never in pieces.
