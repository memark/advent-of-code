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

const fileNumber = 0;

const files = ["input.txt"];
const file = Deno.readTextFileSync(files[fileNumber]);

const hexToBits = (s: string) =>
  R.map(
    (h) => parseInt(h, 16).toString(2).padStart(4, "0"),
    s.split(""),
  ).join("");

// Dela upp L i två olika

interface BasePacket {
  V: number;
  T: number;
  R?: number;
}

interface Packet4 extends BasePacket {
  T: 4;
  LV: number;
}

interface PacketX extends BasePacket {
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
  // length of the sub-packets in bits
  const L = parseBin(bits.slice(0, 0 + 15));
  const subPacketsBits = bits.slice(15, 15 + L);

  const S = [] as Packet[];
  let rem = subPacketsBits;
  let totalConsumed = 0;
  while (totalConsumed < L) {
    const { packet, consumed } = parsePacket(rem);
    S.push(packet);
    rem = rem.slice(consumed);
    totalConsumed += consumed;
  }

  if (totalConsumed !== L) {
    throw Error();
  }

  return { L, S, consumed: 15 + totalConsumed };
};

const parseOperator1 = (bits: string) => {
  // number of sub-packets
  const L = parseBin(bits.slice(0, 0 + 11));
  const subPacketsBits = bits.slice(11);

  const S = [] as Packet[];
  let rem = subPacketsBits;
  let totalConsumed = 0;
  while (S.length < L) {
    const { packet, consumed } = parsePacket(rem);
    S.push(packet);
    rem = rem.slice(consumed);
    totalConsumed += consumed;
  }

  return { L, S, consumed: 11 + totalConsumed };
};

const calculateResult = (
  T: number,
  packet: {
    packet: PacketX;
    consumed: number;
  },
) => {
  const op = {
    0: R.sum,
    1: R.product,
    2: myMin,
    3: myMax,
    5: (rs) => rs[0] > rs[1] ? 1 : 0,
    6: (rs) => rs[0] < rs[1] ? 1 : 0,
    7: (rs) => rs[0] === rs[1] ? 1 : 0,
  } as Record<number, (_: number[]) => number>;

  const rs = R.map((s) => s.R!, packet.packet.S);
  const r = op[T](rs);
  return {
    packet: { ...packet.packet, R: r },
    consumed: packet.consumed,
  } as ParseResult;
};

const parsePacket = (bits: string) => {
  // log("parsePacket", { bits });
  const V = parseBin(bits.slice(0, 0 + 3));
  const T = parseBin(bits.slice(3, 3 + 3));

  if (T === 4) {
    // log("parsePacket - literal");
    const { LV, consumed } = parseLiteral(bits.slice(6));

    return {
      packet: { V, T: T as Packet4["T"], LV, R: LV },
      consumed: 3 + 3 + consumed,
    };
  } else {
    const I = parseBin(bits[6]);
    if (I !== 0 && I !== 1) throw Error();

    const op = (I === 0 ? parseOperator0 : parseOperator1);
    const { L, S, consumed } = op(bits.slice(7));
    const packet = {
      packet: { V, T: T as PacketX["T"], I, L, S },
      consumed: 3 + 3 + 1 + consumed,
    };
    // log({ packet });

    return calculateResult(T, packet) as ParseResult;
  }
};

const assertAndLogPacket = (hex: string, expected: any) => {
  const actual = parsePacket(hexToBits(hex));
  // R.equals({ V: 6, T: 4, L: 2021 }, { V: 6, T: 4, L: 2021 }));
  // if (!R.equals(actual, expected)) {
  if (JSON.stringify(actual.packet) !== JSON.stringify(expected)) {
    throw `${hex}: Expected ${Deno.inspect(expected)} but got ${
      Deno.inspect(actual)
    }`;
  }
  log({ hex }, "=>", actual);
  log();
};

log();
// assertAndLogPacket("D2FE28", { V: 6, T: 4, LV: 2021 });
// assertAndLogPacket("38006F45291200", {
//   V: 1,
//   T: 6,
//   I: 0,
//   L: 27,
//   S: [{ V: 6, T: 4, LV: 10 }, { V: 2, T: 4, LV: 20 }],
// });
// assertAndLogPacket("EE00D40C823060", {
//   V: 7,
//   T: 3,
//   I: 1,
//   L: 3,
//   S: [{ V: 2, T: 4, LV: 1 }, { V: 4, T: 4, LV: 2 }, { V: 1, T: 4, LV: 3 }],
// });

const sumVersions = R.compose(
  R.sum,
  R.map(parseInt),
  R.map((m: string) => m.split(":")[1]),
  R.match(/"V":(\d+)/g),
  JSON.stringify,
);

// log(sumVs(parsePacket(hexToBits("8A004A801A8002F478"))), 16);
// log(sumVs(parsePacket(hexToBits("620080001611562C8802118E34"))), 12);
// log(sumVs(parsePacket(hexToBits("C0015000016115A2E0802F182340"))), 23);
// log(sumVs(parsePacket(hexToBits("A0016C880162017C3686B18A3D4780"))), 31);

// log(parsePacket(hexToBits("C200B40A82")).packet.R, 3);
// log(parsePacket(hexToBits("04005AC33890")).packet.R, 54);
// log(parsePacket(hexToBits("880086C3E88112")).packet.R, 7);
// log(parsePacket(hexToBits("CE00C43D881120")).packet.R, 9);
// log(parsePacket(hexToBits("D8005AC2A8F0")).packet.R, 1);
// log(parsePacket(hexToBits("F600BC2D8F")).packet.R, 0);
// log(parsePacket(hexToBits("9C005AC2F8F0")).packet.R, 0);
// log(parsePacket(hexToBits("9C0141080250320F1802104A08")).packet.R, 1);

part1();
part2();

function part1() {
  const answer = sumVersions(parsePacket(hexToBits(file)));

  printSolution("part1", answer); // ..., 969
}

function part2() {
  const answer = parsePacket(hexToBits(file)).packet.R;

  printSolution("part2", answer); // ..., 124921618408
}
