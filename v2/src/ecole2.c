/**************************************************************************
 * ecole.c
 * generate GF(2^n) using irreducible unsigned intnomial
 * ゼフ対数表を作るためのプログラム。正規基底を生成します。 
 *
 * MIT LISENSE
 * Author : fumumue
 * edition history
 * date       version comment                                    by
 * ---------- ------- ------------------------------------------ -------------
 * 2022/06/20 0.1     created                                    fumumue
 * 2022/06/22 0.1.1   whole refactoring                          rubato6809
 * 2022/06/22 0.2     8192 version,                              fumumue
 * 2022/06/23 0.2.1   specify table size on command line         rubato6809
 */
#include <stdio.h>
#include <stdlib.h>

/* table size 最大 32768 要素まで */
#define MAX_TBL_SZ  0x8000
#define DEF_TBL_SZ  8192      // デフォルトサイズ

unsigned int fg[MAX_TBL_SZ] = {0};
unsigned int gf[MAX_TBL_SZ] = {0};

/* generate Galois Field over GF(2^?) */
static const unsigned int normal[15] = {
    0b1011,
    // "11001", /* GF(16) */
    0b10011,
    0b110111,
    0b1100001,
    0b11000001,
    // 0b100011011, // aes (irreducible but not primitive)
    // 0b110101001, // normal
    0b100011101, // sage
    // 0b100011011,
    0b1100110001,
    // 0b11000010011,
    0b10001101111, // sage1024
    0b110000001101,
    0b1000011101011, // sage 4096
    // 0b1100101000001, /* 4096 */
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
    ans.quotient  = qt;    // しょう
    ans.remainder = rm;    // あまり
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

    while (a > 0) {
        JOZAN kotae = jozan(d, a);
        unsigned short q = kotae.quotient; 
        unsigned short r = kotae.remainder;
        unsigned short t = x ^ seki(q, s);

        d = a, x = s;
        a = r, s = t;
    }

    return f2_mod((x ^ n), f2quot(n, d));
}

/*************************************************
 * outputTbl(char *tblName, unsigned int *pTbl, int size)
 * C言語、定数テーブル形式でテキスト出力する
 */
static void outputTbl(char *tblName, unsigned int pTbl[], int size)
{
    printf("static const unsigned short %s[%d]={", tblName, size);
    int last = size - 1;
    for (int i = 0; i < last; i++) {
        printf("%u,", pTbl[i]);
    }
    printf("%u};\n",  pTbl[last]);
}

// 有限体の逆元生成（ゼフ対数表：対数部）
/*************************************************
 * mkfg(int nElement) -- output fg[nElement]
 * nElement : 2^n
 */
void mkfg(int nElement)
{
    for (int i = 0; i < nElement; i++) {
        for (int j = 0; j < nElement; j++) {
            if (gf[i] == j)
                fg[j] = i;
        }
    }

    /* テーブルを出力する */
    outputTbl("fg", fg, nElement);
}

// 有限体の元の生成（ゼフ対数表：指数部）
/*************************************************
 * mkgf(int nElement) -- output gf[nElement]
 * nElement : 2^n, table size of gf[], fg[]
 */
void mkgf(int nElement)
{
    int msbPos = n2bitnum(nElement);        // 最上位ビット位置
    unsigned int pol = normal[msbPos - 4];  // ???
    unsigned int L = 1 << (msbPos - 1);     // 原始多項式の最大次数
    unsigned int N = pol ^ L;  // 原始多項式の最大次数を消した多項式の残り。
    unsigned int bit = 1;

    //// 各値を確認
    // fprintf(stderr, "nElement = 0x%x, pol = 0x%x\n", nElement, pol);
    // fprintf(stderr, "L = 0x%x, N = 0x%x\n", L, N);
    for (int i = 1; i < L; i++) {
        if (bit > L - 1) {     // もしbitが最大次数に達したら
            bit = bit - L;     // bitから最大次数の項 x^n を消す。
            bit = bit ^ N;     // 最大次数の項以下の原始多項式をbitにxorする。
        }
        gf[i] = bit;           // 最初は何もしないでx^iを入れていく。
        bit <<= 1;             // 原始多項式の次数を1上げる。
    }

    /* テーブルを出力する */
    printf("%d\n", nElement);  // ←必要ですか？
    outputTbl("gf", gf, nElement);
}

/*************************************************
 * main()
 * コマンドライン引数で生成するテーブルサイズを指定可能。
 * Note: 指定するサイズは２のべき乗を想定
 *       指定しない場合、DEF_TBL_SZ (== 8192) がデフォルトサイズ
 *       指定可能なサイズ上限は MAX_TBL_SZ (== 32768)
 *
 * Usage：
 *      $ ecole2 1024 > tbl1024.c
 *      $ ecole2      > tbl8192.c
 */
int main(int argc, char *argv[])
{
    int tblSize = DEF_TBL_SZ; 

    if (argc >= 2) tblSize = atoi(argv[1]);
    mkgf(tblSize);    // output gf[??]
    mkfg(tblSize);    // output fg[??]
    return 0;
}