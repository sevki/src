trait PartialEq {
    fn eq(&self, other: &Self) -> bool;
    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

trait PartialOrd: PartialEq {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering>;
    fn lt(&self, other: &Self) -> bool {
        matches!(self.partial_cmp(other), Some(Ordering::Less))
    }
    fn le(&self, other: &Self) -> bool {
        matches!(self.partial_cmp(other), Some(Ordering::Less) | Some(Ordering::Equal))
    }
    fn gt(&self, other: &Self) -> bool {
        matches!(self.partial_cmp(other), Some(Ordering::Greater))
    }
    fn ge(&self, other: &Self) -> bool {
        matches!(self.partial_cmp(other), Some(Ordering::Greater) | Some(Ordering::Equal))
    }
}