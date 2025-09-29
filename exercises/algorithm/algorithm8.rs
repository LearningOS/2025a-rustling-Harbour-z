/*
	queue
	This question requires you to use queues to implement the functionality of the stac
*/

#[derive(Debug)]
pub struct Queue<T> {
    elements: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }

    pub fn enqueue(&mut self, value: T) {
        self.elements.push(value)
    }

    pub fn dequeue(&mut self) -> Result<T, &str> {
        if !self.elements.is_empty() {
            Ok(self.elements.remove(0usize))
        } else {
            Err("Queue is empty")
        }
    }

    pub fn peek(&self) -> Result<&T, &str> {
        match self.elements.first() {
            Some(value) => Ok(value),
            None => Err("Queue is empty"),
        }
    }

    pub fn size(&self) -> usize {
        self.elements.len()
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

impl<T> Default for Queue<T> {
    fn default() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }
}

pub struct MyStack<T>
{
	//TODO
	q1:Queue<T>,
	q2:Queue<T>
}
impl<T> MyStack<T> {
    pub fn new() -> Self {
        Self {
			//TODO
			q1:Queue::<T>::new(),
			q2:Queue::<T>::new()
        }
    }
    pub fn push(&mut self, elem: T) {
        //TODO
        self.q1.enqueue(elem);
    }
    pub fn pop(&mut self) -> Result<T, &str> {
        if self.q1.is_empty() {
        return Err("Stack is empty");
    }
    // Step 1: Move all but the last element from q1 to q2
    while self.q1.size() > 1 {
        let value = self.q1.dequeue().unwrap(); // 安全：size > 1
        self.q2.enqueue(value);
    }
    // Step 2: Get the last element (the one we want to return)
    // ⚠️ 关键：我们立刻解包成 owned value，结束对 q1 的借用！
    let top_element = match self.q1.dequeue() {
        Ok(val) => val,
        Err(_) => unreachable!(), // 已经确保 size >= 1
    };
    // 🔥 此刻：`dequeue()` 调用结束，对 `q1` 的可变借用已经释放！
    // Step 3: Move all elements back from q2 to q1
    while let Ok(val) = self.q2.dequeue() {
        self.q1.enqueue(val); // ✅ 现在可以安全借用 `&mut self.q1`
    }
    // Return the popped element
    Ok(top_element) // ✅ 直接返回拥有所有权的值
		
    }
    pub fn is_empty(&self) -> bool {
		//TODO

        self.q1.is_empty()
    }
}

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn test_queue(){
		let mut s = MyStack::<i32>::new();
		assert_eq!(s.pop(), Err("Stack is empty"));
        s.push(1);
        s.push(2);
        s.push(3);
        assert_eq!(s.pop(), Ok(3));
        assert_eq!(s.pop(), Ok(2));
        s.push(4);
        s.push(5);
        assert_eq!(s.is_empty(), false);
        assert_eq!(s.pop(), Ok(5));
        assert_eq!(s.pop(), Ok(4));
        assert_eq!(s.pop(), Ok(1));
        assert_eq!(s.pop(), Err("Stack is empty"));
        assert_eq!(s.is_empty(), true);
	}
}