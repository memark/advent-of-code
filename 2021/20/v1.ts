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

type Coord = { x: number; y: number };

const actives = new Set<Coord>();

for (const y of range(0, image.length)) {
  for (const x of range(0, image[y].length)) {
    if (image[y][x] == "#") {
      actives.add({ x, y });
    }
  }
}

function printState(state: Set<Coord>) {
  const a = Array.from(state);
  const extraForPrint = 3;

  const xStart = myMin(a.map((_) => _.x)) - extraForPrint;
  const xEnd = myMax(a.map((_) => _.x)) + extraForPrint;
  const yStart = myMin(a.map((_) => _.y)) - extraForPrint;
  const yEnd = myMax(a.map((_) => _.y)) + extraForPrint;

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
  actives: Set<Coord>,
  maxCycles: number,
  currentCycle = 1,
): Set<Coord> {
  // console.log({ actives });
  if (currentCycle > maxCycles) return actives;

  const coordsToCheck = new Set<Coord>();
  for (const a of Array.from(actives)) {
    for (const aa of Array.from(getSelfAndNeighboursPlusExtra(a))) {
      if (!has(aa, coordsToCheck)) coordsToCheck.add(aa);
    }
  }
  // console.log("coordsToCheck", coordsToCheck);
  // printState(coordsToCheck);

  const newActives = new Set<Coord>();
  for (const c of Array.from(coordsToCheck)) {
    if (getNewState(c, actives) && !has(c, newActives)) newActives.add(c);
  }

  // printState(newActives);
  // log(Array.from(newActives).length);
  // log(newActives);

  return recurse(newActives, maxCycles, currentCycle + 1);
}

function getSelfAndNeighboursPlusExtra(coord: Coord) {
  // Alla pixlar långt bort från våra byter state varje iteration pga algo[0] === "#".
  const extra = 0;
  const res = new Set<Coord>();
  for (const y of range(coord.y - 1 - extra, coord.y + 1 + extra + 1)) {
    for (const x of range(coord.x - 1 - extra, coord.x + 1 + extra + 1)) {
      res.add({ x, y });
    }
  }
  return res;
}

function getNewState(coord: Coord, actives: Set<Coord>) {
  const san = Array.from(getSelfAndNeighbours(coord));
  // if (san.length !== 9) throw Error();
  // log({ san });
  let b = "";
  for (const n of san) {
    // log({ n }, has(n, actives));
    b += has(n, actives) ? "1" : "0";
  }
  // log({ b });
  const d = parseBin(b);
  // log({ d });
  // log(algo[d]);
  return algo[d] === "#";
}

function getSelfAndNeighbours(coord: Coord) {
  const res = new Set<Coord>();
  for (const y of range(coord.y - 1, coord.y + 1 + 1)) {
    for (const x of range(coord.x - 1, coord.x + 1 + 1)) {
      res.add({ x, y });
    }
  }
  return res;
}

function has(coord: Coord, actives: Set<Coord>) {
  return R.any((a) => a.x == coord.x && a.y == coord.y, Array.from(actives));
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
  const answer = res.size;

  printSolution("part1", answer); // 35, 4778 wrong, 4562 too low, 4928 too high, 5491 ?, 6186 ?, 6670 ?
}

function part2() {
  const answer = 42;

  printSolution("part2", answer); // ?, ?
}
