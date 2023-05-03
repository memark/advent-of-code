import qualified Data.Char as Char
import qualified Data.List as List

sample_1a = "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0"
sample_1b = "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0"
sample_1c = "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0"

sample_2a = "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5"
sample_2b = "3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10"

main = do
  input <- readFile "input.txt"

  -- Part One -- 116680
  -- print . maximum . map (eval (parse input) 0) . List.permutations $ [0..4]

  -- Part Two -- 
  print . map (eval' (parse sample_2a) 0) $ [[9,8,7,6,5]] -- . List.permutations $ [0..4]
  print . map (eval' (parse sample_2b) 0) $ [[9,7,8,5,6]] -- . List.permutations $ [0..4]


  putStr ""


eval :: Memory -> Input -> Phases -> Output
eval parsedData initialInput phaseSettings = outp5
  where outp5 = runProgram 0 inp5 parsedData
        inp5 = [phaseSettings !! 4,outp4]
        outp4 = runProgram 0 inp4 parsedData
        inp4 = [phaseSettings !! 3,outp3]
        outp3 = runProgram 0 inp3 parsedData
        inp3 = [phaseSettings !! 2,outp2]
        outp2 = runProgram 0 inp2 parsedData
        inp2 = [phaseSettings !! 1,outp1]
        outp1 = runProgram 0 inp1 parsedData
        inp1 = (phaseSettings !! 0):[initialInput]


eval' :: Memory -> Input -> Phases -> Outputs
eval' mem ii ps = fst $ rec (mem,mem,mem,mem,mem) ii (ps!!0,ps!!1,ps!!2,ps!!3,ps!!4) 50

type M = Memory
type I = Input
type O = Output
type R = Result
type P = Phase

rec :: (M,M,M,M,M) -> I -> (P,P,P,P,P) -> Int -> ([O], (M,M,M,M,M))
rec (m1,m2,m3,m4,m5) _ _ 0 = ([], (m1,m2,m3,m4,m5))
rec (m1,m2,m3,m4,m5) ii (ps1,ps2,ps3,ps4,ps5) i = (outp5 : fst r2, snd r2)
  where
    r2 = rec (nm1,nm2,nm3,nm4,nm5) outp5 (ps1,ps2,ps3,ps4,ps5) (i-1)

    (OutputResult _ outp5 nm5) = runProgram' 0 inp5 m5
    inp5 = [ps5, outp4]
    (OutputResult _ outp4 nm4) = runProgram' 0 inp4 m4
    inp4 = [ps4, outp3]
    (OutputResult _ outp3 nm3) = runProgram' 0 inp3 m3
    inp3 = [ps3, outp2]
    (OutputResult _ outp2 nm2) = runProgram' 0 inp2 m2
    inp2 = [ps2, outp1]
    (OutputResult _ outp1 nm1) = runProgram' 0 inp1 m1
    inp1 = ps1:[ii]

type Phase = Int
type Phases = [Phase]
type Input = Int
type Inputs = [Input]
type Output = Int
type Outputs = [Output]
type Memory = [Int]
type Pointer = Int

data Result = OutputResult Pointer Output Memory | HaltedResult Memory


runProgram :: Pointer -> Inputs -> Memory -> Output
runProgram ip inp mem = o
  where (OutputResult _ o _) = runProgram' ip inp mem


runProgram' :: Pointer -> Inputs -> Memory -> Result
runProgram' ip inp mem
  | op == 01 = op_add
  | op == 02 = op_multiply
  | op == 03 = op_input
  | op == 04 = op_output
  | op == 05 = op_jump_if_true
  | op == 06 = op_jump_if_false
  | op == 07 = op_less_than
  | op == 08 = op_equals
  | op == 99 = op_halt

  where
    jump nip         = runProgram' nip              inp                        mem
    store val        = runProgram' (ip+1 + 3)       inp  (update p3 val        mem)
    op_input         = runProgram' (ip+1 + 1) (tail inp) (update p1 (head inp) mem)
    
    op_add           = store (pv1 + pv2)
    op_multiply      = store (pv1 * pv2)
    op_less_than     = store (if pv1 <  pv2 then 1   else 0)
    op_equals        = store (if pv1 == pv2 then 1   else 0)
    op_jump_if_true  = jump  (if pv1 /= 0   then pv2 else ip+1 + 2)
    op_jump_if_false = jump  (if pv1 == 0   then pv2 else ip+1 + 2)
    
    op_output        = OutputResult (ip+1 + 1) pv1 mem
    op_halt          = HaltedResult mem

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
