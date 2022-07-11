/*
 * @Author: lishengyong bakerhello@163.com
 * @Date: 2022-07-02 20:44:40
 * @LastEditors: lishengyong bakerhello@163.com
 * @LastEditTime: 2022-07-11 17:17:59
 * @FilePath: /betting_generation/src/main.rs
 * @Description: 这是默认设置,请设置`customMade`, 打开koroFileHeader查看配置 进行设置: https://github.com/OBKoro1/koro1FileHeader/wiki/%E9%85%8D%E7%BD%AE
 */

use std::{fmt::{Debug}, ops::Range};
use serde_json::Value;
use serde::{Deserialize};

pub type MatchCover = [bool;4];


#[derive(Clone, Copy, PartialEq)]
pub enum MatchResult {
    Three,
    One,
    Zero,
    NotChoose
}

pub type Bet = Vec<MatchResult>;



pub fn get_bet_info(match_set: &Bet) -> (usize, usize, usize, usize) {
    let mut success = 0;
    let mut draw = 0;
    let mut defeat = 0;
    let mut not_choose = 0;
    for bet in match_set {
        match *bet {
            MatchResult::Three => success += 1,
            MatchResult::One => draw += 1,
            MatchResult::Zero => defeat += 1,
            MatchResult:: NotChoose => not_choose += 1
        }
    }


    (success, draw, defeat, not_choose)
}

pub fn check_cover_set(match_cover_set: &Vec<MatchCover>) -> Result<(), String> {
    for cover in match_cover_set {
        if cover[3] == true {
            if cover[0] == true || cover[1] == true || cover[2] ==true {
                return Err("Not_choose can not be selected with other choise".into());
            }
        }
    }
    Ok(())
}



pub fn generate_bet_from_covers(match_guass_vec: &Vec<MatchCover> ) -> Result<Vec<Bet>, String> {
    // check if the input is valid, all cover must be set
    let _ =  check_cover_set(match_guass_vec)?;
    let mut result = Vec::new();
    
    // calculate how many possiblities of the bet/guass
    let mut start = 1;
    for guass in match_guass_vec {
        let mut temp = 0;
        // continue if not choose
        if guass[3] == true {
            continue;
        }
        if guass[0] == true {
            temp += 1
        }
        if guass[1] == true {
            temp += 1
        }
        if guass[2] == true {
            temp += 1
        }
        if temp == 0 {
            temp = 1;
        }
        start *= temp
    }

    static THRESHOLD: i32 = 20000;
    if start > THRESHOLD {
        return Err("The covers is too big".into());
    }

    let bet_length = match_guass_vec.len();

    // use a stack to generate result
    let mut stack: Vec<MatchResult> = Vec::new();
    loop {
        // if stack is not full, push element to full 
        if stack.len() < bet_length {
            for i in stack.len().. bet_length {
                stack.push(pick_next(&match_guass_vec[i], None).unwrap());
            }
            result.push(stack.clone());
        } 
        // if stack is full, pick next element
        else {
            loop {
                let popped = stack.pop();
                if popped.is_none() {
                    // if popped is none, it means that all possibilities are coverd
                    return Ok(result);
                }
                let index = stack.len();
                let new_res = pick_next(&match_guass_vec[index], popped);
                if new_res.is_some() {
                    stack.push(new_res.unwrap());
                    if stack.len() == bet_length {
                        result.push(stack.clone())
                    }
                    break;
                }
            }
        }
        
    }

}

fn pick_next(match_cover: &MatchCover, current: Option<MatchResult>) -> Option<MatchResult> {
    // get the next possible result of a guass, current must be true in match_guass
    match current {
        Some(curr) => {
            let index: usize = curr.to_index();
            if match_cover[index] == false {
                return None;
            }
            for i in (index+1) .. 4 {
                if match_cover[i] == true {
                    return Some(MatchResult::from_index(i))
                }
            }
            return None
        },
        None => {
            for i in 0 .. 4 {
                if match_cover[i] == true {
                    return Some(MatchResult::from_index(i))
                }
            }
            return Some(MatchResult::NotChoose)
        }
    }
}


impl MatchResult {
    fn to_index(self) -> usize {
        match self {
            MatchResult::Three => 0,
            MatchResult::One => 1,
            MatchResult::Zero => 2,
            MatchResult::NotChoose => 3
        }
    }
}

impl MatchResult {
    fn from_index(index: usize) -> Self {
        match index {
            0 => Self::Three,
            1 => Self::One,
            2 => Self::Zero,
            3 => Self::NotChoose,
            _ => panic!("Unable to convert from usize `{}` to MatchReuslt", index)
        }
    }
}

impl Debug for MatchResult {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let res;
        match self {
            &MatchResult::Three => res = "3",
            &MatchResult::One => res = "1",
            &MatchResult::Zero => res = "0",
            &MatchResult::NotChoose => res = "x",
        }

        print!("{}", res);
        Ok(())
    }
}

trait Constrain {
    fn check(&self, match_result: &Bet) -> bool;
}

#[derive(Clone)]
pub struct RangeConstr {
    suc_con: Range<usize>,
    draw_con: Range<usize>,
    defe_con: Range<usize>
}

impl Constrain for RangeConstr {
    fn check(&self, match_result_set: &Vec<MatchResult>) -> bool {
        let (success, draw, defeat, _) = get_bet_info(match_result_set);
        if self.suc_con.contains(&success) &&
            self.draw_con.contains(&draw) &&
            self.defe_con.contains(&defeat)
        {
            return true
        } else {
            return false
        }

    }
}

impl RangeConstr {
    pub fn from_closed(suc_0: usize, suc_1: usize, draw_0: usize, draw_1: usize, defe_0: usize, defe_1:usize) -> Self {
        return Self { suc_con: suc_0..(suc_1+1), draw_con: draw_0..(draw_1+1), defe_con: defe_0..(defe_1+1) }
    }
}

pub struct ParticalSatisConstr {
    // partically satisfied constrain
    pub record: Vec<(usize, MatchResult)>,
    pub satisfied_range: Range<usize>

}

impl Constrain for ParticalSatisConstr {
    fn check(&self, match_result: &Bet) -> bool {
        let mut satis_num: usize = 0;
        self.record.iter().for_each(|(index, mr)| {
            if match_result[*index] == *mr {
                satis_num += 1;
            }
        });
        return self.satisfied_range.contains(&satis_num);
    }
}

pub struct BetPlanning {
    pub covers: Vec<MatchCover>,
    pub global_constrain: Option<RangeConstr>,
    pub ps_constrains: Vec<ParticalSatisConstr>
}

impl BetPlanning {
    pub fn new(covers: Vec<MatchCover>, global_constrain: Option<RangeConstr>) -> Self {
        return Self { covers: covers, global_constrain: global_constrain, ps_constrains: Vec::new() }
    }

    pub fn add_partial_satis_constrain(&mut self, ps_cons: ParticalSatisConstr) {
        self.ps_constrains.push(ps_cons);
    }

    pub fn solve(&self) -> Result<Vec<Bet>, String> {
        let mut result: Vec<Bet> = Vec::new();
        let bets = generate_bet_from_covers(&self.covers)?;
        
        for bet in bets {
            // check if it satisfied with global constrains
            if self.global_constrain.is_some() {
                if !self.global_constrain.as_ref().unwrap().check(&bet) {
                    continue;
                }
            }

            let mut is_passed = true;
            for ps_constr in self.ps_constrains.iter() {
                if !ps_constr.check(&bet) {
                    // gg
                    is_passed =false;
                    break;
                }
            }
            // if not satisfied with all ps_constrains, continue
            if !is_passed {
                continue;
            }

            // add bet to result
            result.push(bet)
        }

        Ok(result)
    }

    pub fn from_json_bet_plan(jbp: &JsonBetPlan) -> Self {
        let mut result =  Self {
            covers: jbp.covers.0.clone(),
            global_constrain: None,
            ps_constrains: Vec::new()
        };

        // convert json Value to RangConstr
        let global_constrain_array: Vec<usize> = match jbp.global_constrain.as_array() {
            Some(array) => array.iter().map(|x| {
                match x.as_i64() {
                    Some(x) => x as usize,
                    None => 0usize,
                } 
                
            }).collect(),
            None => vec![0, 14, 0, 14, 0, 14]
        };

        result.global_constrain = Some(RangeConstr::from_closed(
            global_constrain_array[0],
            global_constrain_array[1],
            global_constrain_array[2],
            global_constrain_array[3],
            global_constrain_array[4],
            global_constrain_array[5],
        ));

        // convert json Value to Vec<ParticalSatisConstr>
        let ps_constrains: Vec<ParticalSatisConstr> = jbp.ps_constrains.iter().map(|ps_obj| {
            let satis_range: Vec<usize>= match ps_obj.get("satisfied_range").unwrap().as_array() {
                Some(x) => {
                    vec![x[0].as_i64().unwrap() as usize, x[1].as_i64().unwrap() as usize]
                },
                None => vec![0, 14],
            };

            let record: Vec<(usize, MatchResult)>  = ps_obj.get("record").unwrap().as_array().unwrap().iter().map(|rec| {
                let rec_array = rec.as_array().unwrap();
                return (rec_array[0].as_i64().unwrap() as usize, match rec_array[1].as_str().unwrap() {
                    "3" => MatchResult::Three,
                    "1" => MatchResult::One,
                    "0" => MatchResult::Zero,
                    "x" => MatchResult::NotChoose,
                    _ => panic!("not invalid, mut be 3/1/0/x")
                })
            }).collect();
            ParticalSatisConstr {
                satisfied_range: satis_range[0]..(satis_range[1]+1),
                record: record
            }
        }).collect();

        result.ps_constrains = ps_constrains;

        return result
    }

}

#[derive(Deserialize, Clone)]
pub struct CoverVec(pub Vec<MatchCover>);


#[derive(Deserialize)]
pub struct JsonBetPlan {
    pub covers: CoverVec,
    pub global_constrain: Value,
    pub ps_constrains: Vec<Value>
}

