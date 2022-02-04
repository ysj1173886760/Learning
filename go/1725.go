func countGoodRectangles(rectangles [][]int) int {
    ans := 0
    cnt := 0
    for _, rect := range rectangles {
        l := 0
        if rect[0] < rect[1] {
            l = rect[0]
        } else {
            l = rect[1]
        }
        switch {
            case l == ans:
                cnt += 1
            case l > ans:
                ans = l
                cnt = 1
            default:
                //
        }
    }
    return cnt
}
