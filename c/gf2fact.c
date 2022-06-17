#include <stdio.h>

typedef unsigned long long int poly;
poly quo,quo_low,res,res_low;

#define MSB (~(~0ULL>>1))

int i,k;

poly seki(register poly a ,register poly b){

register poly  c=0;
  while(a!=0){
    if ((a & 1)==1){
	 c^=b;
    }
    b<<=1; a>>=1;
  }

  return c;
}


poly itob(register poly n){
register poly k=0;


  while(n>0){
    if(n%2==1){
      k=k+1;
}
    n=(n>>1);
}

  //#  printf("k=%d\n",k);

  return k;
}


/*
def i2b(n)
  x=n
  i=0;
  while(x>0)
    x=(x>>1); i=i+1;
  }
c=i


  while(c>0) 
    i=c-1
    if(n[i]==1)
      print "1";
    }
  
    if(n[i]==0)
      print "0";
    }
  c=c-1;
  }
#  print "\n";

}


def itob(n)

k=0;
  while(n>0)
    if(n%2==1)
      k=k+1;
    }
    n=(n>>1);
  }

  #  printf("k=%d\n",k);

  return k;
}
*/

#/* ���ӥå��������֤� */
int cb(register poly x){
int  i=0;

  while(x>0){
    x=(x>>1); i=i+1;
}

  return i;
}


#/* F_2 quot */
poly pd(register poly p,register poly d){

register poly t[64]={0};
register poly q,y,r,i;
 
  if(cb(p)<cb(d)){
    return p;
  }

// print "a"

  q=p; y=d;
  r=0;
  i=cb(q)-cb(y);

  y=(y<<i);

  if(y>p){
    y=(y>>1);
  }
  while(cb(q) >= cb(d)){
    y=d;
    i=cb(q)-cb(y);
    //print "i=",i,"\n"
    if(i>0){
      y=(y<<i);
    }

    if(i<0){
      break;
    }
    if(cb(q)==cb(y)){
      r=r + (1<<i);
      itob(q);
      itob(y);
      q=(q^y);
      itob(q);
      itob(y);
    }
  }

  return q;

}


/* F_2 mod */
poly pq(register poly p,register poly d){
register poly y,q,r,i;
register poly t[64]={0};

 
  if(cb(p)<cb(d)){
    return p;
  }


  q=p;
  y=d;
  r=0;
  i=cb(q)-cb(y);
  y=(y<<i);

  if(y>p){
    y=(y>>1);
  }

  while(cb(q) >= cb(d)){
    y=d;
    i=cb(q)-cb(y);

    if(i>0){
      y=(y<<i);
    }

    if(i<0){
      break;
    }
    if(cb(q)==cb(y)){
      r=r + (1<<i);
      itob(q);
      itob(y);
      q=(q^y);
      itob(q);
      itob(y);
    }
  }


  return r;

}

void write_poly(poly p,poly p_low)
{
	poly q;

	q=MSB;
	while (q>=p_low){
		printf((p & q) ? "1":"0");
		q>>=1;
	}
}

void divide(poly a,poly a_low,poly b,poly b_low)
{
int i;
	quo=0; quo_low=MSB; res=a; res_low=a_low;
	if(res_low>b_low) return;
	for(;;){
		if(res & MSB){
			quo|=quo_low; res^=b;
		}
		if(res_low==b_low) break;
		res_low<<=1; res<<=1; quo_low>>=1;
/* printf("%d ",i); */
	}
}

void factorize(poly p,poly p_low)
{
	poly d,d_low;

	d=MSB; d_low=MSB>>1;
	while(d_low>p_low){
		divide(p,p_low,d,d_low);
		if(res==0){	/* break; */
			write_poly(d,d_low); printf("*");
			p=quo; p_low=quo_low;
		}else{
			d +=d_low;
			if(d==0){
				d=MSB; d_low>>=1;
			}
		}
	}
	write_poly(p,p_low);
}


int main(){
	poly p,p_low;

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
	register poly a=0b11111111111111111111111111111111,b=0b10000000000000000000000000000011;
 // while(1)
  {
	a=seki(a,b);
	printf("%b\n",a);
  a+=2;
  b+=2;
  }
	return 0;
}



