// @deno-types="https://raw.githubusercontent.com/selfrefactor/rambda/master/index.d.ts"
import {
  median,
  sum,
} from "https://raw.githubusercontent.com/selfrefactor/rambda/master/dist/rambda.esm.js";
import * as R from "https://raw.githubusercontent.com/selfrefactor/rambda/master/dist/rambda.esm.js";
import { green, red } from "https://deno.land/std@0.117.0/fmt/colors.ts";

const printSolution = (part, answer) => console.info(green(part), "\t", answer);

const log = console.debug;

const useExample = 0;
const filename = useExample ? "example.txt" : "input.txt";
const data = Deno.readTextFileSync(filename);
const lines = data.split("\n");

const starts = "([{<";
const stops = ")]}>";

const expected = (lastStart) => stops.at(starts.indexOf(lastStart));

{
  const points = lines.map(pointsForCorruptLine);
  const answer = sum(points);

  printSolution("part1", answer); // 26397, 193275
}

function pointsForCorruptLine(line: string): number {
  const pointsForChar = (
    char,
  ) => ({ ")": 3, "]": 57, "}": 1197, ">": 25137 }[char]);

  const state: string[] = [];

  for (let pos = 0; pos < line.length; pos++) {
    const c = line[pos];

    if (starts.includes(c)) {
      state.push(c);
    } else if (stops.includes(c)) {
      const e = expected(state.at(-1));
      if (c === e) {
        state.pop();
      } else {
        // log(
        //   red(`Corrupt! Expected ${e}, but found ${c} instead at pos ${pos}`),
        // );
        return pointsForChar(c);
      }
    } else {
      log(red("Invalid char."), { c });
    }
  }

  if (state.length === 0) {
    // log(green("Complete."));
    return 0;
  } else {
    // log(green("Incomplete."));
    return 0;
  }
}

{
  const points = lines.map(pointsForIncompleteLine).filter((p) => p > 0);
  const answer = median(points);

  printSolution("part2", answer); // 288957, 2429644557
}

function pointsForIncompleteLine(line: string): number {
  const state: string[] = [];

  for (let pos = 0; pos < line.length; pos++) {
    const c = line[pos];
    if (starts.includes(c)) {
      state.push(c);
    } else if (stops.includes(c)) {
      const e = expected(state.at(-1));
      if (c === e) {
        state.pop();
      } else {
        return 0;
      }
    } else {
      log(red("Invalid char."), { c });
    }
  }

  if (state.length === 0) {
    // log(green("Complete."));
    return 0;
  } else {
    const score = scoreState(state.reverse());
    // log(green("Incomplete."), { score }, { state: state.join("") }, {
    //   missing: state.map(expected).join(""),
    // });
    return score;
  }
}

function scoreState(state: string[]): number {
  let totalScore = 0;
  for (const s of state) {
    const e = expected(s);
    const score = { ")": 1, "]": 2, "}": 3, ">": 4 }[e!];
    totalScore = totalScore * 5 + score!;
  }
  return totalScore;
}
