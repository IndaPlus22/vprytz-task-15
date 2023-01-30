/***
 * Based on Template for Rust solutions to Kattis problems
 * By: Viola Söderlund <violaso@kth.se>
 * See: https://open.kattis.com/help/rust
 * Author: Vilhelm Prytz <vilhelm@prytznet.se>
 */

use std::io;
use std::io::prelude::*;
use std::process::exit;

/// Kattis calls main function to run your solution.
fn main() {
    // get standard input stream
    let input = io::stdin();

    // get input lines as iterative
    let mut lines = input.lock().lines().map(|_line| _line.ok().unwrap());
    // and get one line at a time,
    let next_line = lines.next().unwrap();

    // first line contains two integers, n and m "8 2", for example
    let mut line = next_line.split_whitespace();
    let n: u32 = line.next().unwrap().parse().unwrap(); // number of integers to start with in our collection
    let m: u32 = line.next().unwrap().parse().unwrap(); // number of operations to perform

    // initialize a set, an array of arrays, where each array is a set
    // initially, each array in the big array just contains one integer
    // the big array will at start look like: collection = [[1], [2], [3], [4], [5], [6], [7], [8], ... [n]]
    let mut collection: Vec<Vec<u32>> = Vec::new();
    for i in 1..n + 1 {
        collection.push(vec![i]);
    }

    // print it
    eprintln!("collection: {:?}", collection);

    eprintln!("we got n: {} and m: {}", n, m);

    // read each line, one at a time, in for loop
    // but only if we have more lines to read (m)
    for _ in 0..m {
        // get next line
        let line = lines.next().unwrap();

        // split line into two integers
        let mut line = line.split_whitespace();
        let op: u8 = line.next().unwrap().parse().unwrap(); // operation to perform
        let p: u32 = line.next().unwrap().parse().unwrap(); // first integer
        let mut q: u32 = 0;

        // if op is 3, we only have one integer
        if op != 3 {
            q = line.next().unwrap().parse().unwrap(); // second integer
        }

        // perform operation
        match op {
            1 => {
                // union sets in collection containing p and q
                eprintln!("union sets containing {} and {}", p, q);

                // find index of set containing p
                let mut p_index = 0;
                for i in 0..collection.len() {
                    if collection[i].contains(&p) {
                        p_index = i;
                        break;
                    }
                }

                // find index of set containing q
                let mut q_index = 0;
                for i in 0..collection.len() {
                    if collection[i].contains(&q) {
                        q_index = i;
                        break;
                    }
                }

                // if p and q are in the same set, do nothing
                if p_index == q_index {
                    continue;
                }

                // union the two sets
                let mut q_set = collection[q_index].clone();
                collection[p_index].append(&mut q_set);
                collection.remove(q_index);

                // print it
                eprintln!("collection: {:?}", collection);
            }
            2 => {
                // move p to the set containing q
                eprintln!("move {} to set containing {}", p, q);

                // find index of set containing p
                let mut p_index = 0;
                for i in 0..collection.len() {
                    if collection[i].contains(&p) {
                        p_index = i;
                        break;
                    }
                }

                // find index of set containing q
                let mut q_index = 0;
                for i in 0..collection.len() {
                    if collection[i].contains(&q) {
                        q_index = i;
                        break;
                    }
                }

                // if p and q are in the same set, do nothing
                if p_index == q_index {
                    continue;
                }

                // remove p from set containing p
                collection[p_index].retain(|&x| x != p);

                // add p to set containing q
                collection[q_index].push(p);

                // print it
                eprintln!("collection: {:?}", collection);
            }
            3 => {
                // return the number of elements and the sum of elements in the set containing p
                eprintln!(
                    "return number of elements and sum of elements in set containing {}",
                    p
                );

                // find index of set containing p
                let mut p_index = 0;
                for i in 0..collection.len() {
                    if collection[i].contains(&p) {
                        p_index = i;
                        break;
                    }
                }

                // print it
                eprintln!("collection: {:?}", collection);

                // print number of elements and sum of elements in set containing p
                println!(
                    "{} {}",
                    collection[p_index].len(),
                    collection[p_index].iter().sum::<u32>()
                );
            }
            _ => {
                eprintln!("unknown operation");
                exit(1);
            }
        }
    }

    eprintln!("Kattis skips this comment!");
    //println!("Print to standard output.");
}
