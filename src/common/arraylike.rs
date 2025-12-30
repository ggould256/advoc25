#![allow(dead_code)]
//! Utilities for things that look like arrays, such as iterators-of-iterators.

/// Transpose a 2D array-like structure.
pub fn transpose<T, U>(data: &[U]) -> Vec<Vec<T>>
where
    U: IntoIterator<Item = T> + Clone,
{
    let mut result: Vec<Vec<T>> = Vec::new();
    for row in data.iter().cloned() {
        for (i, item) in row.into_iter().enumerate() {
            if result.len() <= i {
                result.push(Vec::new());
            }
            result[i].push(item);
        }
    }
    result
}

/// Transpose a slice of strings, treating each string as a row of characters.
pub fn transpose_strings(data: &[&str]) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    for row in data.iter() {
        for (i, item) in row.chars().enumerate() {
            if result.len() <= i {
                result.push(String::new());
            }
            result[i].push(item);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;  
    #[test]
    fn test_transpose() {
        let data = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
        ];
        let transposed = transpose(&data);
        assert_eq!(transposed, vec![
            vec![1, 4],
            vec![2, 5],
            vec![3, 6],
        ]);
    }

    fn test_transpose_strings() {
        let data = vec![
            "abc",
            "def",
        ];
        let transposed = transpose_strings(&data);
        assert_eq!(transposed, vec![
            "ad",
            "be",
            "cf",
        ]);
    }
}
