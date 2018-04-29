# h2s-stuff

## Native game of life
If you don't already have Rustup installed, do it with:
```
curl https://sh.rustup.rs -sSf | sh
```
Answer the questions with default answers.

Then, after cloning the repository (`git clone https://github.com/anirudhb/h2s-stuff.git`):
```
cd GameOfLife-native && cargo build && cargo run
```

#### Controls
Key | Action | Holdable/Draggable
----|--------|-------------------
Tab | Step one generation. | Yes
 T  | Step 10 generations. | Yes
 B  | Toggle continuously running the simulation. | Yes?
Right arrow or L | Move the cursor right. | Yes
Left arrow or H | Move the cursor left. | Yes
Down arrow or J | Move the cursor down. | Yes
Up arrow or K | Move the cursor up. | Yes
Enter or Space | Toggle the cell under the cursor. | Yes
Left mouse button | Move the cursor to the mouse. | No
Right mouse button | Toggle the cell under the mouse. | No
