extern crate movies_lib;
extern crate rand;
// 用于引入 movies_lib::movies::play。如果没有这句，那么使用 play() 函数就要明确指定模块和模块下的文件。
extern crate w1_lib;

use std::collections;
use std::fmt::Display;
use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::io::Read;
use std::io::Write;
use std::ops::Deref;
// 导入时间模块
use std::sync::mpsc;
use std::thread;
// 导入线程模块
use std::time::Duration;

// 用于引入我们刚刚创建的模块 movies_lib。这个是必须的
use movies_lib::moviess::play;
use rand::random;
// 用于引入我们刚刚创建的模块 movies_lib。这个是必须的
use w1_lib::sss1::play2;

// as 关键字为标识符添加别名：
use crate::nation::government::govern;

fn main() {
    println!("Hello, world!");
    game()
}

//  多线程
fn thread() {
    //创建一个新线程
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    // 主线程要执行的代码
    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

//     线程句柄 join()
//     join() 方法用于把子线程加入主线程等待队列。
    handle.join().unwrap();

//     move 强制所有权迁移
//     在子线程中尝试使用当前函数的资源，这一定是错误的！因为所有权机制禁止这种危险情况的产生，它将破坏所有权机制销毁资源的一定性。我们可以使用闭包的 move 关键字来处理：
    let s = "hello";

//     消息传递
// Rust 中一个实现消息传递并发的主要工具是通道（channel），通道有两部分组成，一个发送者（transmitter）和一个接收者（receiver）。
//
// std::sync::mpsc 包含了消息传递的方法：

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}

// 智能指针 box
fn zn_box() {
    let var_i32 = 5;           // 默认数组保存在 栈 上
    let b = Box::new(var_i32); // 使用 Box 后数据会存储在堆上
    println!("b = {}", b);
//     当我们使用 Box::new() 把一个数据存储在堆上之后，为了访问存储的具体数据，我们必须 解引用。
// 解引用 需要使用操作符 星号 *，因此 星号 * 也称之为 解引用操作符。

    println!("{}", 5 == *b); // 为了访问 b 存储的具体数据，需要解引用


//     deref() 方法从某些方面说用于借用 self 对象并返回一个指向内部数据的指针。

    let x = 5;
    let y = MyBox::new(x);  // 调用静态方法 new() 返回创建一个结构体实例

    println!("5==x is {}", 5 == x);
    println!("5==*y is {}", 5 == *y);  // 解引用 y
    println!("x==*y is {}", x == *y);  // 解引用 y

    struct MyBox<T>(T);

    impl<T> MyBox<T> {
        // 范型方法
        fn new(x: T) -> MyBox<T> {
            MyBox(x)
        }
    }

    impl<T> Deref for MyBox<T> {
        type Target = T;
        fn deref(&self) -> &T {
            &self.0 // 返回数据
        }
    }

    // 删除特质 Drop Trait 类似析构函数
    impl<T> Drop for MyBox<T> {
        fn drop(&mut self) {
            println!("dropping MyBox object from memory ");
        }
    }
}

//Closure 闭包
fn closure() {
    let closure_function = |parameter| {
        // 闭包的具体逻辑
        println!("闭包");
        parameter + 2;
        return 1;
    };

    let is_even = |x| {
        x % 2 == 0
    };
    let no = 13;
    println!("{} is even ? {}", no, is_even(no));
    println!("{} is even ? {}", no, closure_function(no));
}

// 迭代器
fn iterator() {
    //创建一个数组
    let a = [10, 20, 30];

    let mut iter = a.iter(); // 从一个数组中返回迭代器
    println!("{:?}", iter);

    //使用 next() 方法返回迭代器中的下一个元素
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());

    /*
    方法	描述
iter()	返回一个只读可重入迭代器，迭代器元素的类型为 &T
into_iter()	返回一个只读不可重入迭代器，迭代器元素的类型为 T
iter_mut()	返回一个可修改可重入迭代器，迭代器元素的类型为 &mut T
    */
}

use rand::Rng;

// cargo 管理 猜数字游戏 
fn game() {
    println!("Welcome to no guessing game");
    
    let mut rng = rand::thread_rng();
    let correct: u8 = random();
    // 指定范围  u8 256
    let correct : u8 = rng.gen_range(60, 101);    
    println!("u8类型 最大:256 correct value is {}", correct);
    loop {
        let guess = get_guess();
        if handle_guess(guess, correct) {
            break;
        }
    }

    fn get_guess() -> u8 {
        loop {
            println!("Input guess");
            let mut guess = String::new();
            io::stdin().read_line(&mut guess)
                .expect("could not read from stdin");
            match guess.trim().parse::<u8>() { // 需要去除输入首尾的空白
                Ok(v) => return v,
                Err(e) => println!("could not understand input {}", e)
            }
        }
    }
    fn handle_guess(guess: u8, correct: u8) -> bool {
        if guess < correct {
            println!("小了");
            false
        } else if guess > correct {
            println!("大了");
            false
        } else {
            println!("那就是对了 ..");
            true
        }
    }
}

fn file_rw() {
    /*
    模块	方法
方法签名	说明
std::fs::File	open()
pub fn open(path: P) -> Result	静态方法，以 只读 模式打开文件
std::fs::File	create()
pub fn create(path: P) -> Result	静态方法，以 可写 模式打开文件。
如果文件存在则清空旧内容
如果文件不存在则新建
std::fs::remove_file	remove_file()
pub fn remove_file(path: P) -> Result<()>	从文件系统中删除某个文件
std::fs::OpenOptions	append()
pub fn append(&mut self, append: bool) -> &mut OpenOptions	设置文件模式为 追加
std::io::Writes	write_all()
fn write_all(&mut self, buf: &[u8]) -> Result<()>	将 buf 中的所有内容写入输出流
std::io::Read	read_to_string()
fn read_to_string(&mut self, buf: &mut String) -> Result	读取所有内容转换为字符串后追加到 buf 中
    */

    let file = std::fs::File::open("data.txt").unwrap();
    println!("文件打开成功：{:?}", file);

    let file = std::fs::File::create("data.txt").expect("create failed");
    println!("文件创建成功:{:?}", file);


    let mut file = std::fs::File::create("data.txt").expect("create failed");
    file.write_all("简单教程".as_bytes()).expect("write failed");
    file.write_all("\n简单编程".as_bytes()).expect("write failed");
    println!("data written to file");


    // 从一个文件中读取所有剩余的内容并转换为字符串。
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);

//     append() 方法修改文件的打开模式为 追加。

    let mut file = OpenOptions::new().append(true).open("data.txt").expect(
        "cannot open file");
    file.write_all("www.twle.cn".as_bytes()).expect("write failed");
    file.write_all("\n简单教程".as_bytes()).expect("write failed");
    file.write_all("\n简单编程".as_bytes()).expect("write failed");
    println!("数据追加成功");
}

fn io() {
    let mut line = String::new();
    println!("请输入你的名字:");
    let b1 = std::io::stdin().read_line(&mut line).unwrap();
    println!("你好 , {}", line);
    println!("读取的字节数为：{}", b1);

    // 使用标准库输出信息
    let b1 = std::io::stdout().write("简单教程 ".as_bytes()).unwrap();
    let b2 = std::io::stdout().write(String::from("www.twle.cn").as_bytes()).unwrap();
    std::io::stdout().write(format!("\n写入的字节数为：{}\n", (b1 + b2)).as_bytes()).unwrap();


    // 命令参数

    let cmd_line = std::env::args();
    println!("总共有 {} 个命令行参数", cmd_line.len()); // 传递的参数个数
    for arg in cmd_line {
        println!("[{}]", arg); // 迭代输出命令行传递的参数
    }
}

// 泛型与特性
fn flood_type() {
    // 泛型结构体泛型结构体的定义语法如下
    struct StructName<T> {
        field: T,
    }

    struct Data<T> {
        // value2: StructName<T>,
        value: T,
    }

    // 泛型为 i32
    let t: Data<i32> = Data { value: 350 };
    println!("value is :{} ", t.value);
    // 泛型为 String
    let t2: Data<String> = Data { value: "Tom".to_string() };
    println!("value is :{} ", t2.value);

    //     特质 Traits 特质 其实就是一组方法原型。它的语法格式如下
    // Rust 中的 特质 相当于其它语言的 接口 （ interface ）。
    trait some_trait {
        // 没有任何实现的虚方法
        fn method1(&self);

        // 有具体实现的普通方法
        fn method2(&self) {
            //方法的具体代码
        }
    }
//     实现特质 impl for
// impl for 语句的语法格式如下
// 特质的方法是结构体的成员方法，因此第一个参数是 &self。
    /*    impl some_trait for structure_name {
            // 实现 method1() 的具体代码
            fn method1(&self) {}
        }*/
//     范例: impl for 的简单使用
// 下面的范例，我们首先定义了一个结构体 Book 和一个特质 Printable。
// 然后我们使用 impl for 语句为 Book 实现特质 Printable。

    //创建结构体 Book 的实例
    let b1 = Book {
        id: 1001,
        name: String::from("Rust in Action"),
    };
    b1.print();

    //声明结构体
    struct Book {
        name: String,
        id: i32,
    }
    // 声明特质
    trait Printable {
        fn print(&self);
    }
    // 实现特质
    impl Printable for Book {
        fn print(&self) {
            println!("Printing book with id:{} and name {}", self.id, self.name)
        }
    }

//     泛型函数

    fn print_pro<T: Display>(t: T) {
        println!("Inside print_pro generic function:");
        println!("{}", t);
    }
    print_pro(10 as u8);
    print_pro(20 as u16);
    print_pro("Hello TutorialsPoint");
}

// 集合 容器
fn collections() {

    // 向量
    let mut v = Vec::new();
    // 宏来简化向量的创建。
    // let vector_name = vec![val1,val2,val3];
    /*
    new()	pub fn new()->Vec	创建一个空的向量的实例
    push()	pub fn push(&mut self, value: T)	将某个值 T 添加到向量的末尾
    remove()	pub fn remove(&mut self, index: usize) -> T	删除并返回指定的下标元素。
    contains()	pub fn contains(&self, x: &T) -> bool	判断向量是否包含某个值
    len()	pub fn len(&self) -> usize	返回向量中的元素个数
    */

    v.push(20);
    v.push(30);

    v.push(40);

    //& 传引用
    for i in &v {
        println!("{}", i);
    }

    println!("size of vector is :{}", v.len());
    // println!("{:?}",v);


    // 哈希表 hashMap
    let mut stateCodes = collections::HashMap::new();
    /*
    方法	方法签名	说明
    insert()	pub fn insert(&mut self, k: K, v: V) -> Option	插入/更新一个键值对到哈希表中，如果数据已经存在则返回旧值，如果不存在则返回 None
    len()	pub fn len(&self) -> usize	返回哈希表中键值对的个数
    get()	pub fn get<Q: ?Sized>(&lself, k: &Q) -> Option<&V>	根据键从哈希表中获取相应的值
    iter()	pub fn iter(&self) -> Iter<K, V>	返回哈希表键值对的无序迭代器，迭代器元素类型为 (&'a K, &'a V)
    contains_key	pub fn contains_key<Q: ?Sized>(&self, k: &Q) -> bool	如果哈希表中存在指定的键则返回 true 否则返回 false
    remove()	pub fn remove_entry<Q: ?Sized>(&mut self, k: &Q) -> Option<(K, V)>	从哈希表中删除并返回指定的键值对
    */

    stateCodes.insert("name", "简单教程");
    stateCodes.insert("site", "https://www.twle.cn");

    // 获取值
    match stateCodes.get(&"name") {
        Some(v) => {
            println!("Value for key name is {}", v);
        }
        None => {
            println!("nothing found");
        }
    }
    // 迭代哈希表 iter()
    // iter() 方法会返回哈希表中 键值对的引用 组成的无序迭代器。

    for (key, val) in stateCodes.iter() {
        println!("key: {} val: {}", key, val);
    }


    if stateCodes.contains_key(&"name") {
        println!("found key");
    }

//     哈希集合 HashSet
//     是没有重复值的相同数据类型的值的集合。

    let mut languages = collections::HashSet::new();
    /*
    方法	方法原型	描述
insert()	pub fn insert(&mut self, value: T) -> bool	插入一个值到集合中
如果集合已经存在值则插入失败
len()	pub fn len(&self) -> usize	返回集合中的元素个数
get()	pub fn get<Q:?Sized>(&self, value: &Q) -> Option<&T>	根据指定的值获取集合中相应值的一个引用
iter()	pub fn iter(&self) -> Iter	返回集合中所有元素组成的无序迭代器
迭代器元素的类型为 &'a T
contains_key	pub fn contains<Q: ?Sized>(&self, value: &Q) -> bool	判断集合是否包含指定的值
remove()	pub fn remove<Q: ?Sized>(&mut self, value: &Q) -> bool	从结合中删除指定的值
    */
    languages.insert("Python");
    languages.insert("Rust");
    languages.insert("Ruby");
    languages.insert("PHP");

    languages.insert("Rust"); // 插入失败但不会引发异常

//     迭代器元素的顺序是无序的，毫无规则的。而且每次调用 iter() 返回的元素顺序都可能不一样。
}

// 错误处理
fn error() {
    // 不可恢复错误
    // panic!("error occured");
    println!("Hello, Rust");

    /*
    可恢复的错误
此概念十分类似于 Java 编程语言中的异常。实际上在 C 语言中我们就常常将函数返回值设置成整数来表达函数遇到的错误，在 Rust 中通过 Result<T, E> 枚举类作返回值来进行异常表达：
*/
    enum Result<T, E> {
        Ok(T),
        Err(E),
    }
//     在 Rust 标准库中可能产生异常的函数的返回值都是 Result 类型的。例如：当我们尝试打开一个文件时：
    let f = File::open("file/hello.txt");
    match f {
        Ok(file) => {
            println!("File opened successfully.");
        }
        Err(err) => {
            println!("Failed to open the file.");
        }
    }

//     如果想使一个可恢复错误按不可恢复错误处理，Result 类提供了两个办法：unwrap() 和 expect(message: &str) ：
//     let f1 = File::open("hello.txt").unwrap();
//     let f2 = File::open("hello.txt").expect("Failed to open.");

    //     可恢复的错误的传递
    fn fs(i: i32) -> std::result::Result<i32, bool> {
        if i >= 0 { Ok(i) } else { Err(false) }
    }
    let r = fs(10000);
    if let Ok(v) = r {
        println!("Ok: f(-1) = {}", v);
    } else {
        println!("Err");
    }
    let r = g(10000);

    if let Ok(v) = r {
        println!("Ok: g(10000) = {}", v);
    } else {
        println!("Err");
    }
    // 函数 g 传递了函数 f 可能出现的错误（这里的 g 只是一个简单的例子，实际上传递错误的函数一般还包含很多其它操作）。
    // 这样写有些冗长，Rust 中可以在 Result 对象后添加 ? 操作符将同类的 Err 直接传递出去：

    // ? 符的实际作用是将 Result 类非异常的值直接取出，如果有异常就将异常 Result 返回出去。
    // 所以，? 符仅用于返回值类型为 Result<T, E> 的函数，其中 E 类型必须和 ? 所处理的 Result 的 E 类型一致。
    fn g1(i: i32) -> std::result::Result<i32, bool> {
        let t = fs(i)?;
        Ok(t) // 因为确定 t 不是 Err, t 在这里已经是 i32 类型
    }

    fn g(i: i32) -> std::result::Result<i32, bool> {
        let t = fs(i);
        return match t {
            Ok(i) => Ok(i),
            Err(b) => Err(b)
        };
    }

    fn read_text_from_file(path: &str) -> std::result::Result<String, io::Error> {
        let mut f = File::open(path)?;
        let mut s = String::new();
        f.read_to_string(&mut s)?;
        Ok(s)
    }

    let str_file = read_text_from_file("hello.txt");
    match str_file {
        Ok(s) => println!("{}", s),
        Err(e) => {
            match e.kind() {
                io::ErrorKind::NotFound => {
                    println!("No such file");
                }
                _ => {
                    println!("Cannot read the file");
                }
            }
        }
    }
}

// 模块
// main.rs
mod second_module;

// use 关键字
// use 关键字能够将模块标识符引入当前作用域：
mod nation {
    pub mod government {
        pub fn govern() {
            println!("govern")
        }
    }
}

// 用于引入 movies_lib::movies::play。如果没有这句，那么使用 play() 函数就要明确指定模块和模块下的文件。

// 模块
fn module() {
    play("简单教程".to_string());
    play2("简单教程".to_string());
    w1_lib::mm::nihao();

    govern();

    mod nation {
        mod government {
            fn govern() {}
        }

        mod congress {
            fn legislate() {}
        }

        mod court {
            fn judicial() {}
        }
    }
    /*
    访问权限
Rust 中有两种简单的访问权：公共（public）和私有（private）。

默认情况下，如果不加修饰符，模块中的成员访问权将是私有的。

如果想使用公共权限，需要使用 pub 关键字。

对于私有的模块，只有在与其平级的位置或下级的位置才能访问，不能从其外部访问。
    */

    println!("This is the main module.");
    println!("{}", second_module::message());
    second_module::movies::play("sss".to_string())
}

// 枚举
fn enum_my() {
    enum Book {
        Papery(u64),
        Electronic { url: String },
    }
    let book = Book::Papery(1001);
    //  switch 语法很经典，但在 Rust 中并不支持，
    match book {
        Book::Papery(i) => {
            println!("{}", i);
        }
        Book::Electronic { url } => {
            println!("{}", url);
        }
    }

//     在例外情况下没有任何要做的事 . 例外情况用下划线 _ 表示：
    let t = "abc";
    match t {
        "abc" => println!("Yes"),
        _ => {}
    }

    //Rust 在语言层面彻底不允许空值 null 的存在，但无奈null 可以高效地解决少量的问题，所以 Rust 引入了 Option 枚举类：
    enum Option<T> {
        Some(T),
        None,
    }
    // 如果你想定义一个可以为空值的类，你可以这样：
    let opt = Option::Some("Hello");

    // 空值
    let opt: Option<&str> = Option::None;
    match opt {
        Option::Some(something) => {
            println!("{}", something);
        }
        Option::None => {
            println!("opt is nothing");
        }
    }
}

//结构体
fn struct_my() {
    struct Site {
        domain: String,
        name: String,
        nation: String,
        traffic: u32,
        found: u32,
    }

    let runoob = Site {
        domain: String::from("www.runoob.com"),
        name: String::from("RUNOOB"),
        nation: String::from("China"),
        traffic: 0,
        found: 2013,
    };
    let domain = String::from("www.runoob.com");
    let name = String::from("RUNOOB");
    let runoob = Site {
        domain,  // 等同于 domain : domain,
        name,    // 等同于 name : name,
        nation: String::from("China"),
        traffic: 2013,
        found: 0,
    };
    // 继承 你想要新建一个结构体的实例，其中大部分属性需要被设置成与现存的一个结构体属性一样，仅需更改其中的一两个字段的值，可以使用结构体更新语法：
    let site = Site {
        domain: String::from("www.runoob.com"),
        name: String::from("RUNOOB"),
        ..runoob
    };

    // 元组结构体
    // 有一种更简单的定义和使用结构体的方式：元组结构体。
    struct Color(u8, u8, u8);
    struct Point(f64, f64);

    let black = Color(0, 22, 0);
    let origin = Point(0.0, 0.0);
    // 元组结构体对象的使用方式和元组一样，通过 . 和下标来进行访问：
    println!("black = ({}, {}, {})", black.0, black.1, black.2);
    println!("origin = ({}, {})", origin.0, origin.1);

//     结构体所有权
// 结构体必须掌握字段值所有权，因为结构体失效的时候会释放所有字段。

    //     输出一整个结构体的方法：
    struct Rectangle {
        width: u32,
        height: u32,
    }
    let rect1 = Rectangle { width: 30, height: 50 };
    // 属性较多的话可以使用另一个占位符 {:#?} 。
    // println!("rect1 is {:?}", rect1);

    //     结构体方法
//     结构体方法的第一个参数必须是 &self，不需声明类型，因为 self 不是一种风格而是关键字。
    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }
    }
    println!("rect1's area is {}", rect1.area());
//     多参数的例子

    impl Rectangle {
        fn wider(&self, rect: &Rectangle) -> bool {
            println!("area111111111 {}", self.area());
            self.width > rect.width
        }
    }

    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 40, height: 20 };

    println!("rect1 {}", rect1.wider(&rect2));

    //     结构体关联函数
// 在 impl 块中却没有 &self 参数。
    impl Rectangle {
        fn create(width: u32, height: u32) -> Rectangle {
            return Rectangle { width, height };
        }
    }
    let rect = Rectangle::create(30, 50);
    println!("{}", rect.width);

    // 结构体可以只作为一种象征而无需任何成员：
    // 我们称这种没有身体的结构体为单元结构体（Unit Struct）。
    // 单元结构体
    struct UnitStruct;
}

// 切片
fn slice() {
    let mut s = String::from("broadcast");

    let part1 = &s[0..5];
    let part2 = &s[5..9];
    // 注意：到目前为止，尽量不要在字符串中使用非英文字符，因为编码的问题。具体原因会在"字符串"章节叙述。
    // 被切片引用的字符串禁止更改其值：

    // s.push_str("yes!"); // 错误

    println!("{}={}+{}", s, part1, part2);

//     在 Rust 中有两种常用的字符串类型：str 和 String。str 是 Rust 核心语言类型，就是本章一直在讲的字符串切片（String Slice），常常以引用的形式出现（&str）。
// 凡是用双引号包括的字符串常量整体的类型性质都是 &str：
    let s = "hello";//  &str 类型的变量。
    // 注意：切片结果必须是引用类型，但开发者必须自己明示这一点:
    let slice = &s[0..3];

    // 有一个快速的办法可以将 String 转换成 &str：
    let s1 = String::from("hello");
    let s2 = &s1[..];

    // 除了字符串以外，其他一些线性数据结构也支持切片操作，例如数组：

    let arr = [1, 3, 5, 7, 9];
    let part = &arr[0..3];
    for i in part.iter() {
        println!("{}", i);
    }
}

// 所有权
fn ownership2() {
    let mut s1 = String::from("hello");
    // 垂悬引用（Dangling References）
    // let reference_to_nothing = dangle(s1);
    dangle2(&mut s1);
    println!("{},{}", 2, s1);

    // 编译器检查 不允许逃逸
    fn dangle(s: String) -> String {
        s
    }
    // 引用
    fn dangle2(s: &mut String) {
        s.push_str("22")
    }


    // 引用
    println!("The length of '{}|{}'.", s1, s1.len());
    let mut len = calculate_length(&mut s1);

    println!("The length of '{}|{}' is {}.", s1, s1.len(), len);

    fn calculate_length(s: &mut String) -> usize {
        s.push_str("@");
        s.len()
    }

    // 租借
    let len = calculate_length2(&s1);

    println!("The length of '{}' is {}.", s1, len);

    fn calculate_length2(s: &String) -> usize {
        s.len()
    }
}

// 所有权
fn ownership() {
//     所有权规则
// 所有权有以下三条规则：
    // Rust 中的每个值都有一个变量，称为其所有者。
    // 一次只能有一个所有者。
    // 当所有者不在程序运行范围时，该值将被删除。
// 这三条规则是所有权概念的基础。
// 接下来将介绍与所有权概念有关的概念。
    {
        // 在声明以前，变量 s 无效
        let s = "runoob";
        // 这里是变量 s 的可用范围
    }
// 变量范围已经结束，变量 s 无效


    // 移动
    // 在把 s1 的值赋给 s2 以后 s1 将不可以再被使用
    let s1 = String::from("hello");
    let s2 = s1;
    println!("{}, world!", s2); // 错误！s1 已经失效

    let aw = 2;
    // 克隆
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);

//     涉及函数的所有权机制
    let s = String::from("hello");
    // s 被声明有效

    takes_ownership(s);
    // s 的值被当作参数传入函数
    // 所以可以当作 s 已经被移动，从这里开始已经无效

    let x = 5;
    // x 被声明有效

    makes_copy(x);
    // x 的值被当作参数传入函数
    // 但 x 是基本类型，依然有效
    // 在这里依然可以使用 x 却不能使用 s

    fn takes_ownership(some_string: String) {
        // 一个 String 参数 some_string 传入，有效
        println!("{}", some_string);
    } // 函数结束, 参数 some_string 在这里释放

    fn makes_copy(some_integer: i32) {
        // 一个 i32 参数 some_integer 传入，有效
        println!("{}", some_integer);
    } // 函数结束, 参数 some_integer 是基本类型, 无需释放


    // 函数返回值的所有权机制
    let s1 = gives_ownership();
    // gives_ownership 移动它的返回值到 s1

    let s2 = String::from("hello");
    // s2 被声明有效

    let s3 = takes_and_gives_back(s2);
    // s2 被当作参数移动, s3 获得返回值所有权

    // main结束 最后 s3 无效被释放, s2 被移动, s1 无效被释放.
    println!("s1,s3 {},{}", s1, s3);

    fn gives_ownership() -> String {
        let some_string = String::from("hello");
        // some_string 被声明有效

        return some_string;
        // some_string 被当作返回值移动出函数
    }

    fn takes_and_gives_back(a_string: String) -> String {
        // a_string 被声明有效

        a_string  // a_string 被当作返回值移出函数
    }

    // 引用与租借
    let s1 = String::from("hello");
    let s2 = &s1;
    println!("s1 is {}, s2 is {}", s1, s2);

    fn calculate_length(s: &String) -> usize {
        s.len()
    }


    //引用不会获得值的所有权。
    // 引用只能租借（Borrow）值的所有权。
    let s1 = String::from("hello");
    let mut s2 = &s1;
    let s3 = s1;
    s2 = &s3; // 重新从 s3 租借所有权 否则不能使用s2
    println!("{}", s2);

    //如果尝试利用租借来的权利来修改数据会被阻止：
    // s2.push_str("oob"); // 错误，禁止修改租借的值


    // 用 &mut 修饰可变的引用类型。
    // 可变引用与不可变引用相比除了权限不同以外，可变引用不允许多重引用，但不可变引用可以：
    let mut s1 = String::from("run");
    // s1 是可变的

    let s2 = &mut s1;
    // s2 是可变的引用

    s2.push_str("oob");
    println!("{}", s2);


    // 垂悬引用（Dangling References）
    let reference_to_nothing = dangle(s2);
    println!("{}", reference_to_nothing);
    dangle2(s2);
    println!("{}", s2);

    // 编译器检查 不允许逃逸
    fn dangle(s: &String) -> &String {
        &s
    }
    // 编译器检查 不允许逃逸
    fn dangle2(s: &mut String) {
        s.push_str("22")
    }
    /*
Rust 区别与其他高级语言的重要特征，在于其内存管理的两个特点：

（1）变量超出作用域会自动释放。对于简单值类型的栈内存（如int，struct）超出作用域后自动释放，这个逻辑在各个语言都有实现。而对于 new 出来的堆内存，在c/c++中是要手动释放的，在java和dotnet中要委托垃圾回收释放或手动写 dispose 语句释放。而垃圾回收不是实时的，会影响性能。而释放语句总会有懒人忘记写的。而 Rust 对栈内存和堆内存一视同仁，超出作用域一律自动释放。Rust 的这个特点在兼顾性能的情况下、有效的减少了代码量和内存泄漏隐患。

（2） “所有权” ：某段内存只能被最后的变量名所有，前面声明过的变量都作废，这有效的避免被多个变量释放的问题，而且该操作是在编译期就可以检查到的，这策略可在编译期就能有效的避免空指针问题。

部分对于所有权的设定，看起来很奇怪，其实本质上就是在语言层面禁止了同一个可变数据会有多个变量引用的情况，一旦作为参数传递了，就会发生所有权的移动（Move）或借用（Borrow）。赋值给另一个变更也就自动放弃了所有权。从根本上杜绝了并发情景下的数据共享冲突。
*/
}

// 循环
fn loops() {
    // while 循环
    let mut number = 1;
    while number != 4 {
        println!("{}", number);
        number += 1;
    }
    // for 循环
    let a = [10, 20, 30, 40, 50];
    for i in a.iter() {
        println!("值为 : {}", i);
    }

    // 下标来访问数组的
    for i in 0..5 {
        println!("a[{}] = {}", i, a[i]);
    }

    // 原生的无限循环结构 —— loop：
    let s = ['R', 'U', 'N', 'O', 'O', 'B'];
    let mut i = 0;
    loop {
        let ch = s[i];
        if ch == 'O' {
            break;
        }
        println!("\'{}\'", ch);
        i += 1;
    }

    // break 关键字类似于 return 一样使整个循环退出并给予外部一个返回值。
    let location = loop {
        let ch = s[i];
        if ch == 'O' {
            break i;
        }
        i += 1;
    };
    println!(" \'O\' 的索引为 {}", location);

    println!("EXIT");
}

// 条件语句
fn condition() {
    let number = 3;
    // 条件表达式必须是 bool 类型
    if number < 5 {
        println!("条件为 true");
    } else {
        println!("条件为 false");
    }

    // 函数表达式
    let a = 3;
    let number = if a > 0 { 1 } else { -1 };
    println!("number 为 {}", number);
}

// Rust 函数名称的命名风格是小写字母以下划线分割：
fn func() {
// fn <函数名> ( <参数> ) <函数体>

    //可以在一个用 {} 包括的块里编写一个较为复杂的表达式：
    let x = 5;

    let y = {
        let x = 3;
        // 注意：x + 1 之后没有分号，否则它将变成一条语句！
        x + 1
    };

    println!("x 的值为 : {}", x);
    println!("y 的值为 : {}", y);

//     函数定义可以嵌套

    fn five() -> i32 {
        5
    }
    println!("five() 的值为: {}", five());

    //     函数返回值
//     不支持自动返回值类型判断！如果没有明确声明函数返回值的类型，函数将被认为是"纯过程"，不允许产生返回值，return 后面不能有返回值表达式。
//  这样做的目的是为了让公开的函数能够形成可见的公报。
// 注意：函数体表达式并不能等同于函数体，它不能使用 return 关键字。
    fn add(a: i32, b: i32) -> i32 {
        return a + b;
    }
}

// 基础数据类型
fn data_type() {
    // 整型
    let mut i8num: i8 = 5;

    println!("{}", i8num);

//     Rust 与其它语言一样支持 32 位浮点数（f32）和 64 位浮点数（f64）。默认情况下，64.0 将表示 64 位浮点数，因为现代计算机处理器对两种浮点数计算的速度几乎相同，但 64 位浮点数精度更高。
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32

    //    注意：Rust 不支持 ++ 和 --，因为这两个运算符出现在变量的前后会影响代码可读性，减弱了开发者对变量改变的意识能力。

    let sum = 5 + 10; // 加
    let difference = 95.5 - 4.3; // 减
    let product = 4 * 30; // 乘
    let quotient = 56.7 / 32.2; // 除
    let remainder = 43 % 5; // 求余
//     布尔型
// 布尔型用 bool 表示，值只能为 true 或 false。
    let boolv = true;
//     字符型
// 字符型用 char 表示。
//     在Rust 中字符串和字符都必须使用 UTF-8 编码，否则编译器会报错。
    let mut str = "@";
    str = "2323";

    let str2: &str = "23";

//     复合类型
// 元组用一对 ( ) 包括的一组数据，可以包含不同种类的数据
    let tup: (i32, f64, u8) = (500, 6.4, 1);
// tup.0 等于 500
// tup.1 等于 6.4
// tup.2 等于 1
    let (x, y, z) = tup;
// y 等于 6.4

    // 数组
    let a = [1, 2, 3, 4, 5];
// a 是一个长度为 5 的整型数组

    let b = ["January", "February", "March"];
// b 是一个长度为 3 的字符串数组

    let c: [i32; 5] = [1, 2, 3, 4, 5];
// c 是一个长度为 5 的 i32 数组

    let d = [3; 5];
// 等同于 let d = [3, 3, 3, 3, 3];

    let first = a[0];
    let second = a[1];
// 数组访问

    // a[0] = 123; // 错误：数组 a 不可变
    let mut a = [1, 2, 3];
    a[0] = 4; // 正确
}

// 基础语法
fn grammar() {
//     变量
// 首先必须说明，Rust 是强类型语言，但具有自动判断变量类型的能力。这很容易让人与弱类型语言产生混淆。
// 如果要声明变量，需要使用 let 关键字。例如：
    //let 定义不可变变量
    //mut  定义变量可变
    let mut a = 123;
    println!("{}", a);
    a = 456;
    println!("{}", a);

    // 没有使用的变量会警告
    let a = 123;   // 可以编译，但可能有警告，因为该变量没有被使用
    println!("{}", a);
    let a = 456;
    println!("{}", a);

    // 定义变量类型
    let a: u64 = 123;
    println!("{}", a);

    // 重影（Shadowing）
    // 重影就是指变量的名称可以被重新使用的机制：
// 重影是指用同一个名字重新代表另一个变量实体，其类型、可变属性和值都可以变化
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);

    // 定义类型
    let a: u64 = 123;
    let mut s = "123";
    println!("{}", s);
    s = "23";
    println!("{}", s);
    let s = s.len();
    let w = s * 2;
    println!("{},{}", w, a);
}