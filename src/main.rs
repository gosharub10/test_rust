use std::{sync::{Arc, Mutex}, thread, time::Duration};

fn main() {
    let list1 = List::new().prepend(22).prepend(232).prepend(1).prepend(2);
    let list2 = List::new().prepend(-3).prepend(2).prepend(1).prepend(3);

    let data1 = Arc::new(Mutex::new(list1.clone()));
    let data2 = Arc::new(Mutex::new(list2.clone()));

    let thread1 = thread::spawn({
        let data_copy1 = Arc::clone(&data1);
        let data_copy2 = Arc::clone(&data2);
        move ||{
            let guard = data_copy1.lock().unwrap();
            let mut iter = guard.iter();
            for _ in 1..=guard.length{
                thread::sleep(Duration::from_millis(250));
                println!("Thread1: {}", iter.next().unwrap())
            }

            check_mutex_and_print(data_copy2);
        }
    });

    let thread2 = thread::spawn({
        let data_copy1 = Arc::clone(&data1);
        let data_copy2 = Arc::clone(&data1);

        move ||{
            let guard = data_copy2.lock().unwrap();
            let mut iter = guard.iter();
            for _ in 1..=guard.length{
                thread::sleep(Duration::from_millis(250));
                println!("Thread2: {}", iter.next().unwrap())
            }

            check_mutex_and_print(data_copy1);
        }
    });    

    thread1.join().unwrap();
    thread2.join().unwrap();
}

fn check_mutex_and_print(data_clone: Arc<Mutex<List<i32>>>){
    let check = data_clone.try_lock();
    match check {
        Ok(list) => {
            let mut iter = list.iter();
            for _ in 1..=list.length{
                thread::sleep(Duration::from_millis(250));
                println!(" {}", iter.next().unwrap());
            }
        },
        Err(_) => println!("Faild to take a list, continuing")
    }
}

#[derive(Clone, Debug)]
struct List<T> {
    head: Link<T>,
    length: u32
}

type Link<T> = Option<Arc<Node<T>>>;

#[derive(Clone, Debug)]
struct Node<T> {
    value: T,
    next: Link<T>,
}

impl<T> List<T> {
    #![allow(unused)]
    fn new() -> Self {
        List { head: None, length:0 }
    }

    pub fn prepend(&self, value: T) -> List<T> {
        List {
            head: Some(Arc::new(Node {
                value: value,
                next: self.head.clone(),
            })),
            length: self.length+1
        }
    }

    fn pop(&self) -> List<T> {
        List {
            head: self.head.as_ref().and_then(|node| node.next.clone()),
            length: self.length - 1
        }
    }

    fn peek_to_head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.value)
    }

    fn iter(&self) -> Iter<T> {
        Iter {
            next: self.head.as_deref(),
        }
    }
}


impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut head = self.head.take();
        while let Some(node) = head {
            if let Ok(mut node) = Arc::try_unwrap(node) {
                head = node.next.take();
            } else {
                break;
            }
        }
    }
}

struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.value
        })
    }
}
