import { NearAccount, toYocto, Worker } from "near-workspaces";
import anyTest, { TestFn } from "ava";

const test = anyTest as TestFn<{
  worker: Worker;
  accounts: Record<string, NearAccount>;
}>;

/* If using `test.before`, each test is reusing the same worker;
If you'd like to make a copy of the worker, use `beforeEach` after `afterEach`,
which allows you to isolate the state for each test */
test.beforeEach(async (t) => {
  const worker = await Worker.init();
  const root = worker.rootAccount;
  const contract = await root.createAndDeploy(
    "near-rust-riddle",
    "../res/near_rust_riddles.wasm",
    { method: "new" }
  );
  /* Account that you will be able to use in your tests */
  const ali = await root.createSubAccount("ali");
  const bob = await root.createSubAccount("bob");
  t.context.worker = worker;
  t.context.accounts = { root, contract, ali, bob };
});

test("Root creates", async (t) => {
  const { contract, root } = t.context.accounts;

  await root.call(
    contract,
    "create_riddle",
    {
      title: "riddle1",
      text: "riddle1",
      answer: "riddle1",
      hint: "riddle1",
    },
    { attachedDeposit: toYocto("1") }
  );

  t.deepEqual(await contract.view("get_riddle", { title: "riddle1" }), {
    title: "riddle1",
    text: "riddle1",
    bounty: +toYocto("1"),
    solved: false,
  });
});

test("create riddle, get riddle hint, solve riddle", async (test) => {
  const { contract, root, ali } = test.context.accounts;

  await root.call(
    contract,
    "create_riddle",
    { title: "riddle1", text: "riddle1", answer: "riddle1", hint: "riddle1" },
    { attachedDeposit: toYocto("1") }
  );

  const hint = await ali.call<string>(
    contract,
    "get_riddle_hint",
    { title: "riddle1" },
    { attachedDeposit: toYocto("0.1") }
  );

  await ali.call<void>(
    contract,
    "solve_riddle",
    { title: "riddle1", answer: "riddle1" },
    { attachedDeposit: toYocto("0.1") }
  );

  const solved = await ali.call<boolean>(contract, "get_riddle_solved", {
    title: "riddle1",
  });

  test.is(hint, "riddle1");

  test.is(solved, true);
});

test("create multiple riddles, solve some, list solved and list unsolved", async (test) => {
  const { contract, root, ali } = test.context.accounts;
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
    await ali.call<void>(
      contract,
      "solve_riddle",
      { title: `riddle${i}`, answer: `riddle${i}` },
      { attachedDeposit: toYocto("0.1") }
    );
  }

  const solved = await ali.call<
    [string, { title: string; text: string; solved: boolean; bounty: number }][]
  >(contract, "get_riddles", { from_index: 0, limit: 10, solved: true });

  const unsolved = await ali.call<
    [string, { title: string; text: string; solved: boolean; bounty: number }][]
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
});

test.afterEach(async (t) => {
  // Stop Sandbox server
  await t.context.worker.tearDown().catch((error) => {
    console.log("Failed to tear down the worker:", error);
  });
});
