Lib ← &i "../../lib.ua"
  SplitLines ← Lib "SplitLines"
  SplitByDelimiter ← Lib "SplitByDelimiter"
  StartsWith ← Lib "StartsWith"
  Trim ← Lib "Trim"

# SolvePartOne ← x SplitLines
# ---
# ⍤:≍, x SolvePartOne &fras "example_1"
# ⍤:≍, x SolvePartOne &fras "input"
# ---

# SolvePartTwo ← x SplitLines
# ---
# ⍤:≍, x SolvePartTwo &fras "example_1"
# ⍤:≍, x SolvePartTwo &fras "input"
# ---

# &p $"Part 1: _" SolvePartOne &fras "input"
# &p $"Part 2: _" SolvePartTwo &fras "input"

# ---

# ⊕□ => group plockar bort alla \n och lägger i en egen bucket

# Seed -> Soil -> Fert -> Water -> Light -> Temp -> Humid -> Loc

ParseSeedRow ← ⊐≡parse ⊐SplitByDelimiter@ 

ParseMapRow ← ⊐≡parse SplitByDelimiter@ 
ParseMapSection ← ⊐≡ParseMapRow SplitByDelimiter@\n

ParseFixedFile ← (
  -:1⌕ "\n\n" .
  ⊜□
  ⊂⊃(
    □ParseSeedRow ⊢
  | ⊐≡ParseMapSection ↘1
  )
)

M ← ParseFixedFile &fras ("example_1"|"input") 1
&s $"M:"
&s M

Seed ← ⊐⊢⊢ M
&s "Seed:"
&s Seed

&s "---"
&s "Stack:"

Fst ← ⊡0
Snd ← ⊡1
Trd ← ⊡2

And ← ↧
Or ← ↥

MatchesLowerBound ← ≥Snd:
---
⍤:≍, 0 MatchesLowerBound 0 0_1_2
⍤:≍, 1 MatchesLowerBound 1 0_1_2
⍤:≍, 1 MatchesLowerBound 2 0_1_2
⍤:≍, 1 MatchesLowerBound 3 0_1_2
⍤:≍, 0 MatchesLowerBound 0 9_1_2
⍤:≍, 1 MatchesLowerBound 1 9_1_2
⍤:≍, 1 MatchesLowerBound 2 9_1_2
⍤:≍, 1 MatchesLowerBound 3 9_1_2
---

MatchesHigherBound ← <+⊃(Snd|Trd):
---
⍤:≍, 1 MatchesHigherBound 0 0_1_2
⍤:≍, 1 MatchesHigherBound 1 0_1_2
⍤:≍, 1 MatchesHigherBound 2 0_1_2
⍤:≍, 0 MatchesHigherBound 3 0_1_2
⍤:≍, 1 MatchesHigherBound 0 9_1_2
⍤:≍, 1 MatchesHigherBound 1 9_1_2
⍤:≍, 1 MatchesHigherBound 2 9_1_2
⍤:≍, 0 MatchesHigherBound 3 9_1_2
---

MatchesMapRow ← And⊃(MatchesLowerBound|MatchesHigherBound)
---
⍤:≍, 0 MatchesMapRow 79 50_98_2
⍤:≍, 1 MatchesMapRow 79 52_50_48
---

# vi har redan kollat att den matchar
ApplyMapRow ← +-:⊙(:⊃(Fst|Snd))
---
⍤:≍, 81 ApplyMapRow 79 52_50_48
---

# num MapSection
TranslateWithMapSection ← (
  ⊙(♭⊐▽ ♭⊐⊠MatchesMapRow ¤ ⊙.) .
  # Any source numbers that aren't mapped correspond to the same destination number.
  # jag borde asserta på att det inte är fler än en som mappar
  # just nu räknar jag innehållet i raden jag fick, (dst, src, rng)...
  (∘|ApplyMapRow) ≥1⧻,
)

# TranslateWithMapSection Seed ⊡1M

Seeds ← ⊔⊢M

# jag borde byta ordning, så att seed är underst, blir nog enklare överallt, då fokus är på att dra ett seed genom processen, inte att dra en matris/sektion
Seeds
♭⊠TranslateWithMapSection :⊡1M
♭⊠TranslateWithMapSection :⊡2M
♭⊠TranslateWithMapSection :⊡3M
♭⊠TranslateWithMapSection :⊡4M
♭⊠TranslateWithMapSection :⊡5M
♭⊠TranslateWithMapSection :⊡6M
♭⊠TranslateWithMapSection :⊡7M
/↧

#!!! Any source numbers that aren't mapped correspond to the same destination number. !!!
