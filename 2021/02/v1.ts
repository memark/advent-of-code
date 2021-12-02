const log = console.log;

const instructions = Deno
  .readTextFileSync("input.txt")
  .split("\n")
  .map((x) => x.split(" "))
  .map((x) => ({ op: x[0], n: Number(x[1]) }));

{
  let depth = 0;
  let horizontal = 0;

  for (const i of instructions) {
    switch (i.op) {
      case "up":
        depth -= i.n;
        break;
      case "down":
        depth += i.n;
        break;
      case "forward":
        horizontal += i.n;
        break;
    }
  }

  log(horizontal * depth);
}

{
  let depth = 0;
  let horizontal = 0;
  let aim = 0;

  for (const i of instructions) {
    switch (i.op) {
      case "up":
        aim -= i.n;
        break;
      case "down":
        aim += i.n;
        break;
      case "forward":
        horizontal += i.n;
        depth += aim * i.n;
        break;
    }
  }

  log(horizontal * depth);
}
