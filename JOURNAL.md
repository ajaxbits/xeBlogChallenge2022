# Journal

## 2022-03-04
- Starting the project today :)
- Deciding to start with the code from Sylvain Kerkour's blog post, "[Building a static site generator in 100 lines of Rust](https://kerkour.com/rust-static-site-generator)," since I'm already familiar with that code.
    - I used that code as the base for the "News" section of [gracebobber.com](https://github.com/ajaxbits/grace-bobber-web), so it just makes sense in my brain as a starting point. 
    - Helps me get over that "blank page" feeling. But I do want to think more carefully about the structure that I might want, since I think that's where I'll learn the most. I don't want to automatically fall into the patterns that the blog lays out -- even if they're good -- since I want to find my own way.
- Thinking in broad strokes:
    - Host on netlify, since I probably want to keep using that.
    - Flake install and automatic build via GH push?
- Scared to to SQLite for some reason. Like for some reason I feel like there's a "better" way to do this?? But I've never worked with SQL before at all, so the whole thing is a little foreign to me, ansd I'm like scared of state??? Like databases feel like mutable state, that I don't want? I don't know. I should look into it. But like can I do this in a different way?
    - I feel like I'd want to uperate on markdown file checked into a git repo, honestly.
- Actually, I think I'm going to go with something like [this](https://github.com/actix/examples/blob/master/basics/basics/src/main.rs)
- Going with actix because it's #1 on [lib.rs](https://lib.rs) for web servers lol

## 2022-03-05
> **A Quick Love Letter to Rust's `Option<T>`**:
> I just want to take a quick second of appreciation for the `Option` type in Rust. Before this, the only major executable thing I programmed was a python thing, and I can't tell you how many hours went down the drain due to `None`. That thing was the bane of my existence. The `Option` type seems, at this moment in my journey, like an excellent answer to this problem. Still gives me the information I need about possible incompatible values, but doesn't screw me over unexpectedly. And the compiler/rust-analyzer will tell me about it before I even `cargo r`! Goodbye, dozens of hours of running and rerunning before finding an errant `None`!

- Reading [Rust by Example](https://doc.rust-lang.org/rust-by-example/) to get more familiar with things.
- Been thinking about it, and realizing that the web server I picked actually serves all the content! I know, seems like a dumb thing to point out, but let me explain:
    - My only experience with web stuff so far has been with Netlify and [serving static files in a directory structure](https://github.com/ajaxbits/grace-bobber-web).
    - However, with `actix_web`, I can arbitrarily define where content will be served, endpoint-wise. So I can put my content wherever, and actix will _serve that content at an arbitrary endpoint I specify_. (At least that's how I think it works in this moment).
    - This is super cool, since it helps me get away from the limitations that the heirarchical structure of my static site.
    - However, I now have to have a place to _run the server_. It doesn't seem like I can use a Netlify for this project anymore, since `actix_web` doesn't output files. At least I don't think it does right now. 
    - I want to keep using actix, since I've never worked with this kind of server before, and would love the learning.
    - However, I will need to see if it can output files. Since I'm not opposed to spinning up a server of a FaaS thing to host the blog, but that's way more expensive than _free_, which something like Neflify can provide.

## 2022-03-07
- Going to work with the `Post` type for now, just assuming that I'll get a SQLite connection going in the future.

## 2022-03-08
- No notes recently, since we're in the hard part of this thing
- Just set up the SQLite dateabse. Easier than I thought
    - I hope it's ok to check the db into git. Like I hope that's what one "does."
- Ok, now I have to set up this route to work...
    - Going to follow this [example](https://github.com/actix/examples/blob/master/databases/sqlite/src/db.rs) from the `actix_web` repo

## 2022-03-09
- Ok, started getting Rust to talk to the DB. 
    - Started with `rusqlite` and got it to be able to get a vec of posts. 
    - However, I wonder if I want to render these routes dynamically? Like do I want actix to handle this??
    - Going to switch to the example's way, with a connection pool and all of that, since it seems better. More complicated, but better.

## 2022-03-10
- It's aliveeeeeee
- Been a struggle recently, so I've not been as verbose lol
- Decided to 'dynamically' query the posts db when I need to fetch a post. I kind of want to immediatley post something from the post form, and be able to see the route. So I think that's the way. At least for now.
