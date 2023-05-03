import qualified Data.Char as Char
import qualified Data.List as List
import qualified Data.Map.Strict as Map
import qualified Data.Set as Set
import qualified Control.Exception as Ex
import qualified Control.Monad
import Text.Printf ()
import Data.Tuple.Select ()


sample_1a = "\
\<x=-1, y=0, z=2>\n\
\<x=2, y=-10, z=-7>\n\
\<x=4, y=-8, z=8>\n\
\<x=3, y=5, z=-1>"

zample_1a = [
  [-1,0,2],
  [2,-10,-7],
  [4,-8,8],
  [3,5,1]]


input = "\
\<x=3, y=2, z=-6>\n\
\<x=-13, y=18, z=10>\n\
\<x=-8, y=-1, z=13>\n\
\<x=5, y=10, z=4>"

zinput = [
  [3,2,-6],
  [-13,18,10],
  [-8,-1,13],
  [5,10,4]]


type Position = [Int] -- (Int,Int,Int)
type Velocity = [Int] -- (Int,Int,Int)


main = do
  putStrLn ""
  input <- readFile "input.txt"

  let poss = zample_1a
  let vels = [[0,0,0],[0,0,0],[0,0,0],[0,0,0]]

  prettyPrint' [(0,poss,vels)]
  
  -- let ps = pairs [0..3]
  -- print ps

  -- print $ gravity 3 5

  prettyPrint' $ simulate 1 5 poss vels

  print $ gravity' 3 5
  print $ gravity'xyz [3,3,3] [5,3,1]

  -- Part One -- 

  -- Part Two -- 

  putStr ""

-- spx = sp !! 0
-- tpx = tp !! 0


-- if Ganymede has an x position of 3,
-- and Callisto has a x position of 5,
-- then Ganymede's x velocity changes by +1 (because 5 > 3)
-- and Callisto's x velocity changes by -1 (because 3 < 5).
-- However, if the positions on a given axis are the same,
-- the velocity on that axis does not change for that pair of moons.
gravity p1 p2 = vc
  where vc = case p1 `compare` p2 of EQ -> ( 0, 0)
                                     LT -> ( 1,-1)
                                     GT -> (-1, 1)

gravityAll p1s p2s = vc
  where vc = zipWith gravity p1s p2s

-- onödigt krångligt med pairs! bättre att utvärdera en i taget.


gravity' c oc = vc
  where vc = case c `compare` oc of EQ ->  0
                                    LT ->  1
                                    GT -> -1

-- är detta zipWith igen?!
gravity'xyz p op = vc
  where vc = [a 0, a 1, a 2]
        a i = gravity' (p!!i) (op!!i)

-- gravity'All ps = vcs


-- kan man använda iterate() eller scan...() för att köra denna funktion om och om igen?
-- istället för att låta den vara rekursiv. känns väl som ett vanligt pattern.
-- särskilt nu när input och output är så lika varandra (bara state som ändras).
simulate :: Int -> Int -> [Position] -> [Velocity] -> [(Int,[Position],[Velocity])]
simulate st ms poss vels =
  if st > ms then []
  else next
    where
      next = (st,newposs,newvels) : simulate (st+1) ms newposs newvels
      newposs = zipWith (zipWith (+)) poss newvels
      newvels = [[1,1,1],[1,1,1],[1,1,1],[1,1,1]]


pairs l = [(x,y) | (x:ys) <- List.tails l, y <- ys]


prettyPrint' [] = return ()
prettyPrint' (a:rest) = do
  prettyPrint a
  prettyPrint' rest

prettyPrint :: (Int,[Position],[Velocity]) -> IO ()
prettyPrint (st,ps,vs) = do
  putStrLn $ "After " ++ show st ++ " steps:"
  putStrLn $ ns 0
  putStrLn $ ns 1
  putStrLn $ ns 2
  putStrLn $ ns 3
  putStrLn ""
  where
    ns i = "pos " ++ show (ps!!i) ++ "   vel " ++ show (vs!!i)
