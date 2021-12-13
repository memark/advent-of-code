// @deno-types="https://raw.githubusercontent.com/selfrefactor/rambda/master/index.d.ts"
import {
  any,
  filter,
  flatten,
  includes,
  length,
  map,
  reduce,
  repeat,
  union,
  uniq,
} from "https://raw.githubusercontent.com/selfrefactor/rambda/master/dist/rambda.esm.js";
import * as R from "https://raw.githubusercontent.com/selfrefactor/rambda/master/dist/rambda.esm.js";
import { bold, green, red } from "https://deno.land/std@0.117.0/fmt/colors.ts";

const printSolution = (part, answer) => console.info(green(part), "\t", answer);

const log = console.debug;
log();

const fileNumber = 0;
const files = ["input.txt", "example.txt", "example2.txt", "example3.txt"];
const data = Deno.readTextFileSync(files[fileNumber]);
const [a, b] = data.split("\n\n");
const dotLines = a.split("\n").map((l) => l.split(",").map((x) => parseInt(x)));
const foldLines = b.split("\n").map((l) => l.split(" ")[2].split("="));

const maxX = Math.max(...dotLines.map((l) => l[0]));
const maxY = Math.max(...dotLines.map((l) => l[1]));

const matrix = createFilled2dArray(maxY + 1, maxX + 1, 0);

dotLines.forEach((dot) => matrix[dot[1]][dot[0]] = 1);

type Dot = 0 | 1;
type Matrix = Dot[][];

part1();
part2();

function part1() {
  const foldedOnce = foldPaper(matrix, foldLines[0]);
  const answer = sum(flatten(foldedOnce));

  printSolution("part1", answer); // 17, 710
}

function part2() {
  const folded = reduce(foldPaper, matrix, foldLines);
  const answer = formatMatrixGreen(folded);

  printSolution("part2", "\n" + answer); // "O", "EPLGRULR"
}

function foldPaper(matrix: Matrix, foldLine: string[]) {
  const maxY = matrix.length;
  const maxX = matrix[0].length;

  const axis = foldLine[0];
  const line = parseInt(foldLine[1]);
  // log("Folding along", { axis, line });

  if (axis === "y") {
    const newMatrix = array0toN(line).map((y) =>
      array0toN(maxX).map((x) => matrix[y][x] | matrix[maxY - y - 1][x])
    ) as Matrix;
    return newMatrix;
  }

  if (axis === "x") {
    const newMatrix = array0toN(maxY).map((y) =>
      array0toN(line).map((x) => matrix[y][x] | matrix[y][maxX - x - 1])
    ) as Matrix;
    return newMatrix;
  }

  throw `Unknown axis: ${axis}`;
}

function array0toN(n: number) {
  return [...Array(n).keys()];
}

function formatMatrix(matrix: Matrix) {
  const res = matrix.map((y) => y.map((x) => x === 1 ? "#" : ".").join(""))
    .join("\n");
  return res;
}

function formatMatrixGreen(matrix: Matrix) {
  const res = matrix.map((y) => y.map((x) => x === 1 ? "#" : " ").join(""))
    .join("\n");
  return green(res);
}

function createFilled2dArray(numRows, numColumnns, fillValue) {
  return [...Array(numRows)].map(() => Array(numColumnns).fill(fillValue));
}

function sum(numbers: number[]) {
  return numbers.reduce((acc, n) => acc + n);
}
