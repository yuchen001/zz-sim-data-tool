use serde::{Deserialize, Serialize};

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
    pub fn size(&self) -> usize {
        1 + self.children.iter().map(|c| c.size()).sum::<usize>()
    }

    pub fn exists(&self, name: &str) -> bool {
        if self.name == name {
            return true;
        }
        
        self.children.iter().any(|c| c.exists(name))
    }

    pub fn show(&self, name: Option<&str>) {
        match name {
            None => {
                self.show_with_descendants(0);
            }
            Some(target) => {
                if let Some(p) = self.find(target) {
                    p.show_with_descendants(0);
                } else {
                    println!("未找到【{}】", target);
                }
            }
        }
    }

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

    fn show_with_descendants(&self, level: usize) {
        let indent = "  ".repeat(level);
        println!("{}{}", indent, self.summary());
        for child in &self.children {
            child.show_with_descendants(level + 1);
        }
    }

    fn find(&self, name: &str) -> Option<&FamilyMember> {
        if self.name == name {
            return Some(self);
        }
        self.children
          .iter()
          .find_map(|c| c.find(name))
    }
}