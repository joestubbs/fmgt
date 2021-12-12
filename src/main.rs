use std::io;
use std::path::PathBuf;
use std::fs;
#[macro_use] extern crate rocket;
// use rocket::data::{Data, ToByteUnit};
use rocket::Data;
use rocket::serde::{Deserialize, Serialize};
use rocket::serde::json::Json;
use rocket::fs::NamedFile;
use rocket::data::ByteUnit;
use rocket::response::status::NotFound;

// todo -- ready this from an env var?
const ROOT_DIR: &str = "/";

// ------------
// Structs
// ------------


// A generic JSONResponse struct that can serialize itself
#[derive(Serialize)]
struct JSONResponse<T: Serialize>{
    status: String,
    message: String,
    result: T,
    meta: String,
    version: String,
}

#[derive(Deserialize)]
struct MakedirRequest<'r>{
    directory: &'r str
}


// -----------------------
// Response constructors
// -----------------------

fn err_json_rsp(message: Option<String>) -> JSONResponse<String> {
    let mut msg = String::from("Error processing request.");
    if let Some(m) = message {
        msg = m.to_string();
    }    
    let r = JSONResponse::<String> {
        status: String::from("error"), 
        message: msg,
        result: String::from("none"),
        meta: String::from("none"),
        version: String::from("0.1.0")
    };
    r
}


fn ok_json_rsp(result: Option<String>, meta: Option<String>, message: Option<String>) -> JSONResponse<String> {
    
    // provide default values for result, meta and message
    let mut rst = String::from("[]");
    if let Some(r) = result {
        rst = r.to_string();
    }

    let mut met = String::from("{}");
    if let Some(m) = meta {
        met = m.to_string();
    }
    let mut msg = String::from("The request was successful.");    
    if let Some(m) = message {
        msg = m.to_string();
    }
    JSONResponse {
        status: String::from("success"), 
        message: msg,
        result: rst,
        meta: met,
        version: String::from("0.1.0")
    }
}


// ----------
// Utils
// ----------

// Some conversion functions for working with string types. cf., https://nicholasbishop.github.io/rust-conversions

// Returns None if the input is not valid UTF-8.
pub fn path_buf_to_str(input: &PathBuf) -> Option<&str> {
    input.as_path().to_str()
}

// Returns None if the input is not valid UTF-8.
pub fn path_buf_to_string(input: PathBuf) -> Option<String> {
    input.as_path().to_str().map(|s| s.to_string())
}

fn make_error_rsp(message: String) -> JSONResponse<Vec<String>>{
    let empty: Vec<String> = Vec::new();
    JSONResponse{
        result: empty,
        message: message,
        version: String::from("0.1.0"),
        meta: String::from("none"),
        status: String::from("error"),
    }
}


// ----------------
// Routes 
// ----------------


#[get("/ready")]
fn ready() -> Json<JSONResponse<String>> {
    let message = String::from("Tapis agent ready");
    Json(ok_json_rsp(None, None, Some(message)))
}

#[get("/list/<path..>")]
fn list(path: PathBuf) -> Json<JSONResponse<Vec<String>>> {
    println!("top of /list/<path>");
    // Actual vector of string paths to ultimately return
    let mut path_strs: Vec<String> = Vec::new();
    
    // first, check if we can convert to a str for printing. This will work if the path is valid utf-8
    let path_str = path_buf_to_str(&path);
    if let Some(s) = path_str {
        println!("path: {}", s);
    } else {
        let rsp = make_error_rsp(String::from("Invalid path; paths must be UTF-8"));
        return Json(rsp);
    }

    // prepend the root directory to the `path` paramter.
    let mut full_path = PathBuf::from(ROOT_DIR);
    full_path.push(path);
    println!("full path: {:?}", path_buf_to_str(&full_path));
    // check if `full_path` actually exists
    if !full_path.exists() {
        let message = String::from(format!("Invalid path; path {:?} does not exist", path_buf_to_str(&full_path)));
        let rsp = make_error_rsp(message);
        return Json(rsp);
    }
    // read the paths from the file system
    // todo -- handle case full_path is a file, not a directory
    let paths = fs::read_dir(full_path).unwrap();
    
    // convert the PathBuf objects to Vec<String>
    for path in paths {        
        let s = path.unwrap().file_name().into_string(); // should be safe because we checked that full_path existed
        path_strs.push(s.unwrap());
    }
    let rsp = JSONResponse{
        status: String::from("success"),
        message: String::from("file listing returned suceesfully"), 
        result: path_strs,
        meta: String::from("none"),
        version: String::from("0.1.0"),
    };
    Json(rsp)
}

#[post("/makedir/<path..>", data="<data>")]
fn makedir(path: PathBuf, data:Json<MakedirRequest>) -> Json<JSONResponse<Vec<String>>>{
    // This route ensures the directory `dir` inside the mesage body at path `path` exists.
    // prepend the root directory to the `path` paramter.
    let mut full_path = PathBuf::from(ROOT_DIR);
    full_path.push(path);
    println!("full path: {:?}", path_buf_to_str(&full_path));
    
    // check if `full_path` actually exists ---
    if !full_path.exists() {
        let message = String::from(format!("Invalid path; path {:?} does not exist", path_buf_to_str(&full_path)));
        let rsp = make_error_rsp(message);
        return Json(rsp);
    };
    
    // parse the POST body ---
    full_path.push(data.directory);
    println!("full path with directory body: {:?}", path_buf_to_str(&full_path));

    // create the full directory path to ensure it exists
    let result = fs::create_dir_all(&full_path);
    match result {
        Err(e) => {
            let message = String::from(format!("Unable to make directory; error: {}", e));
            let rsp = make_error_rsp(message);
            return Json(rsp);
        },
        Ok(_r) => ()
    }

    // let result = fs::create_dir_all(&full_path).unwrap_or_else( |error| {
    //     let message = String::from(format!("Unable to make directory; error: {}", error));
    //     let rsp = make_error_rsp(message);
    //     Json(rsp)
    // });

    let empty: Vec<String> = Vec::new();
    let rsp = JSONResponse{
        status: String::from("success"),
        message: String::from("directory creaated suceesfully"), 
        result: empty,
        meta: String::from("none"),
        version: String::from("0.1.0"),
    };
    Json(rsp)

}

#[get("/contents/<path..>")]
async fn get_file(path: PathBuf) -> Result<NamedFile, NotFound<String>> {
    let mut full_path = PathBuf::from(ROOT_DIR);
    full_path.push(path);
    // todo -- check if path exists...
    NamedFile::open(&full_path).await.map_err(|e| NotFound(e.to_string()))
}

#[post("/contents/<path..>", data = "<data>")]
async fn post_file(path: PathBuf, data: Data<'_>) -> Json<JSONResponse<String>> {
    let mut full_path = PathBuf::from(ROOT_DIR);
    full_path.push(path);
    // todo -- check that we can write to full_path
    // todo -- write more than just 1Gi?
    // todo -- write file as a specific uid/gid?
    let one_gib: ByteUnit = "1GiB".parse().unwrap();
    data.open(one_gib).into_file(full_path).await;
    let rsp = JSONResponse{
        status: String::from("success"),
        message: String::from("file uploaded suceesfully"), 
        result: String::from(""),
        meta: String::from("none"),
        version: String::from("0.1.0"),
        };
        Json(rsp)    
}


// ---------------------
// Main server config 
// ---------------------

#[launch]
fn rocket() -> _ {
    rocket::build()
    .mount("/status", routes![ready])
    .mount("/", routes![list, get_file, post_file, makedir])
}
