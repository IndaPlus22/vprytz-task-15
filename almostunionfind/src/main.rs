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
    if parent[x as usize] != x {
        return find(parent[x as usize], parent);
    } else {
        return x;
    }
}

fn union(x: u32, y: u32, parent: &mut Vec<u32>) -> () {
    let y_root = find(y, parent);
    parent[y_root as usize] = find(x, parent);
}

fn sum(x: u32, parent: &mut Vec<u32>) -> u32 {
    let mut sum = 0;
    let x_parent = find(x, parent);
    println!("x_parent: {}", x_parent);

    for i in 0..parent.len() {
        let i_parent = find(i as u32, parent);

        if i_parent == x_parent {
            println!("i: {}, i_parent: {}", i, i_parent);
            println!("adding {} to sum", i + 1);
            sum += (i + 1) as u32;
        }
    }
    return sum;
}

fn num_elements(x: u32, parent: &mut Vec<u32>) -> u32 {
    // number of elements in set containing x
    let mut num = 0;
    for i in 0..parent.len() {
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
    for i in 0..n {
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
        let mut p: u32 = line.next().unwrap().parse().unwrap(); // first integer, but minus 1 to get index (because reasons)
        p = p - 1;
        let mut q: u32 = 0;

        // if op is 3, we only have one integer
        if op != 3 {
            q = line.next().unwrap().parse().unwrap(); // second integer
            q = q - 1;
        }

        // perform operation
        match op {
            1 => {
                // union sets in collection containing p and q
                union(p, q, &mut parent);

                // print parent!
                println!("{:?}", parent);
            }
            2 => {
                // move p to the set containing q
                let p_index = find(p, &mut parent) as usize;
                let q_index = find(q, &mut parent) as usize;
                parent[p_index] = q_index as u32;

                // print parent!
                println!("{:?}", parent);
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
