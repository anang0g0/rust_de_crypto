#include <stdio.h>
#include <math.h>

#include <stdio.h>
#include <inttypes.h>
#include <immintrin.h>


/*
** Using documented GCC type unsigned __int128 instead of undocumented
** obsolescent typedef name __uint128_t.  Works with GCC 4.7.1 but not
** GCC 4.1.2 (but __uint128_t works with GCC 4.1.2) on Mac OS X 10.7.4.
*/
typedef unsigned __int128 uint128_t;

/*      UINT64_MAX 18446744073709551615ULL */
#define P10_UINT64 10000000000000000000ULL   /* 19 zeroes */
#define E10_UINT64 19

#define STRINGIZER(x)   # x
#define TO_STRING(x)    STRINGIZER(x)

static int print_u128_u(uint128_t u128)
{
    int rc;
    if (u128 > UINT64_MAX)
    {
        uint128_t leading  = u128 / P10_UINT64;
        uint64_t  trailing = u128 % P10_UINT64;
        rc = print_u128_u(leading);
        rc += printf("%." TO_STRING(E10_UINT64) PRIu64, trailing);
    }
    else
    {
        uint64_t u64 = u128;
        rc = printf("%" PRIu64, u64);
    }
    return rc;
}
typedef unsigned long long int poly;
poly quo, quo_low, res, res_low;

#define MSB (~(~0ULL >> 1))

poly i, k;

poly seki( poly a,  poly b)
{

   poly c = 0;
  while (a != 0)
  {
    if ((a & 1) == 1)
    {
      c ^= b;
    }
    b <<= 1;
    a >>= 1;
  }

  return c;
}

poly itob( poly n)
{
   poly k = 0;

  while (n > 0)
  {
    if (n % 2 == 1)
    {
      k = k + 1;
    }
    n = (n >> 1);
  }

  //#  printf("k=%d\n",k);

  return k;
}

/* ���ӥå��������֤� */
int cb( poly x)
{
  int i = 0;

  while (x > 0)
  {
    x = (x >> 1);
    i = i + 1;
  }

  return i;
}

// F_2 quot
poly pq(poly p, poly d)
{

  poly t[64], q, y, r;

  if (cb(p) < cb(d))
    return p;

  q = p;
  y = d;
  r = 0;
  i = cb(q) - cb(y);
  y = (y << i);

  if (y > p)
    y = (y >> 1);

  while (cb(q) >= cb(d))
  {
    printf("bbb\n");
    y = d;
    i = cb(q) - cb(y);

    if (i > 0)
      y = (y << i);

    if (i < 0)
      break;

    if (cb(q) == cb(y))
    {
      r = r + (1 << i);
      itob(q);
      itob(y);
      q = (q ^ y);
      itob(q);
      itob(y);
    }
  }

  return r;
}

/* F_2 mod */
poly pd(poly p, poly d)
{
  poly t[64];
  poly q, y, r;

  if (cb(p) < cb(d))
  {
    return p;
  }

  //  print "a"

  q = p;
  y = d;
  r = 0;
  i = cb(q) - cb(y);

  y = (y << i);

  if (y > p)
  {
    y = (y >> 1);
  }
  while (cb(q) >= cb(d))
  {
    y = d;
    i = cb(q) - cb(y);
    printf("i=%d\n", i);
    if (i > 0)
      y = (y << i);

    if (i < 0)
      break;

    if (cb(q) == cb(y))
    {
      r = r + (1 << i);
      itob(q);
      itob(y);
      q = (q ^ y);
      itob(q);
      itob(y);
    }
  }

  return q;
}

// invert of integer
poly inv(poly a, poly n)
{
  poly d;
  poly q, t, r, x, s /*, gcd*/;

  x = 0;
  s = 1;

  d = n;
  while (a != 0)
  {
    q = pq(d, a);
    r = pd(d, a);
    d = a;
    a = r;
    t = x ^ seki(q, s);
    x = s;
    s = t;
    printf("aaa\n");
  }

  // gcd = d;

  return pd((x ^ n), pq(n, d));
}

void write_poly(poly p, poly p_low)
{
  poly q;

  q = MSB;
  while (q >= p_low)
  {
    printf((p & q) ? "1" : "0");
    q >>= 1;
  }
}

void divide(poly a, poly a_low, poly b, poly b_low)
{
  int i;
  quo = 0;
  quo_low = MSB;
  res = a;
  res_low = a_low;
  if (res_low > b_low)
    return;
  for (;;)
  {
    if (res & MSB)
    {
      quo |= quo_low;
      res ^= b;
    }
    if (res_low == b_low)
      break;
    res_low <<= 1;
    res <<= 1;
    quo_low >>= 1;
    /* printf("%d ",i); */
  }
}

void factorize(poly p, poly p_low)
{
  poly d, d_low;

  d = MSB;
  d_low = MSB >> 1;
  while (d_low > p_low)
  {
    divide(p, p_low, d, d_low);
    if (res == 0)
    { /* break; */
      write_poly(d, d_low);
      printf("*");
      p = quo;
      p_low = quo_low;
    }
    else
    {
      d += d_low;
      if (d == 0)
      {
        d = MSB;
        d_low >>= 1;
      }
    }
  }
  write_poly(p, p_low);
}

// test gcd
poly agcd(poly xx, poly yy)
{
  poly tt = 0, tmp;

  if (xx < yy)
  {
    tmp = xx;
    xx = yy;
    yy = tmp;
  }
  tt = pd(xx, yy);
  while (tt != 0)
  {
    xx = yy;
    yy = tt;
    tt = pd(xx, yy);
    printf("%b %b %b %b\n", yy, xx, tt, tmp);
  }

  return yy;
}

int testbit(poly bit, poly i)
{
  if (bit == 0)
    return 0;
  if (bit & (1 << i))
  {
    return 1;
  }
  else
  {
    return 0;
  }
}

int bitctr(poly c)
{
  int bit;

  bit = __builtin_popcount(c);
  printf("%b %d\n", c, bit);

  return bit;
}

int bitch(poly c)
{
  int bit;

  bit = __builtin_parityll(c);
  printf("%b %d\n", c, bit);

  return bit;
}




int main()
{
  poly p, p_low;

   uint128_t u128a = ((uint128_t)UINT64_MAX + 1) * 0x1234567890ABCDEFULL +
                      0xFEDCBA9876543210ULL;
    uint128_t u128b = ((uint128_t)UINT64_MAX + 1) * 0xF234567890ABCDEFULL +
                      0x1EDCBA987654320FULL;
    int ndigits = print_u128_u(u128a);
    printf("\n%d digits\n", ndigits);
    ndigits = print_u128_u(u128b);
    printf("\n%d digits\n", ndigits);

    uint128_t a = (uint128_t)0b1111111111111111111111111111111111111111111100000000000000000001;
    uint128_t d = (uint128_t)0b1000000000000000000000000000000000000000000000000000000000001111;
    uint128_t c=0;
    uint128_t e = (uint128_t)0b1111111111111111111111111111111111111111111000000001111111111111;
    ndigits = print_u128_u(a);
    printf("\n%d digits\n", ndigits);
    ndigits = print_u128_u(d);
    printf("\n%d digits\n", ndigits);
    c=a*d;
    ndigits = print_u128_u(c);
    printf("\n%d digits\n", ndigits);

  /*
  scanf("%d",&i);
  p=MSB; p_low= (MSB>>i);

    while(p_low!=0){
      write_poly(p,p_low); printf("=");
      factorize(p,p_low); printf("\n");
      p +=p_low;
      if(p==0){
        p=MSB; p_low>>=1;
      }
    }
    */
  
  
  uint128_t bit = (uint128_t)0b1000000000000000000000000000000000000000000000000000000000001111;
  uint128_t b=(uint128_t)0b1111111111111111111111111111111111111111111;

//  printf("%f %f\n",ceil(log2(a)),ceil(log2(b)));
  c = inv(a, bit);
  a = agcd(bit, a);
  printf("%llu\n", a);
  c=seki(d,e);
print_u128_u(c);
print_u128_u(e);
printf("\n%d\n",cb(e));
  exit(1);

  while (cb(c) < 32)
  {
    c = seki(a, b);
    printf("%b\n", c);
    a += 2;
    b += 2;
  }
  return 0;
}
