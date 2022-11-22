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
  // @ts-ignore dynamic value
  fileNumber === 0 ? "input.txt" : `example${fileNumber}.txt`,
);
log(file);

const a = file.split("\n")[1].replaceAll("#", "").split("");
const [h01, h02, h03, h04, h05, h06, h07, h08, h09, h10, h11] = a;
log(a);

const b = file.split("\n")[2].replaceAll("#", "").split("");
log(b);
const [r11, r21, r31, r41] = b;

const c = file.split("\n")[3].replaceAll(" ", "").replaceAll("#", "").split("");
log(c);
const [r12, r22, r32, r42] = c;

const m = new Map<any, any>();
// m.set()

interface Space {
  location: [number, number];
  name: string;
  adjacentNames: string[];
}

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

function range(from: number, to: number): number[] {
  return Array.from(Array(Math.floor(to - from))).map((_, k) => from + k);
}
