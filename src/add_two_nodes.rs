
pub struct Solution {}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut n1 = 0;
        let mut x = l1;
        let mut pow = 0;
        while let Some(x1) = x.clone() {
            n1 += x1.val * 10_i32.pow(pow);
            x = x1.next;
            pow += 1;
        }
        let mut n2 = 0;
        let mut y = l2;
        let mut pow = 0;
        while let Some(y1) = y.clone() {
            n2 += y1.val * 10_i32.pow(pow);
            y = y1.next;
            pow += 1;
        }
        let mut result = n1 + n2;
        let mut answer: Option<Box<ListNode>> = None;
        while result > 0 {
            let n = result % 10;
            result /= 10;

            if answer.is_none() {
                answer = Some(Box::new(ListNode::new(n)))
            } else {
                let mut tail = answer.clone().unwrap().clone().next;
                while let Some(t) = &tail {
                    answer = t.clone().next;
                }
                tail = Some(Box::new(ListNode::new(n)))
            }
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
                next: Some(Box::new(ListNode::new(3)))
            }))
        };

        let l2 = ListNode {
            val: 5,
            next: Some(Box::new(ListNode {
                val: 6,
                next: Some(Box::new(ListNode::new(4)))
            }))
        };


        assert_eq!(Solution::add_two_numbers(Some(Box::new(l1)), Some(Box::new(l2))), None);
    }

    // #[test]
    // fn test2() {
    //     assert_eq!(Solution::rob(vec![2, 7, 9, 3, 1]), 12);
    // }
}
