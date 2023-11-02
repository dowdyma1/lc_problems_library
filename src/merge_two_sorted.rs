use crate::listnode::ListNode;

pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    unimplemented!()
}

mod tests{
    #[cfg(test)]
    fn test1(){
        use crate::{tests_helper::create_nodes_from_array, merge_two_lists};

        let list1 = create_nodes_from_array(&[1,2,4]);
        let list2 = create_nodes_from_array(&[1,3,4]);
        let expected_list = create_nodes_from_array(&[1,1,2,3,4,4]);
        let asserted_list = merge_two_lists(list1, list2);
        assert_eq!(expected_list, asserted_list);
    }
}