/* generate GF(2^n) using irreducible unsigned intnomial */
//ゼフ対数表を作るためのプログラム。正規基底を生成します。

#include <stdio.h>
#include <stdlib.h>

#define O 256

/* generate Galois Field over GF(2^?) */
static const unsigned int normal[15] = {
    0b1011,
    //"11001", /* GF(16) */
    0b10011,
    0b110111,
    0b1100001,
    0b11000001,
    //0b100011011, //aes(not primitive !!)
    //0b110101001, //normal
    0b100011101, //sage
    //0b11000010011,
    0b10001101111, //sage1024
    0b110000001101,
    0b1000011101011, //sage 4096
    //0b1100101000001, /* 4096 */
    //0b11011000000001, /* 8192 */
    0b10000000011011, /* Classic McEliece */
    0b110000100010001,
    0b1100000000000001,
    0b11010000000010001};

unsigned int /*gf[O]={0},*/ fg[O]={0};
unsigned int gf[O]={0};
void makefg(int n)
{
  int i, j;

  for (i = 0; i < n; i++)
  {
    for (j = 0; j < n; j++)
    {
      if (gf[i] == j)
        fg[j] = i;
    }
  }
  printf("static const unsigned short fg[%d]={", O);
  for (i = 0; i < O; i++)
  {
    if (i < O - 1)
    {
      printf("%d,", fg[i]);
    }
    else
    {
      printf("%d", fg[i]);
    }
  }
  printf("};\n");

  return;
}

void mkgf(int n)
{
  int i, j, bit, count = 0;
  unsigned int pol, N, M, L;

  for (i = 0; i < 13; i++)
    pol = normal[i]; //strtoul(normal[i],(char **)NULL,2);

// define pol 
  switch (n)
  {

  case 8:
    pol = normal[0];
    printf("%d\n", n);
    break;

  case 16:
    pol = normal[1];
    printf("%d\n", n);
    break;

  case 32:
    pol = normal[2];
    printf("%d\n", n);
    break;

  case 64:
    pol = normal[3];
    printf("%d\n", n);
    break;

  case 128:
    pol = normal[4];
    printf("%d\n", n);
    break;

  case 256:
    pol = normal[5];
    printf("%d\n", n);
    break;

  case 512:
    pol = normal[6];
    printf("%d\n", n);
    break;

  case 1024:
    pol = normal[7];
    printf("%d\n", n);
    break;

  case 2048:
    pol = normal[8];
    printf("%d\n", n);
    break;

  case 4096:
    pol = normal[9];
    printf("%d\n", n);
    break;

  case 8192:
    pol = normal[10];
    printf("%d\n", n);
    break;

  default:  //16384 
    pol = normal[11];
    printf("%d\n", n);
    break;
  }
//printf("%d\n",pol);
//exit(1);
  L = pol;
  while (L > 0) //原始多項式の最大次数を計算する。
  {
    L = (L >> 1);
    count++;
  }
  //L = (L >> 1);
  count--;
  printf("deg=%d\n",count);
  //exit(1);
  N = pol ^ (1 << count); //原始多項式の最大次数を消した多項式の残り。
printf("N=%d\n",N);
//exit(1);

  gf[0] = 0;
  bit = 1;
  for (i = 1; i < 256; i++)
  {
    if (bit >= O) //もしbitが最大次数に達したら
    {
      bit = bit - O; //bitから最大次数の項 x^n を消す。
      printf("num==%d\n",bit);
      bit = bit ^ N; //最大次数の項以下の原始多項式を bit に xorする。
      printf("red==%d\n",bit);
    }
    gf[i] = bit; //最初は何もしないでx^iを入れていく。
    printf("gf[%d]=%d\n",i,bit);
    bit = (bit << 1); //原始多項式の次数を1上げる。
    printf("after=%d\n",bit);
  }
  printf("static const unsigned short gf[%d]={", O);
  for (i = 0; i < O; i++)
  {
    if (i < O - 1)
    {
      printf("%u,", gf[i]);
    }
    else
    {
      printf("%u", gf[i]);
    }
  }

  printf("};\n");
}

unsigned int seki(register unsigned int a, register unsigned int b)
{

  register unsigned int c = 0;
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


/* ���ӥå��������֤� */
int cb(register unsigned int x)
{
  int i = 0;

  while (x > 0)
  {
    x = (x >> 1);
    i = i + 1;
  }

  return i;
}

typedef struct {
  int rem;
  int quo;
} div;

// F_2 quot
div pq(int p, int d)
{
  div={0};

  int t[64], q, y, r,i;

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
      div.rem = r + (1 << i);
      div.qup = (q ^ y);
    }
  }

  return div;
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


void mk2(int n)
{
  int i, j, bit, count = 0;
  unsigned int pol, N, M, L;

  for (i = 0; i < 13; i++)
    pol = normal[i]; //strtoul(normal[i],(char **)NULL,2);

// define pol 
  switch (n)
  {

  case 8:
    pol = normal[0];
    printf("%d\n", n);
    break;

  case 16:
    pol = normal[1];
    printf("%d\n", n);
    break;

  case 32:
    pol = normal[2];
    printf("%d\n", n);
    break;

  case 64:
    pol = normal[3];
    printf("%d\n", n);
    break;

  case 128:
    pol = normal[4];
    printf("%d\n", n);
    break;

  case 256:
    pol = normal[5];
    printf("%d\n", n);
    break;

  case 512:
    pol = normal[6];
    printf("%d\n", n);
    break;

  case 1024:
    pol = normal[7];
    printf("%d\n", n);
    break;

  case 2048:
    pol = normal[8];
    printf("%d\n", n);
    break;

  case 4096:
    pol = normal[9];
    printf("%d\n", n);
    break;

  case 8192:
    pol = normal[10];
    printf("%d\n", n);
    break;

  default:  //16384 
    pol = normal[11];
    printf("%d\n", n);
    break;
  }

    int ii=0;
    //cc = 0b100011011;
  //aa=0b10;
  //b=0b1;
  int b=0;
  printf("static const unsigned short invfg[%d]={",O);
  for(ii=0;ii<256;ii++){
  b=inv(ii,pol);
  printf("%d,",b);
  }
  printf("};\n");

}


int main()
{
  int i, j, k;
  
  printf("static const unsigned short invgf[%d]={",O);
  for(i=0;i<O;i++){
  gf[i]=i;
  printf("%d,",gf[i]);
  }
  printf("};\n");
  mk2(O);
  //mkgf(O);
  //exit(1);

  //makefg(O);

  return 0;
}
