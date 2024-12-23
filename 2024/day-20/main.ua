# https://adventofcode.com/2024/day/20

# uiua 0.15.0-dev.1
# Experimental!

# # shortest ? maze
# ? maze
TryMaze ← (
  ⊸⊃($s♭⊚=@S|$e♭⊚=@E) # get s and e
  path(
    ⊙◌         # drop e
    ≡⌟+A₂      # get neighbors
    ▽⤚≡⌟(≠@#⊡) # filter out #
  )≍           # match
  □
)

# kan nog skrivas om med dubbel 'under'
GetCheats ← (
  ⊸=@#
  ↘₁↘₋₁
  ⍜⍉(↘₁↘₋₁)
  +₁ ⊚
)

&fras"example.txt"
$m⊜∘⊸≠@\n

⟜TryMaze
GetCheats
# ↙1↘6
≡⌟⍜⊡⋅@.
≡TryMaze
⊂:

⍚⊢ # get one of the shortest
# ⍚(⨬@\s@!°⊚) # visualize
≡◇⧻ # get lengths
⟜/↥ # get max length
-   # get diffs
⍆   # sort
▽⊸≠0

⊸⊛
# ⊕⧻
⊕([⊃⧻⊢])

# The map consists of track (.) - including the start (S) and end (E) positions (both of which also count as track) - and walls (#).

# When a program runs through the racetrack, it starts at the start position. Then, it is allowed to move up, down, left, or right; each such move takes 1 picosecond. The goal is to reach the end position as quickly as possible. In this example racetrack, the fastest time is 84 picoseconds.

# Because there is only a single path from the start to the end and the programs all go the same speed, the races used to be pretty boring. To make things more interesting, they introduced a new rule to the races: programs are allowed to cheat.

# The rules for cheating are very strict. Exactly once during a race, a program may disable collision for up to 2 picoseconds. This allows the program to pass through walls as if they were regular track. At the end of the cheat, the program must be back on normal track again; otherwise, it will receive a segmentation fault and get disqualified.

# Each cheat has a distinct start position (the position where the cheat is activated, just before the first move that is allowed to go through walls) and end position; cheats are uniquely identified by their start position and end position.

# In this example, the total number of cheats (grouped by the amount of time they save) are as follows:

# There are 14 cheats that save 2 picoseconds.
# There are 14 cheats that save 4 picoseconds.
# There are 2 cheats that save 6 picoseconds.
# There are 4 cheats that save 8 picoseconds.
# There are 2 cheats that save 10 picoseconds.
# There are 3 cheats that save 12 picoseconds.
# There is one cheat that saves 20 picoseconds.
# There is one cheat that saves 36 picoseconds.
# There is one cheat that saves 38 picoseconds.
# There is one cheat that saves 40 picoseconds.
# There is one cheat that saves 64 picoseconds.

# You aren't sure what the conditions of the racetrack will be like, so to give yourself as many options as possible, you'll need a list of the best cheats. How many cheats would save you at least 100 picoseconds?
