/**
* 单向链表反转
*/
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}


impl ListNode {
    fn new (val: i32) -> Self{
        ListNode {
            val,
            next: None,
        }
    }
}


fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut prev = None;
    let mut curr = head;

    while let Some(mut node) = curr {
        // 将当前节点的下一个不再指向以前的 next，而是 pre 节点
        let next = node.next.take();
        node.next = prev;

        // 移动 pre 指针到当前 node 指针为止
        prev = Some(node);

        // 将当前指针 head 指向为 node 的 next 指针
        // curr = node.next.take(); 这里报错
        curr = next; // 为什么这里就可以
    }

    prev
}

/**
将两个升序链表合并为一个新的 升序 链表并返回。新链表是通过拼接给定的两个链表的所有节点组成的
 */
pub fn merge_two_lists(mut list1: Option<Box<ListNode>>, mut list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    // 创建新节点
    let mut dummy = Box::new(ListNode::new(0));
    let mut current = &mut dummy;

    while list1.is_some() && list2.is_some() {
        /**
        Rust 中，Option<T> 提供 .as_ref() 方法将其变为 Option<&T>：
        list1: Option<Box<ListNode>> 是拥有所有权的链表。
        list1.as_ref() → Option<&Box<ListNode>>，允许我们只读访问链表头节点的值，不拿走它的所有权。
        .as_ref()	Option<&T>	否	否	只读访问内部值
        .as_mut()	Option<&mut T>	是	否	可变访问，修改内部值
        */
        let l1_val = list1.as_ref().unwrap().val;
        let l2_val = list2.as_ref().unwrap().val;
        if l1_val < l2_val {
            let next = list1.as_mut().unwrap().next.take();
            current.next = list1;
            list1 = next;
        } else {
            let next = list2.as_mut().unwrap().next.take();
            current.next = list2;
            list2 = next;
        }

        current = current.next.as_mut().unwrap();
    }

    // 连接剩余部分
    current.next = if list1.is_some() {
        list1
    } else {
        list2
    };

    dummy.next
}


/**
给你一个链表的头节点 head ，判断链表中是否有环
 */
pub fn has_cycle(mut head: Option<&Box<ListNode>>) -> bool {
    if head.is_none() || head.as_ref().unwrap().next.is_none() {
        return false;
    }

    let mut slow = head;
    let mut fast = head;

    // (Some(s), Some(f)) = (slow, fast)
    // 这是元组解构匹配，表示：slow 必须是 Some(s)，即非空;fast 必须是 Some(f)，即非空
    while let (Some(s), Some(f)) = (slow, fast) {
        if let Some(fast_next) = &f.next {

        }
    }
}
