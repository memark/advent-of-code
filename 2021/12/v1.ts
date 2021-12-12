// @deno-types="https://raw.githubusercontent.com/selfrefactor/rambda/master/index.d.ts"
import {
  any,
  filter,
  includes,
  length,
  map,
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
const files = ["input.txt", "example1.txt", "example2.txt", "example3.txt"];
const data = Deno.readTextFileSync(files[fileNumber]);
const lines = data.split("\n").map((l) => l.split("-")) as Connection[];
// log(lines);
// log();

type Cave = string;
type Connection = [Cave, Cave];
type Path = Cave[];

part1();
part2();

function part1() {
  const paths = goOneStep(lines, "start", []);
  // log(formatPaths(paths));

  const answer = length(paths);

  printSolution("part1", answer); // 10, 19, 226, 4104
}

function part2() {
  const paths = goOneStep(lines, "start", [], true);
  // log(formatPaths(paths));

  const answer = length(paths);

  printSolution("part2", answer); // 36, 103, 3509, 119760
}

function isFinalPathValid(path: Path): boolean {
  const visitedSmall = filter(isLower, path);
  const visitedSmallUniq = uniq(visitedSmall);
  const counts = map(
    (v) => filter((vv) => vv === v, visitedSmall).length,
    visitedSmallUniq,
  );
  const smallsMoreThanOnce = filter((c) => c > 1, counts);
  if (smallsMoreThanOnce.length > 1) {
    return false;
  }
  return true;
}

function goOneStep(
  lines: Connection[],
  current: Cave,
  visited: Cave[],
  partTwo = false,
): Cave[][] {
  const newVisited = [...visited, current];
  const indent = repeat(" ", newVisited.length).join("");
  // log({ current: indent + current });

  if (current === "end") {
    return [newVisited];
  }

  const fromCurrent0 = filter((l) => l[0] === current, lines);
  const fromCurrent1 = filter((l) => l[1] === current, lines);
  // log({ fromCurrent });
  const allNexts0 = map((l) => l[1], fromCurrent0);
  const allNexts1 = map((l) => l[0], fromCurrent1);
  // log({ allNexts0 });
  // log({ allNexts1 });
  const allNexts = union(allNexts0, allNexts1);

  const temp = allNexts.find((n) => n === "dc");
  const temp2 = "start,kj,HN,dc,HN,kj".split(",");
  // if (temp) log(validPart2(temp, temp2));
  // log(temp);

  // "start,kj,HN,dc,HN,kj,dc,HN,end".split(",")

  const validNexts = filter(
    (n) => partTwo ? validPart2(n, visited) : validPart1(n, visited),
    allNexts,
  );

  const temp3 = validNexts.find((n) => n === "kj");
  // if (temp3) log({ visited }, { validNexts });

  const paths = map(
    (n) => goOneStep(lines, n, newVisited, partTwo),
    validNexts,
  ).flat();

  return paths;
}

function validPart1(cave: Cave, visited: Cave[]) {
  return isUpper(cave) || !includes(cave, visited);
}

function validPart2(cave: Cave, visited: Cave[]) {
  if (cave === "start") {
    return false;
  }

  if (isUpper(cave)) {
    return true;
  }

  if (!includes(cave, visited)) {
    return true;
  }

  const visitedSmall = filter(isLower, visited);
  const visitedSmallUniq = uniq(visitedSmall);
  const counts = map(
    (v) => filter((vv) => vv === v, visitedSmall).length,
    visitedSmallUniq,
  );
  const anySmallMoreThanOnce = any((c) => c > 1, counts);
  if (anySmallMoreThanOnce) {
    return false;
  }

  return true;
}

function formatPath(path: Path): string {
  return path.join(",");
}

function formatPaths(paths: Path[]): string {
  return map(formatPath, paths).sort().join("\n");
}

function isLower(s) {
  return s.toLowerCase() === s;
}

function isUpper(s) {
  return s.toUpperCase() === s;
}
