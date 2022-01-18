/**
 * Welcome to near-workspaces-ava!
 *
 * This is a working test which checks the functionality of [the status-message
 * contract][1]. For quick reference, here's the contract's implementation:
 *
 *     impl StatusMessage {
 *         pub fn set_status(&mut self, message: String) {
 *             let account_id = env::signer_account_id();
 *             self.records.insert(&account_id, &message);
 *         }

 *         pub fn get_status(&self, account_id: String) -> Option<String> {
 *             return self.records.get(&account_id);
 *         }
 *     }
 *
 * As you can see, this contract only has two methods, a setter and a getter.
 * The setter sets a status message for the account that signed the call to the
 * contract. The getter accepts an `account_id` param and returns the status for
 * that account.
 *
 * The tests below create a local blockchain with this contract deployed to
 * one account and two more accounts which store statuses in the contract.
 *
 *   [1]: https://github.com/near-examples/rust-status-message/tree/4e4767db257b748950bb3393352e2fff6c8e9b17
 */

/**
 * Start off by importing Workspace from near-workspaces-ava.
 */
import { NearAccount, toYocto, Workspace } from "near-workspaces-ava";

/**
 * Initialize a new workspace. In local sandbox mode, this will:
 *
 *   - Create a new local blockchain
 *   - Create the root account for that blockchain (see `root` below)
 *   - Execute any actions passed to the function
 *   - Shut down the newly created blockchain, but *save the data*
 */
const workspace = Workspace.init(
  async ({
    root,
  }): Promise<{ account: NearAccount; contract: NearAccount }> => {
    // Create a subaccount of the root account, like `alice.sandbox`
    // (the actual account name is not guaranteed; you can get it with `alice.accountId`)
    const account = await root.createAccount("test");

    // Create a subaccount of the root account, and also deploy a contract to it
    const contract = await root.createAndDeploy(
      "near-rust-riddle",
      "../res/near_rust_riddles.wasm",
      { method: "new" }
    );

    return { account, contract };
  }
);

workspace.test("root creates riddle", async (test, { contract, root }) => {
  await root.call(
    contract,
    "create_riddle",
    { title: "riddle1", text: "riddle1", answer: "riddle1", hint: "riddle1" },
    { attachedDeposit: toYocto("1") }
  );

  test.deepEqual(await contract.view("get_riddle", { title: "riddle1" }), {
    title: "riddle1",
    text: "riddle1",
    bounty: +toYocto("1"),
    solved: false,
  });
});

workspace.test(
  "create riddle, get riddle hint, solve riddle",
  async (test, { account, contract, root }) => {
    await root.call(
      contract,
      "create_riddle",
      { title: "riddle1", text: "riddle1", answer: "riddle1", hint: "riddle1" },
      { attachedDeposit: toYocto("1") }
    );

    const hint = await account.call<string>(
      contract,
      "get_riddle_hint",
      { title: "riddle1" },
      { attachedDeposit: toYocto("0.1") }
    );

    await account.call<void>(
      contract,
      "solve_riddle",
      { title: "riddle1", answer: "riddle1" },
      { attachedDeposit: toYocto("0.1") }
    );

    const solved = await account.call<boolean>(contract, "get_riddle_solved", {
      title: "riddle1",
    });

    test.is(hint, "riddle1");

    test.is(solved, true);
  }
);

workspace.test(
  "create multiple riddles, solve some, list solved and list unsolved",
  async (test, { account, contract, root }) => {
    for (let i = 0; i < 10; i++) {
      await root.call<void>(
        contract,
        "create_riddle",
        {
          title: `riddle${i}`,
          text: `riddle${i}`,
          answer: `riddle${i}`,
          hint: `riddle${i}`,
        },
        { attachedDeposit: toYocto("1") }
      );
    }

    for (let i = 0; i < 10; i += 2) {
      await account.call<void>(
        contract,
        "solve_riddle",
        { title: `riddle${i}`, answer: `riddle${i}` },
        { attachedDeposit: toYocto("0.1") }
      );
    }

    const solved = await account.call<
      [
        string,
        { title: string; text: string; solved: boolean; bounty: number }
      ][]
    >(contract, "get_riddles", { from_index: 0, limit: 10, solved: true });

    const unsolved = await account.call<
      [
        string,
        { title: string; text: string; solved: boolean; bounty: number }
      ][]
    >(contract, "get_riddles", { from_index: 0, limit: 10, solved: false });

    const all = [...Array(10).keys()].map((i) => ({
      title: `riddle${i}`,
      text: `riddle${i}`,
      solved: i % 2 === 0,
      bounty: +toYocto("1"),
    }));

    test.deepEqual(
      solved,
      all.filter((r) => r.solved).map((r) => [r.title, r])
    );
    test.deepEqual(
      unsolved,
      all.filter((r) => !r.solved).map((r) => [r.title, r])
    );
  }
);

// For more example tests, see:
// https://github.com/near/workspaces-js/tree/main/__tests__
