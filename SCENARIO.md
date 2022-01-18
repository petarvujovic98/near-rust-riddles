# Contract scenario

This file explains what the basic logic behind the contract is.

**Note**: for details on how to use this contract, please see the [README.md](README.md).

## Idea

The contract is supposed to simulate a riddle board with rewards.
Anyone can post a riddle and put up money as a bounty for solving it.
Anyone can look at the list of riddles, either solved or unsolved, and look at a certain riddle's details.

You can ask for a riddle's hint by paying a certain amount of NEAR.
You can also try to solve a riddle by paying a fee for each attempt.

Each time a user requests a hint, the bounty on that riddle increases by the amount of NEAR that was spent on the hint.
Likewise, each time a user tries to solve a riddle and fails, the bounty on that riddle increases by the amount of NEAR that was spent on the attempt.

## Models

There are two models in this contract:

- `Riddle`: a riddle with a title, description, hint, answer, bounty and status
- `Board`: a board that saves all the riddles

### Riddle

This model is the basic building block of the contract. It carries all the information about a riddle, as well as stores the bounty of the riddle.
We also have a RiddleView struct that is used to display the riddle without the hint and answer.

### Board

This model is the main contract and stores all the riddles. It handles the creation of new riddles, the retrieval of riddles, purchasing hints and solving riddles.
Riddle retrieval is free, while the creation of riddles, hint purchasing and riddle solving are charged. This mechanisam incentivises users to take the least amount of attempts and ask for hints as little as possible, and also _ranks_ the riddles by diffuculty based on the amount of NEAR spent on solving them.
