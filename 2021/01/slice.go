// Functions that operate on slices
//
// Usage Notes
//
//  • arity - all functions have a arity of two wherein the function is prepared with all arguments in the first call and the input data is applied in the second call
package main

import (
	"sort"
)

// Map applies an iterator function to each element in a slice and returns a slice with the new elements
//
// Parameters
//
//  • iterator: function that's applied to each element
//  • data:     slice of elements
//
// Return Value
//
//  • result:   slice of elements with the iterator applied
//
func Map[TValue any, TResult any](iterator func(TValue) TResult) func([]TValue) []TResult {
	return func(data []TValue) []TResult {
		out := make([]TResult, len(data))
		for i, v := range data {
			out[i] = iterator(v)
		}
		return out
	}
}

// MapIndexed is the same as Map but also makes the index and input elements available to the iterator function
//
// Parameters
//
//  • iterator: function that's applied to each element
//  • data: list of elements
//
// Return Value
//
//  • result: list of elements with the iterator applied
//
func MapIndexed[TValue any, TResult any](iterator func(TValue, int, []TValue) TResult) func([]TValue) []TResult {
	return func(data []TValue) []TResult {
		out := make([]TResult, len(data))
		for i, v := range data {
			out[i] = iterator(v, i, data)
		}
		return out
	}
}

// Reduce iterates over a slice and calls the iterator function on each element while passing the accumulator to the next call
//
// Parameters
//
//  • iterator: function that's applied to each element
//  • initialAcc: initial value of the accumulator
//  • data: list of elements
//
// Return Value
//
//  • result: the list of elements reduced to a single data type
//
func Reduce[TValue any, TAcc any](iterator func(TAcc, TValue, int) TAcc, initialAcc TAcc) func([]TValue) TAcc {
	return func(data []TValue) TAcc {
		out := initialAcc
		for i, v := range data {
			out = iterator(out, v, i)
		}
		return out
	}
}

// Filter calls the predicate function on each element and removes those elements that do not satisfy the predicate
//
// Parameters
//
//  • predicate: function that is called against each element
//  • data: list of elements
//
// Return Value
//
//  • result: list of elements where the predicate is not satisfied
//
func Filter[TValue any](predicate func(TValue) bool) func([]TValue) []TValue {
	return func(data []TValue) []TValue {
		out := make([]TValue, 0)
		for _, v := range data {
			if predicate(v) {
				out = append(out, v)
			}
		}
		return out
	}
}

// Reject calls the predicate function on each element and removes those elements that do satisfy the predicate
//
// Parameters
//
//  • predicate: function that is called against each element
//  • data: list of elements
//
// Return Value
//
//  • result: list of elements where the predicate is satisfied
//
func Reject[TValue any](predicate func(TValue) bool) func([]TValue) []TValue {
	return func(data []TValue) []TValue {
		out := make([]TValue, 0)
		for _, v := range data {
			if !predicate(v) {
				out = append(out, v)
			}
		}
		return out
	}
}

// GroupBy creates a map where the key is a group identifier and the value is a slice with the elements that have the same identifer
//
// Parameters
//
//  • grouper: function receives each element and returns a string identifier for the element
//  • data: list of elements
//
// Return Value
//
//  • result: map of elements grouped by identifiers
//
func GroupBy[TValue any](grouper func(TValue) string) func([]TValue) map[string][]TValue {
	return func(data []TValue) map[string][]TValue{
		var id string
		out := make(map[string][]TValue)
		for _, v := range data {
			id = grouper(v)
			out[id] = append(out[id], v)
		}
		return out
	}
}

// Concat combines two slices of the same type together
//
// Parameters
//
//  • slice: list of elements to concatenate
//  • data: list of elements
//
// Return Value
//
//  • result: map of elements grouped by identifiers
//
func Concat[TValue any](slice []TValue) func([]TValue) []TValue {
	return func(data []TValue) []TValue{
		return append(data, slice...)
	}
}

// UniqBy returns a slice of only unique values based on a string identifier
//
// Parameters
//
//  • identify: function that returns a string representation to uniquely identify the element
//  • data: list of elements
//
// Return Value
//
//  • result: list of unique elements
//
func UniqBy[TValue any](identify func(TValue) string) func([]TValue) []TValue {
	return func(data []TValue) []TValue{
		var id string
		out := make([]TValue, 0)
		identifiers := make(map[string]bool)
		for _, v := range data {
			id = identify(v)
			if _, ok := identifiers[id]; !ok {
				identifiers[id] = true
				out = append(out, v)
			}
		}
		return out
	}
}

// Drop removes the first n elements from the slice and returns the remaining slice
//
// Parameters
//
//  • count: number of elements to remove
//  • data: list of elements
//
// Return Value
//
//  • result: slice of remaining elements
//
func Drop[TValue any](count int) func([]TValue) []TValue {
	return func(data []TValue) []TValue{

		if count <=0 {
			return data
		}

		if count >= len(data) {
			return make([]TValue, 0)
		}

		return data[count:]
	}
}

// DropLast removes the last n elements from the slice and returns the remaining slice
//
// Parameters
//
//  • count: number of elements to remove
//  • data: list of elements
//
// Return Value
//
//  • result: slice of remaining elements
//
func DropLast[TValue any](count int) func([]TValue) []TValue {
	return func(data []TValue) []TValue{

		if count <=0 {
			return data
		}

		data_length := len(data)

		if count >= data_length {
			return make([]TValue, 0)
		}

		return data[:(data_length-count)]
	}
}

// Take returns the first n elements of the slice
//
// Parameters
//
//  • count: number of elements to keep
//  • data: list of elements
//
// Return Value
//
//  • result: slice of remaining elements
//
func Take[TValue any](count int) func([]TValue) []TValue {
	return func(data []TValue) []TValue{

		if count >= len(data) || count <=0 {
			return data
		}

		return data[:count]
	}
}

// TakeLast returns the last n elements of the slice
//
// Parameters
//
//  • count: number of elements to keep
//  • data: list of elements
//
// Return Value
//
//  • result: slice of remaining elements
//
func TakeLast[TValue any](count int) func([]TValue) []TValue {
	return func(data []TValue) []TValue{

		data_length := len(data)

		if count >= data_length || count <=0 {
			return data
		}

		return data[(data_length-count):]
	}
}

// Flatten creates a new slice where one level of nested elements are unnested
//
// Parameters
//
//  • data: list of elements
//
// Return Value
//
//  • result: unnested list slice of elements
//
func Flatten[TValue any](data [][]TValue) []TValue {

	out := make([]TValue, 0)

	for _, v := range data {
		out = append(out, v...)
	}

	return out
}

// SortBy applies a comparator against each element to sort the slice
//
// Parameters
//
//  • comparator: function used to compare two elements in a slice
//  • data: list of elements
//
// Return Value
//
//  • result: sorted slice of elements
//
func SortBy[TValue any](comparator func(TValue, TValue) bool) func([]TValue) []TValue {

	return func(data []TValue) []TValue {

		out := data

		sort.SliceStable(out, func(prevIndex int, nextIndex int) bool {
			return comparator(data[prevIndex], data[nextIndex])
		})
		
		return out
	}
}

// Append returns the slice with the additional element added to the end
//
// Parameters
//
//  • element: element to add
//  • data: list of elements
//
// Return Value
//
//  • result: slice of elements with the additional element
//
func Append[TValue any](element TValue) func([]TValue) []TValue {

	return func(data []TValue) []TValue {

		out := append(data, element)
		
		return out
	}
}

// Prepend returns the slice with the additional element added to the beggining
//
// Parameters
//
//  • element: element to add
//  • data: list of elements
//
// Return Value
//
//  • result: slice of elements with the additional element
//
func Prepend[TValue any](element TValue) func([]TValue) []TValue {

	return func(data []TValue) []TValue {

		out := append([]TValue{element}, data...)
		
		return out
	}
}

// Partition splits elements into two groups - one where the predicate is satisfied and one where the predicate is not
//
// Parameters
//
//  • predicate: function that is called against each element returning true or false
//  • data: list of elements
//
// Return Value
//
//  • result: slice with two nested slices where the first are all elements satisfying the predicate and the second where all elements do not satisfy the predicate 
//
func Partition[TValue any](predicate func(TValue) bool) func([]TValue) [][]TValue {
	return func(data []TValue) [][]TValue {
		outTrue := make([]TValue, 0)
		outFalse := make([]TValue, 0)
		for _, v := range data {
			if predicate(v) {
				outTrue = append(outTrue, v)
			} else {
				outFalse = append(outFalse, v)
			}
		}
		return [][]TValue{outTrue, outFalse}
	}
}

// Tail returns the input slice with all elements except the first element
//
// Parameters
//
//  • data: list of elements
//
// Return Value
//
//  • result: slice without the first element 
//
func Tail[TValue any](data []TValue) []TValue {

	if len(data) <= 1 {
		return []TValue{}
	}

	return data[1:]
}

// Head returns first element of a slice
//
// Parameters
//
//  • data: list of elements
//
// Return Value
//
//  • result: first element in a slice
//
func Head[TValue any](data []TValue) TValue {

	var out TValue

	if len(data) <= 0 {
		return out
	}

	out = data[:1][0]

	return out
}

// SplitEvery returns elements in equal length slices
//
// Parameters
//
//  • size: number of elements per chunk
//  • data: list of elements
//
// Return Value
//
//  • result: slice with sub-slices of specified length
func SplitEvery[TValue any](size int) func([]TValue) [][]TValue {
	return func(data []TValue) [][]TValue {

		if size <= 0 || len(data) <= 1 {
			return [][]TValue{data}
		}

		out := make([][]TValue, 0)
		currentGroup := make([]TValue, 0)
		for i, v := range data {
			if len(currentGroup) < size {
				currentGroup = append(currentGroup, v)
			} else {
				out = append(out, currentGroup)
				currentGroup = []TValue{v}
			}

			if (i + 1) >= len(data) {
				out = append(out, currentGroup)
			}
		}

		return out
	}
}