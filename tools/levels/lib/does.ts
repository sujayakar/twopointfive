// WHAT A BOX DOES, as a level's generators ask it: the row of the table of materials its word takes (lib/materials.ts, written from Rust; the rules ask the same of the
// same rows: rust/game_types/src/scene_materials, which says what each column is and why a box says two words).
import { DOES_WORDS, type DoesWord } from '../../../src/draw/names';
import { MATERIALS, type Does } from './materials';

/** THE DEFAULT ROW's word: what a box that says nothing does. Never written on a box or in a file where nothing else speaks: absent means this */
export const DOES_DEFAULT: DoesWord = DOES_WORDS[0];
export const doesWordOk = (v: unknown): v is DoesWord => typeof v === 'string' && Object.hasOwn(MATERIALS, v);
/** the columns of a word (the default row's for a word the table does not have: a file opened leniently can carry any string) */
export const doesOfWord = (w: string | undefined): Readonly<Does> => MATERIALS[doesWordOk(w) ? w : DOES_DEFAULT];
/** WHAT A BOX DOES: its own word's row, else the default row */
export const doesOf = (b: { does?: string } | null | undefined): Readonly<Does> => doesOfWord(b?.does);
/** the word a box carries for `w`: nothing for the default row (absent = default everywhere a box is kept) */
export const doesSaid = (w: string | undefined): { does?: DoesWord } => w !== undefined && w !== DOES_DEFAULT && doesWordOk(w) ? { does: w } : {};
