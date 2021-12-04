const log = console.log;

const data = Deno
  // .readTextFileSync("example.txt")
  .readTextFileSync("input.txt")
  .split("\n");

const numbers = data[0].split(",").map((x) => parseInt(x));
// log(numbers);

let rest = data.slice(2).filter((x) => x);
// log(rest);

type Board = number[][];

let boards: Board[] = [];
while (rest.length > 0) {
  const [a, b, c, d, e, ...remaining] = rest;
  const a1 = a.split(" ").filter((x) => x).map((x) => parseInt(x));
  const b1 = b.split(" ").filter((x) => x).map((x) => parseInt(x));
  const c1 = c.split(" ").filter((x) => x).map((x) => parseInt(x));
  const d1 = d.split(" ").filter((x) => x).map((x) => parseInt(x));
  const e1 = e.split(" ").filter((x) => x).map((x) => parseInt(x));
  rest = remaining;
  const board = [a1, b1, c1, d1, e1];
  boards = [...boards, board];
}

// log(boards);

part1(); // 33462, 33462
part2(); // 1924, 30070

function part1() {
  let markedNumbers: number[] = [];
  for (const n of numbers) {
    // log("draw number", n);
    markedNumbers = [...markedNumbers, n];
    const winningBoard = boards.find((b) => boardHasBingo(b, markedNumbers));
    if (winningBoard) {
      // log(winningBoard);
      const unmarkedNumbers = winningBoard.flat().filter((n) =>
        !markedNumbers.includes(n)
      );
      // log(unmarkedNumbers);
      const unmarkedSum = sum(unmarkedNumbers);
      // log(unmarkedSum);
      const lastNumber = n;
      const score = unmarkedSum * lastNumber;

      // log();
      log("part1\t", score);
      break;
    }
  }
}

function part2() {
  let markedNumbers: number[] = [];
  let winningBoards: Board[] = [];

  outerLoop:
  for (const n of numbers) {
    // log("draw number", n);
    markedNumbers = [...markedNumbers, n];

    const newWinningBoards = boards.filter((b) =>
      boardHasBingo(b, markedNumbers) && !winningBoards.includes(b)
    );

    for (const winningBoard of newWinningBoards) {
      // log("winningBoard", winningBoard);

      winningBoards = [...winningBoards, winningBoard];
      // log("winningBoards", winningBoards.length, "/", boards.length);

      if (winningBoards.length === boards.length) {
        const unmarkedNumbers = winningBoard.flat().filter((n) =>
          !markedNumbers.includes(n)
        );
        // log("unmarkedNumbers", unmarkedNumbers);
        const unmarkedSum = sum(unmarkedNumbers);
        // log("unmarkedSum", unmarkedSum);
        const lastNumber = n;
        const score = unmarkedSum * lastNumber;

        // log();
        log("part2\t", score);
        break outerLoop;
      }
    }
    // log("----");
  }
}

function boardHasBingo(board: Board, markedNumbers: number[]): boolean {
  // rows
  for (const i in array0toN(5)) {
    if (board[i].every((n) => markedNumbers.includes(n))) {
      return true;
    }
  }

  // columns
  for (const i in array0toN(5)) {
    if (board.every((r) => markedNumbers.includes(r[i]))) {
      return true;
    }
  }

  return false;
}

function sum(numbers: number[]) {
  return numbers.reduce((acc, n) => acc + n);
}

function array0toN(n: number) {
  return [...Array(n).keys()];
}
