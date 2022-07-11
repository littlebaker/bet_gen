
/*
 * @Author: lishengyong bakerhello@163.com
 * @Date: 2022-07-10 16:38:49
 * @LastEditors: lishengyong bakerhello@163.com
 * @LastEditTime: 2022-07-10 17:26:32
 * @FilePath: /betting_generation/src/main.rs
 * @Description: 这是默认设置,请设置`customMade`, 打开koroFileHeader查看配置 进行设置: https://github.com/OBKoro1/koro1FileHeader/wiki/%E9%85%8D%E7%BD%AE
 */
use betgenlib::*;
use std::process;


fn main() {
    let mut covers = vec![[true, false, false, false]; 14];
    covers[0] = [true, true, false, false];
    covers[1] = [true, true, true, false];
    covers[2] = [true, true, false, false];
    let covers_vec = CoverVec(covers.clone());
    match check_cover_set(&covers) {
        Ok(_) => {},
        Err(s) => {
            println!("{}", s);
            process::exit(1);
        }
    }
    let all_result = generate_bet_from_covers(&covers).unwrap();
    // println!("{:?}", &all_result);
    println!("number: {}", &all_result.len());
    let range_cons = RangeConstr::from_closed(13, 14, 0, 14, 0, 14);
    // for result_set in &all_result {
    //     println!("{:?}  {}", result_set, range_cons.check(result_set));
    // }

    let mut bet_planning = BetPlanning::new(covers, Some(range_cons.clone()));
    bet_planning.add_partial_satis_constrain(ParticalSatisConstr { record: vec![(0, MatchResult::Three),(1, MatchResult::Three)], satisfied_range: 0..2 });
    bet_planning.add_partial_satis_constrain(ParticalSatisConstr { record: vec![(0, MatchResult::One)], satisfied_range: 1..2 });
    println!("{:?}", bet_planning.solve().unwrap());
}

