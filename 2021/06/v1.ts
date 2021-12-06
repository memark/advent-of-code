const log = console.log;

const useExample = 1;

const filename = useExample ? "example.txt" : "input.txt";
const data = Deno.readTextFileSync(filename);

// "3,4,3,1,2"
const fish = data.split(",").map((x) => parseInt(x));
// log(fish);

function tick(n: number[]): number[] {
  return n.flatMap(tickOneFish);
}

function tickOneFish(n: number): number[] {
  if (n == 0) {
    return [6, 8];
  } else return [n - 1];
}

{
  const score = getScore(80, fish, 0);

  console.info(
    "part1\t",
    score,
  ); // 5934, 394994
}

function getScore(days: number, fish: number[], shouldLog = 0): number {
  let fishes = [...fish];

  log("Initial state:", fishes);
  for (let d = 1; d <= days; d++) {
    fishes = tick(fishes);
    if (shouldLog) log("After", d, "days:", fishes);
  }

  return fishes.length;
}

{
  const score = getScore(145, fish, 1);

  console.info(
    "part1\t",
    score,
  ); // 26984457539, ?
}

// console.info(
//   "part2\t",
//   totalScore,
// ); //  12, 18864

// import { red } from "https://deno.land/std@0.117.0/fmt/colors.ts";

// function prettyFormat(fish: number[]) {
//   return scores.map((line) =>
//     line.map((p: number) =>
//       p == 0 ? "." : (p >= 2 ? red(p.toString()) : p.toString())
//     ).join("")
//   ).join("\n");
// }

function array0toN(n: number) {
  return [...Array(n).keys()];
}
