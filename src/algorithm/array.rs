use std::cmp::max;
use std::mem::swap;
use std::net::Shutdown::Read;

/**
给定一个整数数组 nums 和一个整数目标值 target，请你在该数组中找出 和为目标值 target  的那 两个 整数，并返回它们的数组下标。

你可以假设每种输入只会对应一个答案，并且你不能使用两次相同的元素。

你可以按任意顺序返回答案。

示例 1：

输入：nums = [2,7,11,15], target = 9
输出：[0,1]
解释：因为 nums[0] + nums[1] == 9 ，返回 [0, 1] 。
*/
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut results = vec![];
    for i in 0..nums.len() {
        for j in (i + 1)..nums.len() {
            if nums[i] + nums[j] == target {
                return vec![i as i32, j as i32]; // 转换 usize 为 i32
            }
        }
    }
    return results;
}


/**
以数组 intervals 表示若干个区间的集合，其中单个区间为 intervals[i] = [starti, endi] 。请你合并所有重叠的区间，并返回 一个不重叠的区间数组，该数组需恰好覆盖输入中的所有区间 。

示例 1：
[[2,6],[1,3],[8,10],[15,18]]
[[1,3],[2,6],[8,10],[15,18]]
输入：intervals = [[1,3],[2,6],[8,10],[15,18]]
输出：[[1,6],[8,10],[15,18]]
解释：区间 [1,3] 和 [2,6] 重叠, 将它们合并为 [1,6].
*/
pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>>{
    if intervals.is_empty() {
        return intervals
    }
    // 按照区间起始位置排序
    intervals.sort_by_key(|interval| interval[0]);
    // 初始化结果数组，默认将第一个子数组添加到结果数组中
    let mut results = vec![intervals[0].clone()];
    for i in 1..intervals.len() {
        // 错误示范: result[result.len() - 1] 会取出一个值的副本（如果类型支持 Copy，否则会 move）
        // let last = results[results.len() - 1]

        // 取出results.last_mut取出最后一个值的借用，返回的是可变引用
        let last = results.last_mut().unwrap();
        // 如果当前子数组第一个元素小于上一个子数组最大的元素，肯定需要合并，但是需要在上一个元素和当前子数组最有一个元素去最大值
        if intervals[i][0] <= last[1] {
            last[1] = last[1].max(intervals[i][1]);
        } else {
            results.push(intervals[i].clone())
        }
    }
    return results
}


/**
给定一个数组 nums，编写一个函数将所有 0 移动到数组的末尾，同时保持非零元素的相对顺序。
请注意 ，必须在不复制数组的情况下原地对数组进行操作。

示例 1:

输入: nums = [0,1,0,3,12]
输出: [1,3,12,0,0]
*/
pub fn move_zeroes(nums: &mut Vec<i32>) {
    if nums.is_empty() {
        return
    }

    let mut slow = 0;

    for fast in 0..nums.len() {
        if nums[fast] != 0 {
            nums.swap(slow, fast);
            slow += 1;
        }
    }
}

/**
给定一个含有 n 个正整数的数组和一个正整数 target 。

找出该数组中满足其总和大于等于 target 的长度最小的 子数组 [numsl, numsl+1, ..., numsr-1, numsr] ，并返回其长度。如果不存在符合条件的子数组，返回 0 。



示例 1：

输入：target = 7, nums = [2,3,1,2,4,3]
输出：2
解释：子数组 [4,3] 是该条件下的长度最小的子数组。
示例 2：

输入：target = 4, nums = [1,4,4]
输出：1
 */
pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
    let mut start:usize = 0;
    let mut count:usize  = usize::MAX;
    let mut sum:i32 = 0;

    for end in 0..nums.len() {
        sum += nums[end];
        while sum >= target {
            if (end  - start + 1) < count {
                count = end - start + 1;
            }
            sum -= nums[start];
            start += 1;
        }
    }

    if (count == usize::MAX) {
        return 0;
    }
    count as i32
}


pub fn reverse_words(s: String) -> String {
    // 去除前后空格并按空格分割
    // let words: Vec<&str> = s.split(' ').collect();
    // 去除前后空格，并按空格分割
    let mut words: Vec<&str> = s.split_whitespace().collect();

    // 反转单词数组
    let len = words.len();
    for i in 0..len / 2 {
        words.swap(i, len - 1 - i);
    }

    // 用单个空格连接单词
    words.join(" ")
}