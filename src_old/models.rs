pub trait Render {
    fn render(&self) -> Vec<String>;
}

#[derive(Debug, Deserialize)]
pub struct ChorePile {
    pub day: Option<u8>,
    pub people: Vec<String>,
    pub tasks: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct ChoreData {
    pub people: Vec<String>,
    pub daily: Vec<ChorePile>,
    pub weekly: Vec<ChorePile>,
    pub weekly_on_day: Vec<ChorePile>,
    pub monthly: Vec<ChorePile>,
    pub first_person: String,
}

#[derive(Debug, Clone)]
pub struct ChoreChunk {
    pub person: String,
    pub chores: Vec<String>,
}

impl ChoreChunk {
    pub fn new(person: String) -> Self {
        ChoreChunk { person, chores: vec![] }
    }
    pub fn add_task(&mut self, task: String) -> () {
        self.chores.push(task)
    }
}

impl Render for ChoreChunk {
    fn render(&self) -> Vec<String> {
        let mut tags = vec![];
        tags.push(format!("<td><strong>{}</strong></td>", self.person));
        for n in 0..self.chores.len() {
            tags.push(format!("<td>{}</td>", self.chores[n]));
        };
        tags
    }
}

#[derive(Debug, Clone)]
pub struct ChoreDay {
    pub day: u8,
    pub chunks: Vec<ChoreChunk>,
}

impl Render for ChoreDay {
    fn render(&self) -> Vec<String> {
        let mut tags = vec![];
        let days = ["Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday"];
        tags.push(format!("<th><strong>{}</strong></th>", days[self.day as usize]));
        for n in 0..self.chunks.len() {
            let mut render = self.chunks[n].render();
            tags.append(&mut render);
        };
        tags
    }
}

impl ChoreDay {
    pub fn new(day: u8, people: &Vec<String>) -> Self {
        ChoreDay { day,
            chunks: people.iter()
                .map(|name| ChoreChunk::new(name.to_string()))
                .collect()
        }
    }

    pub fn new_week(people: Vec<String>) -> Vec<Self> {
        let mut week = vec![];
        for n in 0..7 {
            week.push(Self::new(n, &people));
        };
        week
    }

    pub fn add_chores(&mut self, pile: &ChorePile, rotate_index: usize) -> () {
        let mut chunks: Vec<&mut ChoreChunk> = self.chunks.iter_mut().filter(|c| pile.people == vec!["all".to_string()] || pile.people.contains(&c.person)).collect();
        let tasks = pile.tasks.clone();
        let task_len = tasks.len();
        let chunk_len = chunks.len();
        chunks.rotate_left(rotate_index % chunk_len);
        let mut tasks = tasks.iter().enumerate();
        while let Some((n, task)) = tasks.next() {
            let chunk_index = n % task_len;
            let chunk = &mut chunks[chunk_index];
            chunk.add_task(task.to_string());
        }
    }
}

