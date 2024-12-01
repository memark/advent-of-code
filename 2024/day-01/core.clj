(require '[clojure.string :refer [split split-lines blank?]])

(def parseLines
  (partial map (comp
                #(map Integer/parseInt %)
                #(remove blank? %)
                #(split % #" "))))

(def transpose (partial apply mapv vector))

(defn getSortedTransposedRows [fileName]
  (->> (-> fileName slurp split-lines)
       parseLines
       transpose
       (map sort)))

(defn partOne [fileName]
  (let [p (getSortedTransposedRows fileName)
        a (apply map (comp abs -) p)]
    (reduce + a)))

(prn (partOne "input.txt"))
