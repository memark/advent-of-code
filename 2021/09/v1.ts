import {
  includes,
  sum,
} from "https://raw.githubusercontent.com/selfrefactor/rambda/master/dist/rambda.esm.js";
import * as R from "https://raw.githubusercontent.com/selfrefactor/rambda/master/dist/rambda.esm.js";
import { green } from "https://deno.land/std@0.117.0/fmt/colors.ts";

const printSolution = (part, answer) => console.info(green(part), "\t", answer);

const log = console.debug;

const useExample = 0;
const filename = useExample ? "example.txt" : "input.txt";
const data = Deno.readTextFileSync(filename);
const heights = data.split("\n").map((l) =>
  l.split("").map((p) => parseInt(p))
);
const h = heights;

const yl = heights.length;
const xl = heights[0].length;

{
  let lowCoords: [number, number][] = [];

  for (let y = 0; y < yl; y++) {
    for (let x = 0; x < xl; x++) {
      const v = h[y][x];
      const ns: number[] = [];
      if (y - 1 >= 0) ns.push(h[y - 1][x]);
      if (x + 1 < xl) ns.push(h[y][x + 1]);
      if (y + 1 < yl) ns.push(h[y + 1][x]);
      if (x - 1 >= 0) ns.push(h[y][x - 1]);
      if (ns.every((n) => v < n)) {
        lowCoords = [...lowCoords, [x, y]];
      }
    }
  }

  const lowNumbers = lowCoords.map(([x, y]) => heights[y][x]);
  const riskLevels = lowNumbers.map((n) => n + 1);
  const answer = sum(riskLevels);

  printSolution("part1", answer); // 15, 498
}

type Coord = [number, number];

let visited: Coord[] = [];

{
  let basins: Coord[][] = [];

  for (let y = 0; y < yl; y++) {
    for (let x = 0; x < xl; x++) {
      basins = [...basins, findBasin([y, x])];
    }
  }

  const basinLengths = basins.map((b) => b.length).filter((l) => l > 0).sort((
    a,
    b,
  ) => b - a);

  const [b0, b1, b2, ..._rest] = basinLengths;
  const answer = b0 * b1 * b2;

  printSolution("part2", answer); // 1134, 1071000
}

function findBasin(c: Coord): Coord[] {
  const [y, x] = c;
  const v = h[y][x];

  if (includes(c, visited)) {
    return [];
  }

  visited = [...visited, c];

  if (v == 9) {
    return [];
  }

  let newer: Coord[] = [];

  if (y - 1 >= 0) {
    const n = [y - 1, x] as Coord;
    if (!includes(n, visited)) {
      const r = findBasin(n);
      newer = [...newer, ...r];
    }
  }
  if (x + 1 < xl) {
    const n = [y, x + 1] as Coord;
    if (!includes(n, visited)) {
      const r = findBasin(n);
      newer = [...newer, ...r];
    }
  }
  if (y + 1 < yl) {
    const n = [y + 1, x] as Coord;
    if (!includes(n, visited)) {
      const r = findBasin(n);
      newer = [...newer, ...r];
    }
  }
  if (x - 1 >= 0) {
    const n = [y, x - 1] as Coord;
    if (!includes(n, visited)) {
      const r = findBasin(n);
      newer = [...newer, ...r];
    }
  }

  return [c, ...newer];
}
