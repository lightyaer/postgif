const anchor = require("@project-serum/anchor");
const { SystemProgram } = anchor.web3;

const main = async () => {
  console.log("🚀 Starting test...");

  const provider = anchor.Provider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Postgif;

  const baseAccount = anchor.web3.Keypair.generate();

  let tx = await program.rpc.initialize({
    accounts: {
      baseAccount: baseAccount.publicKey,
      user: provider.wallet.publicKey,
      systemProgram: SystemProgram.programId,
    },
    signers: [baseAccount],
  });

  console.log("📝 Your transaction signature", tx);

  let account = await program.account.baseAccount.fetch(baseAccount.publicKey);
  console.log("👀 GIF Count", account.totalGifs.toString());

  // You'll need to now pass a GIF link to the function! You'll also need to pass in the user submitting the GIF!
  await program.rpc.addGif(
    "https://media.giphy.com/media/bUyNEbEglZg41CMw6y/giphy.gif",
    {
      accounts: {
        baseAccount: baseAccount.publicKey,
        user: provider.wallet.publicKey,
      },
    }
  );

  await program.rpc.upVote(new anchor.BN(0), {
    accounts: {
      baseAccount: baseAccount.publicKey,
      user: provider.wallet.publicKey,
    },
  });

  await program.rpc.downVote(new anchor.BN(0), {
    accounts: {
      baseAccount: baseAccount.publicKey,
      user: provider.wallet.publicKey,
    },
  });

  console.log("📝 Your transaction signature", tx);
  account = await program.account.baseAccount.fetch(baseAccount.publicKey);
  console.log("👀 GIF Count", account.totalGifs.toString());
  console.log("👀 GIF List", account.gifList);
};

const runMain = async () => {
  try {
    await main();
    process.exit(0);
  } catch (error) {
    console.error(error);
    process.exit(1);
  }
};

runMain();
