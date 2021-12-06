import {
  add,
  adjust,
  append,
  compose,
  equals,
  filter,
  identity,
  map,
  range,
  reduce,
  sort,
  split,
  sum,
  tail,
} from "https://raw.githubusercontent.com/selfrefactor/rambda/master/dist/rambda.esm.js";

const useExample = 0;
const filename = useExample ? "example.txt" : "input.txt";
const data = Deno.readTextFileSync(filename);
const fish = compose(sort(identity), map(parseInt), split(","))(data);

console.info("part1\t", getScore(80, fish));
console.info("part2\t", getScore(256, fish));

function getScore(days: number, fish: number[]) {
  const buckets = map((v) => filter(equals(v), fish).length, range(0, 9));

  const iterate = (vs) => compose(adjust(6, add(vs[0])), shift)(vs);
  const finalBuckets = reduce(iterate, buckets, range(0, days));

  return sum(finalBuckets);
}

function shift(vs) {
  return compose(append(vs[0]), tail)(vs);
}
