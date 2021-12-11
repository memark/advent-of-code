// @deno-types="https://raw.githubusercontent.com/selfrefactor/rambda/master/index.d.ts"
import {
  clone,
  inc,
  includes,
  median,
  sum,
} from "https://raw.githubusercontent.com/selfrefactor/rambda/master/dist/rambda.esm.js";
import * as R from "https://raw.githubusercontent.com/selfrefactor/rambda/master/dist/rambda.esm.js";
import { bold, green, red } from "https://deno.land/std@0.117.0/fmt/colors.ts";

const printSolution = (part, answer) => console.info(green(part), "\t", answer);

const log = console.debug;

const useExample = 0;
const filename = useExample ? "example.txt" : "input.txt";
const data = Deno.readTextFileSync(filename);
const lines = data.split("\n").map((l) => l.split("").map((c) => parseInt(c)));
// log(lines);
// log();
log("Before any steps");
log(formatGrid(lines));
log();

type Grid = number[][];
const steps = 1195;

let temp = clone(lines);

let flashes = 0;

array0toN(steps).forEach((step) => {
  temp = doOneStep(temp, step);
  log("After step", step + 1, { flashes });
  log(formatGrid(temp));
  log();
});

function doOneStep(grid: Grid, step: number): Grid {
  // First, the energy level of each octopus increases by 1.
  const grid2 = grid.map((l) => l.map(inc));
  // printGrid(grid2);

  // Then, any octopus with an energy level greater than 9 flashes. This increases the energy level of all adjacent octopuses by 1, including octopuses that are diagonally adjacent. If this causes an octopus to have an energy level greater than 9, it also flashes. This process continues as long as new octopuses keep having their energy level increased beyond 9. (An octopus can only flash at most once per step.)
  let flashed: number[][] = [];
  let flashedNow = false;
  let grid3 = clone(grid2);
  let stepFlashes = 0;

  do {
    flashedNow = false;
    for (const i of array0toN(grid.length)) {
      for (const j of array0toN(grid[0].length)) {
        const v = grid3[i][j];
        if (v > 9 && !includes([i, j], flashed)) {
          flashed = [...flashed, [i, j]];
          flashedNow = true;
          flashes++;
          stepFlashes++;

          const ns = [
            [-1, 0],
            [-1, +1],
            [0, +1],
            [+1, +1],
            [+1, 0],
            [+1, -1],
            [0, -1],
            [-1, -1],
          ] as [number, number][];

          const ns2 = ns.map((n) => [i + n[0], j + n[1]]).filter((n) =>
            valid(n[0], n[1])
          );
          // log({ i }, { j }, { ns2 });
          ns2.forEach((n) => grid3[n[0]][n[1]] = grid3[n[0]][n[1]] + 1);
        }
      }
    }
  } while (flashedNow);
  log(flashed.length);
  log(lines.flat(2).length);
  log(stepFlashes);
  if (flashed.length === lines.flat(2).length) {
    throw step;
  }

  // Finally, any octopus that flashed during this step has its energy level set to 0, as it used all of its energy to flash.
  const grid4 = grid3.map((l) => l.map((l) => l > 9 ? 0 : l));
  // log("AFTER STEP");
  // log(formatGrid(grid4));
  // log();

  return grid4;
}

function valid(i, j) {
  return 0 <= i && i < lines.length && 0 <= j && j < lines[0].length;
}

{
  // printSolution("part1", answer); // 1656, 1743
}

{
  // printSolution("part2", answer); // 195, 364
}

function formatGrid(grid: Grid) {
  const res = grid.map((l) =>
    l.map((n) => n === 0 ? bold(n.toString()) : n.toString()).join("")
  ).join("\n");
  return res;
}

function array0toN(n: number) {
  return [...Array(n).keys()];
}
