use bitflags::bitflags;

pub const HEADER_SIZE: usize = 0x6A0;
pub const MAX_SCORE: u32 = 99999950;

#[derive(Copy, Clone, PartialEq)]
pub enum PlayerPowerup {
    None,
    Mushroom,
    FireFlower,
    MiniMushroom,
    PropellerMushroom,
    PenguinSuit,
    IceFlower,
    HammerSuit,
}

pub const POWERUP_COUNT: usize = 8;

#[derive(Copy, Clone, PartialEq)]
pub enum PlayerCharacter {
    Mario,
    Luigi,
    BlueToad,
    YellowToad,
}

pub const PLAYER_COUNT: usize = 4;

pub const STAGE_COUNT: usize = 42;
pub const WORLD_COUNT: usize = 10;
pub const ACTUAL_WORLD_COUNT: usize = 9;

#[allow(clippy::upper_case_acronyms)]
#[derive(PartialEq)]
pub enum SaveFileRegion {
    NTSC,
    PAL,
    JPN,
    KOR,
    CHN,
    TW,
}

pub const AMBUSH_ENEMY_COUNT: usize = 4;

#[derive(Copy, Clone)]
pub enum EnemyDirection {
    ToNextNode,     // "forwards" to the next node
    ToPreviousNode, // "backwards" to the previous node
    FirstTimeValue, // the initial value that was set prior to entering the world for the first time
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct GameCompletionFlags: u8 {
        const SaveEmpty           = 0b00000001;
        const FinalBossBeaten     = 0b00000010;
        const AllGoals            = 0b00000100;
        const AllStarCoinsReg     = 0b00001000;
        const AllStarCoinsSpe     = 0b00010000;
        const GameCompleted       = 0b00100000;
        const SuperGuideTriggered = 0b01000000;
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct PlayerCreationFlags: u8 {
        const StarPower  = 0b00000001;
        const Yoshi      = 0b00000010;
        const Bubble     = 0b00000100;
        const RescueToad = 0b00001000;
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct StageCompletionFlags: u32 {
        const StarCoin1 = 0x1;
        const StarCoin2 = 0x2;
        const StarCoin3 = 0x4;
        const GoalNormal = 0x10;
        const GoalSecret = 0x20;
        const SuperGuideGoalNormal = 0x80;
        const SuperGuideGoalSecret = 0x100;
        const StageUnlocked = 0x200;
    }
}

// the powerup names used in the items menu on the world map
pub const ITEM_MENU_POWERUP_NAMES: [&str; 8] = [
    "Mushroom",
    "Fire Flower",
    "Propeller Mushroom",
    "Ice Flower",
    "Penguin Suit",
    "Mini Mushroom",
    "Star",
    "Hammer Suit",
];

pub const PLAYER_POWERUP_STATUS: [&str; 8] = [
    "None",
    "Mushroom",
    "Fire Flower",
    "Mini Mushroom",
    "Propeller Mushroom",
    "Penguin Suit",
    "Ice Flower",
    "Hammer Suit",
];

pub const POWERUP_STOCK_MAX: u8 = 99;
pub const PLAYER_LIFE_MAX: u8 = 99;

pub const PLAYER_NAMES: [&str; 4] = ["Mario", "Luigi", "Blue Toad", "Yellow Toad"];
