Lines ← partition box != over @\n
SplitBy ← partition box != over
Trim ← keep * under(couple reverse) rows scan max duplicate != @ duplicate
GetRGB ← (
  SplitBy @ over
  pack rows Trim
  pack rows SplitBy @ 
  reverse select rise rows pick 1 duplicate
  rows (under first pack parse)
)
EnsureAllColors ← (
  assert "empty string" >0 len duplicate
  ($"_, 0 red"|∘) reduce max find "red" duplicate
  ($"_, 0 blue"|∘) reduce max find "blue" duplicate
  ($"_, 0 green"|∘) reduce max find "green" duplicate
)
DropColorNames ← pack rows first
GetGameId ← pack (parse first reverse SplitBy @ ) firstSplitBy @:
GetRounds ← pack Trim first reverse SplitBy @:
SplitRounds ← pack rows Trim SplitBy @;
RoundIsPossible ← ≥0 reduce min -
GameIsPossible ← first reduce * cross RoundIsPossible dip fix
GameStringIsPossible ← (
  pack rows (
    DropColorNames GetRGB EnsureAllColors
  ) SplitRounds GetRounds
  GameIsPossible
)

SolvePartOne ← (
  pack rows fork(GetGameId)(GameStringIsPossible flip 12_13_14) Lines
  reduce + keep flip
)
SolvePartTwo ← (
  reduce + pack rows (
    reduce * reduce max pack rows (
      DropColorNames GetRGB EnsureAllColors
    ) SplitRounds GetRounds
  ) Lines
)

&p $"Part 1: _" SolvePartOne &fras "input"
&p $"Part 2: _" SolvePartTwo &fras "input"
