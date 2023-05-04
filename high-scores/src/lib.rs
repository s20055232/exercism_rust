#[derive(Debug)]
pub struct HighScores {
    scores: Vec<u32>,
}

impl HighScores {
    pub fn new(scores: &[u32]) -> Self {
        HighScores {
            scores: scores.to_vec(),
        }
        // unimplemented!("Construct a HighScores struct, given the scores: {scores:?}")
    }

    pub fn scores(&self) -> &[u32] {
        &self.scores
        // unimplemented!("Return all the scores as a slice")
    }

    pub fn latest(&self) -> Option<u32> {
        self.scores.last().copied()
        // unimplemented!("Return the latest (last) score")
    }

    pub fn personal_best(&self) -> Option<u32> {
        self.scores.iter().max().copied()
        // unimplemented!("Return the highest score")
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut scores = self.scores.clone();
        scores.sort_unstable();
        scores.reverse();
        let tmp = 3.min(scores.len());
        scores[..tmp].to_vec()
        // unimplemented!("Return 3 highest scores")
    }
}
