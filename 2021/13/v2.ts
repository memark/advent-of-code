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

const dots = R.compose(
  R.map(R.map(myParseint)),
  R.map(splitBy(",")),
  splitBy("\n"),
)(top) as Dot[];

const folds = R.compose(
  R.map((s: string) => ({ direction: s[0], line: parseInt(s[1]) })),
  R.map(splitBy("=")),
  R.map(index(2)),
  R.map(splitBy(" ")),
  splitBy("\n"),
)(bottom) as Fold[];

const mirror = (line: number) =>
  (value: number) => Math.min(value, line * 2 - value);

type Dot = [number, number];
interface Fold {
  direction: string;
  line: number;
}

const foldOnce = (dots: Dot[], fold: Fold) =>
  R.uniq(
    R.map(([x, y]: Dot) =>
      fold.direction === "y" //
        ? [x, mirror(fold.line)(y)]
        : [mirror(fold.line)(x), y]
    )(dots),
  ) as Dot[];

part1();
part2();

function part1() {
  const answer = R.reduce(foldOnce, dots, [folds[0]]).length;

  printSolution("1", answer); // 17, 710
}

function part2() {
  const answer = formatDots(R.reduce(foldOnce, dots, folds), true);

  printSolution("2", "\n" + answer); // O, EPLGRULR
}

function formatDots(dots: Dot[], greenAndBlack = false) {
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
