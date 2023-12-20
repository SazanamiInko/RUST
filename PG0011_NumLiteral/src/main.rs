/////////////////////////////////
///
///数値リテラル
/// 
//////////////////////////////////

/// . メイン関数
fn main() {
   //数値をセット
   //2進数  
   let bval=0b11;
   //8進数
   let oval=0o11;
   //10進数
   let val=11;
   //16進数
   let xval=0x11;

   //表示
   println!("11は");
   println!("2進数では{}",bval);
   println!("8進数では{}",oval);
   println!("10進数では{}",val);
   println!("16進数では{}",xval);

    println!("です");
}