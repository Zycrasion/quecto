# Example Programs
These programs should compile once the checklist is complete.

## Fibonacci Sequence
```
fn:u8 start()
{
    let:i32 n1 = 0;
    let:i32 n2 = 1;
    loop while n < 100
    {
        let:i32 n3 = n1 + n2;

        io.print(n1);

        n1 = n2;
        n2 = n3;
    }
    
    return 0;
}
```

## cat
Referring to the linux command.

```
fn:u8 start()
{
    if env.argc == 0
    {
        return 1;
    }

    let fd:i32 = io.open(env.args[1]);
    if fd == -1
    {
        io.print("File Doesn't Exist")
        return 1;
    }

    let contents:str = io.read_file(fd);
    io.print(contents);
    return 0;
}
```