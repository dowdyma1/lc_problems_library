use crate::listnode::ListNode;

pub fn create_nodes_from_array(arr: &[i32]) -> Option<Box<ListNode>> {
    if arr.is_empty() {
        return None;
    }
    
    let mut head = Some(Box::new(ListNode::new(arr[0])));
    let mut cur_node = head.as_mut().unwrap();

    for &val in &arr[1..] {
        let next_node = Box::new(ListNode::new(val));
        cur_node.next = Some(next_node);
        cur_node = cur_node.next.as_mut().unwrap();
    }

    head
}