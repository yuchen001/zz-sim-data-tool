mod model;
use model::FamilyMember;
use serde_json;
use std::{env, fs};
use std::io::{self, Write};

const HELP_TEXT: &str = r#"================== 祖宗模拟器帮助 ==================
命令列表:
    help
      显示此帮助信息

    exit | quit
      退出程序

    count
      显示家族成员总数

    exists <姓名>
      检查某个家族成员是否存在
      示例: exists 张三

    show [<姓名>]
      显示家族成员信息
      如果不带参数，显示整个家族树
      如果带参数，显示该成员及其所有后代
      示例: show
            show 李四

提示:
  - 输入命令时不区分大小写
  - 输入 exit 或按 Ctrl+D 可以退出
===================================================="#;

fn prompt() {
    // 打印提示符并立刻刷新，否则可能缓存着不显示
    print!("zz> ");
    io::stdout().flush().expect("flush stdout failed");
}

fn get_data_file() -> String {
    match env::var("ZZ_SIM_FAMILY_DATA") {
        Ok(path) => path,
        Err(_) => panic!("❌ 环境变量 ZZ_SIM_FAMILY_DATA 未设置，请使用 export ZZ_SIM_FAMILY_DATA=/path/to/offspring_tree.json"),
    }
}

fn main() {
    println!("祖宗模拟器数据处理 CLI 已启动");
    println!("输入 `help` 查看命令；输入 `exit`/`quit` 或按 Ctrl+D 退出。\n");

    let data_file = get_data_file();
    let data = fs::read_to_string(data_file).expect("读取数据文件失败");
    let tree: FamilyMember = serde_json::from_str(&data).expect("解析数据失败");

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
                let line = input.trim();
                if line.is_empty() {
                    continue;
                }

                let mut parts = line.split_whitespace();
                let command = parts.next().unwrap().to_lowercase();
                let args: Vec<&str> = parts.collect();

                match command.as_str() {
                    "help" => {
                        println!("{HELP_TEXT}" );
                    }
                    "exit" | "quit" => {
                        break;
                    }

                    "count" => {
                        println!("总共的家族人数：{}.", tree.size())
                    }

                    "exists" => {
                        if args.len() != 1 {
                            println!("用法: exists <name>");
                        } else {
                            let name = args[0];
                            if tree.exists(name) {
                                println!("【{name}】存在于家族中。");
                            } else {
                                println!("【{name}】不存在于家族中。");
                            }
                        }
                    }

                    "show" => {
                        if args.len() > 1 {
                            println!("用法: show [<name>]");
                        } else if args.len() == 1 {
                            let name = args[0];
                            tree.show(Some(name));
                        } else {
                            tree.show(None);
                        }
                    }

                    _ => {
                        println!("未知命令: '{line}'. 输入 'help' 查看可用命令。");
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
