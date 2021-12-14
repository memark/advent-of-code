// @deno-types="https://raw.githubusercontent.com/selfrefactor/rambda/master/index.d.ts"
import * as R from "https://raw.githubusercontent.com/selfrefactor/rambda/master/dist/rambda.esm.js";
import { bold, green, red } from "https://deno.land/std@0.117.0/fmt/colors.ts";

const log = console.debug;
const printSolution = (part, answer) => console.info(green(part), "\t", answer);

const splitBy = (sep: string) => (s: string) => s.split(sep);
const myParseint = (s: string) => parseInt(s);
const index = (n: number) => (a: [] | [number, number]) => a[n];
const myMax = (n: number[]) => Math.max(...n);
const myMin = (n: number[]) => Math.min(...n);
const array0toN = (n: number) => [...Array(n).keys()];

const files = ["input.txt", "example.txt"];
const file = Deno.readTextFileSync(files[0]);

const [top, bottom] = splitBy("\n\n")(file);

const template = top as Input;
log({ template });

const rules = R.compose(
  // R.map(r=>({r}))
  R.map(splitBy(" -> ")),
  splitBy("\n"),
)(bottom) as Rule[];
log({ rules });

type Input = string;
type Rule = [string, string];

const runOneStep = (rules: Rule[]) =>
  (input: Input) => {
    const splitInput = input.split("");
    const zippedInput = R.zipWith(
      (a, b) => a + b,
      splitInput.slice(0),
      splitInput.slice(1),
    );
    // log({ zippedInput });

    let res = input;
    for (let i = 0; i < res.length; i++) {
      const pair = res[i] + res[i + 1]; // slice!
      const m = R.find((r) => r[0] === pair, rules)?.[1];
      // log(m);
      // log("before", { res });
      if (m) {
        // log(`Adding ${m} between ${res[i]} and ${res[i + 1]}`);
        // res += input[i] + m + input[i + 1];
        res = res.slice(0, i + 1) + m + res.slice(i + 1);
        i++;
      }
      // log("after", { res });
    }
    // log({ res });
    return res;
  };

// log(runOneStep(rules)(template));
// log(runOneStep(rules)(runOneStep(rules)(template)));

part1();
// part2();

function part1() {
  const polymer = R.reduce(
    (prev, curr) => runOneStep(rules)(prev),
    template,
    array0toN(10),
  );
  log(polymer);

  const elements = R.uniq(polymer.split(""));
  log({ elements });

  const counts = R.map(
    (e) => R.filter((p) => p === e, polymer.split("")).length,
    elements,
  );
  log({ counts });

  const mostCommon = myMax(counts);
  log({ mostCommon });
  const leastCommon = myMin(counts);
  log({ leastCommon });
  const answer = mostCommon - leastCommon;

  printSolution("1", answer); // 1488
}

function part2() {
  const answer = 42;

  printSolution("2", "\n" + answer); //
}

function formatDots(dots, greenAndBlack = false) {
  const maxCoordAtPos = (pos: number) =>
    R.compose(
      myMax,
      R.map(index(pos)),
    )(dots);
  const maxX = maxCoordAtPos(0);
  const maxY = maxCoordAtPos(1);

  const filler = greenAndBlack ? " " : ".";
  const matrix = createFilled2dArray(maxY + 1, maxX + 1, filler) as string[][];
  dots.forEach((d) => matrix[d[1]][d[0]] = "#");

  return (greenAndBlack ? green : R.identity)(
    matrix.map((y) => y.join("")).join("\n"),
  );
}

function createFilled2dArray(numRows, numColumnns, fillValue) {
  return [...Array(numRows)].map(() => Array(numColumnns).fill(fillValue));
}
