use clap::{Parser, Subcommand};
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::fs;    
use std::io;   

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]

enum TaskStatus {
    Todo,  
    Done,  
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Task {
    id: usize,               
    description: String,     
    status: TaskStatus,       
}

impl Task {
    
    fn new(id: usize, description: String) -> Self {
        Task {
            id,
            description,
            status: TaskStatus::Todo,  
        }
    }

    fn mark_done(&mut self) {
        self.status = TaskStatus::Done;
    }

    fn mark_todo(&mut self) {
        self.status = TaskStatus::Todo;
    }
  
    fn display(&self) {
        let status_symbol = match self.status {
            TaskStatus::Todo => "☐".bright_red(),    
            TaskStatus::Done => "☑".bright_green(),  
        };

        let description = match self.status {
            TaskStatus::Todo => self.description.bright_white(),                     
            TaskStatus::Done => self.description.bright_black().strikethrough(),      
        };

       
        println!("[{}] {} {}", 
                 self.id.to_string().bright_cyan(),  
                 status_symbol,                       
                 description);                        
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct TodoList {
    tasks: Vec<Task>,    
    next_id: usize,      
}

impl TodoList {
    
    fn new() -> Self {
        TodoList {
            tasks: Vec::new(),   
            next_id: 1,          
        }
    }

    fn get_file_path() -> String {
        "tasks.json".to_string()
    }

    fn load() -> io::Result<Self> {
        let path = Self::get_file_path();

        if !std::path::Path::new(&path).exists() {
            return Ok(Self::new());
        }

        let contents = fs::read_to_string(path)?;
        
        let todo_list: TodoList = serde_json::from_str(&contents)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        Ok(todo_list)
    }

    fn save(&self) -> io::Result<()> {
        let path = Self::get_file_path();
        
        let json = serde_json::to_string_pretty(self)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        fs::write(path, json)?;
        
        Ok(())  
    }

    fn add_task(&mut self, description: String) -> usize {
        let id = self.next_id;                           
        let task = Task::new(id, description);          
        self.tasks.push(task);                           
        self.next_id += 1;                              
        id                                              
    }

    fn mark_done(&mut self, id: usize) -> bool {
        
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
            task.mark_done();
            true
        } else {
            false  
        }
    }

    fn mark_todo(&mut self, id: usize) -> bool {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
            task.mark_todo();
            true
        } else {
            false
        }
    }

    fn remove_task(&mut self, id: usize) -> bool {

        if let Some(pos) = self.tasks.iter().position(|t| t.id == id) {
            self.tasks.remove(pos);  
            true
        } else {
            false
        }
    }

    fn list_all(&self) {
        if self.tasks.is_empty() {
            println!("{}", "No tasks yet! Add one with: todo add \"your task\"".yellow());
            return;
        }

        println!("\n{}\n", "All Tasks:".bold().bright_blue());
        for task in &self.tasks {
            task.display();
        }
        println!();
    }

    fn list_todo(&self) {
        let todos: Vec<&Task> = self.tasks
            .iter()
            .filter(|t| t.status == TaskStatus::Todo)
            .collect();

        if todos.is_empty() {
            println!("{}", "No pending tasks! 🎉".green().bold());
            return;
        }

        println!("\n{}\n", "Pending Tasks:".bold().bright_red());
        for task in todos {
            task.display();
        }
        println!();
    }
  
    fn list_done(&self) {
        let done: Vec<&Task> = self.tasks
            .iter()
            .filter(|t| t.status == TaskStatus::Done)
            .collect();

        if done.is_empty() {
            println!("{}", "No completed tasks yet.".yellow());
            return;
        }

        println!("\n{}\n", "Completed Tasks:".bold().bright_green());
        for task in done {
            task.display();
        }
        println!();
    }

    fn clear_done(&mut self) -> usize {
        let initial_count = self.tasks.len();
       
        self.tasks.retain(|t| t.status == TaskStatus::Todo);
        
        initial_count - self.tasks.len()  
    }
}

#[derive(Parser)]
#[command(name = "todo")]
#[command(about = "Manage your tasks from the command line", long_about = None)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    
    Add {
       
        description: String,
    },

    List {
       
        #[arg(short, long)]
        todo: bool,

        #[arg(short, long)]
        done: bool,
    },

    Done {
      
        id: usize,
    },

    Undone {

        id: usize,
    },

    Remove {

        id: usize,
    },

    Clear,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let cli = Cli::parse();
    
let mut todo_list = TodoList::load().unwrap_or_else(|_| TodoList::new());

match cli.command {
    Commands::Add { description } => {
        let id = todo_list.add_task(description.clone());
        todo_list.save()?;
        println!("{} Task #{} added: {}", 
                 "✓".green().bold(), 
                 id.to_string().cyan().bold(),
                 description.bright_white());
    }
    Commands::List { todo, done } => {
        if todo {
            todo_list.list_todo();
        } else if done {
            todo_list.list_done();
        } else {
            todo_list.list_all();
        }
    }
    Commands::Done { id } => {
        if todo_list.mark_done(id) {
            todo_list.save()?;
            println!("{} Task #{} marked as done!", 
                     "✓".green().bold(), 
                     id.to_string().cyan().bold());
        } else {
            eprintln!("{} Task #{} not found.", 
                     "✗".red().bold(), 
                     id.to_string().cyan());
        }
    }
    Commands::Undone { id } => {
        if todo_list.mark_todo(id) {
            todo_list.save()?;
            println!("{} Task #{} marked as todo.", 
                     "✓".green().bold(), 
                     id.to_string().cyan().bold());
        } else {
            eprintln!("{} Task #{} not found.", 
                     "✗".red().bold(), 
                     id.to_string().cyan());
        }
    }
    Commands::Remove { id } => {
        if todo_list.remove_task(id) {
            todo_list.save()?;
            println!("{} Task #{} removed.", 
                     "✓".green().bold(), 
                     id.to_string().cyan().bold());
        } else {
            eprintln!("{} Task #{} not found.", 
                     "✗".red().bold(), 
                     id.to_string().cyan());
        }
    }
    Commands::Clear => {
        let count = todo_list.clear_done();
        todo_list.save()?;
        println!("{} Cleared {} completed task(s).", 
                 "✓".green().bold(), 
                 count.to_string().cyan().bold());
    }
}

Ok(())
}
