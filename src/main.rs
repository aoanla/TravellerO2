#![feature(let_chains)]
use rand::{Rng};
use std::collections::HashMap;


#[derive(Debug, Clone)]
struct DiePool {
    rng: rand::rngs::ThreadRng
}

impl DiePool {
    fn new() -> DiePool {
        DiePool{ rng: rand::thread_rng() }
    }

    fn d6(&self) -> i8 {
        (self.rng.gen::<f64>() * 6.0).ceil() as i8
    }

    fn twod6(&self) -> i8 {
        self.d6() + self.d6()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Stat {
    STR,
    DEX,
    END,
    INT,
    EDU,
    SOC,
    PSI
}


#[derive(Debug, Clone, PartialEq, Eq)]
enum Skill{
    BasicSkill{name: BasicSkill},
    SpecSkill{name: BasicSkill, spec: String},
}

impl Skill {
    fn new_basic(name: BasicSkill) -> Skill {
        Skill::BasicSkill{name}
    }

    fn new_spec(name: BasicSkill, spec: String) -> Option<Skill> {
        //enforce the list of specialisable strings
        if name as i32 > BasicSkill::LAST as i32 {
            None 
        } else {
            Some(Skill::SpecSkill{name, spec})
        }
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BasicSkill {
    Animals,
    Art,
    Athletics,
    Drive,
    Electronics,
    Engineer,
    Flyer,
    Gunner,
    GunCombat,
    HeavyWeapons,
    Language,
    Melee,
    Pilot,
    Profession,
    Science,
    Seafarer,
    Tactics,
    LAST,
    //skills without specialisms from here
    Admin,
    Advocate,
    Astrogation,
    Broker,
    Carouse,
    Deception,
    Diplomat,
    Explosives,
    Gambler,
    Investigate,
    JackOfAllTrades,
    Leadership,
    Mechanic,
    Medic, 
    Navigation,
    Persuade,
    Recon,
    Stealth,
    Steward,
    Streetwise,
    Survival,
    VaccSuit,
}


#[derive(Debug, Clone, PartialEq, Eq)]
enum Benefit {
    CASH(i32),
    Armour,
    Ally,
    Blade,
    CharInc(Stat),
    CyberImplant,
    Contact,
    FreeTrader,
    Gun,
    LabShip,
    PersonalVehicle,
    ScienceEquip,
    ScoutShip,
    ShipsBoat,
    ShipShares(i8),
    Weapon,
    TASMember,
    Yacht
}


struct Row {
    txt: String, 
    action: fn(&mut CharSheet)->()
}

struct Table<const N: usize>{
    offset: i8,
    rows: [Row; N],
}

trait Test {
    fn test(&self, charsheet: &CharSheet ) -> Effect;
}


type StatTest = (Stat, i8);

type Effect = i8; 


impl Test for StatTest {

    fn test(&self, charsheet: &CharSheet ) -> Effect {
        charsheet.get_stat_mod(self.0) + charsheet.twod6() - self.1 
    }
}

struct SkillsAndTraining {
    assignment: [Table<6>; 3],
    pd: Table<6>,
    adv: Table<6>,
    service: Table<6>,
    commissioned: Option<Table<6>>
}

struct Career {
    qualification: StatTest,
    assignment_names: [String; 3],
    survival: [StatTest; 3],
    advancement: [StatTest; 3],
    s_and_t: SkillsAndTraining,
    ranks: Table<6>,
    mishaps: Table<6>,
    events: Table<11>,
}


struct CharSheet {
    stats: [i8; 7],
    skills: HashMap<Skill>,
    career: Vec<Career>,
    cash: i32,
    benefits: Vec<Benefit>,
    diepool: DiePool,
    life_event: Table<12>,
    aging: Table<8>,
    injury: Table<6>,
    pension: Table<5>
}

impl CharSheet {

    fn new()->CharSheet {
        CharSheet {
            stats: [0;7],
            skills: ,
            career: Vec::new(),
            cash: 0,
            benefits: Vec::new(),
            diepool: DiePool::new(),
        }
    }

    fn get_stat(&self, stat: Stat) -> i8 {
        self.stats[stat as usize]
    }

    fn get_stat_mod(&self, stat: Stat) -> i8 {
        match self.stats[stat as usize] {
            0 => -3,
            s => s / 3 - 2
        }
    }

    fn d6(&self) -> i8 {
        self.diepool.d6()
    }

    fn twod6(&self) -> i8 {        
        self.diepool.twod6()
    }

    fn set_skill(&mut self, skill: Skill, val: i8) -> Result<(),Err> {
        match &skill {
            Skill::BasicSkill{name: s} => {
                //TODO: need to enforce the invariant that a basic skill with a specialist skill pairing cannot have val > 0
                // (instead you need to pick a specialism)
                let mut v = val;
                if *s as i32 >= BasicSkill::LAST as i32 && v > 0{
                    v = 0;
                    //throw error? call set_skill_interactive to get them to pick a specialism?
                }
                if let Some(vv) = self.skills.get_mut(skill) {
                    *vv = v;
                } else {
                    self.skills.insert(skill, v);
                };
            },
            Skill::SpecSkill{name: s, spec: sp} => {
                //TODO: also need to enforce invariant that SpecSkills never have value 0 - that just gives you the BasicSkill
                //need to enforce invariant that the basic skill must also exist if we add a "new specialist skill"
                if let Some(v) = self.skills.get_mut(skill) && val > 0 { 
                    *v = val; 
                } else {
                    let basic = Skill::BasicSkill{name: *s};
                    if self.skills.contains_key(basic) == false {
                        self.skills.insert(basic, 0)
                    }
                    if val > 0 {
                        self.skills.insert(skill, val);
                    }
                }
            }
        }
        Ok(())
    }

    fn inc_skill(&mut self, skill: Skill) -> Result<(),Err> {
        match &skill {
            Skill::BasicSkill{name: s} => {
                //TODO: need to enforce the invariant that a basic skill with a specialist skill pairing cannot have val > 0
                // (instead you need to pick a specialism)
                if *s as i32 >= BasicSkill::LAST as i32 {
                    if let Some(vv) = self.skills.get_mut(skill) { //must be == 0 if it has a specialism
                        if vv != 0 {
                            panic!("Basic var of special skill {:?} with nonzero positive skill value", skill)
                        }

                        let specialism = select_specialism_skill(&s, &mut self.skills); //launch prompt to pick a specialism to increase
                        self.inc_skill(specialism);

                    } else {
                        self.skills.insert(skill, 0); //add basic skill at level 0
                    }
                } else {
                    if let Some(vv) = self.skills.get_mut(skill) {
                        *vv += 1;
                    } else {
                        self.skills.insert(skill, 0);
                    }
                };
            },
            Skill::SpecSkill{name: s, spec: sp} => {
                //need to enforce invariant that the basic skill must also exist if we add a "new specialist skill"
                if let Some(v) = self.skills.get_mut(skill) { 
                    *v += 1; 
                } else {
                    let basic = Skill::BasicSkill{name: *s};
                    if self.skills.contains_key(basic) == false { //if we never had the skill, we get basic at 0
                        self.skills.insert(basic, 0)
                    } else { //we have basic skill but not the specialism, which we get at 1
                        self.skills.insert(skill, 1)
                    }
                }
            }
        }
        Ok(())
    }

    fn set_min_skill(&mut self, skill: Skill, val: i8) -> Result<(),Err> {
        match &skill {
            Skill::BasicSkill{name: s} => {
                //TODO: need to enforce the invariant that a basic skill with a specialist skill pairing cannot have val > 0
                // (instead you need to pick a specialism)
                if *s as i32 >= BasicSkill::LAST as i32 && val > 0 {
                    
                    //we interpret this as meaning "free selection for specialism skill"
                    let specialism = select_specialism_skill(&s, &mut self.skills);
                    self.set_min_skill(specialism, val);

                } else if let Some(vv) = self.skills.get_mut(skill) {
                    if vv < val {
                        *vv = val;
                    }
                } else {
                    self.skills.insert(skill, val);
                };
            },
            Skill::SpecSkill{name: s, spec: sp} => {
                //TODO: also need to enforce invariant that SpecSkills never have value 0 - that just gives you the BasicSkill
                //need to enforce invariant that the basic skill must also exist if we add a "new specialist skill"
                if let Some(vv) = self.skills.get_mut(skill) && val > 0 { 
                    if vv < val {
                        *vv = val;
                    }
                } else {
                    let basic = Skill::BasicSkill{name: *s};
                    if self.skills.contains_key(basic) == false {
                        self.skills.insert(basic, 0)
                    }
                    if val > 0 {
                        self.skills.insert(skill, val);
                    }
                }
            }
        }
        Ok(())
    }

}



fn main() {
    
}
