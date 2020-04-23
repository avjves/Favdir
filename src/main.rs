use structopt::StructOpt;
use dirs::home_dir;

#[derive(StructOpt)]
struct Cli {
    action: String,
    identifier: Option<String>,
}


fn main()  {//  -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::from_args();
    let database = load_favorites();
    match args.action.as_str() {
//        // Change the current directory to saved favorite
//        "cd" => change_directory(&args.identifier),
//        // Add a new favorite
//        "s" => save_favorite(&args.identifier),
//        // Delete a favorite
//        "d" => delete_favorite(&args.identifier),
//        // List all favorites
//        "ls" => list_all_favorites(&database),
        &_ => panic!("Given action is not currently implemented!")
    }
}

fn load_favorites() -> std::collections::HashMap<String, String> {
    let mut fav_file = get_home_dir();
    fav_file.push(".favs");
    println!("Favorite file is: {:?}", &fav_file);
    let favorites_as_string = match std::fs::read_to_string(&fav_file) {
        Ok(content) => content,
        Err(error) => String::from("")
    };
    let mut favorites_map = std::collections::HashMap::new();
    for line in favorites_as_string.lines() {
       let mut values = line.split(",");
       let key = get_split_value(values.next());
       let value = get_split_value(values.next());
       favorites_map.insert(key, value);
    }
    return favorites_map;

}

fn get_split_value(value: Option<&str>) -> String{
    let out = match value {
        Some(content) => content.to_string(),
        None => panic!("Could not parse favourites file.")
    };
    return out;
}

fn get_home_dir() -> std::path::PathBuf {
    let home_dir = match home_dir() {
        Some(path) => path, 
        None => panic!("Could not get current user's home directory!")
    };
    return home_dir;
}

fn list_all_favorites(database: &std::collections::HashMap<String, String>) {
    for (k, v) in database.iter() {
        println!("{}: {}", k, v);
    }
}

//fn change_directory(identifier: str) {
//   
//}
