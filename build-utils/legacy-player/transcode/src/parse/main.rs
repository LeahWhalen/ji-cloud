#![allow(warnings)]

use std::{future::Future, path::PathBuf};
use dotenv::dotenv;
use simplelog::*;
use structopt::StructOpt;
use std::fs::File;
use std::io::Write;
use ::transcode::{
    src_manifest::*,
};
use shared::domain::jig::module::ModuleBody;
use image::gif::{GifDecoder, GifEncoder};
use image::{Frame, ImageDecoder, AnimationDecoder};
use flate2::Compression;
use flate2::write::ZlibEncoder;
use std::process::Command;
use reqwest::Client; 

mod options;
use options::*;
mod convert;

// url
// http://localhost:4104/module/legacy/play/debug?slide_index=0&game_id=ID

#[tokio::main]
async fn main() {
    dotenv().ok();
    let mut opts = Opts::from_args();
    init_logger(opts.verbose);
    opts.sanitize();

    let client = Client::new();

    let json_urls = match opts.game_json_url.as_ref() {
        Some(url) => {
            vec![url.as_str()]
        },
        None => {
            vec![
                    // // David Test 002 (houdini) - 17736
                    // https://jitap.net/activities/gemy/play/david-test-002
                    // "https://d24o39yp3ttic8.cloudfront.net/5D00A147-73B7-43FF-A215-A38CB84CEBCD/game.json",

                    // // // Corinne Houdini - 17762
                    // https://jitap.net/activities/geno/play/houdini-states
                    // "https://d24o39yp3ttic8.cloudfront.net/42C980D6-9FCE-4552-A5F2-ECFC0EA8D129/game.json",

                    // // // Soundboard - 17765
                    // // // https://jitap.net/activities/genr/play/soundboard-states 
                    // "https://d24o39yp3ttic8.cloudfront.net/6A973171-C29A-4C99-A650-8033F996C6E7/game.json",

                    // // // say something - 17746
                    // // // https://jitap.net/activities/gen8/play/say-something-options
                    // "https://d24o39yp3ttic8.cloudfront.net/86DCDC1D-64CB-4198-A866-257E213F0405/game.json",

                    // // // video - 17771 
                    // // // https://jitap.net/activities/genx/play/ 
                    // "https://d24o39yp3ttic8.cloudfront.net/94FB3C73-FE29-46A8-933D-75D261DD4B8F/game.json",

                    // // // ask a question - 17792
                    // // // https://jitap.net/activities/geoi/play/testing-ask-a-question-legacy-player
                    // "https://d24o39yp3ttic8.cloudfront.net/236F4AC1-9B06-49EA-B580-4AE806B0A337/game.json",

                    // // puzzle - 17822
                    // // https://jitap.net/activities/gepc/play/test-puzzles-for-legacy-player
                    // "https://d24o39yp3ttic8.cloudfront.net/23DA1A28-88D7-4059-8DD1-167E18C0D5B7/game.json",

                    // // talk or type - 17820
                    // // https://jitap.net/activities/gepa/play/test-talk-or-type-for-legacy-player
                    // "https://d24o39yp3ttic8.cloudfront.net/A789925B-6130-47BA-9DBB-BE4B5D7CCCDC/game.json"
            ]
        }
    };
    
    for url in json_urls {
        transcode_game(&opts, client.clone(), url).await;
    }

}

async fn transcode_game(opts: &Opts, client:Client, game_json_url: &str) {

    log::info!("loading game data from {}", game_json_url);

    let (src_manifest, raw_game_json) = convert::load_url(game_json_url, &client).await;

    let slide_ids:Vec<String> = src_manifest.structure.slides.iter().map(|slide| slide.slide_id()).collect();

    log::info!("loaded manifest, game id: {}", src_manifest.game_id());

    let dest_dir = opts.dest_base_path.join(&src_manifest.game_id());
    std::fs::create_dir_all(&dest_dir);

    let (slides, medias) = convert::into_slides(src_manifest, &client, &opts).await;

    if opts.write_json {
        let dest_path = dest_dir.join(&opts.dest_json_dir);
        std::fs::create_dir_all(&dest_path);

        {
            let mut file = File::create(&dest_path.join("game.json")).unwrap();
            let mut cursor = std::io::Cursor::new(raw_game_json);

            std::io::copy(&mut cursor, &mut file).unwrap();
        }

        {
            let dest_path = dest_path.join("slides");
            std::fs::create_dir_all(&dest_path);

            for (index, slide) in slides.into_iter().enumerate() {
                let id = &slide_ids[index];
                let mut file = File::create(&dest_path.join(format!("{}.json", id))).unwrap();
                serde_json::to_writer_pretty(file, &slide).unwrap();
            }
        }
    }

    if opts.download_media {
        let dest_path = dest_dir.join(&opts.dest_media_dir);
        std::fs::create_dir_all(&dest_path);

        for media in medias.iter() {

            let dest_dir = dest_path.join(&media.basepath);
            std::fs::create_dir_all(&dest_dir);
            let dest_path = dest_dir.join(&media.filename);


            if !opts.skip_download_exists || !dest_path.exists() {
                log::info!("downloading {} -> {}", media.url, dest_path.to_str().unwrap());
                match client.get(&media.url)
                    .send()
                    .await
                    .unwrap()
                    .error_for_status() {
                        Ok(resp) => {
                            let data = resp
                                .bytes()
                                .await
                                .unwrap();

                            let mut cursor = std::io::Cursor::new(data);

                            let mut dest_file = std::fs::File::create(&dest_path).unwrap();
                            std::io::copy(&mut cursor, &mut dest_file).unwrap();
                        },
                        Err(err) => {
                            if opts.allow_empty_media {
                                log::warn!("couldn't download {}", media.url)
                            } else {
                                panic!("couldn't download {}", media.url)
                            }
                        }
                    }

            }
        }


        if opts.transcode_media {

            for media in medias.iter() {
            
                let dest_dir = dest_path.join(&media.basepath);
                let src_file_path = dest_dir.join(&media.filename);
                let src_file_path_str = src_file_path.to_str().unwrap();

                if !src_file_path.exists() {
                    log::warn!("transcode src missing: {}", src_file_path_str);
                    continue;
                }

                if let Some((transcode, filename_dest)) = media.transcode.as_ref() {
                    match transcode {
                        MediaTranscode::Audio => {
 
                            let dest_file_path = dest_dir.join(filename_dest);
                            if !opts.skip_transcode_exists || !dest_file_path.exists() {
                                let dest_file_path = dest_file_path.to_str().unwrap();
                                log::info!("converting audio {} to {}...", media.file_stem(), dest_file_path);

                                Command::new("ffmpeg")
                                    .arg("-i")
                                    .arg(src_file_path_str)
                                    .arg("-acodec")
                                    .arg("libmp3lame")
                                    .arg(dest_file_path)
                                    .output()
                                    .expect("failed to execute process");
                            }
                        },
                        MediaTranscode::Video => {
 
                            let dest_file_path = dest_dir.join(filename_dest);
                            if !opts.skip_transcode_exists || !dest_file_path.exists() {
                                let dest_file_path = dest_file_path.to_str().unwrap();
                                log::info!("converting video {} to {}...", media.file_stem(), dest_file_path);

                                Command::new("ffmpeg")
                                    .arg("-i")
                                    .arg(src_file_path_str)
                                    .arg("-vcodec")
                                    .arg("libx264")
                                    .arg("-acodec")
                                    .arg("aac")
                                    .arg("-pix_fmt")
                                    .arg("yuv420p")
                                    .arg("-crf")
                                    .arg("20")
                                    .arg("-maxrate")
                                    .arg("1M")
                                    .arg(dest_file_path)
                                    .output()
                                    .expect("failed to execute process");
                            }
                        },

                        // ffmpeg -an -i input.mov -vcodec libx264 -pix_fmt yuv420p -profile:v baseline -level 3 output.mp4

                        // no longer transcoding animation
                        // MediaTranscode::Animation => {

                        //     let dest_file_path = dest_dir.join(&format!("{}.webm", media.file_stem()));

                        //     if !opts.skip_transcode_exists || !dest_file_path.exists() {
                        //         let dest_file_path = dest_file_path.to_str().unwrap();
                        //         log::info!("converting animation {} to {}...", media.file_stem(), dest_file_path);

                        //         // ffmpeg -i 2_anim.gif -c vp9 -b:v 0 -crf 26 -pix_fmt yuva420p test001.webm
                        //         Command::new("ffmpeg")
                        //             .arg("-i")
                        //             .arg(src_file_path_str)
                        //             .arg("-c")
                        //             .arg("vp9")
                        //             .arg("-b:v")
                        //             .arg("0")
                        //             .arg("-crf")
                        //             .arg("26")
                        //             .arg("-pix_fmt")
                        //             .arg("-yuva420p")
                        //             .arg(dest_file_path)
                        //             .output()
                        //             .expect("failed to execute process");
                        //     }

                            // works for prores but huge file:
                            // ffmpeg -i 2_anim.gif -c prores -pix_fmt yuva444p10le -profile:v 4 -vf pad="width=ceil(iw/2)*2:height=ceil(ih/2)*2" test001.mov
                            
                            // unfortunately, yuva420p isn't supported here
                            // let dest_file_path = dest_dir.join(&format!("{}.mp4", media.file_stem()));

                            // if !opts.skip_transcode_exists || !dest_file_path.exists() {
                            //     let dest_file_path = dest_file_path.to_str().unwrap();
                            //     log::info!("converting animation {} to {}...", media.file_stem(), dest_file_path);
                            //      ffmpeg -i 2_anim.gif -c libx265 -b:v 0 -crf 26 -pix_fmt yuva420p test006.mp4
                            //     Command::new("ffmpeg")
                            //         .arg("-i")
                            //         .arg(src_file_path_str)
                            //         .arg("-c")
                            //         .arg("libx265")
                            //         .arg("-vtag")
                            //         .arg("hvc1")
                            //         .arg("-preset")
                            //         .arg("slow")
                            //         .arg("-b:v")
                            //         .arg("0")
                            //         .arg("-crf")
                            //         .arg("26")
                            //         .arg("-pix_fmt")
                            //         .arg("-yuva420p")
                            //         .arg(dest_file_path)
                            //         .output()
                            //         .expect("failed to execute process");
                            // }
                        // }
                    }
                }
            }
        }
    }

}

fn init_logger(verbose:bool) {
    if verbose {
        CombinedLogger::init(vec![
            TermLogger::new(LevelFilter::Info, Config::default(), TerminalMode::Mixed),
        ])
        .unwrap();
    } else {
        CombinedLogger::init(vec![
            TermLogger::new(LevelFilter::Warn, Config::default(), TerminalMode::Mixed),
        ])
        .unwrap();
    }
}