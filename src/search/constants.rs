pub type Milliseconds = u128;
pub type Nodes = u64;
pub type Depth = i8;
pub type Ply = u8;
const MAX_DEPTH: Depth = i8::MAX;
pub const MAX_PLY: Ply = MAX_DEPTH as u8;

pub type EvalScore = i32;
pub const INF: EvalScore = (i16::MAX - 10) as i32;
pub const EVAL_MAX: EvalScore = INF - 1;
pub const MATE_THRESHOLD: EvalScore = EVAL_MAX - (MAX_PLY as i32);
