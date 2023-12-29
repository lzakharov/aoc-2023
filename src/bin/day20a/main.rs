use std::collections::{HashMap, VecDeque};
use std::io::BufRead;
use std::{fs::File, io};

fn main() {
    let file = File::open("input/day20.txt").unwrap();
    let reader = io::BufReader::new(file);

    let mut modules: HashMap<String, Module> = HashMap::new();
    let mut inputs: HashMap<String, Vec<String>> = HashMap::new();

    for line in reader.lines() {
        if let Ok(line) = line {
            let (module, outputs) = line.split_once(" -> ").unwrap();
            let outputs: Vec<String> = outputs.split(", ").map(|s| s.to_string()).collect();

            let start = module.starts_with(['%', '&']) as usize;

            for output in outputs.iter() {
                inputs
                    .entry(output.to_string())
                    .or_insert(Vec::new())
                    .push(module[start..].to_string());
            }

            if module.starts_with('%') {
                modules.insert(
                    module[1..].to_string(),
                    Module {
                        module_type: ModuleType::FlipFlop(false),
                        outputs: outputs,
                    },
                );
            } else if module.starts_with('&') {
                modules.insert(
                    module[1..].to_string(),
                    Module {
                        module_type: ModuleType::Conjunction(HashMap::new()),
                        outputs: outputs,
                    },
                );
            } else {
                modules.insert(
                    module.to_string(),
                    Module {
                        module_type: ModuleType::Broadcaster,
                        outputs: outputs,
                    },
                );
            }
        }
    }

    for (name, module) in &mut modules {
        if let ModuleType::Conjunction(mem) = &mut module.module_type {
            for input in inputs.remove(name).unwrap() {
                mem.insert(input, Pulse::Low);
            }
        }
    }

    let mut low = 0;
    let mut high = 0;

    for _ in 0..1000 {
        let mut q = VecDeque::new();

        low += 1;
        q.push_back((String::new(), String::from("broadcaster"), Pulse::Low));

        while let Some((parent, name, pulse)) = q.pop_front() {
            if let Some(module) = modules.get_mut(&name) {
                if pulse == Pulse::High {
                    if let ModuleType::FlipFlop(_) = &module.module_type {
                        continue;
                    }
                }

                let mut pulse = pulse.clone();

                match &mut module.module_type {
                    ModuleType::Broadcaster => {}
                    ModuleType::FlipFlop(on) => {
                        let on = *on;
                        pulse = if !on { Pulse::High } else { Pulse::Low };
                        module.module_type = ModuleType::FlipFlop(!on);
                    }
                    ModuleType::Conjunction(mem) => {
                        mem.insert(parent, pulse);
                        pulse = if mem.values().all(|pulse| *pulse == Pulse::High) {
                            Pulse::Low
                        } else {
                            Pulse::High
                        };
                    }
                }

                if pulse == Pulse::Low {
                    low += module.outputs.len();
                } else {
                    high += module.outputs.len();
                }
                module
                    .outputs
                    .iter()
                    .for_each(|next| q.push_back((name.clone(), next.clone(), pulse)));
            }
        }
    }

    println!("{}", low * high)
}

#[derive(Debug)]
struct Module {
    module_type: ModuleType,
    outputs: Vec<String>,
}

#[derive(Debug, PartialEq, Eq)]
enum ModuleType {
    Broadcaster,
    FlipFlop(bool),
    Conjunction(HashMap<String, Pulse>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pulse {
    Low,
    High,
}
