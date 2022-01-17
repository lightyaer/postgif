<script setup>
import twitterLogo from "./assets/twitter-logo.svg";
import { onMounted, ref, watch } from "vue";
import idl from "../../target/idl/postgif.json";
import { _keypair } from "../../keypair.json";
import { clusterApiUrl, Connection, PublicKey } from "@solana/web3.js";
import { Program, Provider, web3, BN } from "@project-serum/anchor";

// SystemProgram is a reference to the Solana runtime!
const { SystemProgram, Keypair } = web3;

// Create a keypair for the account that will hold the GIF data.
// let baseAccount = Keypair.generate();

const baseAccount = web3.Keypair.fromSecretKey(
  new Uint8Array(Object.values(_keypair.secretKey))
);

// Get our program's id from the IDL file.
const programID = new PublicKey(idl.metadata.address);

// Set our network to devnet.
const network = clusterApiUrl("devnet");

// Controls how we want to acknowledge when a transaction is "done".
const opts = {
  preflightCommitment: "processed",
};

const walletAddress = ref(null);
const inputValue = ref(null);
const gifList = ref([]);

onMounted(() => {
  checkIfWalletIsConnected();
});

watch(walletAddress, (current) => {
  if (current) {
    getGifList();
  }
});

const checkIfWalletIsConnected = async () => {
  try {
    const { solana } = window;

    if (solana) {
      if (solana.isPhantom) {
        const response = await solana.connect({ onlyIfTrusted: true });
        console.log(
          "Connected with Public Key:",
          response.publicKey.toString()
        );
        walletAddress.value = response.publicKey.toString();
      }
    } else {
      alert("Solana object not found! Get a Phantom Wallet 👻");
    }
  } catch (error) {
    console.error(error);
  }
};

const connectWallet = async () => {
  const { solana } = window;

  if (solana) {
    const response = await solana.connect();
    console.log("Connected with Public Key:", response.publicKey.toString());
    walletAddress.value = response.publicKey.toString();
  }
};

const sendGif = async () => {
  if (
    !inputValue.value ||
    (inputValue.value && inputValue.value.length === 0) ||
    (typeof inputValue.value === "string" && !inputValue.value.endsWith(".gif"))
  ) {
    console.log("No gif link given");
    return;
  }
  console.log("GIF LINK", inputValue.value);

  try {
    const provider = getProvider();
    const program = new Program(idl, programID, provider);

    await program.rpc.addGif(inputValue.value, {
      accounts: {
        baseAccount: baseAccount.publicKey,
        user: provider.wallet.publicKey,
      },
    });

    console.log("Gif successfully  sent to program", inputValue);

    await getGifList();

    inputValue.value = null;
  } catch (error) {
    console.error(error);
    inputValue.value = null;
  }
};

const getProvider = () => {
  const { solana } = window;
  const connection = new Connection(network, opts.preflightCommitment);
  return new Provider(connection, solana, opts.preflightCommitment);
};

const createGifAccount = async () => {
  try {
    const provider = getProvider();
    const program = new Program(idl, programID, provider);
    console.log("ping");
    await program.rpc.initialize({
      accounts: {
        baseAccount: baseAccount.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
      },
      signers: [baseAccount],
    });
    console.log(
      "Created a new BaseAccount w/ address:",
      baseAccount.publicKey.toString()
    );
    await getGifList();
  } catch (error) {
    console.log("Error creating BaseAccount account:", error);
  }
};

const getGifList = async () => {
  try {
    const provider = getProvider();
    const program = new Program(idl, programID, provider);
    const account = await program.account.baseAccount.fetch(
      baseAccount.publicKey
    );

    console.log("Got the account", account);
    gifList.value = account.gifList;
  } catch (error) {
    console.log("Error in getGifList: ", error);
    gifList.value = null;
  }
};

const voteGif = async (index, upOrDown) => {
  try {
    const provider = getProvider();
    const program = new Program(idl, programID, provider);

    console.log(index, upOrDown);

    if (upOrDown) {
      await program.rpc.upVote(new BN(index), {
        accounts: {
          baseAccount: baseAccount.publicKey,
          user: provider.wallet.publicKey,
        },
      });
    } else {
      await program.rpc.downVote(new BN(index), {
        accounts: {
          baseAccount: baseAccount.publicKey,
          user: provider.wallet.publicKey,
        },
      });
    }

    await getGifList();
  } catch (error) {
    console.log("Error in up Voting Gif");
  }
};
</script>

<template>
  <div class="App">
    <div class="container">
      <div :class="walletAddress ? 'authed-container' : 'header-container'">
        <p class="header">PostGif</p>
        <p class="sub-text">giphy in solana</p>
        <button
          v-if="!walletAddress"
          class="cta-button connect-wallet-button"
          @Click="connectWallet"
        >
          Connect to Wallet
        </button>
        <div v-else-if="gifList === null" class="connected-container">
          <button
            class="cta-button submit-gif-button"
            @click="createGifAccount"
          >
            Do One-Time Initialization For GIF Program Account
          </button>
        </div>
        <div v-else class="connected-container">
          <form @submit.prevent="sendGif">
            <input
              v-model="inputValue"
              type="text"
              placeholder="Enter gif link!"
            />
            <button type="submit" class="cta-button submit-gif-button">
              Submit
            </button>
          </form>
          <div class="gif-grid">
            <div v-for="(gif, index) in gifList" class="gif-item" :key="index">
              <div class="item-container">
                <div class="item-votes">
                  <button @click="() => voteGif(index, true)">⬆︎</button>
                  <br />
                  <button @click="() => voteGif(index, false)">⬇︎</button>
                </div>
                <div class="item-main">
                  <img :src="gif.gifLink" :alt="gif.gifLink" />
                  <p class="gradient-text">
                    <strong>from:</strong> {{ gif.userAddress.toString() }}
                  </p>
                  <p class="gradient-text">
                    <strong>votes:</strong> {{ gif.votes.toString() }}
                  </p>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
      <div class="footer-container">
        <img alt="Twitter Logo" class="twitter-logo" :src="twitterLogo" />
        <a
          class="footer-text"
          href="https://twitter.com/_buildspace"
          target="_blank"
          rel="noreferrer"
          >built on @_buildspace</a
        >

        <a
          class="footer-text"
          href="https://twitter.com/lightyaer"
          target="_blank"
          rel="noreferrer"
          >by @lightyaer</a
        >
      </div>
    </div>
  </div>
</template>

<style>
body {
  margin: 0;
  font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", "Roboto", "Oxygen",
    "Ubuntu", "Cantarell", "Fira Sans", "Droid Sans", "Helvetica Neue",
    sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
}

code {
  font-family: source-code-pro, Menlo, Monaco, Consolas, "Courier New",
    monospace;
}

.App {
  height: 100vh;
  background-color: #1a202c;
  overflow: scroll;
  text-align: center;
}

.container {
  height: 100%;
  display: flex;
  flex-direction: column;
  justify-content: center;
  padding: 0 30px 0 30px;
}

.authed-container {
  height: 100%;
  display: flex;
  flex-direction: column;
  padding: 30px;
}

.header {
  margin: 0;
  font-size: 50px;
  font-weight: bold;
  color: white;
}

.sub-text {
  font-size: 25px;
  color: white;
}

.gradient-text {
  background: -webkit-linear-gradient(left, #60c657 30%, #35aee2 60%);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
}

.cta-button {
  height: 45px;
  border: 0;
  width: auto;
  padding-left: 40px;
  padding-right: 40px;
  border-radius: 10px;
  cursor: pointer;
  font-size: 16px;
  font-weight: bold;
  color: white;
}

.connect-wallet-button {
  background: -webkit-linear-gradient(left, #60c657, #35aee2);
  background-size: 200% 200%;
  animation: gradient-animation 4s ease infinite;
}

.submit-gif-button {
  background: -webkit-linear-gradient(left, #4e44ce, #9edbfc);
  background-size: 200% 200%;
  animation: gradient-animation 4s ease infinite;
  margin-left: 10px;
}

.item-container {
  display: flex;
}

.item-votes {
  display: flex;
  flex-direction: column;
  margin-right: 16px;
}

.item-votes button {
  border: 0;
  width: auto;
  padding-left: 16px;
  padding-right: 16px;
  border-radius: 8px;
  cursor: pointer;
  font-size: 30px;
  font-weight: bold;
  color: #262626;
  background: -webkit-linear-gradient(left, #4e44ce, #9edbfc);
  background-size: 200% 200%;
  animation: gradient-animation 4s ease infinite;
}

.item-votes button:hover {
  color: whitesmoke;
}

.item-main {
  flex: 1;
}

.footer-container {
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  width: 100%;
  bottom: 0;
  left: 0;
  padding-bottom: 45px;
}

.twitter-logo {
  width: 35px;
  height: 35px;
}

.footer-text {
  color: white;
  font-size: 16px;
  font-weight: bold;
  line-height: 24px;
}

.gif-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(500px, 1fr));
  grid-gap: 1.5rem;
  justify-items: center;
  margin: 0;
  padding: 0;
}

.gif-grid .gif-item {
  display: flex;
  flex-direction: column;
  position: relative;
  justify-self: center;
  align-self: center;
}

.gif-item img {
  width: 100%;
  height: 300px;
  border-radius: 10px;
  object-fit: cover;
}

.connected-container input[type="text"] {
  display: inline-block;
  color: white;
  padding: 10px;
  width: 50%;
  height: 60px;
  font-size: 16px;
  box-sizing: border-box;
  background-color: rgba(0, 0, 0, 0.25);
  border: none;
  border-radius: 10px;
  margin: 50px auto;
}

.connected-container button {
  height: 50px;
}
</style>
