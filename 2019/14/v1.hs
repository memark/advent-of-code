import qualified Data.Char as Char
import qualified Data.List as List
import qualified Data.Map.Strict as Map
import Data.Map.Strict ((!), (!?), Map)
import qualified Data.Set as Set
import qualified Control.Exception as Ex
import qualified Control.Monad
import Text.Printf ( printf )
import Data.Tuple.Select ()
import Data.List.Split (splitOn)


sample_1a = "\
\10 ORE => 10 A\n\
\1 ORE => 1 B\n\
\7 A, 1 B => 1 C\n\
\7 A, 1 C => 1 D\n\
\7 A, 1 D => 1 E\n\
\7 A, 1 E => 1 FUEL"

-- \3 ORE => 1 B\n\
-- \3 A, 1 B => 1 FUEL

type C = String
type Q = Int
type I = (Q, C)
type O = (Q, C)
type R = ([I], O)
type T = Map C Q

type RS = Map C (Q, [(C, Q)])

main = do
  putStrLn ""
  input <- readFile "input.txt"

  -- let d = parse sample_1a
  -- putStrLn . List.intercalate "\n" . map show $ d
  -- putStrLn ""

  -- print $ makeChem d "FUEL"
  -- putStrLn ""

  -- let rs' = parse' sample_2
  -- test rs' "A"
  -- test rs' "B"
  -- test rs' "FUEL"
  -- putStrLn ""

  let rs' = parse' sample_3
  print rs'
  putStrLn ""
  test rs' "A"
  test rs' "B"
  test rs' "FUEL"
  putStrLn ""

  -- Part One -- 
  -- Part Two -- 
  putStr ""


test rs' c =
  printf "%4s => %2d (%s)\n" c (fst res) (show $ snd res)
    where res = makeChem' rs' Map.empty c

sample_1 = "\
\4 ORE => 3 A\n\
\7 A => 1 FUEL"

sample_2 = "\
\2 ORE => 3 A\n\
\2 A => 1 B\n\
\2 B => 1 FUEL"

sample_3 = "\
\1 ORE => 1 A\n\
\1 A => 1 B\n\
\1 A, 1 B => 1 FUEL"


makeChem' :: RS -> T -> C -> (Q, T)
makeChem' _ t "ORE" = (1, t)
makeChem' rs t c = 
  let
    -- bryt ut och gör samma sak som nedan fast för "is"
    (q, (ic,iq):is) = rs!c
      where a = 2 -- det går bra att ha where på detta sätt
    have = Map.findWithDefault 0 ic t
    need = iq - have
    batchSize = fst $ Map.findWithDefault (1,[]) ic rs
    noBatches = getNoBatches need batchSize
    made = noBatches * batchSize
    left = made - need
    t' = Map.insert ic left t
    rec = makeChem' rs t' ic
    cost = noBatches * fst rec
  in
    (cost, snd rec)


getNoBatches need batch =
  if need `mod` batch == 0
  then need `div` batch
  else need `div` batch + 1


parse' :: String -> Map C (Q, [(C, Q)])
parse' = Map.fromList . map parseLine . lines . trim
  where 
    parseLine :: String -> (C, (Q, [(C, Q)]))
    parseLine = parseParts . toTuple . splitOn " => "

    parseParts :: (String, String) -> (C, (Q, [(C, Q)]))
    parseParts (is, o) = (fst po, (snd po, pis))
      where pis = parseInputs is
            po = parseNumChem o

    parseInputs :: String -> [(C, Q)]
    parseInputs a = map parseNumChem . splitOn ", " $ a

    parseNumChem :: String -> (C, Q)
    parseNumChem b = case splitOn " " b of [q,c] -> (c, read q :: Int)


makeChem rs "ORE" = 1
makeChem rs chem = sum $ map (\ i -> fst i * makeChem rs (snd i) `div` oq) is
  where 
    (oq,oc) = o
    (is,o) = findR rs chem


findR :: [R] -> C -> R
findR rs oc =
  case f rs of Just c -> c
               Nothing -> error (show oc ++ " was not found")
    where f = List.find (\ (is,o) -> snd o == oc)


parse :: String-> [([I], O)]
parse = map parseLine . lines . trim
  where parseLine = parseParts . toTuple . splitOn " => "
        parseParts (inputs, output) = (parseInputs inputs, parseNumChem output)
        parseInputs a = map parseNumChem . splitOn ", " $ a
        parseNumChem b = case splitOn " " b of [b1,b2] -> (read b1 :: Int,b2)

toTuple [a,b] = (a,b)


-- splitBy _ [] = []
-- splitBy c s  =
--   let
--     i = (length . takeWhile (/= c)) s
--     (as, bs) = splitAt i s
--   in as : splitBy c (if null bs then [] else tail bs)


trim = List.dropWhileEnd Char.isSpace . dropWhile Char.isSpace