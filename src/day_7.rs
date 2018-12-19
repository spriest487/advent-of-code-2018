use {
    std::{
        fmt,
        collections::HashMap,
    },
};

#[derive(Debug)]
struct Dependency {
    require: char,
    next: char,
}

impl Dependency {
    fn parse(s: &str) -> Self {
        // "Step A must be finished before step B can begin"
        let require: char = s[5..6].parse().unwrap();
        let next: char = s[36..37].parse().unwrap();

        Self { require, next }
    }
}

impl fmt::Display for Dependency {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Step {} must be finished before step {} can begin", self.require, self.next)
    }
}

#[derive(Debug, Clone)]
struct Step {
    id: char,
    deps: Vec<char>,
}

impl Step {
    fn duration(&self) -> usize {
        BASE_DURATION + 1 + (self.id as usize) - ('A' as usize)
    }
}

#[derive(Copy, Clone)]
struct Job {
    id: char,
    finish_time: usize,
}

const ELF_COUNT: usize = 5;
const BASE_DURATION: usize = 60;

fn main() {
    let input = include_str!("day_7.txt");
    let deps: Vec<_> = input.lines().map(Dependency::parse).collect();

    let mut steps: Vec<_> = {
        let mut step_deps = HashMap::new();
        for dep in deps {
            // make sure the dep exists
            step_deps.entry(dep.require).or_insert_with(Vec::new);

            step_deps.entry(dep.next)
                .or_insert_with(Vec::new)
                .push(dep.require);
        }

        step_deps.into_iter()
            .map(|(id, deps)| Step { id, deps })
            .collect()
    };

    steps.sort_by_key(|step| step.id);

    for step in steps.iter() {
        println!("{:?}", step);
    }

    let mut todo = steps.clone();
    let mut completed_steps = Vec::new();
    let mut elf_jobs: [Option<Job>; ELF_COUNT] = [None; ELF_COUNT];

    for time in 0.. {
        // finish running jobs
        for elf in 0..elf_jobs.len() {
            if let Some(job) = &elf_jobs[elf] {
                if time >= job.finish_time {
                    println!("elf {} completed step {} at {}", elf, job.id, time);

                    completed_steps.push(job.id);
                    elf_jobs[elf] = None;
                }
            }
        }

        if todo.is_empty() && elf_jobs.iter().all(Option::is_none) {
            break;
        }

        // find free elves to assign jobs
        let free_elves: Vec<_> = elf_jobs.iter()
            .enumerate()
            .filter_map(|(elf, job)| if job.is_none() {
                Some(elf)
            } else {
                None
            })
            .collect();

        for elf in free_elves {
            if todo.is_empty() {
                break;
            }

            let next = todo.iter()
                .position(|step| step.deps.iter().all(|dep| {
                    completed_steps.iter().any(|completed| *completed == *dep)
                }));

            if let Some(next) = next {
                let next_step = todo.remove(next);
                let finish_time = time + next_step.duration();

                elf_jobs[elf] = Some(Job { id: next_step.id, finish_time });
                println!("elf {} started job {} at {}", elf, next_step.id, time);
            }
        }
    }

    println!("sequence: {}", completed_steps.into_iter()
        .collect::<String>()
        .to_ascii_uppercase());
}