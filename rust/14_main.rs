use std::cmp;
use std::collections::HashMap;

fn main() {
    let strs = vec!["flower".to_string(),
                    "flow".to_string(),
                    "flight".to_string()];
    let ans = longest_common_prefix_trie(strs);
    println!("{}", ans);
}

pub fn longest_common_prefix_trie(strs: Vec<String>) -> String {
    if strs.is_empty() { return String::new(); }  
    if strs.len() == 1 { return strs[0].to_owned(); }

    let mut trie = Trie::new();      
    for string in strs.to_owned() {
        trie.insert(string);
    }
    return trie.search_longest_prefix(strs[0].to_owned());
}

struct Trie {
    root: TrieNode,
}

struct TrieNode {
    ch: char,
    is_root: bool,
    children: HashMap<char, TrieNode>
}

impl Trie {
    fn new() -> Self {
        Trie {
            root: TrieNode {
                ch: '*',
                is_root: true,
                children: HashMap::new(),
            } 
        }
    }
    fn insert(&mut self, word: String) {
        let mut current = &mut self.root;
        for c in word.chars() {
            if !current.children.contains_key(&c) {
                current.children.insert(
                    c,
                    TrieNode {
                        ch: c,
                        is_root: true,
                        children: HashMap::new(),
                    },
                );
            }
            current = current.children.get_mut(&c).unwrap();
        }
        current.is_root = false;
    }

    fn search(&self, word: String) -> bool {
        let mut current = &self.root;
        for c in word.chars() {
            if !current.children.contains_key(&c) {
                return false;
            }
            current = current.children.get(&c).unwrap();
        }

        !current.is_root
    }

    fn starts_with(&self, prefix: String) -> bool {
        let mut current = &self.root;
        for c in prefix.chars() {
            if !current.children.contains_key(&c) {
                return false;
            }
            current = current.children.get(&c).unwrap();
        }
        true
    }

    fn search_longest_prefix(&self, word: String) -> String {
        let mut node = &self.root;
        let mut prefix = String::new();
        for index in 0..word.len() {
            let current_letter = word.chars().nth(index).unwrap();
            if node.children.contains_key(&current_letter) 
                && node.children.len() == 1 && node.is_root {
                prefix.push(current_letter);
                node = node.children.get(&current_letter).unwrap();
            } else {
                return prefix;
            }
        }
        return prefix;
    }
}

pub fn longest_common_prefix_binary_search(strs: Vec<String>) -> String {
    if strs.is_empty() {
        return String::new();
    }
    let mut min_len = usize::MAX;
    for string in strs.to_owned() {
        min_len = cmp::min(min_len, string.len());
    }
    let mut low = 1;
    let mut high = min_len;
    while low <= high {
        let middle = (low + high) / 2;
        if is_common_prefix(&strs, middle) {
            low = middle + 1;
        } else {
            high = middle - 1;
        }
    }
    return strs[0].chars().take((low + high) / 2).collect();
}

pub(self) fn is_common_prefix(strs: &Vec<String>, len: usize) -> bool {
    let string: String = strs[0].chars().take(len).collect();
    for index in 0..strs.len() {
        if !strs[index].starts_with(&string) { return false; }
    }
    return true;
}

pub fn longest_common_prefix_divide_conquer(strs: Vec<String>) -> String {
    let vec_length = strs.len();
    if strs.is_empty() { return String::new() };    
    return longest_common_prefix_divide(strs, 0 , vec_length - 1);
}

pub(self) fn longest_common_prefix_divide(strs: Vec<String>, l: usize, r: usize) -> String {
    if l == r {
        return strs[l].to_owned();
    } else {
        let mid = (l + r)/2;
        let lcp_left = longest_common_prefix_divide(strs.to_owned(), l , mid);
        let lcp_right = longest_common_prefix_divide(strs.to_owned(), mid + 1, r);
        return common_prefix(lcp_left, lcp_right);
   }
}

pub(self) fn common_prefix(left: String, right: String) -> String {
    let min = cmp::min(left.len(), right.len());       
    for index in 0..min {
        if left.chars().nth(index) != right.chars().nth(index) {
            return left.chars().take(index).collect();
        }
    }
    return left.chars().take(min).collect();
}

pub fn longest_common_prefix_vertical_scanning(strs: Vec<String>) -> String {
    if strs.is_empty() { return String::new() };
    for index1 in 0..strs[0].len() {
        let ch = strs[0].to_owned().chars().nth(index1);
        for index2 in 1..strs.len() {
            if index1 == strs[index2].len() || strs[index2].chars().nth(index1) != ch {
                return strs[0].chars().skip(0).take(index1).collect()
            }
        }
    }
    return strs[0].to_owned();
}

pub fn longest_common_prefix_horizontal_scanning(strs: Vec<String>) -> String {
    if strs.len() == 0 { 
        return String::new() 
    };
    let mut prefix = strs[0].to_owned();
    for index in 0..strs.len() {
        while strs[index].find(prefix.as_str()) != Some(0) {
            prefix = prefix.chars().skip(0).take(prefix.len() - 1).collect();
            if prefix.is_empty() { return String::new() };
        }
    }        
    return prefix;
}

pub fn longest_common_prefix_my_solution_horizontal_scanning(strs: Vec<String>) -> String {
    let words_count = strs.len();
    let mut ans: String = String::new(); 
    if words_count == 0 { return ans };
    let mut small_word_length = 1000;

    for index in 0..words_count {
        let word_len = strs[index].len() ;
        if word_len < small_word_length {
            small_word_length = word_len;
        }
    }

    for word_index in 0..small_word_length {
        let mut count = 1;
        let mut last_char = '0';
        for vec_index in 0..words_count {
            let char = strs[vec_index].chars().nth(word_index).unwrap();
            if vec_index > 0 {
                if char == last_char {
                    count += 1;
                } else {
                    break;
                }
            } else {
                last_char = char;
            }
            if count == words_count {
                ans.push(char);
            }
        }
        println!("count = {}", count);
        if count < words_count { break };
    }
    println!("{} {} {}", words_count, small_word_length, ans);
    return ans;       
}
