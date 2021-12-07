import {
  compose,
  identity,
  map,
  range,
  split,
  subtract,
  sum,
} from "https://raw.githubusercontent.com/selfrefactor/rambda/master/dist/rambda.esm.js";
import * as R from "https://raw.githubusercontent.com/selfrefactor/rambda/master/dist/rambda.esm.js";
import { green } from "https://deno.land/std@0.117.0/fmt/colors.ts";

const printSolution = (part, answer) => console.info(green(part), "\t", answer);

const useExample = 0;
const filename = useExample ? "example.txt" : "input.txt";
const data = Deno.readTextFileSync(filename);
const positions = compose(
  map(parseInt),
  split(","),
)(
  data,
) as number[];

const abs = Math.abs;
const min = Math.min;
const max = Math.max;

const totalFuel = (fuelConsumption) =>
  (positions) =>
    (stop) =>
      compose(
        sum,
        map(fuelConsumption),
        map(abs),
        map(subtract(stop)),
      )(positions);

const possiblePositions = range(min(...positions), max(...positions) + 1);

const minFuel = (fuelConsumption) =>
  min(...map(
    totalFuel(fuelConsumption)(positions),
    possiblePositions,
  ) as number[]);

{
  const constantFuel = identity;
  printSolution("part1", minFuel(constantFuel)); // 37, 337833
}

{
  const increasingFuel = (distance) => distance * (distance + 1) / 2;
  printSolution("part2", minFuel(increasingFuel)); // 168, 96678050
}
