import qualified Data.Char as Char
import qualified Data.List as List

main = do
    input <- readFile "input.txt"
    let parsedInput = parse input

    -- Part One -- 3895705
    print (runProgram 0 . replace1and2 12 02 $ parsedInput)

    -- Part Two -- 6417
    let (noun, verb) = searchForResult parsedInput [0..99] [0..99] 19690720
    print (100 * noun + verb)

parse = map toInt . splitBy ',' . trim

toInt x = read x :: Int

replace1and2 v1 v2 = replaceNth 1 v1 . replaceNth 2 v2 

replaceNth n x a = take n a ++ [x] ++ drop (n+1) a

runProgram :: Int -> [Int] -> Int
runProgram ip mem
  | op == 99 = head mem
  | op == 1 = runProgram (ip + 4) (replaceNth p3 (v1 + v2) mem)
  | op == 2 = runProgram (ip + 4) (replaceNth p3 (v1 * v2) mem)
  where op = mem !! ip
        p1 = mem !! (ip + 1)
        v1 = mem !! p1
        p2 = mem !! (ip + 2)
        v2 = mem !! p2
        p3 = mem !! (ip + 3)

searchForResult parsedInput nounRange verbRange target =
  head [(noun, verb) |
        noun <- [0..99],
        verb <- [0..99],
        let result = runProgram 0 . replace1and2 noun verb $ parsedInput,
        result == target]

splitBy _ [] = []
splitBy c s  =
  let
    i = (length . takeWhile (/= c)) s
    (as, bs) = splitAt i s
  in as : splitBy c (if null bs then [] else tail bs)

trim = List.dropWhileEnd Char.isSpace . dropWhile Char.isSpace