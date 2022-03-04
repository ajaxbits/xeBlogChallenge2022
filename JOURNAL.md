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
