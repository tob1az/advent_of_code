#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

pub struct Line {
    pub a: Point,
    pub b: Point,
}

pub const LINES: [Line; 500] = [
    Line {
        a: Point { x: 645, y: 570 },
        b: Point { x: 517, y: 570 },
    },
    Line {
        a: Point { x: 100, y: 409 },
        b: Point { x: 200, y: 409 },
    },
    Line {
        a: Point { x: 945, y: 914 },
        b: Point { x: 98, y: 67 },
    },
    Line {
        a: Point { x: 22, y: 934 },
        b: Point { x: 22, y: 681 },
    },
    Line {
        a: Point { x: 935, y: 781 },
        b: Point { x: 524, y: 370 },
    },
    Line {
        a: Point { x: 750, y: 304 },
        b: Point { x: 854, y: 408 },
    },
    Line {
        a: Point { x: 974, y: 27 },
        b: Point { x: 26, y: 975 },
    },
    Line {
        a: Point { x: 529, y: 58 },
        b: Point { x: 979, y: 58 },
    },
    Line {
        a: Point { x: 979, y: 515 },
        b: Point { x: 550, y: 944 },
    },
    Line {
        a: Point { x: 925, y: 119 },
        b: Point { x: 17, y: 119 },
    },
    Line {
        a: Point { x: 178, y: 594 },
        b: Point { x: 45, y: 461 },
    },
    Line {
        a: Point { x: 252, y: 366 },
        b: Point { x: 92, y: 206 },
    },
    Line {
        a: Point { x: 25, y: 593 },
        b: Point { x: 250, y: 593 },
    },
    Line {
        a: Point { x: 956, y: 34 },
        b: Point { x: 21, y: 969 },
    },
    Line {
        a: Point { x: 200, y: 671 },
        b: Point { x: 200, y: 369 },
    },
    Line {
        a: Point { x: 628, y: 614 },
        b: Point { x: 628, y: 637 },
    },
    Line {
        a: Point { x: 697, y: 428 },
        b: Point { x: 237, y: 428 },
    },
    Line {
        a: Point { x: 554, y: 40 },
        b: Point { x: 554, y: 949 },
    },
    Line {
        a: Point { x: 927, y: 197 },
        b: Point { x: 469, y: 197 },
    },
    Line {
        a: Point { x: 504, y: 779 },
        b: Point { x: 593, y: 868 },
    },
    Line {
        a: Point { x: 227, y: 882 },
        b: Point { x: 227, y: 982 },
    },
    Line {
        a: Point { x: 56, y: 905 },
        b: Point { x: 56, y: 81 },
    },
    Line {
        a: Point { x: 438, y: 874 },
        b: Point { x: 566, y: 746 },
    },
    Line {
        a: Point { x: 989, y: 73 },
        b: Point { x: 113, y: 949 },
    },
    Line {
        a: Point { x: 82, y: 36 },
        b: Point { x: 616, y: 570 },
    },
    Line {
        a: Point { x: 670, y: 423 },
        b: Point { x: 670, y: 873 },
    },
    Line {
        a: Point { x: 100, y: 435 },
        b: Point { x: 291, y: 435 },
    },
    Line {
        a: Point { x: 242, y: 81 },
        b: Point { x: 978, y: 817 },
    },
    Line {
        a: Point { x: 367, y: 335 },
        b: Point { x: 367, y: 332 },
    },
    Line {
        a: Point { x: 890, y: 584 },
        b: Point { x: 116, y: 584 },
    },
    Line {
        a: Point { x: 572, y: 192 },
        b: Point { x: 572, y: 561 },
    },
    Line {
        a: Point { x: 391, y: 516 },
        b: Point { x: 391, y: 559 },
    },
    Line {
        a: Point { x: 525, y: 62 },
        b: Point { x: 525, y: 540 },
    },
    Line {
        a: Point { x: 787, y: 540 },
        b: Point { x: 812, y: 515 },
    },
    Line {
        a: Point { x: 749, y: 732 },
        b: Point { x: 423, y: 406 },
    },
    Line {
        a: Point { x: 745, y: 911 },
        b: Point { x: 694, y: 911 },
    },
    Line {
        a: Point { x: 805, y: 18 },
        b: Point { x: 972, y: 18 },
    },
    Line {
        a: Point { x: 701, y: 565 },
        b: Point { x: 280, y: 144 },
    },
    Line {
        a: Point { x: 930, y: 92 },
        b: Point { x: 129, y: 893 },
    },
    Line {
        a: Point { x: 15, y: 989 },
        b: Point { x: 970, y: 34 },
    },
    Line {
        a: Point { x: 409, y: 920 },
        b: Point { x: 409, y: 345 },
    },
    Line {
        a: Point { x: 192, y: 743 },
        b: Point { x: 312, y: 863 },
    },
    Line {
        a: Point { x: 724, y: 12 },
        b: Point { x: 29, y: 707 },
    },
    Line {
        a: Point { x: 323, y: 664 },
        b: Point { x: 323, y: 897 },
    },
    Line {
        a: Point { x: 161, y: 423 },
        b: Point { x: 391, y: 653 },
    },
    Line {
        a: Point { x: 59, y: 363 },
        b: Point { x: 250, y: 554 },
    },
    Line {
        a: Point { x: 407, y: 676 },
        b: Point { x: 19, y: 288 },
    },
    Line {
        a: Point { x: 449, y: 585 },
        b: Point { x: 449, y: 301 },
    },
    Line {
        a: Point { x: 914, y: 798 },
        b: Point { x: 914, y: 806 },
    },
    Line {
        a: Point { x: 917, y: 401 },
        b: Point { x: 288, y: 401 },
    },
    Line {
        a: Point { x: 588, y: 800 },
        b: Point { x: 647, y: 800 },
    },
    Line {
        a: Point { x: 897, y: 883 },
        b: Point { x: 897, y: 276 },
    },
    Line {
        a: Point { x: 115, y: 606 },
        b: Point { x: 41, y: 532 },
    },
    Line {
        a: Point { x: 692, y: 482 },
        b: Point { x: 777, y: 482 },
    },
    Line {
        a: Point { x: 428, y: 736 },
        b: Point { x: 69, y: 736 },
    },
    Line {
        a: Point { x: 405, y: 44 },
        b: Point { x: 405, y: 632 },
    },
    Line {
        a: Point { x: 198, y: 482 },
        b: Point { x: 198, y: 620 },
    },
    Line {
        a: Point { x: 988, y: 816 },
        b: Point { x: 988, y: 598 },
    },
    Line {
        a: Point { x: 254, y: 461 },
        b: Point { x: 186, y: 393 },
    },
    Line {
        a: Point { x: 560, y: 783 },
        b: Point { x: 208, y: 783 },
    },
    Line {
        a: Point { x: 856, y: 766 },
        b: Point { x: 215, y: 125 },
    },
    Line {
        a: Point { x: 182, y: 30 },
        b: Point { x: 569, y: 30 },
    },
    Line {
        a: Point { x: 504, y: 242 },
        b: Point { x: 656, y: 242 },
    },
    Line {
        a: Point { x: 393, y: 929 },
        b: Point { x: 131, y: 929 },
    },
    Line {
        a: Point { x: 597, y: 359 },
        b: Point { x: 26, y: 930 },
    },
    Line {
        a: Point { x: 502, y: 690 },
        b: Point { x: 255, y: 443 },
    },
    Line {
        a: Point { x: 149, y: 608 },
        b: Point { x: 149, y: 748 },
    },
    Line {
        a: Point { x: 293, y: 662 },
        b: Point { x: 622, y: 662 },
    },
    Line {
        a: Point { x: 697, y: 154 },
        b: Point { x: 697, y: 228 },
    },
    Line {
        a: Point { x: 587, y: 804 },
        b: Point { x: 983, y: 804 },
    },
    Line {
        a: Point { x: 715, y: 63 },
        b: Point { x: 715, y: 709 },
    },
    Line {
        a: Point { x: 496, y: 831 },
        b: Point { x: 23, y: 358 },
    },
    Line {
        a: Point { x: 461, y: 48 },
        b: Point { x: 68, y: 441 },
    },
    Line {
        a: Point { x: 927, y: 565 },
        b: Point { x: 595, y: 565 },
    },
    Line {
        a: Point { x: 972, y: 350 },
        b: Point { x: 689, y: 350 },
    },
    Line {
        a: Point { x: 728, y: 438 },
        b: Point { x: 728, y: 221 },
    },
    Line {
        a: Point { x: 173, y: 134 },
        b: Point { x: 173, y: 804 },
    },
    Line {
        a: Point { x: 720, y: 368 },
        b: Point { x: 121, y: 368 },
    },
    Line {
        a: Point { x: 690, y: 66 },
        b: Point { x: 201, y: 66 },
    },
    Line {
        a: Point { x: 218, y: 680 },
        b: Point { x: 841, y: 680 },
    },
    Line {
        a: Point { x: 80, y: 792 },
        b: Point { x: 80, y: 467 },
    },
    Line {
        a: Point { x: 624, y: 319 },
        b: Point { x: 624, y: 461 },
    },
    Line {
        a: Point { x: 248, y: 348 },
        b: Point { x: 532, y: 64 },
    },
    Line {
        a: Point { x: 357, y: 260 },
        b: Point { x: 505, y: 408 },
    },
    Line {
        a: Point { x: 296, y: 814 },
        b: Point { x: 13, y: 531 },
    },
    Line {
        a: Point { x: 819, y: 216 },
        b: Point { x: 819, y: 932 },
    },
    Line {
        a: Point { x: 696, y: 233 },
        b: Point { x: 696, y: 840 },
    },
    Line {
        a: Point { x: 219, y: 93 },
        b: Point { x: 868, y: 93 },
    },
    Line {
        a: Point { x: 537, y: 63 },
        b: Point { x: 905, y: 63 },
    },
    Line {
        a: Point { x: 777, y: 940 },
        b: Point { x: 777, y: 84 },
    },
    Line {
        a: Point { x: 286, y: 133 },
        b: Point { x: 286, y: 735 },
    },
    Line {
        a: Point { x: 969, y: 967 },
        b: Point { x: 969, y: 823 },
    },
    Line {
        a: Point { x: 254, y: 222 },
        b: Point { x: 859, y: 827 },
    },
    Line {
        a: Point { x: 426, y: 728 },
        b: Point { x: 426, y: 388 },
    },
    Line {
        a: Point { x: 854, y: 561 },
        b: Point { x: 854, y: 363 },
    },
    Line {
        a: Point { x: 755, y: 861 },
        b: Point { x: 755, y: 947 },
    },
    Line {
        a: Point { x: 570, y: 754 },
        b: Point { x: 439, y: 754 },
    },
    Line {
        a: Point { x: 333, y: 351 },
        b: Point { x: 333, y: 828 },
    },
    Line {
        a: Point { x: 436, y: 693 },
        b: Point { x: 436, y: 262 },
    },
    Line {
        a: Point { x: 982, y: 987 },
        b: Point { x: 172, y: 177 },
    },
    Line {
        a: Point { x: 267, y: 178 },
        b: Point { x: 267, y: 270 },
    },
    Line {
        a: Point { x: 218, y: 201 },
        b: Point { x: 747, y: 730 },
    },
    Line {
        a: Point { x: 811, y: 602 },
        b: Point { x: 829, y: 584 },
    },
    Line {
        a: Point { x: 602, y: 659 },
        b: Point { x: 766, y: 659 },
    },
    Line {
        a: Point { x: 536, y: 544 },
        b: Point { x: 483, y: 597 },
    },
    Line {
        a: Point { x: 280, y: 881 },
        b: Point { x: 547, y: 881 },
    },
    Line {
        a: Point { x: 584, y: 125 },
        b: Point { x: 129, y: 125 },
    },
    Line {
        a: Point { x: 386, y: 210 },
        b: Point { x: 757, y: 210 },
    },
    Line {
        a: Point { x: 605, y: 855 },
        b: Point { x: 605, y: 668 },
    },
    Line {
        a: Point { x: 19, y: 985 },
        b: Point { x: 988, y: 16 },
    },
    Line {
        a: Point { x: 980, y: 655 },
        b: Point { x: 836, y: 655 },
    },
    Line {
        a: Point { x: 73, y: 189 },
        b: Point { x: 267, y: 383 },
    },
    Line {
        a: Point { x: 621, y: 645 },
        b: Point { x: 533, y: 645 },
    },
    Line {
        a: Point { x: 36, y: 12 },
        b: Point { x: 255, y: 231 },
    },
    Line {
        a: Point { x: 538, y: 889 },
        b: Point { x: 130, y: 481 },
    },
    Line {
        a: Point { x: 921, y: 217 },
        b: Point { x: 921, y: 724 },
    },
    Line {
        a: Point { x: 873, y: 59 },
        b: Point { x: 873, y: 311 },
    },
    Line {
        a: Point { x: 76, y: 918 },
        b: Point { x: 970, y: 24 },
    },
    Line {
        a: Point { x: 694, y: 448 },
        b: Point { x: 694, y: 983 },
    },
    Line {
        a: Point { x: 573, y: 891 },
        b: Point { x: 573, y: 337 },
    },
    Line {
        a: Point { x: 796, y: 358 },
        b: Point { x: 403, y: 358 },
    },
    Line {
        a: Point { x: 532, y: 928 },
        b: Point { x: 351, y: 928 },
    },
    Line {
        a: Point { x: 123, y: 717 },
        b: Point { x: 123, y: 446 },
    },
    Line {
        a: Point { x: 874, y: 714 },
        b: Point { x: 874, y: 886 },
    },
    Line {
        a: Point { x: 350, y: 458 },
        b: Point { x: 728, y: 458 },
    },
    Line {
        a: Point { x: 798, y: 140 },
        b: Point { x: 798, y: 242 },
    },
    Line {
        a: Point { x: 832, y: 406 },
        b: Point { x: 864, y: 406 },
    },
    Line {
        a: Point { x: 188, y: 55 },
        b: Point { x: 188, y: 641 },
    },
    Line {
        a: Point { x: 903, y: 376 },
        b: Point { x: 509, y: 376 },
    },
    Line {
        a: Point { x: 50, y: 954 },
        b: Point { x: 989, y: 15 },
    },
    Line {
        a: Point { x: 42, y: 294 },
        b: Point { x: 25, y: 294 },
    },
    Line {
        a: Point { x: 544, y: 273 },
        b: Point { x: 974, y: 273 },
    },
    Line {
        a: Point { x: 804, y: 756 },
        b: Point { x: 103, y: 55 },
    },
    Line {
        a: Point { x: 398, y: 184 },
        b: Point { x: 570, y: 12 },
    },
    Line {
        a: Point { x: 82, y: 179 },
        b: Point { x: 902, y: 179 },
    },
    Line {
        a: Point { x: 461, y: 728 },
        b: Point { x: 905, y: 284 },
    },
    Line {
        a: Point { x: 429, y: 241 },
        b: Point { x: 26, y: 241 },
    },
    Line {
        a: Point { x: 128, y: 715 },
        b: Point { x: 207, y: 715 },
    },
    Line {
        a: Point { x: 239, y: 545 },
        b: Point { x: 934, y: 545 },
    },
    Line {
        a: Point { x: 978, y: 769 },
        b: Point { x: 978, y: 576 },
    },
    Line {
        a: Point { x: 250, y: 77 },
        b: Point { x: 515, y: 77 },
    },
    Line {
        a: Point { x: 521, y: 533 },
        b: Point { x: 521, y: 434 },
    },
    Line {
        a: Point { x: 955, y: 844 },
        b: Point { x: 314, y: 203 },
    },
    Line {
        a: Point { x: 144, y: 601 },
        b: Point { x: 702, y: 43 },
    },
    Line {
        a: Point { x: 313, y: 784 },
        b: Point { x: 339, y: 784 },
    },
    Line {
        a: Point { x: 388, y: 692 },
        b: Point { x: 805, y: 275 },
    },
    Line {
        a: Point { x: 540, y: 872 },
        b: Point { x: 540, y: 72 },
    },
    Line {
        a: Point { x: 971, y: 19 },
        b: Point { x: 17, y: 973 },
    },
    Line {
        a: Point { x: 816, y: 540 },
        b: Point { x: 386, y: 540 },
    },
    Line {
        a: Point { x: 933, y: 246 },
        b: Point { x: 560, y: 619 },
    },
    Line {
        a: Point { x: 800, y: 600 },
        b: Point { x: 387, y: 187 },
    },
    Line {
        a: Point { x: 272, y: 791 },
        b: Point { x: 129, y: 934 },
    },
    Line {
        a: Point { x: 908, y: 133 },
        b: Point { x: 110, y: 931 },
    },
    Line {
        a: Point { x: 759, y: 191 },
        b: Point { x: 910, y: 40 },
    },
    Line {
        a: Point { x: 420, y: 479 },
        b: Point { x: 749, y: 150 },
    },
    Line {
        a: Point { x: 604, y: 946 },
        b: Point { x: 804, y: 946 },
    },
    Line {
        a: Point { x: 633, y: 404 },
        b: Point { x: 771, y: 266 },
    },
    Line {
        a: Point { x: 948, y: 974 },
        b: Point { x: 948, y: 734 },
    },
    Line {
        a: Point { x: 735, y: 198 },
        b: Point { x: 105, y: 828 },
    },
    Line {
        a: Point { x: 889, y: 653 },
        b: Point { x: 889, y: 688 },
    },
    Line {
        a: Point { x: 157, y: 172 },
        b: Point { x: 822, y: 837 },
    },
    Line {
        a: Point { x: 206, y: 670 },
        b: Point { x: 297, y: 670 },
    },
    Line {
        a: Point { x: 50, y: 122 },
        b: Point { x: 792, y: 864 },
    },
    Line {
        a: Point { x: 656, y: 664 },
        b: Point { x: 27, y: 664 },
    },
    Line {
        a: Point { x: 966, y: 33 },
        b: Point { x: 523, y: 33 },
    },
    Line {
        a: Point { x: 985, y: 40 },
        b: Point { x: 101, y: 924 },
    },
    Line {
        a: Point { x: 394, y: 367 },
        b: Point { x: 574, y: 547 },
    },
    Line {
        a: Point { x: 440, y: 573 },
        b: Point { x: 268, y: 573 },
    },
    Line {
        a: Point { x: 159, y: 989 },
        b: Point { x: 159, y: 130 },
    },
    Line {
        a: Point { x: 867, y: 123 },
        b: Point { x: 867, y: 891 },
    },
    Line {
        a: Point { x: 316, y: 153 },
        b: Point { x: 316, y: 249 },
    },
    Line {
        a: Point { x: 680, y: 59 },
        b: Point { x: 773, y: 152 },
    },
    Line {
        a: Point { x: 52, y: 928 },
        b: Point { x: 52, y: 182 },
    },
    Line {
        a: Point { x: 128, y: 595 },
        b: Point { x: 225, y: 595 },
    },
    Line {
        a: Point { x: 508, y: 719 },
        b: Point { x: 591, y: 719 },
    },
    Line {
        a: Point { x: 595, y: 447 },
        b: Point { x: 709, y: 333 },
    },
    Line {
        a: Point { x: 930, y: 783 },
        b: Point { x: 283, y: 136 },
    },
    Line {
        a: Point { x: 366, y: 236 },
        b: Point { x: 283, y: 236 },
    },
    Line {
        a: Point { x: 820, y: 512 },
        b: Point { x: 381, y: 951 },
    },
    Line {
        a: Point { x: 135, y: 450 },
        b: Point { x: 135, y: 766 },
    },
    Line {
        a: Point { x: 750, y: 838 },
        b: Point { x: 534, y: 838 },
    },
    Line {
        a: Point { x: 259, y: 304 },
        b: Point { x: 626, y: 671 },
    },
    Line {
        a: Point { x: 414, y: 631 },
        b: Point { x: 916, y: 129 },
    },
    Line {
        a: Point { x: 193, y: 862 },
        b: Point { x: 901, y: 154 },
    },
    Line {
        a: Point { x: 362, y: 595 },
        b: Point { x: 362, y: 209 },
    },
    Line {
        a: Point { x: 377, y: 215 },
        b: Point { x: 377, y: 499 },
    },
    Line {
        a: Point { x: 723, y: 16 },
        b: Point { x: 577, y: 16 },
    },
    Line {
        a: Point { x: 335, y: 238 },
        b: Point { x: 790, y: 693 },
    },
    Line {
        a: Point { x: 670, y: 266 },
        b: Point { x: 871, y: 65 },
    },
    Line {
        a: Point { x: 288, y: 313 },
        b: Point { x: 213, y: 313 },
    },
    Line {
        a: Point { x: 48, y: 423 },
        b: Point { x: 592, y: 967 },
    },
    Line {
        a: Point { x: 960, y: 323 },
        b: Point { x: 911, y: 323 },
    },
    Line {
        a: Point { x: 177, y: 182 },
        b: Point { x: 177, y: 235 },
    },
    Line {
        a: Point { x: 773, y: 918 },
        b: Point { x: 757, y: 918 },
    },
    Line {
        a: Point { x: 216, y: 432 },
        b: Point { x: 147, y: 432 },
    },
    Line {
        a: Point { x: 808, y: 500 },
        b: Point { x: 656, y: 500 },
    },
    Line {
        a: Point { x: 205, y: 451 },
        b: Point { x: 776, y: 451 },
    },
    Line {
        a: Point { x: 598, y: 985 },
        b: Point { x: 598, y: 608 },
    },
    Line {
        a: Point { x: 193, y: 253 },
        b: Point { x: 241, y: 205 },
    },
    Line {
        a: Point { x: 912, y: 384 },
        b: Point { x: 912, y: 532 },
    },
    Line {
        a: Point { x: 214, y: 194 },
        b: Point { x: 214, y: 738 },
    },
    Line {
        a: Point { x: 508, y: 356 },
        b: Point { x: 508, y: 792 },
    },
    Line {
        a: Point { x: 16, y: 372 },
        b: Point { x: 30, y: 372 },
    },
    Line {
        a: Point { x: 384, y: 854 },
        b: Point { x: 986, y: 252 },
    },
    Line {
        a: Point { x: 361, y: 569 },
        b: Point { x: 851, y: 569 },
    },
    Line {
        a: Point { x: 923, y: 550 },
        b: Point { x: 923, y: 441 },
    },
    Line {
        a: Point { x: 271, y: 257 },
        b: Point { x: 318, y: 304 },
    },
    Line {
        a: Point { x: 651, y: 345 },
        b: Point { x: 651, y: 397 },
    },
    Line {
        a: Point { x: 885, y: 14 },
        b: Point { x: 929, y: 14 },
    },
    Line {
        a: Point { x: 199, y: 547 },
        b: Point { x: 925, y: 547 },
    },
    Line {
        a: Point { x: 803, y: 176 },
        b: Point { x: 104, y: 875 },
    },
    Line {
        a: Point { x: 840, y: 302 },
        b: Point { x: 197, y: 945 },
    },
    Line {
        a: Point { x: 971, y: 743 },
        b: Point { x: 355, y: 127 },
    },
    Line {
        a: Point { x: 684, y: 951 },
        b: Point { x: 684, y: 292 },
    },
    Line {
        a: Point { x: 58, y: 867 },
        b: Point { x: 58, y: 953 },
    },
    Line {
        a: Point { x: 351, y: 187 },
        b: Point { x: 351, y: 831 },
    },
    Line {
        a: Point { x: 701, y: 413 },
        b: Point { x: 701, y: 728 },
    },
    Line {
        a: Point { x: 482, y: 159 },
        b: Point { x: 134, y: 159 },
    },
    Line {
        a: Point { x: 118, y: 52 },
        b: Point { x: 950, y: 884 },
    },
    Line {
        a: Point { x: 115, y: 968 },
        b: Point { x: 115, y: 137 },
    },
    Line {
        a: Point { x: 437, y: 739 },
        b: Point { x: 627, y: 929 },
    },
    Line {
        a: Point { x: 653, y: 153 },
        b: Point { x: 549, y: 153 },
    },
    Line {
        a: Point { x: 604, y: 504 },
        b: Point { x: 560, y: 460 },
    },
    Line {
        a: Point { x: 538, y: 865 },
        b: Point { x: 840, y: 563 },
    },
    Line {
        a: Point { x: 114, y: 876 },
        b: Point { x: 114, y: 124 },
    },
    Line {
        a: Point { x: 152, y: 899 },
        b: Point { x: 925, y: 126 },
    },
    Line {
        a: Point { x: 973, y: 224 },
        b: Point { x: 973, y: 387 },
    },
    Line {
        a: Point { x: 492, y: 360 },
        b: Point { x: 861, y: 729 },
    },
    Line {
        a: Point { x: 927, y: 902 },
        b: Point { x: 108, y: 83 },
    },
    Line {
        a: Point { x: 754, y: 678 },
        b: Point { x: 754, y: 647 },
    },
    Line {
        a: Point { x: 526, y: 671 },
        b: Point { x: 423, y: 671 },
    },
    Line {
        a: Point { x: 675, y: 608 },
        b: Point { x: 243, y: 608 },
    },
    Line {
        a: Point { x: 147, y: 241 },
        b: Point { x: 147, y: 242 },
    },
    Line {
        a: Point { x: 456, y: 770 },
        b: Point { x: 456, y: 665 },
    },
    Line {
        a: Point { x: 953, y: 50 },
        b: Point { x: 102, y: 901 },
    },
    Line {
        a: Point { x: 415, y: 869 },
        b: Point { x: 415, y: 733 },
    },
    Line {
        a: Point { x: 979, y: 533 },
        b: Point { x: 169, y: 533 },
    },
    Line {
        a: Point { x: 336, y: 385 },
        b: Point { x: 336, y: 18 },
    },
    Line {
        a: Point { x: 927, y: 176 },
        b: Point { x: 927, y: 587 },
    },
    Line {
        a: Point { x: 370, y: 317 },
        b: Point { x: 933, y: 880 },
    },
    Line {
        a: Point { x: 450, y: 349 },
        b: Point { x: 450, y: 103 },
    },
    Line {
        a: Point { x: 755, y: 235 },
        b: Point { x: 408, y: 235 },
    },
    Line {
        a: Point { x: 342, y: 55 },
        b: Point { x: 931, y: 55 },
    },
    Line {
        a: Point { x: 417, y: 707 },
        b: Point { x: 887, y: 237 },
    },
    Line {
        a: Point { x: 141, y: 95 },
        b: Point { x: 131, y: 85 },
    },
    Line {
        a: Point { x: 776, y: 209 },
        b: Point { x: 590, y: 23 },
    },
    Line {
        a: Point { x: 39, y: 732 },
        b: Point { x: 469, y: 302 },
    },
    Line {
        a: Point { x: 743, y: 602 },
        b: Point { x: 743, y: 358 },
    },
    Line {
        a: Point { x: 473, y: 439 },
        b: Point { x: 473, y: 545 },
    },
    Line {
        a: Point { x: 270, y: 290 },
        b: Point { x: 270, y: 640 },
    },
    Line {
        a: Point { x: 904, y: 963 },
        b: Point { x: 949, y: 963 },
    },
    Line {
        a: Point { x: 71, y: 91 },
        b: Point { x: 956, y: 976 },
    },
    Line {
        a: Point { x: 865, y: 757 },
        b: Point { x: 276, y: 757 },
    },
    Line {
        a: Point { x: 59, y: 72 },
        b: Point { x: 966, y: 979 },
    },
    Line {
        a: Point { x: 46, y: 184 },
        b: Point { x: 788, y: 926 },
    },
    Line {
        a: Point { x: 360, y: 833 },
        b: Point { x: 561, y: 833 },
    },
    Line {
        a: Point { x: 120, y: 452 },
        b: Point { x: 528, y: 452 },
    },
    Line {
        a: Point { x: 704, y: 927 },
        b: Point { x: 158, y: 381 },
    },
    Line {
        a: Point { x: 140, y: 481 },
        b: Point { x: 140, y: 350 },
    },
    Line {
        a: Point { x: 929, y: 920 },
        b: Point { x: 929, y: 342 },
    },
    Line {
        a: Point { x: 328, y: 381 },
        b: Point { x: 328, y: 866 },
    },
    Line {
        a: Point { x: 897, y: 389 },
        b: Point { x: 227, y: 389 },
    },
    Line {
        a: Point { x: 341, y: 614 },
        b: Point { x: 29, y: 614 },
    },
    Line {
        a: Point { x: 609, y: 327 },
        b: Point { x: 609, y: 582 },
    },
    Line {
        a: Point { x: 727, y: 858 },
        b: Point { x: 727, y: 941 },
    },
    Line {
        a: Point { x: 349, y: 536 },
        b: Point { x: 349, y: 500 },
    },
    Line {
        a: Point { x: 280, y: 959 },
        b: Point { x: 259, y: 959 },
    },
    Line {
        a: Point { x: 973, y: 637 },
        b: Point { x: 832, y: 637 },
    },
    Line {
        a: Point { x: 161, y: 255 },
        b: Point { x: 979, y: 255 },
    },
    Line {
        a: Point { x: 512, y: 826 },
        b: Point { x: 149, y: 826 },
    },
    Line {
        a: Point { x: 308, y: 769 },
        b: Point { x: 22, y: 769 },
    },
    Line {
        a: Point { x: 60, y: 692 },
        b: Point { x: 60, y: 262 },
    },
    Line {
        a: Point { x: 787, y: 31 },
        b: Point { x: 753, y: 31 },
    },
    Line {
        a: Point { x: 932, y: 166 },
        b: Point { x: 932, y: 127 },
    },
    Line {
        a: Point { x: 514, y: 77 },
        b: Point { x: 514, y: 646 },
    },
    Line {
        a: Point { x: 535, y: 434 },
        b: Point { x: 535, y: 979 },
    },
    Line {
        a: Point { x: 838, y: 799 },
        b: Point { x: 838, y: 332 },
    },
    Line {
        a: Point { x: 565, y: 956 },
        b: Point { x: 565, y: 477 },
    },
    Line {
        a: Point { x: 74, y: 195 },
        b: Point { x: 274, y: 195 },
    },
    Line {
        a: Point { x: 916, y: 715 },
        b: Point { x: 907, y: 715 },
    },
    Line {
        a: Point { x: 721, y: 655 },
        b: Point { x: 721, y: 542 },
    },
    Line {
        a: Point { x: 180, y: 784 },
        b: Point { x: 928, y: 784 },
    },
    Line {
        a: Point { x: 16, y: 128 },
        b: Point { x: 313, y: 128 },
    },
    Line {
        a: Point { x: 23, y: 330 },
        b: Point { x: 23, y: 704 },
    },
    Line {
        a: Point { x: 530, y: 723 },
        b: Point { x: 530, y: 88 },
    },
    Line {
        a: Point { x: 869, y: 272 },
        b: Point { x: 765, y: 376 },
    },
    Line {
        a: Point { x: 878, y: 185 },
        b: Point { x: 353, y: 185 },
    },
    Line {
        a: Point { x: 72, y: 800 },
        b: Point { x: 514, y: 800 },
    },
    Line {
        a: Point { x: 319, y: 117 },
        b: Point { x: 307, y: 117 },
    },
    Line {
        a: Point { x: 436, y: 405 },
        b: Point { x: 496, y: 345 },
    },
    Line {
        a: Point { x: 327, y: 459 },
        b: Point { x: 641, y: 145 },
    },
    Line {
        a: Point { x: 358, y: 309 },
        b: Point { x: 661, y: 612 },
    },
    Line {
        a: Point { x: 60, y: 225 },
        b: Point { x: 811, y: 976 },
    },
    Line {
        a: Point { x: 113, y: 130 },
        b: Point { x: 794, y: 130 },
    },
    Line {
        a: Point { x: 559, y: 950 },
        b: Point { x: 32, y: 423 },
    },
    Line {
        a: Point { x: 626, y: 110 },
        b: Point { x: 626, y: 319 },
    },
    Line {
        a: Point { x: 50, y: 39 },
        b: Point { x: 989, y: 978 },
    },
    Line {
        a: Point { x: 257, y: 627 },
        b: Point { x: 799, y: 627 },
    },
    Line {
        a: Point { x: 581, y: 843 },
        b: Point { x: 581, y: 493 },
    },
    Line {
        a: Point { x: 869, y: 18 },
        b: Point { x: 208, y: 18 },
    },
    Line {
        a: Point { x: 184, y: 395 },
        b: Point { x: 184, y: 263 },
    },
    Line {
        a: Point { x: 454, y: 888 },
        b: Point { x: 165, y: 599 },
    },
    Line {
        a: Point { x: 637, y: 920 },
        b: Point { x: 637, y: 544 },
    },
    Line {
        a: Point { x: 170, y: 982 },
        b: Point { x: 273, y: 982 },
    },
    Line {
        a: Point { x: 98, y: 354 },
        b: Point { x: 668, y: 924 },
    },
    Line {
        a: Point { x: 32, y: 409 },
        b: Point { x: 32, y: 925 },
    },
    Line {
        a: Point { x: 154, y: 175 },
        b: Point { x: 273, y: 294 },
    },
    Line {
        a: Point { x: 425, y: 896 },
        b: Point { x: 870, y: 451 },
    },
    Line {
        a: Point { x: 198, y: 319 },
        b: Point { x: 615, y: 736 },
    },
    Line {
        a: Point { x: 170, y: 582 },
        b: Point { x: 170, y: 712 },
    },
    Line {
        a: Point { x: 141, y: 645 },
        b: Point { x: 141, y: 639 },
    },
    Line {
        a: Point { x: 482, y: 768 },
        b: Point { x: 486, y: 768 },
    },
    Line {
        a: Point { x: 940, y: 969 },
        b: Point { x: 24, y: 53 },
    },
    Line {
        a: Point { x: 680, y: 360 },
        b: Point { x: 959, y: 360 },
    },
    Line {
        a: Point { x: 315, y: 905 },
        b: Point { x: 315, y: 96 },
    },
    Line {
        a: Point { x: 22, y: 666 },
        b: Point { x: 22, y: 247 },
    },
    Line {
        a: Point { x: 722, y: 40 },
        b: Point { x: 722, y: 714 },
    },
    Line {
        a: Point { x: 585, y: 31 },
        b: Point { x: 585, y: 21 },
    },
    Line {
        a: Point { x: 479, y: 254 },
        b: Point { x: 307, y: 254 },
    },
    Line {
        a: Point { x: 291, y: 182 },
        b: Point { x: 291, y: 855 },
    },
    Line {
        a: Point { x: 684, y: 698 },
        b: Point { x: 402, y: 698 },
    },
    Line {
        a: Point { x: 20, y: 984 },
        b: Point { x: 988, y: 16 },
    },
    Line {
        a: Point { x: 256, y: 424 },
        b: Point { x: 17, y: 663 },
    },
    Line {
        a: Point { x: 825, y: 380 },
        b: Point { x: 820, y: 385 },
    },
    Line {
        a: Point { x: 254, y: 622 },
        b: Point { x: 254, y: 315 },
    },
    Line {
        a: Point { x: 98, y: 855 },
        b: Point { x: 98, y: 694 },
    },
    Line {
        a: Point { x: 220, y: 719 },
        b: Point { x: 220, y: 117 },
    },
    Line {
        a: Point { x: 615, y: 653 },
        b: Point { x: 656, y: 694 },
    },
    Line {
        a: Point { x: 427, y: 12 },
        b: Point { x: 427, y: 745 },
    },
    Line {
        a: Point { x: 20, y: 64 },
        b: Point { x: 828, y: 872 },
    },
    Line {
        a: Point { x: 739, y: 203 },
        b: Point { x: 434, y: 203 },
    },
    Line {
        a: Point { x: 546, y: 576 },
        b: Point { x: 130, y: 160 },
    },
    Line {
        a: Point { x: 730, y: 835 },
        b: Point { x: 299, y: 835 },
    },
    Line {
        a: Point { x: 927, y: 512 },
        b: Point { x: 927, y: 586 },
    },
    Line {
        a: Point { x: 411, y: 192 },
        b: Point { x: 868, y: 192 },
    },
    Line {
        a: Point { x: 917, y: 630 },
        b: Point { x: 678, y: 630 },
    },
    Line {
        a: Point { x: 620, y: 588 },
        b: Point { x: 620, y: 26 },
    },
    Line {
        a: Point { x: 786, y: 488 },
        b: Point { x: 486, y: 488 },
    },
    Line {
        a: Point { x: 746, y: 640 },
        b: Point { x: 251, y: 145 },
    },
    Line {
        a: Point { x: 585, y: 556 },
        b: Point { x: 585, y: 119 },
    },
    Line {
        a: Point { x: 977, y: 202 },
        b: Point { x: 762, y: 202 },
    },
    Line {
        a: Point { x: 587, y: 244 },
        b: Point { x: 587, y: 877 },
    },
    Line {
        a: Point { x: 693, y: 479 },
        b: Point { x: 693, y: 859 },
    },
    Line {
        a: Point { x: 59, y: 816 },
        b: Point { x: 59, y: 475 },
    },
    Line {
        a: Point { x: 191, y: 941 },
        b: Point { x: 878, y: 254 },
    },
    Line {
        a: Point { x: 150, y: 920 },
        b: Point { x: 926, y: 144 },
    },
    Line {
        a: Point { x: 856, y: 397 },
        b: Point { x: 856, y: 739 },
    },
    Line {
        a: Point { x: 380, y: 965 },
        b: Point { x: 549, y: 796 },
    },
    Line {
        a: Point { x: 637, y: 323 },
        b: Point { x: 909, y: 595 },
    },
    Line {
        a: Point { x: 848, y: 219 },
        b: Point { x: 304, y: 763 },
    },
    Line {
        a: Point { x: 123, y: 146 },
        b: Point { x: 589, y: 146 },
    },
    Line {
        a: Point { x: 546, y: 122 },
        b: Point { x: 651, y: 122 },
    },
    Line {
        a: Point { x: 131, y: 711 },
        b: Point { x: 814, y: 28 },
    },
    Line {
        a: Point { x: 727, y: 274 },
        b: Point { x: 296, y: 274 },
    },
    Line {
        a: Point { x: 101, y: 546 },
        b: Point { x: 479, y: 168 },
    },
    Line {
        a: Point { x: 508, y: 517 },
        b: Point { x: 615, y: 410 },
    },
    Line {
        a: Point { x: 492, y: 115 },
        b: Point { x: 492, y: 250 },
    },
    Line {
        a: Point { x: 212, y: 65 },
        b: Point { x: 770, y: 623 },
    },
    Line {
        a: Point { x: 118, y: 938 },
        b: Point { x: 857, y: 199 },
    },
    Line {
        a: Point { x: 623, y: 843 },
        b: Point { x: 98, y: 843 },
    },
    Line {
        a: Point { x: 86, y: 153 },
        b: Point { x: 701, y: 768 },
    },
    Line {
        a: Point { x: 81, y: 98 },
        b: Point { x: 81, y: 604 },
    },
    Line {
        a: Point { x: 173, y: 313 },
        b: Point { x: 173, y: 533 },
    },
    Line {
        a: Point { x: 792, y: 396 },
        b: Point { x: 792, y: 242 },
    },
    Line {
        a: Point { x: 975, y: 985 },
        b: Point { x: 10, y: 20 },
    },
    Line {
        a: Point { x: 762, y: 661 },
        b: Point { x: 726, y: 661 },
    },
    Line {
        a: Point { x: 216, y: 327 },
        b: Point { x: 216, y: 122 },
    },
    Line {
        a: Point { x: 446, y: 658 },
        b: Point { x: 98, y: 658 },
    },
    Line {
        a: Point { x: 85, y: 184 },
        b: Point { x: 314, y: 184 },
    },
    Line {
        a: Point { x: 165, y: 750 },
        b: Point { x: 313, y: 750 },
    },
    Line {
        a: Point { x: 729, y: 583 },
        b: Point { x: 729, y: 640 },
    },
    Line {
        a: Point { x: 382, y: 36 },
        b: Point { x: 382, y: 326 },
    },
    Line {
        a: Point { x: 487, y: 32 },
        b: Point { x: 225, y: 32 },
    },
    Line {
        a: Point { x: 389, y: 722 },
        b: Point { x: 582, y: 915 },
    },
    Line {
        a: Point { x: 954, y: 965 },
        b: Point { x: 86, y: 965 },
    },
    Line {
        a: Point { x: 747, y: 376 },
        b: Point { x: 747, y: 96 },
    },
    Line {
        a: Point { x: 254, y: 259 },
        b: Point { x: 254, y: 482 },
    },
    Line {
        a: Point { x: 149, y: 256 },
        b: Point { x: 149, y: 871 },
    },
    Line {
        a: Point { x: 893, y: 207 },
        b: Point { x: 708, y: 22 },
    },
    Line {
        a: Point { x: 195, y: 907 },
        b: Point { x: 195, y: 82 },
    },
    Line {
        a: Point { x: 342, y: 686 },
        b: Point { x: 457, y: 571 },
    },
    Line {
        a: Point { x: 647, y: 469 },
        b: Point { x: 468, y: 469 },
    },
    Line {
        a: Point { x: 150, y: 525 },
        b: Point { x: 832, y: 525 },
    },
    Line {
        a: Point { x: 90, y: 907 },
        b: Point { x: 90, y: 31 },
    },
    Line {
        a: Point { x: 389, y: 554 },
        b: Point { x: 389, y: 318 },
    },
    Line {
        a: Point { x: 138, y: 327 },
        b: Point { x: 138, y: 310 },
    },
    Line {
        a: Point { x: 861, y: 126 },
        b: Point { x: 861, y: 549 },
    },
    Line {
        a: Point { x: 355, y: 583 },
        b: Point { x: 355, y: 534 },
    },
    Line {
        a: Point { x: 591, y: 182 },
        b: Point { x: 181, y: 592 },
    },
    Line {
        a: Point { x: 73, y: 84 },
        b: Point { x: 897, y: 908 },
    },
    Line {
        a: Point { x: 326, y: 989 },
        b: Point { x: 425, y: 989 },
    },
    Line {
        a: Point { x: 835, y: 688 },
        b: Point { x: 724, y: 799 },
    },
    Line {
        a: Point { x: 844, y: 493 },
        b: Point { x: 844, y: 974 },
    },
    Line {
        a: Point { x: 172, y: 436 },
        b: Point { x: 172, y: 12 },
    },
    Line {
        a: Point { x: 536, y: 933 },
        b: Point { x: 48, y: 445 },
    },
    Line {
        a: Point { x: 192, y: 531 },
        b: Point { x: 287, y: 531 },
    },
    Line {
        a: Point { x: 286, y: 547 },
        b: Point { x: 80, y: 547 },
    },
    Line {
        a: Point { x: 929, y: 795 },
        b: Point { x: 697, y: 795 },
    },
    Line {
        a: Point { x: 790, y: 681 },
        b: Point { x: 433, y: 681 },
    },
    Line {
        a: Point { x: 692, y: 229 },
        b: Point { x: 731, y: 229 },
    },
    Line {
        a: Point { x: 377, y: 667 },
        b: Point { x: 14, y: 304 },
    },
    Line {
        a: Point { x: 535, y: 226 },
        b: Point { x: 116, y: 645 },
    },
    Line {
        a: Point { x: 338, y: 861 },
        b: Point { x: 338, y: 343 },
    },
    Line {
        a: Point { x: 668, y: 160 },
        b: Point { x: 853, y: 160 },
    },
    Line {
        a: Point { x: 188, y: 157 },
        b: Point { x: 667, y: 636 },
    },
    Line {
        a: Point { x: 62, y: 934 },
        b: Point { x: 951, y: 45 },
    },
    Line {
        a: Point { x: 948, y: 820 },
        b: Point { x: 978, y: 820 },
    },
    Line {
        a: Point { x: 860, y: 884 },
        b: Point { x: 157, y: 884 },
    },
    Line {
        a: Point { x: 794, y: 251 },
        b: Point { x: 783, y: 251 },
    },
    Line {
        a: Point { x: 317, y: 381 },
        b: Point { x: 591, y: 655 },
    },
    Line {
        a: Point { x: 459, y: 876 },
        b: Point { x: 459, y: 307 },
    },
    Line {
        a: Point { x: 146, y: 822 },
        b: Point { x: 903, y: 65 },
    },
    Line {
        a: Point { x: 374, y: 739 },
        b: Point { x: 891, y: 739 },
    },
    Line {
        a: Point { x: 619, y: 575 },
        b: Point { x: 973, y: 929 },
    },
    Line {
        a: Point { x: 544, y: 351 },
        b: Point { x: 544, y: 124 },
    },
    Line {
        a: Point { x: 300, y: 335 },
        b: Point { x: 818, y: 335 },
    },
    Line {
        a: Point { x: 158, y: 220 },
        b: Point { x: 418, y: 480 },
    },
    Line {
        a: Point { x: 107, y: 953 },
        b: Point { x: 988, y: 953 },
    },
    Line {
        a: Point { x: 304, y: 753 },
        b: Point { x: 543, y: 753 },
    },
    Line {
        a: Point { x: 948, y: 95 },
        b: Point { x: 140, y: 903 },
    },
    Line {
        a: Point { x: 832, y: 451 },
        b: Point { x: 526, y: 145 },
    },
    Line {
        a: Point { x: 966, y: 34 },
        b: Point { x: 402, y: 598 },
    },
    Line {
        a: Point { x: 72, y: 123 },
        b: Point { x: 716, y: 123 },
    },
    Line {
        a: Point { x: 336, y: 294 },
        b: Point { x: 84, y: 294 },
    },
    Line {
        a: Point { x: 116, y: 605 },
        b: Point { x: 116, y: 889 },
    },
    Line {
        a: Point { x: 700, y: 742 },
        b: Point { x: 700, y: 217 },
    },
    Line {
        a: Point { x: 551, y: 554 },
        b: Point { x: 973, y: 554 },
    },
    Line {
        a: Point { x: 684, y: 181 },
        b: Point { x: 66, y: 799 },
    },
    Line {
        a: Point { x: 86, y: 949 },
        b: Point { x: 86, y: 173 },
    },
    Line {
        a: Point { x: 834, y: 361 },
        b: Point { x: 834, y: 942 },
    },
    Line {
        a: Point { x: 508, y: 668 },
        b: Point { x: 627, y: 549 },
    },
    Line {
        a: Point { x: 213, y: 695 },
        b: Point { x: 704, y: 695 },
    },
    Line {
        a: Point { x: 260, y: 979 },
        b: Point { x: 868, y: 371 },
    },
    Line {
        a: Point { x: 825, y: 435 },
        b: Point { x: 825, y: 67 },
    },
    Line {
        a: Point { x: 956, y: 854 },
        b: Point { x: 66, y: 854 },
    },
    Line {
        a: Point { x: 390, y: 444 },
        b: Point { x: 697, y: 444 },
    },
    Line {
        a: Point { x: 360, y: 450 },
        b: Point { x: 720, y: 810 },
    },
    Line {
        a: Point { x: 153, y: 514 },
        b: Point { x: 794, y: 514 },
    },
    Line {
        a: Point { x: 253, y: 261 },
        b: Point { x: 253, y: 298 },
    },
    Line {
        a: Point { x: 925, y: 679 },
        b: Point { x: 925, y: 499 },
    },
    Line {
        a: Point { x: 391, y: 282 },
        b: Point { x: 441, y: 282 },
    },
    Line {
        a: Point { x: 86, y: 366 },
        b: Point { x: 779, y: 366 },
    },
    Line {
        a: Point { x: 687, y: 312 },
        b: Point { x: 687, y: 629 },
    },
    Line {
        a: Point { x: 304, y: 172 },
        b: Point { x: 732, y: 600 },
    },
    Line {
        a: Point { x: 571, y: 518 },
        b: Point { x: 263, y: 518 },
    },
    Line {
        a: Point { x: 814, y: 252 },
        b: Point { x: 118, y: 252 },
    },
    Line {
        a: Point { x: 108, y: 920 },
        b: Point { x: 108, y: 162 },
    },
    Line {
        a: Point { x: 154, y: 965 },
        b: Point { x: 928, y: 191 },
    },
    Line {
        a: Point { x: 635, y: 875 },
        b: Point { x: 635, y: 947 },
    },
    Line {
        a: Point { x: 986, y: 31 },
        b: Point { x: 47, y: 970 },
    },
    Line {
        a: Point { x: 746, y: 35 },
        b: Point { x: 746, y: 636 },
    },
    Line {
        a: Point { x: 735, y: 849 },
        b: Point { x: 334, y: 448 },
    },
    Line {
        a: Point { x: 826, y: 510 },
        b: Point { x: 906, y: 590 },
    },
    Line {
        a: Point { x: 834, y: 745 },
        b: Point { x: 834, y: 949 },
    },
    Line {
        a: Point { x: 843, y: 401 },
        b: Point { x: 564, y: 122 },
    },
    Line {
        a: Point { x: 179, y: 212 },
        b: Point { x: 179, y: 32 },
    },
    Line {
        a: Point { x: 354, y: 906 },
        b: Point { x: 233, y: 906 },
    },
    Line {
        a: Point { x: 593, y: 439 },
        b: Point { x: 196, y: 42 },
    },
    Line {
        a: Point { x: 707, y: 446 },
        b: Point { x: 242, y: 446 },
    },
    Line {
        a: Point { x: 511, y: 84 },
        b: Point { x: 511, y: 406 },
    },
    Line {
        a: Point { x: 109, y: 299 },
        b: Point { x: 100, y: 290 },
    },
    Line {
        a: Point { x: 410, y: 79 },
        b: Point { x: 410, y: 784 },
    },
    Line {
        a: Point { x: 806, y: 923 },
        b: Point { x: 54, y: 171 },
    },
    Line {
        a: Point { x: 592, y: 83 },
        b: Point { x: 592, y: 189 },
    },
    Line {
        a: Point { x: 413, y: 28 },
        b: Point { x: 413, y: 469 },
    },
    Line {
        a: Point { x: 17, y: 844 },
        b: Point { x: 17, y: 691 },
    },
    Line {
        a: Point { x: 130, y: 419 },
        b: Point { x: 205, y: 344 },
    },
    Line {
        a: Point { x: 374, y: 247 },
        b: Point { x: 849, y: 247 },
    },
    Line {
        a: Point { x: 650, y: 344 },
        b: Point { x: 653, y: 344 },
    },
    Line {
        a: Point { x: 563, y: 942 },
        b: Point { x: 563, y: 726 },
    },
    Line {
        a: Point { x: 771, y: 966 },
        b: Point { x: 450, y: 966 },
    },
    Line {
        a: Point { x: 499, y: 693 },
        b: Point { x: 788, y: 693 },
    },
    Line {
        a: Point { x: 962, y: 458 },
        b: Point { x: 962, y: 356 },
    },
    Line {
        a: Point { x: 28, y: 683 },
        b: Point { x: 765, y: 683 },
    },
    Line {
        a: Point { x: 432, y: 546 },
        b: Point { x: 432, y: 708 },
    },
    Line {
        a: Point { x: 519, y: 974 },
        b: Point { x: 176, y: 974 },
    },
    Line {
        a: Point { x: 797, y: 744 },
        b: Point { x: 280, y: 227 },
    },
    Line {
        a: Point { x: 505, y: 228 },
        b: Point { x: 547, y: 228 },
    },
    Line {
        a: Point { x: 401, y: 366 },
        b: Point { x: 401, y: 754 },
    },
    Line {
        a: Point { x: 356, y: 470 },
        b: Point { x: 123, y: 470 },
    },
    Line {
        a: Point { x: 57, y: 909 },
        b: Point { x: 229, y: 909 },
    },
    Line {
        a: Point { x: 343, y: 880 },
        b: Point { x: 539, y: 880 },
    },
    Line {
        a: Point { x: 221, y: 851 },
        b: Point { x: 221, y: 297 },
    },
    Line {
        a: Point { x: 520, y: 677 },
        b: Point { x: 894, y: 677 },
    },
    Line {
        a: Point { x: 216, y: 805 },
        b: Point { x: 688, y: 805 },
    },
    Line {
        a: Point { x: 158, y: 901 },
        b: Point { x: 847, y: 901 },
    },
    Line {
        a: Point { x: 98, y: 129 },
        b: Point { x: 98, y: 969 },
    },
    Line {
        a: Point { x: 793, y: 203 },
        b: Point { x: 210, y: 786 },
    },
    Line {
        a: Point { x: 852, y: 855 },
        b: Point { x: 135, y: 138 },
    },
    Line {
        a: Point { x: 944, y: 90 },
        b: Point { x: 103, y: 931 },
    },
    Line {
        a: Point { x: 691, y: 768 },
        b: Point { x: 583, y: 768 },
    },
    Line {
        a: Point { x: 784, y: 617 },
        b: Point { x: 637, y: 764 },
    },
    Line {
        a: Point { x: 222, y: 160 },
        b: Point { x: 819, y: 757 },
    },
    Line {
        a: Point { x: 145, y: 982 },
        b: Point { x: 145, y: 216 },
    },
    Line {
        a: Point { x: 837, y: 355 },
        b: Point { x: 99, y: 355 },
    },
    Line {
        a: Point { x: 324, y: 121 },
        b: Point { x: 324, y: 14 },
    },
    Line {
        a: Point { x: 773, y: 851 },
        b: Point { x: 773, y: 413 },
    },
    Line {
        a: Point { x: 778, y: 550 },
        b: Point { x: 686, y: 458 },
    },
    Line {
        a: Point { x: 81, y: 56 },
        b: Point { x: 338, y: 313 },
    },
    Line {
        a: Point { x: 356, y: 512 },
        b: Point { x: 356, y: 441 },
    },
];
