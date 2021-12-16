// deno-lint-ignore-file no-explicit-any
// @deno-types="https://raw.githubusercontent.com/selfrefactor/rambda/master/index.d.ts"
import * as R from "https://raw.githubusercontent.com/selfrefactor/rambda/master/dist/rambda.esm.js";
import { bold, green, red } from "https://deno.land/std@0.117.0/fmt/colors.ts";

const log = console.debug;
const printSolution = (part, answer) => console.info(green(part), "\t", answer);

const splitBy = (sep: string) => (s: string) => s.split(sep);
const myParseint = (s: string) => parseInt(s);
const index = (n: number) => (a: [] | [number, number]) => a[n];
const myMax = (n: number[]) => Math.max(...n);
const myMin = (n: number[]) => Math.min(...n);
const array0toN = (n: number) => [...Array(n).keys()];
const parseBin = (s: string) => parseInt(s, 2);
const parseHex = (s: string) => parseInt(s, 16);

const fileNumber = 0;

const files = ["input.txt"];
const file = Deno.readTextFileSync(files[fileNumber]);

const hexToBits = (s: string) =>
  R.map(
    (h) => parseHex(h).toString(2).padStart(4, "0"),
    s.split(""),
  ).join("");

interface Packet4 {
  V: number;
  T: 4;
  LV: number;
}

interface PacketX {
  V: number;
  T: 0 | 1 | 2 | 3 | 5 | 6 | 7;
  I: number;
  L: number;
  S: Packet[];
}

type Packet = Packet4 | PacketX;

interface ParseResult {
  packet: Packet;
  consumed: number;
}

const parseLiteral = (bits: string) => {
  let acc = "";
  let rem = bits;
  while (true) {
    const [curr, left] = R.splitAt(5, rem);
    const [cont, data] = R.splitAt(1, curr);
    acc += data;
    rem = left;
    if (cont === "0") break;
  }

  const LV = parseBin(acc);
  const consumed = bits.length - rem.length;

  return { LV, consumed };
};

const parseOperator0 = (bits: string) => {
  const bitLengthOfSubPackets = parseBin(bits.slice(0, 0 + 15));
  const subPacketsBits = bits.slice(15, 15 + bitLengthOfSubPackets);

  const S = [] as Packet[];
  let rem = subPacketsBits;
  let totalConsumed = 0;
  while (totalConsumed < bitLengthOfSubPackets) {
    const { packet, consumed } = parsePacket(rem);
    S.push(packet);
    rem = rem.slice(consumed);
    totalConsumed += consumed;
  }

  if (totalConsumed !== bitLengthOfSubPackets) {
    throw Error();
  }

  return { L: bitLengthOfSubPackets, S, consumed: 15 + totalConsumed };
};

const parseOperator1 = (bits: string) => {
  const numberOfSubPackets = parseBin(bits.slice(0, 0 + 11));
  const subPacketsBits = bits.slice(11);

  const S = [] as Packet[];
  let rem = subPacketsBits;
  let totalConsumed = 0;
  while (S.length < numberOfSubPackets) {
    const { packet, consumed } = parsePacket(rem);
    S.push(packet);
    rem = rem.slice(consumed);
    totalConsumed += consumed;
  }

  return { L: numberOfSubPackets, S, consumed: 11 + totalConsumed };
};

const parsePacket = (bits: string) => {
  const V = parseBin(bits.slice(0, 0 + 3));
  const T = parseBin(bits.slice(3, 3 + 3));

  if (T === 4) {
    const { LV, consumed } = parseLiteral(bits.slice(6));

    return {
      packet: { V, T, LV } as Packet4,
      consumed: 3 + 3 + consumed,
    } as ParseResult;
  } else if ([0, 1, 2, 3, 5, 6, 7].includes(T)) {
    const I = parseBin(bits[6]);
    if (I !== 0 && I !== 1) throw Error();

    const op = (I === 0 ? parseOperator0 : parseOperator1);
    const { L, S, consumed } = op(bits.slice(7));

    return {
      packet: { V, T, I, L, S } as PacketX,
      consumed: 3 + 3 + 1 + consumed,
    } as ParseResult;
  } else throw Error();
};

const calculateVersionSum = (packet: Packet) => {
  if (packet.T === 4) {
    return packet.V;
  } else {
    return packet.V + R.sum(R.map(calculateVersionSum, packet.S));
  }
};

const calculateResult = (
  packet: Packet,
) => {
  if (packet.T === 4) {
    return packet.LV;
  } else {
    const op = {
      0: R.sum,
      1: R.product,
      2: myMin,
      3: myMax,
      5: (rs) => rs[0] > rs[1] ? 1 : 0,
      6: (rs) => rs[0] < rs[1] ? 1 : 0,
      7: (rs) => rs[0] === rs[1] ? 1 : 0,
    } as Record<number, (_: number[]) => number>;

    const rs = R.map(calculateResult, packet.S);
    return op[packet.T](rs);
  }
};

log();
part1();
part2();

function part1() {
  const answer = calculateVersionSum(parsePacket(hexToBits(file)).packet);

  printSolution("part1", answer); // ..., 969
}

function part2() {
  const answer = calculateResult(parsePacket(hexToBits(file)).packet);

  printSolution("part2", answer); // ..., 124921618408
}
