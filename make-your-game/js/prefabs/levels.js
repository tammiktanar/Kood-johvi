export let gameLevels = []

let template = [
    ".....................",
    ".....................",
    ".....................",
    ".....................",
    ".....................",
    ".....................",
    ".....................",
    ".....................",
    ".....................",
    ".....................",
    ".....................",
    ".....................",
    ".....................",
    ".....................",
    ".....................",
    ".....................",
]

/*
 *      A - Takes 2 to break
 *      P - Power up brick
 *      D - Debuff brick
 *      M - Moving brick
 *      X - Unbreakable brick
 *      R - Regenerative brick
 *      | - Lowers above brick
 *      / - Lowers above brick, while also adding health
 *      _ - Extends previous brick
 *      + - Extends previous brick, while also adding health
 *
 *      --- Colors ---
 *
 *      b - Blue brick
 *      g - Green brick
 *      r - Red brick
 *      y - Yellow brick
 *      p - Pink brick
 *      w - White brick
 *      h - Grey brick
 *      o - Orange brick
 *
 *
 */

const olari1 = [
	".....................",
	".....................",
	"b_b_b_b_b_b_b_b_b_b_b",
	"gg_g_g_g_g_g_g_g_g_g_",
	"b_b_b_b_b_b_b_b_b_b_b",
	"gg_g_g_g_g_g_g_g_g_g_",
	".....................",
	".....................",
	"r_r_r_r_r_r_r_r_r_r_r",
	"yy_y_y_y_y_y_y_y_y_y_",
	"r_r_r_r_r_r_r_r_r_r_r",
	"yy_y_y_y_y_y_y_y_y_y_",
	".....................",
	".....................",
	".....................",
	".....................",
]

const olari2 = [
	".....................",
	".....................",
	".....................",
	".h+h+h+h+h+_h+h+h+h+.",
	".hgg___rgg___rgg___h.",
	"./|bb_g||bb_g||bb_g/.",
	".h||pb|r||pb|r||pb|h.",
	"./||p|||||p|||||p||/.",
	".h|b_||r|b_||r|b_||h.",
	"./g___||g___||g___|/.",
	".h+h+h+h+h+_h+h+h+h+.",
	".....................",
	".....................",
	".....................",
	".....................",
	".....................",
]

const olari3 = [
	".....................",
	".....................",
	".....................",
	"...X....X...Xy___X...",
	"...|b___|...|y___|...",
	"...Xp_p_X...Xo___X...",
	"...|p_p_|...|o___|...",
	"...Xp_p_X...Xr___X...",
	"...|b___|...|r___|...",
	"h+_X_X_X_h+_X_X_X_h+_",
	".....................",
	".....................",
	".....................",
	".....................",
	".....................",
	".....................",
]

const olari4 = [
	".....................",
	".....................",
	"X_X_X_X_X_X_X_X_X_X_X",
	".......h_h_h_h_h_wby|",
	"................h|||X",
	"................|wgp|",
	".......h_h_h_h_h_|||X",
	".....XX_X_X_X_X_X_X_|",
	".....|.......wh......",
	".....Xo_o_o_.h|......",
	".....|o_o_o_.|h......",
	".....X.......w|......",
	".....|X_X_X_X_X_.....",
	".....................",
	".....................",
	".....................",
]
const olari5 = [
	".....................",
	".....................",
	"..b_....hh__h....b_..",
	".b_....h|rrr|h....b_.",
	"..b_...|wrrrw|...b_..",
	".b_....h|w__|h....b_.",
	"..b_...|wooow|...b_..",
	".b_....h|||||h....b_.",
	"..b_...|w|||w|...b_..",
	".b_....h|y|y|h....b_.",
	"..b_...|.|y|.|...b_..",
	".b_.......|.......b_.",
	"..b_.............b_..",
	".b_...............b_.",
	".....................",
	".....................",
]

let colorPalet = [
    "AA.PP.DD.MM.XX.RR....",
    "AA.PP.DD.MM.XX.RR....",
    ".....................",
    ".A..P..D..M..X..R....",
    ".....................",
    ".....................",
    ".....................",
    "bb.rr.gg.yy.pp.ww.hh.",
    "bb.rr.gg.yy.pp.ww.hh.",
    ".....................",
    "oo...................",
    "oo...................",
    ".....................",
    ".b.r.g.y.p.w.h.o.....",
    ".....................",
    "X____________________",
]

let level0 = [
    ".....................",
    "..r.r....r.r....r.r..",
    ".r...r..r...r..r...r.",
    ".r...r..r...r..r...r.",
    ".....................",
    ".r...r..r...r..r...r.",
    "..r.r....r.r....r.r..",
    "..r.r....r.r....r.r..",
    ".r...r..r...r..r...r.",
    ".r...r..r...r..r...r.",
    ".....................",
    ".r...r..r...r..r...r.",
    "..r.r....r.r....r.r..",
	".....................",
]

let level1 = [
    ".....................",
    "...r......r......r...",
    "..rgr....rgr....rgr..",
    ".rgbgr..rgbgr..rgbgr.",
    "..rgr....rgr....rgr..",
    "...r......r......r...",
	".....................",
	"...r......r......r...",
    "..rgr....rgr....rgr..",
    ".rgbgr..rgbgr..rgbgr.",
    "..rgr....rgr....rgr..",
    "...r......r......r...",
	".....................",
]

let level2 = [
    ".rr.rr..rr.rr..rr.rr.",
    ".rr.rr..rr.rr..rr.rr.",
    ".....................",
    ".bb.bb..bb.bb..bb.bb.",
    ".bb.bb..bb.bb..bb.bb.",
    ".....................",
    ".gg.gg..gg.gg..gg.gg.",
    ".gg.gg..gg.gg..gg.gg.",
    ".rr.rr..rr.rr..rr.rr.",
    ".rr.rr..rr.rr..rr.rr.",
    ".....................",
    ".bb.bb..bb.bb..bb.bb.",
    ".bb.bb..bb.bb..bb.bb.",
    ".....................",
    ".gg.gg..gg.gg..gg.gg.",
    ".gg.gg..gg.gg..gg.gg.",

]

let level3 = [
    "r...r..",
    "...r...",
    "..r...r",
    ".r...r.",
    "r...r..",
    "...r...",
    "..r...r",
    ".r...r.",
    "r...r..",
    "...r...",
]

let level4 = [
    "MMMMMMMMMMMMMMMMMMMMM",
    "MMMMMMMMMMMMMMMMMMMMM",
    "MMMMMMMMMMMMMMMMMMMMM",
    "MMMMMMMMMMMMMMMMMMMMM",
    "MMMMMMMMMMMMMMMMMMMMM",
    "MMMMMMMMMMMMMMMMMMMMM",
    "MMMMMMMMMMMMMMMMMMMMM",
    "MMMMMMMMMMMMMMMMMMMMM",
    "MMMMMMMMMMMMMMMMMMMMM",
    "MMMMMMMMMMMMMMMMMMMMM",
    "MMMMMMMMMMMMMMMMMMMMM",
    "MMMMMMMMMMMMMMMMMMMMM",
    "MMMMMMMMMMMMMMMMMMMMM",
    "MMMMMMMMMMMMMMMMMMMMM",
    "MMMMMMMMMMMMMMMMMMMMM",
    "MMMMMMMMMMMMMMMMMMMMM",
]

let level5 = [
    "......M......M......M",
    ".....M......M......M.",
    "....M......M......M..",
    "...M......M......M...",
    "..M......M......M....",
    ".M......M......M.....",
    "M......M......M......",
    ".M......M......M.....",
    "..M......M......M....",
    "...M......M......M...",
    "....M......M......M..",
    ".....M......M......M.",
    "......M......M......M",
    "M......M......M......",
    ".M......M......M.....",
    "..M......M......M....",
]

let level6 = [
    ".....................",
    ".........hhhh........",
    ".......hhrrrrhh......",
    "......hrrrrrrrrh.....",
    ".....hrrwrrrrrrhh....",
    ".....hrwrrrrrrrrh....",
    "....hrrrrrrrrrrrrh...",
    "....hhhrrrrhhhrrrh...",
    "....hhhhrrhwwwhrrh...",
    "....hwwwhhhwwwhhhh...",
    ".....hwwwhhwwwhwh....",
    ".....hwwwwwhhhwwh....",
    "......hwwwwwwwwh.....",
    ".......hhwwwwhh......",
    ".........hhhh........",
    ".....................",
]

let level7 = [
    ".....................",
    ".....................",
    ".......hhhhh.........",
    "......hrrrrrh........",
    ".....hrrrrrrrh.......",
    ".....hhhhrrrrhhh.....",
    "....hbwbbhrrrhrrh....",
    "....hbbbbhrrrhrrh....",
    ".....hhhhrrrrhrrh....",
    ".....hrrrrrrrhrrh....",
    ".....hrrrrrrrhrrh....",
    ".....hrrrhrrrhhh.....",
    ".....hrrh.hrrh.......",
    ".....hrrh.hrrh.......",
    "......hh...hh........",
    ".....................",
]

let level8 = [
    "bbbbbbb",
    "bbbbbbb",
    "bbbbbbb",
    "bbbbbbb",
    "bbbbbbb",
    "bbbbbbb",
    "bbbbbbb",
    "bbbbbbb",
]
let level9 = [
    "M...M..",
    "...M...",
    "..M...M",
    ".M...M.",
    "M...M..",
    "...M...",
    "..M...M",
    ".M...M.",
    "M...M..",
    "...M...",
]

let level10 = [
    "XXXXXXX",
    "M..X...",
    ".X.X.X.",
    "./.X.X.",
    "./.X.X.",
    "./.X.X.",
    ".|.X.X.",
    ".|.X.X.",
    "r|...X.",
    "R+++++.",
];

let level11 = [
    "..................................",
    "..bgrypwhoAAAo__bA_A_AAA+____AAA..",
    "..A.A__A.AA|Ah_g_P_A_AA|/||||AAA..",
    "..AAA__A.AA|Aw_r_D_A_AA|/||||AAA..",
    "..APDXRAAAA|Ap_y_X_R_AAA++++++++..",
    "..................................",
];

let level12 = [
	"r",
]

//levelid
let klevel1 = [
    ".....................",
    ".....................",
    ".....................",
    "bbbbb...........bbbbb",
    "bbbbb...........bbbbb",
    ".....................",
    "...XrrrrrrrrrrrrrX...",
    "...XbbbbbbbbbbbbbX...",
    "...XgggggggggggggX...",
    "...XyyyyyyyyyyyyyX...",
    "...XXXXXXXXXXXXXXX...",
    ".....................",
    ".....................",
    ".....................",
    ".....................",
    ".....................",
]

let klevel2 = [
    ".....................",
    ".....................",
    "....XyyyXXXXXXXXX....",
    "....XrrrrrrrroooX....",
    "....XrrrrrrrroooX....",
    "....XpppXXXXXoooX....",
    "....XpppXbbbXoooX....",
    "....XpppXXbbXoooX....",
    "....XpppbbbbXoooX....",
    "....XpppbbbbXoooX....",
    "....XXXXXXXXXoooX....",
    ".....................",
    "yyyyy...........yyyyy",
    ".....................",
    ".....................",
    ".....................",
]

let klevel3 = [
    ".....................",
    ".....................",
    ".....................",
    "...XrrrrrrrrrrrrrX...",
    "....XrrrrrrrrrrrX....",
    ".y...XyyyyyyyyyX...y.",
    ".yy...XyyyyyyyX...yy.",
    ".ggg...XgggggX...ggg.",
    ".gggg...XgggX...gggg.",
    ".ggggg...XgX...ggggg.",
    ".....................",
    ".....................",
    "XXXX......X......XXXX",
    ".....................",
    ".....................",
    ".....................",
]

let klevel4 = [
    "wbwbwbwbwbwr_________",
    "bwbwbwbwbwbw_________",
    "wbwbwbwbwbwr_________",
    "bwbwbwbwbwbw_________",
    "wbwbwbwbwbwr_________",
    "bwbwbwbwbwbw_________",
    "wbwbwbwbwbwr_________",
    "bwbwbwbwbwbw_________",
    "wbwbwbwbwbwr_________",
    "w____________________",
    "r____________________",
    "w____________________",
    "r____________________",
    "w____________________",
    "r____________________",
    ".....................",
]

let klevel5 = [
    ".....................",
    "..................ggg",
    ".................ggg.",
    "................ggg..",
    "...............ggg...",
    "..............ggg....",
    "..g....g.....ggg.....",
    ".ggg..ggg...ggg......",
    "..g....g...ggg.......",
    ".....................",
    ".XXX..XXX..XXX..XXX..",
    ".....................",
    ".....................",
    ".....................",
    ".....................",
    ".....................",
]

let tanarLevel1 = [
    ".....................",
    ".........hhhh........",
    ".......hhrrrrhh......",
    "......hrrrrrrrrh.....",
    ".....hrrwrrrrrrhh....",
    ".....hrwrrrrrrrrh....",
    "....hrrrrrrrrrrrrh...",
    "....hhhrrrrhhhrrrh...",
    "....hhhhrrhwwwhrrh...",
    "....hwwwhhhwwwhhhh...",
    ".....hwwwhhwwwhwh....",
    ".....hwwwwwhhhwwh....",
    "......hwwwwwwwwh.....",
    ".......hhwwwwhh......",
    ".........hhhh........",
    ".....................",
]


let tanarLevel2 = [
    ".....................",
    ".....................",
    ".......hhhhh.........",
    "......hrrrrrh........",
    ".....hrrrrrrrh.......",
    ".....hhhhrrrrhhh.....",
    "....hbwbbhrrrhrrh....",
    "....hbbbbhrrrhrrh....",
    ".....hhhhrrrrhrrh....",
    ".....hrrrrrrrhrrh....",
    ".....hrrrrrrrhrrh....",
    ".....hrrrhrrrhhh.....",
    ".....hrrh.hrrh.......",
    ".....hrrh.hrrh.......",
    "......hh...hh........",
    ".....................",
]


let tanarLevel3 = [
    ".....................",
    ".....................",
    "..r.r....r.r....r.r..",
    ".r...r..r...r..r...r.",
    ".r...r..r...r..r...r.",
    ".....................",
    ".r...r..r...r..r...r.",
    "..r.r....r.r....r.r..",
    "..r.r....r.r....r.r..",
    ".r...r..r...r..r...r.",
    ".r...r..r...r..r...r.",
    ".....................",
    ".r...r..r...r..r...r.",
    "..r.r....r.r....r.r..",
    ".....................",
    ".....................",
]


let tanarLevel4 = [
    ".....................",
    ".....................",
    "...r......r......r...",
    "..rgr....rgr....rgr..",
    ".rgbgr..rgbgr..rgbgr.",
    "..rgr....rgr....rgr..",
    "...r......r......r...",
    ".....................",
    ".....................",
    "...r......r......r...",
    "..rgr....rgr....rgr..",
    ".rgbgr..rgbgr..rgbgr.",
    "..rgr....rgr....rgr..",
    "...r......r......r...",
    ".....................",
    ".....................",
]


let tanarLevel5 = [
    ".rr.rr..rr.rr..rr.rr.",
    ".rr.rr..rr.rr..rr.rr.",
    ".....................",
    ".bb.bb..bb.bb..bb.bb.",
    ".bb.bb..bb.bb..bb.bb.",
    ".....................",
    ".gg.gg..gg.gg..gg.gg.",
    ".gg.gg..gg.gg..gg.gg.",
    ".rr.rr..rr.rr..rr.rr.",
    ".rr.rr..rr.rr..rr.rr.",
    ".....................",
    ".bb.bb..bb.bb..bb.bb.",
    ".bb.bb..bb.bb..bb.bb.",
    ".....................",
    ".gg.gg..gg.gg..gg.gg.",
    ".gg.gg..gg.gg..gg.gg.",
]

gameLevels.push(tanarLevel3)
gameLevels.push(tanarLevel4)
gameLevels.push(olari1)
gameLevels.push(klevel4)
gameLevels.push(tanarLevel2)
gameLevels.push(tanarLevel1)
gameLevels.push(olari4)
gameLevels.push(olari2)
gameLevels.push(olari3)
gameLevels.push(tanarLevel5)
gameLevels.push(klevel3)
gameLevels.push(klevel2)
gameLevels.push(klevel5)
gameLevels.push(olari5)
gameLevels.push(klevel1)