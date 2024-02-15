use std::{cell::OnceCell, collections::BTreeMap, rc::Rc, sync::mpsc};

use thiserror::Error;

const MONSTER_RACE_DEFINITIONS_URL: &str = "https://raw.githubusercontent.com/hengband/hengband/develop/lib/edit/MonsterRaceDefinitions.txt";

type Result<T> = std::result::Result<T, SearchError>;

#[derive(Debug, Error)]
pub enum SearchError {
    #[error("Preparing")]
    Preparing,
    #[error("Failed to download MonsterRaceDefinitions.txt")]
    FailedToDownload,
    #[error("Monster ID not found")]
    IdNotFound,
}

#[derive(Debug, Default)]
pub struct MonsterRace {
    pub id: u32,
    pub name: String,
    pub english_name: String,
    pub definition: String,
}

impl MonsterRace {
    fn new(id: u32) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }
}

pub struct MonsterRaceDataBase {
    monsters: OnceCell<BTreeMap<u32, Rc<MonsterRace>>>,
    receiver: mpsc::Receiver<String>,
}

impl MonsterRaceDataBase {
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel();

        let request = ehttp::Request::get(MONSTER_RACE_DEFINITIONS_URL);
        ehttp::fetch(request, move |result: ehttp::Result<ehttp::Response>| {
            if let Some(definitions_txt) = result
                .ok()
                .filter(|response| response.status == 200)
                .and_then(|response| String::from_utf8(response.bytes).ok())
            {
                sender.send(definitions_txt).unwrap();
            }
        });

        Self {
            monsters: OnceCell::new(),
            receiver,
        }
    }

    pub fn search(&self, query: &str) -> Vec<Rc<MonsterRace>> {
        self.fetch_monsters()
            .map(|monsters| {
                monsters
                    .values()
                    .filter(|monster| monster.name.contains(query))
                    .map(Rc::clone)
                    .collect()
            })
            .unwrap_or_default()
    }

    pub fn get(&self, id: u32) -> Result<Rc<MonsterRace>> {
        self.fetch_monsters()?
            .get(&id)
            .cloned()
            .ok_or(SearchError::IdNotFound)
    }

    pub fn id_range(&self) -> std::ops::RangeInclusive<u32> {
        let Ok(monsters) = self.fetch_monsters() else {
            return 0..=0;
        };

        let min_id = monsters.keys().next().copied().unwrap_or(0);
        let max_id = monsters.keys().next_back().copied().unwrap_or(0);
        min_id..=max_id
    }

    fn fetch_monsters(&self) -> Result<&BTreeMap<u32, Rc<MonsterRace>>> {
        match self.monsters.get() {
            Some(monsters) => Ok(monsters),
            None => match self.receiver.try_recv() {
                Ok(definitions_txt) => {
                    let monsters = parse_monster_race_definitions(definitions_txt.as_str());
                    self.monsters.set(monsters).unwrap();
                    Ok(self.monsters.get().unwrap())
                }
                Err(mpsc::TryRecvError::Empty) => Err(SearchError::Preparing),
                Err(mpsc::TryRecvError::Disconnected) => Err(SearchError::FailedToDownload),
            },
        }
    }
}

fn parse_monster_race_definitions(definitions_txt: &str) -> BTreeMap<u32, Rc<MonsterRace>> {
    let mut monsters = BTreeMap::new();

    let mut current_monster = MonsterRace::new(0);
    for line in definitions_txt.lines() {
        if line.starts_with('#') || line.trim().is_empty() {
            continue;
        }

        if let Some(stripped) = line.strip_prefix("N:") {
            let Some(index) = stripped.find(':') else {
                continue;
            };

            let Ok(new_id) = stripped[..index].parse::<u32>() else {
                continue;
            };

            if new_id != current_monster.id {
                if current_monster.id != 0 {
                    monsters.insert(current_monster.id, Rc::new(current_monster));
                }
                current_monster = MonsterRace::new(new_id);
            }

            current_monster.name = stripped[index + 1..].to_string();
        }

        if let Some(stripped) = line.strip_prefix("E:") {
            current_monster.english_name = stripped.to_string();
        }

        current_monster.definition.push_str(line);
        current_monster.definition.push('\n');
    }

    if current_monster.id != 0 {
        monsters.insert(current_monster.id, Rc::new(current_monster));
    }

    monsters
}
