#![allow(non_snake_case)]
#![allow(dead_code)]

use itertools::Itertools;

fn main() {
    println!("Hello, world!");
}

/*
centuryFromYear
0 -> 0
1..=100 -> 1
2..=200 -> 2
1801..=1900 -> 19
1901..=2000 -> 20
2001..=2100 -> 21
https://stackoverflow.com/questions/2422712/rounding-integer-division-instead-of-truncating/2422723#2422723

palindrome
- https://stackoverflow.com/questions/27996430/reversing-a-string-in-rust/38083610#38083610
- https://stackoverflow.com/questions/22118221/how-do-you-iterate-over-a-string-by-character
*/
fn centuryFromYear(year: i32) -> i32 {
    (year + 100 - 1) / 100
}

fn adjacentElementsProduct(inputArray: Vec<i32>) -> i32 {
    let mut max = inputArray[0] * inputArray[1];
    if inputArray.len() > 2 {
        for i in 1..(inputArray.len() - 1) {
            let product = inputArray[i] * inputArray[i + 1];
            if product > max {
                max = product;
            }
        }
    }
    max
}

fn shapeArea(n: i32) -> i32 {
    if n < 2 {
        n
    } else {
        n * n + (n - 1) * (n - 1)
    }
}

fn makeArrayConsecutive(statues: Vec<i32>) -> i32 {
    let len = statues.len() as i32;
    if len == 1 {
        0
    } else {
        let mut sorted: Vec<i32> = statues.to_vec();
        sorted.sort();
        sorted[sorted.len() - 1] - sorted[0] + 1 - len
    }
}

fn almostIncreasingSequence(sequence: Vec<i32>) -> bool {
    if sequence.len() < 3 {
        true
    } else if sequence.len() == 3 {
        sequence[0] < sequence[1] || sequence[0] < sequence[2] || sequence[1] < sequence[2]
    } else {
        let mut no_increase = sequence[1] <= sequence[0];
        for i in 2..(sequence.len() - 1) {
            //println!("sequence[{}] {}", i, sequence[i]);
            //println!("no_increase {:?}", no_increase);
            if sequence[i] <= sequence[i - 1] {
                if no_increase
                    || sequence[i] <= sequence[i - 2] && sequence[i + 1] <= sequence[i - 1]
                {
                    return false;
                }
                no_increase = true;
            }
        }

        if no_increase && sequence[sequence.len() - 1] <= sequence[sequence.len() - 2] {
            return false;
        }
        true
    }
}

// tuple_windows clones the iterator elements so that they can be part of successive windows, this makes it most suited for iterators of references and other values that are cheap to copy.
// https://docs.rs/itertools/0.8.2/itertools/trait.Itertools.html#method.tuple_windows
fn almostIncreasingSequenceItertools(sequence: Vec<i32>) -> bool {
    println!("sequence {:?}", sequence);
    if sequence.len() < 3 {
        println!("sequence.len() < 3");
        true
    } else if sequence.len() == 3 {
        println!("second case");
        sequence[1] > sequence[0] || sequence[2] > sequence[1] || sequence[2] > sequence[0]
    } else {
        println!("third case");
        let mut no_increase = sequence[1] <= sequence[0];
        println!("no_increase {:?}", no_increase);
        for (prevprev, prev, curr, next) in (&sequence).into_iter().tuple_windows() {
            println!(
                "prevprev {}, prev {}, curr {}, next {}",
                prevprev, prev, curr, next
            );
            println!("no_increase {:?}", no_increase);
            if curr <= prev {
                if no_increase || curr <= prevprev && next <= prev {
                    return false;
                }
                no_increase = true;
            }
        }

        if no_increase && sequence[sequence.len() - 1] <= sequence[sequence.len() - 2] {
            return false;
        }
        true
    }
}

fn matrixElementsSum(matrix: Vec<Vec<i32>>) -> i32 {
    let mut sum = 0;
    for col in 0..matrix[0].len() {
        for row in 0..matrix.len() {
            if matrix[row][col] == 0 {
                break;
            }
            sum += matrix[row][col];
            //println!("matrix[{}][{}]: {}", row, col, matrix[row][col]);
        }
        //println!("");
    }
    sum
}

// Also interesting solution by csteffen https://app.codesignal.com/arcade/intro/level-2/xskq4ZxLyqQMCLshr/solutions?solutionId=dix4tb3TBg8i4Nw7A
fn matrixElementsSum2(matrix: Vec<Vec<i32>>) -> i32 {
    (0..matrix[0].len())
        .map(|col| {
            matrix
                .iter()
                .map(|r| r[col])
                .take_while(|&p| p > 0)
                .sum::<i32>()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_almostIncreasingSequence() {
        let samples: Vec<(Vec<i32>, bool)> = vec![
            (vec![], true),
            (vec![1], true),
            (vec![1, 1], true),
            (vec![1, 1, 1], false),
            (vec![1, -1, 0], true),
            (vec![3, 6, -2, -5, 7, 3], false),
            (vec![1, 3, 2, 1], false),
            (vec![1, 2, 1, 2], false),
            (vec![0, -2, 5, 6], true),
            (vec![10, 1, 2, 3, 4, 5], true),
            (vec![40, 50, 60, 10, 20, 30], false),
            (vec![3, 6, 5, 8, 10, 20, 15], false),
            (vec![1, 1, 2, 3, 4, 4], false),
        ];

        for (inputArray, expected) in samples {
            println!("\n{:?}", inputArray);
            assert_eq!(expected, almostIncreasingSequence(inputArray));
        }
    }

    #[test]
    fn test_almostIncreasingSequenceItertools() {
        let samples: Vec<(Vec<i32>, bool)> = vec![
            (vec![], true),
            (vec![1], true),
            (vec![1, 1], true),
            (vec![1, 1, 1], false),
            (vec![1, -1, 0], true),
            (vec![3, 6, -2, -5, 7, 3], false),
            (vec![1, 3, 2, 1], false),
            (vec![1, 2, 1, 2], false),
            (vec![0, -2, 5, 6], true),
            (vec![10, 1, 2, 3, 4, 5], true),
            (vec![40, 50, 60, 10, 20, 30], false),
            (vec![3, 6, 5, 8, 10, 20, 15], false),
            (vec![1, 1, 2, 3, 4, 4], false),
        ];

        for (inputArray, expected) in samples {
            println!("\n{:?}", inputArray);
            let result = almostIncreasingSequenceItertools(inputArray);
            assert_eq!(expected, result);
        }
    }

    #[test]
    fn test_matrixElementsSum() {
        let samples: Vec<(Vec<Vec<i32>>, i32)> = vec![(
            vec![vec![0, 1, 1, 2], vec![0, 5, 0, 0], vec![2, 0, 3, 3]],
            9,
        )];

        for (inputArray, expected) in samples {
            println!("\n{:?}", inputArray);
            assert_eq!(expected, matrixElementsSum(inputArray));
        }
    }
}
