#include <stdio.h>

#define A 8
#define B 32

int main(){
unsigned int i,j,k;
unsigned short plain[A][B]={0};
unsigned short crypt[B][A]={0};
short c[A*B]={0};
unsigned short buf[A*B]={0};

gets(c);
printf("%s\n",c);

for(i=0;i<A*B;i++){
    for(j=0;j<16;j++){
    buf[i]=(buf[i]<<1);
    buf[i]^=c[i]%2;
    c[i]=(c[i]>>1);
    }
}

for(j=0;j<B;j++){
    for(k=0;k<A;k++){
        plain[k][j]=buf[j*A+k];
        printf("%d,",plain[k][j]);
        //printf("%d,",buf[j*8+k]);
    }
    printf("\n");
}
for(i=0;i<A;i++){
    for(j=0;j<B;j++){
    crypt[j][i]=plain[i][j];
    printf("%d,",crypt[j][i]);
    }
    printf("\n");
}

return 0;
}
