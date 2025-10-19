use serde::{Deserialize, Serialize};
use unicode_width::UnicodeWidthStr;

/// 家族成员节点
///
/// 每个成员包含基本信息（姓名、出生年、职位等），
/// 以及子女（`children`）。构成一棵多叉树。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FamilyMember {
    pub name: String,
    pub birth_year: u16,
    pub hoser_power_add: u8,
    pub member_type: String,

    #[serde(default)]
    pub position: Option<String>,
    #[serde(default)]
    pub children: Vec<FamilyMember>,
}

impl FamilyMember {
    /// 计算以当前成员为根的家族树规模（包含所有子孙）。
    ///
    /// # Returns
    /// 总成员数量（包括自己）。
    pub fn size(&self) -> usize {
        1 + self.children.iter().map(|c| c.size()).sum::<usize>()
    }

    pub fn exists(&self, name: &str) -> bool {
        if self.name == name {
            return true;
        }

        self.children.iter().any(|c| c.exists(name))
    }

    /// 打印家族树。
    ///
    /// - 若 `name` 为 `None`，则显示以当前成员为根的整棵家族树。
    /// - 若指定 `name`，则仅显示该成员及其子孙。
    pub fn show(&self, name: Option<&str>) {
        const TREE_COLUMN_WIDTH: usize = 30; // 树形符号+姓名的总宽度
        const BIRTH_WIDTH: usize = 8;
        const TYPE_WIDTH: usize = 12;
        const POSITION_WIDTH: usize = 18;
        const ATTR_WIDTH: usize = 8;
        const CHILD_WIDTH: usize = 8;

        let border = "━".repeat(80);

        println!("{border}");

        // 表头 - 手动填充每一列
        let header_name = format!(
            "{}{}",
            "姓名",
            " ".repeat(TREE_COLUMN_WIDTH.saturating_sub("姓名".width()))
        );
        let header_birth = format!(
            "{}{}",
            "出生",
            " ".repeat(BIRTH_WIDTH.saturating_sub("出生".width()))
        );
        let header_type = format!(
            "{}{}",
            "类别",
            " ".repeat(TYPE_WIDTH.saturating_sub("类别".width()))
        );
        let header_position = format!(
            "{}{}",
            "职位",
            " ".repeat(POSITION_WIDTH.saturating_sub("职位".width()))
        );
        let header_attr = format!(
            "{}{}",
            "威望+",
            " ".repeat(ATTR_WIDTH.saturating_sub("威望+".width()))
        );
        let header_child = format!(
            "{}{}",
            "子嗣",
            " ".repeat(CHILD_WIDTH.saturating_sub("子嗣".width()))
        );

        println!(
            "{}{}{}{}{}{}",
            header_name, header_birth, header_type, header_position, header_attr, header_child
        );

        println!("{border}");

        match name {
            None => self.show_with_descendants(0),
            Some(target) => {
                if let Some(p) = self.find_member_by_name(target) {
                    p.show_with_descendants(0);
                } else {
                    println!("未找到【{}】", target);
                }
            }
        }

        println!(); // 空行结尾
    }

    /// 批量添加子嗣到指定成员
    ///
    /// # 参数
    /// * `parent_name` - 父辈成员的姓名
    /// * `child_json` - 子嗣信息的 JSON 数组字符串
    ///
    /// # 返回值
    /// * `Ok(usize)` - 成功添加的子嗣数量
    /// * `Err(String)` - 错误信息（格式错误或重名）
    ///
    /// # 事务性保证
    /// 采用"全部成功或全部失败"策略：
    /// - 若任一子嗣重名，则所有子嗣都不会被添加
    /// - 只有全部通过检查后才会执行添加操作
    pub fn add_children(&mut self, parent_name: &str, child_json: &str) {
        let Ok(children_vec) = serde_json::from_str::<Vec<FamilyMember>>(child_json) else {
            eprintln!("添加的子代格式不正确。");
            return;
        };

        // 提前检查，保证一次添加原子化
        for node in &children_vec {
            if self.exists(&node.name) {
                println!("【{}】在当前家族树中重名，请重新命名。", node.name);
                return;
            }
        }

        for node in &children_vec {
            self.add_child_entity(parent_name, node)
        }
    }

    /// 递归查找并添加单个子节点到指定父节点
    fn add_child_entity(&mut self, parent_name: &str, child: &FamilyMember) {
        if self.name == parent_name {
            self.children.push(child.to_owned());
            return;
        }

        for node in self.children.iter_mut() {
            node.add_child_entity(parent_name, child);
        }
    }

    /// 按树形结构打印成员及其所有子代
    fn show_with_descendants(&self, level: usize) {
        // 根节点调用辅助方法，不使用树形符号
        self.show_with_descendants_helper(level, true, Vec::new());
    }

    /// 辅助方法：递归打印家族树，支持树形分支符号
    ///
    /// # 参数
    /// * `level` - 当前层级（0为根节点）
    /// * `is_last` - 当前节点是否是父节点的最后一个子节点
    /// * `parent_markers` - 记录每一层的父节点是否是最后一个（用于决定是否画竖线）
    fn show_with_descendants_helper(&self, level: usize, is_last: bool, parent_markers: Vec<bool>) {
        const TREE_COLUMN_WIDTH: usize = 30; // 树形符号+姓名的总宽度
        const BIRTH_WIDTH: usize = 8;
        const TYPE_WIDTH: usize = 12;
        const POSITION_WIDTH: usize = 18;
        const ATTR_WIDTH: usize = 8;
        const CHILD_WIDTH: usize = 8;

        // 构建树形前缀
        let mut tree_prefix = String::new();

        // 为每一层父节点添加竖线或空格
        for &parent_is_last in &parent_markers {
            if parent_is_last {
                tree_prefix.push_str("   "); // 父节点是最后一个，不画竖线
            } else {
                tree_prefix.push_str("│  "); // 父节点不是最后一个，画竖线
            }
        }

        // 当前节点的分支符号（根节点除外）
        let branch_symbol = if level > 0 {
            if is_last {
                "└─ " // 最后一个子节点
            } else {
                "├─ " // 中间子节点
            }
        } else {
            "" // 根节点无符号
        };

        tree_prefix.push_str(branch_symbol);

        // 组合树形前缀和姓名
        let name_with_tree = format!("{}{}", tree_prefix, self.name);

        // 填充到固定总宽度
        let total_display_width = name_with_tree.width();
        let padding = TREE_COLUMN_WIDTH.saturating_sub(total_display_width);
        let name_column = format!("{}{}", name_with_tree, " ".repeat(padding));

        // 出生年 - 手动填充
        let birth_str = self.birth_year.to_string();
        let birth_padding = BIRTH_WIDTH.saturating_sub(birth_str.width());
        let birth_padded = format!("{}{}", birth_str, " ".repeat(birth_padding));

        // 类别 - 手动填充
        let type_padding = TYPE_WIDTH.saturating_sub(self.member_type.width());
        let type_padded = format!("{}{}", self.member_type, " ".repeat(type_padding));

        // 职位 - 手动填充
        let position_str = self.position.as_deref().unwrap_or("-");
        let position_padding = POSITION_WIDTH.saturating_sub(position_str.width());
        let position_padded = format!("{}{}", position_str, " ".repeat(position_padding));

        // 属性+ - 手动填充
        let attr_str = self.hoser_power_add.to_string();
        let attr_padding = ATTR_WIDTH.saturating_sub(attr_str.width());
        let attr_padded = format!("{}{}", attr_str, " ".repeat(attr_padding));

        // 子嗣 - 手动填充
        let child_str = self.children.len().to_string();
        let child_padding = CHILD_WIDTH.saturating_sub(child_str.width());
        let child_padded = format!("{}{}", child_str, " ".repeat(child_padding));

        // 直接拼接输出
        println!(
            "{}{}{}{}{}{}",
            name_column, birth_padded, type_padded, position_padded, attr_padded, child_padded
        );

        // 递归处理子节点
        let child_count = self.children.len();
        for (index, child) in self.children.iter().enumerate() {
            let child_is_last = index == child_count - 1;

            // 更新 parent_markers：添加当前节点的状态
            let mut new_markers = parent_markers.clone();
            new_markers.push(is_last);

            child.show_with_descendants_helper(level + 1, child_is_last, new_markers);
        }
    }

    /// 在当前家族树中递归查找指定姓名的成员。
    ///
    /// # Returns
    /// 若找到则返回 `Some(&FamilyMember)`，否则返回 `None`。
    fn find_member_by_name(&self, name: &str) -> Option<&FamilyMember> {
        if self.name == name {
            return Some(self);
        }
        self.children
            .iter()
            .find_map(|c| c.find_member_by_name(name))
    }
}
