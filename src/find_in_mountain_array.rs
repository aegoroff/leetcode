pub struct Solution {}

impl Solution {
    pub fn find_in_mountain_array(target: i32, mountain_arr: &MountainArray) -> i32 {
        let mut low = 0usize;
        let mut high = mountain_arr.length() as usize - 1;
        while high - low > 1 {
            let mid = low + ((high - low) / 2);
            let v = mountain_arr.get(mid as i32);
            if v < mountain_arr.get(mid as i32 + 1) && v > mountain_arr.get(mid as i32 - 1) {
                low = mid;
            } else {
                high = mid;
            }
        }
        let mount = high;

        if mountain_arr.get(mount as i32) == target {
            return mount as i32;
        }

        low = 0usize;
        high = mount;
        while high - low > 1 {
            let mid = low + ((high - low) / 2);
            let v = mountain_arr.get(mid as i32);
            if target < v {
                high = mid;
            } else if v == target {
                return mid as i32;
            } else {
                low = mid;
            }
        }

        if mountain_arr.get(low as i32) == target {
            return low as i32;
        }

        if mountain_arr.get(high as i32) == target {
            return high as i32;
        }

        low = mount;
        high = mountain_arr.length() as usize - 1;
        while high - low > 1 {
            let mid = low + ((high - low) / 2);
            let v = mountain_arr.get(mid as i32);
            if target < v {
                low = mid;
            } else if v == target {
                return mid as i32;
            } else {
                high = mid;
            }
        }

        if mountain_arr.get(low as i32) == target {
            return low as i32;
        }

        if mountain_arr.get(high as i32) == target {
            return high as i32;
        }

        -1
    }
}

pub struct MountainArray {
    nums: Vec<i32>,
}

impl MountainArray {
    pub fn new(nums: Vec<i32>) -> Self {
        MountainArray { nums }
    }

    pub fn get(&self, index: i32) -> i32 {
        self.nums[index as usize]
    }
    pub fn length(&self) -> i32 {
        self.nums.len() as i32
    }
}

#[cfg(test)]
mod test {
    use crate::find_in_mountain_array::{MountainArray, Solution};

    #[test]
    fn test1() {
        let arr = vec![1, 2, 3, 4, 5, 3, 1];
        let arr = MountainArray::new(arr);
        assert_eq!(Solution::find_in_mountain_array(3, &arr), 2);
    }

    #[test]
    fn test2() {
        let arr = vec![0, 1, 2, 4, 2, 1];
        let arr = MountainArray::new(arr);
        assert_eq!(Solution::find_in_mountain_array(3, &arr), -1);
    }

    #[test]
    fn test3() {
        let arr = vec![1, 5, 2];
        let arr = MountainArray::new(arr);
        assert_eq!(Solution::find_in_mountain_array(1, &arr), 0);
    }

    #[test]
    fn test4() {
        let arr = vec![1, 5, 2];
        let arr = MountainArray::new(arr);
        assert_eq!(Solution::find_in_mountain_array(5, &arr), 1);
    }

    #[test]
    fn test5() {
        let arr = vec![1, 5, 2];
        let arr = MountainArray::new(arr);
        assert_eq!(Solution::find_in_mountain_array(2, &arr), 2);
    }

    #[test]
    fn test6() {
        let arr = vec![0, 5, 3, 1];
        let arr = MountainArray::new(arr);
        assert_eq!(Solution::find_in_mountain_array(0, &arr), 0);
    }

    #[test]
    fn test7() {
        let arr = vec![1, 2, 5, 1];
        let arr = MountainArray::new(arr);
        assert_eq!(Solution::find_in_mountain_array(1, &arr), 0);
    }

    #[test]
    fn test8() {
        let arr = vec![1, 2, 5, 1];
        let arr = MountainArray::new(arr);
        assert_eq!(Solution::find_in_mountain_array(2, &arr), 1);
    }

    #[test]
    fn test9() {
        let arr = vec![1, 2, 3, 5, 3];
        let arr = MountainArray::new(arr);
        assert_eq!(Solution::find_in_mountain_array(1, &arr), 0);
    }
}
