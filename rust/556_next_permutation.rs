impl Solution {
    pub fn next_greater_element(n: i32) -> i32 {
        let mut num: Vec<u8> = n.to_string().bytes().collect();
        let mut ptr = num.len() - 1;
        // find the first break point that breaks the descending order
        while ptr > 0 && num[ptr - 1] >= num[ptr] {
            ptr -= 1;
        }
        // sequence with descending order has no next permutation
        if ptr == 0 {
            return -1;
        }
        // find the first element that is greater than num[ptr - 1]
        let mut j = num.len() - 1;
        while num[j] <= num[ptr - 1] {
            j -= 1;
        }
        // swap them, since num[j] is the first element that is greater than num[ptr - 1]
        // the value in [ptr..j] is greater num[ptr - 1], and the value in [j + 1..] is smaller than num[ptr - 1]
        // thus swapping j and ptr - 1 will holds the order
        num.swap(j, ptr - 1);
        // then reverse the descending seqence. because the basic rule of constructing next permutation is
        // to advance the highest bit, then zero-out the lower bits (which is the ascending order)
        num[ptr..].reverse();

        // re-construct the number
        let ans = num.iter().fold(0, |acc, &x| acc * 10 + ((x - b'0') as i64));
        if ans > (i32::MAX as i64) {
            -1
        } else {
            ans as i32
        }
    }
}
