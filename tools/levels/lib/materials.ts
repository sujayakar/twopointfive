// Written by the program `half` (rust/game_start/src/bin/half) from rust/game_types/src/scene_materials/types.rs: THE TABLE OF MATERIALS, what a thing does, a row a word. To change a row, change it there.
import type { DoesWord } from '../../../src/draw/names';

export type Surf = 'carpet' | 'metal' | 'glass' | 'wood' | 'plastic' | 'cardboard' | 'drywall' | 'concrete' | 'fabric' | 'foliage';
export type BreakClass = 'hard' | 'metal' | 'wood' | 'board' | 'soft';
export type DoesWall = 'board' | 'screen';
export type DoesCover = 'wood' | 'cardboard';
export type SnowKind = 'snow' | 'snowPacked' | 'snowOld';
export interface Does { surf: Surf; breaks: BreakClass; steel: boolean; wall: DoesWall | null; board: boolean; cover: DoesCover | null; structure: boolean; counter: boolean; casing: boolean; paper: boolean; screen: boolean; snow: SnowKind | null }

export const MATERIALS: Readonly<Record<DoesWord, Readonly<Does>>> = {
  'unnamed': { surf: 'concrete', breaks: 'metal', steel: false, wall: null, board: false, cover: null, structure: false, counter: false, casing: false, paper: false, screen: false, snow: null },
  'wood': { surf: 'wood', breaks: 'wood', steel: false, wall: null, board: false, cover: 'wood', structure: false, counter: false, casing: false, paper: false, screen: false, snow: null },
  'steel': { surf: 'metal', breaks: 'metal', steel: true, wall: null, board: false, cover: null, structure: false, counter: false, casing: false, paper: false, screen: false, snow: null },
  'enamel': { surf: 'metal', breaks: 'metal', steel: false, wall: null, board: false, cover: null, structure: false, counter: false, casing: false, paper: false, screen: false, snow: null },
  'fabric': { surf: 'fabric', breaks: 'soft', steel: false, wall: null, board: false, cover: null, structure: false, counter: false, casing: false, paper: false, screen: false, snow: null },
  'carpet': { surf: 'carpet', breaks: 'hard', steel: false, wall: null, board: false, cover: null, structure: true, counter: false, casing: false, paper: false, screen: false, snow: null },
  'grass': { surf: 'carpet', breaks: 'hard', steel: false, wall: null, board: false, cover: null, structure: true, counter: false, casing: false, paper: false, screen: false, snow: null },
  'tarmac': { surf: 'concrete', breaks: 'hard', steel: false, wall: null, board: false, cover: null, structure: true, counter: false, casing: true, paper: false, screen: false, snow: null },
  'plastic': { surf: 'plastic', breaks: 'metal', steel: false, wall: null, board: false, cover: null, structure: false, counter: false, casing: false, paper: false, screen: false, snow: null },
  'plasterboard': { surf: 'drywall', breaks: 'hard', steel: false, wall: 'board', board: true, cover: null, structure: true, counter: false, casing: false, paper: false, screen: false, snow: null },
  'partition': { surf: 'drywall', breaks: 'hard', steel: false, wall: 'screen', board: true, cover: null, structure: true, counter: false, casing: false, paper: false, screen: false, snow: null },
  'ceiling': { surf: 'drywall', breaks: 'hard', steel: false, wall: null, board: true, cover: null, structure: true, counter: false, casing: false, paper: true, screen: false, snow: null },
  'laminate': { surf: 'drywall', breaks: 'hard', steel: false, wall: null, board: true, cover: null, structure: true, counter: true, casing: false, paper: true, screen: false, snow: null },
  'pane': { surf: 'drywall', breaks: 'hard', steel: false, wall: null, board: true, cover: null, structure: true, counter: false, casing: false, paper: true, screen: false, snow: null },
  'concrete': { surf: 'concrete', breaks: 'hard', steel: false, wall: null, board: false, cover: null, structure: true, counter: false, casing: false, paper: false, screen: false, snow: null },
  'stone': { surf: 'concrete', breaks: 'hard', steel: false, wall: null, board: false, cover: null, structure: true, counter: false, casing: false, paper: false, screen: false, snow: null },
  'lino': { surf: 'concrete', breaks: 'hard', steel: false, wall: null, board: false, cover: null, structure: true, counter: false, casing: false, paper: false, screen: false, snow: null },
  'dark carpet': { surf: 'carpet', breaks: 'hard', steel: false, wall: null, board: false, cover: null, structure: true, counter: false, casing: true, paper: false, screen: false, snow: null },
  'card': { surf: 'cardboard', breaks: 'board', steel: false, wall: null, board: false, cover: 'cardboard', structure: false, counter: false, casing: false, paper: false, screen: false, snow: null },
  'casing': { surf: 'metal', breaks: 'hard', steel: false, wall: null, board: false, cover: null, structure: false, counter: false, casing: true, paper: false, screen: false, snow: null },
  'frame': { surf: 'metal', breaks: 'hard', steel: false, wall: null, board: false, cover: null, structure: true, counter: false, casing: false, paper: false, screen: false, snow: null },
  'screen': { surf: 'glass', breaks: 'hard', steel: false, wall: null, board: false, cover: null, structure: false, counter: false, casing: true, paper: false, screen: true, snow: null },
  'plant': { surf: 'foliage', breaks: 'soft', steel: false, wall: null, board: false, cover: null, structure: false, counter: false, casing: false, paper: false, screen: false, snow: null },
  'snow': { surf: 'concrete', breaks: 'hard', steel: false, wall: null, board: false, cover: null, structure: true, counter: false, casing: false, paper: true, screen: false, snow: 'snow' },
  'packed snow': { surf: 'concrete', breaks: 'hard', steel: false, wall: null, board: false, cover: null, structure: true, counter: false, casing: false, paper: false, screen: false, snow: 'snowPacked' },
  'old snow': { surf: 'concrete', breaks: 'hard', steel: false, wall: null, board: false, cover: null, structure: true, counter: false, casing: false, paper: false, screen: false, snow: 'snowOld' },
  'unnamed grey': { surf: 'concrete', breaks: 'metal', steel: true, wall: null, board: false, cover: null, structure: false, counter: false, casing: false, paper: false, screen: false, snow: null },
  'unnamed dark': { surf: 'metal', breaks: 'metal', steel: false, wall: null, board: false, cover: null, structure: false, counter: false, casing: true, paper: false, screen: false, snow: null },
  'unnamed light': { surf: 'drywall', breaks: 'metal', steel: false, wall: null, board: false, cover: null, structure: false, counter: false, casing: false, paper: false, screen: false, snow: null },
  'unnamed light grey': { surf: 'drywall', breaks: 'metal', steel: true, wall: null, board: false, cover: null, structure: false, counter: false, casing: false, paper: false, screen: false, snow: null },
  'unnamed pale': { surf: 'drywall', breaks: 'metal', steel: false, wall: null, board: false, cover: null, structure: false, counter: false, casing: false, paper: true, screen: false, snow: null },
  'unnamed pale grey': { surf: 'drywall', breaks: 'metal', steel: true, wall: null, board: false, cover: null, structure: false, counter: false, casing: false, paper: true, screen: false, snow: null },
  'dark plastic': { surf: 'plastic', breaks: 'metal', steel: false, wall: null, board: false, cover: null, structure: false, counter: false, casing: true, paper: false, screen: false, snow: null },
  'grey plastic': { surf: 'plastic', breaks: 'metal', steel: true, wall: null, board: false, cover: null, structure: false, counter: false, casing: false, paper: false, screen: false, snow: null },
  'dark glass': { surf: 'glass', breaks: 'metal', steel: false, wall: null, board: false, cover: null, structure: false, counter: false, casing: true, paper: false, screen: false, snow: null },
};
