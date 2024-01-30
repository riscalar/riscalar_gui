pub trait BranchPredictor {
    fn lookup(&self, baddr: &u64, btarget: &u64) -> u64;
}

pub struct BranchPredictorTaken;

impl BranchPredictor for BranchPredictorTaken {
    fn lookup(&self, _baddr: &u64, btarget: &u64) -> u64 {
        *btarget
    }
}

pub struct BranchPredictorNotTaken;

impl BranchPredictor for BranchPredictorNotTaken {
    fn lookup(&self, baddr: &u64, _btarget: &u64) -> u64 {
        baddr + 4
    }
}
