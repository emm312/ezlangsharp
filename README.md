An interpreted-vm language.

# Syntax examples
```
fun main() {
    println("hello world")
}
```
```
fun main() {
    let a = 5;
    let b = 6;
    if a + b == 11 {
        ret 0;
    } else {
        a = a + 1;
    }
    let c = "hello world";
    println(c);

    let d = [1, 2, 3];
    println(d);
}
```