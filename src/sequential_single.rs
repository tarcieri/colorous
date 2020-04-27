use crate::sequential::Sequential;
use crate::Gradient;

/// &#8203;
///
/// <img src="https://raw.githubusercontent.com/dtolnay/essay/img/Blues.png" width="100%" height="40" alt="Blues">
pub const BLUES: Gradient = Gradient {
    eval: &Sequential {
        name: "Blues",
        three: colors!(0xdeebf7 0x9ecae1 0x3182bd),
        four: colors!(0xeff3ff 0xbdd7e7 0x6baed6 0x2171b5),
        five: colors!(0xeff3ff 0xbdd7e7 0x6baed6 0x3182bd 0x08519c),
        six: colors!(0xeff3ff 0xc6dbef 0x9ecae1 0x6baed6 0x3182bd 0x08519c),
        seven: colors!(0xeff3ff 0xc6dbef 0x9ecae1 0x6baed6 0x4292c6 0x2171b5 0x084594),
        eight: colors!(0xf7fbff 0xdeebf7 0xc6dbef 0x9ecae1 0x6baed6 0x4292c6 0x2171b5 0x084594),
        nine: colors!(0xf7fbff 0xdeebf7 0xc6dbef 0x9ecae1 0x6baed6 0x4292c6 0x2171b5 0x08519c 0x08306b),
    },
};

/// &#8203;
///
/// <img src="https://raw.githubusercontent.com/dtolnay/essay/img/Greens.png" width="100%" height="40" alt="Greens">
pub const GREENS: Gradient = Gradient {
    eval: &Sequential {
        name: "Greens",
        three: colors!(0xe5f5e0 0xa1d99b 0x31a354),
        four: colors!(0xedf8e9 0xbae4b3 0x74c476 0x238b45),
        five: colors!(0xedf8e9 0xbae4b3 0x74c476 0x31a354 0x006d2c),
        six: colors!(0xedf8e9 0xc7e9c0 0xa1d99b 0x74c476 0x31a354 0x006d2c),
        seven: colors!(0xedf8e9 0xc7e9c0 0xa1d99b 0x74c476 0x41ab5d 0x238b45 0x005a32),
        eight: colors!(0xf7fcf5 0xe5f5e0 0xc7e9c0 0xa1d99b 0x74c476 0x41ab5d 0x238b45 0x005a32),
        nine: colors!(0xf7fcf5 0xe5f5e0 0xc7e9c0 0xa1d99b 0x74c476 0x41ab5d 0x238b45 0x006d2c 0x00441b),
    },
};

/// &#8203;
///
/// <img src="https://raw.githubusercontent.com/dtolnay/essay/img/Greys.png" width="100%" height="40" alt="Greys">
pub const GREYS: Gradient = Gradient {
    eval: &Sequential {
        name: "Greys",
        three: colors!(0xf0f0f0 0xbdbdbd 0x636363),
        four: colors!(0xf7f7f7 0xcccccc 0x969696 0x525252),
        five: colors!(0xf7f7f7 0xcccccc 0x969696 0x636363 0x252525),
        six: colors!(0xf7f7f7 0xd9d9d9 0xbdbdbd 0x969696 0x636363 0x252525),
        seven: colors!(0xf7f7f7 0xd9d9d9 0xbdbdbd 0x969696 0x737373 0x525252 0x252525),
        eight: colors!(0xffffff 0xf0f0f0 0xd9d9d9 0xbdbdbd 0x969696 0x737373 0x525252 0x252525),
        nine: colors!(0xffffff 0xf0f0f0 0xd9d9d9 0xbdbdbd 0x969696 0x737373 0x525252 0x252525 0x000000),
    },
};

/// &#8203;
///
/// <img src="https://raw.githubusercontent.com/dtolnay/essay/img/Oranges.png" width="100%" height="40" alt="Oranges">
pub const ORANGES: Gradient = Gradient {
    eval: &Sequential {
        name: "Oranges",
        three: colors!(0xfee6ce 0xfdae6b 0xe6550d),
        four: colors!(0xfeedde 0xfdbe85 0xfd8d3c 0xd94701),
        five: colors!(0xfeedde 0xfdbe85 0xfd8d3c 0xe6550d 0xa63603),
        six: colors!(0xfeedde 0xfdd0a2 0xfdae6b 0xfd8d3c 0xe6550d 0xa63603),
        seven: colors!(0xfeedde 0xfdd0a2 0xfdae6b 0xfd8d3c 0xf16913 0xd94801 0x8c2d04),
        eight: colors!(0xfff5eb 0xfee6ce 0xfdd0a2 0xfdae6b 0xfd8d3c 0xf16913 0xd94801 0x8c2d04),
        nine: colors!(0xfff5eb 0xfee6ce 0xfdd0a2 0xfdae6b 0xfd8d3c 0xf16913 0xd94801 0xa63603 0x7f2704),
    },
};

/// &#8203;
///
/// <img src="https://raw.githubusercontent.com/dtolnay/essay/img/Purples.png" width="100%" height="40" alt="Purples">
pub const PURPLES: Gradient = Gradient {
    eval: &Sequential {
        name: "Purples",
        three: colors!(0xefedf5 0xbcbddc 0x756bb1),
        four: colors!(0xf2f0f7 0xcbc9e2 0x9e9ac8 0x6a51a3),
        five: colors!(0xf2f0f7 0xcbc9e2 0x9e9ac8 0x756bb1 0x54278f),
        six: colors!(0xf2f0f7 0xdadaeb 0xbcbddc 0x9e9ac8 0x756bb1 0x54278f),
        seven: colors!(0xf2f0f7 0xdadaeb 0xbcbddc 0x9e9ac8 0x807dba 0x6a51a3 0x4a1486),
        eight: colors!(0xfcfbfd 0xefedf5 0xdadaeb 0xbcbddc 0x9e9ac8 0x807dba 0x6a51a3 0x4a1486),
        nine: colors!(0xfcfbfd 0xefedf5 0xdadaeb 0xbcbddc 0x9e9ac8 0x807dba 0x6a51a3 0x54278f 0x3f007d),
    },
};

/// &#8203;
///
/// <img src="https://raw.githubusercontent.com/dtolnay/essay/img/Reds.png" width="100%" height="40" alt="Reds">
pub const REDS: Gradient = Gradient {
    eval: &Sequential {
        name: "Reds",
        three: colors!(0xfee0d2 0xfc9272 0xde2d26),
        four: colors!(0xfee5d9 0xfcae91 0xfb6a4a 0xcb181d),
        five: colors!(0xfee5d9 0xfcae91 0xfb6a4a 0xde2d26 0xa50f15),
        six: colors!(0xfee5d9 0xfcbba1 0xfc9272 0xfb6a4a 0xde2d26 0xa50f15),
        seven: colors!(0xfee5d9 0xfcbba1 0xfc9272 0xfb6a4a 0xef3b2c 0xcb181d 0x99000d),
        eight: colors!(0xfff5f0 0xfee0d2 0xfcbba1 0xfc9272 0xfb6a4a 0xef3b2c 0xcb181d 0x99000d),
        nine: colors!(0xfff5f0 0xfee0d2 0xfcbba1 0xfc9272 0xfb6a4a 0xef3b2c 0xcb181d 0xa50f15 0x67000d),
    },
};
