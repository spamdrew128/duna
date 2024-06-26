#[macro_export]
macro_rules! tuple_constants_enum {
    ($t:ty, $($n:ident),*) => {
        tuple_constants_enum!($t, 0, $($n),*);
    };
    ($t:ty, $val:expr, $name:ident) => {
        pub const $name: $t = <$t>::new($val);
    };
    ($t:ty, $val:expr, $name:ident, $($n:ident),*) => {
        pub const $name: $t = <$t>::new($val);
        tuple_constants_enum!($t, $val + 1, $($n),*);
    };
}

#[macro_export]
macro_rules! bb_from_squares {
    ($($sq:ident),*) => {{
        let mut result = Bitboard::EMPTY;
        $(result = result.or(Square::$sq.as_bitboard());)*
        result
    }};
}

#[macro_export]
macro_rules! bitloop {
    (|$sq:ident| $bb:expr, $body:expr) => {{
        let mut bb = $bb;
        while bb.not_empty() {
            let $sq: Square = bb.pop_lsb();
            $body
        }
    }};
}
