impl Solution {
    pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy_head = Box::new(ListNode::new(0));
        let mut head = &mut dummy_head;
        let (mut l1, mut l2) = (list1, list2);

        while let (Some(lhs), Some(rhs)) = (l1.as_ref(), l2.as_ref()) {
            if lhs.val <= rhs.val {
                head.next = l1;
                head = head.next.as_mut().unwrap();
                l1 = head.next.take();
            } else {
                head.next = l2;
                head = head.next.as_mut().unwrap();
                l2 = head.next.take();
            }
        }

        head.next = if l1.is_some() { l1 } else { l2 };
        dummy_head.next
    }
}
