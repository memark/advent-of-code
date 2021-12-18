// deno-lint-ignore-file no-explicit-any
// @deno-types="https://raw.githubusercontent.com/selfrefactor/rambda/master/index.d.ts"
import * as R from "https://raw.githubusercontent.com/selfrefactor/rambda/master/dist/rambda.esm.js";
import { bold, green, red } from "https://deno.land/std@0.118.0/fmt/colors.ts";

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

const fileNumber = 1;

const files = ["input.txt", "example.txt"];
const file = Deno.readTextFileSync(files[fileNumber]);

{
  // type Pair = [Num, Num] | [Num, Pair] | [Pair, Num] | [Pair | Pair];
  // type Num = number | Pair;

  // const magnitude = (num: Num) => {
  //   log(typeof num);
  //   if (typeof num === "number") {
  //     return num as number;
  //   } else {
  //     // return 3 * magnitude(num[0]) + 2 * magnitude(num[1]);
  //   }
  // };

  // log(magnitude([9, 1]));
  // log(magnitude([[9, 1], [1, 9]]));
}

{
  // class Num {
  //   constructor(public Value?: number, public Left?: Num, public Right?: Num) {}
  // }

  // const magnitude = (num: Num) =>
  //   num.Value ??
  //     3 * magnitude(num.Left!) + 2 * magnitude(num.Right!) as number;

  // const cn = (n) => new Num(n, undefined, undefined);
  // const cp = (l, r) => new Num(undefined, l, r);

  // log(magnitude(cp(cn(9), cn(1))));
  // log(magnitude(cp(cp(cn(9), cn(1)), cp(cn(1), cn(9)))));
}

{
  // interface Num {
  //   Value?: number;
  //   Left?: Num;
  //   Right?: Num;
  // }

  // const magnitude = (num: Num) =>
  //   num.Value ??
  //     3 * magnitude(num.Left!) + 2 * magnitude(num.Right!) as number;

  // log(magnitude({ Left: { Value: 9 }, Right: { Value: 1 } }));
}

const parse = eval;

type Pair = [Num, Num] | [Num, Pair] | [Pair, Num] | [Pair | Pair];
type Num = number | Pair;

// To split a regular number, replace it with a pair; the left element of the pair should be the regular number divided by two and rounded down, while the right element of the pair should be the regular number divided by two and rounded up. For example, 10 becomes [5,5], 11 becomes [5,6], 12 becomes [6,6], and so on.

const split = (n: number) => [Math.floor(n / 2), Math.ceil(n / 2)] as Pair;

// log(split(11)); // [5,6]

// To check whether it's the right answer, the snailfish teacher only checks the magnitude of the final sum. The magnitude of a pair is 3 times the magnitude of its left element plus 2 times the magnitude of its right element. The magnitude of a regular number is just that number.

const magnitude: (num: Num) => number = (num: Num) =>
  typeof num === "number"
    ? num
    : 3 * magnitude(num[0]) + 2 * magnitude(num[1]!);

// log(magnitude(parse("[9,1]"))); // 29
// log(magnitude(parse("[[9, 1], [1, 9]]"))); // 129
// log(magnitude(parse("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]"))); // 3488

// To explode a pair, the pair's left value is added to the first regular number to the left of the exploding pair (if any), and the pair's right value is added to the first regular number to the right of the exploding pair (if any). Exploding pairs will always consist of two regular numbers. Then, the entire exploding pair is replaced with the regular number 0.

// try this now...

const firstPairToExplode = (num: Num) => {
  const four = num[0]?.[0]?.[0]?.[0];
  if (typeof four === "object") {
    return four;
  } else return undefined;
};

log(firstPairToExplode([[[[[9, 8], 1], 2], 3], 4])); // [9, 8]
log(firstPairToExplode([7, [6, [5, [4, [3, 2]]]]]));

// done

// To reduce a snailfish number, you must repeatedly do the first action in this list that applies to the snailfish number:
// - If any pair is nested inside four pairs, the leftmost such pair _explodes_.
// - If any regular number is 10 or greater, the leftmost such regular number _splits_.
// Once no action in the above list applies, the snailfish number is reduced.

// During reduction, at most one action applies, after which the process returns to the top of the list of actions. For example, if split produces a pair that meets the explode criteria, that pair explodes before other splits occur.

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
