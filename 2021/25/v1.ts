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
const range = (from: number, to: number): number[] =>
  Array.from(Array(Math.floor(to - from))).map((_, k) => from + k);
const parseBin = (s: string) => parseInt(s, 2);
const parseHex = (s: string) => parseInt(s, 16);

const fileNumber = 0;

const file = Deno.readTextFileSync(
  // @ts-ignore dynamic value
  fileNumber === 0 ? "input.txt" : `example${fileNumber}.txt`,
);
// log({ file });

const lines = R.map(R.split(""), file.split("\n"));
// log({ lines });

const yl = lines.length;
const xl = lines[0].length;
// log({ xl, yl });

type Coord = [number, number];

let moved = true;
let oldLines = R.clone(lines);
const ms = 60;
let s = 0;

log(`Initial state:\n${formatState(oldLines)}\n`);

while (
  moved //&& s < ms
) {
  s++;
  moved = false;
  {
    const newLines = R.clone(oldLines);
    for (const y of range(0, yl)) {
      for (const x of range(0, xl)) {
        const v = oldLines[y][x];
        if (v === ">") {
          const nx = x + 1 >= xl ? 0 : x + 1;
          if (oldLines[y][nx] === ".") {
            newLines[y][nx] = ">";
            newLines[y][x] = ".";
            moved = true;
          }
        }
      }
    }
    oldLines = newLines;
  }
  {
    const newLines = R.clone(oldLines);
    for (const y of range(0, yl)) {
      for (const x of range(0, xl)) {
        const v = oldLines[y][x];
        if (v === "v") {
          const ny = y + 1 >= yl ? 0 : y + 1;
          if (oldLines[ny][x] === ".") {
            newLines[ny][x] = "v";
            newLines[y][x] = ".";
            moved = true;
          }
        }
      }
    }
    oldLines = newLines;
  }
  log(`After ${s} step:\n${formatState(oldLines)}\n`);
}
log("total steps", s);

log();
// part1();
// part2();

function part1() {
  const answer = 42;

  printSolution("part1", answer); // ?, ?
}

function part2() {
  const answer = 42;

  printSolution("part2", answer); // ?, ?
}

function formatState(state: string[][]) {
  return R.map((s) => s.join(""), state).join("\n");
}
