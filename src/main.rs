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


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

    fn select_specialism_skill(&self, skills: &HashMap<Skill, i8>) -> Option<Self> {
        match self {
            Skill::BasicSkill{name} => {
                if (*name as i32) < BasicSkill::LAST as i32 {
                    let spec_template = Skill::SpecSkill{name: *name, spec: "".to_string()};

                    //can be specialised, so find existing specialisms
                    let specs: Vec<_> = skills.keys().filter(|k| {
                        match k {
                            Skill::BasicSkill{..} => false, 
                            Skill::SpecSkill{name:kn, spec:_kspec} => *kn == *name
                        }
                    }).collect();

                    print!("Please Select a specialism for {:?}", name);
                    print!("Existing specialisms in skill set:");
                    specs.iter().enumerate().for_each(|(i,s)| {
                        if let Skill::SpecSkill{name:_, spec} = s { print!("{}:{}", i, spec)} });
                    print!("{}: new specialism (will be prompted for)", specs.len());
                    //read value
                    let num = 999; //TODO!!!!
                    
                    if num < specs.len() {
                        Some(*specs[num])
                    } else {
                        print!("Enter specialism name:");
                        //read value {ideally we have a hint set of "standard values" here for the menu}
                        let specialism = "";

                        Some(Skill::SpecSkill{name:*name, spec: specialism.to_string()})
                    }
                } else {
                    None //no specialisms exist!
                }

            }, 
            //what should we do if called with an already specialised skill? Return None?
            Skill::SpecSkill{name, spec} => {
                None
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

//This is too general - Skill/Training tables at least should be specialisable as they only have results that increase skills or stats
//admittedly *all the other tables* are v general!
#[derive(Debug, PartialEq, Clone)]
struct Row {
    txt: String, 
    action: fn(&mut CharSheet)->()
}

#[derive(Debug, PartialEq, Clone)]
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

type SkillTest = (Skill, i8);

impl Test for SkillTest {
    fn test(&self, charsheet: &CharSheet) -> Effect {
        charsheet.get_skill(self.0) + charsheet.twod6() - self.1 
    }

}

#[derive(Debug, Clone, PartialEq)]
struct SkillsAndTraining {
    assignment: [Table<6>; 3],
    pd: Table<6>,
    adv: Table<6>,
    service: Table<6>,
    commissioned: Option<Table<6>>
}


enum Career {
    Agent,
    Army, 
    Citizen,
    Drifter,
    Entertainer,
    Marine,
    Merchant,
    Navy,
    Noble,
    Rogue,
    Scholar,
    Scout,
    LAST, //maybe we need a sentinel here?
    Psionic,
    PreUniversity,
    PreMilitary
}

//not sure if I need this - but there's only ever three choices, so I can enforce this way or via checks in CareerPage...
enum CareerAssign {
    One,
    Two,
    Three
}

//this is a Career we can roll on, but is this what we should store in the CharSheet, or should that have properties
#[derive(Debug, Clone, PartialEq)]
struct CareerPage {
    qualification: StatTest,
    assignment_names: [String; 3],
    survival: [StatTest; 3],
    advancement: [StatTest; 3],
    s_and_t: SkillsAndTraining,
    ranks: Table<6>,
    mishaps: Table<6>,
    events: Table<11>,
}

struct CareerElem {
    career: Career,
    assignment: CareerAssign, 
    commissioned: bool,
    rank: u8,

}


struct CharSheet {
    stats: [i8; 7],
    skills: HashMap<Skill, i8>,
    career: Vec<CareerElem>,
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


    fn get_skill(&self , skill: Skill) -> i8 {
        match &skill {
            Skill::BasicSkill{name: s} => {
                if let Some(vv) = self.skills.get(&skill) {
                    *vv
                } else if let Some(joat) = self.skills.get(&Skill::BasicSkill{name:BasicSkill::JackOfAllTrades}) {
                    *joat - 3
                } else {
                    -3
                }
            },
            Skill::SpecSkill{name: s, spec: sp} => {
                //if we don't have the advanced skill, we fall back on the basic one
                if let Some(v) = self.skills.get(&skill) { 
                    *v
                } else {
                    let basic = Skill::BasicSkill{name:*s};
                    self.get_skill(basic)
                }
            }
        }

    }

    fn set_skill(&mut self, skill: Skill, val: i8) -> Result<(),Err> {
        match &skill {
            Skill::BasicSkill{name: s} => {
                //TODO: need to enforce the invariant that a basic skill with a specialist skill pairing cannot have val > 0
                // (instead you need to pick a specialism)
                let mut v = val;
                if (*s as i32) < BasicSkill::LAST as i32 && v > 0{
                    v = 0;
                    //throw error? call set_skill_interactive to get them to pick a specialism?
                }
                if let Some(vv) = self.skills.get_mut(&skill) {
                    *vv = v;
                } else {
                    self.skills.insert(skill, v);
                };
            },
            Skill::SpecSkill{name: s, spec: sp} => {
                //TODO: also need to enforce invariant that SpecSkills never have value 0 - that just gives you the BasicSkill
                //need to enforce invariant that the basic skill must also exist if we add a "new specialist skill"
                if let Some(v) = self.skills.get_mut(&skill) && val > 0 { 
                    *v = val; 
                } else {
                    let basic = Skill::BasicSkill{name: *s};
                    if self.skills.contains_key(&basic) == false {
                        self.skills.insert(basic, 0);
                    }
                    if val > 0 {
                        self.skills.insert(skill, val);
                    }
                }
            }
        }
        Ok(())
    }

    //we could now write this as inc_skill_min(.., .., 0 ); ...
    fn inc_skill(&mut self, skill: Skill) -> Result<(),Err> {
        match &skill {
            Skill::BasicSkill{name: s} => {
                //TODO: need to enforce the invariant that a basic skill with a specialist skill pairing cannot have val > 0
                // (instead you need to pick a specialism)
                if (*s as i32) < BasicSkill::LAST as i32 {
                    if let Some(vv) = self.skills.get_mut(&skill) { //must be == 0 if it has a specialism
                        if *vv != 0 {
                            panic!("Basic var of special skill {:?} with nonzero positive skill value", skill)
                        }

                        if let Some(specialism) = skill.select_specialism_skill(&self.skills) { //launch prompt to pick a specialism to increase
                            self.inc_skill(specialism);
                        }

                    } else {
                        self.skills.insert(skill, 0); //add basic skill at level 0
                    }
                } else {
                    if let Some(vv) = self.skills.get_mut(&skill) {
                        *vv += 1;
                    } else {
                        self.skills.insert(skill, 0);
                    }
                };
            },
            Skill::SpecSkill{name: s, spec: sp} => {
                //need to enforce invariant that the basic skill must also exist if we add a "new specialist skill"
                if let Some(v) = self.skills.get_mut(&skill) { 
                    *v += 1; 
                } else {
                    let basic = Skill::BasicSkill{name: *s};
                    if self.skills.contains_key(&basic) == false { //if we never had the skill, we get basic at 0
                        self.skills.insert(basic, 0);
                    } else { //we have basic skill but not the specialism, which we get at 1
                        self.skills.insert(skill, 1);
                    }
                }
            }
        }
        Ok(())
    }

    fn inc_skill_min(&mut self, skill: Skill, minval: i8) -> Result<(),Err> {
        match &skill {
            Skill::BasicSkill{name: s} => {
                //TODO: need to enforce the invariant that a basic skill with a specialist skill pairing cannot have val > 0
                // (instead you need to pick a specialism)
                if (*s as i32) < BasicSkill::LAST as i32 {
                    if let Some(vv) = self.skills.get_mut(&skill) { //must be == 0 if it has a specialism
                        if *vv != 0 {
                            panic!("Basic var of special skill {:?} with nonzero positive skill value", skill)
                        }

                        if let Some(specialism) = skill.select_specialism_skill(&self.skills) { //launch prompt to pick a specialism to increase
                            self.inc_skill_min(specialism, minval);
                        }

                    } else {
                        self.skills.insert(skill, minval); //add basic skill at level 0
                    }
                } else {
                    if let Some(vv) = self.skills.get_mut(&skill) {
                        if *vv < minval {
                            *vv = minval;
                        } else {
                            *vv += 1;
                        }
                    } else {
                        self.skills.insert(skill, minval);
                    }
                };
            },
            Skill::SpecSkill{name: s, spec: sp} => {
                //need to enforce invariant that the basic skill must also exist if we add a "new specialist skill"
                if let Some(v) = self.skills.get_mut(&skill) {
                    if *v < minval {
                        *v = minval;
                    } else {
                        *v += 1;
                    } 
                } else {
                    let basic = Skill::BasicSkill{name: *s};
                    if self.skills.contains_key(&basic) == true { //we have the basic skill, so we're just upping the true skill
                        let minval_cap = std::cmp::max(minval, 1);
                        self.skills.insert(skill, minval_cap);
                    } else {
                        self.skills.insert(basic, 0); //add the basic skill first then...
                        if minval > 0 { //lets be nice and ignore if minval is 0 rather than throwing an error about giving specialist skills at 0
                            self.skills.insert(skill, minval);
                        }
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
                if (*s as i32) < (BasicSkill::LAST as i32) && val > 0 {
                    
                    //we interpret this as meaning "free selection for specialism skill"
                    if let Some(specialism) = skill.select_specialism_skill(&self.skills) {
                        self.set_min_skill(specialism, val);
                    }

                } else if let Some(vv) = self.skills.get_mut(&skill) {
                    if *vv < val {
                        *vv = val;
                    }
                } else {
                    self.skills.insert(skill, val);
                };
            },
            Skill::SpecSkill{name: s, spec: sp} => {
                //TODO: also need to enforce invariant that SpecSkills never have value 0 - that just gives you the BasicSkill
                //need to enforce invariant that the basic skill must also exist if we add a "new specialist skill"
                if let Some(vv) = self.skills.get_mut(&skill) && val > 0 { 
                    if *vv < val {
                        *vv = val;
                    }
                } else {
                    let basic = Skill::BasicSkill{name: *s};
                    if self.skills.contains_key(&basic) == false {
                        self.skills.insert(basic, 0);
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
