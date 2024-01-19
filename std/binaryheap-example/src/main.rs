#![feature(binary_heap_into_iter_sorted)]
use std::collections::BinaryHeap;
use std::cell::{RefCell, Cell};


#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord)]
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


fn test_binary_heap_refcell_update() {
    let mut heap = BinaryHeap::new();

    // 插入数据
    heap.push(RefCell::new(JanKinCai{id: 1, priority: 100, active: true}));
    heap.push(RefCell::new(JanKinCai{id: 2, priority: 100, active: true}));
    heap.push(RefCell::new(JanKinCai{id: 3, priority: 100, active: true}));

    // 更新id为1的数据
    for value in &heap {
        if value.borrow().id == 1 {
            value.replace(JanKinCai{id: 1, priority: 100, active: false});
        }
    }

    // 删除id为2的数据
    heap.retain(|v| v.borrow().id != 2);

    // 排序遍历顺序
    for value in heap.clone().into_iter_sorted() {
        println!("iter value, {value:?}");
    }
}


fn test_binary_heap_cell_update() {
    let mut heap = BinaryHeap::new();

    // 插入数据
    heap.push(Cell::new(JanKinCai{id: 1, priority: 100, active: true}));
    heap.push(Cell::new(JanKinCai{id: 2, priority: 100, active: true}));
    heap.push(Cell::new(JanKinCai{id: 3, priority: 100, active: true}));

    // 更新id为1的数据
    for value in &heap {
        if value.get().id == 1 {
            value.replace(JanKinCai{id: 1, priority: 100, active: false});
        }
    }

    // 删除id为2的数据
    heap.retain(|v| v.get().id != 2);

    // 排序遍历顺序
    for value in heap.clone().into_iter_sorted() {
        println!("iter value, {value:?}");
    }
}


fn main() {
    let mut heap = BinaryHeap::new();

    // 插入数据
    heap.push(JanKinCai{id: 1, priority: 100, active: true});
    heap.push(JanKinCai{id: 2, priority: 90, active: true});
    heap.push(JanKinCai{id: 3, priority: 110, active: true});
    heap.push(JanKinCai{id: 4, priority: 50, active: true});
    heap.push(JanKinCai{id: 5, priority: 50, active: true});
    heap.push(JanKinCai{id: 6, priority: 50, active: true});
    assert_eq!(heap.peek(), Some(&JanKinCai{id: 3, priority: 110, active: true}));
    assert_eq!(heap.len(), 6);

    // 更新数据，由于没有iter_mut函数，需要先删除，然后重新push

    // 删除数据
    heap.retain(|v| v.id != 6);
    assert_eq!(heap.len(), 5);

    // 注意事项：遍历顺序和pop顺序是不一样的, 遍历顺序是乱序
    // for value in &heap {
    //     println!("{value:?}");
    // }
    // JanKinCai { id: 3, priority: 110, active: true }
    // JanKinCai { id: 2, priority: 90, active: true }
    // JanKinCai { id: 1, priority: 100, active: true }
    // JanKinCai { id: 4, priority: 50, active: true }
    // JanKinCai { id: 5, priority: 50, active: true }

    // 排序遍历顺序, clone影响性能
    for value in heap.clone().into_iter_sorted() {
        println!("iter value, {value:?}");
    }
    // iter value, JanKinCai { id: 3, priority: 110, active: true }
    // iter value, JanKinCai { id: 1, priority: 100, active: true }
    // iter value, JanKinCai { id: 2, priority: 90, active: true }
    // iter value, JanKinCai { id: 4, priority: 50, active: true }
    // iter value, JanKinCai { id: 5, priority: 50, active: true }

    println!("{:?}", heap.pop());
    println!("{:?}", heap.pop());
    println!("{:?}", heap.pop());
    println!("{:?}", heap.pop());
    println!("{:?}", heap.pop());
    // Some(JanKinCai { id: 3, priority: 110, active: true })
    // Some(JanKinCai { id: 1, priority: 100, active: true })
    // Some(JanKinCai { id: 2, priority: 90, active: true })
    // Some(JanKinCai { id: 4, priority: 50, active: true })
    // Some(JanKinCai { id: 5, priority: 50, active: true })


    // 其他测试
    test_binary_heap_refcell_update();
    test_binary_heap_cell_update();
}
