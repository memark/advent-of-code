SplitLines ← ⊜□≠, @\n

NumVowels ← /+/+ ≡= "aoeui" ¤
---
⍤:≍, 3 NumVowels "aei"
⍤:≍, 3 NumVowels "xazegov"
⍤:≍, 15 NumVowels "aeiouaeiouaeiou"
---
ThreeVowels ← ≥3 NumVowels

OneDoubleLetter ← ≥2 /↥ ⊐≡⧻ ⊜□. -@\0
---
⍤:≍, 1 OneDoubleLetter "xx"
⍤:≍, 1 OneDoubleLetter "abcdde"
⍤:≍, 1 OneDoubleLetter "aabbccdd"
---

ContainsForbidden ← /↥/↥⊐≡⌕ {"ab" "cd" "pq" "xy"} ¤

IsNice ← ××⊃(ThreeVowels|OneDoubleLetter|-:1ContainsForbidden)
---
⍤:≍, 1 IsNice "ugknbfddgicrmopn"
⍤:≍, 1 IsNice "aaa"
⍤:≍, 0 IsNice "jchzalrnumimnmhp"
⍤:≍, 0 IsNice "haegwjzuvuyypxyu"
⍤:≍, 0 IsNice "dvszwmarrgswjxmb"
---

SolvePartOne ← /+ ≡IsNice SplitLines
---
⍤:≍, 236 SolvePartOne &fras "input"
---

# SolvePartTwo ← /+ ⊐≡(CalcRibbon ParseRow) SplitLines
# ---
# ⍤:≍, 3812909 SolvePartTwo &fras "input"
# ---

&p $"Part 1: _" SolvePartOne &fras "input"
# &p $"Part 2: _" SolvePartTwo &fras "input"
