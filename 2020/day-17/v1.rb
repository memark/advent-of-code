# @names.each do |name|
#   puts "Hello #{name}!"
# end

# arr.take(3)
# arr.drop(3)

# browsers = %w[Chrome Firefox Safari Opera IE]
# browsers.length
# browsers.count

# browsers.empty?

# browsers.include?('Konqueror')

# arr.map { |a| 2 * a }

# arr.each { |a| print a -= 10, ' ' }

# arr.select { |a| a > 3 }
# arr.reject { |a| a < 3 }
# arr.drop_while { |a| a < 4 }

require "set"

sample_1 = "
.#.
..#
###
"

# puts sample_1
# puts
# puts sample_1.split
# puts
# puts sample_1.split.map { |x| x.split("") }
# puts

d = sample_1.split.map { |x| x.split("") }
# puts d

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
  #   console.log("actives", actives);
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
    # if (actives.has(n)) {
    if actives.include?(n)
      #   if (coord.z == 0 && coord.w == 0) console.log(coord, activeNeighbours)
      activeNeighbours += 1
    end
  end

  #   if (coord.z == 0 && coord.w == 0) console.log(coord, activeNeighbours)

  # If a cube is active and exactly 2 or 3 of its neighbors are also active, the cube remains active.
  # Otherwise, the cube becomes inactive.
  #   if (actives.has(coord)) return [2, 3].includes(activeNeighbours)
  if actives.include?(coord)
    return [2, 3].include?(activeNeighbours)
    # If a cube is inactive but exactly 3 of its neighbors are active, the cube becomes active.
    # Otherwise, the cube remains inactive.
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

# puts res.length
