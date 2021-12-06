const log = console.log;

const useExample = 1;

const filename = useExample ? "example.txt" : "input.txt";
const data = Deno.readTextFileSync(filename);

// "3,4,3,1,2"
const fish = [1]; // data.split(",").map((x) => parseInt(x)).sort();
// log(fish);

// [ 1, 2, 3, 3, 4 ]
log([...fish].sort());

// import * as regression from "https://cdnjs.cloudflare.com/ajax/libs/regression/2.0.1/regression.js";

// log(regression.exponential);

function getScore(days: number, fish: number[], shouldLog = 0): number {
  const fishes = [...fish];
  log("Initial state:", fishes);

  for (let day = 1; day <= days; day++) {
    // Don't loop over newly added.
    const l = fishes.length;

    for (let i = 0; i < l; i++) {
      if (fishes[i] == 0) {
        fishes[i] = 6;
        fishes.push(8);
      } else {
        fishes[i]--;
      }
    }

    if (shouldLog || day % 1 === 0) {
      log("After", day, "days:", fishes.length);
    }
  }

  return fishes.length;
}

{
  // const score = getScore(80, fish, 0);

  // console.info(
  //   "part1\t",
  //   score,
  // ); // 5934, 394994
}

// {
//   const score = getScore(256, fish, 0);

//   console.info(
//     "part2\t",
//     score,
//   ); // 26984457539, ?
// }
