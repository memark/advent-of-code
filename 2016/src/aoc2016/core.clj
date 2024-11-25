(ns aoc2016.core
  (:gen-class)
  (:require
   [clojure.string :as str]))

(defn -main [])

(defn debug [x] (println x) x)

(defn int-parse [x] (Integer/parseInt x))

(def fileName "day-03/example.txt")
(def fileName "day-03/input.txt")

(def lines (-> fileName
               slurp
               str/split-lines))

(defn isTriangle [v] (> (+ (nth v 0) (nth v 1)) (nth v 2)))

(def result (->> lines
                 (map #(str/split % #" "))
                 (map #(filter (complement str/blank?) %))
                 (map #(map int-parse %))
                 (map sort)
                 (filter isTriangle)
                 count))
(prn result)
