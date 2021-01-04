require "set"

sample_1 = "
.#.
..#
###
"

d = sample_1.split.map { |x| x.split("") }

actives = Set.new

for y in 0...d.length
  for x in 0...d[y].length
    if d[y][x] == "#"
      actives << [x, y, 0, 0]
    end
  end
end

puts actives

def recurse(actives, maxCycles, cycle)
  if cycle > maxCycles
    return actives
  end

  coordsToCheck = Set.new
  for a in actives
    for aa in [a, *getNeighbours(a)]
      coordsToCheck << aa
    end
  end
  puts "cycle=#{cycle}: checking #{coordsToCheck.length} relevant coords"

  newActives = Set.new
  for c in coordsToCheck
    if getNewState(c, actives)
      newActives << c
    end
  end
  puts "cycle=#{cycle}: going from #{actives.length} to #{newActives.length} active cubes"

  return recurse(newActives, maxCycles, cycle + 1)
end

def getNewState(coord, actives)
  activeNeighbours = 0
  for n in getNeighbours(coord)
    if actives.include?(n)
      activeNeighbours += 1
    end
  end

  if actives.include?(coord)
    return [2, 3].include?(activeNeighbours)
  else
    return [3].include?(activeNeighbours)
  end
end

def getNeighbours(coord)
  res = Set.new
  for w in ((coord[3] - 1)..(coord[3] + 1))
    for z in ((coord[2] - 1)..(coord[2] + 1))
      for y in ((coord[1] - 1)..(coord[1] + 1))
        for x in ((coord[0] - 1)..(coord[0] + 1))
          res << [x, y, z, w]
        end
      end
    end
  end

  res.delete(coord)
  return res
end

numCycles = 6

res = recurse(actives, numCycles, 1)

puts res.length
