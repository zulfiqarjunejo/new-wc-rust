# new-wc-rust

*Technologies used:* ![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white) ![Git](https://img.shields.io/badge/git-%23F05033.svg?style=for-the-badge&logo=git&logoColor=white)

Implementing a replica of Unix' `wc` tool for the coding challenge: [click here](https://codingchallenges.fyi/challenges/challenge-wc)

## Execution

1. To count number of bytes, execute the following command.

```bash
    ./wc -c test.txt
```

2. To count number of lines, execute the following command.

```bash
    ./wc -l test.txt
```

3. To count number of words, execute the following command.

```bash
    ./wc -w test.txt
```

4. To count number of characters, execute the following command.

```bash
    ./wc -m test.txt
```

**Note:** If no command-line options are passed, nwc will print count of lines, words and bytes by default. For example:

```bash
    ./wc test.txt
```

**Note:** It is also possible to chain out of another command as an input for nwc. For example:

```bash
    cat test.txt | ./wc
```

## Contribution

Please feel free to create pull requests if you like to add any feature. Also, feel free to take snippets of code from this project.