import qualified Data.Char as Char
import qualified Data.List as List

main = do
  input <- readFile "input.txt"

  -- Part One -- 5346030
  print (runProgram 0 [1] [] . parse $ input)

  -- Part Two -- 513116
  print (runProgram 0 [5] [] . parse $ input)

type Input = [Int]

type Output = [Int]

type Memory = [Int]

runProgram :: Int -> Input -> Output -> Memory -> Output
runProgram ip inp outp mem
  | op == 01 = op_add
  | op == 02 = op_multiply
  | op == 03 = op_input
  | op == 04 = op_output
  | op == 05 = op_jump_if_true
  | op == 06 = op_jump_if_false
  | op == 07 = op_less_than
  | op == 08 = op_equals
  | op == 99 = outp

  where
    op_input         = runProgram (ip+1 + 1) (tail inp)        outp  (update p1 (head inp)  mem)
    op_output        = runProgram (ip+1 + 1)       inp  (pv1 : outp)                        mem
    jump nip         = runProgram nip              inp         outp                         mem
    store val        = runProgram (ip+1 + 3)       inp         outp  (update p3 val         mem)

    op_add           = store (pv1 + pv2)
    op_multiply      = store (pv1 * pv2)
    op_less_than     = store (if pv1 <  pv2 then 1 else 0)
    op_equals        = store (if pv1 == pv2 then 1 else 0)
    op_jump_if_true  = jump  (if pv1 /= 0 then pv2 else ip+1 + 2)
    op_jump_if_false = jump  (if pv1 == 0 then pv2 else ip+1 + 2)

    (op, p1m, p2m, p3m) = getModes $ mem !! ip
    p1  = mem !! (ip + 1)
    p2  = mem !! (ip + 2)
    p3  = mem !! (ip + 3)
    pv1 = if p1m == 1 then p1 else mem !! p1
    pv2 = if p2m == 1 then p2 else mem !! p2

getModes opx =
  ( opx `mod` 100,
    opx `div` 100 `mod` 10,
    opx `div` 1000 `mod` 10,
    opx `div` 10000 `mod` 10
  )

update n x a = take n a ++ [x] ++ drop (n + 1) a

parse = map toInt . splitBy ',' . trim

toInt x = read x :: Int

splitBy _ [] = []
splitBy c s =
  let i = (length . takeWhile (/= c)) s
      (as, bs) = splitAt i s
   in as : splitBy c (if null bs then [] else tail bs)

trim = List.dropWhileEnd Char.isSpace . dropWhile Char.isSpace
