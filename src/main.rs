use std::{process::exit, io::{Write, Read}};

use crossterm::{
    event::{self, KeyCode, KeyEventKind, KeyEvent},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    prelude::{CrosstermBackend, Stylize, Terminal},
    widgets::Paragraph,
};

// user is given options to see current tasks
// or to make a new task 

fn main() -> std::io::Result<()> {
    // open alternate scree
    std::io::stdout().execute(EnterAlternateScreen)?;
    // enter raw mode so we can handle inputs
    enable_raw_mode()?;
    // make new terminal and handle the backend with crossterm
    let mut terminal = Terminal::new(CrosstermBackend::new(std::io::stdout()))?;
    // clear the terminal
    terminal.clear()?;

    let mut quit = false;
    let mut var = String::new();
    while !quit {
        terminal.draw(|frame|{
            let area: ratatui::layout::Rect = ratatui::layout::Rect{x: 0,y: 0,width: 20,height: 20};
            frame.render_widget(Paragraph::new("Do stuff here")
                .black().on_cyan(), area);
        })?;

        if event::poll(std::time::Duration::from_millis(16))?{
            if let event::Event::Key(key) = event::read()?{
                // idk screw it im done lmao
                for oth in key.code == (KeyCode::Char('a') ..= KeyCode::Char('z')) {

                }
                if key.kind == KeyEventKind::Press && (key.code == KeyCode::Char('q') || key.code == KeyCode::Char('Q'))
                {
                    quit = true;
                }
            }
        }
    }






    display_options();
    let mut get_init_input = String::new();
    input(&mut get_init_input, "Could not get initial input");

    let todo: Todo = Todo::new();

    if !todo.folder_exist(){
        let mut check_answer = String::new();
        eprintln!("Folder does not exist");
        println!("Would you like to make the folder {} (y/n): ", todo.default_location);
        input(&mut check_answer, "Could not read answer fro folder");
        if check_answer.trim().to_lowercase() == String::from("y"){
            todo.create_file(&String::from("~/.Todo"), &String::from("Todo.txt"))?;
        }
        exit(-1);
    }

    match get_init_input.trim().parse::<i32>().unwrap(){
        // the new task is going to have them input stuff
        // program takes it and writes it to a file that stores the tasks
        1 => {
            // open a buffer that we can write text into
            // TODO: currently an indefinite buffer
            // pipe buffer into a file (either seperate file or in one main file)
            let mut buffer = String::new();
            for line in std::io::stdin().read_to_string(&mut buffer).iter(){
                if line.to_string().as_str() == "\0" {break;}
                buffer += &line.to_string();
                println!("{}", line);
            }
            println!("{}", &buffer);
            todo.append(&buffer)?;

            // let handle = &mut std::io::stdin().lock().read_to_string(&mut buffer);
            // handle.read_buf(&mut buffer)?;
            println!("make a new task")
        },
        // reads the tasks from file
        2 => {println!("Look at current tasks")},
        _ => println!("not an input")
    }

    std::io::stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}


fn display_options(){
    let main_options = String::from("1.New task\n2.See tasks\n");
    println!("{}", main_options);
}

fn input(val: &mut String, err: &str){
    std::io::stdin().read_line(val).expect(err);
}

struct Todo{
    default_location: String,
    filename: String,
}

impl Todo{
    fn new() -> Todo {
        Todo{
            default_location: String::from("~/.Todo"),
            filename: String::from("Todo.txt"),
        }
    }
    fn folder_exist(&self) -> bool{
        use std::path::Path;
        Path::new(&self.default_location.as_str()).exists()
    }
    fn create_file(&self, path: &String, filename: &String) -> std::io::Result<()>{
        std::fs::create_dir_all(format!("{}/", path).as_str())?;
        std::fs::File::create(format!("{}/{}", path,filename).as_str())?;
        Ok(())
    }

    fn append(&self, info: &String) -> std::io::Result<()>{
        use std::{fs, path::Path};
        fs::write(Path::new(format!("{}/{}", self.default_location, self.filename).as_str()), &info)?;
        Ok(())
    }

    #[allow(dead_code)]
    fn src_file_exist(&self) -> bool{
        use std::path::Path;
        Path::new(format!("{}/{}", &self.default_location, &self.filename).as_str()).exists()
    }
}

// initial work just using bits of it now to slowly add gui to programf
// We enter the alternative screen
// enable raw mode
// setup the backend and terminal then clear screen
fn _when_internals_finished() -> std::io::Result<()> {
    std::io::stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(std::io::stdout()))?;
    terminal.clear()?;

    let mut quit = false;
    while !quit {
        terminal.draw(|frame|{
            let area = frame.size();
            frame.render_widget(Paragraph::new("Sucks to suck nerds")
                .black().on_cyan(), area);
        })?;

        if event::poll(std::time::Duration::from_millis(16))?{
            if let event::Event::Key(key) = event::read()?{
                if key.kind == KeyEventKind::Press && (key.code == KeyCode::Char('q') || key.code == KeyCode::Char('Q'))
                {
                    quit = true;
                }
            }
        }
    }

    std::io::stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
