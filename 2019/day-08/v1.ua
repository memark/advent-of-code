# https://adventofcode.com/2019/day/8

ReadTrimFile ← ▽≠@\n. &fras
CalcAnswer ← ×⊙⊡∶⊙⊡. ⊃⊃(⊗/↧.≡/+=0)(≡/+=1)(≡/+=2) ↯⊓'~[/×]'-@0

&pf "Part 1: "
&p CalcAnswer 6_25 ReadTrimFile "./input.txt"
