const log = console.log;

const useExample = 0;

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
if (size <= 10) print(scoresStraight);

const totalScoreStraight = scoresStraight.flat().filter((s) => s >= 2).length;

console.info(
  "part1\t",
  totalScoreStraight,
); // 5, 6283

const scores = array0toN(size).map((y) =>
  array0toN(size).map((x) => scorePoint(x, y, vents))
);
if (size <= 10) print(scores);

const totalScore = scores.flat().filter((s) => s >= 2).length;

console.info(
  "part2\t",
  totalScore,
); //  12, (18843 too low) 18864

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
  const minX = Math.min(v.start.x, v.stop.x);
  const minY = Math.min(v.start.y, v.stop.y);
  const maxX = Math.max(v.start.x, v.stop.x);
  const maxY = Math.max(v.start.y, v.stop.y);

  // vertical
  if (
    v.start.x == v.stop.x && x == v.start.x &&
    (minY <= y && y <= maxY)
  ) {
    return true;
  }

  // horizontal
  if (
    v.start.y == v.stop.y && y == v.start.y &&
    (minX <= x && x <= maxX)
  ) {
    return true;
  }

  if (!isStraightLine(v)) {
    // diagonal
    const x1 = v.start.x;
    const x2 = v.stop.x;
    const y1 = v.start.y;
    const y2 = v.stop.y;

    const lx = Math.abs(x1 - x2);
    const ly = Math.abs(y1 - y2);

    if (lx !== ly) throw "Not 45 deg";

    const dx = x2 > x1 ? 1 : -1;
    const dy = y2 > y1 ? 1 : -1;

    for (let i = 0; i < lx + 1; i++) {
      const cx = x1 + i * dx;
      const cy = y1 + i * dy;
      if (x === cx && y === cy) return true;
    }
  }

  if (shouldLog) log("d");
  return false;
}

function isStraightLine(v: Vent): boolean {
  return v.start.x == v.stop.x || v.start.y == v.stop.y;
}

function print(score: number[][]) {
  for (const line of score) {
    const l2 = line.map((p) => p == 0 ? "." : p).reduce(
      (acc, s) => acc.concat(s.toString()),
      "",
    );
    log(l2);
  }
}

function array0toN(n: number) {
  return [...Array(n).keys()];
}
