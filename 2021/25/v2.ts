// deno-lint-ignore-file no-explicit-any
// @deno-types="https://raw.githubusercontent.com/selfrefactor/rambda/master/index.d.ts"
import * as R from "https://raw.githubusercontent.com/selfrefactor/rambda/master/dist/rambda.esm.js";
import { bold, green, red } from "https://deno.land/std@0.118.0/fmt/colors.ts";

const log = console.debug;
const printSolution = (part: string, answer: number) =>
  console.info(green(part), "\t", answer);

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
const wrapIndex = (min: number, max: number) =>
  (n: number) => n >= max ? min : n;

const fileNumber = 0;

const file = Deno.readTextFileSync(
  // @ts-ignore dynamic value
  fileNumber === 0 ? "input.txt" : `example${fileNumber}.txt`,
);

const lines = R.map(R.split(""), file.split("\n"));
const yl = lines.length;
const xl = lines[0].length;

type State = string[][];

const moveAllInDir = (
  dir: string,
  [xfunc, yfunc]: [(x: number) => number, (y: number) => number],
) =>
  (state: State) => {
    let moved = false;
    const newState = R.clone(state);
    for (const y of range(0, yl)) {
      for (const x of range(0, xl)) {
        if (state[y][x] !== dir) continue;
        const nx = wrapIndex(0, xl)(xfunc(x));
        const ny = wrapIndex(0, yl)(yfunc(y));
        if (state[ny][nx] !== ".") continue;
        newState[ny][nx] = dir;
        newState[y][x] = ".";
        moved = true;
      }
    }
    return [newState, moved] as [State, boolean];
  };

const stepsUntilNoMovement = (state: State) => {
  log(`Initial state:\n${formatState(lines)}\n`);

  for (let step = 1;; step++) {
    const [state1, moved1] = moveAllInDir(">", [R.inc, R.identity])(state);
    const [state2, moved2] = moveAllInDir("v", [R.identity, R.inc])(state1);
    const moved = moved1 || moved2;
    state = state2;

    log(`After ${step} step:\n${formatState(state)}\n`);

    if (!moved) return step;
  }
};

log();
part1();

function part1() {
  const answer = stepsUntilNoMovement(lines);

  printSolution("part1", answer); // 58, 353
}

function formatState(state: string[][]) {
  return R.map((s) => s.join(""), state).join("\n");
}
