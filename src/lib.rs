mod core;
mod error;
mod lang;
// pub use core::lexer;
const LANGUAGE: lang::Language = lang::Language::Chinese;
pub fn test() {
    println!(" 语法分析测试");
    println!("\n成功: ");
    let src = "pg { set awasome}";
    println!("{:?}", core::lexer(src, &mut core::Pos::new()));

    println!("\n失败: ");
    let src = "pg { / }";
    println!("{:?}", core::lexer(src, &mut core::Pos::new()));
    println!("\n");
}
pub fn r#loop() {
    println!("语法分析尚未完成...\n输入一行代码,输出语法分析结果!\nCtrl+C退出");
    loop {
        let mut input = String::new();
        match std::io::stdin().read_line(&mut input) {
            Ok(_) => println!("{:?}", core::lexer(&input, &mut core::Pos::new())),
            Err(_) => todo!("终止"),
        }
    }
}
