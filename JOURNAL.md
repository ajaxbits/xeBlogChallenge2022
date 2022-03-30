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

## 2022-03-11
- Got the 404 to work for a non-existent post!
- Also discovered the [`actix_web` documentation](https://actix.rs/docs/), which I'll be using going forward.

## 2022-03-13
- In a great place! Just tested that I can insert another post into the blog db, and that it will show up in the list after a simple refresh! So live insertion of posts will totally be possible!
- Also, spent a lot of time thinking about deploying with [naersk](https://github.com/nix-community/naersk), but started really spinning my wheels and wasting my time. Not that deployment considerations aren't important -- In fact, that's one of the things I'm most excited about -- but in terms of time management, I feel that I should focus on code first, then deployment.
    - The upside, though, is that I now can build a working nix package and docker image from that package. So a good start!
- Time to start working on the admin stuff, which I'm not exactly relishing. Seems like a hard challenge!
- Going to go with http basic auth for now, but build in a modular way so that I can switch it out if necessary.
- Also going to start by just fetching plaintext from the db, but I'm sure that's not secure lol
- Ok, I just discovered [this app](https://github.com/purton-tech/barricade/tree/master/src), which seems to have some good code to ~steal~ borrow from for inspiration.
- Also, I think I'm just going to put tha admin auth down for now, and do the admin post form first instead

## 2022-03-14
- Feeling a little scattered and all over the place. But going to record my recent progress, then do somthing that nobody has ever thought to do before and is sure to place me on the 30 Under 30 list: I'm going to _make a plan_.
- Recent progress:
    - Finally figured out how image tagging and everything worked to be able to run my package as docker container on boot on my NixOS server box. Used `virtualisation.oci-containers` to do it. See the `flake.nix` for the docker configuration.
    - Got a better sense for the landscape of auth and of the `actix_web` crate ecosystem. Going to go with Http Auth for now, since that's what will get me past the MVP stage. However, I _really_ want to make it modular, so I can change it one day if I want to. 
- Plan:
    - Going to start on these challenges:
        ```markdown
        - [ ] Create a form that lets you create a new post and its associated POST handler
        - [ ] Create a form that lets you edit an existing post and its associated POST handler
        ```
    - Research forms in actix
        - May require more knowledge of forms in general, we shall see.
    - Create a post handler that will put that information into the DB.
    - The two bullets above need to be their own functions, I think.
- Ok, after that extensive planning session, I feel good to start mocking things up.
- You know, it would probably be a good idea to have generic `create_post` and `edit_post` functions!
    - Stealing from [here](https://github.com/actix/examples/blob/master/basics/todo/src/model.rs) today.
- Wow, ok, two hours later and I'm still messing around with my data models. The above link has some really cool ideas that I'm borrowing that will hopefully make this code a lot easier for me to understand. Leaving it for tonight, but got some good work done, I think!
- One last thing, I'm finally starting to understand `map_err`! I think I can use it to map the error type that is returned from Result into another Error type. I can use this to map the template and sqlx errors in `/blog` to `error::ErrorInternalServerError`s!
    - I'm sure there are a lot of things that will be problematic with this. Like how do we determine how to map errors to another type? Or how do I know the error type I'm mapping to is one that the parent function will "accept"? These are all questions that show my continuing ignorance -- but I feel like I'm making some good progress on this topic!

## 2022-03-15
- Wow, I really went too hard this morning on enums and impl's. Going to take a break, but this is a reminder to myself to journal about this.
- This is a memorial to some cool code that I spent all day obsessively writing, but think I'm going to trash. Here's the headstone:
```rust
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum FormMethod {
    Add,
    Edit,
}
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum FormMethodError {
    NotFound,
}

impl FromStr for FormMethod {
    type Err = FormMethodError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "add" => Ok(FormMethod::Add),
            "edit" => Ok(FormMethod::Edit),
            _ => Err(FormMethodError::NotFound),
        }
    }
}

impl fmt::Display for FormMethodError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Could not find the admin path specified.")
    }
}

impl ResponseError for FormMethodError {
    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        match self {
            FormMethodError::NotFound => HttpResponse::NotFound().finish(),
        }
    }
}
```
- What I learned from this, is that I could take any call to `/admin/*`, create a method that would transform that string into an enum member, then do logic based on that. It was really really cool to be able to write my own type, then neatly tuck everything away inside of an enum, that I could match against very cleanly and prettily. 
- What was also cool to learn was how to implement a `ResponseError` for my custom error type. I could transform a failed string parse into a `NotFound` error, which is what it would really represent in this case! It was cool to go from the very abstract "I was unable to extract a FromMethod from this string," to the much more contextually-aware statement "It looks like this string isn't one of the allowed strings for this /edit endpoint." All using the type system! Very neat.
- However, this is causing me intense headaches, since I failed to take into account that someone would, I don't know, want to _pass in a post to the `/edit` endpoint to actually edit it_...
- Therefore, I think I'm going to scrap this and come up with a new plan.
    - Fundamentally, I think I'm going to want to call the `/edit/{date}/{slug}` endpoint to edit a given post. Feels more extensible this way.
    - This seems to be **really** hard to handle with the custom types I defined above. I think I'm just going to have to repeat some code somewhere, and define an `add` function that just takes the db and templates as args, and an `edit` function that takes the db, template, date, and slug as args. I don't really see any other ways to do this right now with my current type system and current skill level.
- Makes me very sad to see it go, but I'm glad I could at least record my learning here.



- [sqlite uuid reference](https://github.com/launchbadge/sqlx/issues/1083)
- Figure out a way to store the uuids in the db already as their 16 bit representations
- Add a way to view posts and add new posts from `/admin`
- parse slugs added through the form to make sure they'll work
    - `lol_html`?

- Do I need more than a `u8` for serializing the vector? 
    - using this example for now:https://github.com/rusqlite/rusqlite with the option u8

- Store true false as 0/1 in sqlite
    - Using [this](https://github.com/serde-rs/serde/issues/1344) to deserialize the values I get.


















https://stackoverflow.com/questions/51128832/what-is-the-best-way-to-design-a-tag-based-data-table-with-sqlite

- Tags are actually really really hard. Like how to store them? How to display them? Lots to think about
- https://turreta.com/2020/06/07/actix-web-basic-and-bearer-authentication-examples/
- https://auth0.com/blog/build-an-api-in-rust-with-jwt-authentication-using-actix-web/
