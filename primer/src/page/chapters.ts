// THE STEPS. Each says what it is about, what it opens with, and which knobs and stages belong to it.

import type { Layout } from '../lab';

export interface Chapter {
  number: number;
  title: string;
  question: string;
  text: string[];
  formula?: string;
  room: string;
  layout: Layout;
  views: string[];
  knobs: string[];
  breaks: string[];
  stages: string[];
}

export const CHAPTERS: Chapter[] = [
  {
    number: 0,
    title: 'The bench',
    question: 'What does each pixel look at?',
    text: [
      'A ray a pixel, P → x. It walks a tree of boxes and meets <b>pieces</b>: a box, cut by planes and by at most one round shape. Each of those is an interval along the ray; the piece is where they overlap.',
      'Everything after this reads one buffer, <code>hit_x</code>, and never asks how x was found. Click a pixel to see all its values. Every field is a view: pick them in the corner of a panel.',
      'Nothing is packed. One declaration of a struct makes the shader’s type, its bytes, these views and the inspector.',
    ],
    room: 'shapes',
    layout: 'four',
    views: ['hit_x.matte', 'hit_x.normal', 'hit_x.position', 'hit_x.steps'],
    knobs: ['render_scale', 'jitter'],
    breaks: [],
    stages: ['first hit'],
  },
  {
    number: 1,
    title: 'The truth',
    question: 'What would infinitely many paths find?',
    text: [
      'One more path a pixel each frame, added to the mean of all before. Nothing is reused, clamped or blurred. It is kept in the same parts as the real-time frame will be, so each buffer to come has a right answer beside it.',
      'Lamps are found two ways at every surface: by aiming, and by running into them along a lobe. Counting both would count twice, so each is weighed by its chance: the two weights add to 1.',
      'Turn <i>Longest path</i> up from 0 to watch light arrive a bounce at a time. In the furnace a white ball sends back exactly what it gets, so it cannot be seen. Mute parts of the sum in the mixer.',
    ],
    formula: 'L(x) = glow + matte · (lamps + sky + 1 bounce + 2 + …) + highlights + mirror',
    room: 'color-box',
    layout: 'four',
    views: ['true picture', 'truth lamps', 'truth one', 'truth two'],
    knobs: ['truth_bounces', 'start_margin', 'exposure', 'tone'],
    breaks: ['break_no_sharing', 'break_no_cosine', 'break_no_margin'],
    stages: ['truth', 'true picture', 'first hit'],
  },
  {
    number: 2,
    title: 'One lamp, one ray',
    question: 'Does x see the lamp?',
    text: [
      'What a lamp would send with nothing in the way is a formula (<code>unblocked</code>: no dice). The one unknown is whether it is seen. So one ray goes to one spot of the lamp, and the answer is 0 or 1: the only noise.',
      'Where the lamp is half hidden the answers disagree: that is the penumbra. Make the lamp larger and it widens; make the lamp tiny and the noise is gone, and so is every soft shadow.',
      'The panel <i>What it sees</i> is the pinned point’s half-sphere as a disc. Its plain mean is the light that lands. The dots are where this frame’s rays went. Change the dice and look at the noise, then freeze them.',
    ],
    formula: 'light(x) = Σ lamps  unblocked · seen',
    room: 'fence',
    layout: 'four',
    views: ['picture', 'still_raw.value', 'still unblocked', 'disc'],
    knobs: ['dice_kind', 'dice_frozen', 'source_still', 'source_changing', 'start_margin'],
    breaks: ['break_no_margin'],
    stages: ['unblocked', 'pick still', 'shadow still', 'sum', 'disc'],
  },
  {
    number: 3,
    title: 'Over time and neighbors',
    question: 'How little noise can one ray a pixel give?',
    text: [
      'The same noise, through several denoisers. Each rung of the rack adds one part to the rung before. The numbers on a panel are measured against the truth: <b>off</b>, how far the mean is wrong; <b>noise</b>, how much single frames still jump.',
      'First let nothing move (Pause): a plain mean wins. Then press Play, or turn the camera, and watch rung 2 smear, rung 3 ghost, rung 4 ring edges with noise, rung 5 trade lag for a little error.',
      'What is denoised is a share from 0 to 1, not light. Switch it to light and see what the filter does to the falloff.',
    ],
    formula: 'kept = kept + (now − kept) / n        noise left = α / (2 − α),  lag ≈ 1 / α frames',
    room: 'fence',
    layout: 'four',
    views: ['rack1', 'rack4', 'rack7', 'rack true'],
    knobs: ['rack_signal', 'history_frames', 'history_leash', 'blur_rounds', 'blur_soft', 'blur_normal', 'dice_kind', 'denoise_what'],
    breaks: [],
    stages: ['rack7 history', 'rack7 blur 1'],
  },
  {
    number: 4,
    title: 'Many lamps',
    question: 'Which of 60 lamps should one ray ask?',
    text: [
      'Score each lamp by what it would send unblocked, and spend the ray by score. <i>Any</i> ignores the score. <i>Best of a few</i> looks at some and keeps one, weighed so that the mean stays right. <i>Walk all</i> leaves only whether it is seen.',
      'A pixel can keep its find in a <b>reservoir</b>: a lamp, a place on it, a weight W, and how many samples M stand behind it. Two reservoirs merge as one candidate joins. Last frame’s is reuse in time, a neighbor’s is reuse in space. That is all ReSTIR is.',
      'Pin a pixel and open <i>Lamps</i> on the right: what each would send, how much of it is really seen, how often it was picked. In Plates, see when to aim and when to follow the lobe.',
    ],
    formula: 'W = 1/score(kept) · 1/M · Σ score(i)/chance(i)        light = send · seen · W',
    room: 'lamp-hall',
    layout: 'four',
    views: ['picture', 'still_raw.value', 'still lamp', 'disc'],
    knobs: [
      'lamp_pick', 'lamp_candidates', 'reuse_time', 'reuse_space', 'reuse_neighbors', 'reuse_radius', 'reuse_memory',
      'reuse_seen', 'reuse_correct', 'denoise_what', 'shine_by', 'source_still', 'source_changing', 'source_shine',
    ],
    breaks: ['break_no_tests', 'break_no_mean', 'break_count_twice'],
    stages: ['pick still', 'reuse still time', 'reuse still space', 'shadow still', 'remember still', 'shine'],
  },
];
