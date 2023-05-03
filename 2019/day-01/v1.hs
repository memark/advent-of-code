main = do
    text <- readFile "input.txt"
    let masses = map toInt . lines $ text

    -- Part One -- 3330521
    print (sum . map fuel $ masses)

    -- Part Two -- 4992931
    print (sum . map totalFuel $ masses)
    
toInt x = read x :: Int

fuel m = m `div` 3 - 2

totalFuel = sum . takeWhile (>0) . tail . iterate fuel
