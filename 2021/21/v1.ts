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

const files = ["input.txt", "example.txt"];
const file = Deno.readTextFileSync(files[fileNumber]);

const [p1, p2] = file.split("\n");

const p1start = parseInt(p1.split(" ")[4]);
const p2start = parseInt(p2.split(" ")[4]);

log({ p1start, p2start });

let numRolls = 0;
let p1Pos = p1start;
let p2Pos = p2start;
let p1Score = 0;
let p2Score = 0;

while (true) {
  {
    const d1 = 1;
    const d2 = 2;
    const d3 = 3;
    numRolls += 3;
    const dsum = d1 + d2 + d3;

    p1Pos += dsum;
    while (p1Pos > 10) p1Pos -= 10;

    p1Score += p1Pos;

    log(
      `Player 1 rolls ${d1}+${d2}+${d3} and moves to space ${p1Pos} for a total score of ${p1Score}.`,
    );

    if (p1Score >= 21) {
      log("p1 wins", { p2Score, numRolls, answer: p2Score * numRolls });
      break;
    }
  }
  {
    const d1 = 1;
    const d2 = 2;
    const d3 = 3;
    numRolls += 3;
    const dsum = d1 + d2 + d3;

    p2Pos += dsum;
    while (p2Pos > 10) p2Pos -= 10;

    p2Score += p2Pos;

    log(
      `Player 2 rolls ${d1}+${d2}+${d3} and moves to space ${p2Pos} for a total score of ${p2Score}.`,
    );

    if (p2Score >= 21) {
      log("p2 wins", { p1Score, numRolls, answer: p2Score * numRolls });
      break;
    }
  }

  // break;
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
