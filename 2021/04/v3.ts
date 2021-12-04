type Board = number[][];

const log = console.log;

const data = Deno
  // .readTextFileSync("example.txt");
  .readTextFileSync("input.txt");

const [a, b] = splitOnce(data, "\n\n");
const numbers = a!.split(",").map((x) => parseInt(x));
const boards = b!.split("\n\n").map(parseBoard);

function parseBoard(s: string): Board {
  return s.split("\n").map((x) => x.trim().split(/ +/).map((x) => parseInt(x)));
}

log(
  "part1\t",
  calculateScore(someWinningBoard(firstCondition, boards, numbers)),
); // 4512, 33462
log(
  "part2\t",
  calculateScore(someWinningBoard(lastCondition, boards, numbers)),
); //  1924, 30070

function someWinningBoard(
  condition: (
    boards: Board[],
    newMarked: number[],
  ) => boolean,
  boards: Board[],
  numbers: number[],
  marked: number[] = [],
): [Board, number[]] {
  const [n, ...remaining] = numbers;
  const newMarked = [...marked, n];

  const allWinning = boards.filter(boardHasBingo(newMarked));
  const newWinning = allWinning.filter((b) => !boardHasBingo(marked)(b));

  return condition(boards, newMarked)
    ? [newWinning[0], newMarked]
    : someWinningBoard(condition, boards, remaining, newMarked);
}

// kan man tom bara skicka in boards.some och boards.every?
// eller array.some och array.every?

function firstCondition(boards: Board[], newMarked: number[]) {
  return boards.some(boardHasBingo(newMarked));
}

function lastCondition(boards: Board[], newMarked: number[]) {
  return boards.every(boardHasBingo(newMarked));
}

function boardHasBingo(markedNumbers: number[]): (board: Board) => boolean {
  return (board) => {
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
  };
}

function calculateScore([board, marked]: [_: Board, _: number[]]) {
  const unmarked = board.flat().filter((n) => !marked.includes(n));
  return sum(unmarked) * marked.at(-1)!;
}

function splitOnce(s: string, on: string) {
  const [first, ...rest] = s.split(on);
  return [first, rest.length > 0 ? rest.join(on) : null];
}

function sum(numbers: number[]) {
  return numbers.reduce((acc, n) => acc + n);
}

function array0toN(n: number) {
  return [...Array(n).keys()];
}
