use super::super::database::Database;
use super::receiver::*;

#[derive(PartialEq,Debug)] // for debug/test
pub enum Command {
    Help,
    Random(Vec<String>),
}

// コマンドがあればそれを↑のEnum形式で、なければNoneを返す
pub fn parse_command(plain_text: &str) -> Option<Command> {
    use Command::*;
    let mut terms = plain_text.split_whitespace().map(|x| x.to_lowercase().replace("/",""));     //ケースインセンシティブ化、スラッシュ除去　全て小文字に直してから処理しています
    let command = terms.next();
    if command.is_none() {
        return None;
    }

    let mut command = command.unwrap();
    if command.as_str() == "@bot_xecua_odai" {
        match terms.next() {
            Some(c) => { command = c; }
            None => { return None; }
        }
    }

    match command.as_str() {
        "help" => Some(Help),
        "random" => Some(Random(terms.map(|x| x.to_string()).collect())),
        _ => None
    }
}

pub const HELP_TEXT: &'static str = r#"## このBotの使い方
スラッシュコマンド形式での投稿を行うと該当する内容を実行します(リプライしてもしなくてもいいです あとリプライのときはスラッシュなくてもいいです)
+ help : このヘルプを出します
+ random : 全曲全譜面から適当にお題を出します
    + さらに、スペース区切りで難易度値(1~10,9+)を指定すると、その中からのみ出題します
## :shiyourei_shi::shiyourei_you::shiyourei_rei:
+ `@BOT_xecua_odai help` と投稿すると、ヘルプを出します
+ `/random` と投稿すると、適当にお題を出します
## 直近のアップデート: v1.1.3
+ 括弧の位置がおかしかったのを修正
+ メンションにcase insensitiveで反応([Pull Request#2](https://github.com/xecua/traQbot/pull/2))
"#;

pub struct RandomOption {
    pub levels: Vec<i32>,
    pub difficulties: Vec<String>
}

impl RandomOption {
    pub fn new() -> RandomOption {
        use std::vec::Vec;

        RandomOption {
            levels: Vec::new(),
            difficulties: Vec::new()
        }
    }
}

pub fn random_choice(terms: Vec<String>, data: &MessageCreated, conn: &Database) -> String {
    use super::super::constants::arcaea::{DIFFICULTY,ODAI};
    use super::super::database::operation::{get_random_one, get_random_one_with_option,SongWithDif};
    use super::super::utils::make_mention;
    use rand::seq::SliceRandom;

    if terms.len() > 0 {
        let mut options = RandomOption::new();

        
        for option in terms {
            if option == "1" {
                options.levels.push(1)
            } else if option == "2" {
                options.levels.push(2);
            } else if option == "3" {
                options.levels.push(3);
            } else if option == "4" {
                options.levels.push(4);
            } else if option == "5" {
                options.levels.push(5);
            } else if option == "6" {
                options.levels.push(6);
            } else if option == "7" {
                options.levels.push(7);
            } else if option == "8" {
                options.levels.push(8);
            } else if option == "9" {
                options.levels.push(9);
            } else if option == "9+" {
                options.levels.push(10);
            } else if option == "10" {
                options.levels.push(11);
            } else if "past".eq_ignore_ascii_case(&option) || "pst".eq_ignore_ascii_case(&option) {  //ここは未実装　この機能欲しい
                options.difficulties.push(String::from("PAST"));
            } else if "present".eq_ignore_ascii_case(&option) || "prs".eq_ignore_ascii_case(&option) {
                options.difficulties.push(String::from("PRESENT"));
            } else if "future".eq_ignore_ascii_case(&option) || "ftr".eq_ignore_ascii_case(&option) {
                options.difficulties.push(String::from("FUTURE"));
            }
        }
        
        match get_random_one_with_option(&*conn, &options) {
            Ok(song) => {
                let task = ODAI.choose(&mut rand::thread_rng()).unwrap();

                format!(
                    "{} 『{}』 {}を{}",
                    make_mention(&data.message.user.name, &data.message.user.id),
                    song.title,
                    song.difficulty,
                    task
                )
            }
            Err(e) => {
                format!(
                    "{} {}",
                    make_mention(&data.message.user.name, &data.message.user.id),
                    e
                )
            }
        }
        
    }
    else {
        match get_random_one(&*conn) {
            Ok(title) => {
                let mut rng = rand::thread_rng();
                let dif = DIFFICULTY.choose(&mut rng).unwrap();
                let task = ODAI.choose(&mut rng).unwrap();

                format!(
                    "{} 『{}』 {}を{}",
                    make_mention(&data.message.user.name, &data.message.user.id),
                    title,
                    dif,
                    task
                )
            }
            Err(e) => {
                format!(
                    "{} {}",
                    make_mention(&data.message.user.name, &data.message.user.id),
                    e
                )
            }
        }
    }
}
