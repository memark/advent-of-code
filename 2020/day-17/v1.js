var sample_1 = "\n.#.\n..#\n###\n";
var data = sample_1;
var d = data.split("\n").map(function (s) { return s.split(""); });
function range(from, to) {
    return Array.from(Array(Math.floor(to - from))).map(function (v, k) { return from + k; });
}
var actives = new Set();
for (var _i = 0, _a = range(0, d.length); _i < _a.length; _i++) {
    var y = _a[_i];
    for (var _b = 0, _c = range(0, d[y].length); _b < _c.length; _b++) {
        var x = _c[_b];
        if (d[y][x] == "#") {
            actives.add({ x: x, y: y, z: 0, w: 0 });
        }
    }
}
console.log(actives);
var numCycles = 6;
var res = recurse(actives, numCycles);
console.log(res.size);
function recurse(actives, maxCycles, cycle) {
    if (cycle === void 0) { cycle = 1; }
    if (cycle > maxCycles)
        return actives;
    // coordsToCheck = {aa for a in actives for aa in [a, *getNeighbours(a)]}
    // const coordsToCheck =
    //     newActives = {c for c in coordsToCheck if getNewState(c, actives)}
    //     return recurse(newActives, maxCycles, cycle+1)
}
function getNewState(coord, actives) {
    var activeNeighbours = 0;
    for (var _i = 0, _a = getNeighbours(coord); _i < _a.length; _i++) {
        var n = _a[_i];
        if (actives.has(n)) {
            activeNeighbours++;
        }
    }
    // If a cube is active and exactly 2 or 3 of its neighbors are also active, the cube remains active.
    // Otherwise, the cube becomes inactive.
    if (actives.has(coord))
        return [2, 3].includes(activeNeighbours);
    // If a cube is inactive but exactly 3 of its neighbors are active, the cube becomes active.
    // Otherwise, the cube remains inactive.
    else
        return [3].includes(activeNeighbours);
}
function getNeighbours(coord) {
    var res = new Set();
    for (var _i = 0, _a = range(coord.w - 1, coord.w + 1 + 1); _i < _a.length; _i++) {
        var w = _a[_i];
        for (var _b = 0, _c = range(coord.z - 1, coord.z + 1 + 1); _b < _c.length; _b++) {
            var z = _c[_b];
            for (var _d = 0, _e = range(coord.y - 1, coord.y + 1 + 1); _d < _e.length; _d++) {
                var y = _e[_d];
                for (var _f = 0, _g = range(coord.x - 1, coord.x + 1 + 1); _f < _g.length; _f++) {
                    var x = _g[_f];
                    res.add({ x: x, y: y, z: z, w: w });
                }
            }
        }
    }
    res["delete"](coord);
    return res;
}
