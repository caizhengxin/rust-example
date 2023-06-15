use std::collections::BinaryHeap;


#[derive(Debug, Clone, PartialEq, Eq, Ord)]
pub struct JanKinCai {
    pub id: u32,
    pub priority: u32,
    pub active: bool,
}


impl PartialOrd for JanKinCai {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.priority.cmp(&other.priority))
    }
}


fn main() {
    let mut heap = BinaryHeap::new();

    heap.push(JanKinCai{id: 1, priority: 100, active: true});
    heap.push(JanKinCai{id: 2, priority: 90, active: true});
    heap.push(JanKinCai{id: 3, priority: 110, active: true});
    heap.push(JanKinCai{id: 4, priority: 50, active: true});

    assert_eq!(heap.peek().unwrap(), &JanKinCai{id: 3, priority: 110, active: true});
    assert_eq!(heap.len(), 4);

    // 注意事项：遍历顺序和pop顺序是不一样的, 遍历顺序是乱序

    for value in &heap {
        println!("{value:?}");
    }
    // JanKinCai { id: 3, priority: 110, active: true }
    // JanKinCai { id: 2, priority: 90, active: true }
    // JanKinCai { id: 1, priority: 100, active: true }
    // JanKinCai { id: 4, priority: 50, active: true }

    println!("{:?}", heap.pop());
    println!("{:?}", heap.pop());
    println!("{:?}", heap.pop());
    println!("{:?}", heap.pop());
    // Some(JanKinCai { id: 3, priority: 110, active: true })
    // Some(JanKinCai { id: 1, priority: 100, active: true })
    // Some(JanKinCai { id: 2, priority: 90, active: true })
    // Some(JanKinCai { id: 4, priority: 50, active: true })
}
