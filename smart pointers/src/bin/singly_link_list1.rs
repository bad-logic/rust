// --------------------------------------------------
//  Singly Link List
// --------------------------------------------------

type Pointer<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
struct Node<T: std::fmt::Debug + std::marker::Copy>{
    element: T,
    next: Pointer<T>
}

#[derive(Debug)]
struct LinkList<T: std::fmt::Debug + std::marker::Copy>{
    head: Pointer<T>
}

impl<T: std::fmt::Debug + std::marker::Copy> LinkList<T>{
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

    fn peek(&self) -> Option<T>{
        match &self.head{
            Some(head) => Some(head.element),
            None => None
        }
    }

    fn printing(&self){
        let mut list_traversal = &self.head;
        while true{
            match list_traversal{
                Some(head) => {
                    println!("{:?}",head.element);
                    list_traversal = &head.next;
                },
                None => break
            }
        }
    }
}


fn main(){
    let list1 = Node{element:1,next:None};
    let list2 = Node{
        element:1, 
        next: Some(Box::new(
            Node{
                element:2,
                next: Some(Box::new(
                    Node{
                        element:3,
                        next: Some(Box::new(
                            Node{
                                element:4,
                                next:None
                            }
                        ))
                    }
                ))
            }
        ))
    };

    // let list3 = LinkList::create_empty_list();

    let list4 = LinkList{
        head: Some(Box::new(Node{
            element: 1,
            next: Some(Box::new(Node{
                element:2,
                next: Some(Box::new(Node{
                    element:3,
                    next: Some(Box::new(Node{
                        element:4,
                        next:None
                    }))
                }))
            }))
        }))
    };

    println!("{:?} \n{:?} \n{:?}",list1,list2,list4);
    let mut list5 = LinkList::create_empty_list();
    list5.add(1);
    list5.add(2);
    list5.add(3);
    list5.add(4);

    println!("{:?}",list5);
    list5.remove();
    println!("{:?}",list5);
    println!("peel >> {:?}",list5.peek());
    list5.printing();


}