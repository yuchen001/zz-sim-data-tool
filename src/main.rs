mod model;
use model::FamilyMember;
use serde_json;
use std::io::{self, Write};
use std::{env, fs};

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

    add
      交互式为指定成员添加子嗣
      按提示输入父辈姓名与子嗣 JSON 数组

      JSON 格式示例:
      [{"name":"张小明","birth_year":2000,"hoser_power_add":5,"children":[]}]

    save
      将当前内存中的家族数据保存到 JSON 文件
      保存到环境变量 ZZ_SIM_FAMILY_DATA 指定的文件

提示:
  - 输入命令时不区分大小写
  - 输入 exit 或按 Ctrl+D 可以退出
===================================================="#;

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
    let data = fs::read_to_string(&data_file).expect("读取数据文件失败");
    let mut tree: FamilyMember = serde_json::from_str(&data).expect("解析数据失败");

    loop {
        print!("zz> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).unwrap_or(0) == 0 {
            // EOF (Ctrl+D)
            break;
        }

        let line = input.trim();
        if line.is_empty() {
            continue;
        }

        let mut parts = line.split_whitespace();
        let command = parts.next().unwrap().to_lowercase();
        let args: Vec<&str> = parts.collect();

        match command.as_str() {
            "help" => {
                println!("{HELP_TEXT}");
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

            "add" => {
                println!("📝 添加子嗣模式");

                // 1. 获取父节点
                let parent_name = loop {
                    print!("请输入成员姓名：");
                    io::stdout().flush().unwrap();

                    let mut input = String::new();
                    io::stdin().read_line(&mut input).ok();
                    let name = input.trim();

                    if name.is_empty() {
                        continue;
                    }

                    if tree.exists(name) {
                        break Some(name.to_string());
                    } else {
                        println!("【{name}】不存在，请重新输入");
                    }
                };

                let Some(parent) = parent_name else { continue };

                // 2. 获取 JSON array 插入子嗣
                println!("✅ 找到【{parent}】");
                print!("> ");
                io::stdout().flush().unwrap();

                let mut json_input = String::new();
                if io::stdin().read_line(&mut json_input).is_ok() {
                    tree.add_children(&parent, json_input.trim());
                }
            }

            "save" => {
                let json = serde_json::to_string_pretty(&tree).unwrap();
                if let Err(e) = fs::write(&data_file, json) {
                    eprintln!("❌ 保存失败: {}", e);
                }
            }

            _ => {
                println!("未知命令: '{line}'. 输入 'help' 查看可用命令。");
            }
        }
    }
}
