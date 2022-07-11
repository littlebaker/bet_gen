/*
 * @Author: lishengyong bakerhello@163.com
 * @Date: 2022-07-08 15:38:51
 * @LastEditors: lishengyong bakerhello@163.com
 * @LastEditTime: 2022-07-10 20:10:54
 * @FilePath: /src-tauri/src/main.rs
 * @Description: 这是默认设置,请设置`customMade`, 打开koroFileHeader查看配置 进行设置: https://github.com/OBKoro1/koro1FileHeader/wiki/%E9%85%8D%E7%BD%AE
 */

#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use betgenlib::BetPlanning;
use betgenlib::JsonBetPlan;
use betgenlib::MatchResult;

fn main() {
    let context = tauri::generate_context!();
    tauri::Builder::default()
        .menu(tauri::Menu::os_default(&context.package_info().name))
        .invoke_handler(tauri::generate_handler![get_state])
        .invoke_handler(tauri::generate_handler![get_bet_planning])
        .run(context)
        .expect("error while running tauri application");
}

#[tauri::command]
fn get_state() -> String {
    return "hello get_state".into();
}

#[tauri::command]
fn get_bet_planning(bet_problem: JsonBetPlan) -> String {
    let bet_plan = BetPlanning::from_json_bet_plan(&bet_problem);
    let result = bet_plan.solve();
    let stringfied_bet: Vec<String> = match &result {
        Ok(res) => res.iter().map(|bet| {
            let mut s = String::with_capacity(bet.len());
            bet.iter().for_each(|mat_r| match *mat_r {
                MatchResult::Three => s.push('3'),
                MatchResult::One => s.push('1'),
                MatchResult::Zero => s.push('0'),
                MatchResult::NotChoose => s.push('x'),
            });

            return s;
        }),
        Err(err_str) => return format!("[\"{}\"]", err_str),
    }
    .collect();
    return serde_json::to_string(&stringfied_bet).unwrap_or_default();
}
