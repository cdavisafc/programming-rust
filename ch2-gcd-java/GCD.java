public class GCD {
  public static void main(String[] args) {

    boolean mutable = args.length > 0 && args[0].equals("m");
    final Gcd f;
    if (mutable) f=GCD::gcd; else f=GCD::gcd_immutable;
  
    long x = 305027408450782760L*30;
    long y = 634074329764L*30*198*730;
    long z = 45295720548L*219328*17;
    int c = 1000000;
    long g = 0;

    while (c > 0) {
      g = f.gcd(x,y);
      g = f.gcd(g,z);
      c=c-1;
    }
    System.out.println("Ran with mutable = " + mutable);
    System.out.println("The GCD of + " + x + ", " + y + "and " + z + " = " + g);
  }

  public interface Gcd {
    long gcd(long n, long m);
  }

  static long gcd(long n, long m) {
    while (m != 0) {
        if (m < n) {
            long t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    return n;
  }

  static long gcd_immutable(long n, long m) {
     return gcd_r(n, m);
  }

  static long gcd_r(long n, long m) {
    if (m == 0) {
        return n;
    } else {
        if (m < n) {
            return gcd_r (m, n % m);
        } else {
            return gcd_r (n, m % n);
        }
    }
  }
}