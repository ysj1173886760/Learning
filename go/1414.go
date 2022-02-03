func findMinFibonacciNumbers(k int) int {
    fib := []int{1, 1}
    for fib[len(fib) - 1] < k {
        fib = append(fib, fib[len(fib) - 1] + fib[len(fib) - 2])
    }
    ans := 0
    for i := len(fib) - 1; i > 0; i-- {
        if k >= fib[i] {
            k -= fib[i]
            ans += 1
        }
    }
    return ans
}
