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

IdentityMatrix ← |1 ⊞=.⇡

TwoSeqs ← ↯⇌⊂[2]⌊÷2⧻.
TwoDropOneSeqs ← ≡⍜⇌(↘1) ↯⇌⊂[3]⌊÷3⧻.

HasDoublePairA ← ≥1+ ∩(≥2 /↥ ⍘⊚⊛ TwoSeqs) ↘1.
HasDoublePairB ← ≥1+ ∩(≥2 /↥ ⍘⊚⊛ TwoDropOneSeqs) ↘1.
HasDoublePair ← ≥1+⊃(HasDoublePairA|HasDoublePairB)
---
⍤:≍, 1 HasDoublePair "xyxy"
⍤:≍, 1 HasDoublePair "axyxy"
# ⍤:≍, 1 HasDoublePair "xxyxx"
⍤:≍, 1 HasDoublePair "aabcdefgaa"
⍤:≍, 0 HasDoublePair "aaa"
---

HasXyX ← /↥ ≡(≍⊃(↙1|↙¯1)) ◫3
---
⍤:≍, 1 HasXyX "xyx"
⍤:≍, 1 HasXyX "xxx"
⍤:≍, 0 HasXyX "xxy"
⍤:≍, 1 HasXyX "abcdefeghi"
---

# MostCommonRow ← ⊏⊢⍖⍘⊚⊛:⊝.

IsNicePartTwo ← ×⊃(HasDoublePair|HasXyX)
---
⍤:≍, 1 IsNicePartTwo "qjhvhtzxzqqjkmpb"
# ⍤:≍, 0 IsNicePartTwo "xxyxx"
---

TwoDropOneSeqs ← ≡⍜⇌(↘1) ↯⇌⊂[3]⌊÷3⧻.

"xxyxx"
↘1.
∩(TwoDropOneSeqs)
# ∩(⍘⊚⊛)
# ∩(/↥)
# ∩(≥2)
# ≥1+
