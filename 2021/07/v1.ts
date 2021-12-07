import {
  compose,
  identity,
  map,
  median,
  range,
  sort,
  split,
  sum,
} from "https://raw.githubusercontent.com/selfrefactor/rambda/master/dist/rambda.esm.js";
import * as R from "https://raw.githubusercontent.com/selfrefactor/rambda/master/dist/rambda.esm.js";
import { green } from "https://deno.land/std@0.117.0/fmt/colors.ts";

const log = console.debug;

const useExample = 1;
const filename = useExample ? "example.txt" : "input.txt";
const data = Deno.readTextFileSync(filename);
const poss = compose(sort(identity), map(parseInt), split(","))(
  data,
) as number[];

{
  const medianPos = median(poss);

  const fuels = map((p) => Math.abs(p - medianPos), poss);

  const fuel = sum(fuels);
  console.info(green("part1\t"), fuel);
}

{
  const mi = Math.min(...poss);
  const ma = Math.max(...poss);

  const r = range(mi, ma + 1);

  const fuels = map(getTotalFuel, r) as number[];
  const minFuel = Math.min(...fuels);

  console.info(green("part2\t"), minFuel);
}

function getTotalFuel(stop) {
  return sum(map((p) => getFuel(stop, p), poss));
}

function getFuel(stop, start) {
  let fuel = 0;
  const diff = Math.abs(stop - start);
  (range(0, diff + 1) as number[]).forEach((e) => {
    fuel += e;
  });
  return fuel;
}
