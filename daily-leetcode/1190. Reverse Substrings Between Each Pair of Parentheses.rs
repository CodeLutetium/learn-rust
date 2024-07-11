// Solution using wormhole technique (from editorial) to solve in O(n) time
impl Solution {
    pub fn reverse_parentheses(s: String) -> String {
        // Helper function to pair the parentheses and return a vector containing the pairs
        fn pair_parentheses(s: &str) -> Vec<usize> {  
            let mut pairs: Vec<usize> = vec![0; s.len()];
            let mut stack: Vec<usize> = vec![];

            for (idx, char) in s.chars().enumerate() {
                if (char =='(') {
                    stack.push(idx);
                } else if (char == ')') {
                    let left_paren: usize = stack.pop().unwrap();
                    pairs[idx] = left_paren;
                    pairs[left_paren] = idx;
                } 
            }
            pairs
        }


        let pairs: Vec<usize> = pair_parentheses(&s);
        
        // Second pass: Build the result string
        let mut result: Vec<char> = vec![];
        let mut curr_idx: usize = 0;
        let mut direction: bool = true;
        let s = s.as_bytes();
        
        while (curr_idx < s.len()) {
            if (s[curr_idx] == b'(' || s[curr_idx] == b')') {
                // Jump through the wormhole and reverse direction
                curr_idx = pairs[curr_idx];
                direction = !direction;
            } else {
                // Append character to result
                result.push(s[curr_idx] as char);
            }
            // Continue traversal
            if (direction == true) {
                curr_idx += 1;
            } else {
                curr_idx -= 1;
            }
        }
        
        result.iter().collect()
    }
}