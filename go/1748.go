func sumOfUnique(nums []int) int {
    mp := make(map[int]int)
    ans := 0
    for _, x := range nums {
        mp[x]++
        if mp[x] == 1 {
            ans += x
        } else if mp[x] == 2 {
            ans -= x
        }
    }
    return ans
}
