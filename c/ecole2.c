/**************************************************************************
 * ecole.c
 * generate GF(2^n) using irreducible unsigned intnomial
 * ゼフ対数表を作るためのプログラム。正規基底を生成します。 
 *
 * Author : fumumue
 * edition history
 * date       version comment                                    by
 * ---------- ------- ------------------------------------------ -------------
 * 2022/06/20 0.1     created                                    fumumue
 * 2022/06/22 0.1.1   whole refactoring                          rubato6809
 *
 */
#include <stdio.h>
#include <stdlib.h>

/* target table size ?? この版は256固定 */
#define POW_OF_TWO  256


unsigned int fg[POW_OF_TWO]={0};
unsigned int gf[POW_OF_TWO]={0};

/* generate Galois Field over GF(2^?) */
static const unsigned int normal[15] = {
    0b1011,
    //"11001", /* GF(16) */
    0b10011,
    0b110111,
    0b1100001,
    0b11000001,
    //0b100011011, //aes
    //0b110101001, //normal
    0b100011101, //sage
    //0b100011011,
    0b1100110001,
    //0b11000010011,
    0b10001101111, //sage1024
    0b110000001101,
    0b1000011101011, //sage 4096
    //0b1100101000001, /* 4096 */
    //0b11011000000001, /* 8192 */
    0b10000000011011, /* Classic McEliece */
    0b110000100010001,
    0b1100000000000001,
    0b11010000000010001
};

/*************************************************
 * seki(unsigned int a, unsigned int b)
 * -- returns product of a & b, maybe.
 */
unsigned int seki(unsigned int a, unsigned int b)
{
    unsigned int prod = 0;    // seki means product ?

    while (a > 0) {
        if (a & 1) {
            prod = prod ^ b;
        }
        b <<= 1;
        a >>= 1;
    }
    return prod;
}

/*************************************************
 * n2bitnum(unsigned int num)
 * -- count bit number, bitサイズを数える(?)
 * argment value : num 
 *  return value : number of bits
 */
int n2bitnum(unsigned int num)
{
    int nbits;       // num to number of bits

    for (nbits = 0; num > 0; nbits++)
        num = num >> 1;

    return nbits;
}

/*************************************************
 * jozan(a, b) -- ２数のジョザン（除算のようなもの）
 * しょう(a / b)、あまり(a % b)、両方を構造体で返す
 */
typedef struct {
    unsigned int quotient;     // しょう（商）
    unsigned int remainder;    // あまり（余）
} JOZAN;

static JOZAN jozan(unsigned int p, unsigned int d)
{
    JOZAN ans = { p, p };         // { 0, p }; ではないだろうか？
    const int bitD = n2bitnum(d);

    if (n2bitnum(p) < bitD)
        return ans;

    unsigned int qt = 0;   // しょうになる
    unsigned int rm = p;   // あまりになる

    while (1) {
        int bitRM = n2bitnum(rm);
        if (bitRM < bitD) break;

        int shift = bitRM - bitD;
        if (shift < 0) break;

        unsigned int y = (shift > 0) ? d << shift : d;
        if (n2bitnum(y) == bitRM) {
            qt = qt + (1 << shift);
            rm = rm ^ y;
        }
    }
    /* 結果出た */
    ans.quotient  = qt;    /* しょう */
    ans.remainder = rm;    /* あまり */
    return ans;
}

/*************************************************
 * f2quot(int p, int d)
 * -- calcullate F_2 quot 
 * return value :  p / d;
 */
unsigned int f2quot(unsigned int p, unsigned int d)
{
    JOZAN t = jozan(p, d);
    return t.quotient;
}

/*************************************************
 * f2_mod(unsigned int p, unsigned int d)
 * -- calcullate F_2 mod, p % d 
 */
unsigned int f2_mod(unsigned int p, unsigned int d)
{
    JOZAN t = jozan(p, d);
    return t.remainder;
}

/*************************************************
 * inv(unsigned short a, unsigned short n)
 * -- invert integer
 */
unsigned short inv(unsigned short a, unsigned short n)
{
    unsigned short d = n;
    unsigned short x = 0;
    unsigned short s = 1;

    // ココロがわからないので、放置
    while (a > 0) {
        unsigned short q, r, t;

        q = f2quot(d, a);
        r = f2_mod(d, a);
        d = a;
        a = r;
        t = x ^ seki(q, s);

        x = s;
        s = t;
    }

    return f2_mod((x ^ n), f2quot(n, d));
}


// 有限体の逆元生成（ゼフ対数表：対数部）
/*************************************************
 * mk2(int nElement) -- output invfg[nElement]
 * nElement : 2^n
 */
void mkfg(int n)
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
  printf("static const unsigned short fg[%d]={",POW_OF_TWO);
  for (i = 0; i < POW_OF_TWO; i++)
  {
    if (i < POW_OF_TWO - 1)
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


// 有限体の元の生成（ゼフ対数表：指数部）
/*************************************************
 * mk1(int nElement) -- output invgf[nElement]
 * nElement : 2^n
 */
void mkgf(int n)
{
  int i, bit, count = 0;
  unsigned int pol, N, L;

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
    if (bit >= POW_OF_TWO) //もしbitが最大次数に達したら
    {
      bit = bit - POW_OF_TWO; //bitから最大次数の項 x^n を消す。
      printf("num==%d\n",bit);
      bit = bit ^ N; //最大次数の項以下の原始多項式を bit に xorする。
      printf("red==%d\n",bit);
    }
    gf[i] = bit; //最初は何もしないでx^iを入れていく。
    printf("gf[%d]=%d\n",i,bit);
    bit = (bit << 1); //原始多項式の次数を1上げる。
    printf("after=%d\n",bit);
  }
  printf("static const unsigned short gf[%d]={", POW_OF_TWO);
  for (i = 0; i < POW_OF_TWO; i++)
  {
    if (i < POW_OF_TWO - 1)
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




/*************************************************
 * main()
 * 注意：この版は256固定。
 */
int main(void)
{
    mkgf(POW_OF_TWO);    // output invgf[??]
    mkfg(POW_OF_TWO);    // output invfg[??]
    return 0;
}