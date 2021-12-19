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

const allScannerReadings = R.compose(
  R.map(R.map(R.map(myParseint))),
  R.map(R.map(R.split(","))),
  R.map(R.tail),
  R.map(R.split("\n")),
  R.split("\n\n"),
)(file);
// log({ allScannerReadings });

const formatReadings = (readings: number[][]) => {
  const yStart = Math.max(0, ...R.map(index(1), readings));
  const yEnd = Math.min(0, ...R.map(index(1), readings));
  for (let y = yStart; y >= yEnd; y--) {
    const xStart = Math.min(0, ...R.map(index(0), readings));
    const xEnd = Math.max(0, ...R.map(index(0), readings));
    let yRes = "";
    for (
      let x = xStart;
      x <= xEnd;
      x++
    ) {
      if (x === 0 && y === 0) {
        yRes += "S";
      } else if (R.includes([x, y], readings)) {
        yRes += "B";
      } else yRes += ".";
    }
    log(yRes);
  }
};
allScannerReadings.forEach((v, i) => {
  log("Scanner", i);
  formatReadings(v);
  log();
});
// formatReadings(allScannerReadings[0]);
// formatReadings(allScannerReadings[1]);

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
