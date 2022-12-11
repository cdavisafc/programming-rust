I've created two versions of the GCD function - one that uses a loop with a mutable variable, and the other which calculates the GCD without any side effects - done recursively. See [this blog post] on why I honed in on this right away, but I'll summarize here:
* When you build your program with no (or fewer) side effects, you open up the possibility of the computer to do some optimizations that would otherwise not be possible. What type of computer program might apply such optimizations? Well, a compiler. And THAT is one of the things that Rust is known for - doing a whole lot of things at compile time that in other languages are not done. 
* Once I had written the gcd function without side effects, I wanted to see if there was a difference in execution speed. So, I created a handful of large ints, and calculated the GCD a million times.
* And indeed, the version without side effects does run faster. Not a lot faster, but enough to see the difference.

Here are my runs - if you pass in the command line arg "m" (for mutable) it will run the original algorithm, any other command line arg (or none) and it will run my recursive algorithm.

```
$ time target/release/gcd m
Ran with mutable = true
The GCD of 9150822253523482800, 2749473108722656800, and 168888536537979648 is 48

real    0m0.260s
user    0m0.256s
sys     0m0.002s
```

and 

```
$ time target/release/gcd
Ran with mutable = false
The GCD of 9150822253523482800, 2749473108722656800, and 168888536537979648 is 48

real    0m0.237s
user    0m0.234s
sys     0m0.002s
```

The above was compiled for release. Of course, there was an occasional run using the version with mutable variables that was slightly faster, and the other that was slightly slower, but what I am showing above is representative of the average.