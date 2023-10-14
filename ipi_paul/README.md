# Useful Rust Functions

## toggle_case

Just as I was starting the first tutorial in the Noq streaming series, I made the usual mistake of typing a web address with the Caps key on. 

 - This something I have always wanted to rectify.
 - This was my first attempt and takes in the second and third parameters when calling the app and toggles the case of each character.
 - On checking VS Code I saw that its function is not available. 

### To run the App and call this function

```
cargo build --release
target/release/ipi_paul togglecase eXAMPLE
```

## toggle_case_clip

 - This uses the clipboard crate.
 - I then added shortcut keys to VS Code to call this in the terminal.
 - Due to an issue not yet fixed where the updated clipboard is cleared once the function completes (going out of scope), I added a 5 second delay to give me enough time to paste in the updated clipboard content.

### To run the App and call this function

```
cargo build --release
target/release/ipi_paul togglecaseclip
```

## swap_around_clip

 - This uses the termion crate to color parameter markers.
 - Between the end of episode 5 and the beginning of episode 6 in the Noq streaming series, tsoding swaped around the shaping syntax.
 - This also uses the clipboard crate.
 - There is one required parameter and two optional parameters.
   - ***--swap*** character/string to swap on (required).
   - ***--unswapped*** characters/strings not to swap around (optional: separated with a space).
   - ***--remove*** characters/strings not wanted (optional: separated with a space).

### To run the App and call this function

```
cargo build --release
target/release/ipi_paul swaparoundclip
target/release/ipi_paul swaparoundclip --swap '|' --unswaped ! --remove '#'
target/release/ipi_paul swaparoundclip --swap , --unswaped alphanumeric
target/release/ipi_paul swaparoundclip --swap =
target/release/ipi_paul swaparoundclip --swap AND
```
Or, append to your code using <- to indicate the right hand side as parameters:
```
left_side | and not right_side
<- {
  swap: '|',
  unswapped: ["and"],
  remove: ["not"]
}
```
Run app without additional paremeters:
```
cargo build --release
target/release/ipi_paul swaparoundclip
```
Result will be:
```
right_side  | and left_side
```

## outer_replace

 - Used for trying out diferent enclosures when coding.
 - You can swap out the opening and closing parentheses, brackets with braces and visa versa.
 - Works soley on the clipboard and uses the clipboard crate.
 - To use add <- to indicate the right hand side to swap out with the left hand side.
 - Add a Shortcut key combination to VS Code to run when terminal has already been initiated.

### To run the App and call this function

Add to your code, select and cut/copy:
```
(let x = 3) <- {}
```
Then run:
```
cargo build --release
target/release/ipi_paul outerreplace
```
Paste the updated clipboard:
```
{let x = 3}
```
