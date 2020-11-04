#[derive(Debug)]
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
        if self.all_scores.len() == 0 {
            None
        } else {
            Some(*self.all_scores.last().expect("None"))
        }
    }

    pub fn personal_best(&self) -> Option<u32> {
        if self.all_scores.len() == 0 {
            return None;
        }

        let mut top = self.all_scores.clone();
        top.sort_by(|a, b| b.cmp(a));

        Some(top[0])
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        if self.all_scores.len() == 0 {
            return vec![] as Vec<u32>;
        }

        let mut top = self.all_scores.clone();
        top.sort_by(|a, b| b.cmp(a));
        
        top[0..min(3, top.len())].to_vec()
    }
}
