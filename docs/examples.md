# Examples

## innitguv
```src
use { native_fs, native_exec } from host
use { fs } from std

struct Innitguv {
    fs: native_fs,
    exec: native_exec
    current_pid: i32
}

impl Exec for Innitguv {
    fn exec(&self, arg0: str, args: vec<str>) [nd, exec, await] -> i32 {
        let path = arg0
        let pid = self.exec.exec(path, args)
        if pid == -1 {
            return -1
        }
        self.current_pid = pid
        yield()
    }
}

impl Actor for Innitguv {
    fn recv(&self, msg: Message) [recv, await] {
        self.exec(msg.path, msg.args)
    }
}
```
