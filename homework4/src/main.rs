use std::process::Command;
use std::io;

enum FileOperation {
    List(String),               // Directory path
    Display(String),            // File path
    Create(String, String),     // File path and content
    Remove(String),             // File path
    Pwd,                        // Print working directory
}


fn perform_operation(operation: FileOperation) {
    // Implement command execution based on the operation
    // use match to make operation decision
    match operation {
        //1
        FileOperation::List(directory_path) => { //string is directory path
            let output = Command::new("ls").arg(directory_path).status().expect("Failed to execute ls");
            if !output.success() { //error handling
                println!("Error: could not list path");
            }
        }
        //2
        FileOperation::Display(file_path) => { //strin is file path
            let output = Command::new("cat").arg(file_path).status().expect("Failed to execute cat");
            if !output.success() { //error handling
                println!("Error: could not list path");
            }
        }
        //3
        FileOperation::Create(file_path, content) => {//string 1 is file path, string 2 is content
            let command = format!("echo '{}' > {}", content, file_path);
            let output = Command::new("sh").arg("-c").arg(command).status().expect("Failed to create file");
            if !output.success() { //error handling
                println!("Error: could not list path");
            }
        }
        //4
        FileOperation::Remove(file_path) => { //string is file path
            let output = Command::new("rm").arg(file_path).status().expect("Failed to remove file");
            if !output.success() { //error handling
                println!("Error: could not list path");
            }
        }
        //5
        FileOperation::Pwd => { //no argument needed
            let output = Command::new("pwd").status().expect("Failed to execute pwd");
            if !output.success() { //error handling
                println!("Error: could not list path");
            }
        }
    }
}

fn main() {
    // menu for user option (use match case) 

    println!("Welcome to the File Operations Program :)");
    print!("");
    //loop through menu
    loop {
        //Print menu
        println!("File Operations Menu: ");
        println!("0. Exit");
        println!("1. List files in a directory"); 
        println!("2. Display file contents");
        println!("3. Create a new file");
        println!("4. Remove a file");
        println!("5. Print working directory");

        //user input
        let mut input = String::new();
        println!("Enter input: ");
        io::stdin().read_line(&mut input).unwrap();

        // match case for input
        match input.trim(){

            // exit program w/ break
            "0" => { 
                    println!{"Exiting program"}; 
                    break;
                }

            //list files in directory
            "1" => { 
                     //get directory input
                    println!("Enter directory path: ");

                    // new string   
                    let mut dir_path = String::new();
                    io::stdin().read_line(&mut dir_path).unwrap();

                    //trim to remove \n (needed to_string() due to error)
                    perform_operation(FileOperation::List(dir_path.trim().to_string()));
                    println!("\n"); //spacer
                }

            //display file contents 
            "2" => { 
                    //get file name
                    println!("Enter file path: ");
                    
                    //new input string
                    let mut file = String::new();
                    io::stdin().read_line(&mut file).unwrap();
                    println!("\n"); //spacer

                    println!("File content: ");
                    perform_operation(FileOperation::Display(file.trim().to_string()));
                    println!("\n"); //spacer
                }

            // create new file    
            "3" => { 
                    // get new file name
                    println!("Enter new file name: ");

                    //new input string for file name
                    let mut new_file = String::new();
                    io::stdin().read_line(&mut new_file).unwrap();

                    //get input for file
                    println!("Enter file content: ");

                    //new input string for content
                    let mut file_contents = String::new();
                    io::stdin().read_line(&mut file_contents).unwrap();
                    
                    perform_operation(FileOperation::Create(new_file, file_contents));

                    println!("File created");
                    println!("\n"); //spacer

                }
            //remove file
            "4" => { 
                     //get file name
                    println!("Enter a file to remove: ");
                   
                    let mut file_name = String::new();
                    io::stdin().read_line(&mut file_name).unwrap();

                    perform_operation(FileOperation::Remove(file_name.trim().to_string()));
                    println!("Successfully removed file");
                    println!("\n"); //spacer
                }

            //pwd
            "5" => { 
                    //shows directory that user is in
                    print!("Current working directory: \n");

                    perform_operation(FileOperation::Pwd);
                    println!("\n"); //spacer
                }
            //invalid input
            _ => {
                println!("Error, invalid choice. Choose betwwen 0-5: ");
                println!("\n"); //spacer
            }

        }
    }
   
}
/*
    notes:
    - had issues with how i want it to look when it prints w/ user input
    - why is user input more complex than other languages
    - tried error handling
    - used module 4 to get examples
    - so many print lines (T^T)
*/
