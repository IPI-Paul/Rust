# tsoding Rust Streams

[Noq Ep.01: New Math Language in Rust](https://m.youtube.com/watch?v=Ra_Fk7JFMoo), [Noq Ep.02: Rust Macros](https://m.youtube.com/watch?v=LYIn_Ewpq-E), [Noq Ep.03: I guess I'm making Another Language](https://m.youtube.com/watch?v=LjI8-JyR864), [Noq Ep.04: Proving Basic Math](https://m.youtube.com/watch?v=Ma4hPxc107s), [Noq Ep.05: Improving Syntax of My Language](https://m.youtube.com/watch?v=Y7vyw_FFj8I), [Noq Ep.06: Cool shell in Rust](https://m.youtube.com/watch?v=qKA2NZ1-kx0)

## Course Work (noq)

 - Learning rust for the first time I wanted to know how to take the user input of a number and combine it with a string to call a function with a name that matches that combination. 
 - Episodes 1 to 2 were easy to follow along and the subject matter is something I have wondered about in the past when writing VBA macros and developing functions in other programming languages.
 - Episode 3 did not cover all the changes tsoding had made, and rather laboriously, I paused the video every time he scrolled through the code and moved the video frame by frame until I got all the changes to enable me to then follow along.
 - Episodes 4, 5 and 6 again did not cover all the changes and this time I used the github history to track the changes and update the code to the point I thought the video was starting from. Unfortunately, due to not compiling in the history for episode 4 I copied historical changes that were ahead of the episode start.

 [I rewrote My Go App in Rust](https://youtu.be/BbIEuNscn_E?si=9X0yaMvBKaJyHJqn)

 ## Course Work (go_app)

  - This course gives further insight into the workings of TCP and sockets by building a chat app server.
  - tsoding carried out a DDOS attack on the app being built and when following along and using the terminal in VS Code (Codium), it brought my system to a halt. Running both the App and the attack in the Linux terminal emulator did not bring the system to a halt.
  - Having learnt how to build a Web App in Go and finding it quite easy, I am now interested to see if the same can be achieved in Rust.
  - The Go App tsoding built and converted only used 158 lines of code, but in Rust it used 189 lines plus an additional line I thought preferable.
  - The discussion around Go Telemetry puts me off studying Go any further and it would be good to find further tutorials on building Rust solutions.
  - It would be good to learn more on building GUIs in Rust as the Warp YouTube video indicates that this is an area that needs vast improvement.

[My Viewers DDoSed my Rust App](https://youtu.be/Sw12N7-zqkk?si=Pz2nCF-XcXw5U96m)

## Course Work (server)

 - This course introduces the concepts of compiling multiple binanries in Rust.
 - The server chat app from the previous course is enhanced with code to produce a random access token and an authentication process.
 - In order to generate random tokens, rutls and getrandom were installed as dependencies.
 - The server was tested by live twitch viewers of the tutorial stream and was DDOSed when multiple IP addresses were used as part of the attack.

[Unreasonably Easy Console Apps in Rust](https://youtu.be/vc5UPu76XOw?si=zdWYtBy0mWFDM9Ty)

## Course Work (client)

 - Although not a fully fledged GUI like those easily achieved in Python and R Programming with the use of tKinter libraries, the console UI produced in this course looks good and works well.
 - The course introduces a client console app for connecting and disconnecting to the server chat app from above.
 - In order to have more control over the console terminal, crossterm was installed as a dependency.

[Forbidden Rust](https://youtu.be/LQ2rX5B0DUA?si=36CQtgO3aQ07kvJk)

## Course Work (command)

 - Finally answers my question of whether Rust can call functions by name dynamically.
 - It is quite a lengthy way to do so, as calling functions by name in Go is simple and straight forward. But, I'm not sure how many lines of code is used in reflect's MethodByName.
 - Although it worked on the version of Rust I am using and tsoding was using, the twitch stream suggested that new versions of Rust would break this app.
 - I agree with tsoding, it would be great to have this app working as it gives the ability to add comments to tutorial exercises, list all exercise names with their comments and call those exercises by name.
 - Gives a quick introduction in the use of a Makefile to compile the procedural macro file as a dynamic library and the main file as a binary.

 [Search Engine in Rust](https://youtu.be/hm5xOJiVEeg?si=qxJ6iCgdB7N-sGdg)
 [Rust Web Development (Search Engine)](https://youtu.be/OYAKjlYm_Ew?si=GfqZuIL6pbm0SxA6)
 [Speeding up Rust Code](https://youtu.be/b0KIDIOL_i4?si=rzdpum819b9xseEI)
 [NLP in Rust](https://youtu.be/zRZZ8i8YhGU?si=7rPE9uWHBH68M87n)
 [Rust Multi-Threading](https://youtu.be/9JULWqjM0Wg?si=IMhDJwUKs2YzA4gr)
 [I compared Two PDF Libraries. C one was faster than Rust one](https://youtu.be/vb7pX7pI7lI?si=9OX2lUwbXlOXIsw_)

## Course Work (seroost)

 - Having a local search engine tool is something I find useful, as I have found on occasion that Windows File Explorer does not always produce results when searching for text within files in a folder. The search engine in this course differs from the one I developed in many ways. I used R Programming to build an app that uses the tkinter library as a GUI and reads all files with a specified extension in a folder and its sub folders searching for the search term. Then the app outputs the results in to a html file and opens it in the browser app feature.
 - I'm not sure whether tf-idf really works without combining words. When searching for a search term, I often feel that the combination of words given should be an important factor in ranking the results. Unfortunately, these tutorials do not cover this factor.
 - Learning which web service to use and how to use it was a definite plus and something that would make Rust one of my top considerations when developing apps.
 - Although VS Code and Rust Analyzer make it easy to work with Rust, a lot of the time I had to just wait and see how tsoding worked out the conflicts.
 - I have made a few modifications here and there using the VS Code and Rust Analyzer hints.
 - I have also added to the JavaScript and Rust code the ability to display the output as links and display the linked files in the browser.
 - Unfortunately, the Snowball add-on does not work with all search terms and will need to improve for the Natural Language Processing (NLP) element of these courses.
 - Performance and speed are continuously improved upon throughout the course and the final feature of adding pdf searches makes this a very useful app.
 - Episodes ep01 to ep06_pre are all unde MIT licence, but due to using poppler ep06 is under GPL licence.

