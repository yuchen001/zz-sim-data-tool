use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use unicode_width::UnicodeWidthStr;

// ============================================================================
// Type Definitions
// ============================================================================

/// 家族成员节点
///
/// 每个成员包含基本信息（姓名、出生年、职位等），
/// 以及子女（`children`）。构成一棵多叉树。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FamilyMember {
    pub name: String,
    pub birth_year: u16,
    pub hoser_power_add: u8,
    pub member_type: MemberType,

    #[serde(default)]
    pub position: Option<String>,
    #[serde(default)]
    pub children: Vec<FamilyMember>,

    #[serde(default)]
    pub is_dead: bool,
}

/// 代际关系枚举
///
/// 表示家族成员与家主的代际距离，从家主（0代）到耳孙（9代）。
/// 使用 `#[repr(u8)]` 以支持代际升降计算。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
#[repr(u8)]
pub(crate) enum Generation {
    家主 = 0,
    儿 = 1,
    孙 = 2,
    曾孙 = 3,
    玄孙 = 4,
    来孙 = 5,
    晜孙 = 6,
    仍孙 = 7,
    云孙 = 8,
    耳孙 = 9,
    其他,
}

/// 性别枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Gender {
    Male,
    Female,
}

/// 血统枚举
///
/// 区分内系（直系血亲）和外系（通过女儿延续的血脉）
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Lineage {
    Direct,  // 内系
    Foreign, // 外系
}

/// 成员类型
///
/// 组合代际、性别、血统三个维度，用于生成成员称谓（如"孙女"、"外曾孙"等）
#[derive(Debug, Clone, Copy)]
pub struct MemberType {
    pub generation: Generation,
    pub gender: Gender,
    pub lineage: Lineage,
}

// ============================================================================
// Trait Implementations
// ============================================================================

impl Serialize for MemberType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for MemberType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        s.parse().map_err(serde::de::Error::custom)
    }
}

impl From<Generation> for u8 {
    fn from(gen: Generation) -> Self {
        gen as u8
    }
}

impl FromStr for MemberType {
    type Err = String;

    /// 从称谓字符串解析成员类型
    ///
    /// 如 "孙女" -> (孙, Female, Direct), "外曾孙" -> (曾孙, Male, Foreign)
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let generation = if s == "家主" {
            Generation::家主
        } else if s.contains("儿") {
            Generation::儿
        } else if s.contains("来孙") {
            Generation::来孙
        } else if s.contains("玄孙") {
            Generation::玄孙
        } else if s.contains("曾孙") {
            Generation::曾孙
        } else if s.contains("晜孙") {
            Generation::晜孙
        } else if s.contains("仍孙") {
            Generation::仍孙
        } else if s.contains("云孙") {
            Generation::云孙
        } else if s.contains("耳孙") {
            Generation::耳孙
        } else if s.contains("孙") {
            Generation::孙
        } else {
            Generation::其他
        };

        let gender = if s.contains('女') {
            Gender::Female
        } else {
            Gender::Male
        };

        let lineage = if s.contains('外') {
            Lineage::Foreign
        } else {
            Lineage::Direct
        };

        Ok(MemberType {
            generation,
            gender,
            lineage,
        })
    }
}

impl fmt::Display for MemberType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Gender::*;
        use Generation::*;
        use Lineage::*;

        let s = match (self.generation, self.gender, self.lineage) {
            (家主, _, _) => "家主",

            (儿, Male, _) => "儿",
            (儿, Female, _) => "女儿",

            (孙, Male, Direct) => "孙",
            (孙, Female, Direct) => "孙女",
            (孙, Male, Foreign) => "外孙",
            (孙, Female, Foreign) => "外孙女",

            (曾孙, Male, Direct) => "曾孙",
            (曾孙, Female, Direct) => "曾孙女",
            (曾孙, Male, Foreign) => "外曾孙",
            (曾孙, Female, Foreign) => "外曾孙女",

            (玄孙, Male, Direct) => "玄孙",
            (玄孙, Female, Direct) => "玄孙女",
            (玄孙, Male, Foreign) => "外玄孙",
            (玄孙, Female, Foreign) => "外玄孙女",

            (来孙, Male, Direct) => "来孙",
            (来孙, Female, Direct) => "来孙女",
            (来孙, Male, Foreign) => "外来孙",
            (来孙, Female, Foreign) => "外来孙女",

            (晜孙, Male, Direct) => "晜孙",
            (晜孙, Female, Direct) => "晜孙女",
            (晜孙, Male, Foreign) => "外晜孙",
            (晜孙, Female, Foreign) => "外晜孙女",

            (仍孙, Male, Direct) => "仍孙",
            (仍孙, Female, Direct) => "仍孙女",
            (仍孙, Male, Foreign) => "外仍孙",
            (仍孙, Female, Foreign) => "外仍孙女",

            (云孙, Male, Direct) => "云孙",
            (云孙, Female, Direct) => "云孙女",
            (云孙, Male, Foreign) => "外云孙",
            (云孙, Female, Foreign) => "外云孙女",

            (耳孙, Male, Direct) => "耳孙",
            (耳孙, Female, Direct) => "耳孙女",
            (耳孙, Male, Foreign) => "外耳孙",
            (耳孙, Female, Foreign) => "外耳孙女",

            (其他, _, _) => "未知",
        };

        write!(f, "{}", s)
    }
}

// ============================================================================
// Method Implementations
// ============================================================================

impl FamilyMember {
    // 表格列宽常量
    const TREE_COLUMN_WIDTH: usize = 30; // 树形符号+姓名的总宽度
    const BIRTH_WIDTH: usize = 8;
    const TYPE_WIDTH: usize = 12;
    const STATUS_WIDTH: usize = 8;
    const POSITION_WIDTH: usize = 18;
    const ATTR_WIDTH: usize = 8;
    const CHILD_WIDTH: usize = 8;

    /// 计算以当前成员为根的家族树规模（包含所有子孙）。
    ///
    /// # Returns
    /// 总成员数量（包括自己）。
    pub fn size(&self) -> usize {
        1 + self
            .children
            .iter()
            .filter(|c| !c.is_dead)
            .map(|c| c.size())
            .sum::<usize>()
    }

    /// 检查指定姓名的成员是否存在
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
        let border = "━".repeat(80);

        println!("{border}");

        // 表头 - 手动填充每一列
        let header_name = format!(
            "{}{}",
            "姓名",
            " ".repeat(Self::TREE_COLUMN_WIDTH.saturating_sub("姓名".width()))
        );
        let header_birth = format!(
            "{}{}",
            "出生",
            " ".repeat(Self::BIRTH_WIDTH.saturating_sub("出生".width()))
        );
        let header_type = format!(
            "{}{}",
            "类别",
            " ".repeat(Self::TYPE_WIDTH.saturating_sub("类别".width()))
        );
        let header_status = format!(
            "{}{}",
            "状态",
            " ".repeat(Self::STATUS_WIDTH.saturating_sub("状态".width()))
        );
        let header_position = format!(
            "{}{}",
            "职位",
            " ".repeat(Self::POSITION_WIDTH.saturating_sub("职位".width()))
        );
        let header_attr = format!(
            "{}{}",
            "威望+",
            " ".repeat(Self::ATTR_WIDTH.saturating_sub("威望+".width()))
        );
        let header_child = format!(
            "{}{}",
            "子嗣",
            " ".repeat(Self::CHILD_WIDTH.saturating_sub("子嗣".width()))
        );

        println!(
            "{}{}{}{}{}{}{}",
            header_name,
            header_birth,
            header_type,
            header_status,
            header_position,
            header_attr,
            header_child
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

    /// 添加子嗣
    ///
    /// 需要指定是谁的子嗣，可以一次添加多个。
    /// 并且实现了事务保证。
    ///
    /// # param
    /// * `parent_name` - 父辈成员的姓名
    /// * `child_json` - 子嗣信息的 JSON 数组字符串
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

    /// 添加职位
    ///
    /// # param
    /// - name: 姓名
    /// - position: 职位
    pub fn add_position(&mut self, name: &str, position: &str) -> Result<(), String> {
        self.find_member_by_name_mut(name)
            .map(|member| member.position = Some(position.to_string()))
            .ok_or_else(|| format!("未找到成员【{}】", name))
    }

    /// 显示从根到指定成员的路径
    pub fn path(&self, name: &str) {
        let mut path = Vec::new();

        if self.find_path_recursive(name, &mut path) {
            let names: Vec<&str> = path.iter().map(|m| m.name.as_str()).collect();
            println!("{}", names.join(" → "));
        } else {
            println!("❌ 未找到【{}】", name);
        }
    }

    /// 清理未来出生的成员
    ///
    /// 用于处理读档后，删除当前年份之后出生的成员（通常因回档导致）
    pub fn prune_future_births(&mut self, year: u16) {
        self.children.retain(|child| child.birth_year <= year);

        for item in &mut self.children {
            item.prune_future_births(year)
        }
    }

    /// 重命名成员
    ///
    /// 确保新名称在家族树中不重复
    pub fn rename(&mut self, old_name: &str, new_name: &str) -> Result<(), String> {
        if self.exists(new_name) {
            return Err(format!("⚠️ 名称【{}】已存在，无法重命名。", new_name));
        }

        if let Some(member) = self.find_member_by_name_mut(old_name) {
            member.name = new_name.to_string();
            Ok(())
        } else {
            Err(format!("未找到成员【{}】", old_name))
        }
    }

    /// 标记成员死亡
    ///
    /// 死亡成员不再计入家族规模统计
    pub fn mark_dead(&mut self, name: &str) -> Result<(), String> {
        if let Some(member) = self.find_member_by_name_mut(name) {
            if member.is_dead {
                return Err(format!("⚠️ 成员【{}】已被标记为死亡。", name));
            }

            member.is_dead = true;
            Ok(())
        } else {
            Err(format!("未找到成员【{}】", name))
        }
    }

    /// 继承家主位
    ///
    /// 将指定成员提升为新家主，并自动调整其后代的代际关系
    pub fn inherit(&self, name: &str) -> Result<FamilyMember, String> {
        let successor = self
            .find_member_by_name(name)
            .ok_or_else(|| format!("找不到【{}】", name))?;

        let generation = successor.member_type.generation;
        if generation > Generation::孙 {
            return Err(format!(
                "只能两代以内的成员继承家主. 当前的【{}】位于第{}代",
                name,
                u8::from(generation)
            ));
        }

        let levels = u8::from(successor.member_type.generation);

        let mut new_head = successor.clone();
        let head_gender = new_head.member_type.gender;
        new_head.member_type = MemberType {
            generation: Generation::家主,
            gender: new_head.member_type.gender,
            lineage: new_head.member_type.lineage,
        };

        for child in new_head.children.iter_mut() {
            child.promote_descendants(levels);
        }

        if matches!(head_gender, Gender::Female) {
            for child in new_head.children.iter_mut() {
                child.member_type.lineage = Lineage::Direct;

                if matches!(child.member_type.gender, Gender::Male) {
                    child.set_lineage_for_descendants(Lineage::Direct);
                }
            }
        }

        Ok(new_head)
    }

    // ------------------------------------------------------------------------
    // 私有辅助方法 (Private Helper Methods)
    // ------------------------------------------------------------------------

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

    /// 递归打印家族树，支持树形分支符号
    ///
    /// # param
    /// * `level` - 当前层级（0为根节点）
    /// * `is_last` - 当前节点是否是父节点的最后一个子节点
    /// * `parent_markers` - 记录每一层的父节点是否是最后一个（用于决定是否画竖线）
    fn show_with_descendants_helper(&self, level: usize, is_last: bool, parent_markers: Vec<bool>) {
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
        let padding = Self::TREE_COLUMN_WIDTH.saturating_sub(total_display_width);
        let name_column = format!("{}{}", name_with_tree, " ".repeat(padding));

        // 出生年 - 手动填充
        let birth_str = self.birth_year.to_string();
        let birth_padding = Self::BIRTH_WIDTH.saturating_sub(birth_str.width());
        let birth_padded = format!("{}{}", birth_str, " ".repeat(birth_padding));

        // 类别 - 手动填充
        let type_padding = Self::TYPE_WIDTH.saturating_sub(self.member_type.to_string().width());
        let type_padded = format!("{}{}", self.member_type, " ".repeat(type_padding));

        // 状态 - 手动填充
        let status_str = if self.is_dead { "已故" } else { "" };
        let status_padding = Self::STATUS_WIDTH.saturating_sub(status_str.width());
        let status_padded = format!("{}{}", status_str, " ".repeat(status_padding));

        // 职位 - 手动填充
        let position_str = self.position.as_deref().unwrap_or("-");
        let position_padding = Self::POSITION_WIDTH.saturating_sub(position_str.width());
        let position_padded = format!("{}{}", position_str, " ".repeat(position_padding));

        // 属性+ - 手动填充
        let attr_str = self.hoser_power_add.to_string();
        let attr_padding = Self::ATTR_WIDTH.saturating_sub(attr_str.width());
        let attr_padded = format!("{}{}", attr_str, " ".repeat(attr_padding));

        // 子嗣 - 手动填充
        let child_str = self.children.len().to_string();
        let child_padding = Self::CHILD_WIDTH.saturating_sub(child_str.width());
        let child_padded = format!("{}{}", child_str, " ".repeat(child_padding));

        // 直接拼接输出
        println!(
            "{}{}{}{}{}{}{}",
            name_column,
            birth_padded,
            type_padded,
            status_padded,
            position_padded,
            attr_padded,
            child_padded
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

    /// 在当前家族树中递归查找指定姓名的成员（可变引用版本）。
    ///
    /// # Returns
    /// 若找到则返回 `Some(&mut FamilyMember)`，否则返回 `None`。
    fn find_member_by_name_mut(&mut self, name: &str) -> Option<&mut FamilyMember> {
        if self.name == name {
            return Some(self);
        }
        self.children
            .iter_mut()
            .find_map(|c| c.find_member_by_name_mut(name))
    }

    /// 递归查找路径（回溯法）
    fn find_path_recursive<'a>(
        &'a self,
        target_name: &str,
        path: &mut Vec<&'a FamilyMember>,
    ) -> bool {
        path.push(self);

        if self.name == target_name {
            return true;
        }

        for child in &self.children {
            if child.find_path_recursive(target_name, path) {
                return true;
            }
        }

        path.pop();
        false
    }

    /// 递归提升后代的代际
    ///
    /// 在继承时调用，将所有子孙的代际向上提升指定层级
    fn promote_descendants(&mut self, levels: u8) {
        self.member_type.generation = self.member_type.generation.promote(levels);
        for child in self.children.iter_mut() {
            child.promote_descendants(levels);
        }
    }

    /// 递归设置所有后代的血统
    fn set_lineage_for_descendants(&mut self, lineage: Lineage) {
        for child in self.children.iter_mut() {
            child.member_type.lineage = lineage;
            child.set_lineage_for_descendants(lineage);
        }
    }
}

impl Generation {
    /// 从数值转换为代际
    fn from_u8(n: u8) -> Self {
        match n {
            0 => Self::家主,
            1 => Self::儿,
            2 => Generation::孙,
            3 => Generation::曾孙,
            4 => Generation::玄孙,
            5 => Generation::来孙,
            6 => Generation::晜孙,
            7 => Generation::仍孙,
            8 => Generation::云孙,
            9 => Generation::耳孙,
            _ => Generation::其他,
        }
    }

    /// 代际提升
    ///
    /// 将当前代际向上提升指定层级（数值减少）
    pub fn promote(self, levels: u8) -> Self {
        let current: u8 = self.into();
        let new_level = current.saturating_sub(levels);
        Self::from_u8(new_level)
    }
}
