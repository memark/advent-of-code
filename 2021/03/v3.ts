const log = console.log;

const lines = Deno
  // .readTextFileSync("example.txt")
  .readTextFileSync("input.txt")
  .split("\n");

const numberOfBits = lines[0].length;
const numbers = lines.map((x) => parseInt(x, 2));

part1(); // 198, 3687446
// part2(); // 230, 4406844

function part1() {
  const gammaBin = arrayNto0(numberOfBits)
    .map((b) => {
      const onlyBitB = numbers.map((n) => n & (1 << b));
      const ratio = sum(onlyBitB) / (1 << b);
      return ratio > numbers.length / 2 ? 1 : 0;
    });

  const gamma = parseInt(toString(gammaBin), 2);
  const epsilon = gamma ^ ((1 << numberOfBits) - 1);

  log("part1\t", gamma * epsilon);
}

// function part2() {
//   const oxygen = parseInt(reduceNumbers(getOxygenFilterBit), 2);
//   // log("oxygen", oxygen);

//   const co2 = parseInt(reduceNumbers(getCo2FilterBit), 2);
//   // log("co2", co2);

//   log("part2\t", oxygen * co2);
// }

// function reduceNumbers(
//   getFilterBit: (ones: number, zeroes: number) => "1" | "0",
// ) {
//   let reducedData = numbers;
//   // log(reducedData.length, reducedData);

//   for (let i = 0; i < numberOfBits; i++) {
//     // log("----");
//     // log("BIT", i);

//     const bits = reducedData.map((x) => x[i]);
//     const ones = bits.filter((x) => x == "1").length;
//     const zeroes = bits.filter((x) => x == "0").length;
//     const filterBit = getFilterBit(ones, zeroes);
//     log("filterBit", filterBit);
//     if (reducedData.length > 1) {
//       reducedData = reducedData.filter((x) => x[i] == filterBit);
//     }
//     // log(reducedData.length, reducedData);
//   }

//   return reducedData[0];
// }

// function getOxygenFilterBit(ones: number, zeroes: number) {
//   return ones >= zeroes ? "1" : "0";
// }

// function getCo2FilterBit(ones: number, zeroes: number) {
//   return ones < zeroes ? "1" : "0";
// }

function sum(numbers: number[]) {
  return numbers.reduce((acc, n) => acc + n);
}

function arrayNto0(n: number) {
  return [...Array(n).keys()].reverse();
}

function toString(bits: (0 | 1)[]): string {
  return bits.reduce((acc, curr) => acc + curr.toString(), "");
}
