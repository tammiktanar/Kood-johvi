use crate::boss::Boss;
use crate::member::{Member, Role};
pub mod boss;
pub mod member;


#[derive(Debug, Clone, PartialEq)]
pub struct Mob {
    pub name: String,
    pub boss: Boss,
    pub members: Vec<Member>,
    pub cities: Vec<(String, u8)>,
    pub wealth: u32,
}

impl Mob {
    pub fn recruit(&mut self, target: &str, age: u8) {
        self.members.push(Member{name: target.to_string(), age: age, role: Role::Associate})
    }

    pub fn attack(&mut self, target: &mut Mob) {
        let mut power = 0;
        let mut enemy_power = 0;

        for member in self.members.iter() {
            power += member.role.power()
        }

        for member in target.members.iter() {
            enemy_power += member.role.power()
        }

        if power <= enemy_power {
            self.members.pop();
            return;
        }

        target.members.pop();
        if target.members.is_empty() {
            self.steal(target, u32::MAX);
            self.cities.append(&mut target.cities);
        }
    }

    pub fn steal(&mut self, mob: &mut Mob, mut amount: u32) {
        if amount > mob.wealth {
            amount = mob.wealth
        }

        mob.wealth -= amount;
        self.wealth += amount;
    }

    pub fn conquer_city(&mut self, mobs: Vec<Mob>, city: String, amount: u8) {
        if mobs.into_iter().any(|mob| mob.cities.iter().any(|(city_name, _)| city_name == &city)) {
            return
        }

        self.cities.push((city, amount));
    }
}