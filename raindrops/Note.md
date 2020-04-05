本次练习主要收获的有两点：

### 字符串连接方式

#### 字符串类型变量创建方式
目前用了两种，分别是：

##### 创建空字符串类型
`String::new()`
#####  创建一个有初始值的字符串类型
`String::from("init")`

#### 连接方式
目前使用到了两种，分别是：

#####  `push_str` or `push`
前者是向后添加字符串，后者是向后添加字符

eg.0 `result.push_str("Hi")`

eg.1 `result.push('h')`

#####  传统 `+` 连接
eg.0 `result += "Hello"`


### 数字、字符串之间的类型转换
类型转换一般分为基本类型转换和复杂类型转换。

#### 基本类型转换
基本类型转换一般会用到 `as` 或者在 `unsafe` 块中使用 `transmute`。

#### 复杂类型转换
##### 数字转字符串
由于基本类型都实现了 `to_string` 的方法，这个类似于 JS 中的 `toString`。

eg.0 `target.to_string()`

##### 字符串转数字
可以使用 `parse` 方法解析字符串。

eg.0 `target.parse::<u32>()`


### 参考
- https://doc.rust-lang.org/std/string/struct.String.html
- https://wiki.jikexueyuan.com/project/rust/casting-between-types.html
- https://doc.rust-lang.org/std/primitive.str.html#method.parse