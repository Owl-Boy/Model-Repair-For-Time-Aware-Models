mod seq_pnet;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use crate::seq_pnet::*;

    #[test]
    fn make_seq_pnet() {
        let mut seq_pnet : SeqPnet = Default::default();
        seq_pnet.create_n_places(5);
        println!("{}", seq_pnet);
        assert_eq!(4, 4);
    }
}
