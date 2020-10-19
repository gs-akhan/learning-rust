pub struct HighScores {
    all_scores: Vec<u32>,
}

impl HighScores {
    pub fn new(scores: &[u32]) -> Self {
        Self {
            all_scores: scores.to_vec(),
        }
    }

    pub fn scores(&self) -> &[u32] {
        &self.all_scores
    }

    pub fn latest(&self) -> Option<u32> {
        Some(10 as u32)
    }

    pub fn personal_best(&self) -> Option<u32> {
        unimplemented!("Return the highest score")
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        unimplemented!("Return 3 highest scores")
    }
}

fn main() {
    println!("Hello Structs with Methods !!");
    let a = HighScores::new(&[30, 50, 20, 70]);
    a.scores().iter().for_each(|x| println!("{}", x));
    println!("Last {}", a.latest().unwrap());
}
