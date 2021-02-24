use std::rc::Rc;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;
use regex::internal::Input;

//pub fn do_rc() {
//    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
//    println!("rc a = {}", Rc::strong_count(&a));
//    let _b = Cons(3, Rc::clone(&a));
//    println!("rc after creating b = {}", Rc::strong_count(&a));
//    {
//        let _c = Cons(4, Rc::clone(&a));
//        println!("count after creating c = {}", Rc::strong_count(&a));
//    }
//    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
//}

pub fn get_largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}


pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

enum Message {
    NewJob(Job),
    Terminate,
}

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate message to all workers.");

        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        println!("Shutting down all workers.");

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();

            match message {
                Message::NewJob(job) => {
                    println!("Worker {} got a job; executing.", id);

                    job();
                }
                Message::Terminate => {
                    println!("Worker {} was told to terminate.", id);

                    break;
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}

fn do_basic_data_type() {
    // 字符类型i8
    let _a: i8 = -10;

    // 无字符类型u16
    let _b: u16 = 10;

    // 浮点类型f64
    let _a_f64: f64 = 100.01;

    // 单一字符类型char
    let _a_char: char = '*';

    // 布尔类型bool
    let _is_success: bool = true;

    // 后缀注解定义i32的数据b_integer
    let _b_integer   = 3i32;

    // 默认类型f64，i32， 通过数据推测的类型
    let _default_f64   = 100.0; // `f64`
    let _default_i32 = 16;   // `i32`

    // 通过上下文推测数据类型，通过下文4294967296i64推测inferred_i32为i64
    let mut inferred_i32 = 12;
    inferred_i32 = 88888888888i64;

    // mut变量值是可以修改的
    let mut m_i32 = 100; // Mutable `i32`
    m_i32 = 2000;

    // 下划线可以被插入数据类型中，这样可以提高可读性
    // 10_000_000_000 == 1000000000
    let _y: u64 = 10_000_000_000;
    println!("一千万可以这样写 {}", 10_000_000u32);

    // i32数据类型的数组，数组大小固定为6
    let aa:[i32; 6] = [10, 12, 14, 16, 18, 20];

    use std::mem;
    // 数组是栈分配
    println!("数组占 {} bytes", mem::size_of_val(&aa));

    // 切片化int_slice一个矢量
    let vec = vec![1, 2, 3];
    let _int_slice = &vec[..];
    // 强迫一个数组变成一个切片（slice）
    let _str_slice: &[&str] = &["one", "two", "three"];

    // 元组定义
    let _cc = ('a', 'A', 12222, 20000.0001);
}

#[derive(Debug)]
struct Employee {
    name: String,
    age: u8,
    address: String,
}

// 单元结构
struct Unit;

// 定义一个包含两个元素的元组结构
struct Pair(i32, f32);

// 3D 点坐标
struct ThreeDPoint {
    x: f32,
    y: f32,
    z: f32,
}

//定义结构体Rectangle
#[allow(dead_code)]
struct Rectangle {
    // A rectangle can be specified by where the top left and bottom right
    // corners are in space.
    top_left: ThreeDPoint,
    bottom_right: ThreeDPoint,
}

fn do_struct() {
    //快速创建一个name变量
    let name = String::from("张三");
    let a = String::from("广州天河区珠江新城");
    let age = 25;
    let user_info = Employee { name , age,  address: a };
    // Print debug struct
    println!("{:?}", user_info);
    // 实例化一个`3DPoint`
    let point: ThreeDPoint = ThreeDPoint { x: 10.3, y: 0.4, z: 0.5 };

    // Access the fields of the point
    println!("3DPoint 坐标: ({}, {}, {})", point.x, point.y, point.z);

    // 用其他实例初始化结构体的属性
    let bottom_right = ThreeDPoint { x: 5.2, ..point };

    println!("第二个3D point: ({}, {}, {})", bottom_right.x, bottom_right.y, bottom_right.z);

    // 使用let绑定析构一个点
    let ThreeDPoint { x: top_edge, y: left_edge,z: right_edge } = point;

    let _rectangle = Rectangle {
        // 结构体初始化作为struct属性的值
        top_left: ThreeDPoint { x: left_edge, y: top_edge, z: right_edge },
        bottom_right: bottom_right,
    };

    // Instantiate a unit struct
    let _unit = Unit;

    // 初始化元组
    let pair = Pair(1, 0.1);

    //访问元组的元素
    println!("pair 包含 {:?} 和 {:?}", pair.0, pair.1);

    // 析构一个元组结构
    let Pair(integer, decimal) = pair;

    println!("pair 包含 {:?} 和 {:?}", integer, decimal);
}

enum MouseEvent {
    PointerMove,
    MouseWheel,
    // like tuple structs,
    KeyPress(char),
    Paste(String),
    // or c-like structures.
    Click { x: i64, y: i64 },
}

// 检测鼠标事件
fn listen_mouse_event(event: MouseEvent) {
    match event {
        MouseEvent::PointerMove => println!("指针移动"),
        MouseEvent::MouseWheel => println!("鼠标滚轮滑动"),
        // 析构c
        MouseEvent::KeyPress(c) => println!("按键 '{}'.", c),
        MouseEvent::Paste(s) => println!("张贴 \"{}\".", s),
        // 析构Click .
        MouseEvent::Click { x, y } => {
            println!("点击位置 x={}, y={}.", x, y);
        },
    }
}

fn listen_web_event(){
    let pressed = MouseEvent::KeyPress('x');
    // `to_owned()` creates an owned `String` from a string slice.
    let pasted  = MouseEvent::Paste(" 拷贝文本".to_owned());
    let click   = MouseEvent::Click { x: 20, y: 80 };
    let mouse_move    = MouseEvent::PointerMove;
    let mouse_wheel  = MouseEvent::MouseWheel;

    listen_mouse_event(pressed);
    listen_mouse_event(pasted);
    listen_mouse_event(click);
    listen_mouse_event(mouse_move);
    listen_mouse_event(mouse_wheel);
}

// C-like enum
fn do_enum_like_c() {
    // starts at 0
    enum Number {
        Zero,
        One,
        Two,
    }

    // enum with explicit discriminator
    enum Color {
        Red = 0xff0000,
        Green = 0x00ff00,
        Blue = 0x0000ff,
    }
    // `enums` can be cast as integers.
    println!("zero is {}", Number::Zero as i32);
    println!("one is {}", Number::One as i32);

    println!("roses are #{:06x}", Color::Red as i32);
    println!("violets are #{:06x}", Color::Blue as i32);
}


fn do_guess_number() {
    use rand::Rng;
    use std::cmp::Ordering;
    use std::io;
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

static LIBRARY_NAME: &str = "天河图书馆";
const DEFAULT_THRESHOLD: i32 = 100;

fn is_big(n: i32) -> bool {
    n > DEFAULT_THRESHOLD
}

fn do_static() {
    let n = 16;

    // Access constant in the main thread
    println!("天河区的图书馆 {}", LIBRARY_NAME);
    println!("临界值= {}", DEFAULT_THRESHOLD);
    println!("{} 是 {}", n, if is_big(n) { "big" } else { "small" });

    // 错误不可能修改常量
    // DEFAULT_THRESHOLD = 500;
}

fn do_vectors() {
    let mut x = vec![1, 2, 3, 4, 5];

    for i in &x {
        println!("引用 {}", i);
    }

    for i in &mut x {
        println!("一个可变变量的引用 {}", i);
    }

    for i in x {
        println!("矢量的所有权和它的元素{}", i);
    }

}

fn do_shadowing() {
    let x = 100;

    let x = x + 1;
    let x = x * x;
    println!("x = {}", x);

    let spaces = "   "; // 3个空格
    println!("spaces len = {}", spaces);

    // 字符串操作
    let s: &str = "Rust Programming";
    let s: String = s.to_uppercase();
    println!("{}", s); // RUST PROGRAMMING

    let a = 5;
    let _b = a + 1; //6
    let a = 5;
    let _c = a - 1; //4
    let a = 5;
    let _d = a * 2; //10
    let a = 5;
    let _e = a / 2; // ⭐️注意是 2 not 2.5
    let a = 5;
    let _f = a % 2; //1

    let _g = 5.0 / 2.0; //2.5

    let a = 1;
    let b = 2;

    let _c = a == b; //false
    let _d = a != b; //true
    let _e = a < b; //true
    let _f = a > b; //false
    let _g = a <= a; //true
    let _h = a >= a; //true

    let _i = true > false; //true
    let _j = 'a' > 'A'; //true

    let a = true;
    let b = false;

    let _c = !a; //false
    let a = true;
    let _d = a && b; //false
    let a = true;
    let b = false;
    let _e = a || b; //true

    let a = 1;
    let b = 2;

    let _c = a & b;  //0  (01 && 10 -> 00)
    let a = 1;
    let b = 2;
    let _d = a | b;  //3  (01 || 10 -> 11)
    let a = 1;
    let b = 2;
    let _e = a ^ b;  //3  (01 != 10 -> 11)
    let a = 1;
    let b = 2;
    let _f = a << b; //4  a向左位移两位， -> 100)
    let a = 1;
    let b = 2;
    let g = a >> b; //0  a向右位移两位   ̶ -> 0)
    println!("g = {}", g);

    let mut a = 2;

    a += 5; //2 + 5 = 7
    a -= 2; //7 - 2 = 5
    a *= 5; //5 * 5 = 25
    a /= 2; //25 / 2 = 12 not 12.5
    a %= 5; //12 % 5 = 2

    a &= 2; //10 && 10 -> 10 -> 2
    a |= 5; //010 || 101 -> 111 -> 7
    a ^= 2; //111 != 010 -> 101 -> 5
    a <<= 1; //'101'+'0' -> 1010 -> 10
    a >>= 2; //101̶0̶ -> 10 -> 2
    println!("a = {}", a);

    let a = 15;
    let b = (a as f64) / 2.0; //7.5
    println!("b = {}", b);
}

fn do_control_flow(){
    // 月薪数额
    let salary = 7000;

    let salary_level = if salary < 5000 {
        "small"
    } else if salary < 8000 {
        "Medium"
    } else {
        "Large"
    };
    println!("你的薪资基本属于 : {}", salary_level);

    let trousers_width = 21;
    let trousers_size = match trousers_width {
        16 => "S", // check 16
        17 | 18 => "M", // check 17 and 18
        19 ..= 21 => "L", // check from 19 to 21 (19,20,21)
        22 => "XL",
        _ => "Not Available",
    };

    println!("trousers_size : {}", trousers_size); // L

    let is_allowed = false;
    let list_type = match is_allowed {
        true => "Full",
        false => "Restricted"
        // 所有情况已经覆盖
    };

    println!("list_type = {}", list_type); // Restricted

    let pair_a: u8 = 79;
    let pair_b: u8 = 79;

    let output = match (pair_a, pair_b) {
        (100, 100) => "a 和 b都非常优秀",
        (100, _) => "a 非常优秀",
        (_, 100) => "b 非常优秀",
        (x, y) if x > 80 && y > 80 => "a和吧都很好",
        (_, _) => "继续加油！"
    };
    println!("评选结果： {}", output); // 继续加油！

    let mut i = 0;

    loop {
        if i == 0 {
            println!("Skip Value : {}", i);
            i += 1;
            continue;
        } else if i == 2 {
            println!("跳出点i = {}", i);
            break;
        }

        println!("当前值 : {}", i);
        i += 1;
    }

    let mut b1 = 1;

    'outer_loop: loop { //设置外循环标签
        let mut b2 = 1;

        'inner_loop: loop {
            println!("当前值 : [{}][{}]", b1, b2);

            if b1 == 2 && b2 == 2 {
                break 'outer_loop; // 跳出外循环
            } else if b2 == 5 {
                break; // 跳出内循环
            }

            b2 += 1;
        }

        b1 += 1;
    }

    let mut n = 0;
    while n < 10 {
        print!("n = {} \t", n);
        n += 1; // Rust 无++ 操作符
    }

    let mut b = 0;
    while b < 5 {
        if b == 0 {
            println!("跳过点  : {}", b);
            b += 1;
            continue;
        } else if b == 2 {
            println!("断开点 : {}", b);
            break;
        }

        println!("当前b = {}", b);
        b += 1;
    }

    let mut c1 = 1;
    'outer_while: while c1 < 6 { //设置外循环标签
        let mut c2 = 1;

        'inner_while: while c2 < 6 {
            println!("当前值[c1][c2] = [{}][{}]", c1, c2);
            if c1 == 2 && c2 == 2 { break 'outer_while; } // 跳出外循环
            c2 += 1;
        }

        c1 += 1;
    }

    // 0 到 5 ，不包括10的循环。类似for(i = 0; i < 5; i++)
    for i in 0..5 {
        println!("序列值i = {}", i);
    }

    // 0 到 5 ，包括5的循环。类似for(i = 0; i <= 5; i++)
    for i in 0..=5 {
        println!("序列值 i = {}", i);
    }

    let cellphone_list : [&str; 4] = ["小米", "OPPO", "荣耀", "Vivo"];
    for n in 0..cellphone_list.len() { // cellphoneList.len() = 4 -> 0..4 每次都会执行每个迭代的cellphoneList.len()
        println!("cellphone : {}", cellphone_list[n]);
    }

    for cellphone in cellphone_list.iter() { // group.iter() 转换数组到一个简单的迭代器iterator
        println!("手机品牌 : {}", cellphone);
    }
}


fn do_generic() {
    fn get_word_count_from_file(_file_name: &str) -> Result<u32, &str> {
        return Err("文件不存在!")
        // 否则，正确的计算得到的词汇量word_count
        // let mut word_count: u32 = 10000;
        // Ok(word_count);
    }
    let file_name = "file_a";
    match get_word_count_from_file(file_name) {
        Ok(i) => println!("Word Count: {}", i),
        Err(e) => println!("Error: {}", e)
    }
    let number_list = vec![34, 50, 25, 100, 65];
    let result = get_largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result =  get_largest(&char_list);
    println!("The largest char is {}", result);


}

fn do_impl_trait() {
    struct Person {
        first_name: String,
        last_name: String,
    }

    impl Person {
        fn full_name(&self) -> String {
            format!("{} {}", self.first_name, self.last_name)
        }
    }

    let person_1 = Person {
        first_name: "张".to_string(),
        last_name: "晓明".to_string(),
    };

    println!("人员 01: {}", person_1.full_name());

    do_impl_trait2();
}

fn do_impl_trait2() {
    struct Person {
        first_name: String,
        last_name: String,
    }

    trait FullName {
        fn full_name(&self) -> String;
    }

    impl FullName for Person {
        fn full_name(&self) -> String {
            format!("{} {}", self.first_name, self.last_name)
        }
    }

    let person_1 = Person {
        first_name: "张".to_string(),
        last_name: "晓明".to_string(),
    };

    println!("人员 01: {}", person_1.full_name());
    do_impl_trait3()
}

fn do_impl_trait3() {
    struct Person {
        first_name: String,
        last_name: String,
    }

    impl Person {
        fn new(first_name: String, last_name: String) -> Person {
            Person {
                first_name : first_name,
                last_name : last_name,
            }
        }

        fn full_name(&self) -> String {
            format!("{} {}", self.first_name, self.last_name)
        }
    }

    let full_name = Person::new("张".to_string(), "晓明".to_string()).full_name();
    println!("person 姓名: {}", full_name);
    do_impl_trait4();
}

fn do_impl_trait4() {
    trait GetSound {
        fn get_sound(&self) -> String;
    }

    struct Cat {
        sound: String,
    }
    impl GetSound for Cat {
        fn get_sound(&self) -> String {
            self.sound.clone()
        }
    }

    struct Bell {
        sound: String,
    }
    impl GetSound for Bell {
        fn get_sound(&self) -> String {
            self.sound.clone()
        }
    }


    fn make_sound<T: GetSound>(t: &T) {
        println!("{}!", t.get_sound())
    }

    let kitty = Cat { sound: "Meow".to_string() };
    let the_bell = Bell { sound: "Ding Dong".to_string() };

    make_sound(&kitty); // Meow!
    make_sound(&the_bell); // Ding Dong!
}

fn do_ownership() {
    let a = [1, 2, 3];
    let b = a;
    println!("{:?} {:?}", a, b); // [1, 2, 3] [1, 2, 3]
    do_ownership2();
}

fn do_ownership2() {
    let a = vec![1, 2, 3];
    let _b = a;
    //    println!("{:?} {:?}", a, b); //  Error; use of moved value: `a`
    fn change_and_get_first_element(a: &mut Vec<i32>) -> i32 {
        a[0] = 5;
        a[0]
    }

    let mut a = vec![1, 2, 3];
    let b = change_and_get_first_element(&mut a);

    println!("{:?} {}", a, b); // [5, 2, 3] 5
    a[0] = 10;
    println!("{:?} {}", a, b); // [10, 2, 3] 5
}

fn do_lifetime() {
    struct Struct<'a> {
        x: &'a str
    }
    impl<'a> Struct<'a> {
        fn function(&self) -> &'a str {
            self.x
        }
    }
    do_lifetime2();
}
fn do_lifetime2() {
    struct Struct<'a> {
        x: &'a str,
        y: &'a str
    }
    impl<'a> Struct<'a> {
        fn new(x: &'a str, y: &'a str) -> Struct<'a> { // No need to specify <'a> after new; impl already has it
            Struct {
                x: x,
                y: y
            }
        }
    }
    do_lifetime3();
}

fn do_lifetime3() {
    // fn function<F>(f: F) where for<'a> F: FnOnce(&'a Type){}
//    struct Struct<F> where for<'a> F: FnOnce(&'a Type) { x: F }
//    enum Enum<F> where for<'a> F: FnOnce(&'a Type) { Variant(F) }
//    impl<F> Struct<F> where for<'a> F: FnOnce(&'a Type) { fn x(&self) -> &F { &self.x } }
    fn greeting<'a>() -> &'a str {
        "Hi!"
    }
    fn fullname<'a>(fname: &'a str, lname: &'a str) -> String {
        format!("{} {}", fname, lname)
    }
    struct Person<'a> {
        fname: &'a str,
        lname: &'a str
    }
    impl<'a> Person<'a> {
        fn new(fname: &'a str, lname: &'a str) -> Person<'a> { // No need to specify <'a> after new; impl already has it
            Person {
                fname : fname,
                lname : lname
            }
        }

        fn fullname(&self) -> String {
            format!("{} {}", self.fname , self.lname)
        }
    }
    let player = Person::new("张", "晓明");
    let player_fullname = player.fullname();

    println!("姓名: {}", player_fullname);
}
fn greet() -> String {
    "Hello, world!".to_string()
}

#[test] // Test attribute indicates this is a test function
fn test_greet() {
    assert_eq!("Hello, world!", greet());
    greetings::greet();
    phrases::greetings::greet();
}

mod greetings {
    // ⭐️ By default, everything inside a module is private
    pub fn greet() { // ⭐️ So function has to be public to access from outside
        println!("Hello, Rust!");
    }
}

#[cfg(test)] // Only compiles when running tests
mod phrases {
    fn private_fn() {
        println!("Hello, Rust5!");
    }
    pub mod greetings {
        pub fn greet() {
            println!("Hello, Rust2!");
            say();
        }
        fn say() {
            println!("Hello, Rust3!");
            super::private_fn()
        }
    }
    use super::greet; // Import root greet function

    #[test]
    fn test_greet() {
        assert_eq!("Hello, world!", greet());
    }
}




#[allow(unused_variables)] //💡  线头属性用于禁止警告：未使用的变量`y`
fn do_smart_pointer() {
    let x = vec![1, 2, 3];
    let y = x;

//    println!("{:?}", x);

    struct Color {
        r: u8,
        g: u8,
        b: u8,
    }
    let yellow = Color {
        r: 255,
        g: 255,
        b:222,
//        d: 0,
    };

    println!("Yellow = rgb({},{},{})", yellow.r, yellow.g, yellow.b);

    do_cell();
}

fn do_cell() {

    use std::cell::{RefCell, RefMut};
    use std::collections::HashMap;
    let shared_map: Rc<RefCell<_>> = Rc::new(RefCell::new(HashMap::new()));
    // Create a new block to limit the scope of the dynamic borrow
    {
        let mut map: RefMut<_> = shared_map.borrow_mut();
        map.insert("africa", 92388);
        map.insert("kyoto", 11837);
        map.insert("piccadilly", 11826);
        map.insert("marbles", 38);
    }

    // Note that if we had not let the previous borrow of the cache fall out
    // of scope then the subsequent borrow would cause a dynamic thread panic.
    // This is the major hazard of using `RefCell`.
    let total: i32 = shared_map.borrow().values().sum();
    println!("Rc<RefCell<_>> shared_map total: {}", total);

    do_cell2();
}

fn do_cell2() {
    use std::cell::Cell;

    struct XPoint {
        regular_x: u8,
        special_x: Cell<u8>,
    }

    let a_struct = XPoint {
        regular_x: 0,
        special_x: Cell::new(1),
    };
    let new_value = 100;
    // WORKS, although `a_struct` is immutable, field `special_x` is mutable because it is Cell
    a_struct.special_x.set(new_value);
    assert_eq!(a_struct.special_x.get(), new_value);

    do_cell3();
}

fn do_cell3() {
    use std::cell::RefCell;
    let cell = RefCell::new(5);
    let old_value = cell.replace(6);
    assert_eq!(old_value, 5);
    assert_eq!(cell, RefCell::new(6));

    // The borrow lasts until the returned RefMut or all RefMuts
    // derived from it exit scope. The value cannot be borrowed while this borrow is active.
    let c = RefCell::new("hello".to_owned());
    *c.borrow_mut() = "bonjour".to_owned();
    assert_eq!(&*c.borrow(), "bonjour");

    use std::cell::Cell;
    let x = Cell::new(1);
    let y = &x;
    let z = &x;
    x.set(2);
//    y.set(3);
//    z.set(40);
    println!("cell x={}", x.get());

    do_cell4();
}

fn do_cell4() {
    let mut x = 1;
    let z = &mut x;
    *z = 40;
    println!("cell x={}", x);
}
fn do_pattern() {
    let v = vec!['x', 'y', 'z'];
    for (index, value) in v.iter().enumerate() {
        println!("{} 的索引 {}", value, index);
    }
    struct Point {
        x: i32,
        y: i32,
    }
    let p = Point { x: 0, y: 7 };

    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }

    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.")
        }
        Message::Move { x, y } => {
            println!(
                "Move in the x direction {} and in the y direction {}",
                x, y
            );
        }
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => println!(
            "Change the color to red {}, green {}, and blue {}",
            r, g, b
        ),
    }

    let x = 4;
    let y = false;

    match x {
        4 | 5 | 6 if y == true => println!("yes"),
        _ => println!("no"),
    }
}

fn do_box() {
    //    let a = 5;
//    let b = Box::new(a);
//
//    println!("a = {}", a);
//    println!("b = {}", *b);
    struct MyBox<T>(T);

    impl<T> MyBox<T> {
        fn new(x: T) -> MyBox<T> {
            MyBox(x)
        }
    }

    use std::ops::Deref;

    impl<T> Deref for MyBox<T> {
        type Target = T;

        fn deref(&self) -> &T {
            &self.0
        }
    }
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
    do_drop();
}

fn do_drop() {
    struct CustomSmartPointer {
        data: String,
    }

    impl Drop for CustomSmartPointer {
        fn drop(&mut self) {
            println!("清理CustomSmartPointer 数据：`{}`!", self.data);
        }
    }

    {
        let _c = CustomSmartPointer {
            data: String::from("A stuff"),
        };
        let _d = CustomSmartPointer {
            data: String::from("B stuff"),
        };
    }// c/d 的范围结束，调用c/d 的drop方法，其顺序与创建顺序相反。
    println!("清理结束");
    do_drop2()
}

fn do_drop2() {
    struct CustomSmartPointer {
        data: String,
    }
    impl Drop for CustomSmartPointer {
        fn drop(&mut self) {
            println!("清理CustomSmartPointer 数据：`{}`!", self.data);
        }
    }

    use std::mem::drop;
    let c = CustomSmartPointer {
        data: String::from("do something ..."),
    };
    println!("CustomSmartPointer 创新了，并且使用完毕.");
    drop(c);
    println!("此刻CustomSmartPointer 已经清理");

}

fn do_error_handling() {
    let e = vec![1, 2, 3];
    println!("e[2] = {} ", e[2]);
    //    println!("e[3] = {} ", e[3]);
//    use std::fs::File;
//    let f = File::open("hello.txt").unwrap();
    use std::fs::File;
    use std::io;
    use std::io::Read;

    fn read_username_from_file() -> Result<String, io::Error> {
        let mut f = File::open("hello.txt")?;
        let mut s = String::new();
        f.read_to_string(&mut s)?;
        Ok(s)
    }
    read_username_from_file();
}

fn do_concurrent() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        for i in 1..10 {
            println!("子线程: {}!", i);
            thread::sleep(Duration::from_millis(1));
        }
        println!("参数所有权v: {:?} ",  v);
    });
    for i in 1..5 {
        println!("主线程{}!", i);
        thread::sleep(Duration::from_millis(1));
    }
    handle.join().unwrap();
    do_pc();
}

fn do_pc() {
    use std::sync::mpsc;
    use std::thread;
    use std::time::Duration;
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("one"),
            String::from("two"),
            String::from("three"),
            String::from("four"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("1"),
            String::from("2"),
            String::from("3"),
            String::from("4"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });

    for received in rx {
        println!("得到: {}", received);
    }
    do_arc();
}

fn do_arc() {
    use std::sync::{Arc, Mutex};
    use std::thread;

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..19 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}

fn do_unsafe() {
    let raw_p_number: *const u32 = &1000;

    unsafe {
        assert!(*raw_p_number == 1000);
    }

    let mut num = 1000;
    let n1 = &num as *const i32;
    let n2 = &mut num as *mut i32;
    unsafe {
        println!("*const i32 n1 = {}", *n1);
        println!("*mut i32 n2 = {}", *n2);
    }
    do_unsafe_fn();
}

extern "C" {
    fn abs(input: i32) -> i32;
}

fn do_unsafe_fn() {
    use std::slice;
    let some_vector = vec![1, 2, 3, 4];

    let pointer = some_vector.as_ptr();
    let length = some_vector.len();

    unsafe {
        let a_slice: &[u32] = slice::from_raw_parts(pointer, length);

        assert_eq!(some_vector.as_slice(), a_slice);
    }

    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-100));
    }
    do_static_am();
}

static mut COUNTER: u32 = 0;
fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}
fn do_static_am() {
    add_to_count(3);
    add_to_count(1);
    add_to_count(1);
    unsafe {
        println!("计数: {}", COUNTER);
    }
}

fn do_overloading() {
    use std::ops::Add;

    #[derive(Debug, PartialEq)]
    struct Point {
        x: i32,
        y: i32,
    }

    impl Add for Point {
        type Output = Point;

        fn add(self, other: Point) -> Point {
            Point {
                x: self.x + other.x,
                y: self.y + other.y,
            }
        }
    }

    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );

    do_full_qualified_syntax() ;
}

fn do_full_qualified_syntax() {
    trait Pilot {
        fn fly(&self);
    }

    trait Astronaut {
        fn fly(&self);
    }

    struct Human;

    impl Pilot for Human {
        fn fly(&self) {
            println!("Pilot is flying by air plane.");
        }
    }

    impl Astronaut for Human {
        fn fly(&self) {
            println!("Astronaut is up!");
        }
    }

    impl Human {
        fn fly(&self) {
            println!("Flying by tools");
        }
    }

    let person = Human;
    // 人飞行
    person.fly();
    // 飞行员飞行
    Pilot::fly(&person);
    // 宇航员飞行
    Astronaut::fly(&person);

    do_full_qualified_syntax2();
}

fn do_full_qualified_syntax2() {
    trait Vehicle {
        fn get_name() -> String;
    }

    struct Bus;

    impl Bus {
        fn get_name() -> String {
            String::from("Bus")
        }
    }

    impl Vehicle for Bus {
        fn get_name() -> String {
            String::from("Abstract vehicle")
        }
    }
    println!(">>> {}", Bus::get_name());
//    println!(">>> {}", Vehicle::get_name());  // error
    println!(">>> {}", <Bus as Vehicle>::get_name());

    do_full_qualified_syntax3();
}

fn do_full_qualified_syntax3() {
    use std::fmt;
    trait OutlinePrint: fmt::Display {
        fn outline_print(&self) {
            let output = self.to_string();
            let len = output.len();
            println!("{}", "*".repeat(len + 4));
            println!("*{}*", " ".repeat(len + 2));
            println!("* {} *", output);
            println!("*{}*", " ".repeat(len + 2));
            println!("{}", "*".repeat(len + 4));
        }
    }
    struct Point {
        x: i32,
        y: i32,
    }
    impl fmt::Display for Point {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "({}, {})", self.x, self.y)
        }
    }
    impl OutlinePrint for Point {}
    OutlinePrint::outline_print(&Point{x: 110, y: 120});
}

fn do_fn_pointer() {
    fn add_one(x: i32) -> i32 {
        x + 1
    }

    fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
        f(arg) * 2
    }
    let answer = do_twice(add_one, 5);
    println!("f(5) do_twice =: {}", answer);
}

use std::net::TcpListener;
use std::io::prelude::*;
use std::net::TcpStream;
use std::fs;

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();


    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(3));
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}



fn do_simple_web_listener() {
    let listener = TcpListener::bind("0.0.0.0:8000").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down.");
}


pub fn main_thinking() {
    do_basic_data_type();
    do_struct();

    // enum 枚举
    listen_web_event();
    do_enum_like_c();

    // game: guess a number
    // do_guess_number();

    // const vs static
    do_static();

    //vectors
    do_vectors();
    do_shadowing();

    //control flow
    do_control_flow();
    do_generic();
    do_impl_trait();
    do_ownership();
    do_lifetime();

    do_smart_pointer();
    do_pattern();
    do_box();
    do_error_handling();
    do_concurrent();
    do_unsafe();
    do_overloading();
    do_fn_pointer();
//    do_simple_web_listener();

}