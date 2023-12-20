/////////////////////////////////
///
///関数
/// 
//////////////////////////////////

// . メイン関数

fn main() {
    let mut process=0;

    reportProcess(process);
    process+=1;
    reportProcess(process);
    process+=10;
    reportProcess(process);
    process-=1;
    reportProcess(process);
    process-=5;
    reportProcess(process);
}

//報告する関数
fn reportProcess(percrnt:i32 ){
    println!("現在{:>3}%処理中", percrnt);
}
