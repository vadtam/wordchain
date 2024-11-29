use std::collections::{HashMap, HashSet, VecDeque};

#[derive(PartialEq)]
enum StringPattern {
    EQUAL,
    CONNECTING,
    NOTCONNECTING
}

impl StringPattern {
    fn per_char_diff(a: &str, b: &str) -> StringPattern {
        let diff = a.chars().zip(b.chars()).filter(|&(a, b)| a != b).count();
        match diff {
            0 => StringPattern::EQUAL,
            1 => StringPattern::CONNECTING,
            _ => StringPattern::NOTCONNECTING
        }
    }

    fn strip_suffix(ss: &String) -> &str {
        let suffix = "\r";
        if ss.ends_with(suffix) {
            ss.strip_suffix(suffix).unwrap()
        } else {
            ss
        }
    }
}

type Node = usize;
type Neighbours = Vec<Node>;

pub fn word_chain_game(start: &str, end: &str, words: &[String]) -> Option<usize> {
    /*
        observations (and assumptions):
            1. start and end exist in words
            2. the words sorting is not important for N^2 complexity
     */
    
    // compute graph layout
    let mut graph: HashMap<Node, Neighbours> = HashMap::new();
    let mut node_from: Option<Node> = None;
    let mut node_to: Option<Node> = None;
    {
        for (node_index, node) in words.iter().enumerate() {
            let mut neighbour_indexes: Neighbours = Vec::new();
            let node = StringPattern::strip_suffix(node);
            for (neighbour_index, word) in words.iter().enumerate() {
                let word = StringPattern::strip_suffix(word);
                if StringPattern::per_char_diff(node, word) == StringPattern::CONNECTING {
                    neighbour_indexes.push(neighbour_index);
                }
            }
            graph.insert(node_index, neighbour_indexes);

            if node == start {
                if node_from.is_none() {
                    node_from = Some(node_index);
                }
            }
            if node == end {
                if node_to.is_none() {
                    node_to = Some(node_index);
                }
            }
        }
    }

    // assumption check
    if node_from.is_none() || node_to.is_none() {
        return None;
    }

    // build short path, omit loops
    {
        let node_from = node_from.unwrap();
        let node_to = node_to.unwrap();

        if node_from == node_to {
            return Some(0);
        }
    
        let mut queue: VecDeque<(Node, Vec<Node>, usize)> = VecDeque::new();
        let mut visited: HashSet<Node> = HashSet::new();
    
        // Start BFS with the starting node
        queue.push_back((node_from, vec![node_from], 0));
        visited.insert(node_from);
    
        while let Some((current_node, path, path_length)) = queue.pop_front() {
            if let Some(neighbors) = graph.get(&current_node) {
                for &neighbor in neighbors {
                    if !visited.contains(&neighbor) {
                        let mut new_path = path.clone();
                        new_path.push(neighbor);
    
                        if neighbor == node_to {
                            return Some(path_length + 1);
                        }
    
                        queue.push_back((neighbor, new_path, path_length + 1));
                        visited.insert(neighbor);
                    }
                }
            }
        }
    }

    // If no path is found
    None
}

#[cfg(test)]
mod tests {
    use crate::word_chain_game;

    #[test]
    fn test_path_exists() {
        let word_list = read_word_list().unwrap();
        let start = "dog";
        let end = "imp";
        assert_eq!(word_chain_game(start, end, &word_list), Some(7));
    }

    #[test]
    fn test_path_does_not_exist() {
        let word_list = read_word_list().unwrap();
        let start = "mad";
        let end = "gnu";
        assert_eq!(word_chain_game(start, end, &word_list), None);
    }

    #[test]
    fn test_path_len_0() {
        let word_list = read_word_list().unwrap();
        let start = "zek";
        let end = "zek";
        assert_eq!(word_chain_game(start, end, &word_list), Some(0));
    }

    #[test]
    fn test_path_len_1() {
        let word_list = read_word_list().unwrap();
        let start = "zap";
        let end = "zas";
        assert_eq!(word_chain_game(start, end, &word_list), Some(1));
    }

    #[test]
    fn test_path_len_2() {
        let word_list = read_word_list().unwrap();
        let start = "yid";
        let end = "zin";
        assert_eq!(word_chain_game(start, end, &word_list), Some(2));
    }

    fn read_word_list() -> Result<Vec<String>, std::io::Error> {
        let words = std::fs::read_to_string("res/three_letter_words.txt")?;
        let word_list = words.split('\n').map(Into::into).collect();
        Ok(word_list)
    }
}
