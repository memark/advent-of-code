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

const fileNumber = 0;

const files = ["input.txt", "example.txt"];
const file = Deno.readTextFileSync(files[fileNumber]);

const levels = R.compose(
  R.map(R.map(myParseint)),
  R.map(splitBy("")),
  splitBy("\n"),
)(file) as Map;
log({ levels });

type Map = number[][];
type Coord = [number, number];

const size = levels.length;
const start = [0, 0] as Coord;
const end = [size - 1, size - 1] as Coord;
// const end = [3, 3] as Coord;

const ns = [
  // [-1, 0],
  // [-1, +1],
  [0, +1],
  // [+1, +1],
  [+1, 0],
  // [+1, -1],
  // [0, -1],
  // [-1, -1],
] as [-1 | 0 | 1, -1 | 0 | 1][];

const limit = 0 as number;

function getLowestRisk(visited: Coord[]) {
  return function (count: number) {
    return function ([y, x]: Coord): number | null {
      // log({ visited }, { y, x });
      // if (count === limit) log(".");

      // log(visited.length);
      const newVisited = [...visited, [y, x] as Coord];

      if (R.equals([y, x], end) || (limit !== 0 && count === limit)) {
        // log("from point", [y, x], "reached end or limit", { newVisited });
        // return [newVisited];

        const finalPath = newVisited;

        const values = R.map(([y1, x1]) => levels[y1][x1], finalPath);
        // log({ newValues });

        const risk = sum(values);
        // log(newTotalRisks);

        return risk;
      }

      const realNs = R.map(([dy, dx]) => [y + dy, x + dx], ns) as Coord[];
      const nextCoords = R.filter(
        ([y, x]) =>
          0 <= y && y < size && 0 <= x && x < size &&
          !R.includes([y, x], newVisited),
        realNs,
      );

      const partialRisks = R.map(
        getLowestRisk(newVisited)(count + 1),
        nextCoords,
      );
      // const f = newPaths.flat();
      // const fl = f.length;
      // if (fl > 0) log("from point", [y, x], "flat length", fl);

      // const values = R.map(
      //   (p) => R.map(([y1, x1]) => levels[y1][x1], p),
      //   partialPaths.flat(),
      // );
      // // log({ newValues });

      // const risks = R.map(sum, values);
      // // log(newTotalRisks);

      const partialRisksNotNull = R.filter((r) => r !== null, partialRisks);
      if (partialRisksNotNull.length === 0) {
        return null;
      }

      const lowestPartialRisk = myMin(partialRisks as number[]);
      log({ lowestPartialRisk });

      if (lowestPartialRisk === Infinity) {
        // log({ visited });
        log(formatPath(newVisited));
        log({ nextCoords });
        log({ partialRisks });
        log({ count });
        throw "Halting";
      }

      return lowestPartialRisk;

      // return partialPaths.flat();
    };
  };
}

// Jag vill ändra på två saker
// 1. Bara lagra vilka nivåer jag har i min lista
// 2. Bara returnera den av newPaths som har lägst risk

part1();
// part2();

log(R.equals([10, 10], end));

function part1() {
  const lowestRisk = getLowestRisk([])(0)(start);
  // log("got paths");
  // log(paths);
  // R.forEach((p) => log(formatPath(p) + "\n"), paths);

  // const values = R.map(R.map(([y, x]) => levels[y][x]), lowestRisk);
  // log({ values });

  // const totalRisks = R.map(sum, values);
  // log(totalRisks);

  const answer = lowestRisk! - 1;

  printSolution("part1", answer); // 40, ?
}

function part2() {
  const answer = 42;

  printSolution("part2", answer); // ?, ?
}

function sum(numbers: number[]) {
  return numbers.reduce((acc, n) => acc + n);
}

function formatPath(path: Coord[]) {
  const res = R.map(
    (y) =>
      R.map(
        (x) =>
          R.includes([y, x], path)
            ? bold(levels[y][x].toString())
            : levels[y][x].toString(),
        array0toN(size),
      ).join(""),
    array0toN(size),
  ).join("\n");

  // const res = path.map((y) => y.map((x) => x === 1 ? "#" : ".").join(""))
  //   .join("\n");
  return res;
}
