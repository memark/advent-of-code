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

const file = Deno.readTextFileSync(
  // @ts-ignore dynamic value
  fileNumber === 0 ? "input.txt" : `example${fileNumber}.txt`,
);
// log({ file });

const lines = R.map(R.split(" "), file.split("\n"));
// log({ lines });

let numberOfInputsDone = 0;

const recurse = (
  state: { w: number; x: number; y: number; z: number },
  lines: string[][],
  inputs: string[],
) => {
  if (lines.length === 0) {
    return state;
  }

  const [[op, a, b], ...ls] = lines;
  // log("current line:", { op, a, b });

  switch (op) {
    case "inp": {
      // inp a - Read an input value and write it to variable a.
      const [i, ...is] = inputs;
      // log({ i, is });
      if (i === undefined) throw Error(Deno.inspect({ op, a, b }));
      const newState = { ...state };
      newState[a] = parseInt(i);
      // log("inp", { newState });
      numberOfInputsDone++;
      // log({ numberOfInputsDone });
      return recurse(newState, ls, is);
    }

    case "add": {
      // add a b - Add the value of a to the value of b, then store the result in variable a.
      const av = state[a];
      const bv = state?.[b] ?? parseInt(b);
      const res = av + bv;
      const newState = { ...state };
      newState[a] = res;
      // log("add", { a, av, b, bv, res, newState });
      return recurse(newState, ls, inputs);
    }

    case "mul": {
      // mul a b - Multiply the value of a by the value of b, then store the result in variable a.
      const av = state[a];
      const bv = state?.[b] ?? parseInt(b);
      const res = av * bv;
      const newState = { ...state };
      newState[a] = res;
      // log("mul", { a, av, b, bv, res, newState });
      return recurse(newState, ls, inputs);
    }

    case "div": {
      // div a b - Divide the value of a by the value of b, truncate the result to an integer, then store the result in variable a. (Here, "truncate" means to round the value toward zero.)
      const av = state[a];
      const bv = state?.[b] ?? parseInt(b);
      const res = Math.trunc(av / bv);
      const newState = { ...state };
      newState[a] = res;
      // log("div", { a, av, b, bv, res, newState });
      return recurse(newState, ls, inputs);
    }

    case "mod": {
      // mod a b - Divide the value of a by the value of b, then store the remainder in variable a. (This is also called the modulo operation.)
      const av = state[a];
      const bv = state?.[b] ?? parseInt(b);
      const res = av % bv;
      const newState = { ...state };
      newState[a] = res;
      // log("mod", { a, av, b, bv, res, newState });
      return recurse(newState, ls, inputs);
    }

    case "eql": {
      // eql a b - If the value of a and b are equal, then store the value 1 in variable a. Otherwise, store the value 0 in variable a.
      const av = state[a];
      const bv = state?.[b] ?? parseInt(b);
      const res = av === bv ? 1 : 0;
      const newState = { ...state };
      newState[a] = res;
      // log("eql", { a, av, b, bv, res, newState });
      return recurse(newState, ls, inputs);
    }

    default:
      throw Error();
  }
};

// const inputs = "13579246899999".split("");
// log({ inputs });

// log();

const length = 14;

let largest = 0;
while (true) {
  const n = Math.trunc(Math.random() * 10 ** length).toString().padStart(
    length,
    "0",
  );
  if (!n.includes("0")) {
    // log("no zeroes", c, c.split(""));
    const res = recurse({ w: 0, x: 0, y: 0, z: 0 }, lines, n.split(""));
    if (res.z === 0) {
      log("valid", n);
      largest = Math.max(largest, parseInt(n));
      log({ largest });
      // break;
    } else {
      // log("invalid", n);
    }
  }
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
