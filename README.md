# imp-orter
the imp-orter is a WAD manager for doom. It allows the user to quickly choose mods they would like to run with their prefered Doom launcher. Additionally, it will have the ability to download the latest wads from doomworld.

## Author: Travis Block

## What was built?
Essentially, you could call this project a wrapper application that helps another, GZDOOM, to play modded files for Doom II.
Technically it could do other wad based games like Doom, and Heretic, but the most popular is Doom II.
The application helps with extracting mods and organizing them so you can easily play those mods.

I wanted to build this primarily for a few reasons.
1. There are tools that do this, but many are Windows only.
2. There is currently no tool like this for SteamOS and if I would like to continue to work on it,
it could have a place for a purpose.
3. I wanted a tool like this to use.

## How it works
The application leans heavily in the FileDirectory functions and is simply doing string generation of the paths
to files or folders needed to launch GZDoom and run mods.

GZDoom's application can be ran from command line and is easy to set launch parameters with. So, after the
application gets the essential paths to the files, they are concatenated and ran by the Command tool.

Additionally, the application can extract .wad files from zip files and move them to the mods directory that the user defines.
It currently only works with zips because that is the primary compression file that Doomworld.com's repository of files are set up.

## What Didn't Work
I had originally wanted to really dive into using Commands and performing Curl calls to pull zip files from
the mod database on doomworld.com. However, It was already a lot of effort to understand the needs of what was
currently implemented.

Something that I disliked was I could not find many examples online and often relied on the error messages to understand what was going on.
Admittedly it was a lot of guessing. The saving grace was that I was primarily working with strings only so it wasn't too hard to understand after getting it once.
Then, most of the code is very similar and it was easy to move forward. However, finding good examples that I could understand was difficult Even after reviewing the docs for everything I implemented.

Two functions that I had to have ChatGPT Help me with were the mod file path listings, and the zip extractions. I was very lost within the documentation.
In particular, the Slint documentation to get the array of vectors. Slint's documentation is lacking and to add further difficulties, they must have updated their
syntax recently to make it difficult to find good examples of the new one.

## Lessons learned

First and foremost, I really bit off a lot with using a UI framework. I ended up having to understand Rust, but then also
understand their syntax and how the framework works with Rust. If I didn't understand Closures after all the callback writing I had to do, then I failed myself.
With that, This project took a lot of time in so just building out the UI to which isn't Rust. So I felt very behind up until days before the submission date.

Despite this struggle, I am at a point in my development life where I do not want to build things without an interface. I certainly could have chosen a framework
that used web languages to render the ui (of which I'd be familiar with), but Slint was more interesting to me as it can be used on 
embedded systems, and I am hoping to take that Rust class here at PSU and use it.

Aside from the scope creep that seemingly always happens. I learned a lot about understanding the file directory and what I would need to
do in order for this application to run on whatever. Ideally I hope in the near future I can figure out how to actually package this project up for use outside
of an IDE. 

Overall, I am very satisfied with the outcome even if it felt like I was running into the darkness most of the time. 
