#![allow(dead_code)]
use core::fmt;
use std::fmt::{Debug, Display};

use pnets::{ arc::Kind, timed::{time_range::TimeRange, Net, Bound}, PlaceId, TransitionId };

#[derive(Debug, Default)]
pub struct SeqPnet(Net);

#[derive(Debug)]
pub enum SeqPnetErr {
    NoStates,
    NotEnoughTransitions,
    NoNextPlace(PlaceId),
    InvalidPlace(PlaceId),
    InvalidTransition(TransitionId),
}

impl SeqPnet {
    pub fn create_place(&mut self) {
        match self.0.places.len() {
            0 => { self.0.create_place(); },
            _ => {
                let default_time = TimeRange { start : Bound::Closed(0), end : Bound::Closed(0) };
                let last_place = self.last_place();
                let transition_id = self.0.create_transition();
                self.add_transition_time_id(default_time, transition_id).unwrap();
                let place_id = self.0.create_place();

                // Connect the End of the Petri Net to the transitions
                let consume_arc = Kind::Consume(last_place, transition_id, 1);
                let _ =self.0.add_arc(consume_arc);

                // Attach the new node to the transition
                let produce_arc = Kind::Produce(place_id, transition_id, 1);
                let _ =self.0.add_arc(produce_arc);
            },
        }
    }

    pub fn create_n_places(&mut self, n: usize) {
        for _ in 0..n { self.create_place(); }
    }

    fn last_place(&self) -> PlaceId {
        self.0.places.iter()
                     .last().map(|x| x.id())
                     .unwrap()
    }

    fn last_transition(&self) -> TransitionId {
        self.0.transitions.iter()
                     .last().map(|x| x.id())
                     .unwrap()
    }

    fn next_transition_id(&self, curr: PlaceId) -> Result<TransitionId, SeqPnetErr> {
        match self.0.places.get(curr) {
            None => Err(SeqPnetErr::InvalidPlace(curr)),
            Some(place) => {
                if place.consumed_by.is_empty() {
                    Err(SeqPnetErr::NoNextPlace(curr))
                } else {
                    Ok(place.consumed_by.iter().last().unwrap().0)
                }
            }
        }
    }

    fn next_place_id(&self, curr: PlaceId) -> Result<PlaceId, SeqPnetErr> {
        let next_trans = self.next_transition_id(curr)?;
        let trans = self.0.transitions.get(next_trans).unwrap();
        let next : PlaceId = trans.produce.iter().last().unwrap().0;
        Ok(next)
    }

    pub fn delete_place(&mut self) -> Result<(), SeqPnetErr> {
        if self.0.places.is_empty() { // Nothing to delete
            Err(SeqPnetErr::NoStates)
        } else if self.0.transitions.is_empty() { // Single Node
            self.0.places.clear();
            Ok(())
        } else {
            self.0.places.truncate(self.0.places.len() - 1);
            self.0.transitions.truncate(self.0.transitions.len() - 1);
            Ok(())
        }
    }

    pub fn add_transition_time_index(&mut self, range: TimeRange, index: usize) -> Result<(), SeqPnetErr> {
        let mut i = 0;
        for transition in self.0.transitions.iter_mut() {
            if i == index {
                transition.time = range;
                return Ok(());
            }
            i = i + 1;
        }
        Err(SeqPnetErr::NotEnoughTransitions)
    }

    pub fn add_transition_time_id(&mut self, range: TimeRange, id: TransitionId) -> Result<(), SeqPnetErr> {
        match self.0.transitions.get_mut(id) {
            None => Err(SeqPnetErr::InvalidTransition(id)),
            Some(trans) => { trans.time = range; Ok(()) }
        }
    }
}

impl Display for SeqPnet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0.places.len() {
            0 => writeln!(f, "Sequential Time Petri Net {{}}"),
            1 => writeln!(f, "Sequential Time Petri Net {{ {} }}", self.last_place()),
            _ => {
                writeln!(f, "Sequential Time Petri Net {{")?;
                for place in self.0.places.iter() {
                    if place.consumed_by.is_empty() { continue; }
                    let curr = place.id();
                    let trans_id = self.next_transition_id(curr).unwrap();
                    let trans = self.0.transitions.get(trans_id).unwrap();
                    let interval = show(trans.time);
                    let next = self.next_place_id(curr).unwrap();
                    writeln!(f, "\t{curr} --{interval}--> {next}")?;
                }
                writeln!(f, "}}")
            }
        }
    }
}

fn show(tr : TimeRange) -> String {
    let start = match tr.start {
        Bound::Open(x) => format!("({x}, "),
        Bound::Closed(x) => format!("[{x}, "),
        Bound::Infinity => format!("(inf, "),
    };
    let end = match tr.end {
        Bound::Open(x) => format!("{x})"),
        Bound::Closed(x) => format!("{x}]"),
        Bound::Infinity => format!("inf)"),
    };
    return start + &end;
}
