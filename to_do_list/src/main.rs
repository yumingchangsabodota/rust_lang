


#[derive(Debug)]
pub enum Status {
    Pending,
    Done,
}

#[derive(Debug)]
pub struct Task {
    pub title: String,
    pub description: String,
    pub status: Status,
}


impl Task {
    pub fn update_title(&mut self, title: String){
        self.title = title;
    }

    pub fn update_description(&mut self, description: String){
        self.description = description;
    }

    pub fn update_status(&mut self, status: Status){
        self.status = status;
    }
}

pub struct ToDoList {
    pub tasks: Vec<Task>
}

impl ToDoList {
    pub fn add_task(&mut self, task: Task) {
        let _result = &self.tasks.push(task);
    }

    pub fn list_tasks(&self) {
        for (index, task) in self.tasks.iter().enumerate() {
            print!("{:?} - {:?}\n", index, task);
            print!("\n");
        }
    }

    pub fn remove_task(&mut self, title: String) {
        self.tasks.retain(|task| task.title!=title);
    }

}



fn main() {
    let task_1 = Task{
        title: "Buy Fruits".to_string(),
        description: "Go to Costco and buy cherry.".to_string(),
        status: Status::Pending,
    };

    let task_2 = Task{
        title: "Wash Car".to_string(),
        description: "Go to car wash".to_string(),
        status: Status::Pending,
    };

    let mut to_do_list = ToDoList{
        tasks: vec![],
    };

    to_do_list.add_task(task_1);
    to_do_list.add_task(task_2);

    print!("------ Initial State ------");
    print!("\n");
    to_do_list.list_tasks();

    let new_task_1_title = "Buy Cherry";

    for task in to_do_list.tasks.iter_mut(){
        if task.title == "Buy Fruits".to_string(){
            task.update_title(new_task_1_title.to_string());
        }
    }

    print!("------ Update task 1 title ------");
    print!("\n");
    to_do_list.list_tasks();
    to_do_list.remove_task("Wash Car".to_string());

    print!("------ Remove task ------");
    print!("\n");
    to_do_list.list_tasks();

    to_do_list.tasks[0].update_status(Status::Done);


    print!("------ Set task to Done------");
    print!("\n");
    to_do_list.list_tasks();


}
