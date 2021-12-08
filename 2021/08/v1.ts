import {
  difference,
  filter,
  find,
  includes,
  indexOf,
  intersection,
  map,
  sum,
} from "https://raw.githubusercontent.com/selfrefactor/rambda/master/dist/rambda.esm.js";
import * as R from "https://raw.githubusercontent.com/selfrefactor/rambda/master/dist/rambda.esm.js";
import { green } from "https://deno.land/std@0.117.0/fmt/colors.ts";

const printSolution = (part, answer) => console.info(green(part), "\t", answer);

const log = console.debug;

const useExample = 0;
const filename = useExample ? "example.txt" : "input.txt";
const data = Deno.readTextFileSync(filename);
const lines = data.split("\n").map((l) =>
  l.split(" | ").map((p) => p.split(" "))
) as Line[];
// log(lines);

{
  const count1478 = sum(
    lines.map((l) =>
      l[1].filter((o) => [2, 3, 4, 7].includes(o.length)).length
    ),
  );
  printSolution("part1", count1478); // 26, 342
}

type Pattern = string;
type Output = string;
type Line = [Pattern[], Output[]];

{
  const results = map(solveLine, lines);
  // log({ results });

  const result = sum(results);
  // log({ result });

  printSolution("part2", result); // 61229, 1068933
}

function solveLine(line: Line): number {
  const ps = line[0].map((s) => s.split("").sort());
  // log({ ps });

  const pswc = map((x) => [x, x.length], ps);
  // log({ pswc });

  const letters = ["a", "b", "c", "d", "e", "f", "g"];

  const twcs = map(
    (l) => [ps.flat().filter((w) => w == l).length, l],
    letters,
  ) as [number, string][];
  // log("twcs", twcs);

  // 2 segments => p1
  const p1 = filter((p) => p.length == 2, ps)[0];
  // log({ p1 });

  // 4 segments => p4
  const p4 = filter((p) => p.length == 4, ps)[0];
  // log({ p4 });

  // 3 segments => p7
  const p7 = filter((p) => p.length == 3, ps)[0];
  // log({ p7 });

  // 7 segments => p8
  const p8 = filter((p) => p.length == 7, ps)[0];
  // log({ p8 });

  // w0 finns i p7 men inte i p1
  const w0 = filter((w) => !includes(w, p1), p7)[0];
  // log({ w0 });

  // w5 saknas bara i siffran 2
  const w5 = find((twc) => twc[0] === 9, twcs)[1];
  // log({ w5 });

  // w1, w3 finns i p4 men inte i p1
  const w13 = filter((w) => !includes(w, p1), p4);
  // log({ w13 });

  // 6 segments => p0, p6, p9
  const p069 = filter((p) => p.length == 6, ps).map((x) => x.sort());
  // log({ p069 });

  // w46 finns i p8 men inte i p4, p7
  const w46 = filter((w) => !includes(w, p4) && !includes(w, p7), p8);
  // log({ w46 });

  // p8-p0 => w3
  // p8-p6 => w2
  // p8-p9 => w4
  const w234 = map((p) => filter((w) => !includes(w, p), p8), p069).flat();
  // log({ w234 });

  const w4 = intersection(w234, w46)[0];
  // log({ w4 });

  const w6 = difference(w46, w4)[0];
  // log({ w6 });

  const w23 = difference(w234, w4);
  // // log({ w23 });

  const w3 = intersection(w13, w23)[0];
  // log({ w3 });

  const w2 = difference(w23, w3)[0];
  // log({ w2 });

  const w1 = difference(letters, [w0, w2, w3, w4, w5, w6])[0];
  // log({ w1 });

  // outputs

  const os = line[1].map((s) => s.split("").sort());
  // log({ os });

  const wires = [w0, w1, w2, w3, w4, w5, w6];
  // log({ wires });

  // log(transposeOutputs(["acedgfb".split("")]));
  const lineResult = transposeOutputs(os);
  // log({ lineResult });

  // log([2, 6, 1, 0, 4, 3, 5]);
  // log([2, 6, 1, 0, 4, 3, 5].sort());
  // log(getDigit([2, 6, 1, 0, 4, 3, 5]));
  // log(getDigit([2, 6, 1, 0, 4, 3, 5].sort()));

  return lineResult;

  function transposeOutputs(os: string[][]): number {
    const wireNumbers = map(getWireNumbers, os);
    // log({ wireNumbers });
    const digits = map(getDigit, wireNumbers);
    // log({ digits });
    const result = parseInt(digits.join(""));
    // log({ result });

    return result;
  }

  function getWireNumbers(o: string[]): number[] {
    const wireNumbers = map((oo) => indexOf(oo, wires), o) as number[];
    // log({ wireNumbers });
    return wireNumbers;
  }

  function getDigit(wireNumbers: number[]) {
    switch (wireNumbers.sort().join("")) {
      case "012456":
        return 0;
      case "25":
        return 1;
      case "02346":
        return 2;
      case "02356":
        return 3;
      case "1235":
        return 4;
      case "01356":
        return 5;
      case "013456":
        return 6;
      case "025":
        return 7;
      case "0123456":
        return 8;
      case "012356":
        return 9;
      default:
        throw `Unknown sequence '${wireNumbers}'`;
    }
  }
}
