Lib ← &i "../../lib.ua"
  SplitLines ← Lib "SplitLines"
  SplitByDelimiter ← Lib "SplitByDelimiter"
  StartsWith ← Lib "StartsWith"

# SolvePartTwo ← x SplitLines
# ---
# ⍤:≍, x SolvePartTwo &fras "example_1"
# ⍤:≍, x SolvePartTwo &fras "input"
# ---

# &p $"Part 1: _" SolvePartOne &fras "input"
# &p $"Part 2: _" SolvePartTwo &fras "input"

# ---

# example 5
# input 10

# SplitLines &fras "example_1"

GetCardNumFromLine ← parse⊔⊡1⊐SplitByDelimiter @ ⊡0⊐SplitByDelimiter @: ⊡0⊐SplitByDelimiter @|
# GetCardNumFromLine "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"

GetWinNumsFromLine ← ⊐≡parse ⊐SplitByDelimiter @ ⊡1⊐SplitByDelimiter @: ⊡0⊐SplitByDelimiter @|
# GetWinNumsFromLine "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"

GetOwnNumsFromLine ← ⊐≡parse ⊐SplitByDelimiter @ ⊡1⊐SplitByDelimiter @|
# GetOwnNumsFromLine "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"

GetNumMatchesFromLine ← /+ ∊ ⊃(GetOwnNumsFromLine|GetWinNumsFromLine)

GetScoreFromLine ← (0|ⁿ:2-1) >0. GetNumMatchesFromLine
---
⍤:≍, 8 GetScoreFromLine "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"
⍤:≍, 0 GetScoreFromLine "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
---

SolvePartOne ← /+ ⊐≡GetScoreFromLine SplitLines
---
⍤:≍, 13 SolvePartOne &fras "example_1"
⍤:≍, 21568 SolvePartOne &fras "input"
---

# --- Part 2

# börja med att parsa till en struktur
ParseLine ← [⊃(GetCardNumFromLine|GetWinNumsFromLine|GetOwnNumsFromLine)]

M ← ⊐≡ParseLine SplitLines &fras "example_1"
# &s M

# kunna få ut ett visst kort (ursprungliga varianten iaf)
GetCardFromMatrix ← ⊡-1

GetNumMatchesFromParsedLine ← /+ ⊐∊ ⊃(⊡1|⊡2)

# ta en parsad rad och hela matrisen
# returnera ett antal nya parsade rader
# (under detta i stacken ligger hela tiden totala antalet kort)
# r M
TheFunc ← ☇1 ⊠(GetCardFromMatrix) ⊙¤ +1+⇡ ⊃(GetNumMatchesFromParsedLine|⊔⊢)
# TheFunc ⊡3. M
# TheFunc ⊡2. M
# TheFunc ⊡4. M
# ⊔⊢▽>0≡⧻. ⊐⊂

M

⊐≡(TheFunc :M) .
⊐⊂ &s.
⊔⊢▽>0≡⧻.
⊐/⊂
⊔⊢▽>0≡⧻.
⊔⊢▽>0≡⧻.

# ⊔⊢▽>0≡⧻.
# ⊔⊢⊢⊔⊢ ⊔⊢⊢ ⊐/⊂⊐≡(TheFunc :M ⸮) .
# ⊔⊢⊢ ⊐/⊂⊐≡(TheFunc :M ⸮) .
# ⊔⊢⊢ ⊐/⊂⊐≡(TheFunc :M ⸮) .

# ⊔⊢⊢ ⊐/⊂⊐≡(TheFunc :M) .
# ⊢⊢ ⊐/⊂⊐≡(TheFunc :M) M
# ⊢⊢ ⊐/⊂⊐≡(TheFunc :M) M

# ⧻
# r ←
# :⧻.
# ≡GetNumMatchesFromLine

# gå igenom varje rad (håll ett index? kommer behöva vara skilt från card num)

# räkna ut poäng
# för varje poäng, hitta kopia av resp kort
