#include <stdio.h>
#include <math.h>
typedef unsigned long long int poly;
poly quo, quo_low, res, res_low;

#define MSB (~(~0ULL >> 1))

int i, k;

poly seki(register poly a, register poly b)
{

  register poly c = 0;
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

poly itob(register poly n)
{
  register poly k = 0;

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
int cb(register poly x)
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
poly pq(int p, int d)
{

  int t[64], q, y, r;

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
int pd(poly p, poly d)
{
  int t[64];
  int q, y, r;

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
unsigned short inv(unsigned short a, unsigned short n)
{
  unsigned short d;
  unsigned short q, t, r, x, s /*, gcd*/;

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
int agcd(int xx, int yy)
{
  int tt = 0, tmp;

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

int testbit(int bit, int i)
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

int bitctr(int c){
int bit;

  bit = __builtin_popcount(c);
  printf("%b %d\n", c, bit);

return bit;
}

int bitch(int c){
  int bit;

  bit = __builtin_parityll(c);
  printf("%b %d\n", c, bit);

return bit;
}


int main()
{
  poly p, p_low;

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
  register poly a = 0b11, b = 0b100011011, c = 0, bit = 0b1001;
  // printf("%f %f\n",ceil(log2(a)),ceil(log2(b)));
  c = inv(a, b);
  a=agcd(bit,a);
  printf("%b\n",a);
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
