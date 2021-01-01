const sample_1 = `
.#.
..#
###
`;

const data = sample_1;

const d = data.split("\n").map((s) => s.split(""));

function range(from: number, to: number): number[] {
  return Array.from(Array(Math.floor(to - from))).map((v, k) => from + k);
}

const actives = new Set<Coord>();

for (const y of range(0, d.length)) {
  for (const x of range(0, d[y].length)) {
    if (d[y][x] == "#") {
      actives.add({ x, y, z: 0, w: 0 });
    }
  }
}

// console.log(actives);

const numCycles = 6;

const res = recurse(actives, numCycles);

console.log(res.size);

type Coord = { x: number; y: number; z: number; w: number };

function recurse(actives: Set<Coord>, maxCycles: number, cycle: number = 1) {
  //   console.log("actives", actives);
  if (cycle > maxCycles) return actives;

  const coordsToCheck = new Set<Coord>();
  for (const a of Array.from(actives))
    for (const aa of [a, ...Array.from(getNeighbours(a))])
      coordsToCheck.add(aa);
  //   console.log("coordsToCheck", coordsToCheck);

  const newActives = new Set<Coord>();
  for (const c of Array.from(coordsToCheck))
    if (getNewState(c, actives)) newActives.add(c);
  //   console.log("newActives", newActives);

  console.log(newActives.size);

  return recurse(newActives, maxCycles, cycle + 1);
}

function getNewState(coord: Coord, actives: Set<Coord>) {
  let activeNeighbours = 0;
  for (const n of Array.from(getNeighbours(coord))) {
    // if (actives.has(n)) {
    if (has(n, actives)) {
      //   if (coord.z == 0 && coord.w == 0) console.log(coord, activeNeighbours);
      activeNeighbours++;
    }
  }

  //   if (coord.z == 0 && coord.w == 0) console.log(coord, activeNeighbours);

  // If a cube is active and exactly 2 or 3 of its neighbors are also active, the cube remains active.
  // Otherwise, the cube becomes inactive.
  //   if (actives.has(coord)) return [2, 3].includes(activeNeighbours);
  if (has(coord, actives)) return [2, 3].includes(activeNeighbours);
  // If a cube is inactive but exactly 3 of its neighbors are active, the cube becomes active.
  // Otherwise, the cube remains inactive.
  else return [3].includes(activeNeighbours);
}

function has(coord: Coord, actives: Set<Coord>) {
  for (const a of Array.from(actives))
    if (a.x == coord.x && a.y == coord.y && a.z == coord.z && a.w == coord.w)
      return true;
  return false;
}

function getNeighbours(coord: Coord) {
  const res = new Set<Coord>();
  for (const w of range(coord.w - 1, coord.w + 1 + 1))
    for (const z of range(coord.z - 1, coord.z + 1 + 1))
      for (const y of range(coord.y - 1, coord.y + 1 + 1))
        for (const x of range(coord.x - 1, coord.x + 1 + 1))
          res.add({ x, y, z, w });
  res.delete(coord);
  return res;
}

// const a = [1, 2];
// const b = [1, 2];
// // console.log(a == b);

// const c = new Set([a]);
// console.log(c);
// console.log(c.has(a));
// console.log(c.has(b));
