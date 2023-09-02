public class Solution {
    public int RemoveElement(int[] nums, int val) {
        if (nums == null || nums.Length == 0)
            return 0;
        
        int i = 0;

        for (int n = 0; n < nums.Length; n++) {
            while (n < nums.Length && nums[n] == val)
                n++;
            if (n < nums.Length)
                nums[i++] = nums[n];
        }
        return i;
    }
}