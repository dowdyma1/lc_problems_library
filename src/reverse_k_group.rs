use crate::listnode::ListNode;

pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    let k = k as usize;
    let mut stack: Vec<Box<ListNode>> = Vec::with_capacity(k);
    let mut pre_head: Option<Box<ListNode>> = Some(Box::new(ListNode::new(0)));
    let mut change_node = &mut pre_head;

    // cur_node is head
    let mut cur_node: Option<Box<ListNode>> = head;


    'outer: loop {
        // push the next k nodes onto the stack. If there aren't k nodes, return the new_head
        for i in 0..k {
            // take the next node from cur_node so cur_node.next => None
            let next_node = cur_node.as_mut().unwrap().next.take();

            stack.push(cur_node.unwrap());

            cur_node = next_node;
            if cur_node.is_none() && i < k-1 {
                break 'outer;
            }
        }

        while let Some(node) = stack.pop() {
            change_node.as_mut().unwrap().next = Some(node);
            change_node = &mut change_node.as_mut().unwrap().next;
        }
    }

    for node in stack {
        change_node.as_mut().unwrap().next = Some(node);
        change_node = &mut change_node.as_mut().unwrap().next;
    }


    pre_head.unwrap().next
}

#[cfg(test)]
mod tests{
    use crate::reverse_k_group;
    use crate::tests_helper::create_nodes_from_array;

    #[test]
    fn test1(){
        let list = create_nodes_from_array(&[1,2,3,4,5,6,7,8,9,10]);
        let expected_list = create_nodes_from_array(&[3,2,1,6,5,4,9,8,7,10]);
        let asserted_list = reverse_k_group(list, 3);
        assert_eq!(expected_list, asserted_list);
    }
}