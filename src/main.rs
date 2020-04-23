use structopt::StructOpt;
use dirs::home_dir;

#[derive(StructOpt)]
struct Cli {
    action: String,
    identifier: Option<String>,
}


fn main() -> Result<(), std::io::Error> {
    let args = Cli::from_args();
    let mut database = load_favorites();
    let er = match args.action.as_str() {
//        // Change the current directory to saved favorite
//        "cd" => change_directory(&args.identifier),
//        // Add a new favorite
        "s" => save_favorite(args.identifier.unwrap(), database),
//        // Delete a favorite
//        "d" => delete_favorite(&args.identifier),
//        // List all favorites
        "ls" => list_all_favorites(&database),
        &_ => Ok(())
    };
    Ok(())
}

fn load_favorites() -> std::collections::HashMap<String, String> {
    let fav_file = get_fav_file();
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

fn get_fav_file() -> std::path::PathBuf {
    let mut fav_file = match home_dir() {
        Some(path) => path, 
        None => panic!("Could not get current user's home directory!")
    };
    fav_file.push(".favs");
    return fav_file;
}

fn list_all_favorites(database: &std::collections::HashMap<String, String>) -> Result<(), std::io::Error>{
    for (k, v) in database.iter() {
        println!("{}: {}", k, v);
    }
    Ok(())
}

fn save_favorite(identifier: String, mut database: std::collections::HashMap<String, String>) -> Result<(), std::io::Error>{
    let current_dir = std::env::current_dir()?;
    println!("Current_dir : {:?}", current_dir);
    database.insert(identifier, current_dir.display().to_string());
    save_favorites(database);
    Ok(())
}

fn save_favorites(database: std::collections::HashMap<String, String>) -> Result<(), std::io::Error> {
    let fav_file = get_fav_file();
    let map_as_string = get_map_as_string(&database);
    std::fs::write(fav_file, map_as_string);
    Ok(())
}

fn get_map_as_string(database: &std::collections::HashMap<String, String>) -> String {
    let mut map_as_string = String::from("");
    for (k, v) in database.iter() {
        map_as_string.push_str(&k.to_owned());
        map_as_string.push_str(",");
        map_as_string.push_str(&v.to_owned());
        map_as_string.push_str("\n");
    }
    return map_as_string;
}

//fn change_directory(identifier: str) {
//   
//}
