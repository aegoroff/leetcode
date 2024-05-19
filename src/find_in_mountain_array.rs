pub struct Solution {}

impl Solution {
    pub fn find_in_mountain_array(target: i32, mountain_arr: &MountainArray) -> i32 {
        let mut low = 0usize;
        let mut high = mountain_arr.length() as usize - 1;
        let mut size = high;
        while high - low > 1 {
            let mid = low + size / 2;
            let v = mountain_arr.get(mid as i32);
            if v < mountain_arr.get(mid as i32 + 1) && v > mountain_arr.get(mid as i32 - 1) {
                low = mid;
            } else {
                high = mid;
            }
            size = high - low;
        }
        let mount = high;

        low = 0usize;
        high = mount;
        size = high;
        while high - low > 1 {
            let mid = low + size / 2;
            let v = mountain_arr.get(mid as i32);
            match target.cmp(&v) {
                std::cmp::Ordering::Less => high = mid,
                std::cmp::Ordering::Equal => return mid as i32,
                std::cmp::Ordering::Greater => low = mid,
            }
            size = high - low;
        }

        if mountain_arr.get(low as i32) == target {
            return low as i32;
        }

        if mountain_arr.get(high as i32) == target {
            return high as i32;
        }

        low = mount;
        high = mountain_arr.length() as usize - 1;
        size = high - low;
        while high - low > 1 {
            let mid = low + size / 2;
            let v = mountain_arr.get(mid as i32);
            match target.cmp(&v) {
                std::cmp::Ordering::Less => low = mid,
                std::cmp::Ordering::Equal => return mid as i32,
                std::cmp::Ordering::Greater => high = mid,
            }
            size = high - low;
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

    #[test]
    fn test10() {
        let arr = vec![
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
            25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46,
            47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68,
            69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90,
            91, 92, 93, 94, 95, 96, 97, 98, 99, 100, 101, 100, 99, 98, 97, 96, 95, 94, 93, 92, 91,
            90, 89, 88, 87, 86, 85, 84, 83, 82,
        ];
        let arr = MountainArray::new(arr);
        assert_eq!(Solution::find_in_mountain_array(102, &arr), -1);
    }
}
