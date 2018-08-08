use std::collections::VecDeque;
use std::iter::Iterator;

pub type Price = i32;
pub type Size = u32;
pub type Meta = u128;

#[derive(Debug)]
pub struct Basket {
    inner: Vec<(Price, VecDeque<(Size, Meta)>)>,
}

impl Basket {
    pub fn new() -> Basket {
        Basket { inner: Vec::new() }
    }

    pub fn put(&mut self, price: i32, size: u32, meta: u128) {
        match self.inner.iter()
            .position(|(p, _)| *p >= price) {
            Some(i) => {
                if self.inner[i].0 == price {
                    self.inner[i].1.push_back((size, meta));
                } else {
                    self.inner.insert(i, (price, Self::deque((size, meta))))
                }
            }
            None => {
                self.inner.push((price, Self::deque((size, meta))));
            }
        }
    }

    pub fn split(&mut self, price: i32, size: u32) -> Basket {
        let mut price_index = -1;
        let mut size_index = -1;
        let mut size_sum = 0;
        'outer: for (p, size_vec) in self.inner.iter() {
            if *p > price {
                break;
            }
            price_index += 1;
            size_index = -1;
            for (s, _) in size_vec.iter() {
                size_sum += s;
                if size_sum > size {
                    break 'outer;
                }
                size_index += 1;
            }
        }

        if price_index == -1 {
            Basket::new()
        } else {
            let price_index = price_index as usize;
            let mut new = self.inner.drain(0..price_index).collect::<Vec<_>>();

            if size_index >= 0 {
                let size_index = size_index as usize;
                if self.inner[0].1.len() <= size_index + 1 {
                    new.push(self.inner.remove(0));
                } else {
                    let mut first = &mut self.inner[0];
                    new.push((first.0.clone(), first.1.drain(0..size_index + 1).collect::<VecDeque<_>>()));
                }
            }

            Basket {
                inner: new
            }
        }
    }

    pub fn split_1(&mut self, price: i32, size: u32) -> Basket {
        let sums = self.inner.iter()
            .filter(|(p, _)| *p <= price)
            .map(|(_, s_vec)| s_vec.iter().map(|(size, _)| *size).fold(0, |acc, s| acc + s))
            .scan(0, |sum, size_sum| {
                *sum += size_sum;
                Some(*sum)
            })
            .take_while(|sum| sum <= &size)
            .collect::<Vec<_>>();

        if sums.len() == 0 {
            Basket::new()
        } else {
            let mut new = self.inner.drain(0..sums.len()).collect::<Vec<_>>();

            let already_moved = sums.last().unwrap(); // The last element must be awarded.
            if self.inner.get(0).filter(|(p, _)| *p <= price).is_some() {
               let to_move_count: usize = self.inner[0].1.iter().scan(*already_moved, |sum, (size, _)| {
                    *sum += size;
                    Some(*sum)
                }).take_while(|sum| sum <= &size)
                    .map(|_| 1)
                    .sum();

                if to_move_count == self.inner[0].1.len() {
                    new.push(self.inner.remove(0));
                } else {
                    let mut first = &mut self.inner[0];
                    new.push((first.0.clone(), first.1.drain(0..to_move_count).collect::<VecDeque<_>>()));
                }
            }

            Basket {
                inner: new
            }
        }
    }

    pub fn deque(item: (Size, Meta)) -> VecDeque<(Size, Meta)> {
        let mut deq = VecDeque::new();
        deq.push_back(item);
        deq
    }
}

#[test]
fn put_test() {
    let mut basket = Basket::new();
    basket.put(20, 1, 101);
    basket.put(12, 2, 102);
    basket.put(20, 3, 103);
    basket.put(5, 4, 103);
    basket.put(10, 5, 104);
    basket.put(1, 6, 105);
    basket.put(2, 7, 106);
    basket.put(10, 8, 107);
    basket.put(20, 9, 108);
    basket.put(5, 10, 109);
    println!("basket:{:?}", basket);
    check_basket(&basket, vec![
        (1, vec![&(6, 105)]),
        (2, vec![&(7, 106)]),
        (5, vec![&(4, 103), &(10, 109)]),
        (10, vec![&(5, 104), &(8, 107)]),
        (12, vec![&(2, 102)]),
        (20, vec![&(1, 101), &(3, 103), &(9, 108)]),
    ]);

    let mut basket = Basket::new();
    basket.put(20, 1, 101);
    basket.put(21, 2, 102);
    basket.put(21, 3, 103);
    basket.put(22, 4, 104);
    basket.put(23, 5, 105);
    println!("basket:{:?}", basket);
    check_basket(&basket, vec![
        (20, vec![&(1, 101)]),
        (21, vec![&(2, 102), &(3, 103)]),
        (22, vec![&(4, 104)]),
        (23, vec![&(5, 105)]),
    ]);

    let mut basket = Basket::new();
    basket.put(23, 5, 105);
    basket.put(22, 4, 104);
    basket.put(21, 2, 102);
    basket.put(21, 3, 103);
    basket.put(20, 1, 101);
    println!("basket:{:?}", basket);
    check_basket(&basket, vec![
        (20, vec![&(1, 101)]),
        (21, vec![&(2, 102), &(3, 103)]),
        (22, vec![&(4, 104)]),
        (23, vec![&(5, 105)]),
    ]);
}

#[test]
fn split_test() {
    let mut basket = Basket::new();
    let other = basket.split(1000, 1000);
    check_basket(&basket, vec![]);
    check_basket(&other, vec![]);

    basket.put(20, 1, 101);
    basket.put(21, 1, 102);
    basket.put(21, 1, 103);
    basket.put(21, 1, 104);
    basket.put(22, 1, 105);
    basket.put(23, 2, 106);
    let new = basket.split(21, 3);
    check_basket(&new, vec![(20, vec![&(1, 101)]), (21, vec![&(1, 102), &(1, 103)])]);
    check_basket(&basket, vec![(21, vec![&(1, 104)]), (22, vec![&(1, 105)]), (23, vec![&(2, 106)])]);

    let mut basket = Basket::new();
    basket.put(20, 1, 101);
    let new = basket.split(21, 3);

    check_basket(&new, vec![(20, vec![&(1, 101)])]);
    check_basket(&basket, vec![]);

    let mut basket = Basket::new();
    basket.put(20, 1, 101);
    let new = basket.split(1, 3);

    check_basket(&new, vec![]);
    check_basket(&basket, vec![(20, vec![&(1, 101)])]);

    let mut basket = Basket::new();
    basket.put(20, 1, 101);
    basket.put(21, 1, 102);
    basket.put(21, 1, 103);
    basket.put(21, 1, 104);
    basket.put(22, 1, 105);
    basket.put(23, 2, 106);
    let new = basket.split(22, 33);
    check_basket(&new, vec![(20, vec![&(1, 101)]), (21, vec![&(1, 102), &(1, 103), &(1, 104)]), (22, vec![&(1, 105)])]);
    check_basket(&basket, vec![(23, vec![&(2, 106)])]);
}


#[test]
fn split_1_test() {
    let mut basket = Basket::new();
    let other = basket.split_1(1000, 1000);
    check_basket(&basket, vec![]);
    check_basket(&other, vec![]);

    basket.put(20, 1, 101);
    basket.put(21, 1, 102);
    basket.put(21, 1, 103);
    basket.put(21, 1, 104);
    basket.put(22, 1, 105);
    basket.put(23, 2, 106);
    let new = basket.split_1(21, 3);
    check_basket(&new, vec![(20, vec![&(1, 101)]), (21, vec![&(1, 102), &(1, 103)])]);
    check_basket(&basket, vec![(21, vec![&(1, 104)]), (22, vec![&(1, 105)]), (23, vec![&(2, 106)])]);

    let mut basket = Basket::new();
    basket.put(20, 1, 101);
    let new = basket.split_1(21, 3);

    check_basket(&new, vec![(20, vec![&(1, 101)])]);
    check_basket(&basket, vec![]);

    let mut basket = Basket::new();
    basket.put(20, 1, 101);
    let new = basket.split_1(1, 3);

    check_basket(&new, vec![]);
    check_basket(&basket, vec![(20, vec![&(1, 101)])]);

    let mut basket = Basket::new();
    basket.put(20, 1, 101);
    basket.put(21, 1, 102);
    basket.put(21, 1, 103);
    basket.put(21, 1, 104);
    basket.put(22, 1, 105);
    basket.put(23, 2, 106);
    let new = basket.split_1(22, 33);
    check_basket(&new, vec![(20, vec![&(1, 101)]), (21, vec![&(1, 102), &(1, 103), &(1, 104)]), (22, vec![&(1, 105)])]);
    check_basket(&basket, vec![(23, vec![&(2, 106)])]);
}

fn check_basket(basket: &Basket, inner: Vec<(i32, Vec<&(u32, u128)>)>) {
    let basket = basket.inner
        .iter()
        .map(|(p, v)| (p.clone(), v.iter().collect::<Vec<_>>()))
        .collect::<Vec<_>>();
    assert_eq!(basket, inner);
}