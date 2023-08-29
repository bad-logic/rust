//--------------------------------------------
//          Fetching Top Products
//              - Description
//                  - we are given link lists corresponding to top ranked products in different
//                      countries. we need to combine all these link lists into one consolidated
//                      link list containing the ranks in an descending order
//
//              - Tools
//                  - linkList, iterators
//-----------------------------------------------

type Pointer<T> = Option<Box<Node<T>>>;
#[derive(Debug)]
struct Node<T>{
    element: T,
    next: Pointer<T>
}

#[derive(Debug)]
struct LinkList<T: std::fmt::Debug>{
    head: Pointer<T>
}

impl<T: std::fmt::Debug> LinkList<T>{
    fn create_empty_list() -> LinkList<T>{
        LinkList{head:None}
    }

    fn add(&mut self,x:T){
        let prev_head = self.head.take();
        let new_head = Box::new(Node{
            element:x,
            next: prev_head
        });
        self.head = Some(new_head);
    }

    fn remove(&mut self)-> Option<T>{
        let prev_head = self.head.take();
        match prev_head {
            Some(old_head) => {
                self.head = old_head.next;
                Some(old_head.element)
            },
            None => None
        }

    }

    fn peek(&self) -> Option<&T>{
        match &self.head{
            Some(head) => Some(&head.element),
            None => None
        }
    }

    fn printing(&self){
        let mut list_traversal = &self.head;
        while true{
            match list_traversal{
                Some(head) => {
                    print!("{:?} ",head.element);
                    list_traversal = &list_traversal.as_ref().unwrap().next;
                    // list_traversal = &head.next;
                },
                None => break
            }
        }
        println!();
    }

    fn reverse(&mut self){
        if self.head.is_none() || self.head.as_ref().unwrap().next.is_none(){
            return;
        }

        let mut prev = None;
        let mut current = self.head.take();

        while current.is_some(){
            let next = current.as_mut().unwrap().next.take();
            current.as_mut().unwrap().next = prev.take();
            prev = current.take();
            current = next;
        }
        self.head = prev.take();
    }
}

fn sort_lists(vec_list: &mut Vec<LinkList<i32>>)-> LinkList<i32>{
    let mut sorted_list = LinkList::create_empty_list();

    while vec_list.len() > 0 {

        let values = vec_list
        .into_iter()
        .map(|x| x.head.as_ref().unwrap().element)
        .collect::<Vec<i32>>();
        
        // find the minimum values among all heads
        let min_value = *values.iter().min().unwrap();
        // find the index of the minimum value
        let min_index = values.iter().position(|x| *x == min_value).unwrap();
        // remove head of the link list containing that min value
        vec_list[min_index].remove();
        // add that minimum value to the sorted link list
        sorted_list.add(min_value);

        // check if the link list of min_index is empty if so remove it from list
        if vec_list[min_index].head.is_none() {
            vec_list.remove(min_index);
        }
    }
    sorted_list
}

fn main(){
    
    let mut list1 = LinkList::create_empty_list(); // descending order
    list1.add(45);
    list1.add(40);
    list1.add(35);
    list1.add(23);
    list1.add(11);

    let mut list2 = LinkList::create_empty_list(); // descending order
    list2.add(60);
    list2.add(44);

    let mut list3 = LinkList::create_empty_list(); // descending order
    list3.add(85);
    list3.add(20);
    list3.add(15);

    let mut list = vec![list1,list2,list3];

    let mut sorted_list = sort_lists(&mut list); // descending order
    sorted_list.printing();

    println!("reversing the linked list ...");
    sorted_list.reverse();
    sorted_list.printing();



}