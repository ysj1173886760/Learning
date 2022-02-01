func longestNiceSubstring(s string) string {
    if len(s) < 2 {
        return ""
    }
    st := make(map[rune]bool)
    for _, ch := range s {
        st[ch] = true
    }
    for idx, ch := range s {
        if st[unicode.ToUpper(ch)] && st[unicode.ToLower(ch)] {
            continue
        }
        s1 := longestNiceSubstring(s[:idx])
        s2 := longestNiceSubstring(s[(idx + 1):])
        if len(s1) >= len(s2) {
            return s1
        } else {
            return s2
        }
    }
    return s
}
