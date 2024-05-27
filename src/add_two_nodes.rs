pub struct Solution {}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut x = l1;
        let mut y = l2;

        let mut stack: Vec<_> = vec![];

        let mut carry = 0;
        while x.is_some() || y.is_some() {
            let xval = if let Some(x) = x.clone() { x.val } else { 0 };
            let yval = if let Some(y) = y.clone() { y.val } else { 0 };
            let z = xval + yval + carry;
            if z > 9 {
                let rest = z % 10;
                carry = 1;
                stack.push(rest);
            } else {
                carry = 0;
                stack.push(z);
            }
            if let Some(xv) = x {
                x = xv.next;
            }
            if let Some(yv) = y {
                y = yv.next;
            }
        }
        if carry > 0 {
            stack.push(carry);
        }

        let mut answer: Option<Box<ListNode>> = None;

        while let Some(x) = stack.pop() {
            answer = Some(Box::new(ListNode {
                val: x,
                next: answer,
            }));
        }

        answer
    }
}

#[cfg(test)]
mod test {
    use crate::add_two_nodes::{ListNode, Solution};

    #[test]
    fn test1() {
        let l1 = ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 4,
                next: Some(Box::new(ListNode::new(3))),
            })),
        };

        let l2 = ListNode {
            val: 5,
            next: Some(Box::new(ListNode {
                val: 6,
                next: Some(Box::new(ListNode::new(4))),
            })),
        };

        assert_eq!(
            Solution::add_two_numbers(Some(Box::new(l1)), Some(Box::new(l2))),
            Some(Box::new(ListNode {
                val: 7,
                next: Some(Box::new(ListNode {
                    val: 0,
                    next: Some(Box::new(ListNode::new(8))),
                })),
            }))
        );
    }
}
