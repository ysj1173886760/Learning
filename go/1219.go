func dfs(x, y int, grid [][]int) int {
    n := len(grid)
    m := len(grid[0])
    maxx := 0
    dir := [][]int{{0, 1}, {0, -1}, {1, 0}, {-1, 0}}
    tmp := grid[x][y]
    grid[x][y] = 0
    for _, d := range dir {
        nx := x + d[0]
        ny := y + d[1]
        if nx < 0 || ny < 0 || nx >= n || ny >= m || grid[nx][ny] == 0 {
            continue
        }
        res := dfs(nx, ny, grid)
        if res > maxx {
            maxx = res
        }
    }
    grid[x][y] = tmp
    return maxx + tmp
}

func getMaximumGold(grid [][]int) int {
    n := len(grid)
    m := len(grid[0])
    ans := 0
    for i := 0; i < n; i++ {
        for j := 0; j < m; j++ {
            if grid[i][j] != 0 {
                res := dfs(i, j, grid)
                if res > ans {
                    ans = res
                }
            }
        }
    }
    return ans
}
