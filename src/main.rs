use clap::Parser;
use cli_player::common::{event::event_bus::EVENTBUS, path};
use cli_player::music::player::Player;
use rodio::OutputStream;
use rodio::Sink;
use std::{env, sync::Arc};

/// 命令行音乐播放器
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// 音乐路径                    cli-player -p D:\Music\x.mp3
    #[arg(short, default_value = "")]
    play: String,

    /// 音乐目录                    cli-player -d D:\Music
    #[arg(short, default_value = "")]
    dir: String,

    /// 设置播放音量【0 - 100】     cli-player -d D:\Music -v 50
    #[arg(short, default_value_t = 100)]
    volume: i32,
}

static mut VOLUME: f32 = 1.0;
#[async_std::main]
async fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let play_event = Arc::new(|music_name: &str| {
        println!("正在播放 {music_name}");
    });
    let done_event = Arc::new(|music_name: &str| {
        println!("播放完成 {music_name}");
    });
    EVENTBUS.on("musicPlay", play_event);
    EVENTBUS.on("musicDone", done_event);

    let cli: Cli = Cli::parse();
    let path: String = cli.play;
    let dir: String = cli.dir;
    let mut volume: i32 = cli.volume;
    if volume > 0 {
        if volume > 100 {
            volume = 100;
        }
    } else {
        volume = 0;
    }
    unsafe {
        VOLUME = (volume as f32) / 100.0;
    }

    if path != "" {
        if path::is_music(&path) {
            play(&path).await;
        } else {
            println!("该文件({})不支持播放", path);
        }
    }
    if dir != "" {
        if path::is_dir(&dir) {
            play4_dir(&dir).await;
        }
    }
}

async fn play4_dir(dir: &str) {
    let vec: Vec<String> = path::get_file(&dir);
    for (_index, value) in vec.iter().enumerate() {
        if path::is_music(&value) {
            play(value).await;
        }
    }
}

async fn play(path: &str) {
    let (_stream, handle) = OutputStream::try_default().unwrap();
    let sink: Sink = Sink::try_new(&handle).unwrap();
    let mut music_player: Player = Player::new(sink);

    music_player.set_volume(unsafe { VOLUME });
    music_player.init(path.to_owned());

    let player_event = Arc::new(move |msg: &str| {
        let items: Vec<&str> = msg.split(" ").collect();
        match items[0] {
            "play" => {
                music_player.play();
            }
            "pause" => {
                music_player.pause();
            }
            "stop" => {
                music_player.stop();
            }
            "volume" => unsafe {
                let volume = items[1].parse::<f32>();
                VOLUME = {
                    match volume {
                        Ok(num) => num,
                        Err(_) => 1.0,
                    }
                };
                music_player.set_volume(VOLUME);
            },
            "sleep_until_end" => {
                music_player.sleep_until_end();
            }
            _ => {}
        };
    });
    let event_id = EVENTBUS.on("player_event", player_event);
    EVENTBUS.emit("player_event", "play".to_string());
    std::thread::spawn(move || {
        EVENTBUS.emit("player_event", "sleep_until_end".to_owned());
    });

    loop {
        let mut guess = String::new();
        std::io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: &str = guess.trim();
        let items: Vec<&str> = guess.split(" ").collect();
        match items[0] {
            "" => {
                println!();
                println!("输入 play 将播放歌曲");
                println!("输入 pause 将暂停播放歌曲");
                println!("输入 next 将播放下一首歌曲");
                println!("输入 stop 将停止播放当前歌曲，并转跳到下一首歌曲");
                println!("输入 volume [0-100] 设置播放音量");
                println!();
            }
            "next" | "stop" => {
                EVENTBUS.emit("player_event", "stop".to_owned());
                EVENTBUS.off("player_event", &event_id);
                break;
            }
            "volume" => {
                let volume = items[1].parse::<f32>();
                let v = {
                    match volume {
                        Ok(num) => num,
                        Err(_) => continue,
                    }
                };
                let set_volume: f32 = (v as f32) / 100.0;
                EVENTBUS.emit("player_event", format!("volume {set_volume}"));
            }
            "pause" => {
                EVENTBUS.emit("player_event", "pause".to_owned());
            }
            "play" => {
                EVENTBUS.emit("player_event", "play".to_owned());
            }
            _ => {
                println!("{}", items[0]);
            }
        }
    }
}
