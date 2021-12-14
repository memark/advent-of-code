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

const fileNumber = 1;

const files = ["input.txt", "example.txt"];
const file = Deno.readTextFileSync(files[fileNumber]);

const [top, bottom] = splitBy("\n\n")(file);

const template = top as Input;
// log({ template });

const rules = R.compose(
  R.fromPairs,
  R.map(splitBy(" -> ")),
  splitBy("\n"),
)(bottom) as Record<string, string>;
log({ rules });

type Input = string;
type Rule = [string, string];

const runSteps = (totalSteps: number, input: Input) => {
  const resolveInput = (input: string, step: number): string => {
    log({ step });

    // Om max rekursion, returnera input
    if (step === totalSteps) {
      return input;
    }

    // Dela upp i par
    const pairs = R.map(
      (i) => input.slice(i, i + 1 + 1),
      array0toN(input.length - 1),
    );
    // log({ step }, { pairs });

    // Skapa tripletter (genom att slå upp paret och infoga svaret i mitten)
    const triplets = R.map((p) => p[0] + rules[p] + p[1], pairs);
    // log({ step }, { triplets });

    // Rekursera varje triplett
    const recursed = R.map((t) => resolveInput(t, step + 1), triplets);
    // log({ step }, { recursed });

    // Klistra ihop paren
    const [r, ...rs] = recursed;
    const together = r + R.map((x) => x.slice(1), rs).join("");
    // log({ step }, { together });

    // log(
    //   { step: totalSteps - step },
    //   pairs,
    //   "=>",
    //   triplets,
    //   "=>",
    //   recursed,
    //   "=>",
    //   together,
    // );

    // Returnera resultatet
    return together;
  };

  return resolveInput(input, 0);
};

const answerForSteps = (steps) => {
  const polymer = runSteps(steps, template);
  // log(polymer);

  const elements = R.uniq(polymer.split(""));
  // log({ elements });

  const counts = R.map(
    (e) => R.filter((p) => p === e, polymer.split("")).length,
    elements,
  );
  // log({ counts });

  const mostCommon = myMax(counts);
  // log({ mostCommon });
  const leastCommon = myMin(counts);
  // log({ leastCommon });
  const answer = mostCommon - leastCommon;

  return answer;
};

part1();
// part2();

function part1() {
  const answer = answerForSteps(10);

  printSolution("1", answer); // 1588, 3408
}

function part2() {
  const answer = answerForSteps(40);

  printSolution("2", "\n" + answer); // 2188189693529, ?
}
