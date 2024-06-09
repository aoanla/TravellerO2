use rand;
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct DiePool {
    rng: rand::ThreadRng
}

impl DiePool {
    fn new() -> DiePool {
        DiePool{ rng: rand::thread_rng() }
    }

    fn d6(&self) -> i8 {
        (self.rng.gen() * 6).ceil() as i8
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
                if let Some(v) = self.skills.get_mut(skill) {
                    *v += val;
                } else {
                    self.skills.insert(skill, val);
                };
            },
            Skill::SpecSkill{name: s, spec: sp} => {
                //need to enforce invariant that the basic skill must also exist if we add a "new specialist skill"
                if let Some(v) = self.skills.get_mut(skill) { 
                    *v += val; 
                } else {
                    let basic = Skill::BasicSkill{name: *s as BasicSkill};
                    if ! self.skills.contains_key(basic) {
                        self.skills.insert(basic, 0)
                    }
                    self.skills.insert(skill, val);

                }
            }
        }
    }

}



fn main() {
    
}
