func maximumDifference(_ nums: [Int]) -> Int {
  var lp = 0
  var maxDiff = -1

  for rp in 0..<nums.count {
    if nums[rp] > nums[lp] {
      maxDiff = max(maxDiff, nums[rp] - nums[lp])
    } else {
      lp = rp
    }
  }
  return maxDiff
}

let nums = [7, 1, 5, 4]
print(maximumDifference(nums))
