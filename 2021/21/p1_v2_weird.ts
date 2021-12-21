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

let lastDie = 0;
let numRolls = 0;
let p1Pos = p1start;
let p2Pos = p2start;
let p1Score = 0;
let p2Score = 0;

const playOnePlayerTurn = ({ player, lastDie, numRolls, pPos, pScore }) => {
  const d1 = lastDie + 1 <= 100 ? lastDie + 1 : lastDie + 1 - 100;
  const d2 = lastDie + 2 <= 100 ? lastDie + 2 : lastDie + 2 - 100;
  const d3 = lastDie + 3 <= 100 ? lastDie + 3 : lastDie + 3 - 100;
  const nlastDie = d3;
  const nnumRolls = numRolls + 3;
  const dsum = d1 + d2 + d3;

  let npPos = pPos + dsum;
  while (npPos > 10) npPos -= 10;

  const npScore = pScore + npPos;

  log(
    `Player ${player} rolls ${d1}+${d2}+${d3} and moves to space ${npPos} for a total score of ${npScore}.`,
  );

  return {
    player,
    lastDie: nlastDie,
    numRolls: nnumRolls,
    pPos: npPos,
    pScore: npScore,
  };
};

const checkForWin = ({ p1Score, p2Score, numRolls }) => {
  if (p1Score >= 1000) {
    log("Player 1 wins", { p2Score, numRolls, answer: p2Score * numRolls });
    return true;
  }
  if (p2Score >= 1000) {
    log("Player 2 wins", { p1Score, numRolls, answer: p1Score * numRolls });
    return true;
  }
};

let lastRes: any = { lastDie, numRolls, p1Pos, p1Score, p2Pos, p2Score };
while (true) {
  // state = {...state, ...playOnePlayerTurn({player: 1, lastDie, numRolls, pPos:p1Pos, pScore:p1Score })}
  const res1: any = playOnePlayerTurn({
    player: 1,
    lastDie: lastRes.lastDie,
    numRolls: lastRes.numRolls,
    pPos: p1Pos,
    pScore: p1Score,
  });
  if (
    checkForWin({
      p1Score: res1.pScore,
      p2Score: lastRes.pScore,
      numRolls: res1.numRolls,
    })
  ) {
    break;
  }

  const res2: any = playOnePlayerTurn({
    player: 2,
    lastDie: res1.lastDie,
    numRolls: res1.numRolls,
    pPos: lastRes.p2Pos,
    pScore: lastRes.p2Score,
  });
  if (
    checkForWin({
      p1Score: res1.pScore,
      p2Score: res2.pScore,
      numRolls: res2.numRolls,
    })
  ) {
    break;
  }

  lastRes = {
    lastDie: res2.lastDie,
    numRolls: res2.numRolls,
    p1Pos: res1.pPos,
    p1Score: res1.pScore,
    p2Pos: res2.pPos,
    p2Score: res2.pScore,
  };

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
