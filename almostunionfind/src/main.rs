/***
 * Based on Template for Rust solutions to Kattis problems
 * By: Viola Söderlund <violaso@kth.se>
 * See: https://open.kattis.com/help/rust
 * Author: Vilhelm Prytz <vilhelm@prytznet.se>
 */

use std::io;
use std::io::prelude::*;
use std::process::exit;

fn find(x: u32, parent: &mut Vec<u32>) -> u32 {
    if parent[x as usize - 1] == x {
        return x;
    }
    let p = find(parent[x as usize - 1], parent);
    parent[x as usize - 1] = p;
    return p;
}

fn union(x: u32, y: u32, parent: &mut Vec<u32>) -> () {
    let y_root = find(y, parent);
    parent[y_root as usize - 1] = find(x, parent);
}

fn sum(x: u32, parent: &mut Vec<u32>) -> u32 {
    let mut sum = 0;
    for i in 1..parent.len() {
        if find(i as u32, parent) == x {
            sum += i as u32;
        }
    }
    return sum;
}

fn num_elements(x: u32, parent: &mut Vec<u32>) -> u32 {
    // number of elements in set containing x
    let mut num = 0;
    for i in 1..parent.len() {
        if find(i as u32, parent) == x {
            num += 1;
        }
    }
    return num;
}

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

    let mut parent: Vec<u32> = Vec::new();
    for i in 1..n + 1 {
        parent.push(i);
    }

    // get all lines in one go until EOF
    let lines = lines.collect::<Vec<_>>();

    // check that we have the correct number of lines
    if lines.len() != m as usize {
        eprintln!("incorrect number of lines");
        exit(1);
    }

    // loop through all lines
    for line in lines {
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
                union(p, q, &mut parent);
            }
            2 => {
                // move p to the set containing q
                let p_index = find(p, &mut parent) as usize;
                let q_index = find(q, &mut parent) as usize;
                parent[p_index] = q_index as u32;
            }
            3 => {
                // return the number of elements and the sum of elements in the set containing p
                let sum = sum(p, &mut parent);
                let num_elements = num_elements(p, &mut parent);

                // print number of elements and sum of elements in set containing p
                println!("{} {}", num_elements.to_string(), sum.to_string());
            }
            _ => {
                eprintln!("unknown operation");
                exit(1);
            }
        }
    }
}
