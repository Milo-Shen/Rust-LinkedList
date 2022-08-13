// 所以我们应该怎么避免多余的 junk，保持内存分配的一致性，还能保持 null 指针优化呢？
// 枚举可以让我们声明一个类型用于表达多个不同的值，而结构体可以声明一个类型同时包含多个值，只要将这两个类型结合在一起，
// 就能实现之前的目标: 枚举类型用于表示 List，结构体类型用于表示 Node.
pub struct List {
    head: Link,
}

enum Link {
    Empty,
    More(Box<Node>),
}

struct Node {
    elem: i32,
    next: Link,
}

impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }
}
