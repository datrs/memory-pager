use self::Operation::*;
use memory_pager::Pager;
use quickcheck::{quickcheck, Arbitrary, Gen};
use rand::Rng;

const MAX_LENGTH: usize = 50;
const PAGE_LEN: usize = 10;

#[derive(Clone, Debug)]
enum Operation {
  GetMutOrAlloc { index: usize },
  Get { index: usize },
}

impl Arbitrary for Operation {
  fn arbitrary<G: Gen>(g: &mut G) -> Self {
    let index: usize = g.gen_range(0, MAX_LENGTH);

    if g.gen::<bool>() {
      GetMutOrAlloc { index }
    } else {
      Get { index }
    }
  }
}

quickcheck! {
  fn implementation_matches_model(ops: Vec<Operation>) -> bool{
    let mut pager = Pager::new(PAGE_LEN);
    let mut model = vec![false; MAX_LENGTH];

    for op in ops {
      match op {
        GetMutOrAlloc { index } => {
          pager.get_mut_or_alloc(index);
          model[index] = true;
        },
        Get { index } => {
          let page = pager.get(index);
          if model[index] {
            if !page.is_some() {
              return false;
            }
          } else {
            if !page.is_none() {
              return false;
            }
          }
        }
      }
    }

    true
  }
}
