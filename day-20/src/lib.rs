use std::collections::{HashMap, VecDeque};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, char as nomchar, newline},
    multi::separated_list1,
    sequence::preceded,
    IResult,
};

pub fn process_part1(input: &str) -> String {
    let (_, mut configuration) = parse_input(input).unwrap();
    let (mut low, mut high) = (0, 0);
    let mut dummy_tracker = Tracker::dummy();
    for _ in 0..1000 {
        let (dlow, dhigh) = configuration.push_button(&mut dummy_tracker);
        (low, high) = (low + dlow, high + dhigh);
    }
    let total = low * high;
    total.to_string()
}

pub fn process_part2(input: &str) -> String {
    let (_, mut configuration) = parse_input(input).unwrap();
    let mut tracker = Tracker::new(&configuration);
    while !tracker.done_tracking() {
        configuration.push_button(&mut tracker);
    }
    tracker.calculate().to_string()
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Signal {
    Low,
    High,
}

#[derive(Debug)]
struct Pulse<'a> {
    source: &'a str,
    signal: Signal,
    destination: &'a str,
}

#[derive(Debug, PartialEq, Eq)]
enum Module<'a> {
    Broadcaster(&'a str, Vec<&'a str>),
    FlipFlop(&'a str, bool, Vec<&'a str>),
    Conjunction(&'a str, HashMap<&'a str, Signal>, Vec<&'a str>),
}

impl<'a, 'it> Module<'a> {
    fn parse_broadcaster(input: &'a str) -> IResult<&'a str, Self> {
        let (input, id) = tag("broadcaster")(input)?;
        Ok((input, Self::Broadcaster(id, Vec::new())))
    }

    fn parse_flipflop(input: &'a str) -> IResult<&'a str, Self> {
        let (input, id) = preceded(nomchar('%'), alpha1)(input)?;
        Ok((input, Self::FlipFlop(id, false, Vec::new())))
    }

    fn parse_conjunction(input: &'a str) -> IResult<&'a str, Self> {
        let (input, id) = preceded(nomchar('&'), alpha1)(input)?;
        Ok((input, Self::Conjunction(id, HashMap::new(), Vec::new())))
    }

    fn parse_destinations(input: &'a str) -> IResult<&'a str, Vec<&'a str>> {
        let (input, destinations) =
            preceded(tag(" -> "), separated_list1(tag(", "), alpha1))(input)?;
        Ok((input, destinations))
    }

    fn fill_module(module: Self, destinations: Vec<&'a str>) -> Self {
        let mut module = module;
        match &mut module {
            Module::Broadcaster(_, v) => std::mem::replace(v, destinations),
            Module::FlipFlop(_, _, v) => std::mem::replace(v, destinations),
            Module::Conjunction(_, _, v) => std::mem::replace(v, destinations),
        };
        module
    }

    fn parse(input: &'a str) -> IResult<&str, Self> {
        let (input, mut module) = alt((
            Self::parse_broadcaster,
            Self::parse_flipflop,
            Self::parse_conjunction,
        ))(input)?;
        let (input, destinations) = Self::parse_destinations(input)?;
        module = Self::fill_module(module, destinations);
        Ok((input, module))
    }

    fn id(&'_ self) -> &'a str {
        match self {
            Module::Broadcaster(id, _) => id,
            Module::FlipFlop(id, _, _) => id,
            Module::Conjunction(id, _, _) => id,
        }
    }

    fn iter(&'it self) -> ModuleIterator<'a, 'it> {
        ModuleIterator {
            module: self,
            index: 0,
        }
    }

    fn signal(&mut self, signal: Signal, signal_source: &'a str) -> Vec<Pulse<'a>> {
        match self {
            Module::Broadcaster(source, v) => v
                .iter()
                .map(|&destination| Pulse {
                    source,
                    signal,
                    destination,
                })
                .collect(),
            Module::FlipFlop(source, state, v) => match signal {
                Signal::Low => {
                    *state = !*state;
                    let signal = if *state { Signal::High } else { Signal::Low };
                    v.iter()
                        .map(|&destination| Pulse {
                            source,
                            signal,
                            destination,
                        })
                        .collect()
                }
                Signal::High => Vec::new(),
            },
            Module::Conjunction(source, inputs, v) => {
                *inputs
                    .get_mut(signal_source)
                    .expect("source must be captured when initialised") = signal;
                let signal = if inputs.values().all(|&s| s == Signal::High) {
                    Signal::Low
                } else {
                    Signal::High
                };
                v.iter()
                    .map(|&destination| Pulse {
                        source,
                        signal,
                        destination,
                    })
                    .collect()
            }
        }
    }
}

struct ModuleIterator<'m, 'it> {
    module: &'it Module<'m>,
    index: usize,
}

impl<'m, 'it> Iterator for ModuleIterator<'m, 'it> {
    type Item = &'m str;

    fn next(&mut self) -> Option<Self::Item> {
        let v = match self.module {
            Module::Broadcaster(_, v) => v,
            Module::FlipFlop(_, _, v) => v,
            Module::Conjunction(_, _, v) => v,
        };

        if self.index < v.len() {
            self.index += 1;
            Some(v[self.index - 1])
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct Configuration<'a> {
    ids: HashMap<&'a str, usize>,
    modules: Vec<Module<'a>>,
    pulses: VecDeque<Pulse<'a>>,
    cyclers: Vec<&'a str>,
}

impl<'a> Configuration<'a> {
    fn new(modules: Vec<Module<'a>>) -> Self {
        let mut modules = modules;
        let mut ids = HashMap::new();
        let mut inputs: HashMap<&str, Vec<&str>> = HashMap::new();
        // find the inputs of each module
        modules.iter().enumerate().for_each(|(i, m)| {
            ids.insert(m.id(), i);
            m.iter().for_each(|dest| {
                inputs
                    .entry(dest)
                    .and_modify(|v| v.push(m.id()))
                    .or_insert(vec![m.id()]);
            });
        });
        // make Conjunction modules 'aware' of their inputs
        modules.iter_mut().for_each(|m| match m {
            Module::Broadcaster(_, _) => {}
            Module::FlipFlop(_, _, _) => {}
            Module::Conjunction(id, map, _) => {
                inputs
                    .get(id)
                    .expect("module id must exist")
                    .iter()
                    .for_each(|&source| {
                        map.insert(source, Signal::Low);
                    });
            }
        });
        // there's a single conjunction node that outputs to rx
        let final_boss = modules
            .iter()
            .find(|m| m.iter().any(|destination| destination == "rx"))
            .expect("Something must output to rx!")
            .id();
        // find the modules that output to the final boss:
        let mut cyclers = Vec::new();
        modules.iter().for_each(|m| {
            if m.iter().any(|destination| destination == final_boss) {
                cyclers.push(m.id())
            }
        });

        Self {
            ids,
            modules,
            pulses: VecDeque::new(),
            cyclers,
        }
    }

    /// (low pulses, high pulses)
    fn push_button(&mut self, tracker: &mut Tracker) -> (usize, usize) {
        let pulses = &mut self.pulses;
        tracker.push_button();
        pulses.push_back(Pulse {
            source: "button",
            signal: Signal::Low,
            destination: "broadcaster",
        });
        let (mut low, mut high) = (0, 0);
        while let Some(pulse) = pulses.pop_front() {
            tracker.track(&pulse);
            match pulse.signal {
                Signal::Low => low += 1,
                Signal::High => high += 1,
            }
            if let Some(&target_idx) = self.ids.get(pulse.destination) {
                let target_module = self
                    .modules
                    .get_mut(target_idx)
                    .expect("destination module must exist");
                let new_pulses = target_module.signal(pulse.signal, pulse.source);
                new_pulses.into_iter().for_each(|p| pulses.push_back(p));
            }
        }

        (low, high)
    }
}

#[derive(Debug)]
struct Tracker<'a> {
    /// (module, cycle length)
    tracked_modules: Vec<(&'a str, usize)>,
    button_presses: usize,
}

impl<'a> Tracker<'a> {
    fn new(configuration: &Configuration<'a>) -> Self {
        Self {
            tracked_modules: configuration
                .cyclers
                .iter()
                .cloned()
                .map(|module: &'a str| (module, 0))
                .collect(),
            button_presses: 0,
        }
    }

    // empty tracker for part 1
    fn dummy() -> Self {
        Self {
            tracked_modules: Vec::new(),
            button_presses: 0,
        }
    }

    fn track(&mut self, pulse: &Pulse) {
        if self.tracked_modules.is_empty() {
            return;
        }
        if let Pulse {
            source,
            signal: Signal::High,
            destination: _,
        } = pulse
        {
            if let Some((_, count)) = self
                .tracked_modules
                .iter_mut()
                .find(|(module, _)| module == source)
            {
                if count == &0 {
                    *count = self.button_presses
                }
            }
        }
    }

    fn push_button(&mut self) {
        self.button_presses += 1;
    }

    /// returns true if we've seen a high signal from all tracked modules at least once
    fn done_tracking(&self) -> bool {
        // dbg!(&self.tracked_modules);
        self.tracked_modules.iter().all(|(_, count)| count != &0)
    }

    /// calculate the minimum cycle length that gets all tracked modules to output a
    /// high signal at the same time (and thereby activate rx)
    fn calculate(&self) -> usize {
        self.tracked_modules
            .iter()
            .map(|(_, count)| *count)
            .product()
    }
}

fn parse_input(input: &str) -> IResult<&str, Configuration> {
    let (input, modules) = separated_list1(newline, Module::parse)(input)?;
    assert!(input.is_empty());
    let configuration = Configuration::new(modules);
    Ok((input, configuration))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_1() {
        let input = "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";
        let result = process_part1(input);
        assert_eq!(result, "32000000");
    }

    #[test]
    fn part1_2() {
        let input = "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output";
        let result = process_part1(input);
        assert_eq!(result, "11687500");
    }
}
