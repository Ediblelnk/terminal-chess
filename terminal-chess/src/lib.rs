mod board {

    const TOP_BORDER: &'static str = r".-.================================================================================================.-.";
    const BOTTOM_BORDER: &'static str = r"'-'=========[a]=========[b]=========[c]=========[d]=========[e]=========[f]=========[g]=========[h]'-'";
    #[rustfmt::skip]
    const PIECE: [[[&'static str; 13]; 5]; 2] = [
[
[r"::::::::::::",r"::::::::::::",r"::::::::::::",r"::::::::::::",r"::::::::::::",r"::: _ww_ :::",r"::::\++/::::",r"::::::::::::",r"::::::::::::",r"::::::::::::",r"::::::::::::",r"::: _ww_ :::",r"::::\++/::::"],
[r"::::::::::::",r"::::::::::::",r"::: __,,::::",r":::::<>:::::",r"::::UUUU::::",r"::: \  / :::",r"::::(  )::::",r"::::::::::::",r"::: __,, :::",r":::::<>:::::",r"::::UUUU::::",r"::: \@@/ :::",r"::::(@@)::::"],
[r"::::::::::::",r":::::():::::",r":: L   \~ ::",r":::::/\:::::",r"::::|  |::::",r"::: |  | :::",r"::::|  |::::",r":::::():::::",r":: L@@@\~ ::",r":::::/\:::::",r"::: |@@| :::",r"::: |@@| :::",r"::::|@@|::::"],
[r"::::::::::::",r":::::{}:::::",r":::: ) ( :::",r"::::\  /::::",r"::::|  |::::",r"::: /  \ :::",r"::::/  \::::",r":::: @@ ::::",r":::: )@( :::",r"::: \@@/ :::",r"::: |@@| :::",r"::: /@@\ :::",r"::::/@@\::::"],
[r"::::::::::::",r"::::{__}::::",r":: {____} ::",r":::{____}:::",r":::{____}:::",r":: {____} ::",r":::{____}:::",r"::: {@@} :::",r":: {@@@@} ::",r":: {@@@@} ::",r":: {@@@@} ::",r":: {@@@@} ::",r":::{@@@@}:::"],
],
[
[r"            ",r"            ",r"            ",r"            ",r"            ",r"    _ww_    ",r"    \++/    ",r"            ",r"            ",r"            ",r"            ",r"    _ww_    ",r"    \++/    "],
[r"            ",r"            ",r"    __,,    ",r"     <>     ",r"    UUUU    ",r"    \  /    ",r"    (  )    ",r"            ",r"    __,,    ",r"     <>     ",r"    UUUU    ",r"    \@@/    ",r"    (@@)    "],
[r"            ",r"     ()     ",r"   L   \~   ",r"     /\     ",r"    |  |    ",r"    |  |    ",r"    |  |    ",r"     ()     ",r"   L@@@\~   ",r"     /\     ",r"    |@@|    ",r"    |@@|    ",r"    |@@|    "],
[r"            ",r"     {}     ",r"     ) (    ",r"    \  /    ",r"    |  |    ",r"    /  \    ",r"    /  \    ",r"     @@     ",r"     )@(    ",r"    \@@/    ",r"    |@@|    ",r"    /@@\    ",r"    /@@\    "],
[r"            ",r"    {__}    ",r"   {____}   ",r"   {____}   ",r"   {____}   ",r"   {____}   ",r"   {____}   ",r"    {@@}    ",r"   {@@@@}   ",r"   {@@@@}   ",r"   {@@@@}   ",r"   {@@@@}   ",r"   {@@@@}   "],
],
];
}

struct Pieces {
    pawns: u64,
    knight: u64,
    bishop: u64,
    rooks: u64,
    queens: u64,
    kings: u64,
}

impl Pieces {
    fn new(white: bool) -> Self {
        match white {
            true => Self {
                pawns: 0b_00000000_11111111_00000000_00000000_00000000_00000000_00000000_00000000,
                knight: 0b_01000010_00000000_00000000_00000000_00000000_00000000_00000000_00000000,
                bishop: 0b_00100100_00000000_00000000_00000000_00000000_00000000_00000000_00000000,
                rooks: 0b_10000001_00000000_00000000_00000000_00000000_00000000_00000000_00000000,
                queens: 0b_00010000_00000000_00000000_00000000_00000000_00000000_00000000_00000000,
                kings: 0b_00001000_00000000_00000000_00000000_00000000_00000000_00000000_00000000,
            },

            false => Self {
                pawns: 0b_00000000_00000000_00000000_00000000_00000000_00000000_11111111_00000000,
                knight: 0b_00000000_00000000_00000000_00000000_00000000_00000000_00000000_01000010,
                bishop: 0b_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00100100,
                rooks: 0b_00000000_00000000_00000000_00000000_00000000_00000000_00000000_10000001,
                queens: 0b_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00001000,
                kings: 0b_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00001000,
            },
        }
    }
}

pub struct Chess {
    white: Pieces,
    black: Pieces,
}

impl Chess {
    fn new() -> Self {
        Self {
            white: Pieces::new(true),
            black: Pieces::new(false),
        }
    }

    fn print_board() {}
}

pub fn sample_board() -> &'static str {
    r"
.-.================================================================================================.-.
[8]::::::::::::            ::::::::::::    _ww_    ::::\++/::::            ::::::::::::            | |
| |::::UUUU::::    __,,    :::::<>:::::    \  /    ::::(  )::::     <>     ::: __,,::::    UUUU    | |
| |::::|  |::::   L   \~   :::::/\:::::    |  |    ::::|  |::::     /\     :: L   \~ ::    |  |    | |
| |::::|  |::::     ) (    ::::\  /::::    /  \    ::::/  \::::    \  /    :::: ) ( :::    |  |    | |
| |:::{____}:::   {____}   :::{____}:::   {____}   :::{____}:::   {____}   :: {____} ::   {____}   | |
[7]            ::::::::::::            ::::::::::::            ::::::::::::            ::::::::::::| |
| |            ::::::::::::            ::::::::::::            ::::::::::::            ::::::::::::| |
| |     ()     :::::():::::     ()     :::::():::::     ()     :::::():::::     ()     :::::():::::| |
| |     {}     :::::{}:::::     {}     :::::{}:::::     {}     :::::{}:::::     {}     :::::{}:::::| |
| |    {__}    ::::{__}::::    {__}    ::::{__}::::    {__}    ::::{__}::::    {__}    ::::{__}::::| |
[6]::::::::::::            ::::::::::::            ::::::::::::            ::::::::::::            | |
| |::::::::::::            ::::::::::::            ::::::::::::            ::::::::::::            | |
| |::::::::::::            ::::::::::::            ::::::::::::            ::::::::::::            | |
| |::::::::::::            ::::::::::::            ::::::::::::            ::::::::::::            | |
| |::::::::::::            ::::::::::::            ::::::::::::            ::::::::::::            | |
[5]            ::::::::::::            ::::::::::::            ::::::::::::            ::::::::::::| |
| |            ::::::::::::            ::::::::::::            ::::::::::::            ::::::::::::| |
| |            ::::::::::::            ::::::::::::            ::::::::::::            ::::::::::::| |
| |            ::::::::::::            ::::::::::::            ::::::::::::            ::::::::::::| |
| |            ::::::::::::            ::::::::::::            ::::::::::::            ::::::::::::| |
[4]::::::::::::            ::::::::::::            ::::::::::::            ::::::::::::            | |
| |::::::::::::            ::::::::::::            ::::::::::::            ::::::::::::            | |
| |::::::::::::            ::::::::::::            ::::::::::::            ::::::::::::            | |
| |::::::::::::            ::::::::::::            ::::::::::::            ::::::::::::            | |
| |::::::::::::            ::::::::::::            ::::::::::::            ::::::::::::            | |
[3]            ::::::::::::            ::::::::::::            ::::::::::::            ::::::::::::| |
| |            ::::::::::::            ::::::::::::            ::::::::::::            ::::::::::::| |
| |            ::::::::::::            ::::::::::::            ::::::::::::            ::::::::::::| |
| |            ::::::::::::            ::::::::::::            ::::::::::::            ::::::::::::| |
| |            ::::::::::::            ::::::::::::            ::::::::::::            ::::::::::::| |
[2]::::::::::::            ::::::::::::            ::::::::::::            ::::::::::::            | |
| |::::::::::::            ::::::::::::            ::::::::::::            ::::::::::::            | |
| |:::: () ::::     ()     :::: () ::::     ()     :::: () ::::     ()     :::: () ::::     ()     | |
| |:::: @@ ::::     @@     :::: @@ ::::     @@     :::: @@ ::::     @@     :::: @@ ::::     @@     | |
| |::: {@@} :::    {@@}    ::: {@@} :::    {@@}    ::: {@@} :::    {@@}    ::: {@@} :::    {@@}    | |
[1]            ::::::::::::            ::: _ww_ :::    \++/    ::::::::::::            ::::::::::::| |
| |    UUUU    ::: __,, :::     <>     ::: \@@/ :::    (@@)    :::: <> ::::    __,,    ::: UUUU :::| |
| |    |@@|    :: L@@@\~ ::     /\     ::: |@@| :::    |@@|    :::: /\ ::::   L@@@\~   ::: |@@| :::| |
| |    |@@|    :::: )@( :::    \@@/    ::: /@@\ :::    /@@\    ::: \@@/ :::     )@(    ::: |@@| :::| |
| |   {@@@@}   :: {@@@@} ::   {@@@@}   :: {@@@@} ::   {@@@@}   :: {@@@@} ::   {@@@@}   :: {@@@@} ::| |
'-'=========[a]=========[b]=========[c]=========[d]=========[e]=========[f]=========[g]=========[h]'-'"
}
