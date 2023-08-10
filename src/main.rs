use serde::Deserialize;
use std::io;
use std::io::Cursor;
use std::path::PathBuf;
use std::time::Instant;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[derive(Deserialize, Debug)]
struct Player {
    rating: i32,
    profile_id: i32,
    name: String, // civ: i32,
}
#[derive(Deserialize, Debug)]
struct Aoe2DEMatch {
    match_id: String,
    // match_uuid: Uuid,
    players: Vec<Player>,
}

#[derive(Deserialize, Debug)]
struct Aoe2DELeaderBoard {
    // rating: i32,
    leaderboard: Vec<Player>,
}

async fn download_from_url(url: String) -> Result<()> {
    let response = reqwest::get(url).await?;
    // let zip_file_path = file_name + ".zip";
    // let mut file = std::fs::File::create(zip_file_path.to_owned())?;
    let content = Cursor::new(response.bytes().await?);
    // std::io::copy(&mut content, &mut file)?;
    let target = PathBuf::from("./");
    let _ = zip_extract::extract(content, &target, false);
    Ok(())
}

async fn download_player_records(player: Player) -> Result<()> {
    let url = format!(
        "https://aoe2.net/api/player/matches?game=aoe2de&profile_id={profile_id}&count=10",
        profile_id = player.profile_id
    );
    let resp = reqwest::get(url).await?.json::<serde_json::Value>().await?;
    let m: Vec<Aoe2DEMatch> = serde_json::from_value(resp)?;
    let now = Instant::now();
    let mut tasks = vec![];
    for aoe2de_match in m.into_iter() {
        let t = tokio::spawn(async move {
            download_from_url(format!(
                "https://aoe.ms/replay/?gameId={match_id}&profileId={profile_id}",
                match_id = aoe2de_match.match_id,
                profile_id = aoe2de_match.players[0].profile_id
            ))
            .await
        });
        tasks.push(t);
    }
    for task in tasks {
        let _ = task.await.expect("download match failed");
    }
    let elapsed = now.elapsed();
    println!("download time for {}: {:.2?}", player.name, elapsed);
    Ok(())
}

async fn download_leaderboard_top_n(count: i32) -> Result<()> {
    // https://aoe2.net/api/leaderboard?game=aoe2de&leaderboard_id=3&start=1&count=10
    let url = format!(
        "https://aoe2.net/api/leaderboard?game=aoe2de&leaderboard_id=3&start=1&count={count}",
        count = count
    );
    let resp = reqwest::get(url).await?.json::<serde_json::Value>().await?;
    let aoe2de_leaderboard: Aoe2DELeaderBoard = serde_json::from_value(resp)?;
    let mut tasks = vec![];
    for (rank, p) in aoe2de_leaderboard.leaderboard.into_iter().enumerate() {
        println!("{:#?}: (rating:{}, rank:{})", p.name, p.rating, rank);
        let t = tokio::spawn(async move { download_player_records(p).await });
        tasks.push(t);
    }
    for task in tasks {
        let _ = task.await.expect("download player failed");
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("Please enter a number for download_leaderboard_top_n:");
    let mut buffer = String::new();
    let stdin = io::stdin(); // We get `Stdin` here.
    stdin.read_line(&mut buffer)?;
    let n: i32 = buffer.trim().parse().expect("please enter integer");
    download_leaderboard_top_n(n).await?;
    Ok(())
}