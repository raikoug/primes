use clap::{Parser, ValueEnum};
use std::f64;
use std::time::{Instant};

/// My implementation of the Algorythm Sieve of Eratosthenes

#[derive(Parser, Debug, Clone)]
#[command(name = "myPrimes")]
#[command(author = "Riccardo Bella <raikoug@gmail.com>")]
#[command(version = "1.0.0")]
#[command(about = "Prime Numbers utility", long_about = None)]
#[command(author, version, about, long_about = None)]
#[command(about, version, after_help = 
    "A simple utility to calculate prime numbers using the Sieve of Eratosthenes algorythm and more."
)]
struct Cli{
    /// Value
    value: u64,

    /// Mode - List or Check
    #[arg(short, long, value_enum, default_value_t = Mode::List)]
    mode: Mode,

}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum Mode{
    List,
    Check,
}

// fastest sqrt function
fn sqrt(n: u64) -> u64 {
    let mut x = n;
    let mut y = (x + 1) / 2;
    while y < x {
        x = y;
        y = (x + n / x) / 2;
    }
    x
}

fn sieve_001(n: u64) -> Vec<u64>{
    let mut primes = vec![true; (n+1) as usize];
    
    let mut res: Vec<u64> = Vec::new();
    primes[0] = false;
    primes[1] = false;

    for i in 2..(n+1) as usize {
        if primes[i] {
            res.push(i as u64);
            let mut j = i + i;
            while j <= n as usize {
                primes[j] = false;
                j += i;
            }
        }
    }


    res
}

fn sieve_002(n: u64) -> Vec<u64>{
    let mut primes = vec![true; (n+1) as usize];
    
    let mut res: Vec<u64> = Vec::new();
    primes[0] = false;
    primes[1] = false;

    for i in 2..((n+1) as f64).sqrt() as usize {
        if primes[i] {
            res.push(i as u64);
            let mut j = i + i;
            while j <= n as usize {
                primes[j] = false;
                j += i;
            }
        }
    }
    // we just added to res the primes from 2 to sqrt(n), but all other primes are still marked as true
    // so we just need to add them to res
    for i in ((n as f64).sqrt() as usize + 1)..(n+1) as usize {
        if primes[i] {
            res.push(i as u64);
        }
    }

    res
}

fn sieve_003(n: u64) -> Vec<u64>{
    let mut primes = vec![true; (n+1) as usize];
    
    let mut res: Vec<u64> = Vec::new();
    primes[0] = false;
    primes[1] = false;

    for i in 2..((n+1) as f64).sqrt() as usize {
        if primes[i] {
            res.push(i as u64);
            let mut j = i * i;
            while j <= n as usize {
                primes[j] = false;
                j += i;
            }
        }
    }
    // we just added to res the primes from 2 to sqrt(n), but all other primes are still marked as true
    // so we just need to add them to res
    for i in ((n as f64).sqrt() as usize + 1)..(n+1) as usize {
        if primes[i] {
            res.push(i as u64);
        }
    }

    res
}

fn sieve_004(n: u64) -> Vec<u64>{
    let mut primes: Vec<bool> = vec![[false, true]; ((n+1)/2) as usize].into_iter().flatten().collect();
    if (n+1) % 2 != 0 {
        primes.push(false);
    }
    primes[1] = false;
    primes[2] = true;

    let mut res: Vec<u64> = Vec::new();
    res.push(2 as u64);
    
    let limit = ((n as f64).sqrt() as usize) + 1;
    for i in (3..limit).step_by(2) {
        if primes[i] {
            res.push(i as u64);
            let mut j = i * i;
            while j <= n as usize {
                primes[j] = false;
                j += i;
            }
        }
    }
    // we just added to res the primes from 2 to sqrt(n), but all other primes are still marked as true
    // so we just need to add them to res
    for i in ((n as f64).sqrt() as usize + 1)..(n+1) as usize {
        if primes[i] {
            res.push(i as u64);
        }
    }

    res
}



fn _test_vec_creation(n: u64){
    // test if making a true vec and loop over multiple of 2 is faster than take.flatten.collect

    // method 1
    let start = Instant::now();
    let mut primes = vec![true; (n+1) as usize];
    primes[0] = false;
    primes[1] = false;
    for i in (4..(n+1) as usize).step_by(2) {
        primes[i] = false;
    }
    let duration = start.elapsed();
    println!("Method 1 in {:?}", duration); // Print the elapsed time

    // method 2
    let start = Instant::now();
    let mut primes: Vec<bool> = vec![[false, true]; ((n+1)/2) as usize].into_iter().flatten().collect();
    if (n+1) % 2 != 0 {
        primes.push(false);
    }
    primes[1] = false;
    primes[2] = true;

    let duration = start.elapsed();
    println!("Method 2 in {:?}", duration); // Print the elapsed time

}

fn main() {
    let args = Cli::parse();
    if args.mode == Mode::List{
        let start = Instant::now();
        let _values = sieve_001(args.value);
        let duration = start.elapsed();
        println!("001 Done in {:?}", duration); // Print the elapsed time

        let start = Instant::now();
        let _values = sieve_002(args.value);
        let duration = start.elapsed();
        println!("002 Done in {:?}", duration); // Print the elapsed time

        let start = Instant::now();
        let _values = sieve_003(args.value);
        let duration = start.elapsed();
        println!("003 Done in {:?}", duration); // Print the elapsed time

        let start = Instant::now();
        let _values = sieve_004(args.value);
        let duration = start.elapsed();
        println!("004 Done in {:?}", duration); // Print the elapsed time

    }


}
