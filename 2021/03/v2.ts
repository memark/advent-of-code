const log = console.log;

const numbers = Deno
  .readTextFileSync("example.txt")
  // .readTextFileSync("input.txt")
  .split("\n"); //as ("0" | "1")[];

const bitLength = numbers[0].length;

part1(); // 198, 3687446
part2(); // 230, 4406844

function part1() {
  const rotatedBits = [...Array(bitLength).keys()].map((i) =>
    numbers.map((x) => x[i])
  );

  const ones = rotatedBits
    .map((x) =>
      x
        .filter((y) => y == "1").length
    );

  const mostCommon = "".concat(
    ...(ones.map((x) => x > numbers.length / 2 ? "1" : "0")),
  );
  const gamma = parseInt(mostCommon, 2);

  const leastCommon = "".concat(
    ...(ones.map((x) => x < numbers.length / 2 ? "1" : "0")),
  );
  const epsilon = parseInt(leastCommon, 2);

  log("part1\t", gamma * epsilon);
}

function part2() {
  const oxygen = parseInt(reduceNumbers(getOxygenFilterBit), 2);
  log("oxygen", oxygen);

  const co2 = parseInt(reduceNumbers(getCo2FilterBit), 2);
  log("co2", co2);

  log("part2\t", oxygen * co2);
}

function reduceNumbers(
  getFilterBit: (ones: number, zeroes: number) => "1" | "0",
) {
  let reducedData = numbers;
  log(reducedData.length, reducedData);

  for (let i = 0; i < bitLength; i++) {
    log("----");
    log("BIT", i);

    const bits = reducedData.map((x) => x[i]);
    const ones = bits.filter((x) => x == "1").length;
    const zeroes = bits.filter((x) => x == "0").length;
    const filterBit = getFilterBit(ones, zeroes);
    log("filterBit", filterBit);
    if (reducedData.length > 1) {
      reducedData = reducedData.filter((x) => x[i] == filterBit);
    }
    log(reducedData.length, reducedData);
  }

  return reducedData[0];
}

function getOxygenFilterBit(ones: number, zeroes: number) {
  return ones >= zeroes ? "1" : "0";
}

function getCo2FilterBit(ones: number, zeroes: number) {
  return ones < zeroes ? "1" : "0";
}
