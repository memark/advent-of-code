Lib ← &i "../../lib.ua"
  SplitLines ← Lib "SplitLines"

SplitByDelimiter ← ⊜□≠,
TrimWhitespace ← ▽×⍜(⊟⇌)≡\↥.≠@ .

# BUGG?
# '@ ' sist på rad formatteras så att mellanslaget försvinner

GetRGB ← |1 (
  SplitByDelimiter @,
  ⊐≡TrimWhitespace
  ⊐≡(SplitByDelimiter @ )
  ⇌⊏⍏≡⊡1. # => r g b
  ≡(⍜⊢⊐parse)
)
---
⍤:≍, {20 "red"}_{8 "green"}_{6 "blue"} GetRGB "8 green, 6 blue, 20 red"
---

EnsureAllColors ← |1 (
  ⍤"empty string" >0⧻.
  ($"_, 0 red"|∘) /↥⌕"red" .
  ($"_, 0 blue"|∘) /↥⌕"blue" .
  ($"_, 0 green"|∘) /↥⌕"green" .
)
---
⍤:≍, "2 green, 0 red, 0 blue" EnsureAllColors "2 green"
⍤:≍, "2 red, 0 blue, 0 green" EnsureAllColors "2 red"
⍤:≍, "2 blue, 0 red, 0 green" EnsureAllColors "2 blue"
# todo: test empty string
---

DropColorNames ← ⊐≡⊢
---
⍤:≍, 20_8_6 DropColorNames {20 "red"}_{8 "green"}_{6 "blue"}
---

---
⍤:≍, 0_2_0 DropColorNames GetRGB EnsureAllColors "2 green"
---

GetGameId ← |1 ⊐(parse⊢⇌SplitByDelimiter @ ) ⊢SplitByDelimiter @:
---
⍤:≍, 5 GetGameId "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
---

GetRounds ← |1 ⊐TrimWhitespace ⊢⇌SplitByDelimiter @:
---
⍤:≍, "3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green" GetRounds "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"
---

SplitRounds ← |1 ⊐≡TrimWhitespace SplitByDelimiter @;
---
⍤:≍, {"3 blue, 4 red" "1 red, 2 green, 6 blue" "2 green"} SplitRounds "3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"
---

# Round Bag
RoundIsPossible ← |2 ≥0/↧-
---
⍤:≍, 1 RoundIsPossible 4_0_3 5_5_5
⍤:≍, 0 RoundIsPossible 4_0_3 3_3_3
---

# Game Bag
GameIsPossible ← ⊢ /× ⊠RoundIsPossible ⊙¤
---
⍤:≍, 1 GameIsPossible [4_0_3 1_2_6 0_2_0] 12_13_14
⍤:≍, 0 GameIsPossible [4_0_3 1_14_6 0_2_0] 12_13_14
---

# GameString Bag
GameStringIsPossible ← GameIsPossible ⊐≡(DropColorNames GetRGB EnsureAllColors) SplitRounds GetRounds
---
⍤:≍, [1 1 0 0 1] ⊐≡(GameStringIsPossible :12_13_14) SplitLines &fras "example_1"
---

SolvePartOne ← /+▽: ⊐≡⊃(GetGameId|GameStringIsPossible :12_13_14) SplitLines
---
⍤:≍, 8 SolvePartOne &fras "example_1"
⍤:≍, 2331 SolvePartOne &fras "input"
---

SolvePartTwo ← /+⊐≡(/×/↥ ⊐≡(DropColorNames GetRGB EnsureAllColors) SplitRounds GetRounds) SplitLines
---
⍤:≍, 2286 SolvePartTwo &fras "example_1"
⍤:≍, 71585 SolvePartTwo &fras "input"
---

&p $"Part 1: _" SolvePartOne &fras "input"
&p $"Part 2: _" SolvePartTwo &fras "input"
