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
];
