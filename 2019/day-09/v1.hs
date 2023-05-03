import qualified Data.Char as Char
import qualified Data.List as List
import qualified Data.Map as Map
import qualified Control.Exception as Ex
import qualified Control.Monad
import Text.Printf

sample_1a = "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99"
sample_1b = "1102,34915192,34915192,7,4,7,99,0"
sample_1c = "104,1125899906842624,99"

main = do
  input <- readFile "input.txt"
  input_05 <- readFile "../05/input.txt"

  -- print (parse $ sample_1a)
  -- print (parse_ $ sample_1a)

  -- printNice 0 . runProgram 0 0 [1] [] [] . parse_ $ input_05
  -- printNice 0 . runProgram 0 0 [5] [] [] . parse_ $ input_05


  -- print (runProgram 0 0 [] [] . parse_ $ sample_1b)

  -- Part One -- 
  -- print ((length . parse $ input, parse $ input))
  -- printNice 1 . runProgram 0 0 [1] [] [] . parse_ $ input

  printNice_ . parseProgram 0 . parse_ $ input

  -- putStrLn . unlines . map (\ x -> printf "[%03d] %d" (fst x) (snd x)) . Map.assocs . parse_ $ input

  -- Part Two -- 

  putStr ""

printNice c x = do
  Control.Monad.when (c==1) . putStrLn . unlines . snd $ x
  print (fst x)

printNice_ =
  putStrLn . unlines . take 50 . map unwords


-- 203, too low
--   0, incorrect

type Input = [Int]
type Output = [Int]
type Memory = Map.Map Int Int --[Int]
type RelBase = Int


parseProgram :: Int -> Memory -> [[[Char]]]
parseProgram ip mem
  | op == 01 = [d_ip_op, pad "add", show p1, show p2, show p3]:parseProgram (ip+1 + 3) mem
  | op == 02 = [d_ip_op, pad "mul", show p1, show p2, show p3]:parseProgram (ip+1 + 3) mem
  | op == 03 = [d_ip_op, pad "in", show p1]:parseProgram (ip+1 + 1) mem
  | op == 04 = [d_ip_op, pad "out", show p1]:parseProgram (ip+1 + 1) mem
  | op == 05 = [d_ip_op, pad "jt", show p1, show p2]:parseProgram (ip+1 + 2) mem
  | op == 06 = [d_ip_op, pad "jf", show p1, show p2]:parseProgram (ip+1 + 2) mem
  | op == 07 = [d_ip_op, pad "lt", show p1, show p2, show p3]:parseProgram (ip+1 + 3) mem
  | op == 08 = [d_ip_op, pad "eq", show p1, show p2, show p3]:parseProgram (ip+1 + 3) mem
  | op == 09 = [d_ip_op, pad "base", show p1]:parseProgram (ip+1 + 1) mem
  | op == 99 = [d_ip_op, pad "halt"]:parseProgram (ip+1 + 2) mem

  where
    (op, _, _, _) = getModes $ mem !!! ip

    d_ip_op = printf "[%03d]" ip ++ " " ++ printf "%02d" op

    pad s = printf "%-5s" s

    p1 = mem !!! (ip + 1)
    p2 = mem !!! (ip + 2)
    p3 = mem !!! (ip + 3)


runProgram :: Int -> RelBase -> Input -> Output -> [String] -> Memory -> (Output, [String])
runProgram ip rb inp outp ri mem
  | op == 01 = op_add
  | op == 02 = op_multiply
  | op == 03 = op_input
  | op == 04 = op_output
  | op == 05 = op_jump_if_true
  | op == 06 = op_jump_if_false
  | op == 07 = op_less_than
  | op == 08 = op_equals
  | op == 09 = op_rel_base_offs
  | op == 99 = op_halt

  where
    op_input         = runProgram (ip+1 + 1) rb         (tail inp)        outp  (d_input:ri)     (update_ p1 (head inp) mem)
    op_output        = runProgram (ip+1 + 1) rb               inp  (pv1 : outp) (d_output:ri)                           mem 
    jump nip         = runProgram nip        rb               inp         outp  (d_jump nip:ri)                         mem 
    store val        = runProgram (ip+1 + 3) rb               inp         outp  (d_store val:ri) (update_ p3 val        mem)

    op_rel_base_offs = runProgram (ip+1 + 1) (rb + pv1)       inp         outp  (d_base:ri)                             mem 

    d_input =     d_ip_op ++ " (i): "   ++ show p1 ++ "<-" ++ show (head inp)
    d_output =    d_ip_op ++ " (o): "   ++ show (p1,p1m) ++ " => " ++ show pv1
    d_jump nip =  d_ip_op ++ " (j): "   ++ show nip
    d_store val = d_ip_op ++ " (s): "   ++ show p3 ++ "<-" ++ show val
    d_base =      d_ip_op ++ " (b): <-" ++ show (rb + pv1) ++ " =" ++ show rb ++ "+" ++ show pv1
    d_halt =      d_ip_op ++ " (h)"

    d_ip_op = printf "[%03d]" ip ++ " " ++ printf "%02d" op

    op_add           = store (pv1 + pv2)
    op_multiply      = store (pv1 * pv2)
    op_less_than     = store (if pv1 <  pv2 then 1 else 0)
    op_equals        = store (if pv1 == pv2 then 1 else 0)
    op_jump_if_true  = jump  (if pv1 /= 0 then pv2 else ip+1 + 2)
    op_jump_if_false = jump  (if pv1 == 0 then pv2 else ip+1 + 2)

    -- op_halt          = error d_ip_op
    op_halt          = (reverse outp, reverse (d_halt:ri)) --(outp, length (Map.elems mem), Map.elems mem)-- outp

    (op, p1m, p2m, p3m) = getModes $ mem !!! ip

    p1 = mem !!! (ip + 1)
    p2 = mem !!! (ip + 2)
    p3 = mem !!! (ip + 3)

    pv1 = gv p1m p1
    pv2 = gv p2m p2
    gv pm p =
      case pm of 0 -> mem !!! p
                 1 -> p
                 2 -> mem !!! (rb + p)

(!!!) m k = res
  where Just res = Map.lookup k m
  -- where res = Map.findWithDefault 0 k m

getModes opx =
  ( opx `mod` 100,
    opx `div` 100 `mod` 10,
    opx `div` 1000 `mod` 10,
    opx `div` 10000 `mod` 10
  )

update n x a = take n a ++ [x] ++ drop (n + 1) a

update_ = Map.insert

parse = map toInt . splitBy ',' . trim

parse_ :: [Char] -> Map.Map Int Int
parse_ = Map.fromList . indexed . parse

indexed = go 0
  where
    go i (a:as) = (i, a) : go (i + 1) as
    go _ _      = []

toInt x = read x :: Int

splitBy _ [] = []
splitBy c s =
  let i = (length . takeWhile (/= c)) s
      (as, bs) = splitAt i s
   in as : splitBy c (if null bs then [] else tail bs)

trim = List.dropWhileEnd Char.isSpace . dropWhile Char.isSpace
