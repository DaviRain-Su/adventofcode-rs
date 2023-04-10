//! # day1
//!
//! 圣诞老人的驯鹿通常吃普通的驯鹿食物，但是它们需要大量的魔法能量才能在圣诞节送礼物。因此，它们最喜欢的小吃是一种特殊的星星水果，只在深处的丛林中生长。小精灵们带你去采集这种水果的地方。
//!
//! 为了提供足够的魔法能量，这次探险队必须在12月25日之前采集到至少50颗星星。尽管小精灵们保证这片林地有大量的水果，但你决定沿途捡拾任何看到的水果，以防万一。
//!
//! 通过解决难题收集星星。在圣诞日历上，每天会提供两个难题；完成第一个难题后，第二个难题就会解锁。每个难题授予一颗星星。祝好运！
//!
//! 这片丛林可能太过茂密难以开车或从空中进入；小精灵们的探险队传统上步行前进。当你的船靠近陆地时，小精灵们开始清点他们的物资。一个重要的考虑因素是食物 - 特别是每个小精灵携带的热量总量（输入难题的数据）。
//!
//! 小精灵们轮流写下各种餐点、小吃、口粮等的卡路里数，每行一项。每个小精灵通过一个空行将自己的清单与前一个小精灵的清单（如果有的话）分开。
//!
//! 例如，假设小精灵们完成了他们的食物热量清单，并得到以下列表：
//!
//! 1000
//! 2000
//! 3000
//!
//! 4000
//!
//! 5000
//! 6000
//!
//! 7000
//! 8000
//! 9000
//!
//! 10000
//! 这个列表代表五个小精灵携带的食物热量：
//!
//! 第一个小精灵携带1000、2000和3000卡路里的食物，总共6000卡路里。
//! 第二个小精灵携带一种4000卡路里的食品。
//! 第三个小精灵携带5000和6000卡路里的食物，总共11000卡路里。
//! 第四个小精灵携带7000、8000和9000卡路里的食物，总共24000卡路里。
//! 第五个小精灵携带一种10000卡路里的食品。
//!
//! 万一精灵们感到饥饿，需要额外的零食，他们需要知道询问哪个精灵：他们想知道携带最多卡路里的精灵携带了多少卡路里。在上面的例子中，这是24000（由第四个精灵携带）。
//!
//! 找到携带最多卡路里的精灵。那个精灵携带的总卡路里是多少？

use anyhow::Result;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

pub fn process(path: PathBuf) -> Result<u128> {
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let max_value = contents
        .split("\n\n")
        .fold(vec![], |mut result, v| {
            let temp_vector = v
                .split('\n')
                .map(|v| v.parse::<u128>().unwrap_or(0))
                .collect::<Vec<_>>();
            result.push(temp_vector);
            result
        })
        .into_iter()
        .fold(vec![], |mut result, v| {
            result.push(v.into_iter().sum::<u128>());
            result
        })
        .into_iter()
        .max()
        .unwrap_or(0);

    Ok(max_value)
}

#[test]
fn test_process() {
    use std::str::FromStr;
    let path =
        PathBuf::from_str("/Users/davirain/adventofcode-rs/tests/fixtures/day1.txt").unwrap();
    let result = process(path);
    println!("result = {:?}", result);
    assert_eq!(72602, result.unwrap());
}
