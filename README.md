Ever wanted to create some code that prints a message but you felt it was too readable?
Then I have a solution to you.

Simply have rust installed and run
```
cargo run msg.txt
```

> (instead of `msg.txt` you can of course use some other file) <br/>

and presto! you have your beautiful code in front of your eyes.

> It's reccomended to have at least one newline in your message since there's a bug which may print the first char of the table a couple times, and since the table is sorted by the ascii values the newline will end up the first.
> I'm not fixing it since right now it looks uglier than it would be after fixing it

<br/><br/><br/>

Heavily inspired by: https://www.youtube.com/watch?v=hmMtQe_mYr0
