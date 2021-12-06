const log = console.log;

const useExample = 0;

const filename = useExample ? "example.txt" : "input.txt";
const data = Deno.readTextFileSync(filename);

// "3,4,3,1,2"
// const fish = [1];
const fish = data.split(",").map((x) => parseInt(x)).sort();
// log(fish);

// [ 1, 2, 3, 3, 4 ]
// log([...fish].sort());

// @ts-ignore x
const groupBy = (x, f) => x.reduce((a, b) => ((a[f(b)] ||= []).push(b), a), {});

// // @ts-ignore x
// log(groupBy([1, 2, 3, 4, 5, 6, 7, 8, 9], (v) => v));
// // @ts-ignore x
// log(typeof (groupBy([1, 2, 3, 4, 5, 6, 7, 8, 9], (v) => v)));

function getScore(days: number, fish: number[], shouldLog = 0): number {
  const temp = groupBy(fish, (v: number) => v);

  let bucket = [
    temp["0"]?.length ?? 0,
    temp["1"]?.length ?? 0,
    temp["2"]?.length ?? 0,
    temp["3"]?.length ?? 0,
    temp["4"]?.length ?? 0,
    temp["5"]?.length ?? 0,
    temp["6"]?.length ?? 0,
    temp["7"]?.length ?? 0,
    temp["8"]?.length ?? 0,
  ];

  log("Initial state:", sum(bucket), bucket);

  for (let day = 1; day <= days; day++) {
    const newBucket = [...bucket];

    newBucket[0] = bucket[1];
    newBucket[1] = bucket[2];
    newBucket[2] = bucket[3];
    newBucket[3] = bucket[4];
    newBucket[4] = bucket[5];
    newBucket[5] = bucket[6];
    newBucket[6] = bucket[7] + bucket[0];
    newBucket[7] = bucket[8];
    newBucket[8] = bucket[0];

    bucket = newBucket;

    if (shouldLog || day % 10 === 0) {
      log("After", day, "days:", sum(bucket), bucket);
    }
  }

  return sum(bucket);
}

{
  const score = getScore(80, fish, 0);

  console.info(
    "part1\t",
    score,
  ); // 5934, 394994
}

{
  const score = getScore(256, fish, 0);

  console.info(
    "part2\t",
    score,
  ); // 26984457539, ?
}

function sum(numbers: number[]) {
  return numbers.reduce((acc, n) => acc + n);
}
