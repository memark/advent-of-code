// @deno-types="https://raw.githubusercontent.com/selfrefactor/rambda/master/index.d.ts"
import {
  clone,
  equals,
  filter,
  flatten,
  inc,
  includes,
  length,
  map,
  median,
  not,
  pipe,
  sum,
} from "https://raw.githubusercontent.com/selfrefactor/rambda/master/dist/rambda.esm.js";
import * as R from "https://raw.githubusercontent.com/selfrefactor/rambda/master/dist/rambda.esm.js";
import { bold, green, red } from "https://deno.land/std@0.117.0/fmt/colors.ts";

const printSolution = (part, answer) => console.info(green(part), "\t", answer);

const log = console.debug;

const useExample = 1;
const filename = useExample ? "example.txt" : "input.txt";
const data = Deno.readTextFileSync(filename);
const lines = data.split("\n").map((l) => l.split("").map((c) => parseInt(c)));

// log(lines);
// log();
log("Before any steps");
log(formatGrid(lines));
log();

type Grid = number[][];
type Coord = [number, number];

// let temp = clone(lines);

// let flashes = 0;

// array0toN(steps).forEach((step) => {
//   temp = doOneStep(temp, step);
//   log("After step", step + 1, { flashes });
//   log(formatGrid(temp));
//   log();
// });

const ns = [
  [-1, 0],
  [-1, +1],
  [0, +1],
  [+1, +1],
  [+1, 0],
  [+1, -1],
  [0, -1],
  [-1, -1],
] as Coord[];

function doFlashes(grid: Grid /*, step: number*/) {
  // Then, any octopus with an energy level greater than 9 flashes. This increases the energy level of all adjacent octopuses by 1, including octopuses that are diagonally adjacent. If this causes an octopus to have an energy level greater than 9, it also flashes. This process continues as long as new octopuses keep having their energy level increased beyond 9. (An octopus can only flash at most once per step.)
  const valid = (
    i,
    j,
  ) => (0 <= i && i < lines.length && 0 <= j && j < lines[0].length);

  let flashed: number[][] = [];

  const flash = (grid: Grid) => {
    const newGrid = clone(grid);
    array0toN(newGrid.length).forEach((i) =>
      array0toN(newGrid[0].length).forEach((j) => {
        if (newGrid[i][j] > 9 && !includes([i, j], flashed)) {
          flashed = [...flashed, [i, j]];

          // Kan man göra detta med zipWith?
          const ncs = ns.map((n) => [i + n[0], j + n[1]]).filter((n) =>
            valid(n[0], n[1])
          );
          // log({ ncs });
          ncs.filter((n) => newGrid[n[0]][n[1]] <= 9).forEach((n) =>
            newGrid[n[0]][n[1]] = newGrid[n[0]][n[1]] + 1
          );
          // grid[i][j] = 0;
        }
      })
    );
    return newGrid;
  };

  let prev = grid;
  let grid3 = grid;
  do {
    prev = grid3;
    log(formatGrid(prev));
    log();
    grid3 = flash(prev);
  } while (not(equals(prev, grid3)));

  return grid3;
}

const steps = 100;

{
  let sum = 0;
  array0toN(steps).reduce((prev, curr) => {
    // First, the energy level of each octopus increases by 1
    const grid2 = map(map(inc), prev);

    log("Begin step", curr + 1);
    const grid3 = doFlashes(grid2);
    const flashes = length(filter((n: number) => n > 9, flatten(grid3)));

    // Finally, any octopus that flashed during this step has its energy level set to 0, as it used all of its energy to flash.
    const grid4 = map(map((x) => x > 9 ? 0 : x), grid3);

    // log({ flashes });
    sum += flashes;
    log("After step", curr + 1);
    log(formatGrid(grid4));
    log();
    return grid4;
  }, lines);
  printSolution("part1", sum); // 1656, 1743
}

{
  // printSolution("part2", answer); // 195, 364
}

function formatGrid(grid: Grid) {
  const res = grid.map((l) =>
    l.map((n) => n.toString(16).toUpperCase()).map((s) =>
      s === "0" ? bold(s) : s
    ).join("")
  ).join("\n");
  return res;
}

function array0toN(n: number) {
  return [...Array(n).keys()];
}
