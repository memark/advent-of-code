Lib ← &i "../../lib.ua"
  SplitLines ← Lib "SplitLines"

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

# det finns dubletter av part numbers!

# alla nummer är 1-999

IsDigit ← ×⊃(≥@0|≤@9)
IsSymbol ← ×⊃(-:1IsDigit|≠@.)

M ← ⊐SplitLines &fras ("example_1"|"input") 0
# M ← ↯ ⊂.140 M
&s M
&s △M

# koord på alla part numbers
PNC ← ⊚ IsDigit M
# PNC

# koord på alla symboler
SC ← ⊚ IsSymbol M
# SC

# grannar
Nbs ← ⍜(↘4)(↘1) ☇1⊞(⊂).-1⇡3 # => 8 st
# Nbs

# koords grannar
KNbs ← ☇1 ≡+ Nbs ¤

LineNbs ← [0_1 0_¯1]
# LineNbs

# koords radgrannar
KLineNbs ← ☇1 ≡+ LineNbs ¤

# ⊚=@*M

# utgå från en siffra, hitta resten av siffrorna i numret
# (parsa siffran)
# kolla ev symbolgranne till alla siffrorna
# gör detta för alla siffror

# x y
ElementAt ← ⊡⊙⊡

EAtK ← ElementAt ⊃(⊢⇌|⊢)
# EAtK 2_3

# x y
# ElementAt 3 2 M
# y_x
# ElementAt ⊃(⊢⇌|⊢) 2_3 M

# y_x M
KIsWithinRange ← |2 /↧×⊃(≥0|<:⊙⧻)
---
⍤:≍, 1 KIsWithinRange 2_5 ↯ 10_10 0
⍤:≍, 0 KIsWithinRange ¯2_5 ↯ 10_10 0
⍤:≍, 0 KIsWithinRange 2_12 ↯ 10_10 0
---

FindWholeNumberK ← (
  ¤ # start

  ☇1≡KLineNbs .          # neighbors
  ▽≡(KIsWithinRange :M). # within range
  ≡(EAtK :M) .           # chars
  ▽IsDigit               # keep digits
  ⊝⊂                     # join with previous

  ☇1≡KLineNbs .          # neighbors
  ▽≡(KIsWithinRange :M). # within range
  ≡(EAtK :M) .           # chars
  ▽IsDigit               # keep digits
  ⊝⊂                     # join with previous
)

Temp ← (
  ⊚ IsDigit M
  ⊐≡FindWholeNumberK # find whole number koords
  ⊝ ⊐≡(⊏⍏.)          # sort and dedup
)

# För att få fram alla numbers.
Temp
⊐≡(≡(EAtK :M))
# ⊏⍏. ⊐≡parse

# Ta fram vilka number index som är nära en symbol.
Temp
⊐≡(☇1≡KNbs) # find all neighbors
⊐≡(
  ⊝ ⊏⍏. ⊐☇1
  ▽≡(KIsWithinRange :M).
  ≡(EAtK :M)
  ▽≡(IsSymbol).
  >0⧻
)

# Välj ut och parsa part numbers.
▽
⊐≡parse
⊃(/+⊝|/+)

# 328113 (unika) // too low
# 532331 (alla)  // denna var det!

# hitta alla *
AllStars ← (
  ⊚ =@* M
  # ↙1
)

AllStars
⊐≡(KNbs).
⊐≡(▽≡(KIsWithinRange :M).)
⊐≡(▽≡(IsDigit EAtK :M).)
⊐≡(≡FindWholeNumberK).
# ⊐≡(⊝≡(⊏⍏.)).
# ⊐≡△
# ▽=2≡⧻.
# ⊐≡(≡(≡(EAtK :M))).
# ≡(≡(∵parse⊔⸮∵⸮))
# ⊐∵(⊔∵⊔⸮)

# kolla deras grannar
# brancha ut på raden för helt nummer
# sortera, dedup
# räkna antalet
# räkna ihop svaret
