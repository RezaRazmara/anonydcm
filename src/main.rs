use dicom::dictionary_std::tags;
use dicom::core::VR;
use dicom::object::open_file;
use dicom::object::mem::InMemElement;
use std::{fs, ops::Not};
use std::path::{Path, PathBuf};
use clap::{Arg, App, ArgMatches};
use walkdir::WalkDir;

fn commad_line_matches()-> ArgMatches<'static>{
    App::new("anonydcm")
    .version("0.1")
    .author("AUTHOR:\n    Reza Razmara\n    https://www.linkedin.com/in/reza-razmara/\n\n")
    .about("ABOUT:\n    anonymize dicom files")
    .after_help("EXAMPLES:\n    anonydcm --src c:\\dicom_folder \t[description : anonymize and store in a different place automatically]\n    anonydcm --src c:\\dicom_folder -i \t[description : anonymize at the same folder] \n    anonydcm --src c:\\dicom_folder --dst c:\\destination_anonymized_folder\n    anonydcm --src c:\\dicom_folder --dst c:\\destination_anonymized_folder --name sub1 \n    anonydcm -s c:\\dicom_folder -d c:\\destination_anonymized_folder -n sub1 \t[description: rename with sub1]\n    anonydcm -s c:\\dicom_folder -d c:\\destination_anonymized_folder -n \"John Brown\" \t[description: rename with a multipart name]\n    anonydcm -s c:\\dicom_folder -a \t[description: find any dicoms in directory and subdirectories and anonimize it]\n    anonydcm -s c:\\dicom_folder -a -n sub1 \t[description : rename all dicoms in directories and subdirectories with sub1]")
    .arg(Arg::with_name("src")
    .short("s")
    .long("src")
    .help("required : The directory address of dicom files")
    .takes_value(true)
    .required(true))
    .arg(Arg::with_name("dst")
    .short("d")
    .long("--dst")
    .help("optional : destination folder if not set it will create a folder automatically")
    .takes_value(true)
    .required(false))
    .arg(Arg::with_name("new_name")
    .short("n")
    .long("name")
    .help("optional : you can set an new name if not it will be empty.")
    .takes_value(true)
    .required(false))
    .arg(Arg::with_name("in-place")
    .short("i")
    .long("in-place")
    .help("optional : it will anonymized each dicom file in its place.[not stored in seprate folder]")
    .takes_value(false)
    .required(false))
    .arg(Arg::with_name("all-dicoms")
    .short("a")
    .long("all-dicoms")
    .help("optional : it will anonymized all dicoms in all subfolders and folders in the specified path.[not stored in a seprate folder]")
    .takes_value(false)
    .required(false))
    .get_matches()
}

fn get_dst_dir(args: &ArgMatches<'_>, parent_address: &str, name_of_the_folder: &str) -> String{
    let dst_dir = match args.value_of("dst") {
        Some(v) => {
            String::from(v)
        },
        None => {
            let dst = format!("{}\\{}_anonymous",parent_address, name_of_the_folder);
            
            dst
        },
    };

    dst_dir
}

fn get_src_dir<'a>(args: &'a ArgMatches<'a>) -> (&'a str, &'a str, &'a str){
    let src_dir = args.value_of("src").unwrap();
    
    let src_path = Path::new(src_dir);
    let name_of_the_folder: &str = src_path.file_name().unwrap().to_str().unwrap();
    let parent_address: &str = src_path.parent().unwrap().to_str().unwrap();

    (src_dir, parent_address, name_of_the_folder)
}

fn get_new_name<'a>(args: &'a ArgMatches<'a>) -> &'a str {
    let new_name = match args.value_of("new_name"){
        Some(v) => v,
        None => ""
    };

    new_name
}

fn get_list_of_dicoms(src_dir: &str) -> Vec<std::path::PathBuf> {
    let paths = fs::read_dir(src_dir).unwrap().map(|dir_entry| dir_entry.unwrap().path()).filter_map(|path|{
        if path.to_str().unwrap().ends_with("dcm"){
            Some(path)
        }else {
            None
        }
    });

    let list_of_dicoms: Vec<std::path::PathBuf> = paths.collect::<Vec<_>>();
    
    list_of_dicoms
}
fn get_list_of_all_dicoms_in_directory_and_subdirectories(src_dir: &str)-> Vec<std::path::PathBuf>  {
    let mut list_of_dicoms: Vec<std::path::PathBuf> = Vec::new();  
        for entry in WalkDir::new(src_dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().is_file()) // Only files
            .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("dcm")) // Only .dcm files
        {
            list_of_dicoms.push(entry.path().to_path_buf());
        }
    list_of_dicoms
}

fn create_folder_for_anonymized_dicoms_if_doesnt_exist(dst_dir: &String) -> () {
    if Path::new(dst_dir.as_str()).exists().not(){
        let _ = fs::create_dir(dst_dir.clone());
    }
}

fn anonymize_list_of_dicoms(list_of_dicoms: &Vec<PathBuf>, dst_dir: &String, new_name: &str) -> () {
    for dicom_path in list_of_dicoms {
        let file_name = dicom_path.file_name().unwrap().to_str().unwrap();
        rename(dicom_path, &dst_dir, file_name, new_name);
    }
}
fn anonymize_list_of_dicoms_in_place(list_of_dicoms: &Vec<PathBuf>, new_name: &str) -> () {
    for dicom_path in list_of_dicoms {
        let file_name = dicom_path.file_name().unwrap().to_str().unwrap();
        let parent = dicom_path.parent().unwrap().to_str().unwrap();
        rename(dicom_path, parent, file_name, new_name);
    }
}
fn rename(dicom_path: &PathBuf,parent: &str, file_name: &str, new_name: &str){
    let mut obj = open_file(dicom_path.to_str().unwrap()).unwrap();    
    obj.put_element(InMemElement::new(tags::PATIENT_NAME, VR::PN, new_name));
    if new_name.len() == 0 {
        println!("{} anonymized!", file_name);
    }else{
        println!("{} renamed!", file_name);
    }
    let _ = obj.write_to_file(format!("{}\\{}",parent, file_name));
}

fn main() {
    let args: ArgMatches<'_> = commad_line_matches(); 

    let (src_dir, parent_address, name_of_the_folder) = get_src_dir(&args);
    let dst_dir: String = get_dst_dir(&args, &parent_address, &name_of_the_folder);
    let new_name = get_new_name(&args);

    let list_of_dicoms: Vec<std::path::PathBuf>;
    if args.is_present("all-dicoms"){
        list_of_dicoms = get_list_of_all_dicoms_in_directory_and_subdirectories(&src_dir);
    } else {
        list_of_dicoms= get_list_of_dicoms(&src_dir);
    }

    println!("number of dicoms found: {}", list_of_dicoms.len());

    if list_of_dicoms.len() == 0 {
        println!("problem: This folder doesn't have dicom images!");
    } else {
        if args.is_present("in-place") || args.is_present("all-dicoms")  {
            println!("anonymized dicoms stored at: {}", src_dir);
            anonymize_list_of_dicoms_in_place(&list_of_dicoms, &new_name);
        } else {
            println!("anonymized dicoms stored at: {}", dst_dir);
            create_folder_for_anonymized_dicoms_if_doesnt_exist(&dst_dir);
            anonymize_list_of_dicoms(&list_of_dicoms, &dst_dir, &new_name);
        }
    }
}

