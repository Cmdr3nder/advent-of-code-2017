use std::fmt;

fn main() {
    day_1();
    day_2();
    day_3();
}

fn day_1() {
    println!("-- Running Day 1 --");
    let input = "9513446799636685297929646689682997114316733445451534532351778534251427172168183621874641711534917291674333857423799375512628489423332297538215855176592633692631974822259161766238385922277893623911332569448978771948316155868781496698895492971356383996932885518732997624253678694279666572149831616312497994856288871586777793459926952491318336997159553714584541897294117487641872629796825583725975692264125865827534677223541484795877371955124463989228886498682421539667224963783616245646832154384756663251487668681425754536722827563651327524674183443696227523828832466473538347472991998913211857749878157579176457395375632995576569388455888156465451723693767887681392547189273391948632726499868313747261828186732986628365773728583387184112323696592536446536231376615949825166773536471531487969852535699774113163667286537193767515119362865141925612849443983484245268194842563154567638354645735331855896155142741664246715666899824364722914296492444672653852387389477634257768229772399416521198625393426443499223611843766134883441223328256883497423324753229392393974622181429913535973327323952241674979677481518733692544535323219895684629719868384266425386835539719237716339198485163916562434854579365958111931354576991558771236977242668756782139961638347251644828724786827751748399123668854393894787851872256667336215726674348886747128237416273154988619267824361227888751562445622387695218161341884756795223464751862965655559143779425283154533252573949165492138175581615176611845489857169132936848668646319955661492488428427435269169173654812114842568381636982389224236455633316898178163297452453296667661849622174541778669494388167451186352488555379581934999276412919598411422973399319799937518713422398874326665375216437246445791623283898584648278989674418242112957668397484671119761553847275799873495363759266296477844157237423239163559391553961176475377151369399646747881452252547741718734949967752564774161341784833521492494243662658471121369649641815562327698395293573991648351369767162642763475561544795982183714447737149239846151871434656618825566387329765118727515699213962477996399781652131918996434125559698427945714572488376342126989157872118279163127742349";

    let mut char_iter = input.chars();
    let first_char = char_iter.next().unwrap();
    let mut current_char = first_char;
    let mut sum = 0u32;

    for next_char in char_iter {
        if current_char == next_char {
            match current_char.to_digit(10) {
                Some(ref n) => sum = sum + n,
                None => println!("Bad Input")
            };
        }

        current_char = next_char;
    }

    if current_char == first_char {
        match current_char.to_digit(10) {
            Some(ref n) => sum = sum + n,
            None => println!("Bad Input")
        }
    }

    println!("Sum is {}", sum);
}

fn day_2() {
    println!("-- Running Day 2 --");
    let input = "121	59	141	21	120	67	58	49	22	46	56	112	53	111	104	130\n\
                 1926	1910	760	2055	28	2242	146	1485	163	976	1842	1982	137	1387	162	789\n\
                 4088	258	2060	1014	4420	177	4159	194	2794	4673	4092	681	174	2924	170	3548\n\
                 191	407	253	192	207	425	580	231	197	382	404	472	164	571	500	216\n\
                 4700	1161	168	5398	5227	5119	252	2552	4887	5060	1152	3297	847	4525	220	262\n\
                 2417	992	1445	184	554	2940	209	2574	2262	1911	2923	204	2273	2760	506	157\n\
                 644	155	638	78	385	408	152	360	588	618	313	126	172	220	217	161\n\
                 227	1047	117	500	1445	222	29	913	190	791	230	1281	1385	226	856	1380\n\
                 436	46	141	545	122	86	283	124	249	511	347	502	168	468	117	94\n\
                 2949	3286	2492	2145	1615	159	663	1158	154	939	166	2867	141	324	2862	641\n\
                 1394	151	90	548	767	1572	150	913	141	1646	154	1351	1506	1510	707	400\n\
                 646	178	1228	1229	270	167	161	1134	193	1312	1428	131	1457	719	1288	989\n\
                 1108	1042	93	140	822	124	1037	1075	125	941	1125	298	136	94	135	711\n\
                 112	2429	1987	2129	2557	1827	477	100	78	634	352	1637	588	77	1624	2500\n\
                 514	218	209	185	197	137	393	555	588	569	710	537	48	309	519	138\n\
                 1567	3246	4194	151	3112	903	1575	134	150	4184	3718	4077	180	4307	4097	1705";
    let mut sum = 0u32;

    for line in input.lines() {
        let mut max = u32::min_value();
        let mut min = u32::max_value();

        for word in line.split_whitespace() {
            if let Ok(current_num) = word.parse::<u32>() {
                if current_num > max {
                    max = current_num;
                }

                if current_num < min {
                    min = current_num;
                }
            } else {
                println!("Bad Input");
            }
        }

        sum = sum + (max - min);
    }

    println!("Checksum is {}", sum);
}

enum Cardinal {
    North,
    South,
    East,
    West
}

impl Cardinal {
    fn turn_left(&self) -> Cardinal {
        match *self {
            Cardinal::North => Cardinal::West,
            Cardinal::South => Cardinal::East,
            Cardinal::East  => Cardinal::North,
            Cardinal::West  => Cardinal::South
        }
    }

    fn apply_offset(&self, current_coords: (i32, i32)) -> (i32, i32) {
        let offset: (i32, i32) = match *self {
            Cardinal::North => (0, 1),
            Cardinal::South => (0, -1),
            Cardinal::East => (1, 0),
            Cardinal::West => (-1, 0)
        };
        offset 
    }
}

impl fmt::Display for Cardinal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match *self {
            Cardinal::North => "North",
            Cardinal::South => "South",
            Cardinal::East => "East",
            Cardinal::West => "West"
        })
    }
}

fn day_3() {
    println!("-- Running Day 3 --");
    let input = 347991i32;
    //let input = 23i32;
    let (x, y) = day_3_determine_coords(input as f32);
    let taxicab_distance = (0 - x).abs() + (0 - y).abs();
    println!("Taxicab distance is {}", taxicab_distance);
}

fn day_3_determine_coords(position: f32) -> (i32, i32) {
    let n = position as f32;
    let k = ((n.sqrt() - 1f32) / 2f32).ceil();
    let t = (2f32 * k) + 1f32;
    let mut m = t.powi(2);
    let t = t - 1f32;

    if n > (m - t) {
        return ((k - (m - n)).ceil() as i32, (-k).ceil() as i32);
    } else {
        m = m - t;
    }

    if n > (m - t) {
        return ((-k).ceil() as i32, (-k + (m - n)).ceil() as i32);
    } else {
        m = m - t;
    }

    if n > (m - t) {
        return ((-k +  (m - n)).ceil() as i32, k.ceil() as i32);
    } else {
        return (k.ceil() as i32, (k - (m - n - t)).ceil() as i32);
    }
}