use std::process::{Command, exit};
use names::Generator;

fn update_commit_push() {
    
    let add_command = Command::new("git")
         .arg("add")
         .arg("-A")
         .output()
         .expect("Failed to execute");

    if !add_command.status.success() {
        eprintln!("Error: Failed to add files");
        exit(1);
    }

    let commit_command = Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(name_generator())
        .output()
        .expect("Failed");

    let !commit_command.status.success() {
        println!("Err");
        exit(1);
    }

    let push_command = Command::new("git")
        .arg("push")
        .arg("origin")
        .arg("master")
        .output()
        .expect("Err");
    if !push_command.status.success(){
        eprintln!("Err");
        exit(1);
    }

    println!(" Success")
}

fn name_generator() -> String {
    let mut generator = Generator::default();
    generator.next().unwrap()
}

fn main() {
    update_commit_push()
}