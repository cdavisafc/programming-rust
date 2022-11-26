use std::env;

fn main() {

    let args: Vec<String> = env::args().skip(1).collect();

    let mutable = args.len() == 1 && args[0] == "m";
    let f = if mutable { gcd } else { gcd_immutable };

    let x = 305027408450782760*30;
    let y = 634074329764*30*198*730;
    let z = 45295720548*219328*17;
    let mut c = 1000000;
    let mut g = 0;
    while c > 0 {
        //println!("GCD of {} and {} = {}", x,y,gcd (x, y));
        //println!("GCD(i) of {} and {} = {}", x,y,gcd_immutable (x, y));
        //xx = gcd(x,y);
        g = f(x,y);
        g = f(g,z);
        c=c-1;
    }
    println!("Ran with mutable = {}", f == gcd);
    println!("The GCD of {}, {}, and {} is {}", x, y, z, g);
}

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}

fn gcd_immutable(n: u64, m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    gcd_r(n, m)
}

fn gcd_r(n: u64, m: u64) -> u64 {
    if m == 0 {
        n
    } else {
        if m < n {
            gcd_r (m, n % m)
        } else {
            gcd_r (n, m % n)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::gcd_immutable;
    use crate::gcd;


    #[test]
    fn tests() {
        // let primes: [i32; 20] = [2; 3; 5; 7; 11; 13; 17; 19; 23; 29; 31; 37; 41; 43; 47; 53; 59; 61; 67; 71; 73; 79; 83; 89; 97];

        let n1 = 36;
        let n2 = 54;
        let a = 18;

        assert_eq!(gcd(n1, n2), a);
        assert_eq!(gcd(n2, n1), a);
        assert_eq!(gcd_immutable(n1, n2), a);
        assert_eq!(gcd_immutable(n2, n1), a);
 
    }
    

}