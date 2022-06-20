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

int bitctr(int c)
{
  int bit;

  bit = __builtin_popcount(c);
  printf("%b %d\n", c, bit);

  return bit;
}

int bitch(int c)
{
  int bit;

  bit = __builtin_parityll(c);
  printf("%b %d\n", c, bit);

  return bit;
}


poly opowmod2(poly f, poly mod, int n)
{

  poly ret;

  ret = 2;
  while (n > 0)
  {
    if (n & 1)
      ret = pd(seki(ret, f), mod); // n の最下位bitが 1 ならば x^(2^i) をかける
    f = pd(seki(f, f), mod);
    n >>= 1; // n を1bit 左にずらす
    printf("ret=%llb\n", ret);
  }
  return ret;
}

//多項式のべき乗余
poly opowmod(poly f, poly mod, int n)
{
  int i, j = 0;

  //繰り返し２乗法
  for (i = 1; i < n + 1; i++)
  {
    printf("pre==%llu\n", f);
    f = seki(f, f);
    printf("f=%llu\n", f);
    if (cb(f) - 1 > cb(mod) - 1)
      f = pd(f, mod);
  }
  printf("f=%llb mod=%llb\n", f, mod);

  return f;
}

//多項式のべき乗
poly opow(poly f, int n)
{
  int i;
  poly g = 0;

  g = f;

  for (i = 1; i < n; i++)
    g = seki(g, f);

  return g;
}



// GF(2^m) then set m in this function.
int ben_or(poly f)
{
  int i, n, flg = 0;
  poly s = 0, u = 0, r = 0, d = 0;

  // if GF(8192) is 2^m and m==13 or if GF(4096) and m==12 if GF(16384) is testing
  int m = 1;
  // m=12 as a for GF(4096)=2^12 defined @ gloal.h or here,for example m=4 and GF(16)

  s = 2;
  r = s;
  n = cb(f);

  printf("%d\n", n);
  if (n == 0)
    return -1;
  r = 2;
  int j = (cb(f) - 1) / 2 + 1;
  i = 0;

  // r(x)^{q^i} square pow mod
  while (i < 30)
  {
    flg = 1;
    r = opowmod(r, f, 1);
    // r=pd(opow(r,2),f);

    u = r ^ s;
    printf("i=r=u=============%d %llb %llb\n", i, r, u);
    if (cb(u) - 1 == 0 && u == 0)
      return 1;
    if (cb(u) - 1 == 0 && u == 1)
    {
      i++;
      flg = 0;
    }

    if (cb(u) > 1)
      d = agcd(f, u);
    if (cb(d) > 1)
      return 1;
    if (flg == 0)
      i++;
    printf("%b\n", d);
    if (cb(d) > 1)
    {
      printf("!!!!==%b\n", d);
      return 1;
    }
    // printf("i!!!!!!!!!!!!!!!!!!! %d\n",i);
    i++;
  }

  return 0;
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
  a = agcd(bit, a);
  printf("%b %b\n", a,c);
  //exit(1);

  poly aa = 0b11001;
   bit = 0b1000110;
   b = 0b100011011;
  poly cc = 0, dd = 0b11;
  //  printf("%f %f\n",ceil(log2(a)),ceil(log2(b)));

  cc = 0b10000000000000000000000000000001;
  while (1)
  {
    i = ben_or(cc);
    if (i == 1)
    {
      printf("baka\n");
    }
    else
    {
      printf("irreducible=%llb\n", cc);
    }
    cc += 2;
  }
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

