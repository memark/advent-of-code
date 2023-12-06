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

ParseSeedRow ← ⊐≡parse ⊐SplitByDelimiter@ # 

ParseMapRow ← ⊐≡parse SplitByDelimiter@ # 
ParseMapSection ← ⊐≡ParseMapRow SplitByDelimiter@\n

# jag har tagit bort alla "seeds: " och "bla bla mapping: "
ParseFixedFile ← (
  -:1⌕ "\n\n" .
  ⊜□
  ⊂⊃(
    □ParseSeedRow ⊢
  | ⊐≡ParseMapSection ↘1
  )
)

M ← ParseFixedFile &fras ("example_1"|"input") 0

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

# det är ju här allt krångel sitter.
# vet väl knappt vad jag gör med alla dessa deshapes och fixes.
# num MapSection
TranslateWithMapSection ← (
  # &s "TranslateWithMapSection 1"
  ⊙(⊐▽ ♭ ⊐⊠MatchesMapRow ¤ ⊙.) .
  # Any source numbers that aren't mapped correspond to the same destination number.
  # jag borde asserta på att det inte är fler än en som mappar
  # just nu räknar jag innehållet i raden jag fick, (dst, src, rng)...
  # &s "TranslateWithMapSection 2"
  # =1 ⸮ ⧻,
  # ⍤..
  ⧻,
  ⍤"bork"≤1.
  (∘|ApplyMapRow ⊙♭)
  # &s "TranslateWithMapSection 3"
)

Seeds ← ⊔⊢M
# &s Seeds

SeedRanges ← (
  Seeds
  ↯⊂:2 ÷2⧻.
)
&s SeedRanges
&s ⊏⍏. SeedRanges

RangedSeeds ← (
  SeedRanges
  ↙2
  ⊐≡(
    +⇡ :⊃(Fst|Snd)
  )
  ⊐/⊂
  &s $"Shape: _" △.
  ⬚100↙100
  # &s $"Shape: _" △.
)
# &s $"Shape: _" △RangedSeeds
# &s RangedSeeds
# ⍤.0

# &s ⊔⊡1M
# &s ⍜(⍉↘1⍉)(⊏⍏.) ⊔⊡1M

# jag borde byta ordning, så att seed är underst, blir nog enklare överallt, då fokus är på att dra ett seed genom processen, inte att dra en matris/sektion
(Seeds|RangedSeeds) 1
# &s .
♭⊠TranslateWithMapSection :⊡1M &s "phase 1"
♭⊠TranslateWithMapSection :⊡2M &s "phase 2"
♭⊠TranslateWithMapSection :⊡3M &s "phase 3"
♭⊠TranslateWithMapSection :⊡4M &s "phase 4"
♭⊠TranslateWithMapSection :⊡5M &s "phase 5"
♭⊠TranslateWithMapSection :⊡6M &s "phase 6"
♭⊠TranslateWithMapSection :⊡7M &s "phase 7"
/↧

≡(|1
  ⊃(Snd|Fst)
  ⍥(
    +1

    # &s TranslateWithMapSection :⊡1M
  )
) SeedRanges
