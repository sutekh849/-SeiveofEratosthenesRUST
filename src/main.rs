extern crate bit_vec;

use std::fs;
use std::io::{BufWriter, Write};
use bit_vec::BitVec;
/*
 * This is another implementation of the Seive of Eratosthenes algorithm for 
 * finding prime numbers, this time in rust. Mostly wrote it as a way of getting back
 * to both programming and to rust after a long time writing Java for work.
 */
fn main() {
    
   let mut file = BufWriter::new(fs::File::create("primes.txt").unwrap());
   let max: usize = 15485863; //millionth prime number - change if you want more!
   
   let mut bv = BitVec::from_elem(max, true);
   
   bv.set(0, true);
   bv.set(1, true);
   
   for number in 2..max
   {
       if (bv.get(number)) == Some(true)
       {
           writeln!(file, "{}", number).expect("Failed to write to file");
           for number2 in (number.pow(2)..max).step_by(number)
           {
               bv.set(number2, false);
           }
       }
   }

}
