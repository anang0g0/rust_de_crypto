#include <stdio.h>
#include <string.h>


int main(){
unsigned char a=12,b=23;
int i,j;
char m[256]={0};
unsigned char c[256]={0};

gets(m);
for(i=0;i<strlen(m);i++){
c[i]=a*m[i]+b;
printf("%d,",c[i]);
}
printf("\n");


return 0;
}