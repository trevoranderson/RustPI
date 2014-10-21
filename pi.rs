use std::num::pow;
use std::collections::Bitv;
use std::collections::bitv;
fn main() {
     for i in range(0, 10i64) {
         println!("{}", pi_digit(i));
     }
}
 fn pi_digit(nn:i64) -> i64 {
    let n = nn - 1;
    let x = 4f64 * pi_term(1, n) - 2f64 * pi_term(4, n) - pi_term(5, n) - pi_term(6, n);
    ((x - x.floor()) * 16f64) as i64
}
fn pi_term(j:i64, n:i64)-> f64 {
    // Calculate the left sum
    let mut s = 0f64;
    for k in range(0i64,n+1 as i64) {
        let r = 8 * k + j;
        s += (power_mod(16, n-k, r) as f64) / (r as f64);
        s = s - s.floor();
    }
    // Calculate the right sum
    let mut t = 0f64;
    let mut k = n+1 as i64;
    // Keep iterating until t converges (stops changing)
    loop {
        let r = 8 * k + j;
        let newt = t + (pow(16u, (n-k) as uint) as f64) / (r as f64);
        if t == newt {
            break;
        }
        else {
            t = newt;
        }
        k += 1;
    }
    s+t
}
fn power_mod(a:i64, b:i64, m:i64) -> i64 {
    if b == 0 {
        1  //EXIT condition
    }//if
    else  if b == 1 {
        a
    }
    else {
        let temp = power_mod(a,b/2,m);
        if b%2 == 0 {
            (temp*temp)%m
        }
        else{
             ((temp*temp)%m)*a%m
         }
    }
} //POWERMOD method
