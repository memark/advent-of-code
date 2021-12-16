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

const parseLiteral = (bits: string) => {
  // log("parseLiteral", { bits });

  let acc = "";
  let rem = bits;
  while (true) {
    const curr = rem.slice(0, 0 + 5);
    rem = rem.slice(5);

    const cont = curr.slice(0, 0 + 1);
    const data = curr.slice(1, 1 + 4);

    acc += data;

    if (cont === "0") {
      break;
    }
  }
  // log({ acc });

  const LV = parseBin(acc);
  const consumed = bits.length - rem.length;

  return { LV, consumed };
};

const parseOperator0 = (bits: string) => {
  // log("parsePacket - operator type 0");
  // the length of the sub-packets in bits
  const L = parseBin(bits.slice(0, 0 + 15));
  // log({ L });

  const subPacketsBits = bits.slice(15, 15 + L);
  // log({ subPacketsBits });

  let acc: any[] = [];
  let rem = subPacketsBits;
  let totalConsumed = 0;
  while (rem.length > 0) {
    // log({ rem, remLength: rem.length });

    const res = parsePacket(rem);
    // log({ res });
    const { packet, consumed } = res;

    acc = [...acc, packet];
    rem = rem.slice(consumed);
    // log({ newRem: rem, remLength: rem.length });

    totalConsumed += res.consumed;
    // log({ totalConsumed });
  }

  if (totalConsumed !== L) {
    throw Error();
  }

  const S = acc;

  return { L, S, consumed: 15 + L };
};

const parseOperator1 = (bits: string) => {
  // log("parsePacket - operator type 1");
  // the number of sub-packets
  const L = parseBin(bits.slice(0, 0 + 11));

  const subPacketsBits = bits.slice(11);
  // log({ subPacketsBits });

  let acc: any[] = [];
  let rem = subPacketsBits;
  let totalConsumed = 0;
  while (acc.length < L) {
    // log({ rem, remLength: rem.length });

    const res = parsePacket(rem);
    // log({ res });
    const { packet, consumed } = res;

    acc = [...acc, packet];
    rem = rem.slice(consumed);
    // log({ newRem: rem, remLength: rem.length });

    totalConsumed += res.consumed;
    // log({ totalConsumed });
  }

  const S = acc;

  return { L, S, consumed: 11 + totalConsumed };
};

const parsePacket = (bits: string) => {
  // log("parsePacket", { bits });
  const V = parseBin(bits.slice(0, 0 + 3));
  const T = parseBin(bits.slice(3, 3 + 3));

  switch (T) {
    case 4: {
      // log("parsePacket - literal");
      const { LV, consumed } = parseLiteral(bits.slice(6));

      return { packet: { V, T, LV, R: LV }, consumed: 3 + 3 + consumed };
    }

    default: {
      const I = parseBin(bits.slice(6, 6 + 1));

      const packetFunc = () => {
        switch (I) {
          case 0: {
            const { L, S, consumed } = parseOperator0(bits.slice(7));
            return {
              packet: { V, T, I, L, S },
              consumed: 3 + 3 + 1 + consumed,
            };
          }

          case 1: {
            const { L, S, consumed } = parseOperator1(bits.slice(7));
            return {
              packet: { V, T, I, L, S },
              consumed: 3 + 3 + 1 + consumed,
            };
          }

          default:
            throw Error();
        }
      };
      const packet = packetFunc();
      // log({ packet });

      switch (T) {
        case 0: // sum
         {
          const r = R.sum(R.map((s) => s.R, packet.packet.S));
          return {
            packet: { ...packet.packet, R: r },
            consumed: packet.consumed,
          };
        }
        case 1: // product
         {
          const r = R.product(R.map((s) => s.R, packet.packet.S));
          return {
            packet: { ...packet.packet, R: r },
            consumed: packet.consumed,
          };
        }

        case 2: // minimum
         {
          const r = myMin(R.map((s) => s.R as number, packet.packet.S));
          return {
            packet: { ...packet.packet, R: r },
            consumed: packet.consumed,
          };
        }

        case 3: // maximum
         {
          const r = myMax(R.map((s) => s.R as number, packet.packet.S));
          return {
            packet: { ...packet.packet, R: r },
            consumed: packet.consumed,
          };
        }

        case 5: // greater than
         {
          const Rs = R.map((s) => s.R as number, packet.packet.S);
          const r = Rs[0] > Rs[1] ? 1 : 0;
          return {
            packet: { ...packet.packet, R: r },
            consumed: packet.consumed,
          };
        }

        case 6: // less than
         {
          const Rs = R.map((s) => s.R as number, packet.packet.S);
          const r = Rs[0] < Rs[1] ? 1 : 0;
          return {
            packet: { ...packet.packet, R: r },
            consumed: packet.consumed,
          };
        }

        case 7: // equal
         {
          const Rs = R.map((s) => s.R as number, packet.packet.S);
          const r = Rs[0] === Rs[1] ? 1 : 0;
          return {
            packet: { ...packet.packet, R: r },
            consumed: packet.consumed,
          };
        }

        default:
          throw Error();
      }
    }
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
