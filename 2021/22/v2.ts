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

const fileNumber = 1;

const file = Deno.readTextFileSync(
  // @ts-ignore not static value
  fileNumber === 0 ? "input.txt" : `example${fileNumber}.txt`,
);
log(file);

const parseInstruction = (line: string) => {
  const [_, oo, x1, x2, y1, y2, z1, z2] = R.match(
    /(on|off) x=(.*)\.\.(.*),y=(.*)\.\.(.*),z=(.*)\.\.(.*)/,
    line,
  );
  return {
    oo: oo === "on" ? 1 : 0,
    x1: parseInt(x1),
    x2: parseInt(x2),
    y1: parseInt(y1),
    y2: parseInt(y2),
    z1: parseInt(z1),
    z2: parseInt(z2),
  };
};

type Cuboid = Omit<ReturnType<typeof parseInstruction>, "oo">;

// The initialization procedure only uses cubes that have
// x, y, and z positions of at least -50 and at most 50.
// For now, ignore cubes outside this region.
const belongsToInitialization = (c: Cuboid) =>
  c.x1 >= -50 && c.x2 <= 50 && c.y1 >= -50 && c.y2 <= 50 && c.z1 >= -50 &&
  c.z2 <= 50;

const instructions = R.compose(
  // R.filter(belongsToInitialization),
  R.map(parseInstruction),
  splitBy("\n"),
)(file);
log(instructions);

type Coord = [number, number, number];

const cubes: Set<string> = new Set();

for (const i of instructions) {
  for (const x of range(i.x1, i.x2 + 1)) {
    for (const y of range(i.y1, i.y2 + 1)) {
      for (const z of range(i.z1, i.z2 + 1)) {
        const cube = `${x},${y},${z}`;
        if (i.oo) {
          cubes.add(cube);
        } else {
          cubes.delete(cube);
        }
        if (cubes.size % 1000 === 0) log(cubes.size);
      }
    }
  }
}

const between = (a, b, c) => a <= b && b <= c;

const hasOverlap = (a: Cuboid, b: Cuboid) => {
  const xBetween = between(a.x1, b.x1, a.x2) || between(a.x1, b.x2, a.x2);
  const yBetween = between(a.y1, b.y1, a.y2) || between(a.y1, b.y2, a.y2);
  const zBetween = between(a.z1, b.z1, a.z2) || between(a.z1, b.z2, a.z2);
  return xBetween && yBetween && zBetween;
};

const cubeDiff = (a: Cuboid, b: Cuboid) => {
  if (!hasOverlap(a, b)) {
    return a;
  }

  
};

log(cubes);
log(cubes.size);

log();
// part1();
// part2();

// log(R.match(
//   /(on) x=(.*)\.\.(.*),y=(.*)\.\.(.*),z=(.*)\.\.(.*)/,
//   "on x=10..12,y=10..12,z=10..12",
// ));

function part1() {
  const answer = 42;

  printSolution("part1", answer); // ?, ?
}

function part2() {
  const answer = 42;

  printSolution("part2", answer); // ?, ?
}

function range(from: number, to: number): number[] {
  return Array.from(Array(Math.floor(to - from))).map((v, k) => from + k);
}
