//use sha3::{Digest, Sha3_256};
use sha3::{Digest, Keccak256};
use std::{process::exit,str};
use hex_literal::hex;
use hmacsha::HmacSha;
use sha3::Sha3_256;


 
fn s2b(test:&str) -> &[u8]{
    //    let test: &str = "Test";
        let bytes: &[u8] = test.as_bytes();
    // convert bytes => str
    
    //println!("{}", test);
    println!("{:?}", bytes);
    
        bytes
    }
    
fn b2s(bytes:&[u8]) -> String{
    
        let res = bytes.iter().map(|&s| s as char).collect::<String>();
        let converted: String = String::from_utf8(bytes.to_vec()).unwrap();
        let mut be:String;
        //println!("{}",res);
     
        println!("{}", converted);
        
    //    let mut it:&str=&converted;
        //&it=&be;
    
        converted
    }
    
fn S2str(data: &String) -> &str {
        let v = &data[0..data.len()];
        //println!("{:?}",v);
    
        return v;
    }


fn pappy(a:[u8;256])-> [u8;256]{
    // create a SHA3-256 object
    let mut hasher = Keccak256::default();
    // write input message
    let mut count =0;
    
    hasher.update(a);
    let result = hasher.finalize();
    
    let mut buf:[u8;32]=[0;32];
    let mut u2:[u8;256]=[0;256];
    
    for i in 0..32{
        buf[i]=a[i];
    }
    for i in 0..8{
    let mut hasher = Keccak256::default();
    //me=hasher.clone();
    hasher.update(buf);
        // read hash digest
        let result = hasher.finalize();
    
    for i in 0..32{
        buf[i]^=result[i];
        u2[count]=buf[i];
        //print!("{},",result[i]);
        count=count+1;
    }
    //println!("");
    }
    
    u2
    
}

fn hmac(message:String,key:[u8;32])->Vec<u8>{
    let ipad:[u8;32]=[0x36;32];
    let opad:[u8;32]=[0x5c;32];
    let mut m:&[u8]=message.as_bytes();
    let mut hasher=Keccak256::default();
    let mut k1:[u8;32]=key;
    let mut k2:[u8;32]=key;
    for i in 0..32{
        k1[i]^=opad[i];
        k2[i]^=ipad[i];
    }
    let mut K1:String="".to_string();
    let mut K2:String="".to_string();
    let mut K3:String="".to_string();
    for i in 0..32{
    K1+=S2str(&k1[i].to_string());
    K2+=S2str(&k2[i].to_string());
    }
    K2+=S2str(&message);
    hasher.update(K2);
    let result2=hasher.finalize();
    let mut hasher=Keccak256::default();
    for i in 0..result2.len(){
        K1+=S2str(&result2[i].to_string());
    }
    hasher.update(K1);
    let result=hasher.finalize().to_vec();

    for i in 0..32{
    print!("{:0x}",result[i]);
    }
    println!("");

    result
}

fn main() {
    // create a SHAKE256 object
    //let mut hasher = Keccak256::default();
    // create a SHA3-256 object
    let mut hasher = Keccak256::new();
    let mut buf:[u8;256]=[0;256];
    // write input message
    let key:[u8;32]=[17;32];
    let msg:String="kotobahairanai".to_string();
    let str2:&str=&msg;
    let mc:Vec<u8>=hmac(msg,key);
    let mut bff:[u8;32]=[0;32];
    let mut str1:&str="";
    let mut str2:&str="";
    //let res = bytes.iter().map(|&s| s as char).collect::<String>();
    //let converted: String= String::from_utf8(mc).unwrap();
    let mut x:Vec<u8>;
    let aa=mc.len();
    for i in 0..aa{
        x[i+32]=mc[i];
    }
    println!("{:?}",mc);
    exit(1);



    hasher.update(b"kotobahairanai");

    // read hash digest
    let result = hasher.finalize();
    println!("{:0x}",result);
    let l=result.len();
    for i in 0..l{
        buf[i]=result[i];
    }
    let mut f:&[u8]=&buf;
    for i in 0..10{
    buf=pappy(buf);
    for j in 0..buf.len(){
    print!("{:0x}",&buf[j]);
    }
    println!("");
    }
}