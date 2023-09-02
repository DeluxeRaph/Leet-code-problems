function removeDuplicates(nums: number[]): number {
    let d = 1;

    for(let i = 1; i < nums.length; i++) {
        if (nums[i] !== nums[i - 1]) {
            nums[d] = nums[i];
            d += 1;
        }
    }
    return d;
};