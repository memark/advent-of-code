const log = console.log;

const useExample = 1;

const filename = useExample ? "example.txt" : "input.txt";
const data = Deno.readTextFileSync(filename);

// "0,9 -> 5,9"
// "8,0 -> 0,8"
const vents = data.split("\n").map((l) =>
  l.split(" -> ").map((r) => r.split(",")).map((xy) => ({
    x: parseInt(xy[0]),
    y: parseInt(xy[1]),
  }))
).map((v) => ({ start: v[0], stop: v[1] }));

const ventsStraight = vents.filter(isStraightLine);

type Coord = { x: number; y: number };
type Vent = { start: Coord; stop: Coord };

const ventsMaxX = Math.max(
  ...vents.flat().map((v) => [v.start.x, v.stop.x]).flat(),
);
const ventsMaxY = Math.max(
  ...vents.flat().map((v) => [v.start.y, v.stop.y]).flat(),
);

const size = Math.max(ventsMaxX, ventsMaxY) + 1;

const scoresStraight = array0toN(size).map((y) =>
  array0toN(size).map((x) => scorePoint(x, y, ventsStraight))
);
if (useExample) log(prettyFormat(scoresStraight));

const totalScoreStraight = scoresStraight.flat().filter((s) => s >= 2).length;

console.info(
  "part1\t",
  totalScoreStraight,
); // 5, 6283

const scores = array0toN(size).map((y) =>
  array0toN(size).map((x) => scorePoint(x, y, vents))
);
if (useExample) log(prettyFormat(scores));

const totalScore = scores.flat().filter((s) => s >= 2).length;

console.info(
  "part2\t",
  totalScore,
); //  12, 18864

function scorePoint(
  x: number,
  y: number,
  vents: Vent[],
  shouldLog = false,
): number {
  return vents.filter((v) => pointIsOnVent(x, y, v, shouldLog)).length;
}

function pointIsOnVent(
  x: number,
  y: number,
  v: Vent,
  shouldLog = false,
): boolean {
  const x1 = v.start.x;
  const x2 = v.stop.x;
  const y1 = v.start.y;
  const y2 = v.stop.y;

  const lx = Math.abs(x1 - x2);
  const ly = Math.abs(y1 - y2);

  const minX = Math.min(v.start.x, v.stop.x);
  const minY = Math.min(v.start.y, v.stop.y);
  const maxX = Math.max(v.start.x, v.stop.x);
  const maxY = Math.max(v.start.y, v.stop.y);

  if (lx === 0) {
    // vertical
    return x === x1 && minY <= y && y <= maxY;
  } else if (ly === 0) {
    // horizontal
    return y === y1 && minX <= x && x <= maxX;
  } else {
    // diagonal
    if (lx !== ly) throw "Not 45 deg";

    if (!(between(x1, x, x2) && between(y1, y, y2))) {
      return false;
    }

    const dx = x - x1;
    const dy = y - y1;

    const rx = x2 > x1 ? 1 : -1;
    const ry = y2 > y1 ? 1 : -1;

    return (dx * rx === dy * ry);
  }
}

function between(a: number, b: number, c: number): boolean {
  return (a <= b && b <= c) || (a >= b && b >= c);
}

function isStraightLine(v: Vent): boolean {
  return v.start.x == v.stop.x || v.start.y == v.stop.y;
}

import { red } from "https://deno.land/std@0.117.0/fmt/colors.ts";

function prettyFormat(scores: number[][]) {
  return scores.map((line) =>
    line.map((p: number) =>
      p == 0 ? "." : (p >= 2 ? red(p.toString()) : p.toString())
    ).join("")
  ).join("\n");
}

function array0toN(n: number) {
  return [...Array(n).keys()];
}

// Från Johan Andersson
//
// (defun in-line? (p from to)
//   (destructuring-bind ((px py) (fx fy) (tx ty)) (list p from to)
//     (and (or (= fx tx)
//              (= fy ty)
//              (= (- fx fy) (- tx ty)))
//          (<= (min fx tx) px (max fx tx))
//          (<= (min fy ty) py (max fy ty)))))

// DAY05> (in-line? '(2 3) '(2 3) '(3 4))
// T
// DAY05> (in-line? '(2 3) '(1 3) '(3 4))
// NIL
// DAY05> (in-line? '(3 0) '(2 0) '(4 0))
// T
// DAY05> (in-line? '(3 1) '(2 0) '(4 0))
// NIL
