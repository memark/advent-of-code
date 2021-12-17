// deno-lint-ignore-file no-explicit-any
// @deno-types="https://raw.githubusercontent.com/selfrefactor/rambda/master/index.d.ts"
import * as R from "https://raw.githubusercontent.com/selfrefactor/rambda/master/dist/rambda.esm.js";
import { bold, green, red } from "https://deno.land/std@0.117.0/fmt/colors.ts";

const log = console.debug;
const printSolution = (part, answer) => console.info(green(part), "\t", answer);

const splitBy = (sep: string) => (s: string) => s.split(sep);
const myParseint = (s: string) => parseInt(s);
const index = (n: number) => (a: [] | [number, number]) => a[n];
const myMax = (n: number[]) => Math.max(...n);
const myMin = (n: number[]) => Math.min(...n);
const array0toN = (n: number) => [...Array(n).keys()];
const parseBin = (s: string) => parseInt(s, 2);
const parseHex = (s: string) => parseInt(s, 16);

const fileNumber = 0;

const files = ["input.txt", "example.txt"];
const file = Deno.readTextFileSync(files[fileNumber]);

const m = /target area: x=(-?\d+)\.\.(-?\d+), y=(-?\d+)\.\.(-?\d+)/g.exec(file);
const [x1, x2, y1, y2] = m?.slice(1)!.map(myParseint)!;

// The probe's x,y position starts at 0,0.
const startPos = [0, 0];

const doTrajectory = (pos, speed) => {
  let newPos = [...pos];
  let newSpeed = [...speed];
  let maxY = pos[1];
  while (true) {
    // The probe's x position increases by its x velocity.
    // The probe's y position increases by its y velocity.
    newPos = [newPos[0] + newSpeed[0], newPos[1] + newSpeed[1]];

    maxY = Math.max(maxY, newPos[1]);

    // Due to drag, the probe's x velocity changes by 1 toward the value 0; that is, it decreases by 1 if it is greater than 0, increases by 1 if it is less than 0, or does not change if it is already 0.
    // Due to gravity, the probe's y velocity decreases by 1.
    newSpeed = [
      newSpeed[0] === 0 ? 0 : newSpeed[0] - newSpeed[0] / Math.abs(newSpeed[0]),
      newSpeed[1] - 1,
    ];

    if (
      x1 <= newPos[0] && newPos[0] <= x2 && y1 <= newPos[1] && newPos[1] <= y2
    ) {
      return [true, maxY] as [boolean, number];
    }

    if (newPos[1] < y1) return [false, maxY] as [boolean, number];
  }
};

const logTrajectory = (pos, speed) => {
  const [hit, maxY] = doTrajectory(pos, speed);
  if (hit) {
    log({ speed }, "=>", "Hit!", { maxY });
  } else {
    log({ speed }, "=>", "Miss!");
  }
};

// logTrajectory(startPos, [7, 2]); // hit
// logTrajectory(startPos, [9, 0]); // hit
// logTrajectory(startPos, [17, -4]); // miss
// logTrajectory(startPos, [6, 9]); // hit

const getHits = () => {
  const acc: number[] = [];
  for (let x = 0; x < 68; x++) {
    for (let y = -260; y < 1000; y++) {
      const [hit, maxY] = doTrajectory(startPos, [x, y]);
      if (hit) {
        acc.push(maxY);
      }
    }
  }
  return acc;
};

log();
part1();
part2();

function part1() {
  const answer = myMax(getHits());

  printSolution("part1", answer); // 45, 33670
}

function part2() {
  const answer = getHits().length;

  printSolution("part2", answer); // 112, 4903
}
