use std::{fs, thread};
use clap::Parser;
use webscreenshotlib::{screenshot_tab, write_screenshot, OutputFormat};
use std::process::exit;

/// Subdomain Bruteforcer, and directory bruteforcer written in rust, and multi threaded!!!
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args{
    /// path to the file containing urls, one per line
    #[arg(short, long)]
    urls: String,

    ///Threads to use, there is not upper limit so if you lock up your comptuer that's on you
    #[arg(short, long)]
    threads: usize,

    ///  path to a folder to save the screenshots to
    #[arg(short, long)]
    output_dir: String,

}

fn take_screenshot(url: String, output_dir: &String){
    let dir = output_dir.clone();
    let file_name = &url.split("//").collect::<Vec<&str>>()[1].to_string();
    println!("trying to get screenshot from {}", &url);
    let image_data_result = screenshot_tab(url.as_str(), OutputFormat::PNG, 100, true, 1920, 1080, "");
    if image_data_result.is_err(){
        println!("error taking screenshot of {}", &url);
        return;
    }
    let image_data = image_data_result.unwrap();
    let image_path = format!("{}/{}.png", &dir, file_name);
    let screenshot_result = write_screenshot(&image_path, image_data);
    if screenshot_result.is_err(){
        println!("error saving screenshot from {} to {}", &url, image_path);
        return;
    }
    screenshot_result.unwrap();
}
fn main() {
    print!("
         ..............::::.....                                                        
    ...:-=====+==+++====--+:.....                                                       
  ..:-====+======+=+=---..=.............              ..........           ......       
...--==-==+**++*+**=--=+=....:---::----:....      .....:::----::......    ..:--..       
..-=-==+***###%%##*+*#+-+====-::::::::..::--......:::::::::::----------:-------..       
.:.-===+*###%#**#*##**##***++=----:::.....::::::-:...:::--=+==---==+====------.         
...:===+=-=+##%****++*+*****++==----::...::::::::.:::::======-=++-==*+===-----.         
.......-#-----+#***********:+===----:=-----:=---:--=--=:=---==-=--#*+++===---:.         
   ...:-+=======+####***#**+-=+==++-:----::--:===-===-=-=+=+==+=++=+=======--.          
   .-====-=++*****#*****#**+++++*++***==:-=+=-=:-+==-=+*+++++*++===+++=====-:.          
    .:----==+**###****#+***++===++++**#+-+=+==---+*+*++++=::.....:-=====-=-=..          
     ...===+++*#%%#***+=**=:........:=+#*+=--=-==+++++-......  .....-=*=+-=:.           
       ..=+++++**#****++==+=..      ...=*#*-:==--=+*=:..         ..--=-=-=-..           
       ......+++++******+==+..         .-#*=:-:-===-:.        ....-==+*+=-.             
             .-.=+++=**+*=+*..          .+*+--=====-.    ......:-=-=**===:.             
               .+*++*++*#*++===---==-----+*+-=====---===-===---=++-=++=+-.              
               .=*+=+*#*=*++*+==---=--=***+#*-===++===-:==*====+*+=+==+=..              
               .......=++++=+++==+==-=:-*+++**=+*=**===+-+-=:--=+==+++-..               
                    ....+++**=::===:+====++++*=++=+++=++===:+=+++++++:...               
                       ..=+++**++==+==+====+=+=*++++==+++=+===+++++:..                  
                        ...-=+====++=======+===***++++=======++=-....                   
                             ..:-=++=--:...          ........                           
Author: Kevin (Kage) Pyro 

");
    let mut urls_to_try = Vec::new();
    let args = Args::parse();
    let output_dir = args.output_dir;
    match fs::metadata(&output_dir){
        Ok(metadata) => {
            if metadata.is_dir(){
                println!("screenshot directory found! continuing");
            }
            else{
                println!("screenshot path exists but is not a directory!");
            }
        }
        _ => {
            println!("screenshot directory not found, attempting to create...");
            let create_result = fs::create_dir_all(&output_dir);
            if create_result.is_err(){
                println!("error creating screenshot directory!");
                return;
            };
            create_result.unwrap();
        }
    }
    let url_read = fs::read_to_string(args.urls);
    if url_read.is_err(){
        println!("error reading URL file!");
        return;
    }
    let url_text = url_read.unwrap();
    for url in url_text.split("\n").collect::<Vec<&str>>(){
        if url.len() > 0{
            urls_to_try.push(url.to_owned());
        }
    }
    let mut threads = Vec::new();
    let url_count = urls_to_try.len();
    if &args.threads > &url_count{
        println!("Error thread count higher then the number of urls you provided!");
        println!("please reduce thread count to {} or lower", &url_count);
        println!("Url count:{}", &url_count);
        println!("provided thread count:{}", &args.threads);
        exit(1);
    }
    let chunksize = url_count / args.threads;
    let url_vecs: Vec<Vec<String>> = urls_to_try.chunks(chunksize).map(|x| x.to_vec()).collect();
    //let test_url = "https://google.com".to_owned();
    //take_screenshot(&test_url, &output_dir);
    for urlvec in url_vecs{
        for url in urlvec{
            println!("trying {}", &url);
            let dir = output_dir.clone();
            threads.push(thread::spawn(move || {
                take_screenshot(url, &dir);
            }));
        }
    }
    for thread in threads{
        let _ = thread.join();
    } 
}
