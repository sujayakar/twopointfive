// THE TWO TIERS A LEVEL FILE CARRIES AT THE TAIL OF ITS LISTS, known by name. The game leaves either out on its own word (rust/game_start level.rs: `lived`, `destruction`), so they
// come last and every other box, prop, light and fixture has the index it would have without them. Nothing here leaves one out: a level loaded by a tool is the whole file.
//   LIVED-IN ROOMS   the furniture, clutter, floor pieces, loose props and lamps that make a room of a level ITS room, as the office's rooms are (the maintainer, 10-10: "the office level
//                    is awesome and it has a lot of personality per room, we should makes sure all levels are like that"): a bunk room's bunks, a radio room's bench, a tyre store's stacks.
//                    A BOX whose editor group is 'lived' or starts 'lived.' ('lived.<room>'), a PROP, LIGHT or SWITCHABLE whose id starts 'lived.'
//   THE DRESSING     the boxes that are there only as things to wreck (tools/levels/dress.ts appends them, last of all): the group DRESS_GROUP
const LIVED_GROUP = 'lived';
/** is this box group / prop, light or switchable id one of the lived things? */
export const isLived = (tag: string | undefined | null): boolean => typeof tag === 'string' && (tag === LIVED_GROUP || tag.startsWith(LIVED_GROUP + '.'));
export const DRESS_GROUP = 'dressing';
