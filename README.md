Ever wanted to create some code that prints a message but you felt it was too readable? <br/>
Then I have a solution to you.

Simply have rust installed and run
```sh
cargo run msg.txt
```

> (instead of `msg.txt` you can of course use some other file) <br/>

and presto! you have your beautiful code in front of your eyes.

> It's reccomended to have at least one newline in your message since there's a bug which may print the first char of the table a couple times, and since the table is sorted by the ascii values the newline will end up the first.
> I'm not fixing it since right now it looks uglier than it would be after fixing it

# Example
`example.txt`
```
Hello there, my friend.
How are you on this fine day?
```

Output:
```c
#include <stdio.h>
#include <stdint.h>

int main() {
	uint64_t tables[] = {0x6461483F2E2C200A, 0x6F6E6D6C69686665, 0x797775747372};
	uint64_t indicies[] = {0x0A12010F0C0C0805, 0x01150D0102081008, 0x0003070E080B1009, 0x0108100601140F05, 0x12010E0F01130F15, 0x080E0B0901110B0A, 0x000415060701};
	int j = 8;
	for (int i = 0; i < 07; i++)
		while (j == -1 ? (j = 7) : j--)
			putchar(tables[(indicies[i] >> ((0b111 - j) * 010) & 0377) / 010] >> (((indicies[i] >> ((0b111 - j) * 8) & 0377) % 010) * 010));
	return 0;
}
```

<br/><br/><br/>

Heavily inspired by: https://www.youtube.com/watch?v=hmMtQe_mYr0
