// deno-lint-ignore-file no-explicit-any
// @deno-types="https://raw.githubusercontent.com/selfrefactor/rambda/master/index.d.ts"
import * as R from "https://raw.githubusercontent.com/selfrefactor/rambda/master/dist/rambda.esm.js";
import { bold, green, red } from "https://deno.land/std@0.118.0/fmt/colors.ts";

const log = console.debug;
const printSolution = (part, answer) => console.info(green(part), "\t", answer);

const splitBy = (sep: string) => (s: string) => s.split(sep);
const myParseint = (s: string) => parseInt(s);
const index = (n: number) => (a: [] | [number, number] | number[]) => a[n];
const myMax = (n: number[]) => Math.max(...n);
const myMin = (n: number[]) => Math.min(...n);
const array0toN = (n: number) => [...Array(n).keys()];
const parseBin = (s: string) => parseInt(s, 2);
const parseHex = (s: string) => parseInt(s, 16);

const fileNumber = 0;

const files = ["input.txt", "example.txt"];
const file = Deno.readTextFileSync(files[fileNumber]);

const [top, bottom] = file.split("\n\n");

const image = bottom.split("\n").map((l) => l.split(""));
const algo = top.split("");
if (algo.length !== 512) throw Error();

// algo[0] = "#"; // HACK!!!

type Coord = { x: number; y: number };

const actives: Coord[] = [];

for (const y of range(0, image.length)) {
  for (const x of range(0, image[y].length)) {
    if (image[y][x] == "#") {
      actives.push({ x, y });
    }
  }
}

function printState(state: Coord[]) {
  const extraForPrint = 3;

  const xStart = myMin(state.map((_) => _.x)) - extraForPrint;
  const xEnd = myMax(state.map((_) => _.x)) + extraForPrint;
  const yStart = myMin(state.map((_) => _.y)) - extraForPrint;
  const yEnd = myMax(state.map((_) => _.y)) + extraForPrint;

  // log({ xStart, xEnd, yStart, yEnd });

  for (let y = yStart; y <= yEnd; y++) {
    let yRes = "";
    for (let x = xStart; x <= xEnd; x++) {
      yRes += has({ x, y }, state) ? "#" : ".";
    }
    log(yRes);
  }
}

// log({ actives });

// printState(actives);

// log(getSelfAndNeighbours({ x: 5, y: 10 }));

// log(getNewState({ x: 2, y: 2 }, actives));

// const res = recurse(actives, numCycles);
// printState(res);
// log(res.size);

function recurse(
  actives: Coord[],
  maxCycles: number,
  currentCycle = 1,
): Coord[] {
  // console.log({ actives });
  if (currentCycle > maxCycles) return actives;

  const coordsToCheck: Coord[] = [];

  const extra = 1;

  const xStart = myMin(actives.map((_) => _.x)) - extra;
  const xEnd = myMax(actives.map((_) => _.x)) + extra;
  const yStart = myMin(actives.map((_) => _.y)) - extra;
  const yEnd = myMax(actives.map((_) => _.y)) + extra;

  for (let y = yStart; y <= yEnd; y++) {
    for (let x = xStart; x <= xEnd; x++) {
      // yRes += has({ x, y }, state) ? "#" : ".";
      coordsToCheck.push({ x, y });
    }
  }

  // for (const a of actives) {
  //   for (const aa of getSelfAndNeighboursPlusExtra(a)) {
  //     if (!has(aa, coordsToCheck)) coordsToCheck.add(aa);
  //   }
  // }
  // console.log("coordsToCheck", coordsToCheck);
  // printState(coordsToCheck);

  const newActives: Coord[] = [];
  for (const c of coordsToCheck) {
    if (
      getNewState(c, actives, currentCycle % 2 === 0) && !has(c, newActives)
    ) {
      newActives.push(c);
    }
  }

  // printState(newActives);
  // log(newActives.length);
  // log(newActives);

  return recurse(newActives, maxCycles, currentCycle + 1);
}

function getSelfAndNeighboursPlusExtra(coord: Coord) {
  // Alla pixlar långt bort från våra egna byter state varje iteration pga algo[0] === "#".
  const extra = 2;
  const res: Coord[] = [];
  for (const y of range(coord.y - 1 - extra, coord.y + 1 + extra + 1)) {
    for (const x of range(coord.x - 1 - extra, coord.x + 1 + extra + 1)) {
      res.push({ x, y });
    }
  }
  return res;
}

function getNewState(coord: Coord, actives: Coord[], flippedCycle: boolean) {
  const extra = 1;

  const xStart = myMin(actives.map((_) => _.x)) - extra;
  const xEnd = myMax(actives.map((_) => _.x)) + extra;
  const yStart = myMin(actives.map((_) => _.y)) - extra;
  const yEnd = myMax(actives.map((_) => _.y)) + extra;

  const san = getSelfAndNeighbours(coord);
  let b = "";
  for (const n of san) {
    if (n.x < xStart || n.x > xEnd || n.y < yStart || n.y > yEnd) {
      b += !flippedCycle ? "1" : "0";
    } else {
      const h = has(n, actives);
      b += h ? "1" : "0";
    }
  }
  const d = parseBin(b);

  // Detta gav en massa andra konstiga svar.
  // return flippedCycle ? algo[d] === "." : algo[d] === "#";
  return algo[d] === "#";
}

function getSelfAndNeighbours(coord: Coord) {
  const res: Coord[] = [];
  for (const y of range(coord.y - 1, coord.y + 1 + 1)) {
    for (const x of range(coord.x - 1, coord.x + 1 + 1)) {
      res.push({ x, y });
    }
  }
  return res;
}

function has(coord: Coord, actives: Coord[]) {
  return R.any((a) => a.x == coord.x && a.y == coord.y, actives);
}

function range(from: number, to: number): number[] {
  return Array.from(Array(Math.floor(to - from))).map((v, k) => from + k);
}

log();
part1();
// part2();

function part1() {
  const res = recurse(actives, 2);
  printState(res);
  const answer = res.length;

  printSolution("part1", answer); // 35, 4778 wrong, 4562 too low, 4928 too high, ?
}

function part2() {
  const answer = 42;

  printSolution("part2", answer); // ?, ?
}
