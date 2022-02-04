func reversePrefix(word string, ch byte) string {
    j := 0
    for idx, c := range word {
        if byte(c) == ch {
            j = idx
            break
        }
    }
    if j != 0 {
        r := []rune(word)
        for i := 0; i < j; i, j = i + 1, j - 1 {
            r[i], r[j] = r[j], r[i]
        }
        return string(r)
    }
    return word
}
