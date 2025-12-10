import "sort"

func eraseOverlapIntervals(intervals [][]int) int {

    if len(intervals) == 0 {
        return 0
    }

    // sort by end time
    sort.Slice(intervals, func(i, j int) bool {
        return intervals[i][1] < intervals[j][1]
    })

    count := 0
    lastEnd := intervals[0][1]

    for i := 1; i < len(intervals); i++ {

        if intervals[i][0] < lastEnd {
            // overlap → remove
            count++
        } else {
            // no overlap → update end
            lastEnd = intervals[i][1]
        }
    }

    return count
}
