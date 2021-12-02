const log = console.log;

const data = Deno
  .readTextFileSync("example.txt")
  // .readTextFileSync("input.txt")
  .split("\n");

const bitLength = data[0].length;
log(bitLength);

{
  // gamma = most common
  let ga = "";

  // epsilon = least common
  let ep = "";

  for (let i = 0; i < bitLength; i++) {
    const no0 = data.filter((x) => x[i] == "0").length;
    const no1 = data.filter((x) => x[i] == "1").length;
    log(no0, no1);

    if (no0 > no1) {
      ga += "0";
      ep += "1";
    } else {
      ga += "1";
      ep += "0";
    }
    log(ga, ep);
  }

  const gaDec = parseInt(ga, 2);
  const epDec = parseInt(ep, 2);

  log(gaDec, epDec);
  log("part1\t", gaDec * epDec);
}

log();

{
  let dataOx = data;
  for (let i = 0; i < bitLength; i++) {
    log("----");
    log("BIT", i);
    const no0 = dataOx.filter((x) => x[i] == "0").length;
    const no1 = dataOx.filter((x) => x[i] == "1").length;

    let keepOx = "";

    if (no0 > no1) {
      keepOx = "0";
    } else if (no0 == no1) {
      keepOx = "1";
    } else {
      keepOx = "1";
    }

    log(dataOx.length, dataOx);
    log("keepOx", keepOx);
    if (dataOx.length > 1) {
      dataOx = dataOx.filter((x) => x[i] == keepOx);
    }
    log(dataOx.length, dataOx);
    log();
  }
  const oxDec = parseInt(dataOx[0], 2);

  let dataCo = data;
  for (let i = 0; i < bitLength; i++) {
    log("----");
    log("BIT", i);
    const no0 = dataCo.filter((x) => x[i] == "0").length;
    const no1 = dataCo.filter((x) => x[i] == "1").length;

    let keepCo = "";

    if (no0 > no1) {
      keepCo = "1";
    } else if (no0 == no1) {
      keepCo = "0";
    } else {
      keepCo = "0";
    }

    log(dataCo.length, dataCo);
    log("keepCo", keepCo);
    if (dataCo.length > 1) {
      dataCo = dataCo.filter((x) => x[i] == keepCo);
    }
    log(dataCo.length, dataCo);
    log();
  }
  const coDec = parseInt(dataCo[0], 2);

  log(oxDec, coDec);
  log("part2\t", oxDec * coDec);
}
