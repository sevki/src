<img src="taocp.png" align="right" width="200px" />


# src Language

`src` is a domain specific language for manipulating source code and building, progressively distiributed apps or [PDA](https://fistfulofbytes.com/progressive-distributed-apps/).

It draws a lot of inspiration from [Effekt](https://www.effekt-lang.org/) and [Koka](https://koka-lang.github.io/koka/doc/kokaspec.html) languages.

`src` is main aim is to provide a gradually distributed programming
environment for building software. 

It tries to achive these goals by providing a thin veneer over the operating systems `libc` or equivalent by treating the syscalls to the operating system as effects.

Therefore the operating system becomes the [effect handler](https://effect-handlers.org/) for the execution environment.
 
```src
use { host } from std

effect Make: async + throws + execs + reads + writes {
    catch() [throws]
    await<T>(f: Future<T>) [async, throws] -> T
    exec(arg0: string, args: stringvec) [Make] -> i32
}

struct Local {
    host: host
}

impl Make for Local {
    fn catch(self) [throws] {
    }
    fn await<T>(f: Future<T>) [async, trhows] -> T {
        yield()
    }
    fn exec(self, arg0: string, args: vec<string>) [Vm] -> i32 {
        self.host.read("jobserver").await
        if self.host.exec(arg0, args) {
            raise(1)
        }
    }
}
```
