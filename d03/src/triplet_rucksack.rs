use crate::Rucksack;

type Sacklet = Box<[u8]>;

pub struct TripletRucksack(Sacklet, Sacklet, Sacklet);

impl Rucksack for TripletRucksack {
    fn duped_values(&self) -> u32 {
        const N: usize = 53;
        let mut data = [0u8; N];
        for i in self.0.iter() {
            data[*i as usize] |= 0b001
        }
        for i in self.1.iter() {
            data[*i as usize] |= 0b010
        }
        for i in self.2.iter() {
            if data[*i as usize] == 0b011 {
                return *i as u32;
            }
        }

        panic!("no duped values in triplet rucksack")
    }

    fn into_vec(input: &str) -> Vec<Self> {
        let mut sacklets = input
            .lines()
            .map(|line| line.bytes().map(Self::priority).collect::<Sacklet>());

        let mut resp = Vec::new();
        loop {
            let sack = match (sacklets.next(), sacklets.next(), sacklets.next()) {
                (Some(s1), Some(s2), Some(s3)) => TripletRucksack(s1, s2, s3),
                (None, None, None) => break,
                _ => panic!("incorrect total rucksacks"),
            };
            resp.push(sack);
        }
        resp
    }
}
