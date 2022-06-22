#include <stdio.h>
#include <math.h>

#include <inttypes.h>
#include <immintrin.h>

 static const unsigned short gf[256]={0,1,2,4,8,16,32,64,128,29,58,116,232,205,135,19,38,76,152,45,90,180,117,234,201,143,3,6,12,24,48,96,192,157,39,78,156,37,74,148,53,106,212,181,119,238,193,159,35,70,140,5,10,20,40,80,160,93,186,105,210,185,111,222,161,95,190,97,194,153,47,94,188,101,202,137,15,30,60,120,240,253,231,211,187,107,214,177,127,254,225,223,163,91,182,113,226,217,175,67,134,17,34,68,136,13,26,52,104,208,189,103,206,129,31,62,124,248,237,199,147,59,118,236,197,151,51,102,204,133,23,46,92,184,109,218,169,79,158,33,66,132,21,42,84,168,77,154,41,82,164,85,170,73,146,57,114,228,213,183,115,230,209,191,99,198,145,63,126,252,229,215,179,123,246,241,255,227,219,171,75,150,49,98,196,149,55,110,220,165,87,174,65,130,25,50,100,200,141,7,14,28,56,112,224,221,167,83,166,81,162,89,178,121,242,249,239,195,155,43,86,172,69,138,9,18,36,72,144,61,122,244,245,247,243,251,235,203,139,11,22,44,88,176,125,250,233,207,131,27,54,108,216,173,71,142};
 static const unsigned short fg[256]={0,1,2,26,3,51,27,199,4,224,52,239,28,105,200,76,5,101,225,15,53,142,240,130,29,194,106,249,201,9,77,114,6,139,102,48,226,37,16,34,54,148,143,219,241,19,131,70,30,182,195,126,107,40,250,186,202,155,10,121,78,229,115,167,7,192,140,99,103,222,49,254,227,153,38,180,17,146,35,137,55,209,149,207,144,151,220,190,242,211,20,93,132,57,71,65,31,67,183,164,196,73,127,111,108,59,41,85,251,134,187,62,203,95,156,160,11,22,122,44,79,213,230,173,116,244,168,88,8,113,193,248,141,129,100,14,104,75,223,238,50,198,255,25,228,166,154,120,39,185,181,125,18,69,147,218,36,33,138,47,56,64,210,92,150,189,208,206,145,136,152,179,221,253,191,98,243,87,212,172,21,43,94,159,133,61,58,84,72,110,66,163,32,46,68,217,184,124,165,119,197,24,74,237,128,13,112,247,109,162,60,83,42,158,86,171,252,97,135,178,188,205,63,91,204,90,96,177,157,170,161,82,12,246,23,236,123,118,45,216,80,175,214,234,231,232,174,233,117,215,245,235,169,81,89,176};

//aes
static const unsigned short invgf[256]={0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52,53,54,55,56,57,58,59,60,61,62,63,64,65,66,67,68,69,70,71,72,73,74,75,76,77,78,79,80,81,82,83,84,85,86,87,88,89,90,91,92,93,94,95,96,97,98,99,100,101,102,103,104,105,106,107,108,109,110,111,112,113,114,115,116,117,118,119,120,121,122,123,124,125,126,127,128,129,130,131,132,133,134,135,136,137,138,139,140,141,142,143,144,145,146,147,148,149,150,151,152,153,154,155,156,157,158,159,160,161,162,163,164,165,166,167,168,169,170,171,172,173,174,175,176,177,178,179,180,181,182,183,184,185,186,187,188,189,190,191,192,193,194,195,196,197,198,199,200,201,202,203,204,205,206,207,208,209,210,211,212,213,214,215,216,217,218,219,220,221,222,223,224,225,226,227,228,229,230,231,232,233,234,235,236,237,238,239,240,241,242,243,244,245,246,247,248,249,250,251,252,253,254,255,};
static const unsigned short invfg[256]={0,1,141,246,203,82,123,209,232,79,41,192,176,225,229,199,116,180,170,75,153,43,96,95,88,63,253,204,255,64,238,178,58,110,90,241,85,77,168,201,193,10,152,21,48,68,162,194,44,69,146,108,243,57,102,66,242,53,32,111,119,187,89,25,29,254,55,103,45,49,245,105,167,100,171,19,84,37,233,9,237,92,5,202,76,36,135,191,24,62,34,240,81,236,97,23,22,94,175,211,73,166,54,67,244,71,145,223,51,147,33,59,121,183,151,133,16,181,186,60,182,112,208,6,161,250,129,130,131,126,127,128,150,115,190,86,155,158,149,217,247,2,185,164,222,106,50,109,216,138,132,114,42,20,159,136,249,220,137,154,251,124,46,195,143,184,101,72,38,200,18,74,206,231,210,98,12,224,31,239,17,117,120,113,165,142,118,61,189,188,134,87,11,40,47,163,218,212,228,15,169,39,83,4,27,252,172,230,122,7,174,99,197,219,226,234,148,139,196,213,157,248,144,107,177,13,214,235,198,14,207,173,8,78,215,227,93,80,30,179,91,35,56,52,104,70,3,140,221,156,125,160,205,26,65,28,};

/*
** Using documented GCC type unsigned __int128 instead of undocumented
** obsolescent typedef name __uint128_t.  Works with GCC 4.7.1 but not
** GCC 4.1.2 (but __uint128_t works with GCC 4.1.2) on Mac OS X 10.7.4.
*/
typedef unsigned __int128 uint128_t;

/*      UINT64_MAX 18446744073709551615ULL */
#define P10_UINT64 10000000000000000000ULL /* 19 zeroes */
#define E10_UINT64 19

#define STRINGIZER(x) #x
#define TO_STRING(x) STRINGIZER(x)

static int print_u128_u(uint128_t u128)
{
  int rc;
  if (u128 > UINT64_MAX)
  {
    uint128_t leading = u128 / P10_UINT64;
    uint64_t trailing = u128 % P10_UINT64;
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
   // printf("bbb\n");
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
    //printf("i=%d\n", i);
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
   // printf("aaa\n");
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

//有限体の元の逆数
unsigned short
oinv(unsigned short a)
{
    int i;

    if (a == 0)
        return 0;

    return 256-fg[a]+1;

    printf("no return \n");
      exit (1);
    return 0;
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


  uint128_t u128a = ((uint128_t)UINT64_MAX + 1) * 0x1234567890ABCDEFULL +
                    0xFEDCBA9876543210ULL;
  uint128_t u128b = ((uint128_t)UINT64_MAX + 1) * 0xF234567890ABCDEFULL +
                    0x1EDCBA987654320FULL;
  int ndigits = print_u128_u(u128a);
  printf("\n%d digits\n", ndigits);
  ndigits = print_u128_u(u128b);
  printf("\n%d digits\n", ndigits);

  uint128_t aaa = (uint128_t)0b1111111111111111111111111111111111111111111100000000000000000001;
  uint128_t ddd = (uint128_t)0b1000000000000000000000000000000000000000000000000000000000001111;
  uint128_t ccc = 0;
  uint128_t ee = (uint128_t)0b1111111111111111111111111111111111111111111000000001111111111111;
  ndigits = print_u128_u(aaa);
  printf("\n%d digits\n", ndigits);
  ndigits = print_u128_u(ddd);
  printf("\n%d digits\n", ndigits);
  ccc = aaa * ddd;
  ndigits = print_u128_u(ccc);
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
  register poly a = 0b11, b = 0b100011011, c = 0, bit = 0b1001;
  // printf("%f %f\n",ceil(log2(a)),ceil(log2(b)));
 int ui=0;
//while(ui<10000000)
{
  //c = oinv(a); //
  c=inv(a, b);
  ui++;
printf("i=%d\n",c);
}
  //exit(1);
   a = agcd(bit, a);
  printf("%b %b\n", a,c);
// exit(1);


  poly aa = 0b11001;
   bit = 0b1000110;
   b = 0b100011011;
  unsigned short cc = 0, dd = 0b11;
  //  printf("%f %f\n",ceil(log2(a)),ceil(log2(b)));
  unsigned int ii=0;
  cc = 0b100011011;
  aa=0b10;
  b=0b1;
  for(ii=0;ii<10000000;ii++){
  //aa=inv(dd,cc);
  //b=invfg[123];
  b=pd(seki(123,123),cc);
  }
  printf("%d,",b);
  
  exit(1);
  //while (1)
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
    //cc += 2;
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

