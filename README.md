# Alex's Submission for [Xe's Blog Challenge 2022](https://christine.website/blog/new-language-blog-backend-2022-03-02)

This is my submission for Xe's blog challenge 2022 :)

I want to learn Rust, and I've already been writing a HTML templator/SSG for my partner's [website](https://gracebobber.com/), which I wrote and maintain. I feel like this has been a great project to learn Rust, so I want to jump ath the opportunity to do this challenge, since I think I'll get a lot out of it!

I will be keeping a [journal of my thoughts here](./JOURNAL.md).

## TODO
- [X] An abstract "Post" datatype with a title, publication date, a "URL slug" and content in plain text
- [-] A home page at / that describes yourself to the world
    - [ ] Get more granular about this
- [X] A list of blog posts at /blog
- [X] Individual blog posts at /blog/{date}/{slug}
- [-] A /contact page explaining how to contact you
- [ ] An admin area at /admin/\* that is protected by a username and password
- [X] An admin form that lets you create new posts
- [X] An admin form that lets you edit existing posts
- [X] An admin form that lets you delete posts
- [X] An admin view that lets you list all posts
- [X] Use a CSS theme you like (worst case: pick one at random) to make it look nice
- [X] HTML templates for the base layout and page details

### Extra Credit :)
- [X] Add an "updated at" date that shows up if the post has been edited
- [ ] Add tags on posts that let users find posts by tag
- [ ] JSONFeed support
- [ ] "Draft" posts which aren't visible on the public blog index or feeds, but can be shared by URL
- [ ] Use CSRF protection on the admin panel
- [ ] Deploy it on a VPS and serve it to the internet (use Caddy to make this easier)
- [ ] Pagination of post lists

### Xe's recommended steps from the blog
- [X] Serve a static file at / that contains `<h1>Hello, world!</h1>`
- [X] Create a SQLite connection and the posts table
- [X] Insert a post into your database by hand with the sqlite3 console
- [X] Wire up a /blog/{date}/{slug} route to show that post
- [X] Wire up /blog to show all the posts in the database
- [X] Make static pages for / and /contact
- [X] Choose a templating language and create a base template
- [-] Edit all your routes to use that base template
- [ ] Create the admin login page and a POST route to receive the HTML form and check the username/password against something in the SQLite database, creating a session for the admin panel
- [ ] Create an external tool that lets you manually set your username and password
- [ ] Create an admin view that shows all posts
- [ ] Create a form that lets you create a new post and its associated POST handler
- [ ] Create a form that lets you edit an existing post and its associated POST handler
- [ ] Use a CSS theme to make it all look nice
