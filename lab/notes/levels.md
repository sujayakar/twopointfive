# The contract of an object's levels (draft 1, 9 November)

An object is made by someone who thinks about that object alone, with all its levels. The engine only picks one, by how wide the ray is, and makes what is BETWEEN objects (cells) by itself. So the contract is small, and a bench can test every line of it (notes/objects.md).

```ts
interface Levels {
  version: 1;
  levels: Level[];          // fine to coarse
  gone: number;             // the ray width, in metres, at which it leaves the tree for the world's cells; 0 = never
  parts: boolean;           // false: one level is picked for the whole object, for each ray. true: each part picks its own (a person: hands go before arms)
}
interface Level {
  from: number;             // the ray width at which it takes over, at scale 1 (the first: 0)
  error: number;            // metres, the most its surface stands off the finest's. A ray starts that far off; bench: error <= from / 2
  shape: Pieces | Mesh | Voxels | Cover;
  materials: number[];      // what the shape's parts are made of
  spread: number;           // normals it lost: the mean of 1 - |their mean|. It becomes roughness
  rides: string[];          // for each part, the bone or twig that carries it: fewer, the coarser
  soft: number;             // metres: how far the motion it drops still swings (rms). It thins and widens what it covers
}
// Pieces: solids of the lab (box, planes, curves), each with a range of widths [from, to) of its own if the maker wants finer control than a level
// Mesh:   triangles or child placements, as notes/meshes.md has them
// Voxels: cells in the object's (or a twig's) frame, as the world's cells: thickness along x y z, a tight box, six colors, share of leaves
// Cover:  one such cell
```

How the engine uses it
- The width of a ray is f(t) = w + s t, shifted by up to half an octave either way by ONE die a ray. A level is used where from <= f < the next one's from. So all objects on a ray agree, and there is no seam between levels or objects.
- Past `gone` the object is in the world's cells. They are measured by rays through its coarsest level, from six sides; nothing has to be authored for that.
- What separates space (walls, floors, doors, big panels) has gone = 0. Rooms do not leak because of a level.
- Everything that moves has bounds that hold at every level, so the tree need not know which is in use.

What a maker must keep between neighbouring levels, each one at the width where it takes over, alone on a grey floor
1. what it covers, from six sides and four corners, within 3 %
2. what it sends back under a white sky, and under one lamp from each side, within 2 %, colour by colour: a swap may not be seen as a change of brightness
3. its outline within half of `from`
4. its shadow on the floor and its bounce on a wall, within 2 %
5. its highlight: what is lost in shape is there in roughness
6. `soft` is true: the mean over its motion of the finer level looks as the still coarser one does

Defaults, so that a first version needs no thought: one level of pieces, each piece's range ending at its own middle extent (a wire is thin, a wall is not); gone = the largest of those; spread and soft 0.
