use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum PoolCreationError {
    Zero,
    TooMany(usize),
}

#[derive(Debug, Clone, PartialEq)]
pub struct PoolPlan {
    size: usize,
}

impl fmt::Display for PoolCreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PoolCreationError::Zero => write!(f, "pool size must be positive"),
            PoolCreationError::TooMany(n) => write!(f, "pool size {n} exceeds 64"),
        }
    }
}

impl PoolPlan {
    pub fn build(size: usize) -> Result<PoolPlan, PoolCreationError> {
        match size {
            0 => Err(PoolCreationError::Zero),
            n if n > 64 => Err(PoolCreationError::TooMany(n)),
            n => Ok(PoolPlan { size: n }),
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn worker_ids(&self) -> Vec<usize> {
        (0..self.size).collect()
    }

    pub fn assign(&self, job: usize) -> usize {
        job % self.size
    }

    pub fn spread(&self, jobs: usize) -> Vec<usize> {
        (0..self.size)
            .map(|w| jobs / self.size + usize::from(w >= self.size - jobs % self.size))
            .collect()
    }
}
