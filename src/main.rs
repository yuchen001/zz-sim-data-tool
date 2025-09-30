use std::io::{self, Write};

fn prompt() {
    // 打印提示符并立刻刷新，否则可能缓存着不显示
    print!("zz> ");
    io::stdout().flush().expect("flush stdout failed");
}

fn main(){
    println!("祖宗模拟器数据处理 CLI 已启动");
    println!("输入 `help` 查看命令；输入 `exit`/`quit` 或按 Ctrl+D 退出。\n");

    loop {
        prompt();

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(0) => {
                // EOF
                break;
            }
            Ok(_) => {
                // 去掉结尾换行和空白
                let command = input.trim();
                if command.is_empty() {
                    continue;
                }

                match command {
                    "help" | "h" => {
                        println!("可用命令:");
                        println!("  help, h       显示此帮助信息");
                        println!("  exit, quit    退出程序");
                    }
                    "exit" | "quit" => {
                        break;
                    }
                    _ => {
                        println!("未知命令: '{}'. 输入 'help' 查看可用命令。", command);
                    }
                }
            }
            Err(error) => {
                eprintln!("读取输入失败: {error}");
                // 读取失败通常不致命，继续下一轮
                continue;
            }
        }
    }
}

