Lines ← ⊜□≠, @\n
SplitBy ← ⊜□≠,
Trim ← ▽×⍜(⊟⇌)≡\↥.≠@ .
GetRGB ← |1 (
  SplitBy @,
  ⊐≡Trim
  ⊐≡SplitBy @ 
  ⇌⊏⍏≡⊡1.
  ≡(⍜⊢⊐parse)
)
EnsureAllColors ← |1 (
  ⍤"empty string" >0⧻.
  ($"_, 0 red"|∘) /↥⌕"red" .
  ($"_, 0 blue"|∘) /↥⌕"blue" .
  ($"_, 0 green"|∘) /↥⌕"green" .
)
DropColorNames ← ⊐≡⊢
GetGameId ← |1 ⊐(parse⊢⇌SplitBy @ ) ⊢SplitBy @:
GetRounds ← |1 ⊐Trim ⊢⇌SplitBy @:
SplitRounds ← |1 ⊐≡Trim SplitBy @;
RoundIsPossible ← |2 ≥0/↧-
GameIsPossible ← ⊢ /× ⊠RoundIsPossible ⊙¤
GameStringIsPossible ← (
  ⊐≡(
    DropColorNames GetRGB EnsureAllColors
  ) SplitRounds GetRounds
  GameIsPossible
)

SolvePartOne ← (
  /+▽: ⊐≡⊃(GetGameId|GameStringIsPossible :12_13_14) Lines
)
SolvePartTwo ← (
  /+⊐≡(
    /×/↥ ⊐≡(
      DropColorNames GetRGB EnsureAllColors
    ) SplitRounds GetRounds
    ) Lines
)

&p $"Part 1: _" SolvePartOne &fras "input"
&p $"Part 2: _" SolvePartTwo &fras "input"
