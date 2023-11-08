use crate::listnode::ListNode;

pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut cur_node1 = list1;
    let mut cur_node2 = list2;

    let mut pre_head = Box::new(ListNode::new(0));
    let mut cur_merged = &mut pre_head;

    loop {
        let val_1 = get_value(&cur_node1);
        let val_2 = get_value(&cur_node2);

        if val_1 == i32::MAX && val_2 == i32::MAX {
            return pre_head.next
        }
        else if val_1 < val_2 {
            cur_merged.next = Some(Box::new(ListNode::new(val_1)));
            cur_merged = cur_merged.next.as_mut().unwrap();
            cur_node1 = cur_node1.unwrap().next;
        }
        else {
            cur_merged.next = Some(Box::new(ListNode::new(val_2)));
            cur_merged = cur_merged.next.as_mut().unwrap();
            cur_node2 = cur_node2.unwrap().next;
        }
    }
}

fn get_value(node: &Option<Box<ListNode>>) -> i32 {
    match node {
        Some(val) => val.val,
        None => i32::MAX
    }
}

#[cfg(test)]
mod tests{
    use crate::{tests_helper::create_nodes_from_array, merge_two_lists};

    #[test]
    fn test1(){
        let list1 = create_nodes_from_array(&[1,2,4]);
        let list2 = create_nodes_from_array(&[1,3,4]);
        let expected_list = create_nodes_from_array(&[1,1,2,3,4,4]);
        let asserted_list = merge_two_lists(list1, list2);
        assert_eq!(expected_list, asserted_list);
    }

    #[test]
    fn test2(){
        let list1 = create_nodes_from_array(&[]);
        let list2 = create_nodes_from_array(&[]);
        let expected_list = create_nodes_from_array(&[]);
        let asserted_list = merge_two_lists(list1, list2);
        assert_eq!(expected_list, asserted_list);
    }

    #[test]
    fn test3(){
        let list1 = create_nodes_from_array(&[]);
        let list2 = create_nodes_from_array(&[0]);
        let expected_list = create_nodes_from_array(&[0]);
        let asserted_list = merge_two_lists(list1, list2);
        assert_eq!(expected_list, asserted_list);
    }
}