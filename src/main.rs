mod model;
use model::FamilyMember;
use serde_json;
use std::io::{self, Write};
use std::{env, fs, path::Path};

const HELP_TEXT: &str = r#"================== 祖宗模拟器帮助 ==================
命令列表:
    help
      显示此帮助信息

    exit | quit
      退出程序

    count
      显示家族成员总数（忽略已标记死亡者）

    exists <姓名>
      检查某个家族成员是否存在

    show [<姓名>]
      不带参数显示整个家族树，或展示指定成员的所有后代

    add
      交互式为指定成员添加子嗣，按提示粘贴 JSON 数组

      JSON 格式示例:
      [{"name":"张小明","birth_year":2000,"hoser_power_add":5,"children":[]}]

    save
      将当前内存中的家族数据保存到 ZZ_SIM_FAMILY_DATA 指定文件

    position <姓名> <职位>
      为成员设置职位称谓

    year [<年份>]
      不带参数时显示当前年份，带参数时更新年份状态

    stats
      统计信息占位命令，当前尚未实现

    path <姓名>
      显示家主到指定成员的路径

    prune
      删除当前年份之后出生的成员（需先设置 year，操作会二次确认）

    rename <旧名> <新名>
      重命名成员

    die <姓名>
      将成员标记为死亡

    clear
      清空终端显示

    inherit <姓名>
      在 archives/offspring_tree_<年份>.json 归档后，让成员继承家主。
      需先执行 year 设置年份，仅支持两代以内的继承人。

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
    let mut tree = serde_json::from_str::<FamilyMember>(&data).expect("解析数据失败");

    let mut current_year: Option<u16> = None;

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

            "position" => {
                if args.len() != 2 {
                    println!("用法: position <姓名> <职位>");
                    continue;
                }

                let name = args[0];
                let position = args[1];

                match tree.add_position(name, position) {
                    Ok(_) => println!("✅ 已为【{}】设置职位：{}", name, position),
                    Err(e) => eprintln!("❌ {}", e),
                }
            }

            "year" => {
                if args.is_empty() {
                    match current_year {
                        Some(y) => println!("当前年份：{}", y),
                        None => println!("⚠️  尚未设置当前年份"),
                    }
                } else {
                    match args[0].parse::<u16>() {
                        Ok(year) => {
                            current_year = Some(year);
                            println!("✅ 当前年份设置为 {}", year);
                        }
                        Err(_) => println!("❌ 无效的年份"),
                    }
                }
            }

            "stats" => {
                println!("统计功能待实现");
            }

            "path" => {
                if args.len() != 1 {
                    println!("用法: path <姓名>");
                } else {
                    tree.path(args[0]);
                }
            }

            "prune" => match current_year {
                None => {
                    println!("❌ 请先设置年份：year <年份>");
                }
                Some(year) => {
                    println!("⚠️  即将删除 {} 年后出生的所有成员（用于退档）", year);
                    print!("确认删除？(y/n): ");
                    io::stdout().flush().unwrap();

                    let mut confirm = String::new();
                    io::stdin().read_line(&mut confirm).ok();

                    match confirm.trim() {
                        "y" => tree.prune_future_births(year),

                        "n" => {
                            println!("❌ 已取消");
                        }

                        _ => {}
                    }
                }
            },

            "rename" => {
                if args.len() != 2 {
                    println!("用法：rename <旧名> <新名>");
                } else {
                    let old_name = args[0];
                    let new_name = args[1];
                    match tree.rename(old_name, new_name) {
                        Ok(_) => println!("✅ 已将【{}】改名为【{}】", old_name, new_name),
                        Err(e) => println!("❌ {}", e),
                    }
                }
            }

            "die" => {
                if args.len() != 1 {
                    println!("用法：die <姓名>");
                } else {
                    let name = args[0];
                    match tree.mark_dead(name) {
                        Ok(_) => println!("✅ 已将【{}】标记为死亡。", name),
                        Err(e) => println!("❌ {}", e),
                    }
                }
            }

            "clear" => {
                print!("\x1B[2J\x1B[1;1H");
                io::stdout().flush().unwrap();
            }

            "inherit" => {
                if args.len() != 1 {
                    println!("用法：inherit <姓名>");
                }

                let Some(year) = current_year else {
                    println!("❌ 请先执行 year <年份>");
                    continue;
                };

                // 确认
                print!("当前年份 {}，是否归档并继承？(y/n): ", year);
                io::stdout().flush().unwrap();

                let mut input = String::new();
                io::stdin().read_line(&mut input).ok();

                if input.trim().to_lowercase() != "y" {
                    println!("ℹ️ 已取消");
                    continue;
                }

                // 归档
                let archive_path = Path::new(&get_data_file())
                    .parent()
                    .unwrap_or(Path::new("."))
                    .join("archives")
                    .join(format!("offspring_tree_{}.json", year));
                if let Ok(json) = serde_json::to_string_pretty(&tree) {
                    fs::create_dir_all(archive_path.parent().unwrap()).ok();
                    if fs::write(&archive_path, json).is_ok() {
                        println!("🗃️ 已归档到 {}", archive_path.display());
                    }
                }

                // 继承
                let name = args[0];
                match tree.inherit(name) {
                    Ok(new_tree) => {
                        tree = new_tree;
                        println!("✅ 【{}】已继位", args[0]);
                    }
                    Err(e) => eprintln!("❌ {}", e),
                }
            }

            _ => {
                println!("未知命令: '{line}'. 输入 'help' 查看可用命令。");
            }
        }
    }
}
