# _SCELLER_: <i>Systems Components Entities Lacking List Ekindled in Rust</i>

## The Entity Component System that was **not** given the name because someone had already taken SECS.

<p1>SCELLER is en entity component system written in Rust built following a tutorial by [Brooks Builds](https://www.youtube.com/channel/UCT1-XRVnJA-wws2bfbLbFcQ) on Youtube and his [series on how to create an entity component system](https://www.youtube.com/watch?v=CTuTEi4YUb8&list=PLrmY5pVcnuE_SQSzGPWUJrf9Yo-YNeBYs) (written in Rust).</p1>

<p1>Sceller is built on a DOD which means **DATA ORIENTED DESIGN**. This means that instead of your code being centered around objects, all you do is write code that takes **data** concretely, writing **system** around accessing this data. If you want to learn more, I suggest Googling (or using Bing. If you *reaaally* want to) the term [ECS](https://en.wikipedia.org/wiki/Entity_component_system).</p1>

This project currently has no graphics, audio, video, literally anything. It is JUST an entity component system. If you're looking for something actually useful, try [bevy](https://bevyengine.org/), it's really very cool. 

If you want to install and tinker around with the code yourself, run the following commands if you're on Windows:

First, you're going to want to create a new directory: (you can name the folder whatever you want, I named mine sceller_test.) It also goes without saying you will need [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) installed on your device., and 

# IMPORTANT!

Windows users will need [Visual studio build tools](https://visualstudio.microsoft.com/downloads/) 2012, 2017, 2019, 2022, or other I can't exactly remember if you want to compile this crate's dependencies, which are eyre and thiserror. Feel free to clone the repo and remove the need for these crates if you feel like combing through terrible code. And don't think I'm happy about requiring Microsoft's C thing since they won't just let people use c++ normally.

Back to what we were doing:

```bash
C:\...> mkdir sceller_test\
C:\...> cd sceller_test\
C:\...> dir
C:\...> 
```
Then clone the repository into that directory, making sure to add a '.' (fullstop) at the end of the command to make sure to clone into the current directory.
```bash
C:\...> git clone https://github.com/loafoflead/sceller .
Cloning into 'sceller'...
remote: Enumerating objects: 78, done.
remote: Counting objects: 100% (78/78), done.
remote: Compressing objects: 100% (43/43), done.
remote: Total 78 (delta 32), reused 78 (delta 32), pack-reused 0
Receiving objects: 100% (78/78), 22.99 KiB | 2.87 MiB/s, done.
Resolving deltas: 100% (32/32), done.
C:\...> 
```
Then, you can type 'dir' to check the contents of the folder, see if it's all there:
```bash
C:\...> dir
12:19                src
12:19                tests
12:19             22 .gitignore
12:19            279 Cargo.toml
12:19              0 README.md
C:\...>
```
It should look something like the above, with a bit more clutter.

If you want to test this code in Rust, run 
```bash
C:\...> cargo test canary
Updating crates.io index...
    fetch [=>               ] 12/101
...
C:\...>
```
And wait for it to finish. You can also simply run cargo test to see if all the tests pass. 

If you want to find some actual implementations of the crate, check the ```examples``` folder for examples. To run these examples do:
```bash
C:\...> cargo run --example resource_example
```

If you're on Mac or linux, I am pretty sure the process is similar, though less tedious. 
I'm afraid I don't know exactly which dependencies are needed on GNU/linux, but if you're a Rust 
programmer you probably have them installed already, libc and stuff like that, or whatever comes with [build-essential](https://askubuntu.com/questions/158996/how-do-i-get-the-libc-development-libraries-for-ubuntu-12-04) on Ubuntu.

I may test it sometime, I have WSL on my laptop.

# Note

There are some limitations with this library, the main ones being related to Query Functions. 
It goes without saying that you shouldn't use this crate to make an actually serious project, but 
you should check the Query Functions docs anyway, since it shows the limitations.

Happy ecs-ing!