# https://adventofcode.com/2015/day/1

AccFloors ← \+-⊃(=@))(=@()

SolvePartOne ← ⊢⇌ AccFloors
┌─╴test
  ⍤:≍, ¯3 SolvePartOne ")())())"
  ⍤:≍, 138 SolvePartOne &fras "input"
└─╴

SolvePartTwo ← +1 ⊗¯1 AccFloors
┌─╴test
  ⍤:≍, 5 SolvePartTwo "()())"
└─╴

&p $"Part 1: _" SolvePartOne &fras "input"
&p $"Part 2: _" SolvePartTwo &fras "input"
