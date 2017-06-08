use std::collections::BTreeMap;

pub fn tally(input: &str) -> String {
    let mut rs:Vec<String> = vec![String::from("Team                           | MP |  W |  D |  L |  P")];
    let mut team_stat: BTreeMap<String, (u8,u8,u8)> = BTreeMap::new();
    for line in input.split('\n') {
        if line.is_empty() {
            continue;
        }

        let items = line.split(';').collect::<Vec<&str>>();
        let t1 = items[0];
        let t2 = items[1];
        let result = items[2];

        match result {
            "win" => {
                (*team_stat.entry(t1.to_string()).or_insert((0,0,0))).0 += 1;
                (*team_stat.entry(t2.to_string()).or_insert((0,0,0))).1 += 1
            },
            "loss" => {
                (*team_stat.entry(t1.to_string()).or_insert((0,0,0))).1 += 1;
                (*team_stat.entry(t2.to_string()).or_insert((0,0,0))).0 += 1
            },
            "draw" => {
                (*team_stat.entry(t1.to_string()).or_insert((0,0,0))).2 += 1;
                (*team_stat.entry(t2.to_string()).or_insert((0,0,0))).2 += 1
            },
            _ => panic!()
        }
    }

    let mut rows = team_stat.iter().collect::<Vec<(&String,&(u8,u8,u8))>>();
    rows.sort_by_key(|&(t,&(w,l,d))| (-((w*3+d) as i32),t));
    rs.append(&mut rows.iter().map(|&(t,&(w,l,d))| format!("{:30} | {:2} | {:2} | {:2} | {:2} | {:2}",t,w+l+d,w,d,l,w*3+d)).collect::<Vec<String>>());

    rs.join("\n")
}


