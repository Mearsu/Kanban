use serde::Deserialize;
use serde::Serialize;
use std::fs;
use std::env;


#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Copy)]
enum TaskState{
    BackBurner,
    Todo,
    Doing,
    AwaitingChanges,
    Finished,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Task{
    name: String,
    state: TaskState,
    notes: Vec<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Kanban{
    tasks: Vec<Task>,
}

fn load(file: &str)-> Result<Kanban, std::io::Error>{
    let file = fs::read_to_string(file)?;
    let kbn: Kanban = serde_yaml::from_str(file.as_str()).unwrap();
    Ok(kbn)
}

fn save(kbn: Kanban, filename: &str) -> Result<(), serde_yaml::Error>{
    let yaml = serde_yaml::to_string(&kbn)?;
    fs::write(filename, yaml).unwrap();
    Ok(())
}

fn print_kanban(kbn: &Kanban){
    let mut len = 15;
    for t in &kbn.tasks{
        if len < t.name.len(){
           len = t.name.len();
        }
    }
    len = len + 1;
    println!("┌──┬{:─<len$}┬{:─<len$}┬{:─<len$}┬{:─<len$}┬{:─<len$}┐", "", "", "", "", "");
    println!("│  │{: ^len$}│{: ^len$}│{: ^len$}│{: ^len$}│{: ^len$}│", "Back Burner", "Todo", "Doing","Awaiting Changes", "Finished");
    println!("├──┼{:─<len$}┼{:─<len$}┼{:─<len$}┼{:─<len$}┼{:─<len$}┤", "", "", "", "", "");
    let mut i = 0;
    let mut active = &kbn.tasks[0];
    for t in &kbn.tasks{
        match &t.state {
            TaskState::BackBurner =>        println!("│{: <2}│{: <len$}│{: <len$}│{: <len$}│{: <len$}│{: <len$}│",i, t.name,    "",         "",         "",     ""      ),
            TaskState::Todo =>              println!("│{: <2}│{: <len$}│{: <len$}│{: <len$}│{: <len$}│{: <len$}│",i, "",        t.name,     "",         "",     ""      ),
            TaskState::Doing =>             println!("│{: <2}│{: <len$}│{: <len$}│{: <len$}│{: <len$}│{: <len$}│",i, "",        "",         t.name,     "",     ""      ),
            TaskState::AwaitingChanges =>   println!("│{: <2}│{: <len$}│{: <len$}│{: <len$}│{: <len$}│{: <len$}│",i, "",        "",         "",         t.name, ""      ),
            TaskState::Finished =>          println!("│{: <2}│{: <len$}│{: <len$}│{: <len$}│{: <len$}│{: <len$}│",i, "",        "",         "",         "",     t.name  ),
        }
        i = i + 1;
        if t.state == TaskState::Doing{
            active = &t;
        }

    }
    println!("└──┴{:─<len$}┴{:─<len$}┴{:─<len$}┴{:─<len$}┴{:─<len$}┘", "", "", "", "", "");
    if active.state == TaskState::Doing {
        if active.notes.len() != 0{
            println!("Notes for task {}", active.name);
        }
        let mut i = 0;
        for note in &active.notes{
            println!("{}. {}", i, note);
            i = i + 1;
        }
    }

}

fn kanban_add(kbn: &mut Kanban, task: String){
    println!("\"{}\"", task);
    if task == ""{
        return;
    }
    kbn.tasks.push({
        Task{
            name: task,
            state: crate::TaskState::BackBurner,
            notes: vec![],
        }
    }
    )
}
fn kanban_change_state(kbn: &mut Kanban, task: Option<String>, state: TaskState){
    let mut changed_active = false;
    if state == TaskState::Doing{
        for task in &mut kbn.tasks{
            if task.state == TaskState::Doing{
                task.state = TaskState::Todo;
            }
        }
    }
    if let Some(str) = task{
        if let Ok(i) = usize::from_str_radix(str.as_str(), 10){
            if kbn.tasks[i].state == TaskState::Doing{
               changed_active = true;
            }
            kbn.tasks[i].state = state;
        }else{
            eprintln!("Failed to parse {} as base 10 number", str);
        }
     }else{
        eprintln!("Missing task number");
    }
    if (state == TaskState::AwaitingChanges || state == TaskState::Finished) && changed_active{
        for task in &mut kbn.tasks{
            if task.state == TaskState::Todo{
                task.state = TaskState::Doing;
                return;
            }
        }
    }
}

fn kanban_remove_finished(kbn: &mut Kanban){
    let mut i: usize = 0;
    loop{
        if kbn.tasks[i].state == TaskState::Finished{
            kbn.tasks.remove(i);
        }else{
            i = i + 1;
        }
        if i >= kbn.tasks.len(){
            return;
        }
    }
}

fn kanban_process_note(kbn: &mut Kanban, args: &mut std::env::Args){
    let mut active = None;
    for t in &mut kbn.tasks{
        if t.state == TaskState::Doing{
            active = Some(t);
        }
    }
    if let Some(act) = active{
        match args.next() {
            None=>{},
            Some(s) =>{
            if String::from("add").starts_with(&s.to_lowercase()){
                act.notes.push(args.map(|x| format!("{} ", x)).collect::<String>());
            }else if String::from("remove").starts_with(&s.to_lowercase())||
                    String::from("rm").starts_with(&s.to_lowercase())||
                    String::from("delete").starts_with(&s.to_lowercase()){

                let num = usize::from_str_radix(args.next().unwrap().as_str(), 10).unwrap();
                act.notes.remove(num);
            }else{
                eprintln!("Unknown command {}", s);
            }
            }
        }
    }else{
                    eprintln!("No active task");
    }
}

fn main() {
    let kanban_dir : String = if cfg!(debug_assertions) {
        String::from("./kbn.yaml")
    } else {
        format!("{}/.local/kanban.yaml", env::var("HOME").unwrap())
    };

    let mut kbn = if let Ok(kbn) = load(kanban_dir.as_str()){
        kbn
    }else{
        Kanban {
            tasks: vec![]
        }
    };
    let mut args = std::env::args();
    if args.len() > 1{
        args.next();
        match args.next() {
            None=>{},
            Some(s) =>{
                if String::from("add").starts_with(&s.to_lowercase()){
                    kanban_add(&mut kbn, args.map(|x| format!("{} ", x)).collect::<String>());
                }else if String::from("back").starts_with(&s.to_lowercase()){
                    kanban_change_state(& mut kbn, args.next(), TaskState::BackBurner);
                }else if String::from("todo").starts_with(&s.to_lowercase()){
                    kanban_change_state(& mut kbn, args.next(), TaskState::Todo);
                }else if String::from("doing").starts_with(&s.to_lowercase()){
                    kanban_change_state(& mut kbn, args.next(), TaskState::Doing);
                }else if String::from("waiting").starts_with(&s.to_lowercase()){
                    kanban_change_state(& mut kbn, args.next(), TaskState::AwaitingChanges);
                }else if String::from("finished").starts_with(&s.to_lowercase()){
                    kanban_change_state(& mut kbn, args.next(), TaskState::Finished);
                }else if String::from("note").starts_with(&s.to_lowercase()){
                    kanban_process_note(&mut kbn, &mut args);
                }else if String::from("purge").starts_with(&s.to_lowercase()){
                    kanban_remove_finished(&mut kbn);
                }else{
                    eprintln!("Unknown command {}", s);
                }
                }
            }
        }
        print_kanban(&kbn);
    save(kbn, kanban_dir.as_str()).unwrap();
}
