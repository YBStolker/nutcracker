pub const FULL_DECK: u64 = /**/ 0xFFFFFFFFFFFFF;

pub const SPADE: u64 = /*    */ 0x8888888888888;
pub const HEART: u64 = /*    */ 0x4444444444444;
pub const DIAMOND: u64 = /*  */ 0x2222222222222;
pub const CLUB: u64 = /*     */ 0x1111111111111;

pub const ACE: u64 = /*      */ 0xF000000000000;
pub const KING: u64 = /*     */ 0x0F00000000000;
pub const QUEEN: u64 = /*    */ 0x00F0000000000;
pub const JACK: u64 = /*     */ 0x000F000000000;
pub const TEN: u64 = /*      */ 0x0000F00000000;
pub const NINE: u64 = /*     */ 0x00000F0000000;
pub const EIGHT: u64 = /*    */ 0x000000F000000;
pub const SEVEN: u64 = /*    */ 0x0000000F00000;
pub const SIX: u64 = /*      */ 0x00000000F0000;
pub const FIVE: u64 = /*     */ 0x000000000F000;
pub const FOUR: u64 = /*     */ 0x0000000000F00;
pub const THREE: u64 = /*    */ 0x00000000000F0;
pub const TWO: u64 = /*      */ 0x000000000000F;

pub const RANKS: &[u64] = &[
    ACE, KING, QUEEN, JACK, TEN, NINE, EIGHT, SEVEN, SIX, FIVE, FOUR, THREE, TWO,
];
pub const RANK_NAMES: &[&str] = &[
    "A", "K", "Q", "J", "T", "9", "8", "7", "6", "5", "4", "3", "2",
];

pub const SUITS: &[u64] = &[SPADE, HEART, DIAMOND, CLUB];
pub const SUIT_NAMES: &[&str] = &["s", "h", "d", "c"];
