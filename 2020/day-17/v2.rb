require "set"

sample_1 = "
.#.
..#
###
"

d = sample_1.split.map { |x| x.split("") }

actives = (0...d.length)
  .flat_map { |y|
  (0...d[y].length)
    .filter { |x| d[y][x] == "#" }
    .map { |x| [x, y, 0, 0] }
}.to_set

def recurse(actives, maxCycles, cycle)
  if cycle > maxCycles
    return actives
  end

  coordsToCheck = actives.flat_map { |a| [a, *getNeighbours(a)] }.to_set
  puts "cycle=#{cycle}: checking #{coordsToCheck.length} relevant coords"

  newActives = coordsToCheck.filter { |c| getNewState(c, actives) }.to_set
  puts "cycle=#{cycle}: going from #{actives.length} to #{newActives.length} active cubes"

  puts
  return recurse(newActives, maxCycles, cycle + 1)
end

def getNewState(coord, actives)
  activeNeighbours = getNeighbours(coord).filter { |n| actives.include?(n) }.count

  if actives.include?(coord)
    return [2, 3].include?(activeNeighbours)
  else
    return [3].include?(activeNeighbours)
  end
end

def getNeighbours(coord)
  res = ((coord[3] - 1)..(coord[3] + 1)).flat_map do |w|
    ((coord[2] - 1)..(coord[2] + 1)).flat_map do |z|
      ((coord[1] - 1)..(coord[1] + 1)).flat_map do |y|
        ((coord[0] - 1)..(coord[0] + 1)).map do |x|
          [x, y, z, w]
        end
      end
    end
  end

  res.delete(coord)
  return res.to_set
end

numCycles = 6

res = recurse(actives, numCycles, 1).length
puts "should be 848" unless res == 848
