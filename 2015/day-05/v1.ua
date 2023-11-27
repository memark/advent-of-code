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

IsNicePartOne ← ××⊃(ThreeVowels|OneDoubleLetter|-:1ContainsForbidden)
---
⍤:≍, 1 IsNicePartOne "ugknbfddgicrmopn"
⍤:≍, 1 IsNicePartOne "aaa"
⍤:≍, 0 IsNicePartOne "jchzalrnumimnmhp"
⍤:≍, 0 IsNicePartOne "haegwjzuvuyypxyu"
⍤:≍, 0 IsNicePartOne "dvszwmarrgswjxmb"
---

SolvePartOne ← /+ ≡IsNicePartOne SplitLines
---
⍤:≍, 236 SolvePartOne &fras "input"
---

# SolvePartTwo ← /+ ⊐≡(CalcRibbon ParseRow) SplitLines
# ---
# ⍤:≍, 3812909 SolvePartTwo &fras "input"
# ---

# &p $"Part 1: _" SolvePartOne &fras "input"
# &p $"Part 2: _" SolvePartTwo &fras "input"

# ---

"aabcdefgaa"
-@\0.
⊐⊜∘
# ≡⧻
# ⊐≡(↙↧2⧻.)
# ⊐(⊜∘.)
⊏⍏.
▽≥2≡⧻.
⊐≡⊢

"aabcdefgaaff"
:⊝.
⍘⊚⊛
▽≥2

# ¤:
dump∘
⊞(=dump∘)
