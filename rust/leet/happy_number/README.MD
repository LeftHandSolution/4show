# [Happy Number](https://leetcode.com/problems/happy-number)
`easy`

## Complexity:  
- Time: O(1) - largest possible squared digits sum of provided contraint max(n) = 2^31 - 1 is 810 digits number. No matter how big number is, we will cycle only a small number of digits before returning results as the sequences will start repeating.
- Space: O(log_10_N) - at worst we store each unique digit-square sum once and we can have maximum 10 digits based on our constrains. If n has d digits, then d ≈ log₁₀(n) and O(d) = O(log₁₀(n)) = O(log n)

## Notes:  
- 
 