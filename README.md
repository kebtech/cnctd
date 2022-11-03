# cnctd

## Intro
This is the overall project folder for the app I'm currently calling cnctd, which should eventually be hosted at cnctd.world (unless I go with a different name). Other potential names I've tossed around for this project are Werk, werkspace, and werk_space. Who knows, in the end it might just be called HotDog.

The problem this app solves is rather simple: I need a place to store and group all my songwriting ideas, including audio and text/musical notation. I've been looking to build something like this for years, and after learning JavaScript and Rust since the start of the pandemic, I finally feel I can fully tackle this task. I've also built it in a way that it can be expanded to cover other creative project organization, such as book-writing.

<!-- Although a lot of work has gone into this app and it's quite functional, it's still very much in early development. You can (maybe) find the latest version at https://cnctd.herokuapp.com. Just be gentle as UI design is still very much in flux and the app could be in any state of functionality at any moment. -->

Please note: I am self-taught in coding and have been doing it for a little over 2 years now. I have never worked as an engineer within an organization and next to no one has seen my code. Because of this, some of the organization and way things are written may seem bizarre, but I assure you, most things work. And if they don't, they will be fixed. The way I approach coding is to envision the simplest way to build something and try to do it myself with as few libraries as possible. I often do rewrites if I think I could have written something in a more efficient way. 

## Organization
This app is split into two submodules: 

**werk_server**

This repository houses all the code for the database, the backend Rust server and the static web files.

**werk-web**

This contains the Vue project for the frontend, scaffolded with vue-cli.

## Architecture 
**Database - PostgreSQL**

I chose PostgreSQL because I needed the stability of a relational database while maintaining the flexibility to store some data in JSON objects while I develop the frontend structure. 

Currently, I'm hosting the DB using Heroku's free tier. At some point, this will either move or will be converted into a paid version.

**Server - Rust**

Although I find it quite simple to build a good backend using Node and Express (especially if you're integrating with a JS frontend), I decided to go with Rust for this project for a few reasons:
1) I recently learned Rust and am really excited at all it has to offer in performance and flexibility.
2) I will likely branch this into a desktop app using the Tauri framework and can repurpose a lot of the backend code.
3) I like a challenge.
4) Rust is really fun to write.

The server is integrated with the DB using the Diesel crate. Although there is a bit of a learning curve with Diesel, it seems to get the job done and makes it fairly easy to manage migrations.

All web traffic is handled through custom routes created using the Warp crate. I chose Warp because it allows you to integrate both REST APIs and websockets. I currently handle most of my communication via a custom websocket setup. It mostly works but definitely needs some additional love. (Side note: I love websockets)

**Frontend - Vue.js/Typescript**

After working with JQuery in previous projects, I realized I needed a reactive framework for this (and pretty much every project moving forward). I explored React for a bit before I was introduced to Vue. I find Vue 3 to be pretty logical and I love using the composition API because it allows me to structure my data the way I want and use vanilla Javascript to handle most functionality. I've been writing mostly in Typescript as it brings me a little bit closer to a type-based language like Rust. At some point, I need to go clean up all my loose "anys."

I plan to handle some future frontend functionality in Rust using WASM where appropriate.

This project is PWA-ready, although I haven't taken advantage of most of the functionality yet. IndexedDB will certainly be leveraged to store content offline.

**File Hosting**

Currently, all media files are hosted via S3. Files are both uploaded and downloaded via presigned URLs. The server is only responsible for retrieving the URLs. I may rip S3 out in the future, because I'm trying to get away from dependence on large companies like Amazon. 

**Site Hosting**

Currently, I'm using a free-tier Heroku account to host this app. This may also change if I find a good alternative, since Heroku utilizes AWS EC2 instances. If I move the server from Heroku, the database will likely come with it.

## Functionality
**Current**

In its current state, this app has the current functionality (all subject to change):
- User registration/login 
- File manager which supports the uploading and organization of audio, video, and image files
- Collections Manager which allows you to create and organize your music projects into Projects, Playlists, and Songs. There is also an additional Note collection type for quick notes. Although all the functionality is technically built, there are still quite a few design choices that need to be made to make sense of this functionality
- Rhyme dictionary
- Connecting with another account
- A player for audio
- Pitch detection/tuner
- Audio recorder - very elementary
- Tagging for Files and Collections
- Music Dictionary: look up keys and triads
- Add chords to lyrics

**Future**

- Midi router - it's technically there, but the underlying library doesn't seem to properly forward midi signals, so I need to investigate. I've also written this in former projects using a different midi library
- Sharing of collections both with other users in the app and with the public via a link
- Video Chat - just started working on this using the Mediasoup framework
- Multi-track recording (may end up in the full desktop Tauri app)