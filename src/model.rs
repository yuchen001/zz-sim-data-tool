use serde::{Deserialize, Serialize};

/// 家族成员节点
///
/// 每个成员包含基本信息（姓名、出生年、职位等），
/// 以及子女（`children`）。构成一棵多叉树。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FamilyMember {
    pub name: String,
    pub birth_year: u16,
    pub hoser_power_add: u8,

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
    ///
    /// # Example
    /// ```
    /// family.show(None);         // 显示整棵家族树
    /// family.show(Some("张三"));  // 仅显示张三支系
    /// ```
    pub fn show(&self, name: Option<&str>) {
        match name {
            None => {
                self.show_with_descendants(0);
            }
            Some(target) => {
                if let Some(p) = self.find_member_by_name(target) {
                    p.show_with_descendants(0);
                } else {
                    println!("未找到【{}】", target);
                }
            }
        }
    }

    /// 构造当前成员的概要信息（单行文本）。
    fn summary(&self) -> String {
        format!(
            "姓名:{:<6} | 出生年:{:>5} | 职位:{:<12} | 属性加成:{:>2} | 子嗣数:{:>2}",
            self.name,
            self.birth_year,
            self.position.as_deref().unwrap_or("-"),
            self.hoser_power_add,
            self.children.len()
        )
    }

    /// 按层级缩进打印成员及其所有子代。
    ///
    /// - `level`: 当前缩进层级（根节点为 0）
    fn show_with_descendants(&self, level: usize) {
        let indent = "  ".repeat(level);
        println!("{}{}", indent, self.summary());
        for child in &self.children {
            child.show_with_descendants(level + 1);
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
